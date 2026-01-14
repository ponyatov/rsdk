#![allow(unused)]
#![allow(dead_code)]

#[link(name = "rsdk", kind = "static")]
unsafe extern "C" {
    unsafe fn rsdk();
}

mod config;
mod server;
mod vm;

use memmap2::Mmap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{io, process};

fn main() {
    unsafe {
        rsdk();
    }
    std::process::exit(0);
    //
    let argv: Vec<String> = std::env::args().collect();
    let _argc = argv.len();
    arg(0, &argv[0]);
    for (argc, argv) in argv.iter().enumerate().skip(1) {
        arg(argc, argv);
        let file = File::open(Path::new(argv)).unwrap();
        let src = unsafe { Mmap::map(&file).unwrap() };
        eprintln!("\tsize: {} bytes", src.len());
        // eprintln!("{:?}", &mmap[..] as &str);
        io::stdout().write_all(&src[..]).unwrap();
    }
    server::server();
}

fn arg(argc: usize, argv: &str) {
    eprintln!("argv[{argc}] = {argv:?}");
}
