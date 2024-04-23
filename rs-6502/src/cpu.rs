use crate::{
    addressing_mode::{get_addr_mode, AddrMode},
    memory::Memory,
    stack::Stack,
    status_register::StatusRegister,
};

const MASK_MSB: u8 = 0b10000000;
const MASK_SIXTH_BIT: u8 = 0b01000000;
const MASK_LSB: u8 = 0b00000001;

pub struct Cpu {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub memory: Memory,
    pub stack: Stack,
    pub sr: StatusRegister,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            memory: Memory::new(),
            stack: Stack::new(),
            sr: StatusRegister::new(),
        }
    }

    fn fetch_byte(&mut self) -> u8 {
        let instruction = self.memory.read_byte(self.pc);
        self.pc += 1;
        instruction
    }

    fn fetch_word(&mut self) -> u16 {
        let instruction = self.memory.read_word(self.pc);
        self.pc += 2;
        instruction
    }

    pub fn status(&self) {
        print!("A: {:#04X}\t", self.a);
        print!("X: {:#04X}\t", self.x);
        print!("Y: {:#04X}\t", self.y);
        print!("PC: {:#06X}\t", self.pc);
        print!("SP: {:#04X}\n", self.stack.sp);
        self.sr.status();
        println!(
            "Next instruction opcode: {:#04X}",
            self.memory.read_byte(self.pc)
        );
    }

    fn set_zero_and_negative_flags(&mut self, data: u8) {
        self.sr.zero = data == 0;
        self.sr.negative = (data & MASK_MSB) != 0;
    }

    fn get_zero_page_addr(&mut self) -> u16 {
        self.fetch_byte() as u16
    }

    fn get_zero_page_x_addr(&mut self) -> u16 {
        (self.fetch_byte() as u16 + self.x as u16) & 0x00FF
    }

    fn get_zero_page_y_addr(&mut self) -> u16 {
        (self.fetch_byte() as u16 + self.y as u16) & 0x00FF
    }

    fn get_absolute_addr(&mut self) -> u16 {
        self.fetch_word()
    }

    fn get_absolute_x_addr(&mut self) -> u16 {
        self.fetch_word() + self.x as u16
    }

    fn get_absolute_y_addr(&mut self) -> u16 {
        self.fetch_word() + self.y as u16
    }

    fn get_indirect_addr(&mut self) -> u16 {
        let addr_addr = self.fetch_byte() as u16;
        self.memory.read_word(addr_addr)
    }

    fn get_indirect_x_addr(&mut self) -> u16 {
        let addr_addr = (self.fetch_byte() as u16 + self.x as u16) & 0x00FF;
        self.memory.read_word(addr_addr)
    }

    fn get_indirect_y_addr(&mut self) -> u16 {
        let addr_addr = self.fetch_byte() as u16;
        self.memory.read_word(addr_addr) + self.y as u16
    }

    pub fn execute(&mut self) -> u8 {
        let opcode = self.fetch_byte();
        let addr_mode = get_addr_mode(opcode);

        match opcode {
            //BRK
            0x00 => self.break_interrupt(),
            // CLC
            0x18 => self.clear_carry(),
            // CLD
            0xD8 => self.clear_decimal(),
            // CLI
            0x58 => self.clear_interrupt_dis(),
            // CLV
            0xB8 => self.clear_overflow(),
            // NOP
            0xEA => 2,
            // INY
            0xC8 => self.inc_y(),
            // INX
            0xE8 => self.inc_x(),
            // LDA immediate / zpg / zpg, x / abs / abs, x / abs, y / ind, x / ind, y
            0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => self.lda(addr_mode),
            // LDX immediate / zpg / zpg, y / abs / abs, y
            0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => self.ldx(addr_mode),
            // LDY immediate / zpg / zpg, x / abs / abs, x
            0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => self.ldy(addr_mode),
            // LSR accumulator
            0x4A => self.lsr_accumulator(),
            // LSR zpg / zpg, x / abs / abs, x
            0x46 | 0x56 | 0x4E | 0x5E => self.lsr(addr_mode),
            // ORA immediate / zpg / abs / zpg, x / abs, x / (ind, x) / (ind), y
            0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x01 | 0x11 => self.ora(addr_mode),
            // PHA
            0x48 => self.push_accumulator(),
            // PHP
            0x08 => self.push_processor_status(),
            // PLA
            0x68 => self.pull_accumulator(),
            // PLP
            0x28 => self.pull_processor_status(),
            // ROL accumulator
            0x2A => self.rol_accumulator(),
            // ROL zpg / zpg, x / abs / abs, x
            0x26 | 0x36 | 0x2E | 0x3E => self.rol(addr_mode),
            // ASL accumulator
            0x0A => self.asl_acc(),
            // ASL zpg / zpg, x / abs / abs, x
            0x06 | 0x16 | 0x0E | 0x1E => self.asl(addr_mode),
            // STA zpg / zpg, x / abs / abs, x / abs, y / (ind, x) / (ind), y
            0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => self.sta(addr_mode),
            // STX zpg / zpg, y / abs
            0x86 | 0x96 | 0x8E => self.stx(addr_mode),
            // STY zpg / zpg, x / abs
            0x84 | 0x94 | 0x8C => self.sty(addr_mode),
            // TAX
            0xAA => self.tax(),
            // TAY
            0xA8 => self.tay(),
            // TSX
            0xBA => self.tsx(),
            // TXA
            0x8A => self.txa(),
            // TXS
            0x9A => self.txs(),
            // TYA
            0x98 => self.tya(),
            // ADC immediate / zpg / zpg, x / abs / abs, x / abs, y / (ind, x) / (ind), y
            0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => self.adc(addr_mode),
            // AND immediate / zpg / zpg, x / abs / abs, x / abs, y / (ind, x) / (ind), y
            0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => self.and(addr_mode),
            // BRANCH IFS
            0x90 | 0xB0 | 0xF0 | 0x30 | 0xD0 | 0x10 | 0x50 | 0x70 => self.branch(opcode),
            // BIT zpg / abs
            0x24 | 0x2c => self.bit_test(addr_mode),
            // JSR
            0x20 => self.jsr(),
            //JMP abs / ind
            0x4C | 0x6C => self.jmp(addr_mode),
            // RTS
            0x60 => self.rts(),
            // RTI
            0x40 => self.rti(),
            // CMP immediate / zpg / zpg, x / abs / abs, x / abs, y / (ind, x) / (ind), y
            0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => self.cmp(addr_mode),
            // CPX immediate / zpg / abs
            0xE0 | 0xE4 | 0xEC => self.cpx(addr_mode),
            // CPY immediate / zpg / abs
            0xC0 | 0xC4 | 0xCC => self.cpy(addr_mode),
            // DEC zpg / zpg, x / abs / abs, x
            0xC6 | 0xD6 | 0xCE | 0xDE => self.dec(addr_mode),
            // DEX
            0xCA => self.dex(),
            // DEY
            0x88 => self.dey(),
            // EOR immediate / zpg / zpg, x / abs / abs, x / abs, y / (ind, x) / (ind), y
            0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => self.eor(addr_mode),
            // INC zpg / zpg, x / abs / abs, x
            0xE6 | 0xF6 | 0xEE | 0xFE => self.inc(addr_mode),
            // ROR accumulator
            0x6A => self.ror_accumulator(),
            // ROR zpg / zpg, x / abs / abs, x
            0x66 | 0x76 | 0x6E | 0x7E => self.ror(addr_mode),
            // SBC immediate / zpg / zpg, x / abs / abs, x / abs, y / (ind, x) / (ind), y
            0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => self.sbc(addr_mode),
            // SEC
            0x38 => self.sec(),
            // SED
            0xF8 => self.sed(),
            // SEI
            0x78 => self.sei(),
            // Illegal instruction
            _ => panic!("Instruction not implemented: {:#04X}", opcode),
        }
    }

    fn break_interrupt(&mut self) -> u8 {
        self.stack.push_word(self.pc + 2);
        self.stack.push_byte(self.sr.get_status_byte());

        self.pc = self.memory.read_word(0xFFFE);

        self.sr.brk = true;

        7
    }

    fn branch(&mut self, opcode: u8) -> u8 {
        let branch_offset = self.fetch_byte();
        let old_pc: u16 = self.pc;
        let branch_condition: bool;
        let mut cycles = 2;

        match opcode {
            0x90 => branch_condition = self.sr.carry == false,
            0xB0 => branch_condition = self.sr.carry == true,
            0xF0 => branch_condition = self.sr.zero == true,
            0xD0 => branch_condition = self.sr.zero == false,
            0x30 => branch_condition = self.sr.negative == true,
            0x10 => branch_condition = self.sr.negative == false,
            0x50 => branch_condition = self.sr.overflow == false,
            0x70 => branch_condition = self.sr.overflow == true,
            _ => panic!("Invalid branching mode!"),
        }

        if branch_condition {
            self.pc += branch_offset as u16;
            cycles += 1;

            if ((old_pc >> 8) - (self.pc >> 8)) != 0 {
                cycles += 2;
            }
        }

        cycles
    }

    fn bit_test(&mut self, addr_mode: AddrMode) -> u8 {
        let cycles: u8;
        let data: u8;
        let data_addr: u16;

        match addr_mode {
            AddrMode::ZeroPage => {
                data_addr = self.get_zero_page_addr();
                cycles = 3;
            }
            AddrMode::Abs => {
                data_addr = self.get_absolute_addr();
                cycles = 4;
            }
            _ => panic!("Illegal opcode!"),
        }

        data = self.memory.read_byte(data_addr);

        self.sr.negative = (data & MASK_MSB) != 0;
        self.sr.overflow = (data & MASK_SIXTH_BIT) != 0;

        self.sr.zero = (data & self.a) == 0;

        cycles
    }

    fn clear_carry(&mut self) -> u8 {
        self.sr.carry = false;

        2
    }

    fn clear_decimal(&mut self) -> u8 {
        self.sr.decimal = false;

        2
    }

    fn clear_interrupt_dis(&mut self) -> u8 {
        self.sr.interrupt_disable = false;

        2
    }

    fn clear_overflow(&mut self) -> u8 {
        self.sr.overflow = false;

        2
    }

    fn inc_y(&mut self) -> u8 {
        self.y += 1;

        2
    }

    fn inc_x(&mut self) -> u8 {
        self.x += 1;

        2
    }

    fn lda(&mut self, addr_mode: AddrMode) -> u8 {
        let cycles: u8;
        let data: u8;

        match addr_mode {
            AddrMode::Immediate => {
                data = self.fetch_byte();
                cycles = 2;
            }
            AddrMode::ZeroPage => {
                let data_addr = self.get_zero_page_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 3;
            }
            AddrMode::ZeroPageX => {
                let data_addr = self.get_zero_page_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 3;
            }
            AddrMode::Abs => {
                let data_addr = self.get_absolute_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4;
            }
            AddrMode::AbsX => {
                let data_addr = self.get_absolute_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4 as u8 + (data_addr > 0x00FF) as u8;
            }
            AddrMode::AbsY => {
                let data_addr = self.get_absolute_y_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4 as u8 + (data_addr > 0x00FF) as u8;
            }
            AddrMode::IndX => {
                let data_addr = self.get_indirect_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 6;
            }
            AddrMode::IndY => {
                let data_addr = self.get_indirect_y_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 5 as u8 + (data_addr > 0x00FF) as u8;
            }
            _ => panic!("Addressing mode not supported"),
        }

        self.a = data;
        self.set_zero_and_negative_flags(self.a);

        cycles
    }

    fn ldx(&mut self, addr_mode: AddrMode) -> u8 {
        let cycles: u8;
        let data: u8;

        match addr_mode {
            AddrMode::Immediate => {
                data = self.fetch_byte();
                cycles = 2;
            }
            AddrMode::ZeroPage => {
                let data_addr = self.get_zero_page_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 3;
            }
            AddrMode::ZeroPageY => {
                let data_addr = self.get_zero_page_y_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4;
            }
            AddrMode::Abs => {
                let data_addr = self.get_absolute_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4;
            }
            AddrMode::AbsY => {
                let data_addr = self.get_absolute_y_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4 as u8 + (data_addr > 0x00FF) as u8;
            }
            _ => panic!("Addressing mode not supported"),
        }

        self.x = data;
        self.set_zero_and_negative_flags(self.x);

        cycles
    }

    fn ldy(&mut self, addr_mode: AddrMode) -> u8 {
        let cycles: u8;
        let data: u8;

        match addr_mode {
            AddrMode::Immediate => {
                data = self.fetch_byte();
                cycles = 2;
            }
            AddrMode::ZeroPage => {
                let data_addr = self.get_zero_page_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 3;
            }
            AddrMode::ZeroPageX => {
                let data_addr = self.get_zero_page_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4;
            }
            AddrMode::Abs => {
                let data_addr = self.get_absolute_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4;
            }
            AddrMode::AbsX => {
                let data_addr = self.get_absolute_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4 as u8 + (data_addr > 0x00FF) as u8;
            }
            _ => panic!("Addressing mode not supported"),
        }

        self.y = data;
        self.set_zero_and_negative_flags(self.y);

        cycles
    }

    fn lsr_accumulator(&mut self) -> u8 {
        self.sr.carry = (self.a & MASK_LSB) != 0;

        self.a = self.a >> 1;

        self.set_zero_and_negative_flags(self.a);

        2
    }

    fn lsr(&mut self, addr_mode: AddrMode) -> u8 {
        let cycles: u8;
        let data_addr: u16;
        let mut data: u8;

        match addr_mode {
            AddrMode::ZeroPage => {
                data_addr = self.get_zero_page_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 5;
            }
            AddrMode::ZeroPageX => {
                data_addr = self.get_zero_page_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 6;
            }
            AddrMode::Abs => {
                data_addr = self.get_absolute_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 6;
            }
            AddrMode::AbsX => {
                data_addr = self.get_absolute_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 7;
            }
            _ => panic!("Addressing mode not supported"),
        }

        self.sr.carry = (data & MASK_LSB) != 0;

        data = data >> 1;

        self.memory.write_byte(data_addr, data);

        self.set_zero_and_negative_flags(data);

        cycles
    }

    fn ora(&mut self, addr_mode: AddrMode) -> u8 {
        let data_addr: u16;
        let data: u8;
        let cycles;

        match addr_mode {
            AddrMode::Immediate => {
                data = self.fetch_byte();
                cycles = 2;
            }
            AddrMode::ZeroPage => {
                data_addr = self.get_zero_page_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 3;
            }
            AddrMode::ZeroPageX => {
                data_addr = self.get_zero_page_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4;
            }
            AddrMode::Abs => {
                data_addr = self.get_absolute_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4;
            }
            AddrMode::AbsX => {
                data_addr = self.get_absolute_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4 as u8 + (data_addr > 0x00FF) as u8
            }
            AddrMode::IndX => {
                data_addr = self.get_indirect_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 6;
            }
            AddrMode::IndY => {
                data_addr = self.get_indirect_y_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 5 as u8 + (data_addr > 0xFF) as u8
            }
            _ => panic!("Addressing mode not supported"),
        }

        self.a = self.a | data;

        self.set_zero_and_negative_flags(self.a);

        cycles
    }

    fn push_accumulator(&mut self) -> u8 {
        self.stack.push_byte(self.a);

        3
    }

    fn push_processor_status(&mut self) -> u8 {
        let processor_status = self.sr.get_status_byte();
        self.stack.push_byte(processor_status);

        3
    }

    fn pull_accumulator(&mut self) -> u8 {
        let popped_acc = self.stack.pop_byte();
        self.a = popped_acc;

        self.set_zero_and_negative_flags(popped_acc);

        4
    }

    fn pull_processor_status(&mut self) -> u8 {
        let popped_sr = self.stack.pop_byte();
        self.sr.set_status_byte(popped_sr);

        4
    }

    fn rol(&mut self, addr_mode: AddrMode) -> u8 {
        let old_carry: u8;
        let mut data: u8;
        let data_addr: u16;
        let cycles;

        match addr_mode {
            AddrMode::ZeroPage => {
                old_carry = self.sr.carry as u8;
                data_addr = self.get_zero_page_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 5;
            }
            AddrMode::ZeroPageX => {
                old_carry = self.sr.carry as u8;
                data_addr = self.get_zero_page_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 6;
            }
            AddrMode::Abs => {
                old_carry = self.sr.carry as u8;
                data_addr = self.get_absolute_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 6;
            }
            AddrMode::AbsX => {
                old_carry = self.sr.carry as u8;
                data_addr = self.get_absolute_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 7;
            }
            _ => panic!("Addressing mode not supported"),
        }

        self.sr.carry = (data & MASK_MSB) != 0;

        data = (data << 1) | old_carry;

        self.set_zero_and_negative_flags(data);

        self.memory.write_byte(data_addr, data);

        cycles
    }

    fn rol_accumulator(&mut self) -> u8 {
        let old_carry = self.sr.carry as u8;

        self.sr.carry = (self.a & MASK_MSB) != 0;

        self.a = (self.a << 1) | old_carry;

        self.set_zero_and_negative_flags(self.a);

        2
    }

    fn asl(&mut self, addr_mode: AddrMode) -> u8 {
        let data_addr: u16;
        let mut data: u8;
        let cycles;

        match addr_mode {
            AddrMode::ZeroPage => {
                data_addr = self.get_zero_page_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 5;
            }
            AddrMode::ZeroPageX => {
                data_addr = self.get_zero_page_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 5;
            }
            AddrMode::Abs => {
                data_addr = self.get_absolute_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 6;
            }
            AddrMode::AbsX => {
                data_addr = self.get_absolute_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 7;
            }
            _ => panic!("Addressing mode not supported"),
        }

        self.sr.carry = (data & MASK_MSB) != 0;

        data = data << 1;

        self.set_zero_and_negative_flags(data);

        self.memory.write_byte(data_addr, data);

        cycles
    }

    fn asl_acc(&mut self) -> u8 {
        let old_byte = self.a;

        self.sr.carry = (old_byte & MASK_MSB) != 0;

        self.a = self.a << 1;

        self.set_zero_and_negative_flags(self.a);

        2
    }

    fn sta(&mut self, addr_mode: AddrMode) -> u8 {
        let data_addr: u16;
        let cycles: u8;

        match addr_mode {
            AddrMode::ZeroPage => {
                data_addr = self.get_zero_page_addr();
                cycles = 3;
            }
            AddrMode::ZeroPageX => {
                data_addr = self.get_zero_page_x_addr();
                cycles = 4;
            }
            AddrMode::Abs => {
                data_addr = self.get_absolute_addr();
                cycles = 4;
            }
            AddrMode::AbsX => {
                data_addr = self.get_absolute_x_addr();
                cycles = 5;
            }
            AddrMode::AbsY => {
                data_addr = self.get_absolute_y_addr();
                cycles = 5;
            }
            AddrMode::IndX => {
                data_addr = self.get_indirect_x_addr();
                cycles = 6;
            }
            AddrMode::IndY => {
                data_addr = self.get_indirect_y_addr();
                cycles = 6;
            }
            _ => panic!("Addressing mode not supported"),
        }

        self.memory.write_byte(data_addr, self.a);

        cycles
    }

    fn stx(&mut self, addr_mode: AddrMode) -> u8 {
        let data_addr: u16;
        let cycles: u8;

        match addr_mode {
            AddrMode::ZeroPage => {
                data_addr = self.get_zero_page_addr();
                cycles = 3;
            }
            AddrMode::ZeroPageY => {
                data_addr = self.get_zero_page_y_addr();
                cycles = 4;
            }
            AddrMode::Abs => {
                data_addr = self.get_absolute_addr();
                cycles = 4;
            }
            _ => panic!("Addressing mode not supported"),
        }

        self.memory.write_byte(data_addr, self.x);

        cycles
    }

    fn sty(&mut self, addr_mode: AddrMode) -> u8 {
        let data_addr: u16;
        let cycles: u8;

        match addr_mode {
            AddrMode::ZeroPage => {
                data_addr = self.get_zero_page_addr();
                cycles = 3;
            }
            AddrMode::ZeroPageX => {
                data_addr = self.get_zero_page_x_addr();
                cycles = 4;
            }
            AddrMode::Abs => {
                data_addr = self.get_absolute_addr();
                cycles = 4;
            }
            _ => panic!("Addressing mode not supported"),
        }

        self.memory.write_byte(data_addr, self.y);

        cycles
    }

    fn tax(&mut self) -> u8 {
        self.x = self.a;
        self.set_zero_and_negative_flags(self.x);

        2
    }

    fn tay(&mut self) -> u8 {
        self.y = self.a;
        self.set_zero_and_negative_flags(self.y);

        2
    }

    fn tsx(&mut self) -> u8 {
        self.x = self.stack.sp;
        self.set_zero_and_negative_flags(self.x);

        2
    }

    fn txa(&mut self) -> u8 {
        self.a = self.x;
        self.set_zero_and_negative_flags(self.a);

        2
    }

    fn txs(&mut self) -> u8 {
        self.stack.sp = self.x;
        self.set_zero_and_negative_flags(self.stack.sp);

        2
    }

    fn tya(&mut self) -> u8 {
        self.a = self.y;
        self.set_zero_and_negative_flags(self.a);

        2
    }

    fn adc(&mut self, addr_mode: AddrMode) -> u8 {
        let data: u8;
        let cycles: u8;

        match addr_mode {
            AddrMode::Immediate => {
                data = self.fetch_byte();
                cycles = 2;
            }
            AddrMode::ZeroPage => {
                let data_addr = self.get_zero_page_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 3;
            }
            AddrMode::ZeroPageX => {
                let data_addr = self.get_zero_page_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4;
            }
            AddrMode::Abs => {
                let data_addr = self.get_absolute_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4;
            }
            AddrMode::AbsX => {
                let data_addr = self.get_absolute_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4 + (data_addr > 0x00FF) as u8;
            }
            AddrMode::AbsY => {
                let data_addr = self.get_absolute_y_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4 + (data_addr > 0x00FF) as u8;
            }
            AddrMode::IndX => {
                let data_addr = self.get_indirect_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 6;
            }
            AddrMode::IndY => {
                let data_addr = self.get_indirect_y_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 5 + (data_addr > 0x00FF) as u8;
            }
            _ => panic!("Addressing mode not supported"),
        }

        self.sr.carry = data as u16 + self.a as u16 + self.sr.carry as u16 > 0xFF;
        self.sr.overflow = data as u16 + self.a as u16 + self.sr.carry as u16 > 0xFF;
        self.a += data + self.sr.carry as u8;

        cycles
    }

    fn and(&mut self, addr_mode: AddrMode) -> u8 {
        let data: u8;
        let cycles: u8;

        match addr_mode {
            AddrMode::Immediate => {
                data = self.fetch_byte();
                cycles = 2;
            }
            AddrMode::ZeroPage => {
                let data_addr = self.get_zero_page_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 3;
            }
            AddrMode::ZeroPageX => {
                let data_addr = self.get_zero_page_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4;
            }
            AddrMode::Abs => {
                let data_addr = self.get_absolute_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4;
            }
            AddrMode::AbsX => {
                let data_addr = self.get_absolute_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4 + (data_addr > 0x00FF) as u8;
            }
            AddrMode::AbsY => {
                let data_addr = self.get_absolute_y_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4 + (data_addr > 0x00FF) as u8;
            }
            AddrMode::IndX => {
                let data_addr = self.get_indirect_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 6;
            }
            AddrMode::IndY => {
                let data_addr = self.get_indirect_y_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 5 + (data_addr > 0x00FF) as u8;
            }
            _ => panic!("Addressing mode not supported"),
        }

        self.a = self.a & data;
        self.set_zero_and_negative_flags(self.a);

        cycles
    }

    fn jsr(&mut self) -> u8 {
        self.stack.push_word(self.pc + 0x02);
        self.pc = self.fetch_word();

        3
    }

    fn rts(&mut self) -> u8 {
        self.pc = self.stack.pop_word() + 1;

        6
    }

    fn jmp(&mut self, addr_mode: AddrMode) -> u8 {
        let cycles: u8;
        let new_pc_addr: u16;

        match addr_mode {
            AddrMode::Abs => {
                new_pc_addr = self.get_absolute_addr();
                cycles = 3
            }
            AddrMode::Ind => {
                new_pc_addr = self.get_indirect_addr();
                cycles = 5
            }
            _ => panic!("Addressing mode not supported"),
        }

        self.pc = new_pc_addr;

        cycles
    }

    fn rti(&mut self) -> u8 {
        self.sr.set_status_byte(self.stack.pop_byte());
        self.pc = self.stack.pop_word();

        6
    }

    fn cmp(&mut self, addr_mode: AddrMode) -> u8 {
        let data: u8;
        let cycles: u8;

        match addr_mode {
            AddrMode::Immediate => {
                data = self.fetch_byte();
                cycles = 2;
            }
            AddrMode::ZeroPage => {
                let data_addr = self.get_zero_page_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 3;
            }
            AddrMode::ZeroPageX => {
                let data_addr = self.get_zero_page_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4;
            }
            AddrMode::Abs => {
                let data_addr = self.get_absolute_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4;
            }
            AddrMode::AbsX => {
                let data_addr = self.get_absolute_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4 + (data_addr > 0x00FF) as u8;
            }
            AddrMode::AbsY => {
                let data_addr = self.get_absolute_y_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4 + (data_addr > 0x00FF) as u8;
            }
            AddrMode::IndX => {
                let data_addr = self.get_indirect_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 6;
            }
            AddrMode::IndY => {
                let data_addr = self.get_indirect_y_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 5 + (data_addr > 0x00FF) as u8;
            }
            _ => panic!("Addressing mode not supported"),
        }

        self.sr.carry = self.a >= data;
        self.set_zero_and_negative_flags(self.a - data);

        cycles
    }

    fn cpx(&mut self, addr_mode: AddrMode) -> u8 {
        let data: u8;
        let cycles: u8;

        match addr_mode {
            AddrMode::Immediate => {
                data = self.fetch_byte();
                cycles = 2;
            }
            AddrMode::ZeroPage => {
                let data_addr = self.get_zero_page_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 3;
            }
            AddrMode::Abs => {
                let data_addr = self.get_absolute_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4;
            }
            _ => panic!("Addressing mode not supported"),
        }

        self.sr.carry = self.x >= data;
        self.set_zero_and_negative_flags(self.x - data);

        cycles
    }

    fn cpy(&mut self, addr_mode: AddrMode) -> u8 {
        let data: u8;
        let cycles: u8;

        match addr_mode {
            AddrMode::Immediate => {
                data = self.fetch_byte();
                cycles = 2;
            }
            AddrMode::ZeroPage => {
                let data_addr = self.get_zero_page_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 3;
            }
            AddrMode::Abs => {
                let data_addr = self.get_absolute_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4;
            }
            _ => panic!("Addressing mode not supported"),
        }

        self.sr.carry = self.y >= data;
        self.set_zero_and_negative_flags(self.y - data);

        cycles
    }

    fn dec(&mut self, addr_mode: AddrMode) -> u8 {
        let data_addr: u16;
        let mut data: u8;
        let cycles: u8;

        match addr_mode {
            AddrMode::ZeroPage => {
                data_addr = self.get_zero_page_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 5;
            }
            AddrMode::ZeroPageX => {
                data_addr = self.get_zero_page_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 6;
            }
            AddrMode::Abs => {
                data_addr = self.get_absolute_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 6;
            }
            AddrMode::AbsX => {
                data_addr = self.get_absolute_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 7;
            }
            _ => panic!("Addressing mode not supported"),
        }

        data = data - 1;
        self.set_zero_and_negative_flags(data);
        self.memory.write_byte(data_addr, data);

        cycles
    }

    fn dex(&mut self) -> u8 {
        self.x = self.x - 1;
        self.set_zero_and_negative_flags(self.x);

        2
    }

    fn dey(&mut self) -> u8 {
        self.y = self.y - 1;
        self.set_zero_and_negative_flags(self.y);

        2
    }

    fn eor(&mut self, addr_mode: AddrMode) -> u8 {
        let data_addr: u16;
        let data: u8;
        let cycles: u8;

        match addr_mode {
            AddrMode::Immediate => {
                data = self.fetch_byte();
                cycles = 2;
            }
            AddrMode::ZeroPage => {
                data_addr = self.get_zero_page_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 3;
            }
            AddrMode::ZeroPageX => {
                data_addr = self.get_zero_page_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4;
            }
            AddrMode::Abs => {
                data_addr = self.get_absolute_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4;
            }
            AddrMode::AbsX => {
                data_addr = self.get_absolute_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4 + (data_addr > 0x00FF) as u8;
            }
            AddrMode::AbsY => {
                data_addr = self.get_absolute_y_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 4 + (data_addr > 0x00FF) as u8;
            }
            AddrMode::IndX => {
                data_addr = self.get_indirect_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 6;
            }
            AddrMode::IndY => {
                data_addr = self.get_indirect_y_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 5 + (data_addr > 0x00FF) as u8;
            }
            _ => panic!("Addressing mode not supported"),
        }

        self.a = self.a ^ data;
        self.set_zero_and_negative_flags(self.a);

        cycles
    }

    fn inc(&mut self, addr_mode: AddrMode) -> u8 {
        let data_addr: u16;
        let mut data: u8;
        let cycles: u8;

        match addr_mode {
            AddrMode::ZeroPage => {
                data_addr = self.get_zero_page_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 5;
            }
            AddrMode::ZeroPageX => {
                data_addr = self.get_zero_page_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 6;
            }
            AddrMode::Abs => {
                data_addr = self.get_absolute_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 6;
            }
            AddrMode::AbsX => {
                data_addr = self.get_absolute_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 7;
            }
            _ => panic!("Addressing mode not supported"),
        }

        data = data + 1;
        self.set_zero_and_negative_flags(data);
        self.memory.write_byte(data_addr, data);

        cycles
    }

    fn ror_accumulator(&mut self) -> u8 {
        let old_carry = self.sr.carry as u8;

        self.sr.carry = (self.a & MASK_LSB) != 0;

        self.a = (self.a >> 1) | (old_carry << 7);

        self.set_zero_and_negative_flags(self.a);

        2
    }

    fn ror(&mut self, addr_mode: AddrMode) -> u8 {
        let old_carry: u8;
        let mut data: u8;
        let data_addr: u16;
        let cycles;

        match addr_mode {
            AddrMode::ZeroPage => {
                old_carry = self.sr.carry as u8;
                data_addr = self.get_zero_page_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 5;
            }
            AddrMode::ZeroPageX => {
                old_carry = self.sr.carry as u8;
                data_addr = self.get_zero_page_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 6;
            }
            AddrMode::Abs => {
                old_carry = self.sr.carry as u8;
                data_addr = self.get_absolute_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 6;
            }
            AddrMode::AbsX => {
                old_carry = self.sr.carry as u8;
                data_addr = self.get_absolute_x_addr();
                data = self.memory.read_byte(data_addr);
                cycles = 7;
            }
            _ => panic!("Addressing mode not supported"),
        }

        self.sr.carry = (data & MASK_LSB) != 0;

        data = (data >> 1) | (old_carry << 7);

        self.set_zero_and_negative_flags(data);

        self.memory.write_byte(data_addr, data);

        cycles
    }

    fn sbc(&mut self, addr_mode: AddrMode) -> u8 {
        let cycles: u8;
        let data_addr: u16;
        let data: u8;
        let result: u16;

        match addr_mode {
            AddrMode::Immediate => {
                self.pc += 1;
                data_addr = self.pc;
                cycles = 2;
            }
            AddrMode::ZeroPage => {
                data_addr = self.get_zero_page_addr();
                cycles = 3;
            }
            AddrMode::ZeroPageX => {
                data_addr = self.get_zero_page_x_addr();
                cycles = 4;
            }
            AddrMode::Abs => {
                data_addr = self.get_absolute_addr();
                cycles = 4;
            }
            AddrMode::AbsX => {
                data_addr = self.get_absolute_x_addr();
                cycles = 4 + (data_addr > 0x00FF) as u8;
            }
            AddrMode::AbsY => {
                data_addr = self.get_zero_page_y_addr();
                cycles = 4 + (data_addr > 0x00FF) as u8;
            }
            AddrMode::IndX => {
                data_addr = self.get_indirect_x_addr();
                cycles = 6;
            }
            AddrMode::IndY => {
                data_addr = self.get_indirect_y_addr();
                cycles = 5 + (data_addr > 0x00FF) as u8;
            }
            _ => panic!("Adressing mode not supported"),
        }

        data = self.memory.read_byte(data_addr);
        result = self.a as u16 - data as u16 - !(self.sr.carry as u16);

        self.sr.carry = (result & 0x0100) == 0;
        self.sr.overflow = !self.sr.carry;
        self.set_zero_and_negative_flags(result as u8);

        cycles
    }

    fn sec(&mut self) -> u8 {
        self.sr.carry = true;

        2
    }

    fn sed(&mut self) -> u8 {
        self.sr.decimal = true;

        2
    }

    fn sei(&mut self) -> u8 {
        self.sr.interrupt_disable = true;

        2
    }
}
