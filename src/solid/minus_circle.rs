use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct MinusCircle;

impl SolidPrivate for MinusCircle {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("clip-rule") => "evenodd",
            At::from("d") => "M10 18a8 8 0 100-16 8 8 0 000 16zM7 9a1 1 0 000 2h6a1 1 0 100-2H7z",
            At::from("fill-rule") => "evenodd",
            ),],
        ]
    }
}

impl Solid for MinusCircle {}
