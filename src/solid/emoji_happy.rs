use seed::{prelude::*, *};

use super::{solid_trait_private::SolidPrivate, Solid};

pub struct EmojiHappy;

impl SolidPrivate for EmojiHappy {
    fn base<T>(classes: impl ToClasses) -> Node<T> {
        svg![
            C![classes],
            attrs!(
            At::from("fill") => "currentColor",
            At::from("viewBox") => "0 0 20 20",
            ),
            path![attrs!(
            At::from("clip-rule") => "evenodd",
            At::from("d") => "M10 18a8 8 0 100-16 8 8 0 000 16zM7 9a1 1 0 100-2 1 1 0 000 2zm7-1a1 1 0 11-2 0 1 1 0 012 0zm-.464 5.535a1 1 0 10-1.415-1.414 3 3 0 01-4.242 0 1 1 0 00-1.415 1.414 5 5 0 007.072 0z",
            At::from("fill-rule") => "evenodd",
            ),],
        ]
    }
}

impl Solid for EmojiHappy {}
