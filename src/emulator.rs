use crate::{cpu::Cpu, display::Display, keypad::KeyPad, memory::Ram};

#[derive(PartialEq)]
pub struct Opcode {
    n1: u8,
    n2: u8,
    n3: u8,
    n4: u8,
}

impl Opcode {
    pub const fn new(n1: u8, n2: u8, n3: u8, n4: u8) -> Self {
        Opcode { n1, n2, n3, n4 }
    }
}

const CLEAR: Opcode = Opcode::new(0x0, 0x0, 0xE, 0x0);
const RET: Opcode = Opcode::new(0x0, 0x0, 0xE, 0xE);
const GOTO: u8 = 1;
const CALL: u8 = 2;
const SKIP_IF_EQUALS: u8 = 3;
const SKIP_IF_NOTEQUALS: u8 = 4;
const SKIP_IF_X_EQUALS_Y: u8 = 5;
const SET: u8 = 6;
const ADD: u8 = 7;
const SKIP_IF_X_NOT_EQUALS_Y: u8 = 9;
const SET_I: u8 = 0xA;
const DRAW: u8 = 0xD;

pub struct Emulator<'a> {
    cpu: Cpu,
    ram: Ram,
    display: &'a mut Display,
    keypad: KeyPad,
    stack: Vec<u16>,
}

impl<'a> Emulator<'a> {
    pub fn new(cpu: Cpu, ram: Ram, display: &mut Display, keypad: KeyPad, stack: Vec<u16>) -> Emulator<'_> {
        Emulator { cpu, ram, display: display, keypad, stack }
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
        let curr_instruction: Opcode = Opcode::new(nibble_1, nibble_2, nibble_3, nibble_4);
        self.cpu.pc += 2;        
        curr_instruction
    }

    pub fn decode_and_execute(&mut self, opcode: Opcode) {
        match opcode {
            Opcode { n1: SET, .. } => self.set(opcode),
            CLEAR => self.clear(),
            Opcode { n1: ADD, .. } => self.add(opcode),
            Opcode { n1: SET_I, .. } => self.set_i(opcode),
            Opcode { n1: DRAW, .. } => self.draw(opcode),
            Opcode { n1: GOTO, .. } => self.goto(opcode),
            Opcode { n1: CALL, .. } => self.call(opcode),
            RET => self.ret(),
            Opcode { n1: SKIP_IF_EQUALS, .. } => self.skip_if_equals(opcode),
            Opcode { n1: SKIP_IF_NOTEQUALS, .. } => self.skip_if_not_equals(opcode),
            Opcode { n1: SKIP_IF_X_EQUALS_Y, .. } => self.skip_if_x_equals_y(opcode),
            Opcode { n1: SKIP_IF_X_NOT_EQUALS_Y, .. } => self.skip_if_x_not_equals_y(opcode),
            _ => ()
        }
    }

    fn set(&mut self, opcode: Opcode) {
        self.cpu.v_reg[opcode.n2 as usize] = opcode.n3 * 16 + opcode.n4;
    }

    fn add(&mut self, opcode: Opcode) {
        self.cpu.v_reg[opcode.n2 as usize] += opcode.n3 * 16 + opcode.n4;
    }

    fn clear(&mut self) {
        self.display.framebuffer.fill(false);
    }

    fn set_i(&mut self, opcode: Opcode) {
        self.cpu.i = opcode.n2 as u16 * 256 + opcode.n3 as u16 * 16 + opcode.n4 as u16;
    }

    // TODO: From what I understand there is 0 reason why my x coordinate should be reset for every byte but it works & I want to never touch that opcode again so bad
    fn draw(&mut self, opcode: Opcode) {
        let mut y: u16 = self.cpu.v_reg[opcode.n3 as usize] as u16 % self.display.height as u16;
        
        self.cpu.v_reg[0xF] = 0;
        for n in self.cpu.i..self.cpu.i+opcode.n4 as u16 {
            let nth_byte = self.ram.0[n as usize];
            let mut x = self.cpu.v_reg[opcode.n2 as usize] as u16 % self.display.width as u16;
            
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
        self.cpu.pc = opcode.n2 as u16 * 256 + opcode.n3 as u16 * 16 + opcode.n4 as u16;
    }

    fn call(&mut self, opcode: Opcode) {
        self.stack.push(self.cpu.pc);
        self.cpu.pc = opcode.n2 as u16 * 256 + opcode.n3 as u16 * 16 + opcode.n4 as u16;
    }

    fn ret(&mut self) {
        self.cpu.pc = self.stack.pop().expect("An element in the stack should've been found");
    }

    fn skip_if_equals(&mut self, opcode: Opcode) {
        if self.cpu.v_reg[opcode.n2 as usize] == opcode.n3*16+opcode.n4 {
            self.cpu.pc += 2;
        }
    }

    fn skip_if_not_equals(&mut self, opcode: Opcode) {
        if self.cpu.v_reg[opcode.n2 as usize] != opcode.n3*16+opcode.n4 {
            self.cpu.pc += 2;
        }
    }

    fn skip_if_x_equals_y(&mut self, opcode: Opcode) {
        if self.cpu.v_reg[opcode.n2 as usize] == self.cpu.v_reg[opcode.n3 as usize] {
            self.cpu.pc += 2;
        }
    }

    fn skip_if_x_not_equals_y(&mut self, opcode: Opcode) {
        if self.cpu.v_reg[opcode.n2 as usize] != self.cpu.v_reg[opcode.n3 as usize] {
            self.cpu.pc += 2;
        }
    }

    fn add_to_reg(&mut self, opcode: Opcode) {
        self.cpu.v_reg[opcode.n2 as usize] += self.cpu.v_reg[opcode.n3 as usize];
    }
}