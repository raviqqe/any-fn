use super::error::AnyFnError;
use crate::{AnyCell, BoxedFunction};
use alloc::boxed::Box;
use core::any::Any;

/// A dynamically-typed function.
pub struct AnyFn<'a> {
    arity: usize,
    function: BoxedFunction<'a>,
}

impl<'a> AnyFn<'a> {
    /// Creates a dynamically-typed function.
    pub fn new(arity: usize, function: BoxedFunction<'a>) -> Self {
        Self { arity, function }
    }

    /// Returns an arity of unboxed arguments.
    pub const fn arity(&self) -> usize {
        self.arity
    }

    /// Calls a function.
    pub fn call(&mut self, arguments: &[AnyCell]) -> Result<Box<dyn Any>, AnyFnError> {
        (self.function)(arguments)
    }
}
