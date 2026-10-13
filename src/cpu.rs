use std::ops::{Index, IndexMut};

pub struct Cpu {
    pub pc: u16,
    pub i: u16,
    pub d_timer: u8,
    pub s_timer: u8,
    pub v_reg: VariableRegisters,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu { pc: 0x200, i: 0, d_timer: 0, s_timer: 0, v_reg: VariableRegisters { 0: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }}
    }
}

pub struct VariableRegisters([u8; 16]);

impl Index<usize> for VariableRegisters {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for VariableRegisters {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}