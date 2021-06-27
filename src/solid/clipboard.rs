use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct Clipboard;

impl SolidPrivate for Clipboard {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("d") => "M8 3a1 1 0 011-1h2a1 1 0 110 2H9a1 1 0 01-1-1z",
            ),],
            path![attrs!(
            At::from("d") => "M6 3a2 2 0 00-2 2v11a2 2 0 002 2h8a2 2 0 002-2V5a2 2 0 00-2-2 3 3 0 01-3 3H9a3 3 0 01-3-3z",
            ),],
        ]
    }
}

impl Solid for Clipboard {}
