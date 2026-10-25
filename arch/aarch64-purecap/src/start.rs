// NOTE: Most code was copied from [aarch64-purecap](https://github.com/sabikura/aarch64-purecap)

// NOTE(morello): `naked_asm!` does not exist in Rust 1.72. Naked functions are
// written with `asm!` and `options(noreturn)` instead.
use core::arch::asm;

// If the platform boots in A64 mode, this enables first the capability
// instructions, then toggles the instruction set to C64 using the `bx #4`
// instruction (DDI0606 Section 4.4.20).
#[cfg(feature = "hybrid")]
macro_rules! capability_shim {
    () => {
        "
    .code a64
    mrs x0, CPACR_EL1
    // Enable floating point for EL1/0 (issue had in qemu)
    orr x0, x0, #(3 << 20)
    // Enable capability instructions for EL1/0
    orr x0, x0, #(3 << 18)
    msr CPACR_EL1, x0
    isb
    // Toggle the instruction set to C64 and enter purecap
    bx #4
    .code c64
        "
    };
}

// Does nothing for platforms that boot into C64 mode.
#[cfg(not(feature = "hybrid"))]
macro_rules! capability_shim {
    () => {
        ""
    };
}

#[naked]
#[no_mangle]
pub unsafe extern "C" fn initialize_ram_jump_to_main() {
    asm!(
        capability_shim!(),
        "
    // Configure SPSel to be 1 (exceptions use SP_EL1, not SP_EL0)
    // ARM DDI 0487; C5.2.18
    msr SPSel, #1

    // Exceptions taken to EL1 enter in C64 (CCTLR_EL1.C64E)
    mrs x0, CCTLR_EL1
    orr x0, x0, #(1 << 5)
    msr CCTLR_EL1, x0
    isb

    // Configure the stack capability for EL1

    // Compute start address into x0
    adrp    c0, _sstack           // Page address
    add     c0, c0, :lo12:_sstack // Page offset
    gcvalue x0, c0

    // Compute end address into x1
    adrp    c1, _estack
    add     c1, c1, :lo12:_estack
    gcvalue x1, c1

    // Compute the size into x2
    sub x2, x1, x0

    // x3 is the mask for the needed alignment so that a capability of length `x2` is
    // representable
    //
    // start = x0
    // end   = x1
    // size  = x2
    // ~(alignment - 1) = x3
    //
    // The following snippets aligns the start address of the stack to be >= initial address
    // specified in the linker script
    //
    // new_start = (start + alignment - 1) & ~(alignment - 1)
    // new_end   = end & ~(alignment - 1)
    // new_len   = new_end - new_start
    rrmask x3, x2
    mvn x4, x3
    add     x0, x0, x4
    and     x0, x0, x3
    and     x1, x1, x3
    sub     x2, x1, x0

    cvtd    c0, x0
    scbndse c0, c0, x2
    add     c0, c0, x2
    mov     csp, c0

    // Initialize memory

    // Zero BSS out. The Tock linker script only guarantees 4 byte alignment
    // for `_szero` and `_ezero`, so this stores one word at a time. Any
    // non-capability store also clears the capability tag of the memory.
    ldr x0, =_szero
    cvtd c0, x0
    ldr x1, =_ezero
    cvtd c1, x1
100:
    cmp c0, c1
    b.hs 101f
    str wzr, [c0], #4
    b 100b
101:

    // Now initialize .data memory. This involves copying the values right at the
    // end of the .text section (in flash) into the .data section (in RAM).
    ldr x0, =_srelocate         // x0 = first address of data section in RAM
    cvtd c0, x0                 // c0 = DDC derived capability for x0
    ldr x1, =_erelocate         // x1 = first address after data section in RAM
    ldr x2, =_etext             // x2 = address of stored data initial values
    cvtd c2, x2                 // c2 = DDC derived capability for x2

200: // data_init_loop
    cmp x0, x1                  // If we have reached the end of the .data
    b.hs 201f                   // section then we are done.
    ldr w3, [c2], #4            // w3 = *c2, c2 = c2 + 4
    str w3, [c0], #4            // *c0 = w3, c0 = c0 + 4
    b 200b                      // Continue the loop.

201:

    // Initialize the global capabilities from the capability relocation
    // table.
    // After .data is initialized, as the capabilities are written into .data,
    // and before DDC is nulled, as the capabilities are derived from DDC.
    bl {__init_cap_relocs}
    isb
    dsb sy

    // Build the restricted code capability: PCC derived, bounds set to
    // [_spcc, _epcc), permissions reduced to Load, LoadCap, Execute, System,
    // Executive, Global, MutableLoad. The clrperm mask is the same one the
    // compiler uses for executable capabilities (cheri_init_globals.h,
    // 0x13DBC).
    ldr x0, =_spcc
    ldr x1, =_epcc
    sub x2, x1, x0
    cvtp c0, x0
    scbnds c0, c0, x2
    mov x3, #0x3DBC
    movk x3, #0x1, lsl #16
    clrperm c0, c0, x3

    // Configure the EL1 vector table (CVBAR_EL1), see `exceptions.rs`.
    // ARM DDI 0606; 3.2.48
    ldr x4, =_el1_vectors
    scvalue c1, c0, x4
    msr CVBAR_EL1, c1

    // Null DDC. From here on, memory is only reachable through capabilities.
    msr DDC, czr
    isb

    // Enter main through a sealed entry to the restricted code capability.
    // PCC bounds and permissions stay reduced from here on.
    ldr x4, =main
    scvalue c0, c0, x4
    seal c0, c0, rb
    br c0
        ",
        __init_cap_relocs = sym crate::cap_relocs::init_cap_relocs,
        options(noreturn)
    )
}
