use shared::gamestate::GameState;

pub struct ClientState {
    pub game: GameState,
    pub player_id: u8,
}

impl ClientState {}

impl Default for ClientState {
    fn default() -> Self {
        ClientState {
            game: GameState::new(),
            player_id: 1,
        }
    }
}
