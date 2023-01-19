pub mod chip;

use chip::*;

use raylib::prelude::*;
use std::fs;
use std::path::Path;
use std::env;

#[macro_use]
extern crate text_io;

fn main() {
    //Command line arguments
    let args: Vec<String> = env::args().collect();
    let game = args[1].as_str();
    let mut chip8 = Chip8::new();
    chip8.start(game);
}

