use crate::AnyFnError;
use alloc::boxed::Box;
use core::any::Any;
use core::cell::RefCell;
use core::cell::{Ref, RefMut};

/// A dynianmically-typed value.
pub struct Value(RefCell<Box<dyn Any>>);

impl Value {
    /// Creates a value.
    pub fn new(value: impl Any) -> Self {
        Self(RefCell::new(Box::new(value)))
    }

    /// Downcasts a value into a reference.
    pub fn downcast_ref<T: Any>(&self) -> Result<Ref<&T>, AnyFnError> {
        Ref::filter_map(self.0.borrow(), |value| value.downcast_ref())
            .map_err(|_| AnyFnError::Downcast)
    }

    /// Downcasts a value into a mutable reference.
    pub fn downcast_mut<T: Any>(&self) -> Result<RefMut<&mut T>, AnyFnError> {
        RefMut::filter_map(self.0.borrow_mut(), |value| value.downcast_mut())
            .map_err(|_| AnyFnError::Downcast)
    }
}
