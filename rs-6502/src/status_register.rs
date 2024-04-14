pub struct StatusRegister {
    pub negative: bool,
    pub overflow: bool,
    pub brk: bool,
    pub decimal: bool,
    pub interrupt_disable: bool,
    pub zero: bool,
    pub carry: bool,
}

impl StatusRegister {
    #![allow(unused)]
    pub fn new() -> StatusRegister {
        StatusRegister {
            negative: false,
            overflow: false,
            brk: false,
            decimal: false,
            interrupt_disable: false,
            zero: false,
            carry: false,
        }
    }

    pub fn status(&self) {
        print!("N: {}\t", self.negative as u8);
        print!("V: {}\t", self.overflow as u8);
        print!("B: {}\t", self.brk as u8);
        print!("D: {}\t", self.decimal as u8);
        print!("I: {}\t", self.interrupt_disable as u8);
        print!("Z: {}\t", self.zero as u8);
        print!("C: {}\n", self.carry as u8);
    }

    pub fn get_status_byte(&self) -> u8 {
        let negative = (self.negative as u8) << 7;
        let overflow = (self.overflow as u8) << 6;
        let brk = (self.brk as u8) << 4;
        let decimal = (self.decimal as u8) << 3;
        let interrupt = (self.interrupt_disable as u8) << 2;
        let zero = (self.zero as u8) << 1;
        let carry = self.carry as u8;

        // 0bNOBDIZC0
        negative | overflow | brk | decimal | interrupt | zero | carry
    }
}