/// Enumerates all possible MOS 6502 memory addressing modes
pub enum AddrMode {
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
