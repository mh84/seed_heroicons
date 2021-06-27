use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct UserRemove;

impl SolidPrivate for UserRemove {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("d") => "M11 6a3 3 0 11-6 0 3 3 0 016 0zM14 17a6 6 0 00-12 0h12zM13 8a1 1 0 100 2h4a1 1 0 100-2h-4z",
            ),],
        ]
    }
}

impl Solid for UserRemove {}
