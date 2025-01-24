use alloc::boxed::Box;
use core::any::Any;
use core::cell::RefCell;

use crate::AnyFnError;

/// A dynianmically-typed value.
pub struct Value(RefCell<Box<dyn Any>>);

impl Value {
    /// Creates a value.
    pub fn new(value: impl Any) -> Self {
        Self(RefCell::new(Box::new(value)))
    }

    /// Downcasts a value into a reference.
    pub fn downcast_ref<T: Any>(&self) -> Result<&T, AnyFnError> {
        self.0.borrow().downcast_ref().ok_or(AnyFnError::Downcast)
    }

    /// Downcasts a value into a mutable reference.
    pub fn downcast_mut<T: Any>(&self) -> Result<&mut T, AnyFnError> {
        self.0
            .borrow_mut()
            .downcast_mut()
            .ok_or(AnyFnError::Downcast)
    }
}
