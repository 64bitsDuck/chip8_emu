mod cpu;
mod memory;
mod display;
mod keypad;
mod emulator;
use std::println;

use cpu::Cpu;

use crate::{display::Display, emulator::Emulator, memory::Ram, keypad::KeyPad};

const HEIGHT: u8 = 32;
const WIDTH: u8 = 64;

fn main() {
    let cpu: Cpu = Cpu::new();

    let mut ram = Ram::new();
    ram.load_rom("roms/IBM Logo.ch8".to_string());

    let mut display = Display::new(HEIGHT, WIDTH);

    let keypad = KeyPad::new();

    let mut emulator = Emulator::new(cpu, ram, &mut display, keypad);
 
    emulator.cycle();

    display.show();
}
