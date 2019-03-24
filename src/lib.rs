#![crate_name = "cryptonight"]
#![crate_type = "lib"]

#![feature(asm)]
#![feature(repr_simd)]
#![feature(box_syntax)]
#![feature(integer_atomics)]
#![feature(mpsc_select)]

#[macro_use]
extern crate log;

pub mod byte_string;
pub mod hash;
pub mod keccak;
pub mod aes;
pub mod hw_aes;
pub mod sw_aes;
pub mod sse;
pub mod common;
pub mod u64x2;
