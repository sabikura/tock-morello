//! Support for the Aarch64 pure capability (Morello) architecture.

#![no_std]
#![feature(cfg_target_abi)]
#![feature(naked_functions)]

#[cfg(all(target_arch = "aarch64", target_abi = "purecap"))]
mod cap_relocs;
#[cfg(all(target_arch = "aarch64", target_abi = "purecap"))]
pub mod exceptions;
#[cfg(all(target_arch = "aarch64", target_abi = "purecap"))]
pub mod start;
#[cfg(all(target_arch = "aarch64", target_abi = "purecap"))]
pub mod support;
pub mod thread_id;
