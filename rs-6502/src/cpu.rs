use crate::{memory::Memory, stack::Stack, status_register::StatusRegister};

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

    pub fn execute(&mut self) -> u8 {
        let instruction = self.fetch_u8();

        match instruction {
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
            // LDA
            0xA9 => self.lda_immediate(),
            // LDA zpg
            0xA5 => self.lda_zpg(),
            // LDA zpg, X
            0xB5 => self.lda_zpg_x(),
            // LDA abs
            0xAD => self.lda_abs(),
            // LDA abs, X
            0xBD => self.lda_abs_x(),
            // LDA abs, y
            0xB9 => self.lda_abs_y(),
            // LDA (ind, x)
            0xA1 => self.lda_ind_x(),
            // LDA (ind), y
            0xB1 => self.lda_ind_y(),
            // LDX immediate
            0xA2 => self.ldx_immediate(),
            // LDX zpg
            0xA6 => self.ldx_zpg(),
            // LDX zpg, y
            0xB6 => self.ldx_zpg_y(),
            // LDX abs
            0xAE => self.ldx_abs(),
            // LDX abs, y
            0xBE => self.ldx_abs_y(),
            // LDY immediate
            0xA0 => self.ldy_immediate(),
            // LDY zpg
            0xA4 => self.ldy_zpg(),
            // LDY zpg, x
            0xB4 => self.ldy_zpg_x(),
            // LDY abs
            0xAC => self.ldy_abs(),
            // LDY abs, x
            0xBC => self.ldy_abs_x(),
            // LSR accumulator
            0x4A => self.lsr_accumulator(),
            // LSR zpg
            0x46 => self.lsr_zpg(),
            // LSR zpg, x
            0x56 => self.lsr_zpg_x(),
            // LSR abs
            0x4E => self.lsr_abs(),
            // LSR abs, x
            0x5E => self.lsr_abs_x(),
            // ORA immediate
            0x09 => self.ora_immediate(),
            // ORA zpg
            0x05 => self.ora_zpg(),
            // ORA zpg, x
            0x15 => self.ora_zpg_x(),
            // ORA abs
            0x0D => self.ora_abs(),
            // ORA abs, x
            0x1D => self.ora_abs_x(),
            // ORA (indirect, x)
            0x01 => self.ora_indirect_x(),
            // ORA (indirect), y
            0x11 => self.ora_indirect_y(),
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
            0x26 => self.rol_zpg(),
            // ROL zpg, x
            0x36 => self.rol_zpg_x(),
            // ROL abs
            0x2E => self.rol_abs(),
            // ROL abs, x
            0x3E => self.rol_abs_x(),
            // ASL accumulator
            0x0A => self.asl_acc(),
            // ASL zpg
            0x06 => self.asl_zpg(),
            // ASL zpg, X
            0x16 => self.asl_zpg_x(),
            // ASL abs
            0x0E => self.asl_abs(),
            // ASL abs, X
            0x1E => self.asl_abs_x(),
            _ => {
                panic!("Instruction not implemented: {:#04X}", instruction)
            }
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

    fn lda_immediate(&mut self) -> u8 {
        let data = self.fetch_u8();

        self.a = data;
        self.sr.zero = self.a == 0;
        self.sr.negative = (self.a & MASK_MSB) != 0;

        2
    }

    fn lda_zpg(&mut self) -> u8 {
        let data_addr = self.fetch_u8();
        let data = self.memory.read_u8(data_addr as u16);

        self.a = data;
        self.sr.zero = self.a == 0;
        self.sr.negative = (self.a & MASK_MSB) != 0;

        3
    }

    fn lda_zpg_x(&mut self) -> u8 {
        let data_addr = (self.fetch_u8() as u16 + self.x as u16) & 0x00FF;
        let data = self.memory.read_u8(data_addr);

        self.a = data;
        self.sr.zero = self.a == 0;
        self.sr.negative = (self.a & MASK_MSB) != 0;

        3
    }

    fn lda_abs(&mut self) -> u8 {
        let data_addr = self.fetch_u16();
        let data = self.memory.read_u8(data_addr);

        self.a = data;
        self.sr.zero = self.a == 0;
        self.sr.negative = (self.a & MASK_MSB) != 0;

        4
    }

    fn lda_abs_x(&mut self) -> u8 {
        let data_addr = self.fetch_u16() + self.x as u16;
        let data = self.memory.read_u8(data_addr);

        self.a = data;
        self.sr.zero = self.a == 0;
        self.sr.negative = (self.a & MASK_MSB) != 0;

        4 as u8 + (data_addr > 0x00FF) as u8
    }

    fn lda_abs_y(&mut self) -> u8 {
        let data_addr = self.fetch_u16() + self.y as u16;
        let data = self.memory.read_u8(data_addr);

        self.a = data;
        self.sr.zero = self.a == 0;
        self.sr.negative = (self.a & MASK_MSB) != 0;

        4 as u8 + (data_addr > 0x00FF) as u8
    }

    fn lda_ind_x(&mut self) -> u8 {
        let addr_addr = (self.fetch_u8() as u16 + self.x as u16) & 0x00FF;
        let data_addr = self.memory.read_u16(addr_addr);
        let data = self.memory.read_u8(data_addr);

        self.a = data;
        self.sr.zero = self.a == 0;
        self.sr.negative = (self.a & MASK_MSB) != 0;

        6
    }

    fn lda_ind_y(&mut self) -> u8 {
        let addr_addr = self.fetch_u8() as u16;
        let data_addr = self.memory.read_u16(addr_addr) + self.y as u16;
        let data = self.memory.read_u8(data_addr);

        self.a = data;
        self.sr.zero = self.a == 0;
        self.sr.negative = (self.a & MASK_MSB) != 0;

        5 as u8 + (data_addr > 0x00FF) as u8
    }

    fn ldx_immediate(&mut self) -> u8 {
        let data = self.fetch_u8();

        self.x = data;
        self.sr.zero = self.x == 0;
        self.sr.negative = (self.x & MASK_MSB) != 0;

        2
    }

    fn ldx_zpg(&mut self) -> u8 {
        let data_addr = self.fetch_u8() as u16;
        let data = self.memory.read_u8(data_addr);

        self.x = data;
        self.sr.zero = self.x == 0;
        self.sr.negative = (self.x & MASK_MSB) != 0;

        3
    }

    fn ldx_zpg_y(&mut self) -> u8 {
        let data_addr = (self.fetch_u8() as u16 + self.y as u16) & 0x00FF;
        let data = self.memory.read_u8(data_addr);

        self.x = data;
        self.sr.zero = self.x == 0;
        self.sr.negative = (self.x & MASK_MSB) != 0;

        4
    }

    fn ldx_abs(&mut self) -> u8 {
        let data_addr = self.fetch_u16();
        let data = self.memory.read_u8(data_addr);

        self.x = data;
        self.sr.zero = self.x == 0;
        self.sr.negative = (self.x & MASK_MSB) != 0;

        4
    }

    fn ldx_abs_y(&mut self) -> u8 {
        let data_addr = self.fetch_u16() + self.y as u16;
        let data = self.memory.read_u8(data_addr);

        self.x = data;
        self.sr.zero = self.x == 0;
        self.sr.negative = (self.x & MASK_MSB) != 0;

        4 as u8 + (data_addr > 0x00FF) as u8
    }

    fn ldy_immediate(&mut self) -> u8 {
        let data = self.fetch_u8();

        self.y = data;
        self.sr.zero = self.y == 0;
        self.sr.negative = (self.y & MASK_MSB) != 0;

        2
    }

    fn ldy_zpg(&mut self) -> u8 {
        let data_addr = self.fetch_u8() as u16;
        let data = self.memory.read_u8(data_addr);

        self.y = data;
        self.sr.zero = self.y == 0;
        self.sr.negative = (self.y & MASK_MSB) != 0;

        3
    }

    fn ldy_zpg_x(&mut self) -> u8 {
        let data_addr = (self.fetch_u8() as u16 + self.x as u16) & 0x00FF;
        let data = self.memory.read_u8(data_addr);

        self.y = data;
        self.sr.zero = self.y == 0;
        self.sr.negative = (self.y & MASK_MSB) != 0;

        4
    }

    fn ldy_abs(&mut self) -> u8 {
        let data_addr = self.fetch_u16();
        let data = self.memory.read_u8(data_addr);

        self.y = data;
        self.sr.zero = self.y == 0;
        self.sr.negative = (self.y & MASK_MSB) != 0;

        4
    }

    fn ldy_abs_x(&mut self) -> u8 {
        let data_addr = self.fetch_u16() + self.x as u16;
        let data = self.memory.read_u8(data_addr);

        self.y = data;
        self.sr.zero = self.y == 0;
        self.sr.negative = (self.y & MASK_MSB) != 0;

        4 as u8 + (data_addr > 0x00FF) as u8
    }

    fn lsr_accumulator(&mut self) -> u8 {
        self.sr.carry = (self.a & MASK_LSB) != 0;
        self.a = self.a >> 1;
        self.sr.zero = self.a == 0;
        self.sr.negative = false;

        2
    }

    fn lsr_zpg(&mut self) -> u8 {
        let data_addr = self.fetch_u8() as u16;
        let mut data = self.memory.read_u8(data_addr);

        self.sr.carry = (data & MASK_LSB) != 0;

        data = data >> 1;

        self.memory.write_u8(data_addr, data);

        self.sr.zero = data == 0;
        self.sr.negative = false;

        5
    }

    fn lsr_zpg_x(&mut self) -> u8 {
        let data_addr = (self.fetch_u8() as u16 + self.x as u16) & 0x00FF;
        let mut data = self.memory.read_u8(data_addr);

        self.sr.carry = (data & MASK_LSB) != 0;

        data = data >> 1;

        self.memory.write_u8(data_addr, data);

        self.sr.zero = data == 0;
        self.sr.negative = false;

        6
    }

    fn lsr_abs(&mut self) -> u8 {
        let data_addr = self.fetch_u16();
        let mut data = self.memory.read_u8(data_addr);

        self.sr.carry = (data & MASK_LSB) != 0;

        data = data >> 1;

        self.memory.write_u8(data_addr, data);

        self.sr.zero = data == 0;
        self.sr.negative = false;

        6
    }

    fn lsr_abs_x(&mut self) -> u8 {
        let data_addr = self.fetch_u16() + self.x as u16;
        let mut data = self.memory.read_u8(data_addr);

        self.sr.carry = (data & MASK_LSB) != 0;

        data = data >> 1;

        self.memory.write_u8(data_addr, data);

        self.sr.zero = data == 0;
        self.sr.negative = false;

        7
    }

    fn ora_immediate(&mut self) -> u8 {
        let data = self.fetch_u8();

        self.a = self.a | data;
        self.sr.zero = self.a == 0;
        self.sr.negative = (self.a & MASK_MSB) != 0;

        2
    }

    fn ora_zpg(&mut self) -> u8 {
        let data_addr = self.fetch_u8() as u16;
        let data = self.memory.read_u8(data_addr);

        self.a = self.a | data;
        self.sr.zero = self.a == 0;
        self.sr.negative = (self.a & MASK_MSB) != 0;

        3
    }

    fn ora_zpg_x(&mut self) -> u8 {
        let data_addr = (self.fetch_u8() as u16 + self.x as u16) & 0x00FF;
        let data = self.memory.read_u8(data_addr);

        self.a = self.a | data;
        self.sr.zero = self.a == 0;
        self.sr.negative = (self.a & MASK_MSB) != 0;

        4
    }

    fn ora_abs(&mut self) -> u8 {
        let data_addr = self.fetch_u16();
        let data = self.memory.read_u8(data_addr);

        self.a = self.a | data;
        self.sr.zero = self.a == 0;
        self.sr.negative = (self.a & MASK_MSB) != 0;

        4
    }

    fn ora_abs_x(&mut self) -> u8 {
        let data_addr = self.fetch_u16() + self.x as u16;
        let data = self.memory.read_u8(data_addr);

        self.a = self.a | data;
        self.sr.zero = self.a == 0;
        self.sr.negative = (self.a & MASK_MSB) != 0;

        4 as u8 + (data_addr > 0x00FF) as u8
    }

    fn ora_indirect_x(&mut self) -> u8 {
        let addr_addr = (self.fetch_u8() as u16 + self.x as u16) & 0xFF;
        let data_addr = self.memory.read_u16(addr_addr);
        let data = self.memory.read_u8(data_addr);

        self.a = self.a | data;
        self.sr.zero = self.a == 0;
        self.sr.negative = (self.a & MASK_MSB) != 0;

        6
    }

    fn ora_indirect_y(&mut self) -> u8 {
        let addr_addr = self.fetch_u8() as u16;
        let data_addr = addr_addr + self.y as u16;
        let data = self.memory.read_u8(data_addr);

        self.a = self.a | data;
        self.sr.zero = self.a == 0;
        self.sr.negative = (self.a & MASK_MSB) != 0;

        5 as u8 + (data_addr > 0xFF) as u8
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
        self.sr.zero = popped_acc == 0;
        self.sr.negative = (popped_acc & MASK_MSB) != 0;

        4
    }

    fn pull_processor_status(&mut self) -> u8 {
        let popped_sr = self.stack.pop_u8();
        self.sr.set_status_byte(popped_sr);

        4
    }

    fn rol_accumulator(&mut self) -> u8 {
        let old_carry = self.sr.carry as u8;
        self.sr.carry = (self.a & MASK_MSB) != 0;
        self.a = (self.a << 1) | old_carry;
        self.sr.zero = self.a == 0;

        2
    }

    fn rol_zpg(&mut self) -> u8 {
        let old_carry = self.sr.carry as u8;
        let data_addr = self.fetch_u8() as u16;
        let mut data = self.memory.read_u8(data_addr);

        self.sr.carry = (data & MASK_MSB) != 0;

        data = (data << 1) | old_carry;

        self.sr.zero = data == 0;
        self.sr.negative = (data & MASK_MSB) != 0;
        
        self.memory.write_u8(data_addr, data);

        5
    }

    fn rol_zpg_x(&mut self) -> u8 {
        let old_carry = self.sr.carry as u8;
        let data_addr = (self.fetch_u8() as u16 + self.x as u16) & 0x00FF;
        let mut data = self.memory.read_u8(data_addr);

        self.sr.carry = (data & MASK_MSB) != 0;

        data = (data << 1) | old_carry;

        self.sr.zero = data == 0;
        self.sr.negative = (data & MASK_MSB) != 0;

        self.memory.write_u8(data_addr, data);

        6
    }

    fn rol_abs(&mut self) -> u8 {
        let old_carry = self.sr.carry as u8;
        let data_addr = self.fetch_u16();
        let mut data = self.memory.read_u8(data_addr);

        self.sr.carry = (data & MASK_MSB) != 0;
        
        data = (data << 1) | old_carry;
        
        self.sr.zero = data == 0;
        self.sr.negative = (data & MASK_MSB) != 0;

        self.memory.write_u8(data_addr, data);

        6
    }

    fn rol_abs_x(&mut self) -> u8 {
        let old_carry = self.sr.carry as u8;
        let data_addr = self.fetch_u16() + self.x as u16;
        let mut data = self.memory.read_u8(data_addr);

        self.sr.carry = (data & MASK_MSB) != 0;
        
        data = (data << 1) | old_carry;

        self.sr.zero = data == 0;
        self.sr.negative = (data & MASK_MSB) != 0;

        self.memory.write_u8(data_addr, data);

        7
    }

    fn asl_acc(&mut self) -> u8 {
        let old_byte = self.a;

        self.sr.carry = (old_byte & MASK_MSB) != 0;
        
        self.a = self.a << 1;
        
        self.sr.zero = self.a == 0;
        self.sr.negative = (self.a & MASK_MSB) != 0;

        2
    }

    fn asl_zpg(&mut self) -> u8 {
        let data_addr = (self.fetch_u8() as u16 + self.x as u16) & 0x00FF;
        let mut data = self.memory.read_u8(data_addr);

        self.sr.carry = (data & MASK_MSB) != 0;
        
        data = data << 1;
        
        self.sr.zero = data == 0;
        self.sr.negative = (data & MASK_MSB) != 0;

        self.memory.write_u8(data_addr, data);

        5
    }

    fn asl_zpg_x(&mut self) -> u8 {
        let data_addr = self.fetch_u8() as u16;
        let mut data = self.memory.read_u8(data_addr);

        self.sr.carry = (data & MASK_MSB) != 0;
        
        data = data << 1;
        
        self.sr.zero = data == 0;
        self.sr.negative = (data & MASK_MSB) != 0;

        self.memory.write_u8(data_addr, data);

        6
    }

    fn asl_abs(&mut self) -> u8 {
        let data_addr = self.fetch_u16();
        let mut data = self.memory.read_u8(data_addr);

        self.sr.carry = (data & MASK_MSB) != 0;
        
        data = data << 1;
        
        self.sr.zero = data == 0;
        self.sr.negative = (data & MASK_MSB) != 0;

        self.memory.write_u8(data_addr, data);

        6
    }

    fn asl_abs_x(&mut self) -> u8 {
        let data_addr = self.fetch_u16() + self.x as u16;
        let mut data = self.memory.read_u8(data_addr);

        self.sr.carry = (data & MASK_MSB) != 0;
        
        data = data << 1;
        
        self.sr.zero = data == 0;
        self.sr.negative = (data & MASK_MSB) != 0;

        self.memory.write_u8(data_addr, data);

        7
    }
}
