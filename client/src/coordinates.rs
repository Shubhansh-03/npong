#[derive(Default, Clone, Copy)]
pub struct Coordinate {
    rad: u16,
    theta: f32,
}

impl Coordinate {
    pub fn from_cartesian(x: u16, y: u16) -> Coordinate {
        Coordinate {
            rad: ((x as u32 * x as u32) + (y as u32 * y as u32)).isqrt() as u16,
            theta: (y as f32 / x as f32).atan(),
        }
    }
    pub fn get_cartesian(self) -> (u16, u16) {
        let x = ((self.rad as f32) * self.theta.cos()) as u16;
        let y = ((self.rad as f32) * self.theta.sin()) as u16;
        (x, y)
    }

    pub fn get_polar(self) -> (u16, f32) {
        (self.rad, self.theta)
    }
}
