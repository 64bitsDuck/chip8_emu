use minifb::{Key::self, Window};

pub type KeyboardState = [bool; 16];

pub struct KeyPad {
    mapping: KeyboardMapping,
    last_state: KeyboardState,
}

impl KeyPad {
    pub fn new(mapping: KeyboardMapping) -> Self {
        Self { last_state:  [false; 16], mapping }
    }

    pub fn update(&mut self, window: &Window) {
        for (i, key) in self.mapping.map.iter().enumerate() {
            self.last_state[i] = window.is_key_down(*key);
        }
    }

    pub fn is_pressed(&self, key: u8) -> bool {
        self.last_state[key as usize]
    }
}

pub struct KeyboardMapping {
    map: [Key; 16],
}

impl Default for KeyboardMapping {
    fn default() -> Self {
        Self { map: [Key::X, Key::Key1, Key::Key2, Key::Key3, Key::A, Key::Z, Key::E, Key::Q, Key::S, Key::D, Key::W, Key::C, Key::Key4, Key::R, Key::F, Key::V] }
    }
}
