use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct LocationMarker;

impl SolidPrivate for LocationMarker {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("clip-rule") => "evenodd",
            At::from("d") => "M5.05 4.05a7 7 0 119.9 9.9L10 18.9l-4.95-4.95a7 7 0 010-9.9zM10 11a2 2 0 100-4 2 2 0 000 4z",
            At::from("fill-rule") => "evenodd",
            ),],
        ]
    }
}

impl Solid for LocationMarker {}
