#[derive(Default, Clone, Copy, Debug)]
pub struct Coordinate {
    x: f32,
    y: f32,
}

impl Coordinate {
    pub fn from_cartesian(x: f32, y: f32) -> Coordinate {
        Coordinate { x, y }
    }
    
    pub fn get_cartesian(self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn get_polar(self) -> (f32, f32) {
        let rad = (self.x * self.x + self.y * self.y).sqrt();
        let theta = if self.x == 0.0 {
            if self.y == 0.0 {
                0.0
            } else {
                std::f32::consts::FRAC_PI_2
            }
        } else {
            (self.y / self.x).atan()
        };
        (rad, theta)
    }
}
