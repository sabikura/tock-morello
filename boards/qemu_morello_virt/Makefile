# Makefile for building the Tock kernel for the qemu-system-morello `virt`
# machine type.

CARGO := crability cargo -Z unstable-options
# The Morello Rust toolchain is not managed by rustup, skip the rustup checks.
NO_RUSTUP := 1
include ../Makefile.common

# QEMU with Morello support, as installed by `crability qemu-install`.
CRABILITY_BIN ?= $(HOME)/.crability/bin
QEMU_CMD      := $(CRABILITY_BIN)/qemu-system-morello

QEMU_BASE_CMDLINE := \
  $(QEMU_CMD) \
    -machine virt \
    -cpu morello \
    -semihosting

# Run the kernel inside a qemu-system-morello "virt" machine type simulation
.PHONY: run
run: $(TARGET_PATH)/release/$(PLATFORM).elf
	@echo
	@echo -e "Running $$($(QEMU_CMD) --version | head -n1) with\n"\
	  " - kernel $<"
	@echo "To exit type C-a x"
	@echo
	$(QEMU_BASE_CMDLINE) \
	  -kernel $< \
	  -nographic
