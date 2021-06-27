use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct DotsCircleHorizontal;

impl SolidPrivate for DotsCircleHorizontal {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("clip-rule") => "evenodd",
            At::from("d") => "M10 18a8 8 0 100-16 8 8 0 000 16zM7 9H5v2h2V9zm8 0h-2v2h2V9zM9 9h2v2H9V9z",
            At::from("fill-rule") => "evenodd",
            ),],
        ]
    }
}

impl Solid for DotsCircleHorizontal {}
