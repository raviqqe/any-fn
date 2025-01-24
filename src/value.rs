use core::any::Any;

/// A dynianmically-typed value.
pub struct Value<'a>(&'a RefCell<Box<dyn Any>>);

impl Value {
    pub fn new<T: Any>(value: T) -> Self {
        Self(RefCell::new(Box::new(x)))
    }
}
