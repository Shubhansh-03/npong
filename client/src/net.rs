use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use actix_codec::Framed;
use awc::{
    BoxedSocket, Client,
    ws::{Codec, Frame, Message},
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientMsg {
    pub paddle_x: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerMsg {
    pub p1_x: f32,
    pub p2_x: f32,
    pub ball_x: f32,
    pub ball_y: f32,
}

#[derive(Clone)]
pub struct NetHandle {
    outbound: Arc<Mutex<VecDeque<ClientMsg>>>,
    inbound: Arc<Mutex<VecDeque<ServerMsg>>>,
}

impl NetHandle {
    pub fn new() -> Self {
        NetHandle {
            outbound: Arc::new(Mutex::new(VecDeque::new())),
            inbound: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn send(&self, msg: ClientMsg) {
        self.outbound.lock().unwrap().push_back(msg);
    }

    pub fn try_recv(&self) -> Option<ServerMsg> {
        self.inbound.lock().unwrap().pop_front()
    }
}

pub async fn connect() -> Result<(NetHandle, u8), String> {
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
    
    let handle = NetHandle::new();
    let bg_handle = handle.clone();

    actix_rt::spawn(async move {
        net_task(ws, bg_handle).await;
    });

    Ok((handle, player_id))
}

async fn net_task(mut ws: Framed<BoxedSocket, Codec>, handle: NetHandle) {
    let mut interval = tokio::time::interval(Duration::from_millis(4));

    loop {
        tokio::select! {
            _ = interval.tick() => {
                // Drain outbound queue
                let mut q = handle.outbound.lock().unwrap();
                while let Some(client_msg) = q.pop_front() {
                    if let Ok(json) = serde_json::to_string(&client_msg) {
                        
                        if ws.send(Message::Text(json.into())).await.is_err() {
                            println!("Send error, closing WS");
                            return;
                        }
                    }
                }
            }
            frame_opt = ws.next() => {
                match frame_opt {
                    Some(Ok(Frame::Text(raw))) => {
                        if let Ok(text) = std::str::from_utf8(&raw) {
                            
                            if let Ok(server_msg) = serde_json::from_str::<ServerMsg>(text) {
                                handle.inbound.lock().unwrap().push_back(server_msg);
                            }
                        }
                    }
                    Some(Ok(Frame::Ping(payload))) => {
                        let _ = ws.send(Message::Pong(payload)).await;
                    }
                    Some(Ok(Frame::Close(_))) => {
                        println!("Server closed connection");
                        return;
                    }
                    None => {
                        println!("Stream ended");
                        return;
                    }
                    Some(Err(e)) => {
                        println!("Recv error: {}", e);
                        return;
                    }
                    _ => {}
                }
            }
        }
    }
}
