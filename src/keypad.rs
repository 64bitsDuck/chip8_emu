use minifb::{Key, Window};

pub struct KeyPad {
    last_state: [bool; 16]
}

impl KeyPad {
    pub fn new(window: Window) -> KeyPad {
        KeyPad { last_state:  [false; 16] }
    }
}
