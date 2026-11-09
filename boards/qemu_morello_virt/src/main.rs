//! Board file for qemu-system-morello "virt" machine type

#![no_std]
#![no_main]

use kernel::component::Component;
use kernel::deferred_call::{DeferredCall, DeferredCallClient};
use kernel::{capabilities, create_capability, debug, static_init};
use qemu_morello_virt_chip::uart::{UART0_BASE_ADDRESS, Uart};

pub mod io;

type ThreadIdProvider = aarch64_purecap::thread_id::ThreadIdProvider;

kernel::stack_size! {0x4000}

const UART_BAUD_RATE: u32 = 115200;

#[no_mangle]
pub unsafe fn main() {
    kernel::deferred_call::initialize_deferred_call_state::<ThreadIdProvider>();

    let uart0_registers = aarch64_purecap::support::mmio(UART0_BASE_ADDRESS);
    let uart0 = static_init!(Uart<'static>, Uart::new(uart0_registers));
    uart0.register();

    aarch64_purecap::support::null_ddc();

    let uart_mux = components::console::UartMuxComponent::new(uart0, UART_BAUD_RATE)
        .finalize(components::uart_mux_component_static!());

    components::debug_writer::DebugWriterComponent::new::<ThreadIdProvider>(
        uart_mux,
        create_capability!(capabilities::SetDebugWriterCapability),
    )
    .finalize(components::debug_writer_component_static!());

    debug!("Tock on the QEMU Morello \"virt\" machine");

    // No kernel loop yet
    loop {
        while DeferredCall::has_tasks() {
            DeferredCall::service_next_pending();
        }
    }
}
