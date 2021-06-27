use seed::{prelude::*, *};

use super::{outline_trait_private::OutlinePrivate, Outline};

pub struct TrendingDown;

impl OutlinePrivate for TrendingDown {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "none",
            At::from("stroke") => "currentColor",
            At::from("viewBox") => "0 0 24 24",
            ),
            path![attrs!(
            At::from("d") => "M13 17h8m0 0V9m0 8l-8-8-4 4-6-6",
            At::from("stroke-linecap") => "round",
            At::from("stroke-linejoin") => "round",
            At::from("stroke-width") => "2",
            ),],
        ]
    }
}

impl Outline for TrendingDown {}
