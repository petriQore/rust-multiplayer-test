
use serde_derive::{Deserialize, Serialize};
use macroquad::prelude::*;
use std::collections::HashMap;
use std::net::SocketAddr;


#[derive(Serialize, Deserialize)]
pub enum NetworkMessage {
    Hello,
    Rect(MyRectangle),
    Players(HashMap<SocketAddr, MyRectangle>),
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Clone, Copy)]
pub struct MyRectangle {
    pub x: f32,
    pub y: f32, 
    pub w: f32,
    pub h: f32,
    pub clr: (u8, u8, u8, u8),
}

impl MyRectangle{

    pub fn custom_draw_rect(&self) -> () {
        draw_rectangle(self.x, self.y, self.w, self.h, Color::new(
            self.clr.0 as f32 / 255.0,
            self.clr.1 as f32 / 255.0,
            self.clr.2 as f32 / 255.0,
            self.clr.3 as f32 / 255.0,
        ));
        }

    pub fn custom_draw_text(&self, port: String) -> () {
        draw_text(
            &[self.x.to_string(), ",".to_string(), self.y.to_string()].concat(),
            self.x,
            self.y,
            30.0,
            WHITE,
        );

        draw_text(
            &["id:",&port].concat(),
            self.x + self.w / 2.0 - measure_text(&["id:",&port].concat(), None, 24, 1.0).width / 2.0,
            self.y + (self.h / 2.0) - measure_text(&["id:",&port].concat(), None, 24, 1.0).height / 4.0,
            24.0,
            WHITE,
        );
    }    
    }    

    