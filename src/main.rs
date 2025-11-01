struct Sap1 {
    // A register
    reg_a: u8,
    // B register
    reg_b: u8,
    // ALU output
    alu_out: u8,

    // Program counter
    pc: u8,

    // Memory (256 bytes)
    memory: [u8; 256],

    // Flags: Carry and Zero
    cf: bool,
    zf: bool,

    // Halt flag
    hlt: bool,
    // Current execution step
    t_step: u8,

    // Hardware components for visualization
    // Bus
    bus: u8,
    // Memory Address Register
    mar: u8,
    // Instruction Register
    ir: u8,
}

impl Sap1 {
    fn new() -> Self {
        Sap1 {
            reg_a: 0,
            reg_b: 0,
            pc: 0,
            memory: [0; 256],
            cf: false,
            zf: false,
            hlt: false,
            alu_out: 0,
            t_step: 1,
            bus: 0,
            mar: 0,
            ir: 0,
        }
    }

    fn load_program(&mut self, program: &[u8]) {
        for (i, &byte) in program.iter().enumerate() {
            self.memory[i] = byte;
        }
    }

    fn clock_tick(&mut self) {
        match self.t_step {
            1 => {
                // T1: CO, MI
                self.bus = self.pc;
                self.mar = self.bus;
                self.t_step += 1;

                println!("T1 - PC: {}, MAR: {}", self.pc, self.mar);
            }
            2 => {
                // T2: RO, II, CE
                self.bus = self.memory[self.mar as usize];
                self.ir = self.bus;
                self.pc = self.pc.wrapping_add(1);
                self.t_step += 1;

                println!("T2 - IR: {:06b}, PC: {}", self.ir, self.pc);
            }
            _ => {
                match self.ir >> 4 {
                    0x0 => {
                        // NOP
                        println!("T3 - NOP Executed");
                        self.t_step = 1;
                    }
                    0x1 => {
                        // LDA $ (load from memory address)
                        match self.t_step {
                            3 => {
                                // T3: CO, MI
                                self.bus = self.pc;
                                self.mar = self.bus;
                                self.t_step += 1;
                                println!("T3 - LDA $");
                            }
                            4 => {
                                // T4: RO, MI
                                self.bus = self.memory[self.mar as usize];
                                self.mar = self.bus;
                                self.t_step += 1;
                                println!("T4 - LDA $ Address Loaded");
                            }
                            5 => {
                                // T5: RO, AI, CE
                                self.bus = self.memory[self.mar as usize];
                                self.reg_a = self.bus;
                                self.pc = self.pc.wrapping_add(1);
                                self.t_step += 1;
                                println!("T5 - LDA $ Executed");
                            }
                            6 => {
                                // T6 PR
                                self.t_step = 1;
                                println!("T6 - LDA $ Completed");
                            }
                            _ => {}
                        }
                    }
                    0x2 => {
                        // LDA # (immediate value)
                        match self.t_step {
                            3 => {
                                // T3: CO, MI
                                self.bus = self.pc;
                                self.mar = self.bus;
                                self.t_step += 1;
                                println!("T3 - LDA");
                            }
                            4 => {
                                // T4: RO, AI, CE
                                self.bus = self.memory[self.mar as usize];
                                self.reg_a = self.bus;
                                self.pc = self.pc.wrapping_add(1);
                                self.t_step += 1;
                                println!("T4 = LDA Executed");
                            }
                            5 => {
                                // T5 PR
                                self.t_step = 1;
                                println!("T5 - LDA Completed");
                            }
                            _ => {}
                        }
                    }
                    0x3 => {
                        // LDB $ (load from memory address)
                        match self.t_step {
                            3 => {
                                // T3: CO, MI
                                self.bus = self.pc;
                                self.mar = self.bus;
                                self.t_step += 1;
                                println!("T3 - LDB $");
                            }
                            4 => {
                                // T4: RO, MI
                                self.bus = self.memory[self.mar as usize];
                                self.mar = self.bus;
                                self.t_step += 1;
                                println!("T4 - LDB $ Address Loaded");
                            }
                            5 => {
                                // T5: RO, BI, CE
                                self.bus = self.memory[self.mar as usize];
                                self.reg_b = self.bus;
                                self.pc = self.pc.wrapping_add(1);
                                self.t_step += 1;
                                println!("T5 - LDB $ Executed");
                            }
                            6 => {
                                // T6 PR
                                self.t_step = 1;
                                println!("T6 - LDB $ Completed");
                            }
                            _ => {}
                        }
                    }
                    0x4 => {
                        // LDB # (immediate value)
                        match self.t_step {
                            3 => {
                                // T3: CO, MI
                                self.bus = self.pc;
                                self.mar = self.bus;
                                self.t_step += 1;
                                println!("T3 - LDB");
                            }
                            4 => {
                                // T4: RO, BI, CE
                                self.bus = self.memory[self.mar as usize];
                                self.reg_b = self.bus;
                                self.pc = self.pc.wrapping_add(1);
                                self.t_step += 1;
                                println!("T4 - LDB Executed");
                            }
                            5 => {
                                // T5 PR
                                self.t_step = 1;
                                println!("T5 - LDB Completed");
                            }
                            _ => {}
                        }
                    }
                    0xF => {
                        match self.ir & 0x0F {
                            0xF => {
                                // HLT
                                if self.t_step == 3 {
                                    // T3: HLT
                                    println!("T3 - HLT Executed");
                                    self.t_step += 1;
                                    self.hlt = true;
                                } else if self.t_step == 4 {
                                    // T4 PR
                                    println!("T4 - HLT Completed");
                                    self.t_step = 1;
                                }
                            }
                            _ => {
                                println!("Unknown opcode: {:04b}", self.ir >> 4);
                            }
                        }
                    }
                    _ => {
                        println!("Unknown opcode: {:04b}", self.ir >> 4);
                        self.t_step = 1;
                    }
                }
            }
        }
    }
}

fn main() {
    let mut sap1 = Sap1::new();

    let mut program = [0u8; 256];

    program[0] = 0b00100000; // LDA #
    program[1] = 0b00101010; // 42
    program[2] = 0b00010000; // LDA $
    program[3] = 0b00001000; // Address 8
    program[4] = 0b00110000; // LDB $
    program[5] = 0b00001000; // Address 8
    program[6] = 0b11111111; // HLT
    program[8] = 0b00011000; // Data at address 8: 24

    sap1.load_program(&program);
    while !sap1.hlt {
        if sap1.reg_a == sap1.reg_b {
            sap1.zf = true;
        } else {
            sap1.zf = false;
        }
        sap1.clock_tick();
    }
}
