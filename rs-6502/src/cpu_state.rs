use std::fmt;

use crate::cpu::Cpu;

/// Represents the state of the CPU at a given point in time, used to display the
/// current and previous state of the CPU
pub struct CpuState {
    /// Accumulator register
    pub a: u8,
    /// X register
    pub x: u8,
    /// Y register
    pub y: u8,
    /// Stack pointer
    pub sp: u8,
    /// Program counter
    pub pc: u16,
    /// Negative flag
    pub negative: bool,
    /// Overflow flag
    pub overflow: bool,
    // Break flag
    pub brk: bool,
    // Decimal flag
    pub decimal: bool,
    // Interrupt disable flag
    pub interrupt_disable: bool,
    // Zero flag
    pub zero: bool,
    // Carry flag
    pub carry: bool,
    // Number of cycles used
    pub cycles: u32,
    /// Next instruction to be executed.
    pub next_instruction: u8,
}

impl CpuState {
    pub fn new(cpu: &Cpu) -> CpuState {
        CpuState {
            a: cpu.a,
            x: cpu.x,
            y: cpu.y,
            pc: cpu.pc,
            sp: cpu.stack.get_sp(),
            negative: cpu.sr.get_negative(),
            overflow: cpu.sr.get_overflow(),
            brk: cpu.sr.get_brk(),
            decimal: cpu.sr.get_decimal(),
            interrupt_disable: cpu.sr.get_interrupt_disable(),
            zero: cpu.sr.get_zero(),
            carry: cpu.sr.get_carry(),
            cycles: 0,
            next_instruction: cpu.memory.read_byte(cpu.pc),
        }
    }
}

impl fmt::Display for CpuState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "######## REGISTER BANK ########\n
A: 0x{:02X} | X: 0x{:02X} | Y: 0x{:02X} | PC: 0x{:04X} | SP: 0x{:02X}\n
######## STATUS REGISTER FLAGS ########\n
N: {} || O: {} || B: {} || D: {} || I: {} || Z: {} || C: {}\n
Cycles used: {}\n",
            self.a,
            self.x,
            self.y,
            self.pc,
            self.sp,
            self.negative as u8,
            self.overflow as u8,
            self.brk as u8,
            self.decimal as u8,
            self.interrupt_disable as u8,
            self.zero as u8,
            self.carry as u8,
            self.cycles,
        )
    }
}
