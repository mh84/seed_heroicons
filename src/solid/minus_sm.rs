use seed::{prelude::*, *};

use super::Solid;

pub struct MinusSm;

impl Solid for MinusSm {
    fn base<T>(classes: Vec<&str>) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("clip-rule") => "evenodd",
            At::from("d") => "M5 10a1 1 0 011-1h8a1 1 0 110 2H6a1 1 0 01-1-1z",
            At::from("fill-rule") => "evenodd",
            ),],
        ]
    }
}
