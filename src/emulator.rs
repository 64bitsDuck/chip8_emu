use sdl2::sys::SDL_bool::SDL_FALSE;

use crate::{cpu::Cpu, display::Display, keypad::KeyPad, memory::Ram};

type Opcode = [u8; 2];

const CLEAR: Opcode = [0x00, 0xE0];
const RET: Opcode = [0x00, 0xEE];
const GOTO: u8 = 1;
const DO: u8 = 2;
const SKIP_IF_EQUALS: u8 = 3;
const SKIP_IF_NOTEQUALS: u8 = 4;
const SKIP_IF_EQUALS_Y: u8 = 5;
const ASSIGN_HEX: u8 = 6;
const ADD: u8 = 7;

pub struct Emulator {
    cpu: Cpu,
    ram: Ram,
    display: Display,
    keypad: KeyPad,
    stack: [u8],
}

impl Emulator {
    pub fn cycle(&mut self) {
        let opcode = Self::fetch(self);
        Self::decode(self, opcode);
        Self::execute();
    }

    pub fn fetch(&mut self) -> Opcode {
        let curr_instruction: Opcode = [self.ram.0[self.cpu.pc as usize], self.ram.0[(self.cpu.pc+1) as usize]];
        self.cpu.pc += 2;
        curr_instruction
    }

    pub fn decode(&mut self, opcode: Opcode) {
        match opcode {
            JUMP => self.display.framebuffer.fill(false),
            _ => ()
        }
    }

    pub fn execute() {

    }
}