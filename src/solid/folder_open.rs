use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct FolderOpen;

impl SolidPrivate for FolderOpen {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("clip-rule") => "evenodd",
            At::from("d") => "M2 6a2 2 0 012-2h4l2 2h4a2 2 0 012 2v1H8a3 3 0 00-3 3v1.5a1.5 1.5 0 01-3 0V6z",
            At::from("fill-rule") => "evenodd",
            ),],
            path![attrs!(
            At::from("d") => "M6 12a2 2 0 012-2h8a2 2 0 012 2v2a2 2 0 01-2 2H2h2a2 2 0 002-2v-2z",
            ),],
        ]
    }
}

impl Solid for FolderOpen {}
