//! The Tock board components that build with the Morello toolchain.
//!
//! The `boards/components` crate depends on `capsules-extra` and `segger`,
//! which the Morello toolchain cannot compile.

#![forbid(unsafe_code)]
#![no_std]

#[path = "../../../components/src/console.rs"]
pub mod console;
#[path = "../../../components/src/debug_writer.rs"]
pub mod debug_writer;
