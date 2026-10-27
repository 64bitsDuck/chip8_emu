use std::fs;

const FONT_START: usize = 0x050;
const ROM_START: usize = 0x200;

const FONT: [u8; 80] = [0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                        0x20, 0x60, 0x20, 0x20, 0x70, // 1
                        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
                        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
                        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                        0xF0, 0x80, 0xF0, 0x80, 0x80];// F
const CH_SZ: usize = 5;
const FONT_END: usize = FONT_START + FONT.len();

#[derive(Clone, Copy)] 
pub struct Ram(pub [u8; 4096]);

impl Ram {
    pub fn new() -> Self {
        let mut ram = Ram([0; 4096]);

        ram.0[FONT_START..FONT_END].copy_from_slice(&FONT);

        ram
    }

    pub fn load_rom(&mut self, path: String) {    
        let rom = fs::read(path).expect("Should've been able to open the file");
        self.0[ROM_START..ROM_START+rom.len()].copy_from_slice(&rom);
    }

    /// It's needed for the character to be less than 16
    pub fn get_chr_addr(&self, chr: u8) -> usize {
        FONT_START + CH_SZ * chr as usize
    } 
}