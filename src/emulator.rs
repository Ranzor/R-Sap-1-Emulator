pub enum ClockMode {
    RUN,
    STEP,
}
#[allow(non_snake_case)]
#[derive(Debug, Clone, Default)]
pub struct ControlWord {
    pub HLT: bool,
    pub MI: bool,
    pub RI: bool,
    pub RO: bool,
    pub II: bool,
    pub PR: bool,
    pub AI: bool,
    pub AO: bool,
    pub EO: bool,
    pub SU: bool,
    pub BI: bool,
    pub OI: bool,
    pub CE: bool,
    pub CO: bool,
    pub J: bool,
    pub FLG: bool,
}
impl ControlWord {
    pub fn to_array(&self) -> [bool; 16] {
        [
            self.HLT, self.MI, self.RI, self.RO, self.II, self.PR, self.AI, self.AO, self.EO,
            self.SU, self.BI, self.OI, self.CE, self.CO, self.J, self.FLG,
        ]
    }
    pub fn signal_names() -> [&'static str; 16] {
        [
            "HLT", "MI", "RI", "RO", "II", "PR", "AI", "AO", "EO", "SU", "BI", "OI", "CE", "CO",
            "J", "FLG",
        ]
    }
}

pub struct Sap1 {
    // A register
    pub reg_a: u8,
    // B register
    pub reg_b: u8,
    // ALU output
    pub alu_out: u8,
    // Output register
    pub output: u8,
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

    // Control Word
    pub control_word: ControlWord,
}

