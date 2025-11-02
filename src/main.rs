enum ClockMode {
    RUN,
    STEP,
}

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

    // Clock mode
    clock_mode: ClockMode,
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
            clock_mode: ClockMode::STEP,
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
                    0x5 => {
                        // ADD $
                        match self.t_step {
                            3 => {
                                // T3: CP, MI
                                self.bus = self.pc;
                                self.mar = self.bus;
                                self.t_step += 1;
                                println!("T3 - ADD $");
                            }
                            4 => {
                                // T4: RO, MI
                                self.bus = self.memory[self.mar as usize];
                                self.mar = self.bus;
                                self.t_step += 1;
                                println!("T4 - ADD $ Address Loaded");
                            }
                            5 => {
                                // T5: RO, BI
                                self.bus = self.memory[self.mar as usize];
                                self.reg_b = self.bus;
                                self.t_step += 1;
                                println!("T5 - ADD $ B Loaded");
                            }
                            6 => {
                                // T6: EO, AI, CE
                                let (result, carry) = self.reg_a.overflowing_add(self.reg_b);
                                self.reg_a = result;
                                self.cf = carry;
                                self.pc = self.pc.wrapping_add(1);
                                self.t_step += 1;
                                println!("T6 - ADD $ Executed");
                            }
                            7 => {
                                // T7 PR
                                self.t_step = 1;
                                println!("T7 - ADD $ Completed");
                            }
                            _ => {}
                        }
                    }
                    0x6 => {
                        // ADD #
                        match self.t_step {
                            3 => {
                                // T3: CO, MI
                                self.bus = self.pc;
                                self.mar = self.bus;
                                self.t_step += 1;
                                println!("T3 - ADD #");
                            }
                            4 => {
                                // T4: RO, BI
                                self.bus = self.memory[self.mar as usize];
                                self.reg_b = self.bus;
                                self.t_step += 1;
                                println!("T4 - ADD # B Loaded");
                            }
                            5 => {
                                // T5: EO, AI, CE
                                let (result, carry) = self.reg_a.overflowing_add(self.reg_b);
                                self.reg_a = result;
                                self.cf = carry;
                                self.pc = self.pc.wrapping_add(1);
                                self.t_step += 1;
                                println!("T5 - ADD # Executed");
                            }
                            6 => {
                                // T6 PR
                                self.t_step = 1;
                                println!("T6 - ADD # Completed");
                            }
                            _ => {}
                        }
                    }
                    0x7 => {
                        // SUB $
                        match self.t_step {
                            3 => {
                                // T3: CO, MI
                                self.bus = self.pc;
                                self.mar = self.bus;
                                self.t_step += 1;
                                println!("T3 - SUB $");
                            }
                            4 => {
                                // T4: RO, MI
                                self.bus = self.memory[self.mar as usize];
                                self.mar = self.bus;
                                self.t_step += 1;
                                println!("T4 - SUB $ Address Loaded");
                            }
                            5 => {
                                // T5: RO, BI, SU
                                self.bus = self.memory[self.mar as usize];
                                self.reg_b = self.bus;
                                self.t_step += 1;
                                println!("T5 - SUB $ B Loaded");
                            }
                            6 => {
                                // T6: EO, AI, SU, CE
                                let (result, borrow) = self.reg_a.overflowing_sub(self.reg_b);
                                self.reg_a = result;
                                self.cf = borrow;
                                self.pc = self.pc.wrapping_add(1);
                                self.t_step += 1;
                                println!("T6 - SUB $ Executed");
                            }
                            7 => {
                                // T7 PR
                                self.t_step = 1;
                                println!("T7 - SUB $ Completed");
                            }
                            _ => {}
                        }
                    }
                    0x8 => {
                        // SUB #
                        match self.t_step {
                            3 => {
                                // T3: CO, MI
                                self.bus = self.pc;
                                self.mar = self.bus;
                                self.t_step += 1;
                                println!("T3 - SUB #");
                            }
                            4 => {
                                // T4: RO, BI, SI
                                self.bus = self.memory[self.mar as usize];
                                self.reg_b = self.bus;
                                self.t_step += 1;
                                println!("T4 - SUB # B Loaded");
                            }
                            5 => {
                                // T5: EO, AI, SU, CE
                                let (result, borrow) = self.reg_a.overflowing_sub(self.reg_b);
                                self.reg_a = result;
                                self.cf = borrow;
                                self.pc = self.pc.wrapping_add(1);
                                self.t_step += 1;
                                println!("T5 - SUB # Executed");
                            }
                            6 => {
                                // T6: PR
                                self.t_step = 1;
                                println!("T6 - SUB # Completed");
                            }
                            _ => {}
                        }
                    }
                    0x9 => {
                        // STA
                        // This is probably a bugged instruction in the original
                        // R-SAP-1 design
                        match self.t_step {
                            3 => {
                                // T3: CO, MI
                                self.bus = self.pc;
                                self.mar = self.bus;
                                self.t_step += 1;
                                println!("T3 - STA");
                            }
                            4 => {
                                // T4: RI, MI, CE
                                self.bus = self.memory[self.mar as usize];
                                self.mar = self.bus;
                                self.pc = self.pc.wrapping_add(1);
                                self.t_step += 1;
                                println!("T4 - STA Address Loaded");
                            }
                            5 => {
                                // T5: AO, RI
                                self.bus = self.reg_a;
                                self.memory[self.mar as usize] = self.bus;
                                self.t_step += 1;
                                println!("T5 - STA Executed");
                            }
                            6 => {
                                // T6: PR
                                self.t_step = 1;
                                println!("T6 - STA Completed");
                            }
                            _ => {}
                        }
                    }
                    0xA => {
                        // JMP
                        match self.t_step {
                            3 => {
                                // T3: CO, MI
                                self.bus = self.pc;
                                self.mar = self.bus;
                                self.t_step += 1;
                                println!("T3 - JMP");
                            }
                            4 => {
                                // T4: RO, J
                                self.bus = self.memory[self.mar as usize];
                                self.pc = self.bus;
                                self.t_step += 1;
                                println!("T4 - JMP Executed");
                            }
                            5 => {
                                // T5 PR
                                self.t_step = 1;
                                println!("T5 - JMP Completed");
                            }
                            _ => {}
                        }
                    }
                    0xB => {
                        // CMP $
                        match self.t_step {
                            3 => {
                                // T3: CO, MI
                                self.bus = self.pc;
                                self.mar = self.bus;
                                self.t_step += 1;
                                println!("T3 - CMP $");
                            }
                            4 => {
                                // T4: RO, MI
                                self.bus = self.memory[self.mar as usize];
                                self.mar = self.bus;
                                self.t_step += 1;
                                println!("T4 - CMP $ Address Loades");
                            }
                            5 => {
                                // T5: RO, BI, SU
                                self.bus = self.memory[self.mar as usize];
                                self.reg_b = self.bus;
                                let (result, carry) = self.reg_a.overflowing_sub(self.reg_b);
                                self.alu_out = result;
                                self.cf = carry;
                                self.zf = result == 0;
                                self.pc = self.pc.wrapping_add(1);
                                self.t_step += 1;
                                println!("T5 - CMP $ B Loaded");
                            }
                            6 => {
                                // T6: PR
                                self.t_step = 1;
                                println!("T6 - CMP $ Completed");
                            }
                            _ => {}
                        }
                    }
                    0xC => {
                        // CMP #
                        match self.t_step {
                            3 => {
                                // T3; CO, MI
                                self.bus = self.pc;
                                self.mar = self.bus;
                                self.t_step += 1;
                                println!("T3 - CMP #");
                            }
                            4 => {
                                // T4: RO, BI, SU
                                self.bus = self.memory[self.mar as usize];
                                self.reg_b = self.bus;
                                let (result, carry) = self.reg_a.overflowing_sub(self.reg_b);
                                self.alu_out = result;
                                self.cf = carry;
                                self.zf = result == 0;
                                self.pc = self.pc.wrapping_add(1);
                                self.t_step += 1;
                                println!("T4 - CMP # B Loaded");
                            }
                            5 => {
                                // T5: PR
                                self.t_step = 1;
                                println!("T5 - CMP # Completed");
                            }
                            _ => {}
                        }
                    }
                    0xD => {
                        // BNE
                        // TMP Implementation. Actual hardware does not have branching
                        // logic yet.
                        match self.t_step {
                            3 => {
                                // T3: CO, MI
                                self.bus = self.pc;
                                self.mar = self.bus;
                                self.t_step += 1;
                                println!("T3 - BNE");
                            }
                            4 => {
                                // T4: RO, (if ZF=0) J
                                self.bus = self.memory[self.mar as usize];
                                if !self.zf {
                                    self.pc = self.bus;
                                    println!("T4 - BNE Taken, PC set to {}", self.pc);
                                } else {
                                    self.pc = self.pc.wrapping_add(1);
                                    println!("T4 - BNE Not Taken");
                                }
                                self.t_step += 1;
                            }
                            5 => {
                                // T5: PR
                                self.t_step = 1;
                                println!("T5 - BNE Completed");
                            }
                            _ => {}
                        }
                    }
                    0xE => {
                        // JPZ
                        match self.t_step {
                            3 => {
                                // T3: CO, MI
                                self.bus = self.pc;
                                self.mar = self.bus;
                                self.t_step += 1;
                                println!("T3 - JPZ");
                            }
                            4 => {
                                // T4: RO, (IF ZF = 1) J
                                self.bus = self.memory[self.mar as usize];
                                if self.zf {
                                    self.pc = self.bus;
                                    println!("T4 - JPZ Taken, PC set to {}", self.pc);
                                } else {
                                    self.pc = self.pc.wrapping_add(1);
                                    println!("T4 - JPZ Not Taken");
                                }
                                self.t_step += 1;
                            }
                            5 => {
                                // T5: PR
                                self.t_step = 1;
                                println!("T5 - JPZ Completed");
                            }
                            _ => {}
                        }
                    }
                    0xF => {
                        match self.ir & 0x0F {
                            0x0 => {
                                // JPC
                                match self.t_step {
                                    3 => {
                                        // T3: CO, MI
                                        self.bus = self.pc;
                                        self.mar = self.bus;
                                        self.t_step += 1;
                                        println!("T3 - JPC");
                                    }
                                    4 => {
                                        // T4: RO, (if CF=1) J
                                        self.bus = self.memory[self.mar as usize];
                                        if self.cf {
                                            self.pc = self.bus;
                                            println!("T4 - JPC Taken, PC set to {}", self.pc);
                                        } else {
                                            self.pc = self.pc.wrapping_add(1);
                                            println!("T4 - JPC Not Taken");
                                        }
                                        self.t_step += 1;
                                    }
                                    5 => {
                                        // T5: PR
                                        self.t_step = 1;
                                        println!("T5 - JPC Completed");
                                    }
                                    _ => {}
                                }
                            }
                            0x1 => {
                                // INC
                                match self.t_step {
                                    3 => {
                                        // T3: CO, MI
                                        self.bus = self.pc;
                                        self.mar = self.bus;
                                        self.t_step += 1;
                                        println!("T3 - INC");
                                    }
                                    4 => {
                                        // T4: RO, MI, CE
                                        self.bus = self.memory[self.mar as usize];
                                        self.mar = self.bus;
                                        self.pc = self.pc.wrapping_add(1);
                                        self.t_step += 1;
                                        println!("T4 - INC Address Loaded");
                                    }
                                    5 => {
                                        // T5: RO, AI
                                        self.bus = self.memory[self.mar as usize];
                                        self.reg_a = self.bus;
                                        self.reg_b = 1;
                                        self.t_step += 1;
                                        println!("T5 - INC Value Loaded");
                                    }
                                    6 => {
                                        // T6: EO, AI
                                        let (result, carry) =
                                            self.reg_a.overflowing_add(self.reg_b);
                                        self.reg_a = result;
                                        self.cf = carry;
                                        self.zf = result == 0;
                                        self.t_step += 1;
                                        println!("T6 - INC Executed");
                                    }
                                    7 => {
                                        // T7: RI, AO
                                        self.bus = self.reg_a;
                                        self.memory[self.mar as usize] = self.bus;
                                        self.t_step += 1;
                                    }
                                    8 => {
                                        // T8: PR
                                        self.t_step = 1;
                                        println!("T8 - INC Completed");
                                    }
                                    _ => {}
                                }
                            }
                            0x2 => {
                                // DEC
                                match self.t_step {
                                    3 => {
                                        // T3: CO, MI
                                        self.bus = self.pc;
                                        self.mar = self.bus;
                                        self.t_step += 1;
                                        println!("T3 - DEC");
                                    }
                                    4 => {
                                        // T4: RO, MI, CE
                                        self.bus = self.memory[self.mar as usize];
                                        self.mar = self.bus;
                                        self.pc = self.pc.wrapping_add(1);
                                        self.t_step += 1;
                                        println!("T4 - DEC Address Loaded");
                                    }
                                    5 => {
                                        // T5: RO, AI
                                        self.bus = self.memory[self.mar as usize];
                                        self.reg_a = self.bus;
                                        self.reg_b = 1;
                                        self.t_step += 1;
                                        println!("T5 - DEC Value Loaded");
                                    }
                                    6 => {
                                        // T6: EP, AI
                                        let (result, carry) =
                                            self.reg_a.overflowing_sub(self.reg_b);
                                        self.reg_a = result;
                                        self.cf = carry;
                                        self.zf = result == 0;
                                        self.t_step += 1;
                                        println!("T6 - DEC Executed");
                                    }
                                    7 => {
                                        // T7: RI, AO
                                        self.bus = self.reg_a;
                                        self.memory[self.mar as usize] = self.bus;
                                        self.t_step += 1;
                                    }
                                    8 => {
                                        // T8: PR
                                        self.t_step = 1;
                                        println!("T8 - DEC Completed");
                                    }
                                    _ => {}
                                }
                            }
                            0x3 => {
                                // OUT
                                match self.t_step {
                                    3 => {
                                        // AO, OI
                                        self.bus = self.reg_a;
                                        println!("T3 - OUT Executed");
                                        println!("Output: {}", self.bus);
                                        self.t_step += 1;
                                    }
                                    4 => {
                                        // T4 PR
                                        self.t_step = 1;
                                        println!("T4 - Out Completed");
                                    }
                                    _ => {}
                                }
                            }
                            0xF => {
                                // HLT
                                if self.t_step == 3 {
                                    // T3: HLT
                                    println!("T3 - HLT Executed");
                                    self.t_step += 1;
                                } else if self.t_step == 4 {
                                    // T4 PR
                                    println!("T4 - HLT Completed");
                                    self.t_step = 1;
                                    println!("A register: {}", self.reg_a);
                                    self.hlt = true;
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

    // Test program that exercises most instructions

    // Test 1: Store values in memory for later use
    program[0] = 0b00100000; // LDA #
    program[1] = 0b01100100; // Load 100 into A

    program[2] = 0b10010000; // STA
    program[3] = 0b11110000; // Store at address 240

    program[4] = 0b00100000; // LDA #
    program[5] = 0b00110010; // Load 50 into A

    program[6] = 0b10010000; // STA
    program[7] = 0b11110001; // Store at address 241

    // Test 2: ADD $ (add from memory)
    program[8] = 0b00100000; // LDA #
    program[9] = 0b00001010; // Load 10 into A

    program[10] = 0b01010000; // ADD $
    program[11] = 0b11110000; // Add value from address 240 (100)

    program[12] = 0b11110011; // OUT (should output 110)

    // Test 3: SUB $ (subtract from memory)
    program[13] = 0b01110000; // SUB $
    program[14] = 0b11110001; // Subtract value from address 241 (50)

    program[15] = 0b11110011; // OUT (should output 60)

    // Test 4: LDB $ (load B from memory)
    program[16] = 0b00110000; // LDB $
    program[17] = 0b11110001; // Load B from address 241 (50)

    program[18] = 0b01100000; // ADD #
    program[19] = 0b00001010; // Add 10 to A (should be 70)

    program[20] = 0b11110011; // OUT (should output 70, B should be 10 from last operation)

    // Test 5: CMP $ (compare with memory)
    program[21] = 0b00100000; // LDA #
    program[22] = 0b01100100; // Load 100 into A

    program[23] = 0b10110000; // CMP $
    program[24] = 0b11110000; // Compare with address 240 (100)

    program[25] = 0b11100000; // JPZ
    program[26] = 0b00011100; // Jump to 28 if equal (should jump)

    program[27] = 0b11110011; // OUT (should be skipped)

    // Test 6: Test overflow with ADD $
    program[28] = 0b00100000; // LDA #
    program[29] = 0b11111111; // Load 255 into A

    program[30] = 0b10010000; // STA
    program[31] = 0b11110010; // Store at address 242

    program[32] = 0b01010000; // ADD $
    program[33] = 0b11110010; // Add from address 242 (255+255=254 with carry)

    program[34] = 0b11110000; // JPC
    program[35] = 0b00100101; // Jump to 37 if carry set

    program[36] = 0b11110011; // OUT (should be skipped)

    // Test 7: Test underflow with SUB $
    program[37] = 0b00100000; // LDA #
    program[38] = 0b00000101; // Load 5 into A

    program[39] = 0b10010000; // STA
    program[40] = 0b11110011; // Store at address 243

    program[41] = 0b00100000; // LDA #
    program[42] = 0b00000011; // Load 3 into A

    program[43] = 0b01110000; // SUB $
    program[44] = 0b11110011; // Subtract from address 243 (3-5 = underflow)

    program[45] = 0b11110000; // JPC  
    program[46] = 0b00110000; // Jump to 48 if borrow/carry

    program[47] = 0b11110011; // OUT (should be skipped)

    // Test 8: JMP (unconditional jump)
    program[48] = 0b00100000; // LDA #
    program[49] = 0b10101010; // Load 170 into A

    program[50] = 0b10100000; // JMP
    program[51] = 0b00110101; // Jump to address 53

    program[52] = 0b11110011; // OUT (should be skipped)

    // Test 9: NOP (does nothing)
    program[53] = 0b00000000; // NOP
    program[54] = 0b00000000; // NOP
    program[55] = 0b00000000; // NOP

    program[56] = 0b11110011; // OUT (should output 170)

    // Test 10: Final test - multiple operations
    program[57] = 0b00100000; // LDA #
    program[58] = 0b00000000; // Load 0

    program[59] = 0b11000000; // CMP #
    program[60] = 0b00000000; // Compare with 0 (should set ZF)

    program[61] = 0b11010000; // BNE
    program[62] = 0b01000000; // Jump to 64 if not equal (shouldn't jump)

    program[63] = 0b11110011; // OUT (should output 0)

    program[64] = 0b11111111; // HLT

    sap1.load_program(&program);

    println!("Expected outputs: 110, 60, 70, 170, 0");
    println!("Expected final: A=0, CF=true (from underflow), ZF=true");
    println!("\nPress Enter to start...");

    loop {
        if sap1.hlt {
            println!("\n=== Program Halted! ===");
            println!("Final State:");
            println!("A register: {}", sap1.reg_a);
            println!("B register: {}", sap1.reg_b);
            println!("Carry flag: {}", sap1.cf);
            println!("Zero flag: {}", sap1.zf);
            break;
        }

        match sap1.clock_mode {
            ClockMode::STEP => {
                // wait for user to press 's' + Enter
                println!("Press 's' to step, 'r' to run, 'q' to quit: ");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();

                match input.trim() {
                    "s" => sap1.clock_tick(),
                    "r" => sap1.clock_mode = ClockMode::RUN,
                    "q" => break,
                    _ => println!("Unknown command"),
                }
            }
            ClockMode::RUN => {
                sap1.clock_tick();
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }
}
