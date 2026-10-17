use std::collections::HashMap;
use std::{fs::File, io::Read};

use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
pub struct BoardEntry {
    year: u16,
    fat: bool,
    hex: u32,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Config {
    current: u16,
    png_size: u32,
    board: Vec<BoardEntry>,
}

pub struct BoardState {
    pub current: u16,
    pub png_size: u32,
    pub map: HashMap<u16, (u32, bool)>,
}

pub fn get_board_color_hex(color: u32) -> String {
    format!("#{:06x}", color & 0xFFFFFF)
}

pub fn get_board_color_rgb(color: u32) -> (u8,u8,u8) {
    let r = ((color >> 16) & 0xFF) as u8;
    let g = ((color >>  8) & 0xFF) as u8;
    let b = ( color        & 0xFF) as u8;

    (r,g,b)
}

pub fn read_config(path: String) -> BoardState {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let config = toml::from_str::<Config>(&buf).unwrap();
    let current_board = config.current;
    let map = config.board.into_iter().map(|it| {
        (it.year, (it.hex, it.fat))
    }).collect::<HashMap<_,_>>();

    BoardState {
        current: current_board,
        png_size: config.png_size,
        map,
    }
}

