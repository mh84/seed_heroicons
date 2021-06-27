use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct User;

impl SolidPrivate for User {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("clip-rule") => "evenodd",
            At::from("d") => "M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z",
            At::from("fill-rule") => "evenodd",
            ),],
        ]
    }
}

impl Solid for User {}
