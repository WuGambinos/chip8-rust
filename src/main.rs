pub mod chip;
pub mod sound;
mod support;
use anyhow::Result;
use chip::*;
use imgui::*;
use std::env;

/*
fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    let game = args[1].as_str();
    let mut chip8 = Chip8::new();
    chip8.old_start(game)
}
*/

/*
// rect is [x, y, w, h]
fn draw_text_centered(
    ui: &Ui,
    draw_list: &DrawListMut,
    rect: [f32; 4],
    text: &str,
    color: [f32; 3],
) {
    let text_size = ui.calc_text_size(text);
    let cx = (rect[2] - text_size[0]) / 2.0;
    let cy = (rect[3] - text_size[1]) / 2.0;
    draw_list.add_text([rect[0] + cx, rect[1] + cy], color, text);
}
*/

fn main() {
    let args: Vec<String> = env::args().collect();
    let game = args[1].as_str();
    let mut chip8 = Chip8::new();
    let _ = chip8.start(game);

    let system = support::init(file!());
    system.main_loop(move |_, ui| {
        chip8.emulate_cycle();

        ui.window("Debug Window")
            .position([200.0, 500.0], Condition::FirstUseEver)
            .size([150.0, 400.0], Condition::FirstUseEver)
            .build(|| {
                let pc = format!("PC: {:#X}", chip8.pc);
                let sp = format!("SP: {:#X}", chip8.sp);
                let i = format!("I: {:#X}", chip8.i);
                let delay_timer = format!("DELAY TIMER: {}", chip8.delay_timer);
                let sound_timer = format!("SOUND TIMER: {}", chip8.sound_timer);
                let draw_flag = format!("DRAW: {}", chip8.draw_flag);
                ui.text(pc);
                ui.text(sp);
                ui.text(i);
                ui.text(draw_flag);
                ui.text(delay_timer);
                ui.text(sound_timer);

                for (i, reg) in chip8.v.iter().enumerate() {
                    let register = format!("V[{:X}]: {:#X}", i, reg);
                    ui.text(register);
                }
            });
        ui.window("Chip8 Emualtor")
            .size([600.0, 340.0], Condition::FirstUseEver)
            .position([0.0, 0.0], Condition::FirstUseEver)
            .scroll_bar(false)
            .build(|| {
                let draw_list = ui.get_window_draw_list();
                const SQUARE_SIZE: f32 = 10.0;

                let origin = ui.cursor_screen_pos();

                for y in 0..32 {
                    for x in 0..64 {
                        if chip8.display[(y * 64) + x] == 1 {
                            let mut top_left = [x as f32 * SQUARE_SIZE, y as f32 * SQUARE_SIZE];

                            top_left[0] += origin[0];
                            top_left[1] += origin[1];

                            let bottom_right =
                                [top_left[0] + SQUARE_SIZE, top_left[1] + SQUARE_SIZE];

                            let color = ImColor32::from_rgb(0, 255, 0);

                            draw_list
                                .add_rect(top_left, bottom_right, color)
                                .filled(true)
                                .build();
                        } else {
                            let mut top_left = [x as f32 * SQUARE_SIZE, y as f32 * SQUARE_SIZE];

                            top_left[0] += origin[0];
                            top_left[1] += origin[1];

                            let bottom_right =
                                [top_left[0] + SQUARE_SIZE, top_left[1] + SQUARE_SIZE];
                            let color = ImColor32::from_rgb(0, 0, 0);

                            draw_list
                                .add_rect(top_left, bottom_right, color)
                                .filled(true)
                                .build();
                        }
                    }
                }
            });
    });
}
