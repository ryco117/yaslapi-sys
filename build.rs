use std::env;
use std::path::PathBuf;

fn main() {
    let dst = cmake::Config::new("yasl").build_target("yaslapi").build();
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("build").display()
    );

    // Tell rustc to link compiled yaslapi library.
    println!("cargo:rustc-link-lib=static=yaslapi");

    // Tell cargo to invalidate the built crate whenever the yaslapi header changes.
    println!("cargo:rerun-if-changed=yaslapi.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("yaslapi.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
