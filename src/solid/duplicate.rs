use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct Duplicate;

impl SolidPrivate for Duplicate {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("d") => "M7 9a2 2 0 012-2h6a2 2 0 012 2v6a2 2 0 01-2 2H9a2 2 0 01-2-2V9z",
            ),],
            path![attrs!(
            At::from("d") => "M5 3a2 2 0 00-2 2v6a2 2 0 002 2V5h8a2 2 0 00-2-2H5z",
            ),],
        ]
    }
}

impl Solid for Duplicate {}
