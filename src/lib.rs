#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;

mod any_fn;
mod error;
mod into_any_fn;
mod r#ref;
mod ref_mut;

use alloc::boxed::Box;
pub use any_fn::*;
use core::{any::Any, cell::RefCell};
pub use error::*;
pub use into_any_fn::*;
pub use r#ref::*;
pub use ref_mut::*;

type AnyCell<'a> = &'a RefCell<Box<dyn Any>>;
type BoxedFunction<'a> = Box<dyn FnMut(&[AnyCell]) -> Result<Box<dyn Any>, AnyFnError> + 'a>;

/// Creates a dynamically-typed value.
pub fn value<T: 'static>(x: T) -> RefCell<Box<dyn Any>> {
    RefCell::new(Box::new(x))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{format, string::String};

    #[test]
    fn create_function() {
        #[derive(Clone, Debug)]
        struct Foo {}

        const fn foo(x: usize, y: usize) -> usize {
            x + y
        }

        fn bar(name: String, value: Option<Foo>) -> String {
            format!("{name}: {value:?}")
        }

        foo.into_any_fn();
        bar.into_any_fn();
    }

    #[test]
    fn call_function() {
        const fn foo(x: usize, y: usize) -> usize {
            x + y
        }

        assert_eq!(
            *foo.into_any_fn()
                .call(&[&value(1usize), &value(2usize)])
                .unwrap()
                .downcast::<usize>()
                .unwrap(),
            3
        );
    }

    #[test]
    fn call_function_with_mutable_reference_as_last_argument() {
        fn foo(x: usize, y: &mut usize) {
            *y = x;
        }

        let x = value(0usize);

        foo.into_any_fn().call(&[&value(42usize), &x]).unwrap();

        assert_eq!(*x.borrow().downcast_ref::<usize>().unwrap(), 42);
    }

    #[test]
    fn call_function_with_mutable_reference_as_first_argument() {
        fn foo(x: &mut usize, y: usize) {
            *x = y;
        }

        let x = value(0usize);

        foo.into_any_fn().call(&[&x, &value(42usize)]).unwrap();

        assert_eq!(*x.borrow().downcast_ref::<usize>().unwrap(), 42);
    }

    #[test]
    fn call_function_with_all_types() {
        fn foo(x: usize, y: &usize, z: &mut usize) {
            *z = x + *y;
        }

        let x = value(0usize);

        <_ as IntoAnyFn<'_, (_, Ref<usize>, _), _>>::into_any_fn(foo)
            .call(&[&value(40usize), &value(2usize), &x])
            .unwrap();

        assert_eq!(*x.borrow().downcast_ref::<usize>().unwrap(), 42);
    }

    #[test]
    fn mutate_struct() {
        struct Foo {
            foo: usize,
        }

        fn foo(x: usize, y: &mut Foo) {
            y.foo = x;
        }

        let x = value(Foo { foo: 0 });

        foo.into_any_fn().call(&[&value(42usize), &x]).unwrap();

        assert_eq!(x.borrow().downcast_ref::<Foo>().unwrap().foo, 42);
    }
}
