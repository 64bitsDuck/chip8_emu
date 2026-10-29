mod cpu;
mod memory;
mod display;
mod keypad;
mod emulator;

use std::{fs::File, io, thread::sleep, time::Duration};

use cpu::Cpu;

use crate::{display::Display, emulator::Emulator, keypad::KeyPad, memory::{Memory, Rom}};

const HEIGHT: usize = 32;
const WIDTH: usize = 64;

fn main() -> io::Result<()> {
    let cpu: Cpu = Cpu::new();

    let mut rom_file = File::open("roms/TETRIS.ch8")?;
    let rom = Rom::from_reader(&mut rom_file)?;
    let mut ram = Memory::new(rom);

    let mut display = Display::new(HEIGHT, WIDTH);

    let keypad = KeyPad::new();

    let stack: Vec<u16> = Vec::new();

    let mut emulator = Emulator::new(cpu, &mut ram, &mut display, keypad, stack);
 
    emulator.cycle();

    while display.can_continue() {
        sleep(Duration::new(1, 0));
    }

    Ok(())
}
