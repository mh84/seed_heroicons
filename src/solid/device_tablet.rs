use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct DeviceTablet;

impl SolidPrivate for DeviceTablet {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("clip-rule") => "evenodd",
            At::from("d") => "M6 2a2 2 0 00-2 2v12a2 2 0 002 2h8a2 2 0 002-2V4a2 2 0 00-2-2H6zm4 14a1 1 0 100-2 1 1 0 000 2z",
            At::from("fill-rule") => "evenodd",
            ),],
        ]
    }
}

impl Solid for DeviceTablet {}
