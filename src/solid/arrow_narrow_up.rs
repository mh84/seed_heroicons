use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct ArrowNarrowUp;

impl SolidPrivate for ArrowNarrowUp {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("clip-rule") => "evenodd",
            At::from("d") => "M5.293 7.707a1 1 0 010-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 01-1.414 1.414L11 5.414V17a1 1 0 11-2 0V5.414L6.707 7.707a1 1 0 01-1.414 0z",
            At::from("fill-rule") => "evenodd",
            ),],
        ]
    }
}

impl Solid for ArrowNarrowUp {}
