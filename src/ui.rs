use crate::emulator::Sap1;
use eframe::egui;

pub struct Sap1UI {
    emulator: Sap1,
}

impl Sap1UI {
    pub fn new() -> Self {
        let mut emulator = Sap1::new();

        let program = [
            0b00100000, // LDA #
            0b00001010, // 10
            0b11110011, // OUT
            0b11111111, // HLT
        ];

        emulator.load_program(&program);

        Self { emulator }
    }
}

impl eframe::App for Sap1UI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .default_width(150.0)
            .show(ctx, |ui| {
                ui.heading("Memory & Control");

                // Clock Display
                ui.label("Clock:");
                // TODO: Add Clock visualisation

                ui.separator();

                // Memory Address Register
                ui.horizontal(|ui| {
                    ui.label("Mem. Addr.:");
                    ui.monospace(format!("{:08b} ({})", self.emulator.mar, self.emulator.mar));
                });
                // Add more left side-side components here.
            });
        // Central/Right Area - Registers and ALU
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("R-SAP-1 Emulator");

            // Program Counter
            ui.horizontal(|ui| {
                ui.label("Program Counter:");
                ui.monospace(format!("{:08b} ({})", self.emulator.pc, self.emulator.pc));
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Accumulator:");
                ui.monospace(format!("{:08b}", self.emulator.reg_a));
                ui.label(format!("({})", self.emulator.reg_a));
            });
            ui.horizontal(|ui| {
                ui.label("B Register:");
                ui.monospace(format!("{:08b}", self.emulator.reg_b));
                ui.label(format!("({})", self.emulator.reg_b));
            });

            ui.separator();

            if ui.button("Step").clicked() {
                self.emulator.clock_tick();
            }
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
