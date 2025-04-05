#![allow(unused_imports)]
use std::collections::HashMap;
use std::net::SocketAddr;
use std::thread;
use std::time::Duration;

use bincode::{deserialize, serialize};
// use serde_derive::{Deserialize, Serialize};

use laminar::{Packet, Socket, SocketEvent, Config};

mod helper;
use helper::*;
use macroquad::conf;

const SERVER_ADDR: &str = "0.0.0.0:12345";

fn server_address() -> SocketAddr {
    SERVER_ADDR.parse().unwrap()
}


#[allow(unused_must_use)]
pub fn main() {

    let mut players: HashMap<SocketAddr, MyRectangle> = HashMap::new();

    let mut config = Config::default();
    config.idle_connection_timeout = Duration::from_secs(3);

    let mut socket = Socket::bind_with_config(server_address(), config).unwrap();

    let receiver = socket.get_event_receiver();
    
    
    println!("Server started on {}", SERVER_ADDR);
    
    loop {
        
        socket.manual_poll(std::time::Instant::now());

        while let Ok(event) = receiver.try_recv() {
            // println!("Received event: {:?}", event);
            match event {

                SocketEvent::Timeout(addr) => {
                    players.retain(|&k, _| k != addr);
                    println!("Client {} timed out, online users: {}", addr, players.len());
                }
                SocketEvent::Disconnect(addr) => {
                    players.retain(|&k, _| k != addr);
                    println!("Client {} disconnected, online users: {}", addr, players.len());
                }
                
                SocketEvent::Packet(packet) => {
                    match deserialize::<NetworkMessage>(packet.payload()) {
                        Ok(NetworkMessage::Hello) => {
                            if !players.contains_key(&packet.addr()) {
                                players.insert(packet.addr(), MyRectangle {
                                    x: 0.0,
                                    y: 0.0,
                                    w: 100.0,
                                    h: 100.0,
                                    clr: (255, 0, 0, 255),
                                });
                                println!("Client {} connected. online users: {}", packet.addr(), players.len());
                            }
                        }
                        Ok(NetworkMessage::Rect(message)) => {
                            if let Some(player) = players.get_mut(&packet.addr()) {
                                player.x = message.x;
                                player.y = message.y;
                                player.w = message.w;
                                player.h = message.h;
                                player.clr = message.clr;
                            } else {
                                println!("Client {} not found in players list", packet.addr());
                            }
                        }

                        _ => {
                            println!("Failed to deserialize packet");
                        }
                    }
                },
                
        
                _ => (),
            }
        }

        //send all players to all clients
        for (addr, _) in players.iter() {
            let _ = socket.send(Packet::unreliable(
                *addr,
                serialize(&NetworkMessage::Players(players.clone())).unwrap(),
            ));
        }
        
        
        thread::sleep(Duration::from_millis(16));
    }
}