#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;

mod any_fn;
mod error;
mod into_any_fn;
mod ref_mut;

use alloc::boxed::Box;
pub use any_fn::*;
use core::{any::Any, cell::RefCell};
pub use error::*;
pub use into_any_fn::*;
pub use ref_mut::*;

type AnyCell<'a> = &'a RefCell<Box<dyn Any>>;
type BoxedFunction<'a> = Box<dyn FnMut(&[AnyCell]) -> Result<Box<dyn Any>, AnyFnError> + 'a>;

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{format, string::String};
    use core::cell::RefCell;

    #[derive(Clone, Debug)]
    struct Foo {}

    const fn foo(x: usize, y: usize) -> usize {
        x + y
    }

    fn bar(name: String, value: Option<Foo>) -> String {
        format!("{name}: {value:?}")
    }

    fn baz(x: usize, y: &mut usize) {
        *y = x;
    }

    fn wrap<T: 'static>(x: T) -> RefCell<Box<dyn Any>> {
        RefCell::new(Box::new(x))
    }

    #[test]
    fn create_any_fn() {
        foo.into_any_fn();
        bar.into_any_fn();
    }

    #[test]
    fn call_any_fn() {
        assert_eq!(
            *foo.into_any_fn()
                .call(&[&wrap(1usize), &wrap(2usize)])
                .unwrap()
                .downcast::<usize>()
                .unwrap(),
            3
        );
    }

    #[test]
    fn call_any_fn_with_mutable_reference() {
        let x = wrap(0usize);

        baz.into_any_fn().call(&[&wrap(42usize), &x]).unwrap();

        assert_eq!(*x.borrow().downcast_ref::<usize>().unwrap(), 42);
    }
}
