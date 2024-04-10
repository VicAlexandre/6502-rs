pub struct Memory {
    ram: [u8; 0x10000], // 64KB
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            ram: [0; 0x10000],
        }
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    pub fn read_u16(&self, addr: u16) -> u16 {
        // Little Endian implementation
        let least_significant = self.ram[addr as usize] as u16;
        let most_significant = self.ram[(addr + 1) as usize] as u16;
        (most_significant << 8) | least_significant
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
    }
}