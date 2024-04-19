pub struct Stack {
    pub stack: [u8; 0x0100], // 256B
    pub sp: u8,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack: [0; 0x0100],
            sp: 0xFF,
        }
    }

    pub fn push_u8(&mut self, data: u8) {
        self.stack[self.sp as usize] = data;
        self.sp -= 1;
    }

    pub fn pop_u8(&mut self) -> u8 {
        self.sp += 1;

        self.stack[self.sp as usize]
    }

    pub fn push_u16(&mut self, data: u16) {
        self.stack[self.sp as usize] = data as u8;
        self.stack[(self.sp - 1) as usize] = (data >> 8) as u8;
        self.sp -= 2;
    }

    // pub fn pop_u16(&mut self) -> u16 {
    //     self.sp += 2;
    //     let least_significant = self.stack[(self.sp) as usize] as u16;
    //     let most_significant = self.stack[(self.sp - 1) as usize] as u16;

    //     (most_significant << 8) | least_significant
    // }
}