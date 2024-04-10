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
}