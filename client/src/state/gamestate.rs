use crate::object::{Objects, ball::*, paddle::*, wall::*};
// use super::systems::collision::*;
use winit::keyboard::KeyCode;

#[derive(Default)]
pub enum Status {
    #[default]
    Paused,
    Running,
    Reset,
    Exit,
}

#[derive(Default)]
pub struct GameState {
    pub status: Status,
    pub player_id: u8,
    pub objects: Objects,
}

impl GameState {
    pub fn new(id: u8) -> Self {
        GameState {
            status: Status::Paused,
            player_id: id,
            objects: Objects {
                paddles: Paddle::new(2),
                ball: Ball::new(),
                walls: Wall::new(2),
            },
        }
    }

    pub fn update(&mut self, delta: u128) {
        // self.objects.ball.update(delta);
        // self.collision(delta);
    }

    pub fn handle_input(&mut self, input: &crate::systems::input::Input, delta: u128) {
        if let Status::Running = self.status {
            let mut local_movement = false;
            let local_idx = (self.player_id - 1) as usize;

            // Invert movement according to which paddle it is
            if local_idx == 0 {
                if input.pressed.contains(&KeyCode::KeyA)
                    || input.pressed.contains(&KeyCode::ArrowLeft)
                {
                    self.objects.paddles[local_idx].left_shift(delta);
                    local_movement = true;
                }
                if input.pressed.contains(&KeyCode::KeyD)
                    || input.pressed.contains(&KeyCode::ArrowRight)
                {
                    self.objects.paddles[local_idx].right_shift(delta);
                    local_movement = true;
                }
            } else {
                if input.pressed.contains(&KeyCode::KeyA)
                    || input.pressed.contains(&KeyCode::ArrowLeft)
                {
                    self.objects.paddles[local_idx].right_shift(delta);
                    local_movement = true;
                }
                if input.pressed.contains(&KeyCode::KeyD)
                    || input.pressed.contains(&KeyCode::ArrowRight)
                {
                    self.objects.paddles[local_idx].left_shift(delta);
                    local_movement = true;
                }
            }

            if !local_movement {
                self.objects.paddles[local_idx].acceleration = 0.0;
            }

            if input.toggled.contains(&KeyCode::Space) {
                self.status = Status::Paused;
            } else {
                self.status = Status::Running;
            }
        }
    }

    // Function to handle inputs even when game is paused
    pub fn check_paused(&mut self, input: &crate::systems::input::Input) {
        if input.toggled.contains(&KeyCode::Space) {
            self.status = Status::Paused;
        } else {
            // FIXME: Instead of Running it should be what the state was before setting it to Paused
            self.status = Status::Running;
        }
    }

    pub fn reset(&mut self) {
        self.objects.paddles = Paddle::new(2);
        self.objects.ball = Ball::new();
        self.objects.walls = Wall::new(2);
        self.status = Status::Reset;
    }

    pub fn collision(&mut self, delta: u128) {
        // CollisionSystem::collision(self, delta);
    }
}
