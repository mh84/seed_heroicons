use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct Archive;

impl SolidPrivate for Archive {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("d") => "M4 3a2 2 0 100 4h12a2 2 0 100-4H4z",
            ),],
            path![attrs!(
            At::from("clip-rule") => "evenodd",
            At::from("d") => "M3 8h14v7a2 2 0 01-2 2H5a2 2 0 01-2-2V8zm5 3a1 1 0 011-1h2a1 1 0 110 2H9a1 1 0 01-1-1z",
            At::from("fill-rule") => "evenodd",
            ),],
        ]
    }
}

impl Solid for Archive {}
