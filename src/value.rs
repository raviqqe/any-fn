use alloc::boxed::Box;
use core::any::Any;
use core::cell::RefCell;

/// A dynianmically-typed value.
pub struct Value(RefCell<Box<dyn Any>>);

impl Value {
    /// Creates a value.
    pub fn new(value: impl Any) -> Self {
        Self(RefCell::new(Box::new(value)))
    }
}
