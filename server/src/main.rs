use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, get, web};
use actix_ws::{Message, Session};
use futures_util::{StreamExt, lock::Mutex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientMsg {
    pub paddle_x: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerMsg {
    pub p1_x: f32,
    pub p2_x: f32,
    pub ball_x: f32,
    pub ball_y: f32,
}

use shared::coordinates::Coordinate;
use shared::state::gamestate::GameState;

#[derive(Clone)]
struct Room {
    player1: Arc<Mutex<Option<Session>>>,
    player2: Arc<Mutex<Option<Session>>>,
    state: Arc<Mutex<GameState>>,
}

impl Room {
    fn new(_id: u16) -> Self {
        Self {
            player1: Arc::new(Mutex::new(None)),
            player2: Arc::new(Mutex::new(None)),
            state: Arc::new(Mutex::new(GameState::new(0))),
        }
    }
}

#[derive(Default, Clone)]
struct Lobby {
    rooms: HashMap<u16, Arc<Room>>,
    next_room: u16,
}

#[get("/ws")]
async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
    lobby: web::Data<Arc<Mutex<Lobby>>>,
) -> Result<HttpResponse, Error> {
    let (res, mut session, mut msg_stream) = actix_ws::handle(&req, stream)?;

    let mut lobby = lobby.lock().await;

    let mut chosen_room = None;
    let mut is_player1 = true;

    for (_, room) in lobby.rooms.iter() {
        let p1 = room.player1.lock().await;
        let p2 = room.player2.lock().await;

        if p1.is_some() && p2.is_none() {
            chosen_room = Some(room.clone());
            is_player1 = false;
            break;
        } else if p1.is_none() && p2.is_some() {
            chosen_room = Some(room.clone());
            is_player1 = true;
            break;
        }
    }

    let room = if let Some(r) = chosen_room {
        r
    } else {
        let new_room = Arc::new(Room::new(lobby.next_room));
        let next = lobby.next_room;
        lobby.rooms.insert(next, new_room.clone());
        lobby.next_room += 1;
        new_room
    };

    drop(lobby);

    let player_num;
    if is_player1 {
        let mut p1 = room.player1.lock().await;
        *p1 = Some(session.clone());
        player_num = 1;
        let _ = session.text("1").await;
        println!("P1 Connected");
    } else {
        let mut p2 = room.player2.lock().await;
        *p2 = Some(session.clone());
        player_num = 2;
        let _ = session.text("2").await;
        {
            let mut gs = room.state.lock().await;
            gs.status = shared::state::gamestate::Status::Running;
        }
        println!("P2 Connected");
    }

    let state = room.state.clone();
    let r_p1 = room.player1.clone();
    let r_p2 = room.player2.clone();

    actix_rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            if let Message::Text(text) = msg {
                if let Ok(client_msg) = serde_json::from_str::<ClientMsg>(&text) {
                    let mut gs = state.lock().await;
                    let p_idx = if is_player1 { 0 } else { 1 };
                    let (_, y) = gs.objects.paddles[p_idx].position.get_cartesian();
                    gs.objects.paddles[p_idx].position =
                        Coordinate::from_cartesian(client_msg.paddle_x, y);
                }
            }
        }
        println!("P{} Disconnected", player_num);
        if is_player1 {
            let mut p1 = r_p1.lock().await;
            *p1 = None;
        } else {
            let mut p2 = r_p2.lock().await;
            *p2 = None;
        }

        let mut gs = state.lock().await;
        gs.reset();
        gs.status = shared::state::gamestate::Status::Paused;
    });

    Ok(res)
}

async fn broadcast_loop(lobby: web::Data<Arc<Mutex<Lobby>>>) {
    let mut interval = actix_rt::time::interval(Duration::from_millis(16));
    loop {
        interval.tick().await;

        let rooms: Vec<Arc<Room>> = {
            let lobby_lock = lobby.lock().await;
            lobby_lock.rooms.values().cloned().collect()
        };

        for room in rooms {
            let (p1_x, p2_x, ball_x, ball_y) = {
                let gs = room.state.lock().await;
                let (x1, _) = gs.objects.paddles[0].position.get_cartesian();
                let (x2, _) = gs.objects.paddles[1].position.get_cartesian();
                let (bx, by) = gs.objects.ball.position.get_cartesian();
                (x1, x2, bx, by)
            };

            let msg = ServerMsg {
                p1_x,
                p2_x,
                ball_x,
                ball_y,
            };
            if let Ok(json) = serde_json::to_string(&msg) {
                let mut p1 = room.player1.lock().await;
                if let Some(session) = p1.as_mut() {
                    let _ = session.text(json.clone()).await;
                }

                let mut p2 = room.player2.lock().await;
                if let Some(session) = p2.as_mut() {
                    let _ = session.text(json).await;
                }
            }
        }
    }
}

async fn physics_loop(lobby: web::Data<Arc<Mutex<Lobby>>>) {
    let mut interval = actix_rt::time::interval(Duration::from_millis(16)); // ~60fps
    let mut last_update = std::time::Instant::now();
    loop {
        interval.tick().await;
        let now = std::time::Instant::now();
        let delta = now.duration_since(last_update).as_millis();
        last_update = now;

        let rooms: Vec<Arc<Room>> = {
            let lobby_lock = lobby.lock().await;
            lobby_lock.rooms.values().cloned().collect()
        };

        for room in rooms {
            let mut gs = room.state.lock().await;
            gs.update(delta);
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let lobby = web::Data::new(Arc::new(Mutex::new(Lobby {
        ..Default::default()
    })));

    let lobby_clone1 = lobby.clone();
    actix_rt::spawn(async move {
        broadcast_loop(lobby_clone1).await;
    });

    let lobby_clone2 = lobby.clone();
    actix_rt::spawn(async move {
        physics_loop(lobby_clone2).await;
    });

    HttpServer::new(move || {
        App::new()
            .service(websocket_handler)
            .app_data(lobby.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
