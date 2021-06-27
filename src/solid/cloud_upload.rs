use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct CloudUpload;

impl SolidPrivate for CloudUpload {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("d") => "M5.5 13a3.5 3.5 0 01-.369-6.98 4 4 0 117.753-1.977A4.5 4.5 0 1113.5 13H11V9.413l1.293 1.293a1 1 0 001.414-1.414l-3-3a1 1 0 00-1.414 0l-3 3a1 1 0 001.414 1.414L9 9.414V13H5.5z",
            ),],
            path![attrs!(
            At::from("d") => "M9 13h2v5a1 1 0 11-2 0v-5z",
            ),],
        ]
    }
}

impl Solid for CloudUpload {}
