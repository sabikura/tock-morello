//! Board file for qemu-system-morello "virt" machine type

#![no_std]
#![no_main]

// Links the arch crate
use aarch64_purecap as _;

pub mod io;

kernel::stack_size! {0x4000}

#[no_mangle]
pub unsafe fn main() {}
