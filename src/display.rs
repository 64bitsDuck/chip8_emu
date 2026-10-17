use minifb::{Key, Window, WindowOptions};

pub struct Display {
    pub height: u8,
    pub width: u8,
    pub framebuffer: Vec<bool>,
}

impl Display {
    pub fn new(height: u8, width: u8) -> Display {
        let framebuffer: Vec<bool> = vec![false; width as usize * height as usize];
        Display { height, width, framebuffer }
    }

    pub fn show(&self) {
        let buffer: Vec<u32> = self.framebuffer
            .iter()
            .map(|&pixel_on| {
                if pixel_on { 0xFFFFFF } else { 0x000000 }
            })
            .collect();

        let mut window = Window::new("Monochrome", self.width as usize, self.height as usize, WindowOptions::default()).unwrap();
        window.set_target_fps(60);

        while window.is_open() && !window.is_key_down(Key::Escape) {
            window.update_with_buffer(&buffer, self.width as usize, self.height as usize).unwrap();
        }
    }
}