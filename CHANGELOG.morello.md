# Arm Morello support Changelog

This file tracks the changes added to the upstream Tock by this fork.
Currently forked from upstream commit
[ff8a680c146c1ad3a25b995219c07eacb5aa5c47](https://github.com/tock/tock/tree/ff8a680c146c1ad3a25b995219c07eacb5aa5c47).

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),

## 2026-10-18

### Changed

- *(`kernel`)*: Modified Cargo manifest to **Edition 2021** instead of 2024.
- *(`tock-cells`)*: Modified Cargo manifest to **Edition 2021** instead of 2024.
- *(`tock-tbf`)*: Modified Cargo manifest to **Edition 2021** instead of 2024.
- *(`tock_build_scripts`)*: Modified Cargo manifest to **Edition 2021** instead of 2024.
- *(`capsules-core`)*: Modified Cargo manifest to **Edition 2021** instead of 2024.
- *(`capsules-system`)*: Modified Cargo manifest to **Edition 2021** instead of 2024.
- *(`enum_primitive`)*: Modified Cargo manifest to **Edition 2021** instead of 2024.
- *(`kernel`)*: `unsafe` attribute wrappers removed from `link_section`, `no_mangle`
  and `export_name` macros.
- *(`kernel`)*: The `panic_banner()` function has a `#[cfg(morello_toolchain)]` variant since
 the `PanicInfo::message` method returns an `Option` in Rust 1.72.
- *(`kernel`)*: `is_multiple_of` replaced with `%`.
- *(`kernel`)*: `NonNull::from_mut` replaced with `NonNull::from`.
- *(`kernel`)*: Rewrote `let` chains with nested `if`s.
- *(`tock-tbf`)*: `is_multiple_of` replaced with `%`.
- *(`capsules-core`)*: `is_multiple_of` replaced with `%`.
- *(`capsules-core`)*: `is_none_or` replaced with `map_or(true, ..)`.
- Root Cargo manifest ignores warnings about unexpected `morello_toolchain` cfg

### Added

- *(tools)*: Scripts inside the `tools/morello-toolchain` directory used for building with the custom toolchain 
- *(boards/qemu_morello_virt)*: Empty crate setup for the `qemu_morello_virt` board (and separate workspace)
- *(chips/qemu_morello_virt_chip)*: Empty crate setup for the `qemu_morello_virt_chip` chip
- *(chips/qemu_morello_virt_chip)*: `.cargo/config.toml` symlink to the board's cargo config
- *(arch/aarch64-purecap)*: Empty crate setup for the `aarch64-purecap` architecture
- *(arch/aarch64-purecap)*: `.cargo/config.toml` symlink to the board's cargo config
