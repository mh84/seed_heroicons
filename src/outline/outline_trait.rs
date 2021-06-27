use seed::prelude::*;

pub trait Outline {
    fn base<T>(classes: Vec<&str>) -> Node<T>;

    fn classes() -> Vec<&'static str> {
        vec!["h-6", "w-6"]
    }

    fn clean<T>() -> Node<T> {
        Self::base(Vec::new())
    }

    fn default<T>() -> Node<T> {
        Self::base(Self::classes())
    }

    fn append<T>(classes: Vec<&str>) -> Node<T> {
        let mut union = Self::classes();
        union.append(&mut classes.clone());
        Self::base(union)
    }

    fn with<T>(classes: Vec<&str>) -> Node<T> {
        Self::base(classes)
    }
}
