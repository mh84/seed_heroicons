use seed::{prelude::*, *};

use super::{outline_trait_private::OutlinePrivate, Outline};

pub struct ChevronDoubleLeft;

impl OutlinePrivate for ChevronDoubleLeft {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "none",
            At::from("stroke") => "currentColor",
            At::from("viewBox") => "0 0 24 24",
            ),
            path![attrs!(
            At::from("d") => "M11 19l-7-7 7-7m8 14l-7-7 7-7",
            At::from("stroke-linecap") => "round",
            At::from("stroke-linejoin") => "round",
            At::from("stroke-width") => "2",
            ),],
        ]
    }
}

impl Outline for ChevronDoubleLeft {}
