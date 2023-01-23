pub mod chip;
pub mod sound;

use chip::*;
use std::env;
use anyhow::Error;
use anyhow::Result;

#[macro_use]
extern crate text_io;

fn main() -> Result<(), Error>{
    //Command line arguments
    let args: Vec<String> = env::args().collect();
    let game = args[1].as_str();
    let mut chip8 = Chip8::new();
    chip8.start(game)
}

