#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;

mod any_fn;
mod error;
mod into_any_fn;

use alloc::boxed::Box;
pub use any_fn::*;
use core::any::Any;
use core::cell::RefCell;
pub use error::*;
pub use into_any_fn::*;

type AnyCell<'a> = &'a RefCell<Box<dyn Any>>;
type BoxedFunction<'a> = Box<dyn FnMut(&[AnyCell]) -> Result<Box<dyn Any>, DynamicError> + 'a>;
