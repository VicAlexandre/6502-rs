pub struct Stack {
    pub stack: [u8; 0x0100], // 256B
    pub sp: u8,
}

impl Stack {
    #![allow(unused)]
    pub fn new() -> Stack {
        Stack {
            stack: [0; 0x0100],
            sp: 0xFF,
        }
    }

    pub fn push(&mut self, data: u8) {
        self.stack[self.sp as usize];
        self.sp -= 1;
    }

    pub fn pop(&mut self) -> u8 {
        self.sp += 1;
        self.stack[self.sp as usize]
    }
}