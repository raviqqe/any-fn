#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;

mod any_fn;
mod error;
mod into_any_fn;
mod r#ref;
mod ref_mut;
mod value;

pub use any_fn::*;
use core::any::Any;
pub use error::*;
pub use into_any_fn::*;
pub use r#ref::*;
pub use ref_mut::*;
pub use value::*;

/// Creates a dynamically-typed value.
pub fn value(value: impl Any) -> Value {
    Value::new(value)
}

/// Creates a dynamically-typed function.
pub fn r#fn<'a, T, S>(r#fn: impl IntoAnyFn<'a, T, S>) -> AnyFn<'a> {
    r#fn.into_any_fn()
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
            foo.into_any_fn()
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

        assert_eq!(*x.downcast_ref::<usize>().unwrap(), 42);
    }

    #[test]
    fn call_function_with_mutable_reference_as_first_argument() {
        fn foo(x: &mut usize, y: usize) {
            *x = y;
        }

        let x = value(0usize);

        foo.into_any_fn().call(&[&x, &value(42usize)]).unwrap();

        assert_eq!(*x.downcast_ref::<usize>().unwrap(), 42);
    }

    #[test]
    fn call_function_with_all_types() {
        fn foo(x: usize, y: &usize, z: &mut usize) {
            *z = x + *y;
        }

        let x = value(0usize);

        IntoAnyFn::<(_, Ref<_>, _), _>::into_any_fn(foo)
            .call(&[&value(40usize), &value(2usize), &x])
            .unwrap();

        assert_eq!(*x.downcast_ref::<usize>().unwrap(), 42);
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

        assert_eq!(x.downcast_ref::<Foo>().unwrap().foo, 42);
    }

    #[test]
    fn convert_closure() {
        let mut x: usize = 0;

        (|| {
            x = 42;
        })
        .into_any_fn()
        .call(&[])
        .unwrap();

        assert_eq!(x, 42);
    }
}
