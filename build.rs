#![allow(unused)]

use std::env;
use std::path::PathBuf;

fn main() {
    // cross-build supported
    let target = std::env::var("TARGET").unwrap();
    let host = std::env::var("HOST").unwrap();

    eprintln!("TARGET: {}", target);
    eprintln!("HOST: {}", host);

    // 1. Build the C/C++ library
    // let dst = cmake::Config::new("rsdk").build(); // commented: manual cmake
    cc::Build::new()
        .file("src/rsdk.cpp")
        .include("inc")
        .compile("rsdk");

    // 2. tell cargo where to find libraries
    // eprintln!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-search=native=bin"); // static
    println!("cargo:rustc-link-search=bin"); // shared
    // 3. tell rustc what to link (without 'lib' prefix and '.a' suffix)
    println!("cargo:rustc-link-lib=static=rsdk");

    // using bindgen for wrapper modules generation
    let mut bindings = bindgen::Builder::default()
        // input collects all headers
        .header("inc/rsdk.hpp")
        // invalidate the built crate on any .h changed
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));
    //
    bindings = bindings
        .blocklist_file("stdlib.h")
        .blocklist_file("stdio.h")
        .blocklist_file("cstdlib")
        .blocklist_file("cstdio");
    //
    if target.contains("linux") {
        bindings = bindings.clang_args(&["-DLINUX", "-DI5"]);
    }
    //
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .use_core()
        .clang_arg("-DBINDGEN")
        .generate()
        .expect("generate")
        .write_to_file(out_path.join("rsdk.rs"))
        .expect("write_to_file");

    // you might need to link C++ standard library
    if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    } else if target.contains("windows") {
        // println!("cargo:rustc-link-lib=dylib=msvcrt");
        println!("cargo:rustc-link-lib=static=stdc++");
        println!("cargo:rustc-link-lib=static=gcc");
        println!("cargo:rustc-link-lib=static=pthread");
        println!("cargo:rustc-link-lib=dylib=m");
    } else {
        std::process::abort();
    }

    // watch & rebuild
    println!("cargo:rerun-if-changed=bin/librsdk.a");
    // println!("cargo:rerun-if-changed=bin/rsdk.lib"); // Windows
    // println!("cargo:rerun-if-changed=bin/");
}
