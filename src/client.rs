#![allow(unused_imports)]
use std::net::SocketAddr;
use std::time::Instant;
use std::thread;
use std::time::Duration;

use bincode::{deserialize, serialize};
use serde_derive::{Deserialize, Serialize};

use laminar::{Packet, Socket, SocketEvent};
use macroquad::prelude::*;



mod helper;
use helper::MyRectangle;

const SPEED: f32 = 5.0; 

const SERVER_ADDR: &str = "127.0.0.1:12345";

// fn client_address() -> SocketAddr {
//     CLIENT_ADDR.parse().unwrap()
// }

fn server_address() -> SocketAddr {
    SERVER_ADDR.parse().unwrap()
}

    

#[macroquad::main("Graphics")]
async fn main() {
    // let mut server = Socket::bind(server_address()).unwrap();

    let mut client = Socket::bind("0.0.0.0:0").unwrap();

    // client.send(Packet::unreliable(
    //     server_address(),
    //     serialize("test")
    //     .unwrap(),
    // ));

    // client.send(Packet::unreliable(
    //     server_address(),
    //     serialize("hey")
    //     .unwrap(),
    // ));

    // client.send(Packet::unreliable(
    //     server_address(),
    //     serialize("test")
    //     .unwrap(),
    // ));

    // client.manual_poll(Instant::now());

    let mut rect = MyRectangle {
        x: 50.0,
        y: 50.0,
        w: 100.0,
        h: 100.0,
        clr: (255, 0, 0, 255),
    };

    loop {
        clear_background(BLACK);
        rect.custom_draw_rect();
        draw_text(&[rect.x.to_string(),",".to_string(), rect.y.to_string()].concat() , rect.x , rect.y, 30.0, WHITE);
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
        next_frame().await;

        
        let _ = client.send(Packet::unreliable(
            server_address(),
            serialize(&rect)
            .unwrap(),
        ));

    client.manual_poll(Instant::now());
    thread::sleep(Duration::from_millis(16));

    }

}
