//! Arm PrimeCell UART (PL011) registers and the operations on them.
//!
//! The register definitions are copied from the `pl011-uart` crate
//! (<https://github.com/sabikura/pl011-uart>)
//!
//! Documentation: ARM PrimeCell UART (PL011) Technical Reference Manual
//! (ARM DDI 0183G), Chapter 3.

// NOTE(AI): Aided by Claude Fable 5.1

use kernel::ErrorCode;
use kernel::hil;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable, Writeable};
use kernel::utilities::registers::{
    ReadOnly, ReadWrite, WriteOnly, register_bitfields, register_structs,
};

register_bitfields![u32,
    /// Data Register, UARTDR.
    pub DR [
        /// Overrun error. This bit is set to 1 if data is received and the
        /// receive FIFO is already full.
        OE OFFSET(11) NUMBITS(1) [],
        /// Break error. This bit is set to 1 if a break condition was
        /// detected, indicating that the received data input was held LOW for
        /// longer than a full-word transmission time (defined as start, data,
        /// parity and stop bits).
        BE OFFSET(10) NUMBITS(1) [],
        /// Parity error. When set to 1, it indicates that the parity of the
        /// received data character does not match the parity that the EPS and
        /// SPS bits in the Line Control Register, UARTLCR_H select.
        PE OFFSET(9) NUMBITS(1) [],
        /// Framing error. When set to 1, it indicates that the received
        /// character did not have a valid stop bit (a valid stop bit is 1).
        FE OFFSET(8) NUMBITS(1) [],
        /// Receive (read) data character. Transmit (write) data character.
        DATA OFFSET(0) NUMBITS(8) []
    ],

    /// Receive Status Register / Error Clear Register, UARTRSR/UARTECR.
    pub RSR_ECR [
        /// Overrun error.
        OE OFFSET(3) NUMBITS(1) [],
        /// Break error.
        BE OFFSET(2) NUMBITS(1) [],
        /// Parity error.
        PE OFFSET(1) NUMBITS(1) [],
        /// Framing error.
        FE OFFSET(0) NUMBITS(1) []
    ],

    /// Flag Register, UARTFR.
    pub FR [
        /// Ring indicator.
        RI OFFSET(8) NUMBITS(1) [],
        /// Transmit FIFO empty.
        TXFE OFFSET(7) NUMBITS(1) [],
        /// Receive FIFO full.
        RXFF OFFSET(6) NUMBITS(1) [],
        /// Transmit FIFO full.
        TXFF OFFSET(5) NUMBITS(1) [],
        /// Receive FIFO empty.
        RXFE OFFSET(4) NUMBITS(1) [],
        /// UART busy. If this bit is set to 1, the UART is busy transmitting
        /// data.
        BUSY OFFSET(3) NUMBITS(1) [],
        /// Data carrier detect.
        DCD OFFSET(2) NUMBITS(1) [],
        /// Data set ready.
        DSR OFFSET(1) NUMBITS(1) [],
        /// Clear to send.
        CTS OFFSET(0) NUMBITS(1) []
    ],

    /// IrDA Low-Power Counter Register, UARTILPR.
    pub ILPR [
        /// 8-bit low-power divisor value.
        ILPDVSR OFFSET(0) NUMBITS(8) []
    ],

    /// Integer Baud Rate Register, UARTIBRD.
    pub IBRD [
        /// The integer baud rate divisor.
        BAUD_DIVINT OFFSET(0) NUMBITS(16) []
    ],

    /// Fractional Baud Rate Register, UARTFBRD.
    pub FBRD [
        /// The fractional baud rate divisor.
        BAUD_DIVFRAC OFFSET(0) NUMBITS(6) []
    ],

    /// Line Control Register, UARTLCR_H.
    pub LCR_H [
        /// Stick parity select.
        SPS OFFSET(7) NUMBITS(1) [],
        /// Word length.
        WLEN OFFSET(5) NUMBITS(2) [
            FiveBit = 0b00,
            SixBit = 0b01,
            SevenBit = 0b10,
            EightBit = 0b11
        ],
        /// Enable FIFOs.
        FEN OFFSET(4) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        /// Two stop bits select.
        STP2 OFFSET(3) NUMBITS(1) [],
        /// Even parity select.
        EPS OFFSET(2) NUMBITS(1) [
            Odd = 0,
            Even = 1
        ],
        /// Parity enable.
        PEN OFFSET(1) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        /// Send break.
        BRK OFFSET(0) NUMBITS(1) []
    ],

    /// Control Register, UARTCR.
    pub CR [
        /// CTS hardware flow control enable.
        CTSEN OFFSET(15) NUMBITS(1) [],
        /// RTS hardware flow control enable.
        RTSEN OFFSET(14) NUMBITS(1) [],
        /// This bit is the complement of the UART Out2 (nUARTOut2) modem
        /// status output.
        OUT2 OFFSET(13) NUMBITS(1) [],
        /// This bit is the complement of the UART Out1 (nUARTOut1) modem
        /// status output.
        OUT1 OFFSET(12) NUMBITS(1) [],
        /// Request to send.
        RTS OFFSET(11) NUMBITS(1) [],
        /// Data transmit ready.
        DTR OFFSET(10) NUMBITS(1) [],
        /// Receive enable.
        RXE OFFSET(9) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        /// Transmit enable.
        TXE OFFSET(8) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        /// Loopback enable.
        LBE OFFSET(7) NUMBITS(1) [],
        /// SIR low-power IrDA mode.
        SIRLP OFFSET(2) NUMBITS(1) [],
        /// SIR enable.
        SIREN OFFSET(1) NUMBITS(1) [],
        /// UART enable.
        UARTEN OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],

    /// Interrupt FIFO Level Select Register, UARTIFLS.
    pub IFLS [
        /// Receive interrupt FIFO level select.
        RXIFLSEL OFFSET(3) NUMBITS(3) [
            OneEighth = 0b000,
            OneQuarter = 0b001,
            OneHalf = 0b010,
            ThreeQuarters = 0b011,
            SevenEighths = 0b100
        ],
        /// Transmit interrupt FIFO level select.
        TXIFLSEL OFFSET(0) NUMBITS(3) [
            OneEighth = 0b000,
            OneQuarter = 0b001,
            OneHalf = 0b010,
            ThreeQuarters = 0b011,
            SevenEighths = 0b100
        ]
    ],

    /// Interrupt Mask Set/Clear Register, UARTIMSC.
    /// UARTRIS, UARTMIS and UARTICR share the same layout
    pub INTERRUPTS [
        /// Overrun error interrupt.
        OEI OFFSET(10) NUMBITS(1) [],
        /// Break error interrupt.
        BEI OFFSET(9) NUMBITS(1) [],
        /// Parity error interrupt.
        PEI OFFSET(8) NUMBITS(1) [],
        /// Framing error interrupt.
        FEI OFFSET(7) NUMBITS(1) [],
        /// Receive timeout interrupt.
        RTI OFFSET(6) NUMBITS(1) [],
        /// Transmit interrupt.
        TXI OFFSET(5) NUMBITS(1) [],
        /// Receive interrupt.
        RXI OFFSET(4) NUMBITS(1) [],
        /// nUARTDSR modem interrupt.
        DSRMI OFFSET(3) NUMBITS(1) [],
        /// nUARTDCD modem interrupt.
        DCDMI OFFSET(2) NUMBITS(1) [],
        /// nUARTCTS modem interrupt.
        CTSMI OFFSET(1) NUMBITS(1) [],
        /// nUARTRI modem interrupt.
        RIMI OFFSET(0) NUMBITS(1) []
    ],

    /// DMA Control Register, UARTDMACR.
    pub DMACR [
        /// DMA on error. If this bit is set to 1, the DMA receive request
        /// outputs, UARTRXDMASREQ or UARTRXDMABREQ, are disabled when the
        /// UART error interrupt is asserted.
        DMAONERR OFFSET(2) NUMBITS(1) [],
        /// Transmit DMA enable.
        TXDMAE OFFSET(1) NUMBITS(1) [],
        /// Receive DMA enable.
        RXDMAE OFFSET(0) NUMBITS(1) []
    ]
];

