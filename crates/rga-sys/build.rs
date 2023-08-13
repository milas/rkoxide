use std::env;
use std::path::PathBuf;

fn main() {
    let libdir_path = PathBuf::from("rga")
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()
        .expect("cannot canonicalize path");

    let headers_path = libdir_path.join("RgaApi.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    // Tell cargo to look for shared libraries in the specified directory
    println!(
        "cargo:rustc-link-search={}",
        libdir_path.join("build").to_str().expect("invalid path")
    );

    println!("cargo:rustc-link-lib={}", "rga");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed={}", headers_path_str);

    if !std::process::Command::new("meson")
        .arg("build")
        .current_dir(libdir_path.clone())
        .output()
        .expect("could not spawn `meson`")
        .status
        .success()
    {
        panic!("could not setup build for rga");
    }

    if !std::process::Command::new("ninja")
        .current_dir(libdir_path.join("build"))
        .output()
        .expect("could not spawn `ninja`")
        .status
        .success()
    {
        panic!("could not compile rga");
    }

    let bindings = bindgen::Builder::default()
        .header(headers_path_str)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
