pub struct StatusRegister {
    pub negative: bool,
    pub overflow: bool,
    pub ignored: bool,
    pub brk: bool,
    pub decimal: bool,
    pub interrupt: bool,
    pub zero: bool,
    pub carry: bool,
}

impl StatusRegister {
    #![allow(unused)]
    pub fn new() -> StatusRegister {
        StatusRegister {
            negative: false,
            overflow: false,
            ignored: true,
            brk: false,
            decimal: false,
            interrupt: true,
            zero: false,
            carry: false,
        }
    }

    pub fn status(&self) {
        println!("N: {}", self.negative);
        println!("V: {}", self.overflow);
        println!("I: {}", self.ignored);
        println!("B: {}", self.brk);
        println!("D: {}", self.decimal);
        println!("I: {}", self.interrupt);
        println!("Z: {}", self.zero);
        println!("C: {}", self.carry);
    }

    pub fn get_status_byte(&self) -> u8 {
        let negative = (self.negative as u8) << 7;
        let overflow = (self.overflow as u8) << 6;
        let ignored = (self.ignored as u8) << 5;
        let brk = (self.brk as u8) << 4;
        let decimal = (self.decimal as u8) << 3;
        let interrupt = (self.interrupt as u8) << 2;
        let zero = (self.zero as u8) << 1;
        let carry = self.carry as u8;

        negative | overflow | ignored | brk | decimal | interrupt | zero | carry
    }
}