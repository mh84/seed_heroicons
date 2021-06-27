use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct SearchCircle;

impl SolidPrivate for SearchCircle {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("d") => "M9 9a2 2 0 114 0 2 2 0 01-4 0z",
            ),],
            path![attrs!(
            At::from("clip-rule") => "evenodd",
            At::from("d") => "M10 18a8 8 0 100-16 8 8 0 000 16zm1-13a4 4 0 00-3.446 6.032l-2.261 2.26a1 1 0 101.414 1.415l2.261-2.261A4 4 0 1011 5z",
            At::from("fill-rule") => "evenodd",
            ),],
        ]
    }
}

impl Solid for SearchCircle {}
