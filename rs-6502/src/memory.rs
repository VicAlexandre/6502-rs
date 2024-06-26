use std::{fmt, vec};

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

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        // Little Endian implementation
        let least_significant = self.ram[addr as usize] as u16;
        let most_significant = self.ram[(addr + 1) as usize] as u16;
        (most_significant << 8) | least_significant
    }

    pub fn write_byte(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
    }

    pub fn load(&mut self, data: Vec<u8>) {
        for (i, byte) in data.iter().enumerate() {
            self.ram[i] = *byte;
        }
    }

    pub fn get_ram(&self, first_index: usize, mut size: usize) -> Vec<u8> {
        if first_index as u32 + size as u32 > 0x10000 {
            let new_size = 0x10000 - first_index;
            size = new_size;
        }

        let mut ram_vec = vec![0; size];
        for i in 0..size {
            ram_vec[i] = self.ram[first_index + i];
        }

        ram_vec
    }
}
