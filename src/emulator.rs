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

pub struct Emulator<'a> {
    cpu: Cpu,
    ram: Ram,
    display: &'a mut Display,
    keypad: KeyPad,
}

impl<'a> Emulator<'a> {
    pub fn new(cpu: Cpu, ram: Ram, display: &mut Display, keypad: KeyPad) -> Emulator {
        Emulator { cpu, ram, display: display, keypad }
    }

    pub fn cycle(&mut self) {
        let opcode = Self::fetch(self);
        Self::decode(self, opcode);
        Self::execute();
    }

    pub fn fetch(&mut self) -> Opcode {
        let nibble_1 = self.ram.0[self.cpu.pc as usize] / 10;
        let nibble_2 = self.ram.0[self.cpu.pc as usize] % 10;
        let nibble_3 = self.ram.0[(self.cpu.pc + 1) as usize] / 10;
        let nibble_4 = self.ram.0[(self.cpu.pc + 1) as usize] % 10;
        let curr_instruction: Opcode = [nibble_1, nibble_2, nibble_3, nibble_4];
        self.cpu.pc += 2;
        curr_instruction
    }

    pub fn decode(&mut self, opcode: Opcode) {
        match opcode {
            [SET, n2, n3, n4] => self.cpu.v_reg[n2 as usize] = n3 * 10 + n4, //u8::from_str_radix(&n3.to_string(), 16).unwrap() * 10 + u8::from_str_radix(&n4.to_string(), 16).unwrap(),
            CLEAR => self.display.framebuffer.fill(false),
            [ADD, n2, n3, n4] => self.cpu.v_reg[n2 as usize] += n3 * 10 + n4,
            [0xA, n2, n3, n4] => self.cpu.i = n2 as u16 * 100 + n3 as u16 * 10 + n4 as u16,
            [0xD, n2, n3, n4] => {
                let mut x = self.cpu.v_reg[n2 as usize] % self.display.height;
                let mut y = self.cpu.v_reg[n3 as usize] % self.display.width;
                self.cpu.v_reg[0xF] = 0;
                for i in self.cpu.i..n4 as u16 {
                    let nth_byte = self.ram.0[i as usize];
                    
                    for pixels in (0..8).rev() {
                        let bit = (nth_byte >> i) & 1;
                        if bit == 1 && self.display.framebuffer[(x * self.display.width + y) as usize] == true {
                            self.display.framebuffer[(x * self.display.width + y) as usize] = false;
                            self.cpu.v_reg[0xF] = 1;
                        } else {
                            self.display.framebuffer[(x * self.display.width + y) as usize] = true;
                        }
                        x += 1;
                    }
                    y += 1;
                }
                //let sprite = &self.ram.0[self.cpu.i as usize..n4 as usize];
                //let start = self.cpu.v_reg[n2 as usize] * self.display.width + self.cpu.v_reg[n3 as usize]; 
            },
            _ => ()
        }
    }

    pub fn execute() {

    }
}