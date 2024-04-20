use crate::{
    addressing_mode::{get_addr_mode, AddrMode}, memory::Memory, stack::Stack, status_register::StatusRegister,
};

const MASK_MSB: u8 = 0b10000000;
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

    fn fetch_u8(&mut self) -> u8 {
        let instruction = self.memory.read_u8(self.pc);
        self.pc += 1;
        instruction
    }

    fn fetch_u16(&mut self) -> u16 {
        let instruction = self.memory.read_u16(self.pc);
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
        println!("Next instruction opcode: {:#04X}", self.memory.read_u8(self.pc));
    }

    fn set_zero_and_negative_flags(&mut self, data: u8) {
        self.sr.zero = data == 0;
        self.sr.negative = (data & MASK_MSB) != 0;
    }

    fn get_zero_page_addr(&mut self) -> u16 {
        self.fetch_u8() as u16
    }

    fn get_zero_page_x_addr(&mut self) -> u16 {
        (self.fetch_u8() as u16 + self.x as u16) & 0x00FF
    }

    fn get_zero_page_y_addr(&mut self) -> u16 {
        (self.fetch_u8() as u16 + self.y as u16) & 0x00FF
    }

    fn get_absolute_addr(&mut self) -> u16 {
        self.fetch_u16()
    }

    fn get_absolute_x_addr(&mut self) -> u16 {
        self.fetch_u16() + self.x as u16
    }

    fn get_absolute_y_addr(&mut self) -> u16 {
        self.fetch_u16() + self.y as u16
    }

    fn get_indirect_x_addr(&mut self) -> u16 {
        let addr_addr = (self.fetch_u8() as u16 + self.x as u16) & 0x00FF;
        self.memory.read_u16(addr_addr)
    }

    fn get_indirect_y_addr(&mut self) -> u16 {
        let addr_addr = self.fetch_u8() as u16;
        self.memory.read_u16(addr_addr) + self.y as u16
    }

    pub fn execute(&mut self) -> u8 {
        let opcode = self.fetch_u8();
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
            _ => panic!("Instruction not implemented: {:#04X}", opcode),
        }
    }

    fn break_interrupt(&mut self) -> u8 {
        self.stack.push_u16(self.pc + 2);
        self.stack.push_u8(self.sr.get_status_byte());

        self.pc = self.memory.read_u16(0xFFFE);

        self.sr.brk = true;

        7
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
                data = self.fetch_u8();
                cycles = 2;
            }
            AddrMode::ZeroPage => {
                let data_addr = self.get_zero_page_addr();
                data = self.memory.read_u8(data_addr);
                cycles = 3;
            }
            AddrMode::ZeroPageX => {
                let data_addr = self.get_zero_page_x_addr();
                data = self.memory.read_u8(data_addr);
                cycles = 3;
            }
            AddrMode::Abs => {
                let data_addr = self.get_absolute_addr();
                data = self.memory.read_u8(data_addr);
                cycles = 4;
            }
            AddrMode::AbsX => {
                let data_addr = self.get_absolute_x_addr();
                data = self.memory.read_u8(data_addr);
                cycles = 4 as u8 + (data_addr > 0x00FF) as u8;
            }
            AddrMode::AbsY => {
                let data_addr = self.get_absolute_y_addr();
                data = self.memory.read_u8(data_addr);
                cycles = 4 as u8 + (data_addr > 0x00FF) as u8;
            }
            AddrMode::IndX => {
                let data_addr = self.get_indirect_x_addr();
                data = self.memory.read_u8(data_addr);
                cycles = 6;
            }
            AddrMode::IndY => {
                let data_addr = self.get_indirect_y_addr();
                data = self.memory.read_u8(data_addr);
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
                data = self.fetch_u8();
                cycles = 2;
            }
            AddrMode::ZeroPage => {
                let data_addr = self.get_zero_page_addr();
                data = self.memory.read_u8(data_addr);
                cycles = 3;
            }
            AddrMode::ZeroPageY => {
                let data_addr = self.get_zero_page_y_addr();
                data = self.memory.read_u8(data_addr);
                cycles = 4;
            }
            AddrMode::Abs => {
                let data_addr = self.get_absolute_addr();
                data = self.memory.read_u8(data_addr);
                cycles = 4;
            }
            AddrMode::AbsY => {
                let data_addr = self.get_absolute_y_addr();
                data = self.memory.read_u8(data_addr);
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
                data = self.fetch_u8();
                cycles = 2;
            }
            AddrMode::ZeroPage => {
                let data_addr = self.get_zero_page_addr();
                data = self.memory.read_u8(data_addr);
                cycles = 3;
            }
            AddrMode::ZeroPageX => {
                let data_addr = self.get_zero_page_x_addr();
                data = self.memory.read_u8(data_addr);
                cycles = 4;
            }
            AddrMode::Abs => {
                let data_addr = self.get_absolute_addr();
                data = self.memory.read_u8(data_addr);
                cycles = 4;
            }
            AddrMode::AbsX => {
                let data_addr = self.get_absolute_x_addr();
                data = self.memory.read_u8(data_addr);
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
                data = self.memory.read_u8(data_addr);
                cycles = 5;
            }
            AddrMode::ZeroPageX => {
                data_addr = self.get_zero_page_x_addr();
                data = self.memory.read_u8(data_addr);
                cycles = 6;
            }
            AddrMode::Abs => {
                data_addr = self.get_absolute_addr();
                data = self.memory.read_u8(data_addr);
                cycles = 6;
            }
            AddrMode::AbsX => {
                data_addr = self.get_absolute_x_addr();
                data = self.memory.read_u8(data_addr);
                cycles = 7;
            }
            _ => panic!("Addressing mode not supported"),
        }

        self.sr.carry = (data & MASK_LSB) != 0;

        data = data >> 1;

        self.memory.write_u8(data_addr, data);

        self.set_zero_and_negative_flags(data);

        cycles
    }

    fn ora(&mut self, addr_mode: AddrMode) -> u8 {
        let data_addr: u16;
        let data: u8;
        let cycles;

        match addr_mode {
            AddrMode::Immediate => {
                data = self.fetch_u8();
                cycles = 2;
            }
            AddrMode::ZeroPage => {
                data_addr = self.get_zero_page_addr();
                data = self.memory.read_u8(data_addr);
                cycles = 3;
            }
            AddrMode::ZeroPageX => {
                data_addr = self.get_zero_page_x_addr();
                data = self.memory.read_u8(data_addr);
                cycles = 4;
            }
            AddrMode::Abs => {
                data_addr = self.get_absolute_addr();
                data = self.memory.read_u8(data_addr);
                cycles = 4;
            }
            AddrMode::AbsX => {
                data_addr = self.get_absolute_x_addr();
                data = self.memory.read_u8(data_addr);
                cycles = 4 as u8 + (data_addr > 0x00FF) as u8
            }
            AddrMode::IndX => {
                data_addr = self.get_indirect_x_addr();
                data = self.memory.read_u8(data_addr);
                cycles = 6;
            }
            AddrMode::IndY => {
                data_addr = self.get_indirect_y_addr();
                data = self.memory.read_u8(data_addr);
                cycles = 5 as u8 + (data_addr > 0xFF) as u8
            }
            _ => panic!("Addressing mode not supported"),
        }

        self.a = self.a | data;

        self.set_zero_and_negative_flags(self.a);

        cycles
    }

    fn push_accumulator(&mut self) -> u8 {
        self.stack.push_u8(self.a);

        3
    }

    fn push_processor_status(&mut self) -> u8 {
        let processor_status = self.sr.get_status_byte();
        self.stack.push_u8(processor_status);

        3
    }

    fn pull_accumulator(&mut self) -> u8 {
        let popped_acc = self.stack.pop_u8();
        self.a = popped_acc;

        self.set_zero_and_negative_flags(popped_acc);

        4
    }

    fn pull_processor_status(&mut self) -> u8 {
        let popped_sr = self.stack.pop_u8();
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
                data_addr = self.fetch_u8() as u16;
                data = self.memory.read_u8(data_addr);
                cycles = 5;
            }
            AddrMode::ZeroPageX => {
                old_carry = self.sr.carry as u8;
                data_addr = (self.fetch_u8() as u16 + self.x as u16) & 0x00FF;
                data = self.memory.read_u8(data_addr);
                cycles = 6;
            }
            AddrMode::Abs => {
                old_carry = self.sr.carry as u8;
                data_addr = self.fetch_u16();
                data = self.memory.read_u8(data_addr);
                cycles = 6;
            }
            AddrMode::AbsX => {
                old_carry = self.sr.carry as u8;
                data_addr = self.fetch_u16() + self.x as u16;
                data = self.memory.read_u8(data_addr);
                cycles = 7;
            }
            _ => panic!("Addressing mode not supported"),
        }

        self.sr.carry = (data & MASK_MSB) != 0;

        data = (data << 1) | old_carry;

        self.set_zero_and_negative_flags(data);

        self.memory.write_u8(data_addr, data);

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
                data_addr = (self.fetch_u8() as u16 + self.x as u16) & 0x00FF;
                data = self.memory.read_u8(data_addr);
                cycles = 5;
            }
            AddrMode::ZeroPageX => {
                data_addr = self.fetch_u8() as u16;
                data = self.memory.read_u8(data_addr);
                cycles = 5;
            }
            AddrMode::Abs => {
                data_addr = self.fetch_u16();
                data = self.memory.read_u8(data_addr);
                cycles = 6;
            }
            AddrMode::AbsX => {
                data_addr = self.fetch_u16() + self.x as u16;
                data = self.memory.read_u8(data_addr);
                cycles = 7;
            }
            _ => panic!("Addressing mode not supported"),
        }

        self.sr.carry = (data & MASK_MSB) != 0;

        data = data << 1;

        self.set_zero_and_negative_flags(data);

        self.memory.write_u8(data_addr, data);

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

        self.memory.write_u8(data_addr, self.a);

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

        self.memory.write_u8(data_addr, self.x);

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

        self.memory.write_u8(data_addr, self.y);

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
}
