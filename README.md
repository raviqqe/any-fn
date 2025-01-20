# any-fn

[![GitHub Action](https://img.shields.io/github/actions/workflow/status/raviqqe/any-fn/test.yaml?branch=main&style=flat-square)](https://github.com/raviqqe/any-fn/actions)
[![Crate](https://img.shields.io/crates/v/any-fn.svg?style=flat-square)](https://crates.io/crates/any-fn)
[![License](https://img.shields.io/github/license/raviqqe/any-fn.svg?style=flat-square)](https://github.com/raviqqe/any-fn/blob/main/LICENSE)

Dynamically-typed functions via [`core::any::Any`](https://doc.rust-lang.org/stable/core/any/trait.Any.html) in Rust.

## Examples

### Calling a function with unboxed arguments

```rust
use any_fn::{IntoAnyFn, value};

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
```

### Calling a function with mutable reference arguments

```rust
use any_fn::{IntoAnyFn, value};

fn foo(x: usize, y: &mut usize) {
    *y = x;
}

let x = value(0usize);

foo.into_any_fn().call(&[&value(42usize), &x]).unwrap();

assert_eq!(*x.borrow().downcast_ref::<usize>().unwrap(), 42);
```

### Calling a function with unboxed, immutable reference, and mutable reference arguments

```rust
use any_fn::{IntoAnyFn, Ref, value};

fn foo(x: usize, y: &usize, z: &mut usize) {
    *z = x + *y;
}

let x = value(0usize);

<_ as IntoAnyFn<'_, (_, Ref<usize>, _), _>>::into_any_fn(foo)
    .call(&[&value(40usize), &value(2usize), &x])
    .unwrap();

assert_eq!(*x.borrow().downcast_ref::<usize>().unwrap(), 42);
```

## License

[MIT](https://github.com/raviqqe/any-fn/blob/main/LICENSE)
