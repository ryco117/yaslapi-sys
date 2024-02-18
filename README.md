# yaslapi-sys
yaslapi-sys is a Rust library that provides raw bindings to the [Yet Another Scripting Language (YASL)](https://github.com/yasl-lang/yasl) API.

## Installation
First, you must have CMake and a C compiler installed so that YASL can be compiled locally.
To install yaslapi-sys, add the following to your `Cargo.toml` file:

```toml
[dependencies]
yaslapi-sys = "0.2.4"
```

Then run cargo build to build your project.

## Usage
Here’s an example of how to use yaslapi-sys in your Rust code:

```rust
use yaslapi_sys::YASL_State;
use std::ffi::CString;

fn main() {
    const SRC: &str = "let x = 5; echo x**x;";
    let state: *mut YASL_State = unsafe { yaslapi_sys::YASL_newstate_bb(SRC.as_ptr().cast(), SRC.len()) };
    assert!(!state.is_null());
    // ...
}
```

## License
yaslapi-sys is licensed under the [MIT License](/LICENSE).
