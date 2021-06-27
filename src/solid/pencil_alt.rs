use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct PencilAlt;

impl SolidPrivate for PencilAlt {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("d") => "M17.414 2.586a2 2 0 00-2.828 0L7 10.172V13h2.828l7.586-7.586a2 2 0 000-2.828z",
            ),],
            path![attrs!(
            At::from("clip-rule") => "evenodd",
            At::from("d") => "M2 6a2 2 0 012-2h4a1 1 0 010 2H4v10h10v-4a1 1 0 112 0v4a2 2 0 01-2 2H4a2 2 0 01-2-2V6z",
            At::from("fill-rule") => "evenodd",
            ),],
        ]
    }
}

impl Solid for PencilAlt {}
