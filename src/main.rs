struct Chip8 {
    memory: [u8; 4096],
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

    fn emulate_cycle(&self) {
        let opcode = self.memory[self.pc as usize] <<  8 | self.memory[(self.pc+1) as usize];
    }
}


fn main() {
    println!("Hello, world!");


    let chip = Chip8::new();

    println!("{:?}", chip.memory.len());
}
