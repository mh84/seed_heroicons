use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct DotsHorizontal;

impl SolidPrivate for DotsHorizontal {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("d") => "M6 10a2 2 0 11-4 0 2 2 0 014 0zM12 10a2 2 0 11-4 0 2 2 0 014 0zM16 12a2 2 0 100-4 2 2 0 000 4z",
            ),],
        ]
    }
}

impl Solid for DotsHorizontal {}
