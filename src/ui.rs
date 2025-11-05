use crate::emulator::Sap1;
use eframe::egui;

pub struct Sap1UI {
    emulator: Sap1,
}

impl Sap1UI {
    pub fn new() -> Self {
        let mut emulator = Sap1::new();

        let program = [
            // Test 1: Basic arithmetic and JNZ
            0x20, 5, // 00-01: LDA # 5        (Load 5 into A)
            0x80, 5, // 02-03: SUB # 5        (Subtract 5, A = 0, ZF = 1)
            0xD0, 10, // 04-05: JNZ 10         (Should NOT jump, ZF = 1)
            0x20, 99,   // 06-07: LDA # 99       (This should execute)
            0xF3, // 08:    OUT            (Output 99)
            0xA0, 14, // 09-10: JMP 14         (Jump to next test)
            // If JNZ incorrectly jumped here:
            0x20, 255,  // 11-12: LDA # 255      (This should NOT execute)
            0xF3, // 13:    OUT            (Should not output 255)
            // Test 2: JPZ (jump if zero)
            0x20, 10, // 14-15: LDA # 10       (Load 10, ZF = 0)
            0xE0, 24, // 16-17: JPZ 24         (Should NOT jump, ZF = 0)
            0x20, 0, // 18-19: LDA # 0        (Load 0, ZF = 1)
            0xE0, 26, // 20-21: JPZ 26         (Should jump, ZF = 1)
            0x20, 111,  // 22-23: LDA # 111      (Should NOT execute)
            0xF3, // 24:    OUT            (Should NOT output 111)
            0xFF, // 25:    HLT            (Should NOT halt here)
            // Test 3: Successful jump lands here
            0x20, 42,   // 26-27: LDA # 42       (This should execute)
            0xF3, // 28:    OUT            (Output 42)
            // Test 4: Simple addition
            0x20, 10, // 29-30: LDA # 10
            0x60, 5,    // 31-32: ADD # 5        (A = 15)
            0xF3, // 33:    OUT            (Output 15)
            0xFF, // 34:    HLT            (End)
        ];

        emulator.load_program(&program);

        Self { emulator }
    }
}

enum LedColor {
    Data,    // Red
    Control, // Blue
    Address, // Yellow
    Program, // Green
}

impl LedColor {
    fn to_color32(&self) -> egui::Color32 {
        match self {
            LedColor::Data => egui::Color32::from_rgb(220, 50, 50),
            LedColor::Control => egui::Color32::from_rgb(50, 100, 220),
            LedColor::Address => egui::Color32::from_rgb(220, 180, 50),
            LedColor::Program => egui::Color32::from_rgb(50, 220, 100),
        }
    }
}