register_structs! {
    /// PL011 UART register block.
    pub Pl011Registers {
        /// Data Register.
        (0x000 => pub dr: ReadWrite<u32, DR::Register>),
        /// Receive Status Register / Error Clear Register.
        (0x004 => pub rsr_ecr: ReadWrite<u32, RSR_ECR::Register>),
        (0x008 => _reserved0),
        /// Flag Register.
        (0x018 => pub fr: ReadOnly<u32, FR::Register>),
        (0x01c => _reserved1),
        /// IrDA Low-Power Counter Register.
        (0x020 => pub ilpr: ReadWrite<u32, ILPR::Register>),
        /// Integer Baud Rate Register.
        (0x024 => pub ibrd: ReadWrite<u32, IBRD::Register>),
        /// Fractional Baud Rate Register.
        (0x028 => pub fbrd: ReadWrite<u32, FBRD::Register>),
        /// Line Control Register.
        (0x02c => pub lcr_h: ReadWrite<u32, LCR_H::Register>),
        /// Control Register.
        (0x030 => pub cr: ReadWrite<u32, CR::Register>),
        /// Interrupt FIFO Level Select Register.
        (0x034 => pub ifls: ReadWrite<u32, IFLS::Register>),
        /// Interrupt Mask Set/Clear Register.
        (0x038 => pub imsc: ReadWrite<u32, INTERRUPTS::Register>),
        /// Raw Interrupt Status Register.
        (0x03c => pub ris: ReadOnly<u32, INTERRUPTS::Register>),
        /// Masked Interrupt Status Register.
        (0x040 => pub mis: ReadOnly<u32, INTERRUPTS::Register>),
        /// Interrupt Clear Register.
        (0x044 => pub icr: WriteOnly<u32, INTERRUPTS::Register>),
        /// DMA Control Register.
        (0x048 => pub dmacr: ReadWrite<u32, DMACR::Register>),
        (0x04c => _reserved2),
        /// Peripheral Identification Registers.
        (0xfe0 => pub periph_id: [ReadOnly<u32>; 4]),
        /// PrimeCell Identification Registers.
        (0xff0 => pub pcell_id: [ReadOnly<u32>; 4]),
        (0x1000 => @END),
    }
}

