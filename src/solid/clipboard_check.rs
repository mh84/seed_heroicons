use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct ClipboardCheck;

impl SolidPrivate for ClipboardCheck {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("d") => "M9 2a1 1 0 000 2h2a1 1 0 100-2H9z",
            ),],
            path![attrs!(
            At::from("clip-rule") => "evenodd",
            At::from("d") => "M4 5a2 2 0 012-2 3 3 0 003 3h2a3 3 0 003-3 2 2 0 012 2v11a2 2 0 01-2 2H6a2 2 0 01-2-2V5zm9.707 5.707a1 1 0 00-1.414-1.414L9 12.586l-1.293-1.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z",
            At::from("fill-rule") => "evenodd",
            ),],
        ]
    }
}

impl Solid for ClipboardCheck {}
