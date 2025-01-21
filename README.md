# any-fn

[![GitHub Action](https://img.shields.io/github/actions/workflow/status/raviqqe/any-fn/test.yaml?branch=main&style=flat-square)](https://github.com/raviqqe/any-fn/actions)
[![Crate](https://img.shields.io/crates/v/any-fn.svg?style=flat-square)](https://crates.io/crates/any-fn)
[![License](https://img.shields.io/github/license/raviqqe/any-fn.svg?style=flat-square)](https://github.com/raviqqe/any-fn/blob/main/LICENSE)

Dynamically-typed functions via [`core::any::Any`](https://doc.rust-lang.org/stable/core/any/trait.Any.html) in Rust.

## Examples

### Calling a function with unboxed, immutable reference, and mutable reference arguments

```rust
use any_fn::{IntoAnyFn, Ref, value};

fn foo(x: usize, y: &usize, z: &mut usize) {
    *z = x + *y;
}

let x = value(0usize);

foo.into_any_fn()
    .call(&[&value(40usize), &value(2usize), &x])
    .unwrap();

assert_eq!(*x.borrow().downcast_ref::<usize>().unwrap(), 42);
```

### Mutating a `struct`

```rust
use any_fn::{IntoAnyFn, value};

struct Foo {
    foo: usize,
}

fn foo(x: usize, y: &mut Foo) {
    y.foo = x;
}

let x = value(Foo { foo: 0 });

foo.into_any_fn().call(&[&value(42usize), &x]).unwrap();

assert_eq!(x.borrow().downcast_ref::<Foo>().unwrap().foo, 42);
```

## License

[MIT](https://github.com/raviqqe/any-fn/blob/main/LICENSE)
