use seed::{prelude::*, *};

use super::{outline_trait_private::OutlinePrivate, Outline};

pub struct ArrowNarrowDown;

impl OutlinePrivate for ArrowNarrowDown {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "none",
            At::from("stroke") => "currentColor",
            At::from("viewBox") => "0 0 24 24",
            ),
            path![attrs!(
            At::from("d") => "M16 17l-4 4m0 0l-4-4m4 4V3",
            At::from("stroke-linecap") => "round",
            At::from("stroke-linejoin") => "round",
            At::from("stroke-width") => "2",
            ),],
        ]
    }
}

impl Outline for ArrowNarrowDown {}
