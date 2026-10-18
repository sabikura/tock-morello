# Makefile for building the Tock kernel for the qemu-system-morello `virt`
# platform / machine type.

CARGO := crability cargo -Z unstable-options
# The Morello Rust toolchain is not managed by rustup, skip the rustup checks.
NO_RUSTUP := 1
include ../Makefile.common

QEMU_CMD              := qemu-system-morello
# WORKING_QEMU_VERSIONS := 8.2.7, 9.1.3, 9.2.3, 10.0.2
# BROKEN_QEMU_VERSIONS  := <= 8.1.5

# Peripherals attached by default:
# QEMU_BASE_CMDLINE := \
#   $(QEMU_CMD) \
#     -machine virt \
#     -semihosting \
#     -global driver=riscv-cpu,property=smepmp,value=true \
#     -global virtio-mmio.force-legacy=false \
#     -device virtio-rng-device \
#     -device virtio-keyboard-device \
#     $(QEMU_NETDEV_CMDLINE)

# Run the kernel inside a qemu-riscv64-system "virt" machine type simulation
# .PHONY: run
# run: $(TARGET_PATH)/release/$(PLATFORM).elf
# 	@echo
# 	@echo -e "Running $$($(QEMU_CMD) --version | head -n1)"\
# 	  "(tested: $(WORKING_QEMU_VERSIONS); known broken: $(BROKEN_QEMU_VERSIONS)) with\n"\
#           " - kernel $(TARGET_PATH)/release/$(PLATFORM).elf"
# 	@echo "To exit type C-a x"
# 	@echo
# 	$(QEMU_BASE_CMDLINE) \
# 	  -bios $< \
# 	  -nographic
