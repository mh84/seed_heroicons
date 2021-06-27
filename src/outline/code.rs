use seed::{prelude::*, *};

use super::{outline_trait_private::OutlinePrivate, Outline};

pub struct Code;

impl OutlinePrivate for Code {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "none",
            At::from("stroke") => "currentColor",
            At::from("viewBox") => "0 0 24 24",
            ),
            path![attrs!(
            At::from("d") => "M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4",
            At::from("stroke-linecap") => "round",
            At::from("stroke-linejoin") => "round",
            At::from("stroke-width") => "2",
            ),],
        ]
    }
}

impl Outline for Code {}
