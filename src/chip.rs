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
///Representation of CHIP-8 Virtual Machine
pub struct Chip8 {
    ///Memory
    memory: [u8; 4096],

    ///Display
    display: [u8; 64 * 32],

    ///Program Counter
    pc: u16,

    ///Index Register
    i: u16,

    ///Stack
    stack: [u16; 16],

    ///Stack Pointer
    sp: u16,

    ///Delay Timer
    delay_timer: u16,

    ///Sound Timer
    sound_timer: u16,

    ///Current opcode
    opcode: u16,

    ///Keys
    key: [u8; 16],

    ///General Purpose Registers
    v: [u8; 16],

    ///Halt Flag
    halt: u8,

    ///Draw Flag
    pub draw_flag: u8,
}

impl Chip8 {
    ///Instantiate an instance of CHIP-8
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

    //Load fontset into memory
    pub fn load_fontset(&mut self) {
        for (i, v) in FONT.iter().enumerate() {
            self.memory[i] = *v;
        }
    }

    ///Execute a Cycle
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

    /// THIS FUNCTION CONTAINS MULTIPLE OPCODES
    ///
    /// 00E0 - Clears screen
    ///
    /// 00EE - Return (exit a subroutine)
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

    /// 1NNN - JMP NNN
    ///
    /// Sets Program Counter to NNN
    pub fn op_1(&mut self) {
        if self.opcode & 0x0FFF == self.pc {
            self.halt = 1;
            println!("INFINTE LOOP");
        }

        self.pc = self.opcode & 0x0FFF;
    }

