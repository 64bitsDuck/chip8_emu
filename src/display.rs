const HEIGHT: usize = 32;
const WIDTH: usize = 64;

pub struct Display {
    height: u8,
    width: u8,
    framebuffer: Vec<bool>,
}