use super::Input;
use super::object::{Objects, ball::*, paddle::*, wall::*};
use super::systems::collision::*;
use super::{HEIGHT, WIDTH};
use winit::keyboard::KeyCode;

#[derive(Default)]
pub enum Status {
    #[default]
    Paused,
    Running,
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
                ball: Ball {
                    radius: 15,
                    x: (WIDTH / 2) as i32,
                    y: (HEIGHT / 2) as i32,
                    vx: 0.4,
                    vy: 0.30,
                },
                walls: [
                    Wall {
                        x: 0,
                        y: 0,
                        height: HEIGHT,
                        width: 10,
                    },
                    Wall {
                        x: (WIDTH - 10) as i32,
                        y: 0,
                        height: HEIGHT,
                        width: 10,
                    },
                    Wall {
                        x: 0,
                        y: 0,
                        height: 10,
                        width: WIDTH,
                    },
                    Wall {
                        x: 0,
                        y: (HEIGHT - 10) as i32,
                        height: 10,
                        width: WIDTH,
                    },
                ],
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
        }
        if input.toggled.contains(&KeyCode::Space) {
            self.status = Status::Paused;
        } else {
            self.status = Status::Running;
        }
    }

    pub fn collision(&mut self, delta: u128) {
        CollisionSystem::collision(self, delta);
    }
}
