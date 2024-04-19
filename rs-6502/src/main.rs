use crate::cpu::Cpu;
use std::env;

mod addressing_mode;
mod cpu;
mod memory;
mod stack;
mod status_register;

fn main() {
    println!("# RS-6502 Emulator #");

    let mut cpu = Cpu::new();
    let mut cycles: u32 = 0;

    if env::args().len() < 2 {
        println!("Usage: cargo run <path-to-rom>");
        return;
    }

    let rom_path = env::args().nth(1).unwrap();
    let rom = std::fs::read(rom_path).unwrap();

    cpu.memory.load(rom);
    print!("{}", cpu.memory);

    loop {
        cycles += cpu.execute() as u32;
        println!("Cycles: {}", cycles);
        cpu.status();

        // wait for a key press
        println!("Press Enter to continue...");
        std::io::stdin().read_line(&mut String::new()).unwrap();
    }
}
