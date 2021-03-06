use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct DotsVertical;

impl SolidPrivate for DotsVertical {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("d") => "M10 6a2 2 0 110-4 2 2 0 010 4zM10 12a2 2 0 110-4 2 2 0 010 4zM10 18a2 2 0 110-4 2 2 0 010 4z",
            ),],
        ]
    }
}

impl Solid for DotsVertical {}
