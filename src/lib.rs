//! Rust API for ARM Base ISA A32
//!
//! This crate provides:
//!
//! - Safe wrappers around assembly instructions
//!
//! For now, there's not much. I will update it gradually.
//! If you want to contribute, feel free to reach out!

#![no_std]
#![cfg_attr(feature = "inline-asm", feature(asm))]
#![feature(core_intrinsics)]

pub mod register;
