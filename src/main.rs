mod cpu;
mod memory;
mod display;
mod keypad;
mod emulator;

use std::{thread::sleep, time::Duration};

use cpu::Cpu;

use crate::{display::Display, emulator::Emulator, memory::Ram, keypad::KeyPad};

const HEIGHT: usize = 32;
const WIDTH: usize = 64;

fn main() {
    let cpu: Cpu = Cpu::new();

    let mut ram = Ram::new();
    ram.load_rom("roms/3-corax+.ch8".to_string());

    let mut display = Display::new(HEIGHT, WIDTH);

    let keypad = KeyPad::new();

    let stack: Vec<u16> = Vec::new();

    let mut emulator = Emulator::new(cpu, ram, &mut display, keypad, stack);
 
    emulator.cycle();

    while display.can_continue() {
        sleep(Duration::new(1, 0));
    }
}