impl Sap1 {
    pub fn new() -> Self {
        Sap1 {
            reg_a: 0,
            reg_b: 0,
            pc: 0,
            memory: [0; 256],
            cf: false,
            zf: true,
            hlt: false,
            alu_out: 0,
            output: 0,
            t_step: 0,
            bus: 0,
            mar: 0,
            ir: 0,
            clock_mode: ClockMode::STEP,
            control_word: ControlWord::default(),
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        for (i, &byte) in program.iter().enumerate() {
            self.memory[i] = byte;
        }
    }
    pub fn clock_tick(&mut self) {
        let control = self.get_control_word(self.ir, self.t_step);
        self.control_word = control.clone();
        self.t_step += 1;
        self.execute_control_word(&control);
    }

    fn get_control_word(&self, opcode: u8, t_step: u8) -> ControlWord {
        match (opcode >> 4, t_step) {
            (_, 0) => ControlWord {
                CO: true,
                MI: true,
                ..Default::default()
            },
            (_, 1) => ControlWord {
                RO: true,
                II: true,
                CE: true,
                ..Default::default()
            },
            (0x0, 2) => ControlWord {
                PR: true,
                ..Default::default()
            },
            (0x1, 2) => ControlWord {
                CO: true,
                MI: true,
                ..Default::default()
            },
            (0x1, 3) => ControlWord {
                RO: true,
                MI: true,
                ..Default::default()
            },
            (0x1, 4) => ControlWord {
                RO: true,
                AI: true,
                CE: true,
                ..Default::default()
            },
            (0x1, 5) => ControlWord {
                PR: true,
                ..Default::default()
            },
            (0x2, 2) => ControlWord {
                CO: true,
                MI: true,
                ..Default::default()
            },
            (0x2, 3) => ControlWord {
                RO: true,
                AI: true,
                CE: true,
                ..Default::default()
            },
            (0x2, 4) => ControlWord {
                PR: true,
                ..Default::default()
            },
            (0x3, 2) => ControlWord {
                CO: true,
                MI: true,
                ..Default::default()
            },
            (0x3, 3) => ControlWord {
                RO: true,
                MI: true,
                ..Default::default()
            },
            (0x3, 4) => ControlWord {
                RO: true,
                BI: true,
                CE: true,
                ..Default::default()
            },
            (0x3, 5) => ControlWord {
                PR: true,
                ..Default::default()
            },
            (0x4, 2) => ControlWord {
                CO: true,
                MI: true,
                ..Default::default()
            },
            (0x4, 3) => ControlWord {
                RO: true,
                BI: true,
                CE: true,
                ..Default::default()
            },
            (0x4, 4) => ControlWord {
                PR: true,
                ..Default::default()
            },
            (0x5, 2) => ControlWord {
                CO: true,
                MI: true,
                ..Default::default()
            },
            (0x5, 3) => ControlWord {
                RO: true,
                MI: true,
                ..Default::default()
            },
            (0x5, 4) => ControlWord {
                RO: true,
                BI: true,
                ..Default::default()
            },
            (0x5, 5) => ControlWord {
                EO: true,
                AI: true,
                CE: true,
                ..Default::default()
            },
            (0x5, 6) => ControlWord {
                PR: true,
                ..Default::default()
            },
            (0x6, 2) => ControlWord {
                CO: true,
                MI: true,
                ..Default::default()
            },
            (0x6, 3) => ControlWord {
                RO: true,
                BI: true,
                ..Default::default()
            },
            (0x6, 4) => ControlWord {
                EO: true,
                AI: true,
                CE: true,
                ..Default::default()
            },
            (0x6, 5) => ControlWord {
                PR: true,
                ..Default::default()
            },
            (0x7, 2) => ControlWord {
                CO: true,
                MI: true,
                ..Default::default()
            },
            (0x7, 3) => ControlWord {
                RO: true,
                MI: true,
                ..Default::default()
            },
            (0x7, 4) => ControlWord {
                RO: true,
                BI: true,
                SU: true,
                ..Default::default()
            },
            (0x7, 5) => ControlWord {
                EO: true,
                AI: true,
                SU: true,
                CE: true,
                ..Default::default()
            },
            (0x7, 6) => ControlWord {
                PR: true,
                ..Default::default()
            },
            (0x8, 2) => ControlWord {
                CO: true,
                MI: true,
                ..Default::default()
            },
            (0x8, 3) => ControlWord {
                RO: true,
                BI: true,
                SU: true,
                ..Default::default()
            },
            (0x8, 4) => ControlWord {
                EO: true,
                AI: true,
                SU: true,
                CE: true,
                ..Default::default()
            },
            (0x8, 5) => ControlWord {
                PR: true,
                ..Default::default()
            },
            (0x9, 2) => ControlWord {
                CO: true,
                MI: true,
                ..Default::default()
            },
            (0x9, 3) => ControlWord {
                RI: true,
                MI: true,
                CE: true,
                ..Default::default()
            },
            (0x9, 4) => ControlWord {
                AO: true,
                RI: true,
                ..Default::default()
            },
            (0x9, 5) => ControlWord {
                PR: true,
                ..Default::default()
            },
            (0xA, 2) => ControlWord {
                CO: true,
                MI: true,
                ..Default::default()
            },
            (0xA, 3) => ControlWord {
                RO: true,
                J: true,
                ..Default::default()
            },
            (0xA, 4) => ControlWord {
                PR: true,
                ..Default::default()
            },
            (0xB, 2) => ControlWord {
                CO: true,
                MI: true,
                ..Default::default()
            },
            (0xB, 3) => ControlWord {
                RO: true,
                MI: true,
                ..Default::default()
            },
            (0xB, 4) => ControlWord {
                RO: true,
                BI: true,
                SU: true,
                ..Default::default()
            },
            (0xB, 5) => ControlWord {
                PR: true,
                ..Default::default()
            },
            (0xC, 2) => ControlWord {
                CO: true,
                MI: true,
                ..Default::default()
            },
            (0xC, 3) => ControlWord {
                RO: true,
                BI: true,
                SU: true,
                ..Default::default()
            },
            (0xC, 4) => ControlWord {
                PR: true,
                ..Default::default()
            },
            (0xD, 2) => ControlWord {
                CO: true,
                MI: true,
                ..Default::default()
            },
            (0xD, 3) => ControlWord {
                FLG: !self.zf,
                RO: true,
                J: true,
                ..Default::default()
            },
            (0xD, 4) => ControlWord {
                PR: true,
                ..Default::default()
            },
            (0xE, 2) => ControlWord {
                CO: true,
                MI: true,
                ..Default::default()
            },
            (0xE, 3) => ControlWord {
                FLG: self.zf,
                RO: true,
                J: true,
                ..Default::default()
            },
            (0xE, 4) => ControlWord {
                PR: true,
                ..Default::default()
            },
            (0xF, _) => match (opcode & 0x0F, t_step) {
                (0x0, 2) => ControlWord {
                    CO: true,
                    MI: true,
                    ..Default::default()
                },
                (0x0, 3) => ControlWord {
                    FLG: self.cf,
                    RO: true,
                    J: true,
                    ..Default::default()
                },
                (0x0, 4) => ControlWord {
                    PR: true,
                    ..Default::default()
                },
                (0x1, 2) => ControlWord {
                    CO: true,
                    MI: true,
                    ..Default::default()
                },
                (0x1, 3) => ControlWord {
                    RO: true,
                    MI: true,
                    CE: true,
                    ..Default::default()
                },
                (0x1, 4) => ControlWord {
                    RO: true,
                    AI: true,
                    // reg_b = 1
                    ..Default::default()
                },
                (0x1, 5) => ControlWord {
                    EO: true,
                    AI: true,
                    ..Default::default()
                },
                (0x1, 6) => ControlWord {
                    RI: true,
                    AO: true,
                    ..Default::default()
                },
                (0x1, 7) => ControlWord {
                    PR: true,
                    ..Default::default()
                },
                (0x2, 2) => ControlWord {
                    CO: true,
                    MI: true,
                    ..Default::default()
                },
                (0x2, 3) => ControlWord {
                    RO: true,
                    MI: true,
                    CE: true,
                    ..Default::default()
                },
                (0x2, 4) => ControlWord {
                    RO: true,
                    AI: true,
                    // reg_b = 1
                    ..Default::default()
                },
                (0x2, 5) => ControlWord {
                    EO: true,
                    AI: true,
                    SU: true,
                    ..Default::default()
                },
                (0x2, 6) => ControlWord {
                    RI: true,
                    AO: true,
                    ..Default::default()
                },
                (0x2, 7) => ControlWord {
                    PR: true,
                    ..Default::default()
                },
                (0x3, 2) => ControlWord {
                    AO: true,
                    OI: true,
                    ..Default::default()
                },
                (0x3, 3) => ControlWord {
                    PR: true,
                    ..Default::default()
                },
                (0xF, 2) => ControlWord {
                    HLT: true,
                    ..Default::default()
                },
                (0xF, 3) => ControlWord {
                    PR: true,
                    ..Default::default()
                },
                _ => ControlWord::default(),
            },

            _ => ControlWord::default(),
        }
    }
    fn execute_control_word(&mut self, control: &ControlWord) {
        self.alu_out = if control.SU {
            let (result, borrow) = self.reg_a.overflowing_sub(self.reg_b);
            self.cf = borrow;
            self.zf = result == 0;
            result
        } else {
            let (result, carry) = self.reg_a.overflowing_add(self.reg_b);
            self.cf = carry;
            self.zf = result == 0;
            result
        };

        if control.CO {
            self.bus = self.pc;
        }
        if control.RO {
            self.bus = self.memory[self.mar as usize];
        }
        if control.AO {
            self.bus = self.reg_a;
        }
        if control.EO {
            self.bus = self.alu_out;
        }

        if control.MI {
            self.mar = self.bus;
        }
        if control.RI {
            self.memory[self.mar as usize] = self.bus;
        }
        if control.II {
            self.ir = self.bus;
        }
        if control.AI {
            self.reg_a = self.bus;
        }
        if control.BI {
            self.reg_b = self.bus;
        }
        if control.OI {
            self.output = self.bus;
        }

        if control.CE {
            self.pc = self.pc.wrapping_add(1);
        }
        if control.J {
            if control.FLG {
                self.pc = self.bus;
            } else {
                self.pc = self.pc.wrapping_add(1);
            }
        }
        if control.PR {
            self.t_step = 0;
        }
        if control.HLT {
            self.hlt = true;
        }

        if self.alu_out == 0 {
            self.zf = true;
        } else {
            self.zf = false;
        }
    }
}
