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

    // pub fn update(&mut self, delta: u128) {
    //     self.objects.ball.update(delta);
    //     self.collision(delta);
    // }

    // pub fn handle_input(&mut self, input: &Input, delta: u128) {
    //     if let Status::Running = self.status {
    //         let mut paddle1_movement = false;
    //         let mut paddle2_movement = false;
    //         if input.pressed.contains(&KeyCode::KeyA) {
    //             self.objects.paddles[0].left_shift(delta);
    //             paddle1_movement = true;
    //         }
    //         if input.pressed.contains(&KeyCode::KeyD) {
    //             self.objects.paddles[0].right_shift(delta);
    //             paddle1_movement = true;
    //         }
    //         if input.pressed.contains(&KeyCode::ArrowLeft) {
    //             self.objects.paddles[1].left_shift(delta);
    //             paddle2_movement = true;
    //         }
    //         if input.pressed.contains(&KeyCode::ArrowRight) {
    //             self.objects.paddles[1].right_shift(delta);
    //             paddle2_movement = true;
    //         }
    //
    //         if !paddle1_movement {
    //             self.objects.paddles[0].acceleration = 0.0;
    //         }
    //         if !paddle2_movement {
    //             self.objects.paddles[1].acceleration = 0.0;
    //         }
    //
    //         if input.toggled.contains(&KeyCode::Space) {
    //             self.status = Status::Paused;
    //         } else {
    //             self.status = Status::Running;
    //         }
    //     }
    // }
    //
    // // Function to handle inputs even when game is paused
    // pub fn check_paused(&mut self, input: &Input) {
    //     if input.toggled.contains(&KeyCode::Space) {
    //         self.status = Status::Paused;
    //     } else {
    //         // FIXME: Instead of Running it should be what the state was before setting it to Paused
    //         self.status = Status::Running;
    //     }
    // }

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
