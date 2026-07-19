#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameInput {
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub pause_toggled: bool,
    pub quit_game: bool,
}
