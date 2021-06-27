use seed::{prelude::*, *};

use super::Solid;

pub struct FolderDownload;

impl Solid for FolderDownload {
    fn base<T>(classes: Vec<&str>) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("d") => "M2 6a2 2 0 012-2h5l2 2h5a2 2 0 012 2v6a2 2 0 01-2 2H4a2 2 0 01-2-2V6z",
            ),],
            path![attrs!(
            At::from("d") => "M10 9v4m0 0l-2-2m2 2l2-2",
            At::from("stroke") => "#fff",
            At::from("stroke-linecap") => "round",
            At::from("stroke-linejoin") => "round",
            At::from("stroke-width") => "2",
            ),],
        ]
    }
}
