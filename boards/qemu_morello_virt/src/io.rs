// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2026.

use core::panic::PanicInfo;

/// Panic handler.
#[cfg(not(test))]
#[panic_handler]
pub unsafe fn panic_fmt(_pi: &PanicInfo) -> ! {
    // debug::panic_print::<qemu_rv64_virt_chip::uart::UartPanicWriter, _, _>(
    //     qemu_rv64_virt_chip::uart::UartPanicWriterConfig {
    //         params: uart::Parameters {
    //             baud_rate: 115200,
    //             stop_bits: uart::StopBits::One,
    //             parity: uart::Parity::None,
    //             hw_flow_control: false,
    //             width: uart::Width::Eight,
    //         },
    //     },
    //     pi,
    //     &rv64i::support::nop,
    //     crate::PANIC_RESOURCES.get(),
    // );

    // The system is no longer in a well-defined state. Use
    // semihosting commands to exit QEMU with a return code of 1.
    // crate::qemu_exit(1)
    loop {}
}
