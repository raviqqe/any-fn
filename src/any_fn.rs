use super::error::AnyFnError;
use crate::Value;
use alloc::boxed::Box;
use core::any::{Any, TypeId};

type BoxedFunction<'a> = Box<dyn FnMut(&[&Value]) -> Result<Box<dyn Any>, AnyFnError> + 'a>;

/// A dynamically-typed function.
pub struct AnyFn<'a> {
    parameter_types: Vec<TypeId>,
    return_type: TypeId,
    function: BoxedFunction<'a>,
}

impl<'a> AnyFn<'a> {
    /// Creates a dynamically-typed function.
    pub(crate) fn new(
        parameter_types: Vec<TypeId>,
        return_type: TypeId,
        function: BoxedFunction<'a>,
    ) -> Self {
        Self {
            parameter_types,
            return_type,
            function,
        }
    }

    /// Returns an arity of arguments.
    pub const fn arity(&self) -> usize {
        self.arity
    }

    /// Calls a function.
    pub fn call(&mut self, arguments: &[&Value]) -> Result<Box<dyn Any>, AnyFnError> {
        (self.function)(arguments)
    }
}
