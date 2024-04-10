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
}