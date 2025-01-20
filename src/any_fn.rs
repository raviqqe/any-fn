use super::error::DynamicError;
use alloc::boxed::Box;
use core::{any::Any, cell::RefCell};

/// A dynamic function.
pub struct DynamicFunction<'a> {
    arity: usize,
    function: BoxedFunction<'a>,
}

impl<'a> DynamicFunction<'a> {
    /// Creates a dynamic function.
    pub fn new(arity: usize, function: BoxedFunction<'a>) -> Self {
        Self { arity, function }
    }

    /// Returns an arity of unboxed arguments.
    pub const fn arity(&self) -> usize {
        self.arity
    }

    /// Calls a function.
    pub fn call(&mut self, arguments: &[AnyCell]) -> Result<Box<dyn Any>, DynamicError> {
        (self.function)(arguments)
    }
}
