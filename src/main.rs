mod chip;
use std::fs;
pub use chip::Chip8;
pub use chip::*;
use std::path::Path;
use bracket_lib::prelude::*;

impl GameState for Chip8 {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.print(1, 1, "Hello Bracket World");
    }
}
fn main() -> BError {

    let path: &Path = Path::new("C:/Users/lajua/Desktop/programming/projects/chip8-rust/src/Fishie.ch8");
    let rom = read_file(&path).unwrap();

    let mut chip: Chip8 = Chip8::new();
    load_program(&mut chip, &rom);
    chip.emulate_cycle();

    let context = BTermBuilder::simple::<i32>(100, 50)?
    .with_title("Hello Minimal Bracket World")
    .build()?;

    main_loop(context, chip)
}

fn read_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    //Reads file contents into vector
    fs::read(path)
}


