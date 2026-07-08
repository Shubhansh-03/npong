use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, get, web};
use actix_ws::{Message, Session};
use futures_util::{StreamExt, lock::Mutex};
use serde::{Deserialize, Serialize};
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

#[derive(Default, Clone)]
struct Room {
    player1: Arc<Mutex<Option<Session>>>,
    player2: Arc<Mutex<Option<Session>>>,
    state: Arc<Mutex<GameState>>,
}

#[get("/ws")]
async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<Room>,
) -> Result<HttpResponse, Error> {
    let mut p1 = data.player1.lock().await;
    if p1.is_none() {
        let (res, mut session, mut msg_stream) = actix_ws::handle(&req, stream)?;
        println!("P1 Connected");
        session.text("1").await.unwrap();
        *p1 = Some(session);

        let state = data.state.clone();
        actix_rt::spawn(async move {
            while let Some(Ok(msg)) = msg_stream.next().await {
                if let Message::Text(text) = msg {
                    if let Ok(client_msg) = serde_json::from_str::<ClientMsg>(&text) {
                        let mut gs = state.lock().await;
                        let (_, y) = gs.objects.paddles[0].position.get_cartesian();
                        gs.objects.paddles[0].position =
                            Coordinate::from_cartesian(client_msg.paddle_x, y);
                    }
                }
            }
            println!("P1 Disconnected");
        });

        Ok(res)
    } else {
        let mut p2 = data.player2.lock().await;
        if p2.is_none() {
            let (res, mut session, mut msg_stream) = actix_ws::handle(&req, stream)?;
            println!("P2 Connected");
            session.text("2").await.unwrap();
            *p2 = Some(session);

            let state = data.state.clone();
            {
                let mut gs = state.lock().await;
                gs.status = shared::state::gamestate::Status::Running;
            }

            actix_rt::spawn(async move {
                while let Some(Ok(msg)) = msg_stream.next().await {
                    if let Message::Text(text) = msg {
                        if let Ok(client_msg) = serde_json::from_str::<ClientMsg>(&text) {
                            let mut gs = state.lock().await;
                            let (_, y) = gs.objects.paddles[1].position.get_cartesian();
                            gs.objects.paddles[1].position =
                                Coordinate::from_cartesian(client_msg.paddle_x, y);
                        }
                    }
                }
                println!("P2 Disconnected");
            });

            Ok(res)
        } else {
            Ok(HttpResponse::Forbidden().body("2 players already connected"))
        }
    }
}

async fn broadcast_loop(room: web::Data<Room>) {
    let mut interval = actix_rt::time::interval(Duration::from_millis(16));
    loop {
        interval.tick().await;

        let (p1_x, p2_x, ball_x, ball_y) = {
            let gs = room.state.lock().await;
            let (x1, _) = gs.objects.paddles[0].position.get_cartesian();
            let (x2, _) = gs.objects.paddles[1].position.get_cartesian();
            let (bx, by) = gs.objects.ball.position.get_cartesian();
            (x1, x2, bx, by)
        };

        let msg = ServerMsg { p1_x, p2_x, ball_x, ball_y };
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let room = web::Data::new(Room {
        player1: Arc::new(Mutex::new(None)),
        player2: Arc::new(Mutex::new(None)),
        state: Arc::new(Mutex::new(GameState::new(0))),
    });

    let room_clone = room.clone();
    actix_rt::spawn(async move {
        broadcast_loop(room_clone).await;
    });
    
    let physics_room = room.clone();
    actix_rt::spawn(async move {
        let mut interval = actix_rt::time::interval(Duration::from_millis(16)); // ~60fps
        let mut last_update = std::time::Instant::now();
        loop {
            interval.tick().await;
            let now = std::time::Instant::now();
            let delta = now.duration_since(last_update).as_millis();
            last_update = now;
            
            let mut gs = physics_room.state.lock().await;
            gs.update(delta);
        }
    });

    HttpServer::new(move || App::new().service(websocket_handler).app_data(room.clone()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
