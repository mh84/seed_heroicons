use seed::prelude::*;

pub trait SolidPrivate {
    fn base<T>(classes: impl ToClasses) -> Node<T>;

    fn classes() -> Vec<String> {
        vec![String::from("h-5"), String::from("w-5")]
    }
}
