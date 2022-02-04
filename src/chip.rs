use rand::Rng;
use raylib::prelude::RaylibDrawHandle;
use raylib::prelude::*;

const FONT: &[u8] = &[
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];

#[derive(Debug)]
pub struct Chip8 {
    memory: [u8; 4096],
    display: [u8; 64 * 32],
    pc: u16,
    i: u16,
    stack: [u16; 16],
    sp: u16,
    delay_timer: u16,
    sound_timer: u16,
    opcode: u16,
    key: [u8; 16],
    v: [u8; 16],
    halt: u8,
    pub draw_flag: u8,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            memory: [0; 4096],
            display: [0; 64 * 32],
            pc: 0x200,
            i: 0,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            opcode: 0,
            key: [0; 16],
            v: [0; 16],
            halt: 0,
            draw_flag: 1,
        }
    }

    pub fn load_fontset(&mut self) {
        for (i,v ) in FONT.iter().enumerate() {
            self.memory[i] = *v;
        }
    }

    pub fn emulate_cycle(&mut self) {
        //Opcode
        self.opcode = ((self.memory[self.pc as usize] as u16) << 8)
            | (self.memory[(self.pc + 1) as usize] as u16);

        //First byte of opcode
        let first_byte: u16 = self.opcode & 0xF000;

        match first_byte {
            0x0000 => self.op_0(),
            0x1000 => self.op_1(),
            0x2000 => self.op_2(),
            0x3000 => self.op_3(),
            0x4000 => self.op_4(),
            0x5000 => self.op_5(),
            0x6000 => self.op_6(),
            0x7000 => self.op_7(),
            0x8000 => self.op_8(),
            0x9000 => self.op_9(),
            0xA000 => self.op_a(),
            0xB000 => self.op_b(),
            0xC000 => self.op_c(),
            0xD000 => self.op_d(),
            0xE000 => self.op_e(),
            0xF000 => self.op_f(),
            _ => println!("NO OPCODe"),
        };

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn op_0(&mut self) {
        match self.opcode & 0x000F {
            0x0000 => {
                for i in 0..(64 * 32) {
                    self.display[i as usize] = 0;
                }
                self.draw_flag = 1;
                self.pc += 2;
            }
            0x000E => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
                self.pc += 2;
            }

            _ => println!("Unkown Opcode"),
        };
    }

    pub fn op_1(&mut self) {
        if self.opcode & 0x0FFF == self.pc {
            self.halt = 1;
            println!("INFINTE LOOP");
        }

        self.pc = self.opcode & 0x0FFF;
    }

    pub fn op_2(&mut self) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = self.opcode & 0xFFF;
    }

    pub fn op_3(&mut self) {
        let x: u8 = ((self.opcode & 0xF00) >> 8) as u8;

        if self.v[x as usize] == self.opcode as u8 {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    pub fn op_4(&mut self) {
        let x: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        if self.v[x as usize] != (self.opcode) as u8 {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    pub fn op_5(&mut self) {
        let x: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let y: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        if self.v[x as usize] == self.v[y as usize] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    pub fn op_6(&mut self) {
        let x: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        self.v[x as usize] = self.opcode as u8;
        self.pc += 2;
    }

    pub fn op_7(&mut self) {
        
        let x: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let res = self.opcode & 0x0FF;
        self.v[x as usize] += res as u8;
        self.pc += 2;
    }

    pub fn op_8(&mut self) {
        let x: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let y: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        match self.opcode & 0x000F {
            0x0000 => {
                self.v[x as usize] = self.v[y as usize];
                self.pc += 2;
            }

            0x0001 => {
                self.v[x as usize] |= self.v[y as usize];
                self.pc += 2;
            }

            0x0002 => {
                self.v[x as usize] &= self.v[y as usize];
                self.pc += 2;
            }

            0x0003 => {
                self.v[x as usize] ^= self.v[y as usize];
                self.pc += 2;
            }

            0x0004 => {
                let cp: u8 = 0xFF - self.v[x as usize];

                if self.v[y as usize] > cp {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }

                self.v[x as usize] += self.v[y as usize];
                self.pc += 2;
            }

            0x0005 => {
                if self.v[y as usize] > self.v[x as usize] {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }

                self.v[x as usize] -= self.v[y as usize];
                self.pc += 2;
            }

            0x0006 => {
                self.v[0xF] = self.v[x as usize] & 1;
                self.v[x as usize] = self.v[y as usize] >> 1;

                self.pc += 2;
            }

            0x0007 => {
                if self.v[x as usize] > self.v[y as usize] {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }

                self.v[x as usize] = self.v[y as usize] - self.v[x as usize];
                self.pc += 2;
            }

            0x000E => {
                self.v[0xF] = self.v[x as usize] & 0x80;
                self.v[x as usize] = self.v[y as usize] << 1;
                self.pc += 2;
            }

            _ => {
                println!("NO OPCODE");
            }
        };
    }

    pub fn op_9(&mut self) {
        let x: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let y: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        if self.v[x as usize] != self.v[y as usize] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    pub fn op_a(&mut self) {
        self.i = self.opcode & 0x0FFF;
        self.pc += 2;
    }

    pub fn op_b(&mut self) {
        self.pc += (self.v[0] as u16) + (self.opcode & 0x0FFF);
    }

    pub fn op_c(&mut self) {
        let mut rng = rand::thread_rng();
        let x: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let r: u16 = rng.gen();
        self.v[x as usize] = (r & (self.opcode & 0x00FF)) as u8;
        self.pc += 2;
    }

    pub fn op_d(&mut self) {
        let x: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let y: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        //Height
        let h: u8 = (self.opcode & 0x000F) as u8;

        let mut pixel: u8 = 0;
        self.v[0xF] = 0;

        for yline in 0..h {
            pixel = self.memory[(self.i + (yline as u16)) as usize];

            for xline in 0..8 {
                if (pixel & (0x80 >> xline)) != 0 {
                    if self.display[(self.v[x as usize]
                        + xline
                        + ((self.v[y as usize] + yline) * 64))
                        as usize]
                        == 1
                    {
                        self.v[0xF] = 1;
                    }

                    self.display[(self.v[x as usize] + xline + ((self.v[y as usize] + yline) * 64))
                        as usize] ^= 1;
                }
            }
        }

        self.draw_flag = 1;
        self.pc += 2;
    }

    pub fn op_e(&mut self) {
        let x: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        match self.opcode as u8 {
            0x009E => {
                if self.key[self.v[x as usize] as usize] != 0 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            0x00A1 => {
                if self.key[self.v[x as usize] as usize] == 0 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            _ => println!("NO OPCODe"),
        };
    }

    pub fn op_f(&mut self) {
        let x: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        match self.opcode as u8 {
            0x0007 => {
                self.v[x as usize] = self.delay_timer as u8;
                self.pc += 2;
            }

            0x000A => {
                let mut key_press = 0;
                for i in 0..16 {
                    if self.key[i as usize] != 0 {
                        self.v[x as usize] = i as u8;
                        key_press = 1;
                    }
                }

                if key_press == 0 {
                    return;
                }

                self.pc += 2;
            }

            0x0015 => {
                self.delay_timer = self.v[x as usize] as u16;
                self.pc += 2;
            }

            0x0018 => {
                self.sound_timer = self.v[x as usize] as u16;
                self.pc += 2;
            }

            0x001E => {
                self.i += self.v[x as usize] as u16;
                self.pc += 2;
            }

            0x0029 => {
                self.i = (self.v[x as usize] * 5) as u16;
                self.pc += 2;
            }

            0x0033 => {
                //Hundreds Digit
                self.memory[self.i as usize] = (self.v[x as usize] / 100) % 10;

                //Tens Digit
                self.memory[(self.i + 1) as usize] = (self.v[x as usize] / 10) % 10;

                //Ones digit
                self.memory[(self.i + 2) as usize] = (self.v[x as usize]) % 10;

                self.pc += 2;
            }

            0x0055 => {
                let mut i: u16 = 0x00;

                while i <= x as u16 {
                    self.memory[(self.i + i) as usize] = self.v[i as usize];
                    i += 1;
                }

                self.pc += 2;
            }

            0x0065 => {
                let mut i: u16 = 0x00;

                while i <= x as u16 {
                    self.v[i as usize] = self.memory[(self.i + i) as usize];
                    i += 1;
                }

                self.pc += 2;
            }
            _ => println!("NO OPCODE"),
        }
    }

    pub fn draw_graphics(&self, display: &mut RaylibDrawHandle) {
        for y in 0..32 {
            for x in 0..64 {
                if self.display[(y * 64) + x] != 0 {
                    display.draw_rectangle((x * 10) as i32, (y * 10) as i32, 10, 10, Color::WHITE);
                } else {
                    display.draw_rectangle((x * 10) as i32, (y * 10) as i32, 10, 10, Color::BLACK);
                }
            }
        }
    }
}

pub fn load_program(s: &mut Chip8, rom: &[u8]) {
    for (i, v) in rom.iter().enumerate() {
        s.memory[(i + 0x200) as usize] = *v;
    }
}
