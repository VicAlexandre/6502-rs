use std::fmt;

pub struct Memory {
    ram: [u8; 0x10000], // 64KB
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for i in self.ram.iter() {
            result.push(*i as char);
        }
        write!(f, "{}\n", result)
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory { ram: [0; 0x10000] }
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

    pub fn write_u8(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
    }

    // pub fn write_u16(&mut self, addr: u16, data: u16) {
    //     // Little Endian implementation
    //     let least_significant = data as u8;
    //     let most_significant = (data >> 8) as u8;
    //     self.ram[addr as usize] = least_significant;
    //     self.ram[(addr + 1) as usize] = most_significant;
    // }

    pub fn load(&mut self, data: Vec<u8>) {
        for (i, byte) in data.iter().enumerate() {
            self.ram[i] = *byte;
        }
    }
}
