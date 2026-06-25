use std::sync::Arc;

use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, get, http::StatusCode, web};
use actix_ws::{CloseReason, Session};
use futures_util::lock::Mutex;

#[derive(Default, Clone)]
struct Room {
    player1: Arc<Mutex<Option<Session>>>,
    player2: Arc<Mutex<Option<Session>>>,
}

#[get("/ws")]
async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<Room>,
) -> Result<HttpResponse, Error> {
    let mut p1 = data.player1.lock().await;
    if p1.is_none() {
        let (res, session, mut _msg_stream) = actix_ws::handle(&req, stream)?;
        println!("P1 Connected");
        *p1 = Some(session);
        Ok(res)
    } else {
        let mut p2 = data.player2.lock().await;
        if p2.is_none() {
            let (res, session, mut _msg_stream) = actix_ws::handle(&req, stream)?;
            println!("P2 Connected");
            *p2 = Some(session);
            Ok(res)
        } else {
            Ok(HttpResponse::Forbidden().body("2 players already connected"))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let room = web::Data::new(Room::default());

    HttpServer::new(move || App::new().service(websocket_handler).app_data(room.clone()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
