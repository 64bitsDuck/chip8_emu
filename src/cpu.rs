use std::ops::{Index, IndexMut};

pub struct Cpu {
    pc: u16,
    i: u16,
    d_timer: u8,
    s_timer: u8,
    pub v_reg: VariableRegisters,
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