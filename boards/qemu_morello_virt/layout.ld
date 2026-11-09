/**
 * QEMU Morello `virt` machine DRAM region, which starts at 0x40000000 (128MB
 * by default). Tock is loaded at the start of DRAM as an ELF, using the
 * `-kernel` option of qemu-system-morello.
 *
 * We are using 4MB of RAM, which easily fits into the 128MB default
 * assignment of QEMU.
 */

MEMORY
{
  rom (rx)  : ORIGIN = 0x40000000, LENGTH = 0x100000
  prog (rx) : ORIGIN = 0x40100000, LENGTH = 0x100000
  ram (rwx) : ORIGIN = 0x40200000, LENGTH = 0x200000
}

SECTIONS {
    /* Export the start & end of SRAM and flash as symbols. */
    _sflash = ORIGIN(rom);
    _eflash = ORIGIN(prog) + LENGTH(prog);

    _ssram  = ORIGIN(ram);
    _esram  = ORIGIN(ram) + LENGTH(ram);

    /* Capability relocation table */
    .cap_relocs :
    {
        _scap_relocs = .;
        KEEP(*(__cap_relocs))
        KEEP(*(.__cap_relocs))
        _ecap_relocs = .;
    } > rom

    /* Capability slots for globals (the GOT and relocated read-only data).
     *
     * Code reads these PC-relative, so they have to be inside the PCC region
     * below
     */
    .got :
    {
        *(.got .got.plt)
    } > rom

    .data.rel.ro :
    {
        *(.data.rel.ro .data.rel.ro.*)
    } > rom

    /* Read-only data gets its own section instead of the Tock default of
     * living inside .text.
     * The Morello linker puts the types of capability relocation
     * into the table by its section, and treats everything in an executable
     * section as a function
     */
    .rodata :
    {
        *(.rodata .rodata.* .gnu.linkonce.r.*)
    } > rom

    /* PCC region.
     *
     * The start code restricts PCC to [_spcc, _epcc), which covers everything
     * placed in `rom` up to `_etext`: the sections above and .text.
     * `_etext` is aligned to PAGE_SIZE, so the bounds are representable.
     */
    _spcc = ORIGIN(rom);
    _epcc = _etext;
}

INCLUDE tock_kernel_layout.ld

/* NOTE: The Morello lld treats every `__*_start` symbol as a
 * section start symbol and asserts if it does not sit at the start of its
 * section. The Tock linker script defines the (unused, Rust has no
 * constructors) init/fini array symbols in the middle of `.text`, so redefine
 * them here as absolute values, which clears that flag.
 */
__preinit_array_start = 0;
__preinit_array_end = 0;
__init_array_start = 0;
__init_array_end = 0;
__fini_array_start = 0;
__fini_array_end = 0;
