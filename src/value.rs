use alloc::boxed::Box;
use core::any::Any;
use core::cell::RefCell;

/// A dynianmically-typed value.
pub struct Value<'a>(&'a RefCell<Box<dyn Any>>);

impl<T: Any> Value<T> {
    /// Creates a value.
    pub fn new<T: Any>(value: T) -> Self {
        Self(RefCell::new(Box::new(x)))
    }
}
