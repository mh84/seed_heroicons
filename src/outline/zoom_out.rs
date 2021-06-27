use seed::{prelude::*, *};

use super::{outline_trait_private::OutlinePrivate, Outline};

pub struct ZoomOut;

impl OutlinePrivate for ZoomOut {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "none",
            At::from("stroke") => "currentColor",
            At::from("viewBox") => "0 0 24 24",
            ),
            path![attrs!(
            At::from("d") => "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0zM13 10H7",
            At::from("stroke-linecap") => "round",
            At::from("stroke-linejoin") => "round",
            At::from("stroke-width") => "2",
            ),],
        ]
    }
}

impl Outline for ZoomOut {}
