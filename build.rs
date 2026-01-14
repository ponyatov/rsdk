// extern crate cmake;

fn main() {
    let lib = cmake::Config::new("rsdk").build();
    eprintln!("cargo:rustc-link-search=native={}", lib.display());
    // // 1. Build the C++ library
    // let dst = Config::new("libfoo++").build();

    // // 2. Tell Cargo where to find the library
    // println!("cargo:rustc-link-search=native={}", dst.display());
    // // 3. Link the static library
    // println!("cargo:rustc-link-lib=static=foo++");

    // // 4. Link the C++ standard library (platform-specific)
    // let target = std::env::var("TARGET").unwrap();
    // if target.contains("apple") {
    //     println!("cargo:rustc-link-lib=dylib=c++");
    // } else if target.contains("linux") {
    //     println!("cargo:rustc-link-lib=dylib=stdc++");
    // }
    // // Add handling for other platforms (e.g., windows) as needed.
}
