use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct BookmarkAlt;

impl SolidPrivate for BookmarkAlt {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("clip-rule") => "evenodd",
            At::from("d") => "M3 5a2 2 0 012-2h10a2 2 0 012 2v10a2 2 0 01-2 2H5a2 2 0 01-2-2V5zm11 1H6v8l4-2 4 2V6z",
            At::from("fill-rule") => "evenodd",
            ),],
        ]
    }
}

impl Solid for BookmarkAlt {}
