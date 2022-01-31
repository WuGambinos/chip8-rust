use std::path::Path;
use std::ffi::OsStr;
use std::fs;
use std::io::Error;
struct Chip8 {
    memory: [u8; 4096],
    display: [u8; 64*32],
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
    draw_flag: u8,
}

impl Chip8 {
    fn new() -> Self {
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

    fn emulate_cycle(&mut self) {

        //Opcode 
        self.opcode = (self.memory[self.pc as usize] <<  8) as u16 | (self.memory[(self.pc+1) as usize] as u16);
        
        //First byte of opcode
        let first_byte: u16 = self.opcode & 0xF000;
    }


    fn op_0(&mut self) {

        match self.opcode & 0x000F {
            0x0000 => {
                for i in 0..(64*32) {
                    self.display[i as usize] = 0;
                }
                self.draw_flag = 1;
                self.pc += 2;
            },
            0x000E => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
                self.pc += 2;
            },

            _ => println!("Unkown Opcode"),
        };

    }

    fn op_1(&mut self) {
        if self.opcode & 0x0FFF == self.pc {
            self.halt = 1;
            println!("INFINTE LOOP");
        }

        self.pc = self.opcode & 0x0FFF;
    }

    fn op_2(&mut self) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = self.opcode & 0xFFF;
    }

    fn op_3(&mut self) {
        let x: u8 = ((self.opcode & 0xF00) >> 8) as u8; 

        if(self.v[x as usize] == self.opcode as u8) {
            self.pc += 4;
        } else { 
            self.pc += 2;
        }
    }
}


fn main() {
    println!("Hello, world!");

    let path: &Path = Path::new("Fishie.ch8");
    let rom = read_file(&path).unwrap();    

    let mut chip: Chip8 = Chip8::new();
    
   /* for i in rom.iter() {
        println!("{:#02x}", i);
    }*/

    load_program(&mut chip, &rom);


}


fn read_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    let res = fs::read(path);

    res
}

fn load_program(s: &mut Chip8, rom: &Vec<u8>){
    for i in 0..rom.len() {
        s.memory[(i+0x200) as usize] = rom[i];
    }

}

