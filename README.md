seed_heroicons
==============

Provides a simple to use abstraction for [heroicons](https://heroicons.com/) to embed into [seed](https://github.com/seed-rs/seed) projects with the possibility to use additional classes if needed.

The icons are generated automatically from (source)[https://github.com/tailwindlabs/heroicons] and all thanks for creating them should be given to [Tailwind Labs](https://github.com/tailwindlabs).

## Example

```rust
use seed_heroicons::*;

...

fn view(model: &Model) -> Node<Msg> {
    div![
        // icon with default tailwindcss classes as provided on https://heroicons.com/
        // outline: h-6 w-6
        outline::AcademicCap::default(),
        // solid: h-5 w-5
        solid::AcademicCap::default(),

        // icon with default tailwindcss classes and additional provided classes
        // uses https://docs.rs/seed/0.8.0/seed/virtual_dom/to_classes/trait.ToClasses.html as argument
        outline::AcademicCap::append("text-black"),
        solid::AcademicCap::append(["text-black", "m-2"]),

        // icon without classes
        outline::AcademicCap::clean(),
        solid::AcademicCap::clean(),

        // icon with provided classes
        // uses https://docs.rs/seed/0.8.0/seed/virtual_dom/to_classes/trait.ToClasses.html as argument
        outline::AcademicCap::with("text-black"),
        solid::AcademicCap::with(["text-black", "m-2"]),
    ]
}

...
```