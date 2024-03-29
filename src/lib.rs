//! # yaslapi-sys
//! yaslapi-sys is a Rust library that provides raw bindings to the [Yet Another Scripting Language (YASL)](https://github.com/yasl-lang/yasl) API.
//!
//! Then run cargo build to build your project.
//!
//! ## Usage
//! Here’s an example of how to use yaslapi-sys in your Rust code:
//!
//! ```
//! use yaslapi_sys::YASL_State;
//! use std::ffi::CString;
//!
//! const SRC: &str = "let x = 5; echo x**x;";
//! let state: *mut YASL_State = unsafe { yaslapi_sys::YASL_newstate_bb(SRC.as_ptr().cast(), SRC.len()) };
//! assert!(!state.is_null());
//! ```

#![allow(
    clippy::unreadable_literal,
    improper_ctypes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    fn assert_success(r: i32) {
        assert_eq!(r, YASL_Error_YASL_SUCCESS as i32)
    }

    #[no_mangle]
    unsafe extern "C" fn rust_print_internal(_state: *mut YASL_State) -> i32 {
        println!("This is a test");

        // Return the number of return values pushed to the YASL stack.
        0
    }

    #[test]
    fn test_basic_functionality() {
        unsafe {
            // Initialize test script
            let state = {
                let test_file = CString::new("test.yasl").unwrap();
                let state = YASL_newstate(test_file.as_ptr());
                assert!(!state.is_null());
                state
            };

            // Init new variable `answer` with the top of the stack (in this case, the `42` we just pushed)
            // Push `42` onto the stack
            YASL_pushint(state, 42);

            let var_name = CString::new("answer").unwrap();
            YASLX_initglobal(state, var_name.as_ptr());

            YASL_pushcfunction(state, Some(rust_print_internal), 0);
            let var_name = CString::new("rust_print").unwrap();
            YASLX_initglobal(state, var_name.as_ptr());

            // Execute `test.yasl`, now that we're done setting everything up
            assert_success(YASL_execute(state));

            // Clean up
            assert_success(YASL_delstate(state));
        }
    }
}
