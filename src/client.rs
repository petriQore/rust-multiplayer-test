#![allow(unused_imports)]
use std::collections::HashMap;
use std::net::SocketAddr;
use std::thread;
use std::time::{Duration, Instant,SystemTime, UNIX_EPOCH};

use bincode::{deserialize, serialize};
use macroquad::rand::srand;
use macroquad::rand::RandGenerator;
use serde_derive::{Deserialize, Serialize};

use laminar::{Packet, Socket, SocketEvent};
use macroquad::prelude::*;

mod helper;
use helper::{MyRectangle, NetworkMessage};

const SPEED: f32 = 5.0;

const SERVER_ADDR: &str = "192.168.1.16:12345";

// fn client_address() -> SocketAddr {
//     CLIENT_ADDR.parse().unwrap()
// }

fn server_address() -> SocketAddr {
    SERVER_ADDR.parse().unwrap()
}

#[macroquad::main("Graphics")]
async fn main() {
    // let mut server = Socket::bind(server_address()).unwrap();
    srand(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64);

    let mut players: HashMap<SocketAddr, MyRectangle> = HashMap::new();

    let mut socket = Socket::bind("0.0.0.0:0").unwrap();
    // let port = socket.local_addr().unwrap().port().to_string();

    let receiver = socket.get_event_receiver();

    let _ = socket.send(Packet::reliable_unordered(
        server_address(),
        serialize(&NetworkMessage::Hello).unwrap(),
    ));


    let mut rect = MyRectangle {
        x: rand::gen_range(0, (screen_width()-100.0) as u32) as f32,
        y: rand::gen_range(0, (screen_height()-100.0) as u32) as f32,
        w: 100.0,
        h: 100.0,
        clr: (255, 0, 0, 255),
    };

    loop {
        while let Ok(event) = receiver.try_recv() {
            // println!("Received event: {:?}", event);
            match event {

                SocketEvent::Packet(packet) => {
                    match deserialize::<NetworkMessage>(packet.payload()){
                        Ok(NetworkMessage::Players(players_on_server)) => {
                            players = players_on_server;
                        }

                       _  => ()
                    }
                    
                },

                _ => ()
            }
        }
        clear_background(BLACK);

        if is_key_down(KeyCode::T) {
            rect.y -= SPEED;
        }
        if is_key_down(KeyCode::G) {
            rect.y += SPEED;
        }
        if is_key_down(KeyCode::F) {
            rect.x -= SPEED;
        }
        if is_key_down(KeyCode::H) {
            rect.x += SPEED;
        }

        let _ = socket.send(Packet::unreliable(
            server_address(),
            serialize(&NetworkMessage::Rect(rect)).unwrap(),
        ));

        for (addr, player) in players.iter() {
            player.custom_draw_rect();
            player.custom_draw_text(addr.port().to_string());
        }

        socket.manual_poll(Instant::now());
        thread::sleep(Duration::from_millis(16));

        next_frame().await;

    }
}
