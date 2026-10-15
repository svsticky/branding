use std::collections::HashMap;
use std::{fs::File, io::Read};

use std::sync::RwLock;
use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
pub struct BoardEntry {
    year: u32,
    hex: u32,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Config {
    current: u32,
    sizes: Vec<u32>,
    board: Vec<BoardEntry>,
}

pub struct BoardState {
    pub current: u32,
    pub sizes: Vec<u32>,
    pub map: RwLock<HashMap<u32, u32>>,
}

pub fn get_board_color_hex(state: &BoardState, year: u32) -> Option<String> {
    state.map.read().unwrap().get(&year).map(|it| {
        format!("{:06x}", it & 0xFFFFFF)
    })
}

pub fn get_board_color_rgb(state: &BoardState, year: u32) -> Option<(u8,u8,u8)> {
    state.map.read().unwrap().get(&year).map(|it| {
        let r = ((it >> 16) & 0xFF) as u8;
        let g = ((it >>  8) & 0xFF) as u8;
        let b = ( it        & 0xFF) as u8;

        (r,g,b)
    })
}

pub fn read_config(path: String) -> BoardState {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let config = toml::from_str::<Config>(&buf).unwrap();
    let current_board = config.current;
    let map = config.board.into_iter().map(|it| {
        (it.year, it.hex)
    }).collect::<HashMap<u32,u32>>();

    BoardState {
        current: current_board,
        map: RwLock::new(map),
        sizes: config.sizes,
    }
}

