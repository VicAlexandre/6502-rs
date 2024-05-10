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

    pub fn push_byte(&mut self, data: u8) {
        self.stack[self.sp as usize] = data;
        self.sp -= 1;
    }

    pub fn pop_byte(&mut self) -> u8 {
        self.sp += 1;

        self.stack[self.sp as usize]
    }

    pub fn push_word(&mut self, data: u16) {
        self.stack[self.sp as usize] = (data >> 8) as u8;
        self.stack[(self.sp - 1) as usize] = data as u8;
        self.sp -= 2;
    }

    pub fn pop_word(&mut self) -> u16 {
        let word: u16;

        word = ((self.stack[(self.sp + 1) as usize] as u16)) | ((self.stack[(self.sp + 2) as usize] as u16) << 8);

        self.sp += 2;

        word
    }

    pub fn get_sp(&self) -> u8 {
        self.sp 
    }

    pub fn get_stack(&self) -> Vec<u8> {
        let stack_vec = self.stack.to_vec();

        stack_vec
    }
}
