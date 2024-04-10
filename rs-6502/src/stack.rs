pub struct Stack {
    pub stack: [u8; 0x0100], // 256B
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack: [0; 0x0100],
        }
    }
}