//! The PL011 UART of the qemu-system-morello virt machine

use core::cell::Cell;

use kernel::ErrorCode;
use kernel::deferred_call::{DeferredCall, DeferredCallClient};
use kernel::hil;
use kernel::utilities::cells::{OptionalCell, TakeCell};
use primecell::pl011::Pl011Registers;

/// Address of the first PL011 (`pl011` in QEMU's `info mtree`)
pub const UART0_BASE_ADDRESS: usize = 0x0900_0000;

/// UARTCLK of the `virt` machine, `apb-pclk` in its device tree
pub const UART0_CLOCK_FREQUENCY: u32 = 24_000_000;

pub struct Uart<'a> {
    registers: &'static Pl011Registers,
    tx_client: OptionalCell<&'a dyn hil::uart::TransmitClient>,
    rx_client: OptionalCell<&'a dyn hil::uart::ReceiveClient>,
    tx_buffer: TakeCell<'static, [u8]>,
    tx_len: Cell<usize>,
    deferred_call: DeferredCall,
}

impl<'a> Uart<'a> {
    pub fn new(registers: &'static Pl011Registers) -> Uart<'a> {
        Uart {
            registers,
            tx_client: OptionalCell::empty(),
            rx_client: OptionalCell::empty(),
            tx_buffer: TakeCell::empty(),
            tx_len: Cell::new(0),
            deferred_call: DeferredCall::new(),
        }
    }

    pub fn configure(&self, params: hil::uart::Parameters) -> Result<(), ErrorCode> {
        self.registers.configure(params, UART0_CLOCK_FREQUENCY)
    }
}

// Deferred call until the interrupt support is added
impl DeferredCallClient for Uart<'_> {
    fn register(&'static self) {
        self.deferred_call.register(self)
    }

    fn handle_deferred_call(&self) {
        self.tx_client.map(|client| {
            self.tx_buffer.take().map(|buffer| {
                client.transmitted_buffer(buffer, self.tx_len.get(), Ok(()));
            });
        });
    }
}

impl<'a> hil::uart::Transmit<'a> for Uart<'a> {
    fn set_transmit_client(&self, client: &'a dyn hil::uart::TransmitClient) {
        self.tx_client.set(client);
    }

    fn transmit_buffer(
        &self,
        tx_buffer: &'static mut [u8],
        tx_len: usize,
    ) -> Result<(), (ErrorCode, &'static mut [u8])> {
        if self.tx_buffer.is_some() {
            return Err((ErrorCode::BUSY, tx_buffer));
        }
        if tx_len == 0 || tx_len > tx_buffer.len() {
            return Err((ErrorCode::SIZE, tx_buffer));
        }

        self.registers.transmit_sync(&tx_buffer[..tx_len]);

        self.tx_len.set(tx_len);
        self.tx_buffer.replace(tx_buffer);
        self.deferred_call.set();
        Ok(())
    }

    fn transmit_word(&self, _word: u32) -> Result<(), ErrorCode> {
        Err(ErrorCode::FAIL)
    }

    fn transmit_abort(&self) -> Result<(), ErrorCode> {
        if self.tx_buffer.is_some() {
            Err(ErrorCode::BUSY)
        } else {
            Ok(())
        }
    }
}

impl hil::uart::Configure for Uart<'_> {
    fn configure(&self, params: hil::uart::Parameters) -> Result<(), ErrorCode> {
        self.registers.configure(params, UART0_CLOCK_FREQUENCY)
    }
}

// FIXME: receiving needs the UART interrupt
impl<'a> hil::uart::Receive<'a> for Uart<'a> {
    fn set_receive_client(&self, client: &'a dyn hil::uart::ReceiveClient) {
        self.rx_client.set(client);
    }

    fn receive_buffer(
        &self,
        rx_buffer: &'static mut [u8],
        _rx_len: usize,
    ) -> Result<(), (ErrorCode, &'static mut [u8])> {
        Err((ErrorCode::NOSUPPORT, rx_buffer))
    }

    fn receive_word(&self) -> Result<(), ErrorCode> {
        Err(ErrorCode::NOSUPPORT)
    }

    fn receive_abort(&self) -> Result<(), ErrorCode> {
        Ok(())
    }
}
