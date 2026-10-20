use core::panic;

use minifb::{Key, Scale, Window, WindowOptions};

pub struct Display {
    pub height: usize,
    pub width: usize,
    pub framebuffer: Vec<bool>,
    pub window: Window,
}

impl Display {
    pub fn new(height: usize, width: usize) -> Display {
        if height * width > 100000 {
            panic!()
        }

        let opts = WindowOptions { 
            scale: Scale::X16,
            ..Default::default()
        };
        let mut window = Window::new("Chip8", width, height, opts).unwrap();
        window.set_target_fps(05);

        let framebuffer: Vec<bool> = vec![false; width  * height];
        Display { height, width, framebuffer, window }
    }

    pub fn can_continue(&self) -> bool{
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn update(&mut self) {
        let buffer: Vec<u32> = self.framebuffer
            .iter()
            .map(|&pixel_on| {
                if pixel_on { 0xFFFFFF } else { 0x000000 }
            })
            .collect();
        self.window.update_with_buffer(&buffer, self.width, self.height).unwrap();
    } 
}