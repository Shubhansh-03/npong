#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameInput {
    pub p1_left_pressed: bool,
    pub p1_right_pressed: bool,
    pub p2_left_pressed: bool,
    pub p2_right_pressed: bool,
    pub pause_toggled: bool,
    pub quit_game: bool,
}
