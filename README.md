# yaslapi-sys
yaslapi-sys is a Rust library that provides bindings to the [Yet Another Scripting Language (YASL)](https://github.com/yasl-lang/yasl) API.

## Installation
First, you must have YASL [installed on your system](https://github.com/yasl-lang/yasl/blob/master/README.md#Installation) to use this crate.
To install yaslapi-sys, add the following to your `Cargo.toml` file:

```toml
[dependencies]
yaslapi-sys = "0.1.0"
```

Then run cargo build to build your project.

## Usage
Here’s an example of how to use yaslapi-sys in your Rust code:

```rust
extern crate yaslapi_sys;

use yaslapi_sys::YASL_State;
use std::ffi::CString;

fn main() {
    let test_file = CString::new("test.yasl").unwrap();
    let state: *mut YASL_State = unsafe { yaslapi_sys::YASL_newstate(test_file.as_ptr()) };
    assert!(!state.is_null());
    // ...
}
```

## License
yaslapi-sys is licensed under the [MIT License](/LICENSE).
