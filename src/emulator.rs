use std::println;

use crate::{cpu::Cpu, display::Display, keypad::KeyPad, memory::Ram};

type Opcode = [u8; 4];

const CLEAR: Opcode = [0x0, 0x0, 0xE, 0x0];
const RET: Opcode = [0x0, 0x0, 0xE, 0xE];
const GOTO: u8 = 1;
const DO: u8 = 2;
const SKIP_IF_EQUALS: u8 = 3;
const SKIP_IF_NOTEQUALS: u8 = 4;
const SKIP_IF_EQUALS_Y: u8 = 5;
const SET: u8 = 6;
const ADD: u8 = 7;
const SET_I: u8 = 0xA;
const DRAW: u8 = 0xD;

pub struct Emulator<'a> {
    cpu: Cpu,
    ram: Ram,
    display: &'a mut Display,
    keypad: KeyPad,
}

impl<'a> Emulator<'a> {
    pub fn new(cpu: Cpu, ram: Ram, display: &mut Display, keypad: KeyPad) -> Emulator<'_> {
        Emulator { cpu, ram, display: display, keypad }
    }

    pub fn cycle(&mut self) {
        for _ in 0..265 {
            if !self.display.can_continue() {
                break;
            }

            let opcode = Self::fetch(self);
            Self::decode_and_execute(self, opcode);

            self.display.update();
        }
    }

    pub fn fetch(&mut self) -> Opcode {
        let nibble_1 = self.ram.0[self.cpu.pc as usize] / 16;
        let nibble_2 = self.ram.0[self.cpu.pc as usize] % 16;
        let nibble_3 = self.ram.0[(self.cpu.pc + 1) as usize] / 16;
        let nibble_4 = self.ram.0[(self.cpu.pc + 1) as usize] % 16;
        let curr_instruction: Opcode = [nibble_1, nibble_2, nibble_3, nibble_4];
        self.cpu.pc += 2;        
        curr_instruction
    }

    pub fn decode_and_execute(&mut self, opcode: Opcode) {
        match opcode {
            [SET, _, _, _] => self.set(opcode),
            CLEAR => self.clear(),
            [ADD, _, _, _] => self.add(opcode),
            [SET_I, _, _, _] => self.set_i(opcode),
            [DRAW, _, _, _] => self.draw(opcode),
            [GOTO, _, _, _] => self.goto(opcode),
            _ => ()
        }
    }

    fn set(&mut self, opcode: Opcode) {
        self.cpu.v_reg[opcode[1] as usize] = opcode[2] * 16 + opcode[3];
    }

    fn add(&mut self, opcode: Opcode) {
        self.cpu.v_reg[opcode[1] as usize] += opcode[2] * 16 + opcode[3];
    }

    fn clear(&mut self) {
        self.display.framebuffer.fill(false);
    }

    fn set_i(&mut self, opcode: Opcode) {
        self.cpu.i = opcode[1] as u16 * 256 + opcode[2] as u16 * 16 + opcode[3] as u16;
    }

    fn draw(&mut self, opcode: Opcode) {
        let mut x = self.cpu.v_reg[opcode[1] as usize] as u16 % self.display.width as u16;
        let mut y = self.cpu.v_reg[opcode[2] as usize] as u16 % self.display.height as u16;
        
        self.cpu.v_reg[0xF] = 0;
        for n in self.cpu.i..self.cpu.i+opcode[3] as u16 {
            let nth_byte = self.ram.0[n as usize];
            
            for p in (0..8).rev() {
                let bit = (nth_byte >> p) & 1;
                if bit == 1 && self.display.framebuffer[(y * self.display.width as u16 + x) as usize] == true {
                    self.display.framebuffer[(y * self.display.width as u16 + x) as usize] = false;
                    self.cpu.v_reg[0xF] = 1;
                } else if bit == 1 && self.display.framebuffer[(y * self.display.width as u16 + x) as usize] == false {
                    self.display.framebuffer[(y * self.display.width as u16 + x) as usize] = true;
                }

                if (y * self.display.width as u16 + x) % self.display.width as u16 == self.display.width as u16 {
                    break;
                }
                x += 1;
            }
            y += 1;

            if (y * self.display.width as u16 + x) == (self.display.width*self.display.height) as u16 {
                break;
            }
        }
    }

    fn goto(&mut self, opcode: Opcode) {
        self.cpu.pc = opcode[1] as u16 * 256 + opcode[2] as u16 * 16 + opcode[3] as u16;
    }
}