impl eframe::App for Sap1UI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(150.0)
            .show(ctx, |ui| {
                ui.heading("Memory & Control");

                egui::Frame::NONE
                    .fill(egui::Color32::from_gray(40))
                    .stroke(egui::Stroke::new(2.0, egui::Color32::from_gray(100)))
                    .inner_margin(8.0)
                    .outer_margin(4.0)
                    .show(ui, |ui| {
                        // Clock Display
                        ui.set_min_width(ui.available_width());
                        ui.horizontal(|ui| {
                            ui.label("Clock:");
                            if ui.button("Step").clicked() && !self.emulator.running {
                                if !self.emulator.hlt {
                                    self.emulator.clock_tick();
                                }
                            }
                            if ui.button("Run").clicked() {
                                self.emulator.running = !self.emulator.running;
                            }
                        });
                    });

                ui.separator();

                egui::Frame::NONE
                    .fill(egui::Color32::from_gray(40))
                    .stroke(egui::Stroke::new(2.0, egui::Color32::from_gray(100)))
                    .inner_margin(8.0)
                    .outer_margin(4.0)
                    .show(ui, |ui| {
                        // Memory Address Register
                        ui.set_min_width(ui.available_width());
                        ui.horizontal(|ui| {
                            ui.label("Memory Address:");
                            draw_byte_leds(ui, self.emulator.mar, LedColor::Address, 8);
                            ui.label(format!("({})", self.emulator.mar));
                        });
                    });

                ui.separator();
                egui::Frame::NONE
                    .fill(egui::Color32::from_gray(40))
                    .stroke(egui::Stroke::new(2.0, egui::Color32::from_gray(100)))
                    .inner_margin(8.0)
                    .outer_margin(4.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // RAM
                            ui.set_min_width(ui.available_width());
                            ui.label("RAM:");
                            draw_byte_leds(
                                ui,
                                self.emulator.memory[self.emulator.mar as usize],
                                LedColor::Data,
                                8,
                            );
                            ui.label(format!(
                                "({})",
                                self.emulator.memory[self.emulator.mar as usize]
                            ));
                        });
                    });
                ui.separator();

                egui::Frame::NONE
                    .fill(egui::Color32::from_gray(40))
                    .stroke(egui::Stroke::new(2.0, egui::Color32::from_gray(100)))
                    .inner_margin(8.0)
                    .outer_margin(4.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // Instruction Register
                            ui.set_min_width(ui.available_width());
                            ui.horizontal(|ui| {
                                ui.label("Instruction Register:");
                                draw_byte_leds(ui, self.emulator.ir, LedColor::Control, 8);
                                ui.label(format!("({})", self.emulator.ir));
                            });
                        });
                    });

                ui.separator();

                egui::Frame::NONE
                    .fill(egui::Color32::from_gray(40))
                    .stroke(egui::Stroke::new(2.0, egui::Color32::from_gray(100)))
                    .inner_margin(8.0)
                    .outer_margin(4.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // Instruction Register
                            ui.set_min_width(ui.available_width());
                            ui.horizontal(|ui| {
                                ui.label("Micro Step:");
                                draw_byte_leds(ui, self.emulator.t_step, LedColor::Address, 3);
                                let decoded = decode_t_step(self.emulator.t_step);
                                draw_byte_leds(ui, decoded, LedColor::Program, 7);
                            });
                        });
                        ui.separator();
                        ui.horizontal(|ui| {
                            ui.label("ROM Address:");
                            draw_byte_leds(ui, self.emulator.t_step, LedColor::Address, 3);
                            draw_byte_leds(ui, self.emulator.ir, LedColor::Address, 8);
                        });
                    });

                egui::Frame::NONE
                    .fill(egui::Color32::from_gray(40))
                    .stroke(egui::Stroke::new(2.0, egui::Color32::from_gray(100)))
                    .inner_margin(8.0)
                    .outer_margin(4.0)
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .max_height(350.0)
                            .show(ui, |ui| {
                                ui.set_min_width(ui.available_width());
                                let mut addr = 0;
                                let mut skip_next = false;

                                while addr < self.emulator.memory.len() {
                                    let is_current = addr == self.emulator.mar as usize;
                                    let arrow = if is_current { "->" } else { "  " };
                                    let color = if is_current {
                                        egui::Color32::from_rgb(220, 180, 50)
                                    } else {
                                        egui::Color32::GRAY
                                    };
                                    if !skip_next {
                                        let (mnemonic, is_two_byte) =
                                            dissasemble_byte(&self.emulator.memory, addr);
                                        ui.horizontal(|ui| {
                                            ui.colored_label(color, arrow);
                                            ui.colored_label(color, format!("{:03}: ", addr));
                                            ui.colored_label(
                                                color,
                                                format!("{:08b}", self.emulator.memory[addr]),
                                            );
                                            ui.colored_label(color, mnemonic);
                                        });
                                        if is_two_byte {
                                            skip_next = true;
                                        }
                                    } else {
                                        ui.horizontal(|ui| {
                                            ui.colored_label(color, arrow);
                                            ui.colored_label(color, format!("{:03}: ", addr));
                                            ui.colored_label(
                                                color,
                                                format!("{:08b}", self.emulator.memory[addr]),
                                            );
                                            ui.colored_label(
                                                color,
                                                format!("{}", self.emulator.memory[addr]),
                                            );

                                            skip_next = false;
                                        });
                                    }
                                    addr += 1;
                                }
                            });
                    });
            });

        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(250.0)
            .show(ctx, |ui| {
                ui.heading("Registers & ALU");

                egui::Frame::NONE
                    .fill(egui::Color32::from_gray(40))
                    .stroke(egui::Stroke::new(2.0, egui::Color32::from_gray(100)))
                    .inner_margin(8.0)
                    .outer_margin(4.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // Program Counter
                            ui.set_min_width(ui.available_width());
                            ui.horizontal(|ui| {
                                ui.label("Program Counter:");
                                draw_byte_leds(ui, self.emulator.pc, LedColor::Program, 8);
                                ui.label(format!("({})", self.emulator.pc));
                            });
                        });
                    });

                ui.separator();

                egui::Frame::NONE
                    .fill(egui::Color32::from_gray(40))
                    .stroke(egui::Stroke::new(2.0, egui::Color32::from_gray(100)))
                    .inner_margin(8.0)
                    .outer_margin(4.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // Accumulator
                            ui.set_min_width(ui.available_width());
                            ui.horizontal(|ui| {
                                ui.label("Accumulator:");
                                draw_byte_leds(ui, self.emulator.reg_a, LedColor::Data, 8);
                                ui.label(format!("({})", self.emulator.reg_a));
                            });
                        });
                    });

                ui.separator();

                egui::Frame::NONE
                    .fill(egui::Color32::from_gray(40))
                    .stroke(egui::Stroke::new(2.0, egui::Color32::from_gray(100)))
                    .inner_margin(8.0)
                    .outer_margin(4.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.horizontal(|ui| {
                                ui.label("ALU:");
                                // ALU Output
                                draw_byte_leds(ui, self.emulator.alu_out, LedColor::Data, 8);
                                ui.label(format!("({})", self.emulator.alu_out));
                                ui.label("Flags:");
                                ui.label("Z:");
                                draw_led_bit(
                                    ui,
                                    if self.emulator.zf { true } else { false },
                                    LedColor::Control.to_color32(),
                                );
                                ui.label("C:");
                                draw_led_bit(
                                    ui,
                                    if self.emulator.cf { true } else { false },
                                    LedColor::Control.to_color32(),
                                );
                            });
                        });
                    });

                ui.separator();

                egui::Frame::NONE
                    .fill(egui::Color32::from_gray(40))
                    .stroke(egui::Stroke::new(2.0, egui::Color32::from_gray(100)))
                    .inner_margin(8.0)
                    .outer_margin(4.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // B Register
                            ui.set_min_width(ui.available_width());
                            ui.horizontal(|ui| {
                                ui.label("B Register:");
                                draw_byte_leds(ui, self.emulator.reg_b, LedColor::Data, 8);
                                ui.label(format!("({})", self.emulator.reg_b));
                            });
                        });
                    });

                ui.separator();

                egui::Frame::NONE
                    .fill(egui::Color32::from_gray(40))
                    .stroke(egui::Stroke::new(2.0, egui::Color32::from_gray(100)))
                    .inner_margin(8.0)
                    .outer_margin(4.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // Output Register
                            ui.set_min_width(ui.available_width());
                            ui.horizontal(|ui| {
                                ui.label("Output:");
                                ui.label(format!("{:04}", self.emulator.output));
                            });
                        });
                    });
                ui.separator();

                egui::Frame::NONE
                    .fill(egui::Color32::from_gray(40))
                    .stroke(egui::Stroke::new(2.0, egui::Color32::from_gray(100)))
                    .inner_margin(8.0)
                    .outer_margin(4.0)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            // Control Word
                            ui.set_min_width(ui.available_width());

                            ui.label("Control Word:");
                            ui.horizontal(|ui| {
                                let signals = self.emulator.control_word.to_array();
                                let names = crate::emulator::ControlWord::signal_names();

                                for (_i, (&bit, &name)) in
                                    signals.iter().zip(names.iter()).enumerate()
                                {
                                    ui.vertical(|ui| {
                                        draw_led_bit(ui, bit, LedColor::Control.to_color32());
                                        ui.label(name);
                                    });
                                }
                            });
                        });
                    });
            });
        // Central/Right Area - Registers and ALU
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("R-SAP-1 Emulator");

            ui.heading("Bus");
            // Bus display area
            draw_byte_leds(ui, self.emulator.bus, LedColor::Address, 8);
        });
    }
}
pub fn run() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "R-SAP-1 Emulator",
        options,
        Box::new(|_cc| Ok(Box::new(Sap1UI::new()))),
    );
}

