use seed::{prelude::*, *};

use super::Outline;

pub struct ArrowSmDown;

impl Outline for ArrowSmDown {
    fn base<T>(classes: Vec<&str>) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "none",
            At::from("stroke") => "currentColor",
            At::from("viewBox") => "0 0 24 24",
            ),
            path![attrs!(
            At::from("d") => "M17 13l-5 5m0 0l-5-5m5 5V6",
            At::from("stroke-linecap") => "round",
            At::from("stroke-linejoin") => "round",
            At::from("stroke-width") => "2",
            ),],
        ]
    }
}
