#![allow(unused_imports)]
use std::net::SocketAddr;
use std::thread;
use std::time::Duration;

use bincode::{deserialize, serialize};
// use serde_derive::{Deserialize, Serialize};

use laminar::{Packet, Socket, SocketEvent};

mod helper;
use helper::*;

const SERVER_ADDR: &str = "127.0.0.1:12345";

fn server_address() -> SocketAddr {
    SERVER_ADDR.parse().unwrap()
}


#[allow(unused_must_use)]
pub fn main() {

    let mut socket = Socket::bind(server_address()).unwrap();
    
    let receiver = socket.get_event_receiver();
    
    let _thread = thread::spawn(move || {
        socket.start_polling();
    });
    
    println!("Server started on {}", SERVER_ADDR);
    
    loop {
        if let Ok(event) = receiver.recv() {
            match event {
                SocketEvent::Packet(packet) => {
                    match deserialize::<MyRectangle>(packet.payload()) {
                        Ok(message) => {
                            println!("Message from {}: {:?}, {:?} ", packet.addr(), message.x, message.y);
                            
                            // sender.send(Packet::reliable_unordered())
                        },
                        Err(e) => {
                            println!("Failed to deserialize packet: {}", e);
                        }
                    }
                },
                _ => ()
            }
        }
        
        thread::sleep(Duration::from_millis(16));
    }
}