fn draw_led_bit(ui: &mut egui::Ui, bit: bool, color: egui::Color32) -> egui::Response {
    let size = egui::vec2(16.0, 16.0);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::hover());

    if ui.is_rect_visible(rect) {
        let _visuals = ui.style().noninteractive();

        let fill_color = if bit {
            color
        } else {
            egui::Color32::from_gray(30)
        };
        ui.painter().circle_filled(rect.center(), 6.0, fill_color);
    }
    response
}

fn draw_byte_leds(ui: &mut egui::Ui, value: u8, color: LedColor, num_bits: usize) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        for i in (0..num_bits).rev() {
            let bit = (value >> i) & 1 == 1;
            draw_led_bit(ui, bit, color.to_color32());
        }
    });
}

fn dissasemble_byte(memory: &[u8], address: usize) -> (String, bool) {
    let byte = memory[address];
    let opcode = byte >> 4;

    match opcode {
        0x0 => ("NOP".to_string(), false),
        0x1 => ("LDA $".to_string(), true),
        0x2 => ("LDA #".to_string(), true),
        0x3 => ("LDB $".to_string(), true),
        0x4 => ("LDB #".to_string(), true),
        0x5 => ("ADD $".to_string(), true),
        0x6 => ("ADD #".to_string(), true),
        0x7 => ("SUB $".to_string(), true),
        0x8 => ("SUB #".to_string(), true),
        0x9 => ("STA".to_string(), true),
        0xA => ("JMP".to_string(), true),
        0xB => ("CMP $".to_string(), true),
        0xC => ("CMP #".to_string(), true),
        0xD => ("BNE".to_string(), true),
        0xE => ("JPZ".to_string(), true),
        0xF => match byte {
            0xF0 => ("JPC".to_string(), true),
            0xF1 => ("INC".to_string(), true),
            0xF2 => ("DEC".to_string(), true),
            0xF3 => ("OUT".to_string(), false),
            0xFF => ("HLT".to_string(), false),
            _ => ("???".to_string(), false),
        },

        _ => ("???".to_string(), false),
    }
}
fn decode_t_step(t_step: u8) -> u8 {
    if t_step == 0 {
        0b00000000
    } else if t_step <= 7 {
        1 << (7 - t_step)
    } else {
        0
    }
}
