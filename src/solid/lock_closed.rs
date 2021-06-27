use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct LockClosed;

impl SolidPrivate for LockClosed {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("clip-rule") => "evenodd",
            At::from("d") => "M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z",
            At::from("fill-rule") => "evenodd",
            ),],
        ]
    }
}

impl Solid for LockClosed {}
