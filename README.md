# hmr

[![GitHub Action](https://img.shields.io/github/actions/workflow/status/raviqqe/hmr/test.yaml?branch=main&style=flat-square)](https://github.com/raviqqe/hmr/actions)
[![Crate](https://img.shields.io/crates/v/hmr.svg?style=flat-square)](https://crates.io/crates/hmr)
[![License](https://img.shields.io/github/license/raviqqe/hmr.svg?style=flat-square)](LICENSE)

Hot Module Reloading (HMR) for Rust.

# Examples

```rust
use hmr::Module;
use std::fs::write;
use std::thread::sleep;
use std::time::Duration;

const PATH: &'static str = "test/foo.txt";
static FOO: Module = Module::new(PATH);

fn modify_file(content: &str) {
    write(PATH, content).unwrap();
    sleep(Duration::from_millis(100));
}

assert_eq!(&*FOO.load(), "foo\n".as_bytes());

modify_file("Hello, HMR!\n");
assert_eq!(&*FOO.load(), "Hello, HMR!\n".as_bytes());

modify_file("foo\n");
assert_eq!(&*FOO.load(), "foo\n".as_bytes());
```

## License

[MIT](LICENSE)
