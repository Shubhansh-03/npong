use std::sync::Arc;

use actix_codec::Framed;
use awc::{
    BoxedSocket, Client,
    ws::{Codec, Frame},
};
use futures_util::{StreamExt, lock::Mutex};

pub struct NetHandle {
    ws_connection: Framed<BoxedSocket, Codec>,
    message: String, // outbound: Arc<<Mutex<>>,
}

impl NetHandle {
    pub async fn initialize() -> Result<(NetHandle, u8), String> {
        let client = Client::default();
        let (_res, mut ws) = client
            .ws("ws://127.0.0.1:8080/ws")
            .connect()
            .await
            .map_err(|e| format!("WebSocket connect failed: {}", e))?;

        let mut player_id = 1;
        if let Some(Ok(Frame::Text(raw))) = ws.next().await {
            if let Ok(text) = std::str::from_utf8(&raw) {
                if let Ok(id) = text.parse::<u8>() {
                    player_id = id;
                }
            }
        }

        println!("Connected to the server. Player ID: {}", player_id);
        Ok((
            NetHandle {
                ws_connection: ws,
                message: "".into(),
            },
            player_id,
        ))
    }
}

// // client/src/net/mod.rs
// //
// // The networking layer runs entirely inside a single actix_rt task.
// // The rest of the client talks to it through two queues on NetHandle:
// //
// //   ┌─────────────┐   ServerMsg   ┌──────────┐   WS text   ┌────────┐
// //   │ ClientState │ ◄──────────── │ net task │ ◄────────── │ Server │
// //   │  game loop  │               │          │             │        │
// //   │  render     │ ──────────── ►│          │ ──────────► │        │
// //   └─────────────┘   ClientMsg   └──────────┘   WS text   └────────┘
// //
// // Public API (all cheap to clone):
// //   NetHandle::send(msg)            – queue a ClientMsg to be sent
// //   NetHandle::try_recv() -> Option – poll for the next ServerMsg
//
// use actix_codec::Framed;
// use awc::{
//     BoxedSocket, Client,
//     ws::{Codec, Frame, Message},
// };
// use bytes::Bytes;
// use futures_util::{SinkExt, StreamExt};
// use shared::messages::{ClientMsg, ServerMsg};
// use std::collections::VecDeque;
// use std::sync::{Arc, Mutex, RwLock};
//
// use crate::clientstate::ClientState;
//
// // ── handle ─────────────────────────────────────────────────────────────────
//
// /// Cheap-to-clone handle given to the game loop and renderer.
// #[derive(Clone)]
// pub struct NetHandle {
//     /// Outbound: game loop enqueues messages here; net task drains it.
//     outbound: Arc<Mutex<VecDeque<ClientMsg>>>,
//     /// Inbound: net task pushes decoded ServerMsgs; game loop polls it.
//     inbound: Arc<Mutex<VecDeque<ServerMsg>>>,
// }
//
// impl NetHandle {
//     fn new() -> Self {
//         NetHandle {
//             outbound: Arc::new(Mutex::new(VecDeque::new())),
//             inbound: Arc::new(Mutex::new(VecDeque::new())),
//         }
//     }
//
//     /// Queue a message to be sent to the server on the next net tick.
//     pub fn send(&self, msg: ClientMsg) {
//         self.outbound.lock().unwrap().push_back(msg);
//     }
//
//     /// Non-blocking poll: returns the oldest pending ServerMsg, if any.
//     pub fn try_recv(&self) -> Option<ServerMsg> {
//         self.inbound.lock().unwrap().pop_front()
//     }
//
//     // ── convenience senders ───────────────────────────────────────────────
//
//     /// Send this player's current paddle shift to the server.
//     /// Call this every game tick while the match is running.
//     pub fn send_paddle(&self, shift: i32) {
//         self.send(ClientMsg::PaddleUpdate { shift });
//     }
//
//     /// Gracefully disconnect from the server.
//     pub fn disconnect(&self) {
//         self.send(ClientMsg::Disconnect);
//     }
// }
//
// // ── connection entry-point ─────────────────────────────────────────────────
//
// /// Connect to the server and return:
// ///   - the assigned `player_id`
// ///   - the assigned `room_id`
// ///   - a `NetHandle` for ongoing communication
// ///
// /// Blocks (async) until the server sends `ASSIGNED`, then spawns a
// /// background task that keeps the connection alive.
// pub async fn connect(url: &str) -> Result<(u8, u32, NetHandle), Box<dyn std::error::Error>> {
//     let client = Client::default();
//     let (_res, mut ws) = client
//         .ws(url)
//         .connect()
//         .await
//         .map_err(|e| format!("WebSocket connect failed: {}", e))?;
//
//     // Wait for ASSIGNED before handing control back to the caller.
//     // `ws` is already a live `Framed<BoxedSocket, Codec>` at this point.
//     let (player_id, room_id) = loop {
//         match ws.next().await {
//             Some(Ok(Frame::Text(raw))) => {
//                 let text = std::str::from_utf8(&raw)?;
//                 match ServerMsg::decode(text) {
//                     Some(ServerMsg::Assigned { player_id, room_id }) => {
//                         break (player_id, room_id);
//                     }
//                     Some(ServerMsg::RoomFull) => return Err("Server is full".into()),
//                     // e.g. Waiting – keep draining until ASSIGNED arrives
//                     other => println!("[net] pre-assign msg: {:?}", other),
//                 }
//             }
//             Some(Err(e)) => return Err(format!("WS error during handshake: {}", e).into()),
//             None => return Err("Connection closed before ASSIGNED".into()),
//             _ => {}
//         }
//     };
//
//     println!("[net] assigned player_id={} room_id={}", player_id, room_id);
//
//     let handle = NetHandle::new();
//     let bg_handle = handle.clone();
//
//     // `ws` is the live Framed connection – move it straight into the task.
//     actix_rt::spawn(async move {
//         net_task(ws, bg_handle).await;
//     });
//
//     Ok((player_id, room_id, handle))
// }
//
// // ── background network task ────────────────────────────────────────────────
//
// /// Pumps the live WebSocket connection forever.
// ///
// /// `ws` is the `Framed<BoxedSocket, Codec>` returned by the post-handshake
// /// `.connect().await` call – NOT the pre-handshake `WebsocketsRequest` builder.
// async fn net_task(mut ws: Framed<BoxedSocket, Codec>, handle: NetHandle) {
//     // How long to wait for an inbound frame before looping back to drain
//     // the outbound queue again.
//     let poll_interval = std::time::Duration::from_millis(4);
//
//     loop {
//         // ── drain outbound queue → server ─────────────────────────────────
//         {
//             let mut q = handle.outbound.lock().unwrap();
//             while let Some(client_msg) = q.pop_front() {
//                 let text = client_msg.encode();
//                 if ws.send(Message::Text(text.into())).await.is_err() {
//                     println!("[net] send error – closing");
//                     return;
//                 }
//             }
//         }
//
//         // ── poll for one inbound frame (non-blocking via timeout) ─────────
//         match tokio::time::timeout(poll_interval, ws.next()).await {
//             Ok(Some(Ok(frame))) => {
//                 match frame {
//                     Frame::Text(raw) => {
//                         if let Ok(text) = std::str::from_utf8(&raw) {
//                             if let Some(server_msg) = ServerMsg::decode(text) {
//                                 handle.inbound.lock().unwrap().push_back(server_msg);
//                             } else {
//                                 println!("[net] unknown frame: {}", text);
//                             }
//                         }
//                     }
//                     // Respond to server keep-alive pings.
//                     Frame::Ping(payload) => {
//                         let _ = ws.send(Message::Pong(payload)).await;
//                     }
//                     Frame::Close(_) => {
//                         println!("[net] server closed connection");
//                         return;
//                     }
//                     // Binary / Pong / Continuation – ignore.
//                     _ => {}
//                 }
//             }
//             // Stream ended.
//             Ok(None) => {
//                 println!("[net] stream ended");
//                 return;
//             }
//             // WS error.
//             Ok(Some(Err(e))) => {
//                 println!("[net] recv error: {}", e);
//                 return;
//             }
//             // Timeout – no frame arrived; loop back and drain outbound again.
//             Err(_) => {}
//         }
//     }
// }
//
// // ── dispatcher: call this from the game loop every tick ───────────────────
//
// /// Poll the inbound queue and dispatch each message to the right handler.
// /// Wire this into your game loop alongside `gs.game.update(delta)`.
// pub fn tick_net(handle: &NetHandle, state: &Arc<RwLock<ClientState>>) {
//     while let Some(msg) = handle.try_recv() {
//         match msg {
//             ServerMsg::StateSnapshot { state_str } => on_state_snapshot(&state_str, state),
//             ServerMsg::MatchStart => on_match_start(),
//             ServerMsg::Scored { scorer, scores } => on_scored(scorer, scores),
//             ServerMsg::MatchOver { winner } => on_match_over(winner),
//             ServerMsg::OpponentDisconnected => on_opponent_disconnected(),
//             ServerMsg::Waiting => println!("[net] waiting for opponent…"),
//             // ASSIGNED is handled once in connect(); ignore if repeated.
//             ServerMsg::Assigned { .. } | ServerMsg::RoomFull => {}
//         }
//     }
// }
//
// // ── handlers ──────────────────────────────────────────────────────────────
//
// /// Applies the authoritative server state to the local GameState.
// pub fn on_state_snapshot(state_str: &str, state: &Arc<RwLock<ClientState>>) {
//     state.write().unwrap().game.apply_snapshot(state_str);
// }
//
// /// Called when the server says the match has started.
// /// TODO(you): set any lobby/waiting UI to hidden, show the game, etc.
// pub fn on_match_start() {
//     println!("[net] match started");
// }
//
// /// Called when the server says the opponent disconnected.
// /// TODO(you): show an in-game message, return to lobby screen, etc.
// pub fn on_opponent_disconnected() {
//     println!("[net] opponent disconnected");
// }
//
// /// Called when the server sends a score update.
// pub fn on_scored(scorer: u8, scores: [u8; 2]) {
//     println!("[net] player {} scored – scores {:?}", scorer, scores);
//     // TODO(you): update ClientState scoreboard
// }
//
// /// Called when the match is over.
// pub fn on_match_over(winner: u8) {
//     println!("[net] match over – winner: player {}", winner);
//     // TODO(you): show win/loss screen
// }
