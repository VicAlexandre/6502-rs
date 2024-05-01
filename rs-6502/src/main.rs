use crate::{cpu::Cpu, tui::App};
use std::{env, io};

mod addressing_mode;
mod cpu;
mod memory;
mod stack;
mod status_register;
mod tui;

fn main() -> io::Result<()> {
    if env::args().len() < 2 {
        println!("Usage: cargo run <path-to-rom>");
        panic!("No ROM file specified")
    }
    
    let mut cpu = Cpu::new();
    let rom_path = env::args().nth(1).unwrap();
    let rom = std::fs::read(rom_path).unwrap();

    cpu.memory.load(rom);

    let mut terminal = tui::init()?;
    let app_result = App::new(cpu).run(&mut terminal);
    tui::restore()?;

    app_result
}
