const HEIGHT: usize = 32;
const WIDTH: usize = 64;

pub struct Display {
    pub height: u8,
    pub width: u8,
    pub framebuffer: Vec<bool>,
}