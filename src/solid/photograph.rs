use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct Photograph;

impl SolidPrivate for Photograph {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("clip-rule") => "evenodd",
            At::from("d") => "M4 3a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V5a2 2 0 00-2-2H4zm12 12H4l4-8 3 6 2-4 3 6z",
            At::from("fill-rule") => "evenodd",
            ),],
        ]
    }
}

impl Solid for Photograph {}
