extern crate cc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {

    let out_path = env::var("OUT_DIR").unwrap();

    let openssl_dir = "/usr/local/opt/openssl/include";

    let bindings = bindgen::Builder::default()
        .whitelisted_type("CryptState")
        .clang_arg("-I/usr/local/opt/openssl/include")
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
        .flag("-O3")
        .compile("libocbaes128_sys.a");

    println!("cargo:rustc-link-lib=crypto");
}
