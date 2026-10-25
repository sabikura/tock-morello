//! EL1 vector table.

// NOTE: The weak default handler setup was copied from
// [aarch64-purecap](https://github.com/sabikura/aarch64-purecap)

core::arch::global_asm!(
    r#"
    .arch morello+c64
    .section .text.aarch64_purecap.vectors, "ax"

    .macro vector_entry handler
    .align 7
    b \handler
    .endm

    .align 11
    .global _el1_vectors
_el1_vectors:
    // Current EL with SP_EL0
    vector_entry _el1_sp0_sync
    vector_entry _el1_sp0_irq
    vector_entry _el1_sp0_fiq
    vector_entry _el1_sp0_serror

    // Current EL with SP_EL1
    vector_entry _el1_sync
    vector_entry _el1_irq
    vector_entry _el1_fiq
    vector_entry _el1_serror

    // Lower EL, AArch64
    vector_entry _el0_sync
    vector_entry _el0_irq
    vector_entry _el0_fiq
    vector_entry _el0_serror

    // Lower EL, AArch32
    vector_entry _el0_aarch32_sync
    vector_entry _el0_aarch32_irq
    vector_entry _el0_aarch32_fiq
    vector_entry _el0_aarch32_serror

    // Pad the last entry to its full size, so that the table is 0x800 bytes.
    .align 7
    .size _el1_vectors, . - _el1_vectors

    // Default handler for every entry: hang. It lives after the table, so
    // that the alignment padding is at the end of the section.
    .global _el1_default_handler
    .type _el1_default_handler, %function
_el1_default_handler:
    b .

    // Define every handler symbol as a weak alias of the default handler.
    .macro weak_handler name
    .weak \name
    .set \name, _el1_default_handler
    .endm

    weak_handler _el1_sp0_sync
    weak_handler _el1_sp0_irq
    weak_handler _el1_sp0_fiq
    weak_handler _el1_sp0_serror

    weak_handler _el1_sync
    weak_handler _el1_irq
    weak_handler _el1_fiq
    weak_handler _el1_serror

    weak_handler _el0_sync
    weak_handler _el0_irq
    weak_handler _el0_fiq
    weak_handler _el0_serror

    weak_handler _el0_aarch32_sync
    weak_handler _el0_aarch32_irq
    weak_handler _el0_aarch32_fiq
    weak_handler _el0_aarch32_serror
    "#
);