/// A byte read from the receive FIFO
pub struct ReceivedByte {
    pub byte: u8,
    pub error: hil::uart::Error,
}

impl Pl011Registers {
    pub fn set_baud_rate(&self, clock_frequency: u32, baud_rate: u32) -> Result<(), ErrorCode> {
        // The divisor is `UARTCLK / (16 * baud)`
        if baud_rate == 0 {
            return Err(ErrorCode::INVAL);
        }

        let clock = clock_frequency as u64;
        let baud = baud_rate as u64;
        let divisor = (4 * clock + baud / 2) / baud;
        let ibrd = divisor >> 6;
        let fbrd = divisor & 0x3f;

        // IBRD = 0 and IBRD = 0xffff with a non-zero FBRD is invalid
        if ibrd == 0 || ibrd > 0xffff || (ibrd == 0xffff && fbrd != 0) {
            return Err(ErrorCode::INVAL);
        }

        self.ibrd.write(IBRD::BAUD_DIVINT.val(ibrd as u32));
        self.fbrd.write(FBRD::BAUD_DIVFRAC.val(fbrd as u32));
        Ok(())
    }

    pub fn configure(
        &self,
        params: hil::uart::Parameters,
        clock_frequency: u32,
    ) -> Result<(), ErrorCode> {
        let width = match params.width {
            hil::uart::Width::Six => LCR_H::WLEN::SixBit,
            hil::uart::Width::Seven => LCR_H::WLEN::SevenBit,
            hil::uart::Width::Eight => LCR_H::WLEN::EightBit,
        };
        let parity = match params.parity {
            hil::uart::Parity::None => LCR_H::PEN::Disabled,
            hil::uart::Parity::Odd => LCR_H::PEN::Enabled + LCR_H::EPS::Odd,
            hil::uart::Parity::Even => LCR_H::PEN::Enabled + LCR_H::EPS::Even,
        };
        let stop_bits = match params.stop_bits {
            hil::uart::StopBits::One => LCR_H::STP2::CLEAR,
            hil::uart::StopBits::Two => LCR_H::STP2::SET,
        };
        let flow_control = if params.hw_flow_control {
            CR::CTSEN::SET + CR::RTSEN::SET
        } else {
            CR::CTSEN::CLEAR + CR::RTSEN::CLEAR
        };

        // Flush the FIFOs and disable the peripheral
        self.cr.modify(CR::UARTEN::Disabled);
        while self.fr.is_set(FR::BUSY) {}
        self.lcr_h.modify(LCR_H::FEN::Disabled);

        self.set_baud_rate(clock_frequency, params.baud_rate)?;

        self.lcr_h
            .write(width + parity + stop_bits + LCR_H::FEN::Enabled);

        // Transmit interrupt when at most two bytes are left in the FIFO,
        // receive interrupt once two bytes are in
        self.ifls
            .write(IFLS::TXIFLSEL::OneEighth + IFLS::RXIFLSEL::OneEighth);

        self.imsc.set(0);
        self.icr.set(0x7ff);

        self.cr
            .write(CR::UARTEN::Enabled + CR::TXE::Enabled + CR::RXE::Enabled + flow_control);

        Ok(())
    }

