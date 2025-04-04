#![allow(unused_imports)]
use std::net::SocketAddr;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use bincode::{deserialize, serialize};
// use serde_derive::{Deserialize, Serialize};

use laminar::{Packet, Socket, SocketEvent, Config};

mod helper;
use helper::*;
use macroquad::conf;

const SERVER_ADDR: &str = "127.0.0.1:12345";

fn server_address() -> SocketAddr {
    SERVER_ADDR.parse().unwrap()
}


#[allow(unused_must_use)]
pub fn main() {

    let mut players: Vec<SocketAddr> = Vec::new();

    let mut config = Config::default();
    config.idle_connection_timeout = Duration::from_secs(3);

    let socket = Socket::bind_with_config(server_address(), config).unwrap();
    let socket = Arc::new(Mutex::new(socket));

    let receiver = socket.lock().unwrap().get_event_receiver();
    
    let socket_clone = Arc::clone(&socket);
    let _thread = thread::spawn(move || {
        socket_clone.lock().unwrap().start_polling();
    });
    
    println!("Server started on {}", SERVER_ADDR);
    
    loop {
        while let Ok(event) = receiver.try_recv() {
            // println!("Received event: {:?}", event);
            match event {

                SocketEvent::Disconnect(addr) | SocketEvent::Timeout(addr) => {
                    players.retain(|&x| x != addr);
                    println!("Client {} disconnected, online users: {}", addr, players.len());
                },

                SocketEvent::Packet(packet) => {
                    match deserialize::<NetworkMessage>(packet.payload()) {
                        Ok(NetworkMessage::Hello) => {
                            if !players.contains(&packet.addr()) {
                                players.push(packet.addr());
                                println!("Client {} connected. online users: {}", packet.addr(), players.len());
                            }
                        }
                        Ok(NetworkMessage::Rect(message)) => {
                            println!("Message from {}: {:?}, {:?}", packet.addr(), message.x, message.y);
                        }
                        Err(e) => {
                            println!("Failed to deserialize packet: {}", e);
                        }
                    }
                },
                
        
                _ => (),
            }
        }
        
        
        thread::sleep(Duration::from_millis(16));
    }
}