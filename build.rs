fn main() {
    // 1. Build the C++ library (manual prebuild)
    // let dst = cmake::Config::new("rsdk").build();

    // 2. Tell Cargo where to find the library
    // eprintln!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-search=native=bin");
    // 3. Link without 'lib' prefix and '.a' suffix
    println!("cargo:rustc-link-lib=static=rsdk");

    // cross-build supported
    let target = std::env::var("TARGET").unwrap();
    let host = std::env::var("HOST").unwrap();

    eprintln!("TARGET: {}", target);
    eprintln!("HOST: {}", host);

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
