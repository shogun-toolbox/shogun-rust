extern crate bindgen;

use cmake::Config;

fn main() {
    println!("cargo:rerun-if-changed=shogun-c/shogun.cpp");
    println!("cargo:rerun-if-changed=shogun-c/shogun.h");
    println!("cargo:rerun-if-changed=CMakeLists.txt");

    let dst = Config::new(".").build();

    println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu/");
    println!("cargo:rustc-link-search=native=/usr/local/lib/");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-search={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=shogun-c");
    println!("cargo:rustc-link-lib=static=shogun");
    println!("cargo:rustc-link-lib=dylib=gomp");
    println!("cargo:rustc-link-lib=dylib=pthread");
    println!("cargo:rustc-link-lib=dylib=lapack");
    println!("cargo:rustc-link-lib=dylib=blas");
    println!("cargo:rustc-link-lib=dylib=glpk");
    println!("cargo:rustc-link-lib=static=z");
    println!("cargo:rustc-link-lib=static=protobuf");
    println!("cargo:rustc-link-lib=static=lzma");
    println!("cargo:rustc-link-lib=static=bz2");
    println!("cargo:rustc-link-lib=dylib=arpack");


    let bindings = bindgen::Builder::default()
        .header("shogun-c/src/shogun.hpp")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
