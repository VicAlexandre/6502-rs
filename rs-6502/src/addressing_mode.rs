use core::panic;

/// Enumerates all possible MOS 6502 memory addressing modes
pub enum AddrMode {
    /// Accumulator (implied)
    Accumulator,
    /// Absolute addressing mode
    Abs,
    /// Absolute addressing mode with X register offset
    AbsX,
    /// Absolute addressing mode with Y register offset
    AbsY,
    /// Immediate addressing mode
    Immediate,
    /// Implied addressing mode
    Impl,
    /// Indirect addressing mode
    Ind,
    /// Indirect addressing mode with X register offset
    IndX,
    /// Indirect addressing mode with Y register offset
    IndY,
    /// Relative addressing mode
    Rel,
    /// Zero page addressing mode
    ZeroPage,
    /// Zero page addressing mode with X register offset
    ZeroPageX,
    /// Zero page addressing mode with Y register offset
    ZeroPageY,
}

pub fn get_addr_mode(opcode: u8) -> AddrMode {
    let lo_nibble = opcode & 0x0F;
    let hi_nibble = (opcode & 0xF0) >> 4;

    match lo_nibble {
        0x00 => {
            if hi_nibble == 0x80 {
                panic!(
                    "Illegal opcode, no addressing mode available: {:#04X}",
                    opcode
                )
            }

            if hi_nibble == 0x00 || hi_nibble == 0x04 || hi_nibble == 0x06 {
                AddrMode::Impl
            } else if hi_nibble % 2 == 1 {
                AddrMode::Rel
            } else if hi_nibble == 0x0A || hi_nibble == 0x0C || hi_nibble == 0x0E {
                AddrMode::Immediate
            } else if hi_nibble == 0x02 {
                AddrMode::Abs
            } else {
                panic!(
                    "Illegal opcode, no addressing mode available: {:#04X}",
                    opcode
                )
            }
        }
        0x01 => {
            if hi_nibble % 2 == 0 {
                AddrMode::IndX
            } else {
                AddrMode::IndY
            }
        }
        0x02 => {
            if hi_nibble == 0x0A {
                AddrMode::Immediate
            } else {
                panic!(
                    "Illegal opcode, no addressing mode available: {:#04X}",
                    opcode
                )
            }
        }
        0x04 => {
            if hi_nibble == 0x02 || (hi_nibble >= 8 && hi_nibble % 2 == 0) {
                AddrMode::ZeroPage
            } else if hi_nibble == 0x09 || hi_nibble == 0x0B {
                AddrMode::ZeroPageX
            } else {
                panic!(
                    "Illegal opcode, no addressing mode available: {:#04X}",
                    opcode
                )
            }
        }
        0x05 => {
            if hi_nibble % 2 == 0 {
                AddrMode::ZeroPage
            } else {
                AddrMode::ZeroPageX
            }
        }
        0x06 => {
            if hi_nibble % 2 == 0 {
                AddrMode::ZeroPage
            } else if hi_nibble == 0x09 || hi_nibble == 0x0B {
                AddrMode::ZeroPageY
            } else {
                AddrMode::ZeroPageX
            }
        }
        0x08 => AddrMode::Impl,
        0x09 => {
            if hi_nibble == 0x08 {
                panic!(
                    "Illegal opcode, no addressing mode available: {:#04X}",
                    opcode
                )
            } else if hi_nibble % 2 == 0 {
                AddrMode::Immediate
            } else {
                AddrMode::AbsY
            }
        }
        0x0A => {
            if hi_nibble <= 0x06 && hi_nibble % 2 == 0 {
                AddrMode::Accumulator
            } else if (hi_nibble >= 0x08 && hi_nibble <= 0x0C) || hi_nibble == 0x0E {
                AddrMode::Impl
            } else {
                panic!(
                    "Illegal opcode, no addressing mode available: {:#04X}",
                    opcode
                )
            }
        }
        0x0C => {
            if hi_nibble != 0x00 && hi_nibble != 0x06 && hi_nibble % 2 == 0 {
                AddrMode::Abs
            } else if hi_nibble == 0x06 {
                AddrMode::Ind
            } else if hi_nibble == 0x0B {
                AddrMode::AbsX
            } else {
                panic!(
                    "Illegal opcode, no addressing mode available: {:#04X}",
                    opcode
                )
            }
        }
        0x0D => {
            if hi_nibble % 2 == 0 {
                AddrMode::Abs
            } else {
                AddrMode::AbsX
            }
        }
        0x0E => {
            if hi_nibble % 2 == 0 {
                AddrMode::Abs
            } else if hi_nibble == 0x0B {
                AddrMode::AbsY
            } else if hi_nibble == 0x09 {
                panic!(
                    "Illegal opcode, no addressing mode available: {:#04X}",
                    opcode
                )
            } else {
                AddrMode::AbsX
            }
        }
        _ => panic!(
            "Illegal opcode, no addressing mode available: {:#04X}",
            opcode
        ),
    }
}
