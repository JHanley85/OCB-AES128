extern crate cc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {

    let out_path = env::var("OUT_DIR").unwrap();

    let openssl_dir = "C:\\vcpkg\\installed\\x64-windows\\include";

    let bindings = bindgen::Builder::default()
        .whitelisted_type("CryptState")
        .clang_arg(format!("-I{}", openssl_dir))
        .header("cpp/Wrapper.hpp")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(out_path);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .file("cpp/CryptState.cpp")
        .flag(&format!("-I{}", openssl_dir))
        .compile("libocbaes128_sys.a");

    println!("cargo:rustc-link-search=native=C:\\vcpkg\\installed\\x64-windows\\lib");
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=gdi32");
    println!("cargo:rustc-link-lib=libeay32");
    println!("cargo:rustc-link-lib=ssleay32");
}

