use std::{
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

use crate::{net::NetHandle, state::gamestate::GameState, systems::input::Input};

pub struct Gameloop {
    pub ticks: Duration,
    pub last_update: Instant,
}

impl Gameloop {
    pub async fn game_loop(
        &mut self,
        state: Arc<RwLock<GameState>>,
        inputs: Arc<RwLock<Input>>,
        handle: NetHandle,
    ) {
    }
}
