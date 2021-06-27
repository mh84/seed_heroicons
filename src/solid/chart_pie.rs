use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct ChartPie;

impl SolidPrivate for ChartPie {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("d") => "M2 10a8 8 0 018-8v8h8a8 8 0 11-16 0z",
            ),],
            path![attrs!(
            At::from("d") => "M12 2.252A8.014 8.014 0 0117.748 8H12V2.252z",
            ),],
        ]
    }
}

impl Solid for ChartPie {}
