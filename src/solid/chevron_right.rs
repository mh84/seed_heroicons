use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct ChevronRight;

impl SolidPrivate for ChevronRight {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("clip-rule") => "evenodd",
            At::from("d") => "M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z",
            At::from("fill-rule") => "evenodd",
            ),],
        ]
    }
}

impl Solid for ChevronRight {}
