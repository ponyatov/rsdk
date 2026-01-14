// extern crate cmake;

fn main() {
    // 1. Build the C++ library (manual prebuild)
    // let lib = cmake::Config::new("rsdk").build();
    // let dst = Config::new("libfoo++").build();

    // 2. Tell Cargo where to find the library
    // eprintln!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-search=native=bin");
    // 3. Link without 'lib' prefix and '.a' suffix
    println!("cargo:rustc-link-lib=static=rsdk");

    // On some platforms, you might need to link C++ standard library
    let target = std::env::var("TARGET").unwrap();
    if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
}
