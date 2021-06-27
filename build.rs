use std::{
    env,
    error::Error,
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use git2::{Repository, ResetType};
use minidom::Element;

fn convert(style: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    let lower = &style.to_lowercase();

    let source = &path.join("optimized").join(lower);
    let target = &PathBuf::from(".").join("src").join(lower);

    if !target.exists() {
        fs::create_dir(target)?;
    }

    let mut outline_mods: String = String::new();
    outline_mods.push_str(&format!("mod {}_trait;\n", lower));
    outline_mods.push_str(&format!("pub use {}_trait::{};\n\n", lower, style));

    for entry in fs::read_dir(&source)? {
        if let Ok(entry) = entry {
            if !&entry.metadata()?.is_file() {
                continue;
            }

            let source_filename = &entry.file_name();
            let file_name = if let Some(first) = source_filename
                .to_string_lossy()
                .split('.')
                .collect::<Vec<&str>>()
                .first()
            {
                first.replace('-', "_")
            } else {
                continue;
            };

            outline_mods.push_str(&format!("mod {};\n", file_name));

            let file_path = format!("{}/{}.rs", target.to_string_lossy(), file_name);
            let mut file = fs::File::create(file_path)?;

            let mut file_content = String::from("use seed::{*, prelude::*};\n\n");
            file_content.push_str(&format!("use super::{};\n\n", style));

            let struct_name = file_name.split('_').collect::<Vec<&str>>().iter().fold(
                "".to_string(),
                |mut res, part| {
                    let mut c = part.chars();
                    let s = match c.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().chain(c).collect(),
                    };
                    res.push_str(&s);
                    res
                },
            );

            outline_mods.push_str(&format!("pub use {}::{};\n\n", file_name, struct_name));

            file_content.push_str(&format!("pub struct {};\n\n", struct_name));
            file_content.push_str(&format!("impl {} for {} {{\n", style, struct_name));
            file_content.push_str("fn base<T>(classes: Vec<&str>) -> Node<T> {\n");
            file_content.push_str("svg![\n");
            file_content.push_str("C![classes],\n");
            file_content.push_str("attrs!(\n");

            let contents = fs::read_to_string(source.join(source_filename))?;
            let root: Element = contents.parse()?;

            for attr in root.attrs() {
                file_content.push_str(&format!("At::from(\"{}\") => \"{}\",\n", attr.0, attr.1));
            }

            file_content.push_str("),\n");

            for child in root.children() {
                file_content.push_str("path![\n");
                file_content.push_str("attrs!(\n");

                for attr in child.attrs() {
                    file_content
                        .push_str(&format!("At::from(\"{}\") => \"{}\",\n", attr.0, attr.1));
                }

                file_content.push_str("),\n");
                file_content.push_str("],\n");
            }

            file_content.push_str("]\n");
            file_content.push_str("}\n");
            file_content.push('}');

            if let Err(error) = write!(file, "{}", file_content) {
                return Err(Box::new(error));
            }
        } else {
            panic!();
        }
    }

    let mut outline_mods_file = fs::File::create(target.join("mod.rs"))?;

    if let Err(error) = writeln!(outline_mods_file, "{}", outline_mods) {
        Err(Box::new(error))
    } else {
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    if let None = env::var_os("DO_ICON_GENERATION") {
        return Ok(());
    }

    if let Some(out_dir) = env::var_os("OUT_DIR") {
        let path = PathBuf::from(out_dir).join("heroicons");

        if !path.exists() {
            let url = "https://github.com/tailwindlabs/heroicons";
            Repository::clone(url, &path)?;
        } else {
            let repo = Repository::open(&path)?;
            let branch = "master";

            repo.reset(&repo.revparse_single("HEAD")?, ResetType::Hard, None)?;
            repo.find_remote("origin")?.fetch(&[branch], None, None)?;

            let fetch_head = repo.find_reference("FETCH_HEAD")?;
            let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
            let analysis = repo.merge_analysis(&[&fetch_commit])?;

            if analysis.0.is_fast_forward() {
                let refname = format!("refs/heads/{}", branch);
                let mut reference = repo.find_reference(&refname)?;
                reference.set_target(fetch_commit.id(), "Fast-Forward")?;
                repo.set_head(&refname)?;
                repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
            }
        }

        convert("Outline", &path)?;
        convert("Solid", &path)?;

        Ok(())
    } else {
        panic!()
    }
}
