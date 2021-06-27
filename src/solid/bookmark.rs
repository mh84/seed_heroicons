use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct Bookmark;

impl SolidPrivate for Bookmark {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("d") => "M5 4a2 2 0 012-2h6a2 2 0 012 2v14l-5-2.5L5 18V4z",
            ),],
        ]
    }
}

impl Solid for Bookmark {}
