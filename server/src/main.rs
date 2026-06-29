use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, get, web};
use actix_ws::{Message, Session};
use futures_util::{StreamExt, lock::Mutex};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientMsg {
    pub paddle_x: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerMsg {
    pub p1_x: f32,
    pub p2_x: f32,
}

#[derive(Default)]
struct ServerState {
    p1_x: AtomicU32,
    p2_x: AtomicU32,
}

#[derive(Default, Clone)]
struct Room {
    player1: Arc<Mutex<Option<Session>>>,
    player2: Arc<Mutex<Option<Session>>>,
    state: Arc<ServerState>,
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
                        state.p1_x.store(client_msg.paddle_x.to_bits(), Ordering::Relaxed);
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
            actix_rt::spawn(async move {
                while let Some(Ok(msg)) = msg_stream.next().await {
                    if let Message::Text(text) = msg {
                        if let Ok(client_msg) = serde_json::from_str::<ClientMsg>(&text) {
                            state.p2_x.store(client_msg.paddle_x.to_bits(), Ordering::Relaxed);
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
    let mut interval = actix_rt::time::interval(Duration::from_millis(16)); // ~60fps
    loop {
        interval.tick().await;
        
        let p1_x = f32::from_bits(room.state.p1_x.load(Ordering::Relaxed));
        let p2_x = f32::from_bits(room.state.p2_x.load(Ordering::Relaxed));
        
        let msg = ServerMsg { p1_x, p2_x };
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
    let room = web::Data::new(Room::default());

    // Initialize state properly
    // Player 1 paddle default X: (1200 - 1200/15)/2 = 560
    // Player 2 paddle default X: 560
    room.state.p1_x.store(560.0_f32.to_bits(), Ordering::Relaxed);
    room.state.p2_x.store(560.0_f32.to_bits(), Ordering::Relaxed);

    let room_clone = room.clone();
    actix_rt::spawn(async move {
        broadcast_loop(room_clone).await;
    });

    HttpServer::new(move || App::new().service(websocket_handler).app_data(room.clone()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
