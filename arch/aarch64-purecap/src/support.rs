//! Support functions for the Aarch64 pure capability architecture.

use cheri::{prelude::*, ptr::Perms};
use core::arch::asm;

// NOTE: Copied from cortex-m support
/// NOP instruction
#[cfg(all(target_arch = "arm", target_os = "none"))]
#[inline(always)]
pub fn nop() {
    use core::arch::asm;

    // # Safety
    //
    // - INPUTS: This does not use the existing value of any registers.
    // - OUTPUTS: This does not write any registers.
    // - Options set:
    //   - nomem: We do not read or write memory.
    //   - nostack: This does not use the stack.
    //   - preserves_flags: This does not change flags.
    // - Options not set:
    //   - pure: not required
    //   - readonly: implied by nomem
    //   - noreturn: we do fall-through
    //   - att_syntax: not on arm
    //   - raw: not required
    unsafe {
        asm!("nop", options(nomem, nostack, preserves_flags));
    }
}

/// Semihosting call
///
/// `command` is the semihosting operation number and `param` its parameter
/// (for most 64-bit operations, the address of a parameter block). The result
/// of the operation is returned. For example, to exit QEMU with code 0:
///
/// ```ignore
/// let block = [0x20026, 0]; // ADP_Stopped_ApplicationExit, exit code
/// semihost_command(0x18, block.as_ptr() as usize); // SYS_EXIT
/// ```
pub unsafe fn semihost_command(command: usize, param: usize) -> usize {
    let res;
    asm!(
        "hlt #0xf000",
        inout("x0") command => res, // x0 holds the command (and the result)
        in("x1") param,             // x1 holds the parameter
    );
    res
}

/// Null the current's EL DDC. This should be called by the board after setting up the drivers.
pub unsafe fn null_ddc() {
    asm!(
        "isb",
        "msr DDC, czr",
        "isb",
        options(nomem, nostack, preserves_flags)
    );
}

/// Derive a reference to the registers block of type `T` at `address` from the DDC
pub unsafe fn mmio<T>(address: usize) -> &'static T {
    let ddc = cheri::ptr::default_data_mut::<u8>();
    let registers = ddc
        .wrapping_add(address.wrapping_sub(ddc as usize))
        .cast::<T>()
        .with_bounds(core::mem::size_of::<T>())
        .with_perms_clear_except(Perms::LOAD | Perms::STORE | Perms::GLOBAL);
    &*registers
}
