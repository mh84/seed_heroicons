use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct ChevronDoubleRight;

impl SolidPrivate for ChevronDoubleRight {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("clip-rule") => "evenodd",
            At::from("d") => "M10.293 15.707a1 1 0 010-1.414L14.586 10l-4.293-4.293a1 1 0 111.414-1.414l5 5a1 1 0 010 1.414l-5 5a1 1 0 01-1.414 0z",
            At::from("fill-rule") => "evenodd",
            ),],
            path![attrs!(
            At::from("clip-rule") => "evenodd",
            At::from("d") => "M4.293 15.707a1 1 0 010-1.414L8.586 10 4.293 5.707a1 1 0 011.414-1.414l5 5a1 1 0 010 1.414l-5 5a1 1 0 01-1.414 0z",
            At::from("fill-rule") => "evenodd",
            ),],
        ]
    }
}

impl Solid for ChevronDoubleRight {}
