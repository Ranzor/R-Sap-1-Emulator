struct Sap1 {
    reg_a: u8,
    reg_b: u8,

    pc: u8,

    memory: [u8; 16],

    cf: bool,
    zf: bool,

    hlt: bool,
}

impl Sap1 {
    fn new() -> Self {
        Sap1 {
            reg_a: 0,
            reg_b: 0,
            pc: 0,
            memory: [0; 16],
            cf: false,
            zf: false,
            hlt: false,
        }
    }

    fn load_program(&mut self, program: &[u8]) {
        for (i, &byte) in program.iter().enumerate() {
            self.memory[i] = byte;
        }
    }
}

fn main() {
    let mut sap1 = Sap1::new();

    let program: [u8; 16] = [
        0b00010010, // LDA 2
        0b00100011, // LDB 3
        0b00110000, // ADD
        0b11100000, // HLT
        0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    ];
    sap1.load_program(&program);
}
