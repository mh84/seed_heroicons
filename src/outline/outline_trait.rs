use seed::prelude::*;

use super::outline_trait_private::OutlinePrivate;

pub trait Outline: OutlinePrivate {
    /*
    icon without classes
    */
    fn clean<T>() -> Node<T> {
        let vec: Vec<String> = Vec::new();
        Self::base(vec)
    }

    /*
    icon with default tailwindcss classes as provided on https://heroicons.com/
    default: h-6 w-6
    */
    fn default<T>() -> Node<T> {
        Self::base(Self::classes())
    }

    /*
    icon with default tailwindcss classes and additional provided classes
    default: h-6 w-6
    */
    fn append<T>(classes: impl ToClasses) -> Node<T> {
        if let Some(mut classes) = classes.to_classes() {
            let mut union = Self::classes();
            union.append(&mut classes);
            Self::base(union)
        } else {
            Self::base(Self::classes())
        }
    }

    /*
    icon with provided classes
    */
    fn with<T>(classes: impl ToClasses) -> Node<T> {
        Self::base(classes)
    }
}
