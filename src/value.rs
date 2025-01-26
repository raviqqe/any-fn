use crate::AnyFnError;
use alloc::boxed::Box;
use core::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
};

/// A dynamically-typed value.
pub struct Value(RefCell<Box<dyn Any>>);

impl Value {
    /// Creates a value.
    pub fn new(value: impl Any) -> Self {
        Self(RefCell::new(Box::new(value)))
    }

    /// Returns a type ID.
    pub fn type_id(&self) -> Result<TypeId, AnyFnError> {
        let cell = self.0.try_borrow()?;

        Ok(cell.type_id())
    }

    /// Downcasts a value.
    pub fn downcast<T: Any>(self) -> Result<T, AnyFnError> {
        self.0
            .into_inner()
            .downcast()
            .map_err(|_| AnyFnError::Downcast)
            .map(|value| *value)
    }

    /// Downcasts a value into a reference.
    pub fn downcast_ref<T: Any>(&self) -> Result<Ref<T>, AnyFnError> {
        Ref::filter_map(self.0.try_borrow()?, |value| value.downcast_ref())
            .map_err(|_| AnyFnError::Downcast)
    }

    /// Downcasts a value into a mutable reference.
    pub fn downcast_mut<T: Any>(&self) -> Result<RefMut<T>, AnyFnError> {
        RefMut::filter_map(self.0.try_borrow_mut()?, |value| value.downcast_mut())
            .map_err(|_| AnyFnError::Downcast)
    }
}
