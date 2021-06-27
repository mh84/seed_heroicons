use seed::prelude::*;

use super::outline_trait_private::OutlinePrivate;

pub trait Outline: OutlinePrivate {
    fn clean<T>() -> Node<T> {
        let vec: Vec<String> = Vec::new();
        Self::base(vec)
    }

    fn default<T>() -> Node<T> {
        Self::base(Self::classes())
    }

    fn append<T>(classes: impl ToClasses) -> Node<T> {
        if let Some(mut classes) = classes.to_classes() {
            let mut union = Self::classes();
            union.append(&mut classes);
            Self::base(union)
        } else {
            Self::base(Self::classes())
        }
    }

    fn with<T>(classes: impl ToClasses) -> Node<T> {
        Self::base(classes)
    }
}
