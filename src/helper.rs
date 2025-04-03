
use serde_derive::{Deserialize, Serialize};
use macroquad::prelude::*;

#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
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
    }    