    /// 2NNN - CALL NNN
    ///
    /// Increments stack pointer, then puts the current PC on top of the stack
    ///
    /// PC is then set to NNN
    pub fn op_2(&mut self) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = self.opcode & 0xFFF;
    }

    /// 3XNN - Skip if VX === NN
    ///
    /// Skip next instructino if value in register VX is equal to NN
    ///
    pub fn op_3(&mut self) {
        let x: u8 = ((self.opcode & 0xF00) >> 8) as u8;

        if self.v[x as usize] == self.opcode as u8 {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    /// 4XNN - Skip if VX != NN
    ///
    /// Skip next instruction if value in resgier VX is not equal to NN
    pub fn op_4(&mut self) {
        let x: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        if self.v[x as usize] != (self.opcode) as u8 {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    /// 5XY0 - Skip if VX == VY
    ///
    /// Skip next instruction if value in register VX is equal to value in register VY
    pub fn op_5(&mut self) {
        let x: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let y: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        if self.v[x as usize] == self.v[y as usize] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    /// 6XNN - VX = NN
    ///
    /// Store value NN in register VX
    pub fn op_6(&mut self) {
        let x: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        self.v[x as usize] = self.opcode as u8;
        self.pc += 2;
    }

    /// 7XKK - VX += NN
    ///
    /// Add value NN to register VX
    pub fn op_7(&mut self) {
        let x: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let res = self.opcode & 0x0FF;

        //Fix overflow problem
        self.v[x as usize] = (self.v[x as usize]).wrapping_add(res as u8);
        self.pc += 2;
    }

    /// THIS FUNCTION CONTAINS MULTIPLE OPCODES
    ///
    /// 8XY0 - VX = VY
    ///
    /// Store value of register VY in VX
    ///
    /// 8XY1 - VX = VX |  VY
    ///
    /// Set register VX to the logical OR of register VX and register VY
    ///
    /// 8XY2 - VX = VX & VY
    ///
    /// Set register VX to the logical AND of register VX and register VY
    ///
    /// 8XY3 - VX = VX ^ VY
    ///
    /// Set register VX to the logical XOR or register VX and register VY
    ///
    /// 8XY4 - VX = VX + VY, VF = carry
    ///
    /// Add value of Vy to register VX
    ///
    /// Set VF if carry occurs
    ///
    /// Clear VF otherwise
    ///
    /// 8XY5 - VX = VX - VY, set VF = NOT borrow
    ///
    /// Subtract vlaue of register VY from register VX
    ///
    /// Clear VF if a borrow occurs
    ///
    /// Set VF if a borrow does not occur
    ///
    /// 8XY6 - VX = VX >> 1
    ///
    /// Store the value of register VY shifted right one bit in register VX
    ///
    /// Set register VF to LSB of VX prior to shift
    ///
    /// 8XY7 - VX = VY - VX, set VF = NOT borrow
    ///
    /// Set register VX to the value of VY - VX
    ///
    /// Clear VF if borrow occurs
    ///
    /// Set VF if borrow does not occur
    ///
    /// 8XYE - Store the value of register VY shifted to left by one bit in register VX
    ///
    /// Set register VF to the MSB of VX prior to shift
    ///
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

    /// 9XY - Skip if VX != VY
    ///
    /// Skips next instruction if the value in register VX doesn't equal the value in register VY
    pub fn op_9(&mut self) {
        let x: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let y: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        if self.v[x as usize] != self.v[y as usize] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    /// ANNN - I = NNN
    /// Store address NNN in register I
    ///
    pub fn op_a(&mut self) {
        self.i = self.opcode & 0x0FFF;
        self.pc += 2;
    }

    /// BNNN - PC = NNN + V0
    ///
    /// Jump to address NNN + V0
    pub fn op_b(&mut self) {
        self.pc += (self.v[0] as u16) + (self.opcode & 0x0FFF);
    }

    /// CXKK - VX = (random byte) & (KK)
    ///
    ///
    pub fn op_c(&mut self) {
        let mut rng = rand::thread_rng();
        let x: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let r: u16 = rng.gen();
        self.v[x as usize] = (r & (self.opcode & 0x00FF)) as u8;
        self.pc += 2;
    }

    /// DXYN - Display n-byte sprite starting at memory locatoin I at (VX, VY)
    ///
    /// set VF to 01 if nay set pixels are changed to unset, and 00 otherwise
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
                    let mut y_calc: u64 = (self.v[y as usize] + yline) as u64;
                    y_calc = y_calc.wrapping_mul(64);

                    //let y_calc: u64 = ((self.v[y as usize] + yline) * 64) as u64;

                    let inner: u64 = self.v[x as usize] as u64 + xline as u64 + y_calc;

                    if self.display[inner as usize] == 1 {
                        self.v[0xF] = 1;
                    }

                    self.display[inner as usize] ^= 1;
                }
            }
        }

        self.draw_flag = 1;
        self.pc += 2;
    }

    /// THIS FUNCTION CONTAINS MULTIPLE OPCODES
    ///
    /// EX9E - Skip next instruction if the key corresponding to hex value
    ///
    /// currently stored in register VX is pressed
    ///
    ///
    /// EXA1 - Skip next instruction if the key corresponding to hex value
    ///
    /// currently stored in register VX is not pressed
    ///
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

    /// THIS FUNCTION CONTAINS MULTIPLE OPCODES
    ///
    /// FX07 - VX = delay_timer
    ///
    /// Store delay timer value in register VX
    ///
    /// FX0A - Wait for key press, store the value of key in register VX
    ///
    /// FX15 - delay_timer = VX
    ///
    /// FX18 - sound_timer = VX
    ///
    /// FX1E - I = I + VX
    ///
    /// FX29 - Store address of sprite data corresponding to
    ///
    /// hexadecimal digit stored in register VX
    ///
    /// FX33 - Store BCD representation of value in register VX
    ///
    /// in memory locations I, I+1, and I+2
    ///
    /// FX55 - Store values of registers V0 through VX in memory starting at
    /// location represented by value in register I
    ///
    /// FX65 - Read values in registers V0 through VX from memory starting at
    /// location represented by value in register I
    ///
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

    pub fn check_keys(&mut self, rl: &mut RaylibDrawHandle) {
        //If '1' key is pressed or down
        self.key[0x1] =
            (rl.is_key_pressed(KeyboardKey::KEY_ONE) || rl.is_key_down(KeyboardKey::KEY_ONE)) as u8;

        //If '2' key is pressed or down
        self.key[0x2] =
            (rl.is_key_pressed(KeyboardKey::KEY_TWO) || rl.is_key_down(KeyboardKey::KEY_TWO)) as u8;

        //If '3' key  is pressed or down
        self.key[0x3] = (rl.is_key_pressed(KeyboardKey::KEY_THREE)
            || rl.is_key_down(KeyboardKey::KEY_THREE)) as u8;

        //If '4' key is pressed or down
        self.key[0xC] = (rl.is_key_pressed(KeyboardKey::KEY_FOUR)
            || rl.is_key_down(KeyboardKey::KEY_FOUR)) as u8;
        
        //If 'Q' key is pressed or down
        self.key[0x4] = (rl.is_key_pressed(KeyboardKey::KEY_Q)
        || rl.is_key_down(KeyboardKey::KEY_Q)) as u8;
    
        //If 'W' key is pressed or down
        self.key[0x5] = (rl.is_key_pressed(KeyboardKey::KEY_W)
        || rl.is_key_down(KeyboardKey::KEY_W)) as u8;

        //If 'E' key is pressed or down
        self.key[0x6] = (rl.is_key_pressed(KeyboardKey::KEY_E)
        || rl.is_key_down(KeyboardKey::KEY_E)) as u8;

        //If 'R' key is pressed or down
        self.key[0xD] = (rl.is_key_pressed(KeyboardKey::KEY_R)
        || rl.is_key_down(KeyboardKey::KEY_R)) as u8;
        
        //If 'A' key is pressed or down
        self.key[0x7] = (rl.is_key_pressed(KeyboardKey::KEY_A)
        || rl.is_key_down(KeyboardKey::KEY_A)) as u8;

        //If 'S' key is pressed or down
        self.key[0x8] = (rl.is_key_pressed(KeyboardKey::KEY_S)
        || rl.is_key_down(KeyboardKey::KEY_S)) as u8;

        //If 'D' key is pressed or down
        self.key[0x9] = (rl.is_key_pressed(KeyboardKey::KEY_D)
        || rl.is_key_down(KeyboardKey::KEY_D)) as u8;

        //If 'F' Key is pressed or down
        self.key[0xE] = (rl.is_key_pressed(KeyboardKey::KEY_F)
        || rl.is_key_down(KeyboardKey::KEY_F)) as u8;

        self.key[0xA] = (rl.is_key_pressed(KeyboardKey::KEY_Z)
        || rl.is_key_down(KeyboardKey::KEY_Z)) as u8;

        self.key[0x0] = (rl.is_key_pressed(KeyboardKey::KEY_X)
        || rl.is_key_down(KeyboardKey::KEY_X)) as u8;

        self.key[0xB] = (rl.is_key_pressed(KeyboardKey::KEY_C)
        || rl.is_key_down(KeyboardKey::KEY_C)) as u8;

        self.key[0xF] = (rl.is_key_pressed(KeyboardKey::KEY_V)
        || rl.is_key_down(KeyboardKey::KEY_V)) as u8;


    }

    /// Draws graphics to raylib window
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

///Loads rom into memory of CHIP-8 Virtual Machine
pub fn load_program(s: &mut Chip8, rom: &[u8]) {
    for (i, v) in rom.iter().enumerate() {
        s.memory[(i + 0x200) as usize] = *v;
    }
}
