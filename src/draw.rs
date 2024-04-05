use piston_window::{rectangle,Context,G2d};
use piston_window::types::Color;



const BLOCK_SIZE: f64 = 25.0;



pub fn to_cord(game_cord:i32)-> f64 {
    (game_cord as f64) * BLOCK_SIZE
}

pub fn to_cord_u32(game_cord:i32)-> u32 {
    to_cord(game_cord) as u32
}


pub fn draw_block(col:Color, x:i32,y:i32,con:&Context, g: &mut G2d) {
    let gui_x = to_cord(x);
    let gui_y = to_cord(y);

    rectangle(col,[gui_x,gui_y,BLOCK_SIZE,BLOCK_SIZE],con.transform,g);
}


pub fn draw_rectangle(col:Color, x:i32,y:i32,width:i32,height:i32,con:&Context, g: &mut G2d) {
    let x = to_cord(x);
    let y = to_cord(y);

    rectangle(col,[x,y,BLOCK_SIZE*(width as f64),BLOCK_SIZE*(height as f64)],con.transform,g);
}