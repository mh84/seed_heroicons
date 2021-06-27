use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct Minus;

impl SolidPrivate for Minus {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("clip-rule") => "evenodd",
            At::from("d") => "M3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z",
            At::from("fill-rule") => "evenodd",
            ),],
        ]
    }
}

impl Solid for Minus {}
