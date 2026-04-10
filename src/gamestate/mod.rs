use super::Input;
use super::object::{Objects, ball::*, paddle::*, wall::*};
use super::systems::collision::*;
// use super::{HEIGHT, WIDTH};
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
    pub players: u8,
    pub objects: Objects,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            status: Status::Paused,
            players: 2,
            objects: Objects {
                paddles: Paddle::new(2),
                ball: Ball::new(),
                walls: Wall::new(2),
            },
        }
    }

    pub fn update(&mut self, delta: u128) {
        self.objects.ball.update(delta);
        self.collision(delta);
    }

    pub fn handle_input(&mut self, input: &Input, delta: u128) {
        if let Status::Running = self.status {
            let mut paddle1_movement = false;
            let mut paddle2_movement = false;
            if input.pressed.contains(&KeyCode::KeyA) {
                self.objects.paddles[0].left_shift(delta);
                paddle1_movement = true;
            }
            if input.pressed.contains(&KeyCode::KeyD) {
                self.objects.paddles[0].right_shift(delta);
                paddle1_movement = true;
            }
            if input.pressed.contains(&KeyCode::ArrowLeft) {
                self.objects.paddles[1].left_shift(delta);
                paddle2_movement = true;
            }
            if input.pressed.contains(&KeyCode::ArrowRight) {
                self.objects.paddles[1].right_shift(delta);
                paddle2_movement = true;
            }

            if !paddle1_movement {
                self.objects.paddles[0].acceleration = 0.0;
            }
            if !paddle2_movement {
                self.objects.paddles[1].acceleration = 0.0;
            }

            if input.toggled.contains(&KeyCode::Space) {
                self.status = Status::Paused;
            } else {
                self.status = Status::Running;
            }
        }
    }

    pub fn check_paused(&mut self, input: &Input) {
        if input.toggled.contains(&KeyCode::Space) {
            self.status = Status::Paused;
        } else {
            self.status = Status::Running;
        }
    }

    pub fn reset(&mut self) {
        // players: 2,
        // objects: Objects {
        //     paddles: Paddle::new(2),
        //     ball: Ball::new(),
        //     walls: Wall::new(2),
        // },
        self.objects.paddles = Paddle::new(2);
        self.objects.ball = Ball::new();
        self.objects.walls = Wall::new(2);
        self.status = Status::Reset;
    }

    pub fn collision(&mut self, delta: u128) {
        CollisionSystem::collision(self, delta);
    }
}
