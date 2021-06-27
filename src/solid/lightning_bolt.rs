use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct LightningBolt;

impl SolidPrivate for LightningBolt {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("clip-rule") => "evenodd",
            At::from("d") => "M11.3 1.046A1 1 0 0112 2v5h4a1 1 0 01.82 1.573l-7 10A1 1 0 018 18v-5H4a1 1 0 01-.82-1.573l7-10a1 1 0 011.12-.38z",
            At::from("fill-rule") => "evenodd",
            ),],
        ]
    }
}

impl Solid for LightningBolt {}
