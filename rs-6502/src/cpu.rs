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
    }

    fn set_zero_and_negative_flags(&mut self, data: u8) {
        self.sr.zero = data == 0;
        self.sr.negative = (data & MASK_MSB) != 0;
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
            // LDA immediate
            // LDA zpg
            // LDA zpg, X
            // LDA abs
            // LDA abs, X
            // LDA abs, y
            // LDA (ind, x)
            // LDA (ind), y
            0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => self.lda(addr_mode),
            // LDX immediate
            // LDX zpg
            // LDX zpg, y
            // LDX abs
            // LDX abs, y
            0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => self.ldx(addr_mode),
            // LDY immediate
            // LDY zpg
            // LDY zpg, x
            // LDY abs
            // LDY abs, x
            0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => self.ldy(addr_mode),
            // LSR accumulator
            0x4A => self.lsr_accumulator(),
            // LSR zpg
            // LSR zpg, x
            // LSR abs
            // LSR abs, x
            0x46 | 0x56 | 0x4E | 0x5E => self.lsr(addr_mode),
            // ORA immediate
            // ORA zpg
            // ORA abs
            // ORA zpg, x
            // ORA abs, x
            // ORA (indirect, x)
            // ORA (indirect), y
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
            // ROL zpg
            // ROL zpg, x
            // ROL abs
            // ROL abs, x
            0x26 | 0x36 | 0x2E | 0x3E => self.rol(addr_mode),
            // ASL accumulator
            0x0A => self.asl_acc(),
            // ASL zpg
            // ASL zpg, X
            // ASL abs
            // ASL abs, X
            0x06 | 0x16 | 0x0E | 0x1E => self.asl(addr_mode),
            _ => panic!("Instruction not implemented: {:#04X}", opcode),
        }
    }

    fn break_interrupt(&mut self) -> u8 {
        self.stack.push_u16(self.pc);
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
                let data_addr = self.fetch_u8() as u16;
                data = self.memory.read_u8(data_addr);

                cycles = 3;
            }
            AddrMode::ZeroPageX => {
                let data_addr = (self.fetch_u8() as u16 + self.x as u16) & 0x00FF;
                data = self.memory.read_u8(data_addr);

                cycles = 3;
            }
            AddrMode::Abs => {
                let data_addr = self.fetch_u16();
                data = self.memory.read_u8(data_addr);

                cycles = 4;
            }
            AddrMode::AbsX => {
                let data_addr = self.fetch_u16() + self.x as u16;
                data = self.memory.read_u8(data_addr);

                cycles = 4 as u8 + (data_addr > 0x00FF) as u8;
            }
            AddrMode::AbsY => {
                let data_addr = self.fetch_u16() + self.y as u16;
                data = self.memory.read_u8(data_addr);

                cycles = 4 as u8 + (data_addr > 0x00FF) as u8;
            }
            AddrMode::IndX => {
                let addr_addr = (self.fetch_u8() as u16 + self.x as u16) & 0x00FF;
                let data_addr = self.memory.read_u16(addr_addr);
                data = self.memory.read_u8(data_addr);

                cycles = 6;
            }
            AddrMode::IndY => {
                let addr_addr = self.fetch_u8() as u16;
                let data_addr = self.memory.read_u16(addr_addr) + self.y as u16;
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
                let data_addr = self.fetch_u8();
                data = self.memory.read_u8(data_addr as u16);

                cycles = 3;
            }
            AddrMode::ZeroPageY => {
                let data_addr = (self.fetch_u8() as u16 + self.y as u16) & 0x00FF;
                data = self.memory.read_u8(data_addr);

                cycles = 4;
            }
            AddrMode::Abs => {
                let data_addr = self.fetch_u16();
                data = self.memory.read_u8(data_addr);

                cycles = 4;
            }
            AddrMode::AbsY => {
                let data_addr = self.fetch_u16() + self.y as u16;
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
                let data_addr = self.fetch_u8() as u16;
                data = self.memory.read_u8(data_addr);

                cycles = 3;
            }
            AddrMode::ZeroPageX => {
                let data_addr = (self.fetch_u8() as u16 + self.x as u16) & 0x00FF;
                data = self.memory.read_u8(data_addr);

                cycles = 4;
            }
            AddrMode::Abs => {
                let data_addr = self.fetch_u16();
                data = self.memory.read_u8(data_addr);

                cycles = 4;
            }
            AddrMode::AbsX => {
                let data_addr = self.fetch_u16() + self.x as u16;
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
                data_addr = self.fetch_u8() as u16;
                data = self.memory.read_u8(data_addr);

                cycles = 5;
            }
            AddrMode::ZeroPageX => {
                data_addr = (self.fetch_u8() as u16 + self.x as u16) & 0x00FF;
                data = self.memory.read_u8(data_addr);

                cycles = 6;
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

        self.sr.carry = (data & MASK_LSB) != 0;

        data = data >> 1;

        self.memory.write_u8(data_addr, data);

        self.set_zero_and_negative_flags(data);

        cycles
    }

    fn ora(&mut self, addr_mode: AddrMode) -> u8 {
        let data_addr: u16;
        let data: u8;
        let addr_addr: u16;
        let cycles;

        match addr_mode {
            AddrMode::Immediate => {
                data = self.fetch_u8();
                cycles = 2;
            }
            AddrMode::ZeroPage => {
                data_addr = self.fetch_u8() as u16;
                data = self.memory.read_u8(data_addr);
                cycles = 3;
            }
            AddrMode::ZeroPageX => {
                data_addr = (self.fetch_u8() as u16 + self.x as u16) & 0x00FF;
                data = self.memory.read_u8(data_addr);
                cycles = 4;
            }
            AddrMode::Abs => {
                data_addr = self.fetch_u16();
                data = self.memory.read_u8(data_addr);
                cycles = 4;
            }
            AddrMode::AbsX => {
                data_addr = self.fetch_u16() + self.x as u16;
                data = self.memory.read_u8(data_addr);
                cycles = 4 as u8 + (data_addr > 0x00FF) as u8
            }
            AddrMode::IndX => {
                addr_addr = (self.fetch_u8() as u16 + self.x as u16) & 0xFF;
                data_addr = self.memory.read_u16(addr_addr);
                data = self.memory.read_u8(data_addr);
                cycles = 6;
            }
            AddrMode::IndY => {
                addr_addr = self.fetch_u8() as u16;
                data_addr = addr_addr + self.y as u16;
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
}
