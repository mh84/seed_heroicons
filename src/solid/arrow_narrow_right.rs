use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct ArrowNarrowRight;

impl SolidPrivate for ArrowNarrowRight {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("clip-rule") => "evenodd",
            At::from("d") => "M12.293 5.293a1 1 0 011.414 0l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-2.293-2.293a1 1 0 010-1.414z",
            At::from("fill-rule") => "evenodd",
            ),],
        ]
    }
}

impl Solid for ArrowNarrowRight {}
