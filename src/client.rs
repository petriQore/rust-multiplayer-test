#![allow(unused_imports)]
use std::collections::HashMap;
use std::net::SocketAddr;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use bincode::{deserialize, serialize};
use macroquad::rand::srand;
use macroquad::rand::RandGenerator;
use serde_derive::{Deserialize, Serialize};

use laminar::{Packet, Socket, SocketEvent};
use macroquad::prelude::*;
use macroquad::ui::widgets;
use macroquad::ui::{hash, root_ui, Skin};

mod helper;
use helper::{MyRectangle, NetworkMessage};

const SPEED: f32 = 5.0;

const WINDOW_SIZE: Vec2 = vec2(300.0, 150.0);
enum State {
    Menu,
    Playing,
}

const SERVER_ADDR: &str = "128.85.43.221:12345";

// fn client_address() -> SocketAddr {
//     CLIENT_ADDR.parse().unwrap()
// }

fn server_address(input: &str) -> SocketAddr {
    // input.parse().unwrap()
    match input.parse::<SocketAddr>() {
        Ok(addr) => addr,
        Err(_) => {
            println!("Invalid address, using default: {}", SERVER_ADDR);
            SERVER_ADDR.parse().unwrap()
        }
    }
}

#[macroquad::main("Multiplayer")]
async fn main() {
    
    let mut inputted_server = String::new();
    let mut game_state = State::Menu;
    let mut server_addr = SocketAddr::V4(std::net::SocketAddrV4::new(
        std::net::Ipv4Addr::UNSPECIFIED,
        0,
    ));

    // let mut server = Socket::bind(server_address()).unwrap();
    macroquad::rand::srand(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
    );

    let mut players: HashMap<SocketAddr, MyRectangle> = HashMap::new();

    let mut socket = Socket::bind("0.0.0.0:0").unwrap();
    // let port = socket.local_addr().unwrap().port().to_string();

    let receiver = socket.get_event_receiver();

    let mut rect = MyRectangle {
        x: macroquad::rand::gen_range(0, (screen_width() - 100.0) as u32) as f32,
        y: macroquad::rand::gen_range(0, (screen_height() - 100.0) as u32) as f32,
        w: 100.0,
        h: 100.0,
        clr: (
            macroquad::rand::gen_range(0, 255), 
            macroquad::rand::gen_range(0, 255), 
            macroquad::rand::gen_range(0, 255), 
            255,                     
        ),
    };

    let window_style = root_ui().style_builder().color(LIGHTGRAY).build();

    let button_style = root_ui()
        .style_builder()
        .text_color(BLACK)
        .font_size(24)
        .color_hovered(BLACK)
        .text_color_hovered(WHITE)
        .build();

    let label_style = root_ui()
        .style_builder()
        .text_color(BLACK)
        .font_size(20)
        .build();
    let editbox_style = root_ui().style_builder().font_size(20).build();

    let ui_skin = Skin {
        window_style,
        button_style,
        label_style,
        editbox_style,
        ..root_ui().default_skin()
    };
    root_ui().push_skin(&ui_skin);

    loop {
        clear_background(BLACK);
        match game_state {
            State::Menu => {
                root_ui().window(
                    hash!(),
                    vec2(
                        screen_width() / 2.0 - WINDOW_SIZE.x / 2.0,
                        screen_height() / 2.0 - WINDOW_SIZE.y / 2.0,
                    ),
                    WINDOW_SIZE,
                    |ui| {
                        ui.label(vec2(0.0, 34.0), "use 128.85.43.221:12345 for online");
                        ui.label(vec2(0.0, 59.0), "or use your local server ip with");
                        ui.label(vec2(0.0, 79.0), "port 12345");

                        widgets::InputText::new(hash!())
                        .size(vec2(400.0, 20.0))
                        .label("Server")
                        .ui(ui, &mut inputted_server);
                    

                        if ui.button(vec2(115.0, 110.0), "Connect") {
                            server_addr = server_address(&inputted_server);

                            let _ = socket.send(Packet::reliable_unordered(
                                server_addr,
                                serialize(&NetworkMessage::Hello).unwrap(),
                            ));

                            game_state = State::Playing;
                        }
                    },
                );
            }

            State::Playing => {
                while let Ok(event) = receiver.try_recv() {
                    // println!("Received event: {:?}", event);
                    match event {
                        SocketEvent::Packet(packet) => {
                            match deserialize::<NetworkMessage>(packet.payload()) {
                                Ok(NetworkMessage::Players(players_on_server)) => {
                                    players = players_on_server;
                                }

                                _ => (),
                            }
                        }

                        _ => (),
                    }
                }

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
                    server_addr,
                    serialize(&NetworkMessage::Rect(rect)).unwrap(),
                ));

                for (addr, player) in players.iter() {
                    player.custom_draw_rect();
                    player.custom_draw_text(addr.port().to_string());
                }

                socket.manual_poll(Instant::now());
                thread::sleep(Duration::from_millis(16));
            }
        }
        next_frame().await;
    }
}
