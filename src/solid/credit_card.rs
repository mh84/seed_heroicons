use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct CreditCard;

impl SolidPrivate for CreditCard {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("d") => "M4 4a2 2 0 00-2 2v1h16V6a2 2 0 00-2-2H4z",
            ),],
            path![attrs!(
            At::from("clip-rule") => "evenodd",
            At::from("d") => "M18 9H2v5a2 2 0 002 2h12a2 2 0 002-2V9zM4 13a1 1 0 011-1h1a1 1 0 110 2H5a1 1 0 01-1-1zm5-1a1 1 0 100 2h1a1 1 0 100-2H9z",
            At::from("fill-rule") => "evenodd",
            ),],
        ]
    }
}

impl Solid for CreditCard {}
