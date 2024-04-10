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

    
    pub fn fetch_instruction(&mut self) -> u8 {
        let instruction = self.memory.read_u8(self.pc);
        self.pc += 1;
        instruction
    }

    pub fn status(&self) {
        println!("A: {:#04X}", self.a);
        println!("X: {:#04X}", self.x);
        println!("Y: {:#04X}", self.y);
        println!("PC: {:#06X}", self.pc);
        self.sr.status();
        println!("SP: {:#04X}", self.stack.sp);
    }
}