mod chip;
pub use chip::Chip8;
pub use chip::*;
use raylib::prelude::*;
use std::fs;
use std::path::Path;

#[macro_use] extern crate text_io;
fn main() {

    let file_name = "Fishie.ch8";
    //Path to rom
    let path: &Path = Path::new(file_name);

    //Contents of rom
    let rom: Vec<u8> = read_file(&path).unwrap();

    //Chip8 Chip
    let mut chip: Chip8 = Chip8::new();

    //Load fontsent into memory
    chip.load_fontset();

    //Load rom into memory
    load_program(&mut chip, &rom);

    //String used to store input
    let mut choice: i32 = -1;


    //Keep trying to get input until valid input
    while choice != 0 && choice != 1 {

        println!("Press 0 to enter debug mode");
        println!("Press 1 to run the rom");


        //Read input into "choice" variable
        choice = read!();
    }

    if choice == 1 {
        //Raylib
        let (mut rl, thread) = raylib::init().size(640, 480).title("HEllo, World").build();

        //Set FPS to 60
        rl.set_target_fps(60);

        while !rl.window_should_close() {
            //Begin Drawing
            let mut d = rl.begin_drawing(&thread);

            //Give Window a blue background
            d.clear_background(Color::BLUE);

            //Complete a cycle on the chip8
            chip.emulate_cycle();

            //Draw graphics if draw_flag is set
            if chip.draw_flag == 1 {
                chip.draw_graphics(&mut d);
            }

            //Draw hello world to screen
            d.draw_text("Hello, World!", 10, 10, 20, Color::WHITE);
        }
    } else {
        let mut step = -1;
        println!("Press 1 to go on to next cycle");

        while step == -1 {
            step = read!();

            if step == 1 {
                //Emulate a cycle
                chip.emulate_cycle();

                //Print state of chip
                println!("{:#X?}", chip);
                println!();
                step = -1;

                println!("Press 1 to go on to next cycle");
            }
        }
    }
}

fn read_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    //Reads file contents into vector
    fs::read(path)
}
