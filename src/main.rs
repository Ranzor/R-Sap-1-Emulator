mod emulator;
mod ui;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--no-gui" {
        terminal_mode();
    } else {
        ui::run();
    }
}

fn terminal_mode() {
    use emulator::{ClockMode, Sap1};

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
    program[7] = 0b11110011; // Store at address 241

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
