//! shared config

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

pub mod server {
    pub const ip: &str = "127.0.0.1";
    // pub const IP: &str = "0.0.0.0";
    pub const port: u16 = 12345;
    /// bind address constant
    pub const bind: &str = const_format::formatcp!("{ip}:{port}");
}
