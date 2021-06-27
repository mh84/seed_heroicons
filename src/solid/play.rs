use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct Play;

impl SolidPrivate for Play {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("clip-rule") => "evenodd",
            At::from("d") => "M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z",
            At::from("fill-rule") => "evenodd",
            ),],
        ]
    }
}

impl Solid for Play {}
