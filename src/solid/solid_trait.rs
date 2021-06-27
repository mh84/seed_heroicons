use seed::prelude::*;

pub trait Solid {
    fn base<T>(classes: Vec<&str>) -> Node<T>;

    fn classes() -> Vec<&'static str> {
        vec!["h-5", "w-5"]
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
