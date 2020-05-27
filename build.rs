extern crate bindgen;

use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() { 
    println!("cargo:rerun-if-changed=shogun/shogun.cpp");
    println!("cargo:rerun-if-changed=shogun/shogun.h");
    println!("cargo:rerun-if-changed=CMakeLists.txt");

    let dst = Config::new(".")
        .build();

    println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu/");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-search={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=shogun-c");
    println!("cargo:rustc-link-search=/home/gf712/shogun/build/src/shogun");
    println!("cargo:rustc-env=LD_LIBRARY_PATH=/home/gf712/shogun/build/src/shogun");
    println!("cargo:rustc-link-search=/home/gf712/shogun/build/spdlog/src/SpdLog-build/");
    println!("cargo:rustc-link-lib=dylib=shogun");
    println!("cargo:rustc-link-lib=static=spdlog");

    let bindings = bindgen::Builder::default()
        .clang_arg("-Ishogun")
        .header("shogun/shogun.hpp")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}