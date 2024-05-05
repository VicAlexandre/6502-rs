use crate::addressing_mode::{get_addr_mode, AddrMode};
use crate::cpu::Cpu;
use std::fmt;

pub struct Instruction {
    name: &'static str,
    description: &'static str,
    opcode: u8,
    addr_mode: String,
    assembly: String,
}

impl Instruction {
    pub fn new(cpu: &Cpu) -> Instruction {
        let next_instruction = cpu.memory.read_byte(cpu.pc);
        let name_and_desc = instruction_name_and_description(next_instruction);
        let addr_mode = addr_mode_str(get_addr_mode(next_instruction));

        Instruction {
            name: name_and_desc.0,
            description: name_and_desc.1,
            opcode: next_instruction,
            addr_mode,
            assembly: name_and_desc.0.to_string()
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {}\nOPCODE: 0x{:02X}\nADDR: {}\nASSEMBLY:\n{}",
            self.name, self.description, self.opcode, self.addr_mode, self.assembly
        )
    }
}

fn instruction_name_and_description(opcode: u8) -> (&'static str, &'static str) {
    match opcode {
        0x00 => ("BRK", "Break"),
        0x18 => ("CLC", "Clear Carry"),
        0xD8 => ("CLD", "Clear Decimal"),
        0x58 => ("CLI", "Clear Interrupt"),
        0xB8 => ("CLV", "Clear Overflow"),
        0xEA => ("NOP", "No operation"),
        0xC8 => ("INY", "Increment Y"),
        0xE8 => ("INX", "Increment X"),
        0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => ("LDA", "Load Acc"),
        0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => ("LDX", "Load X Reg"),
        0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => ("LDY", "Load Y Reg"),
        0x4A | 0x46 | 0x56 | 0x4E | 0x5E => ("LSR", "Shift Right"),
        0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x01 | 0x11 => ("ORA", "Inclusive OR"),
        0x48 => ("PHA", "Push Acc"),
        0x08 => ("PHP", "Push Proc Status"),
        0x68 => ("PLA", "Pull Acc"),
        0x28 => ("PLP", "Pull Proc Status"),
        0x2A | 0x26 | 0x36 | 0x2E | 0x3E => ("ROL", "Rotate Left"),
        0x0A | 0x06 | 0x16 | 0x0E | 0x1E => ("ASL", "Shift Left"),
        0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => ("STA", "Store Acc"),
        0x86 | 0x96 | 0x8E => ("STX", "Store X"),
        0x84 | 0x94 | 0x8C => ("STY", "Store Y"),
        0xAA => ("TAX", "Acc -> X"),
        0xA8 => ("TAY", "Acc -> Y"),
        0xBA => ("TSX", "SP -> X"),
        0x8A => ("TXA", "X -> Acc"),
        0x9A => ("TXS", "X -> SP"),
        0x98 => ("TYA", "Y -> Acc"),
        0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => ("ADC", "Add w/ Carry"),
        0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => ("AND", "Logical AND"),
        0x90 => ("BCC", "Branch Carry Clear"),
        0xB0 => ("BCS", "Branch Carry Set"),
        0xF0 => ("BEQ", "Branch Zero"),
        0x30 => ("BMI", "Branch Minus"),
        0xD0 => ("BNE", "Branch Not Zero"),
        0x10 => ("BPL", "Branch Plus"),
        0x50 => ("BVC", "Branch Overflow Clr"),
        0x70 => ("BVS", "Branch Overflow Set"),
        0x24 | 0x2c => ("BIT", "Bit Test"),
        0x20 => ("JSR", "Jump Subroutine"),
        0x4C | 0x6C => ("JMP", "Jump"),
        0x60 => ("RTS", "Ret Subroutine"),
        0x40 => ("RTI", "Ret Interrupt"),
        0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => ("CMP", "Compare"),
        0xE0 | 0xE4 | 0xEC => ("CPX", "Compare X"),
        0xC0 | 0xC4 | 0xCC => ("CPY", "Compare Y"),
        0xC6 | 0xD6 | 0xCE | 0xDE => ("DEC", "Decrement"),
        0xCA => ("DEX", "Decrement X"),
        0x88 => ("DEY", "Decrement Y"),
        0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => ("EOR", "Exclusive OR"),
        0xE6 | 0xF6 | 0xEE | 0xFE => ("INC", "Increment"),
        0x6A | 0x66 | 0x76 | 0x6E | 0x7E => ("ROR", "Rotate Right"),
        0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => ("SBC", "Sub w/ Carry"),
        0x38 => ("SEC", "Set Carry"),
        0xF8 => ("SED", "Set Decimal"),
        0x78 => ("SEI", "Set Interrupt"),
        _ => panic!("Instruction not implemented: {:#04X}", opcode),
    }
}

fn addr_mode_str(addr_mode: AddrMode) -> String {
    match addr_mode {
        AddrMode::Abs => "Abs",
        AddrMode::AbsX => "Abs, X",
        AddrMode::AbsY => "Abs, Y",
        AddrMode::Accumulator => "Acc",
        AddrMode::Immediate => "Imm",
        AddrMode::Impl => "Implied",
        AddrMode::Ind => "Indirect",
        AddrMode::IndX => "Idx Indirect",
        AddrMode::IndY => "Indirect Idx",
        AddrMode::Rel => "Relative",
        AddrMode::ZeroPage => "ZPG",
        AddrMode::ZeroPageX => "ZPG, X",
        AddrMode::ZeroPageY => "ZPG, Y",
    }
    .to_string()
}
