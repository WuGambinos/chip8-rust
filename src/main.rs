mod chip;
use std::fs;
use bracket_lib::prelude::*;
pub use chip::Chip8;
pub use chip::*;

use std::path::Path;

fn main() {
    let path: &Path = Path::new("Fishie.ch8");
    let rom = read_file(&path).unwrap();

    let mut chip: Chip8 = Chip8::new();
    load_program(&mut chip, &rom);
    chip.emulate_cycle();
}

fn read_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    //Reads file contents into vecotr
    fs::read(path)
}


