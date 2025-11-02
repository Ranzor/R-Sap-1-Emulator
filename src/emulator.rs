pub enum ClockMode {
    RUN,
    STEP,
}

pub struct Sap1 {
    // A register
    pub reg_a: u8,
    // B register
    pub reg_b: u8,
    // ALU output
    pub alu_out: u8,
    // Program counter
    pub pc: u8,

    // Memory (256 bytes)
    pub memory: [u8; 256],

    // Flags: Carry and Zero
    pub cf: bool,
    pub zf: bool,

    // Halt flag
    pub hlt: bool,
    // Current execution step
    pub t_step: u8,

    // Hardware components for visualization
    // Bus
    pub bus: u8,
    // Memory Address Register
    pub mar: u8,
    // Instruction Register
    pub ir: u8,

    // Clock mode
    pub clock_mode: ClockMode,
}

impl Sap1 {
    pub fn new() -> Self {
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

    pub fn load_program(&mut self, program: &[u8]) {
        for (i, &byte) in program.iter().enumerate() {
            self.memory[i] = byte;
        }
    }

    pub fn clock_tick(&mut self) {
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
