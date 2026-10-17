use sdl2::event::Event;
use sdl2::keyboard::Scancode;

pub struct KeyPad {
    
}

impl KeyPad {
    pub fn new() -> KeyPad {
        KeyPad {  }
    }
}
/*
pub fn keyboard() {
    for event in event_pump.poll_iter()  {
        match event {
            Event::KeyDown { scancode: Some(Scancode::Q), .. } => {
                // Q was pressed
            }
            _ => {}
        }
    }
}
    */