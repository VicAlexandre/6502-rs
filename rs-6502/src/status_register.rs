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

        // 0bNO0BDIZC
        negative | overflow | brk | decimal | interrupt | zero | carry
    }

    pub fn set_status_byte(&mut self, status_byte: u8) {
        self.negative = (status_byte & 0b10000000) != 0;
        self.overflow = (status_byte & 0b01000000) != 0;
        self.brk = (status_byte & 0b00010000) != 0;
        self.decimal = (status_byte & 0b00001000) != 0;
        self.interrupt_disable = (status_byte & 0b00000100) != 0;
        self.zero = (status_byte & 0b00000010) != 0;
        self.carry = (status_byte & 0b00000001) != 0;
    }
}