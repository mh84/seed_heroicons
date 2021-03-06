use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct Newspaper;

impl SolidPrivate for Newspaper {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("clip-rule") => "evenodd",
            At::from("d") => "M2 5a2 2 0 012-2h8a2 2 0 012 2v10a2 2 0 002 2H4a2 2 0 01-2-2V5zm3 1h6v4H5V6zm6 6H5v2h6v-2z",
            At::from("fill-rule") => "evenodd",
            ),],
            path![attrs!(
            At::from("d") => "M15 7h1a2 2 0 012 2v5.5a1.5 1.5 0 01-3 0V7z",
            ),],
        ]
    }
}

impl Solid for Newspaper {}
