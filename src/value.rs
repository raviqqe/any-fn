use core::any::Any;

/// A dynianmically-typed value.
pub struct Value<'a>(&'a RefCell<Box<dyn Any>>);
