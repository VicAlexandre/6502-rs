use crate::{memory::Memory, stack::Stack, status_register::StatusRegister};

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
    #![allow(unused)]
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
        let cycles: u8;

        match instruction {
            //BRK
            0x00 => {
                self.break_interrupt()
            }
            // CLC
            0x18 => {
                self.clear_carry()
            }
            // CLD
            0xD8 => {
                self.clear_decimal()
            }
            // CLI
            0x58 => {
                self.clear_interrupt_dis()
            }
            // CLV
            0xB8 => {
                self.clear_overflow()
            }
            // NOP
            0xEA => {
                2
            }
            // INY
            0xC8 => {
                self.inc_y()
            }
            // INX
            0xE8 => {
                self.inc_x()
            }
            // LDA
            0xA9 => {
                self.lda_immediate()
            }
            // LDA zpg
            0xA5 => {
                self.lda_zpg()
            }
            // LDA zpg, X
            0xB5 => {
                self.lda_zpg_x()
            }
            // LDA abs
            0xAD => {
                self.lda_abs()
            }
            // LDA abs, X
            0xBD => {
                self.lda_abs_x()
            }
            // LDA abs, y
            0xB9 => {
                self.lda_abs_y()
            }
            // LDA (ind, x)
            0xA1 => {
                self.lda_ind_x()
            }
            // LDA (ind), y
            0xB1 => {
                self.lda_ind_y()
            }
            // LDX immediate
            0xA2 => {
                    self.lda_immediate()
            }
            // LDX zpg
            0xA6 => {
                self.ldx_zpg()
            }
            // LDX zpg, y
            0xB6 => {
                self.ldx_zpg_y()
            }
            // LDX abs
            0xAE => {
                self.ldx_abs()
            }
            // LDX abs, y
            0xBE => {
                self.ldx_abs_y()
            }
            // LDY immediate
            0xA0 => {
                self.ldy_immediate()
            }
            // LDY zpg
            0xA4 => {
                self.ldy_zpg()
            }
            _ => {
                panic!("Instruction not implemented: {:#04X}", instruction)
            }
        }
    }
    
    pub fn break_interrupt(&mut self) -> u8 {
        self.stack.push_u16(self.pc);
        self.stack.push_u8(self.sr.get_status_byte());
        self.pc = self.memory.read_u16(0xFFFE);
        self.sr.brk = true;

        7
    }

    pub fn clear_carry(&mut self) -> u8 {
        self.sr.carry = false;

        2
    }

    pub fn clear_decimal(&mut self) -> u8 {
        self.sr.carry = false;

        2
    }

    pub fn clear_interrupt_dis(&mut self) -> u8 {
        self.sr.interrupt_disable = false;

        2
    }    

    pub fn clear_overflow(&mut self) -> u8 {
        self.sr.overflow = false;

        2
    }   

    pub fn inc_y(&mut self) -> u8 {
        self.y += 1;
        
        2
    }

    pub fn inc_x(&mut self) -> u8 {
        self.x += 1;
        
        2
    }

    pub fn lda_immediate(&mut self) -> u8 {
        let data = self.fetch_u8();
        self.a = data;
        self.sr.zero = self.a == 0;
        self.sr.negative = self.a & 0b10000000 != 0;

        2
    }

    pub fn lda_zpg(&mut self) -> u8 {
        let data_addr = self.fetch_u8();
        let data = self.memory.read_u8(data_addr as u16);
        self.a = data;
        self.sr.zero = self.a == 0;
        self.sr.negative = self.a & 0b10000000 != 0;

        3
    }

    pub fn lda_zpg_x(&mut self) -> u8 {
        let data_addr = (self.fetch_u8() as u16 + self.x as u16) & 0x00FF;
        let data = self.memory.read_u8(data_addr);
        self.a = data;
        self.sr.zero = self.a == 0;
        self.sr.negative = self.a & 0b10000000 != 0;

        3
    }
    
    pub fn lda_abs(&mut self) -> u8 {
        let data_addr = self.fetch_u16();
        let data = self.memory.read_u8(data_addr);
        self.a = data;
        self.sr.zero = self.a == 0;
        self.sr.negative = self.a & 0b10000000 != 0;

        4
    }
    
    pub fn lda_abs_x(&mut self) -> u8 {
        let data_addr = self.fetch_u16() + self.x as u16;
        let data = self.memory.read_u8(data_addr);
        self.a = data;
        self.sr.zero = self.a == 0;
        self.sr.negative = self.a & 0b10000000 != 0;

        4 as u8 + (data_addr > 0x00FF) as u8
    }

    pub fn lda_abs_y(&mut self) -> u8 {
        let data_addr = self.fetch_u16() + self.y as u16;
        let data = self.memory.read_u8(data_addr);
        self.a = data;
        self.sr.zero = self.a == 0;
        self.sr.negative = self.a & 0b10000000 != 0;

        4 as u8 + (data_addr > 0x00FF) as u8
    }
    
    pub fn lda_ind_x(&mut self) -> u8 {
        let addr_addr = (self.fetch_u8() as u16 + self.x as u16) & 0x00FF;
        let data_addr = self.memory.read_u16(addr_addr);
        let data = self.memory.read_u8(data_addr);
        self.a = data;
        self.sr.zero = self.a == 0;
        self.sr.negative = self.a & 0b10000000 != 0;

        6
    }
    
    pub fn lda_ind_y(&mut self) -> u8 {
        let addr_addr = self.fetch_u8() as u16;
        let data_addr = self.memory.read_u16(addr_addr) + self.y as u16;
        let data = self.memory.read_u8(data_addr);
        self.a = data;
        self.sr.zero = self.a == 0;
        self.sr.negative = self.a & 0b10000000 != 0;

        5 as u8 + (data_addr > 0x00FF) as u8
    }

    pub fn ldx_immediate(&mut self) -> u8{
        let data = self.fetch_u8();
        self.x = data;
        self.sr.zero = self.x == 0;
        self.sr.negative = self.x & 0b10000000 != 0;

        2
    }

    pub fn ldx_zpg(&mut self) -> u8 {
        let data_addr = self.fetch_u8() as u16;
        let data = self.memory.read_u8(data_addr);
        self.x = data;
        self.sr.zero = self.x == 0;
        self.sr.negative = self.x & 0b10000000 != 0;

        3
    }

    pub fn ldx_zpg_y(&mut self) -> u8 {
        let data_addr = (self.fetch_u8() as u16 + self.y as u16) & 0x00FF;
        let data = self.memory.read_u8(data_addr);
        self.x = data;
        self.sr.zero = self.x == 0;
        self.sr.negative = self.x & 0b10000000 != 0;
        
        4
    }

    pub fn ldx_abs(&mut self) -> u8 {
        let data_addr = self.fetch_u16();
        let data = self.memory.read_u8(data_addr);
        self.x = data;
        self.sr.zero = self.x == 0;
        self.sr.negative = self.x & 0b10000000 != 0;

        4
    }

    pub fn ldx_abs_y(&mut self) -> u8 {
        let data_addr = self.fetch_u16() + self.y as u16;
        let data = self.memory.read_u8(data_addr);

        4 as u8 + (data_addr > 0x00FF) as u8
    }

    pub fn ldy_immediate(&mut self) -> u8 {
        let data = self.fetch_u8();
        self.y = data;
        self.sr.zero = self.y == 0;
        self.sr.negative = self.y & 0b10000000 != 0;

        2
    }

    pub fn ldy_zpg(&mut self) -> u8 {
        let data_addr = self.fetch_u8() as u16;
        let data = self.memory.read_u8(data_addr);
        self.y = data;
        self.sr.zero = self.y == 0;
        self.sr.negative = self.y & 0b10000000 != 0;

        3
    }
}