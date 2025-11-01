struct Sap1 {
    // A register
    reg_a: u8,
    // B register
    reg_b: u8,

    // Program counter
    pc: u8,

    // Memory (16 bytes)
    memory: [u8; 16],

    // Flags: Carry and Zero
    cf: bool,
    zf: bool,

    // Halt flag
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
    fn fetch(&mut self) -> u8 {
        let instruction = self.memory[self.pc as usize];
        self.pc = self.pc.wrapping_add(1);
        instruction
    }

    fn execute(&mut self, instruction: u8) {
        let opcode = instruction >> 4;
        let operand = instruction & 0x0F;

        match opcode {
            0x0 => {
                // NOP
                // Do Nothing
            }
            0x1 => {
                // LDA $ (from memory address)
                self.reg_a = self.memory[operand as usize];
            }
            0x2 => {
                // LDA # (immediate value)
                self.reg_a = operand;
            }
            0x3 => {
                // LDB $ (from memory address)
                self.reg_b = self.memory[operand as usize];
            }
            0x4 => {
                // LDB # (immediate value)
                self.reg_b = operand;
            }
            0x5 => {
                // ADD $ (from memory address)
                self.reg_b = self.memory[operand as usize];
                let (result, carry) = self.reg_a.overflowing_add(self.reg_b);
                self.reg_a = result;
                self.cf = carry;
                self.zf = result == 0;
            }
            0x6 => {
                // ADD # (immediate value)
                let (result, carry) = self.reg_a.overflowing_add(operand);
                self.reg_a = result;
                self.cf = carry;
                self.zf = result == 0;
            }
            0x7 => {
                // SUB $ (from memory address)
                self.reg_b = self.memory[operand as usize];
                let (result, borrow) = self.reg_a.overflowing_sub(self.reg_b);
                self.reg_a = result;
                self.cf = borrow;
                self.zf = result == 0;
            }
            0x8 => {
                // SUB # (immediate value)
                let (result, borrow) = self.reg_a.overflowing_sub(operand);
                self.reg_a = result;
                self.cf = borrow;
                self.zf = result == 0;
            }
            0x9 => {
                // STA (store A to memory address)
                self.memory[operand as usize] = self.reg_a;
            }
            0xA => {
                // JMP (jump to address)
                self.pc = operand;
            }
            0xB => {
                // CMP $ (compare with memory address)
                self.reg_b = self.memory[operand as usize];
                let result = self.reg_a.wrapping_sub(self.reg_b);
                self.zf = result == 0;
            }
            0xC => {
                // CMP # (compare with immediate value)
                let result = self.reg_a.wrapping_sub(operand);
                self.zf = result == 0;
            }
            0xD => {
                // BNE (branch if not equal)
                if !self.zf {
                    self.pc = operand;
                }
            }
            0xE => {
                // JPZ (jump if zero)
                if self.zf {
                    self.pc = operand;
                }
            }
            0xF => {
                // Special instructions (use operand to differentiate)
                match operand {
                    0x0 => {
                        // JPC (jump if carry)
                        if self.cf {
                            // Jump to address in next operand
                            let address = self.fetch();
                            self.pc = address;
                        }
                    }
                    0x1 => {
                        // INC A
                        self.reg_a = self.reg_a.wrapping_add(1);
                    }
                    0x2 => {
                        // DEC A
                        self.reg_a = self.reg_a.wrapping_sub(1);
                    }
                    0x3 => {
                        // OUT
                        println!("Output: {}", self.reg_a);
                    }
                    0xF => {
                        // HLT
                        self.hlt = true;
                    }
                    _ => {
                        // Unknown special instruction
                        println!("Unknown special instruction: {:04b}", operand);
                    }
                }
            }
            _ => {
                // Unknown instruction
                println!("Unknown instruction: {:08b}", instruction);
            }
        }
    }
}

fn main() {
    let mut sap1 = Sap1::new();

    let program: [u8; 16] = [
        0b00100010, // LDA 2
        0b01000011, // ADD 3
        0b11110011, // OUT
        0b11111111, // HLT
        0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    ];
    sap1.load_program(&program);
    while !sap1.hlt {
        let instruction = sap1.fetch();
        sap1.execute(instruction);
    }
}