    pub fn tx_fifo_full(&self) -> bool {
        self.fr.is_set(FR::TXFF)
    }

    pub fn rx_fifo_empty(&self) -> bool {
        self.fr.is_set(FR::RXFE)
    }

    pub fn busy(&self) -> bool {
        self.fr.is_set(FR::BUSY)
    }

    pub fn write_byte(&self, byte: u8) {
        self.dr.write(DR::DATA.val(byte as u32));
    }

    pub fn read_byte(&self) -> ReceivedByte {
        let data = self.dr.extract();
        let error = if data.is_set(DR::OE) {
            hil::uart::Error::OverrunError
        } else if data.is_set(DR::BE) {
            hil::uart::Error::BreakError
        } else if data.is_set(DR::PE) {
            hil::uart::Error::ParityError
        } else if data.is_set(DR::FE) {
            hil::uart::Error::FramingError
        } else {
            hil::uart::Error::None
        };
        ReceivedByte {
            byte: data.read(DR::DATA) as u8,
            error,
        }
    }

    pub fn transmit_sync(&self, bytes: &[u8]) {
        self.cr.modify(CR::UARTEN::Enabled + CR::TXE::Enabled);

        for byte in bytes {
            while self.tx_fifo_full() {}
            self.write_byte(*byte);
        }

        while self.busy() {}
    }

    pub fn enable_tx_interrupt(&self) {
        self.imsc.modify(INTERRUPTS::TXI::SET);
    }

    pub fn disable_tx_interrupt(&self) {
        self.imsc.modify(INTERRUPTS::TXI::CLEAR);
    }

    pub fn enable_rx_interrupts(&self) {
        self.imsc
            .modify(INTERRUPTS::RXI::SET + INTERRUPTS::RTI::SET);
    }

    pub fn disable_rx_interrupts(&self) {
        self.imsc
            .modify(INTERRUPTS::RXI::CLEAR + INTERRUPTS::RTI::CLEAR);
    }

    pub fn tx_interrupt_pending(&self) -> bool {
        self.mis.is_set(INTERRUPTS::TXI)
    }

    pub fn rx_interrupt_pending(&self) -> bool {
        let pending = self.mis.extract();
        pending.is_set(INTERRUPTS::RXI) || pending.is_set(INTERRUPTS::RTI)
    }

    pub fn clear_rx_timeout_interrupt(&self) {
        self.icr.write(INTERRUPTS::RTI::SET);
    }

    pub fn clear_error_interrupts(&self) {
        self.icr.write(
            INTERRUPTS::OEI::SET
                + INTERRUPTS::BEI::SET
                + INTERRUPTS::PEI::SET
                + INTERRUPTS::FEI::SET,
        );
    }
}
