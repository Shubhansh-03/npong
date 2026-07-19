use std::collections::HashSet;
use winit::{event::KeyEvent, keyboard::*};

#[derive(Default)]
pub struct Input {
    pub pressed: HashSet<KeyCode>,
    pub toggled: HashSet<KeyCode>,
}

impl Input {
    pub fn get_inputs(&mut self, key: &KeyCode, event: KeyEvent) {
        if !event.state.is_pressed() {
            if self.toggled.contains(key) {
                self.toggled.remove(key);
            } else {
                self.toggled.insert(*key);
            }
        }
        if event.state.is_pressed() {
            self.pressed.insert(*key);
        } else {
            self.pressed.remove(key);
        }
    }

    pub fn to_game_input(&self) -> shared::input::GameInput {
        shared::input::GameInput {
            left_pressed: self.pressed.contains(&KeyCode::KeyA)
                || self.pressed.contains(&KeyCode::ArrowLeft),
            right_pressed: self.pressed.contains(&KeyCode::KeyD)
                || self.pressed.contains(&KeyCode::ArrowRight),
            pause_toggled: self.toggled.contains(&KeyCode::Space),
            quit_game: self.toggled.contains(&KeyCode::KeyQ),
        }
    }
}
