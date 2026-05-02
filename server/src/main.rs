use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, get, rt, web};
use actix_ws::Session;
// use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
struct Room {
    state: u8,
    player_1: Option<Session>,
    player_2: Option<Session>,
}

#[derive(Default)]
struct Lobby {
    current_room: Room,
}

type SharedLobby = Arc<Mutex<Lobby>>;

#[get("/ws")]
async fn echo(
    lobby: web::Data<SharedLobby>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;
    let _stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(20));

    let lobby_clone = Arc::clone(&lobby);

    rt::spawn(async move {
        let mut player_id: Option<u8> = None;

        {
            let mut lobby_lock = lobby_clone.lock().unwrap();
            if lobby_lock.current_room.player_1.is_none() {
                lobby_lock.current_room.player_1 = Some(session.clone());
                player_id = Some(0);
                println!("Player 0 joined");
            } else if lobby_lock.current_room.player_2.is_none() {
                lobby_lock.current_room.player_2 = Some(session.clone());
                player_id = Some(1);
                println!("Player 1 joined, room ready");
            }
        }

        if let Some(id) = player_id {
            session.text(id.to_string()).await.unwrap();
        } else {
            session.text("Room Full").await.unwrap();
            return;
        }

        println!("We're ready to rock and roll baby");
    });

    Ok(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let lobby = web::Data::new(Arc::new(Mutex::new(Lobby::default())));

    println!("Server starting on 127.0.0.1:8080");
    HttpServer::new(move || App::new().app_data(lobby.clone()).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
