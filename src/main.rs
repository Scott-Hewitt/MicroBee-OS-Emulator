mod memory; // Import memory.rs as a module
mod cpu;    // Import cpu.rs as a module

use cpu::CPU; // Bring CPU into scope

fn main() -> Result<(), String> {
    let mut cpu = CPU::new(64 * 1024); // CPU with 64KB of memory

    // Program: Load 10, increment it, and store the result
    cpu.memory.write(0, 0x13)?; // LDA (Load direct value into accumulator)
    cpu.memory.write(1, 10)?;   // Value to load: 10
    cpu.memory.write(2, 0x07)?; // INC
    cpu.memory.write(3, 0x02)?; // STORE
    cpu.memory.write(4, 0x20)?; // Address low byte
    cpu.memory.write(5, 0x00)?; // Address high byte
    cpu.memory.write(6, 0xFF)?; // HALT

    // Run the CPU
    cpu.run();

    // Verify the result in memory
    match cpu.memory.read(0x0020) {
        Ok(value) => println!("Result: {}", value),
        Err(err) => println!("Error: {}", err),
    }

    Ok(())
}