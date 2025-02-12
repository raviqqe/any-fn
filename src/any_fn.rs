use super::error::AnyFnError;
use crate::Value;
use alloc::{boxed::Box, vec::Vec};
use core::any::TypeId;

type BoxedFn<'a> = Box<dyn FnMut(&[&Value]) -> Result<Value, AnyFnError> + 'a>;

/// A dynamically-typed function.
pub struct AnyFn<'a> {
    parameter_types: Vec<TypeId>,
    return_type: TypeId,
    function: BoxedFn<'a>,
}

impl<'a> AnyFn<'a> {
    /// Creates a dynamically-typed function.
    pub(crate) fn new(
        parameter_types: Vec<TypeId>,
        return_type: TypeId,
        function: BoxedFn<'a>,
    ) -> Self {
        Self {
            parameter_types,
            return_type,
            function,
        }
    }

    /// Returns parameter types.
    pub fn parameter_types(&self) -> &[TypeId] {
        &self.parameter_types
    }

    /// Returns a return type.
    pub const fn return_type(&self) -> TypeId {
        self.return_type
    }

    /// Returns an arity.
    pub fn arity(&self) -> usize {
        self.parameter_types.len()
    }

    /// Calls a function.
    pub fn call(&mut self, arguments: &[&Value]) -> Result<Value, AnyFnError> {
        (self.function)(arguments)
    }
}
