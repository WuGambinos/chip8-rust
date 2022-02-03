mod chip;
use std::fs;
pub use chip::Chip8;
pub use chip::*;
use std::path::Path;
use raylib::prelude::*;

fn main() {

    let path: &Path = Path::new("Fishie.ch8");
    let rom = read_file(&path).unwrap();

    let mut chip: Chip8 = Chip8::new();
    chip.load_fontset();
    load_program(&mut chip, &rom);

    //Raylib
    let (mut rl, thread) = raylib::init()
    .size(640, 480)
    .title("HEllo, World")
    .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLUE);
        chip.emulate_cycle();

        if chip.draw_flag == 1 {
            chip.draw_graphics(&mut d);
        }


        d.draw_text("Hello, World!", 10, 10, 20, Color::WHITE);
    }


}

fn read_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    //Reads file contents into vector
    fs::read(path)
}


