use std::collections::HashSet;
use winit::{event::KeyEvent, keyboard::*};

#[derive(Default)]
pub struct Input {
    pub pressed: HashSet<KeyCode>,
    pub toggled: HashSet<KeyCode>,
}

impl Input {
    pub fn get_inputs(&mut self, key: &KeyCode, event: KeyEvent) {
        match key {
            KeyCode::Space => {
                if !event.state.is_pressed() {
                    if self.toggled.contains(key) {
                        self.toggled.remove(&KeyCode::Space);
                    } else {
                        self.toggled.insert(KeyCode::Space);
                    }
                }
            }
            _ => {
                if event.state.is_pressed() {
                    self.pressed.insert(*key);
                } else {
                    self.pressed.remove(key);
                }
            }
        }
    }
}
