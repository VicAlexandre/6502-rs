use crate::cpu::Cpu;
// use std::fs;
use std::env;

mod cpu;
mod memory;
mod stack;
mod status_register;

fn main() {
    #![allow(unused_variables)]
    println!("# RS-6502 Emulator #");

    let mut cpu = Cpu::new();

    if env::args().len() < 2 {
        println!("Usage: cargo run <path-to-rom>");
        return;
    }

    let rom_path = env::args().nth(1).unwrap();
    let rom = std::fs::read(rom_path).unwrap();

    cpu.memory.load(rom);
    print!("{}", cpu.memory);
}