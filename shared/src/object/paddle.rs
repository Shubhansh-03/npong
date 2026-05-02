use crate::{HEIGHT, WIDTH};

#[derive(Default)]
pub struct Paddle {
    pub id: u8,
    pub shift: i32,
    pub height: u32,
    pub width: u32,
    pub acceleration: f32,
}

impl Paddle {
    pub fn new(players: u8) -> Vec<Paddle> {
        if players == 2 {
            vec![
                Paddle {
                    id: 0,
                    shift: 0,
                    height: HEIGHT / 60,
                    width: WIDTH / 15,
                    acceleration: 0.0,
                },
                Paddle {
                    id: 1,
                    shift: 0,
                    height: HEIGHT / 60,
                    width: WIDTH / 15,
                    acceleration: 0.0,
                },
            ]
        } else {
            todo!();
        }
    }

    pub fn global_coordinates(&self, player_id: u8) -> (i32, i32) {
        let x = (WIDTH / 2) as i32 + self.shift;
        let y = {
            if player_id == 1 {
                HEIGHT - self.height - 10
            } else {
                self.height + 10
            }
        };
        (x, y as i32)
    }

    pub fn left_shift(&mut self, delta: u128) {
        if self.acceleration > 0.0 {
            self.acceleration = 0.0;
        }
        self.shift += ((-0.3 + self.acceleration) * delta as f32) as i32;
        self.acceleration -= 0.01;
    }

    pub fn right_shift(&mut self, delta: u128) {
        if self.acceleration < 0.0 {
            self.acceleration = 0.0;
        }
        self.shift += ((0.3 + self.acceleration) * delta as f32) as i32;
        self.acceleration += 0.01;
    }

    pub fn local_coordinates(&self, player_id: u8) -> (i32, i32) {
        let x = (WIDTH / 2) as i32 + self.shift;
        let y = self.y_coordinate();

        if player_id == 0 {
            ((WIDTH as i32) - x, (HEIGHT as i32) - y)
        } else {
            (x, y)
        }
    }

    fn y_coordinate(&self) -> i32 {
        let paddle_thick = HEIGHT / 60;
        let wall_offset = 10;

        let multiplier = if self.id == 0 { -1 } else { 1 };

        (HEIGHT / 2) as i32 + (multiplier * (HEIGHT / 2 - wall_offset - paddle_thick / 2) as i32)
    }

    pub fn draw(&self, viewer_id: u8, frame: &mut [u8]) {
        let (x, y) = self.local_coordinates(viewer_id);
        let (h, w) = (self.height, self.width);
        let half_w = (w / 2) as i32;
        let half_h = (h / 2) as i32;

        let color = if self.id == 0 {
            [100, 255, 100, 255]
        } else {
            [255, 100, 100, 255]
        };

        for dy in -half_h..half_h {
            for dx in -half_w..half_w {
                let px = x + dx;
                let py = y + dy;

                if px < 0 || py < 0 || px >= WIDTH as i32 || py >= HEIGHT as i32 {
                    continue;
                }

                let idx = ((py as u32 * WIDTH + px as u32) * 4) as usize;
                frame[idx..idx + 4].copy_from_slice(&color);
            }
        }
    }
}
