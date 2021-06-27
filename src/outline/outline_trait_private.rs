use seed::prelude::*;

pub trait OutlinePrivate {
    fn base<T>(classes: impl ToClasses) -> Node<T>;

    fn classes() -> Vec<String> {
        vec![String::from("h-6"), String::from("w-6")]
    }
}
