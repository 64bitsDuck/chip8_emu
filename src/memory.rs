use std::io;

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

#[derive(Clone)] 
pub struct Memory {
    pub ram: [u8; 4096],
    pub rom: Rom,
}

impl Memory {
    pub fn new(rom: Rom) -> Self {
        let mut ram = [0u8; 4096];
        ram[FONT_START..FONT_END].copy_from_slice(&FONT);
        ram[ROM_START..ROM_START+rom.size].copy_from_slice(&rom.content);
        
        Memory { ram, rom }
    }

    /// It's needed for the character to be less than 16
    pub fn get_chr_addr(&self, chr: u8) -> usize {
        FONT_START + CH_SZ * chr as usize
    } 
}

#[derive(Clone)]
pub struct Rom {
    pub content: Vec<u8>,
    pub size: usize,
}

impl Rom {
    pub fn from_reader(reader: &mut impl io::Read) -> io::Result<Self> {
        let mut content = Vec::with_capacity(128);
        let size = reader.read_to_end(&mut content)?;
        
        Ok(Self { content, size })
    }
}