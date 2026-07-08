use std::{
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

use crate::{net::NetHandle, systems::input::Input};
use shared::state::gamestate::{GameState, Status};

pub struct Gameloop {
    pub ticks: Duration,
    pub last_update: Instant,
}

impl Gameloop {
    pub fn game_loop(
        &mut self,
        state: Arc<RwLock<GameState>>,
        inputs: Arc<RwLock<Input>>,
        handle: NetHandle,
    ) {
        loop {
            let now = Instant::now();
            let delta = now.duration_since(self.last_update).as_millis();
            if delta >= self.ticks.as_millis() {
                self.last_update = now;

                let mut gs = state.write().unwrap();
                let inputs_lock = inputs.read().unwrap();
                
                if let Status::Exit = gs.status {
                    break;
                }

                gs.handle_input(&inputs_lock.to_game_input(), delta);
                
                // If the game is running, handle network sync
                if let Status::Running = gs.status {
                    // Send local paddle x to server
                    let local_paddle_idx = (gs.player_id - 1) as usize;
                    let (local_x, _) = gs.objects.paddles[local_paddle_idx].position.get_cartesian();
                    handle.send(crate::net::ClientMsg { paddle_x: local_x });
                    
                    // Receive latest server state and apply to opponent's paddle
                    while let Some(server_msg) = handle.try_recv() {
                        let remote_paddle_idx = if gs.player_id == 1 { 1 } else { 0 };
                        let remote_x = if gs.player_id == 1 { server_msg.p2_x } else { server_msg.p1_x };
                        
                        let (_, y) = gs.objects.paddles[remote_paddle_idx].position.get_cartesian();
                        gs.objects.paddles[remote_paddle_idx].position = shared::coordinates::Coordinate::from_cartesian(remote_x, y);
                        
                        gs.objects.ball.position = shared::coordinates::Coordinate::from_cartesian(server_msg.ball_x, server_msg.ball_y);
                    }
                }
                
                // Unlock so winit can redraw
                drop(gs);
                drop(inputs_lock);
            } else {
                std::thread::sleep(Duration::from_millis(1));
            }
        }
    }
}
