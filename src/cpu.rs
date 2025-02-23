use crate::memory::Memory; // Import the memory module

pub struct CPU {
    pub pc: u16,        // Program counter
    pub acc: u8,        // Accumulator register
    pub memory: Memory, // Memory module
    pub halted: bool,   // Halt flag to stop the CPU
}

impl CPU {
    /// Create a new instance of the CPU with a specified memory size
    pub fn new(memory_size: usize) -> Self {
        CPU {
            pc: 0,
            acc: 0,
            memory: Memory::new(memory_size),
            halted: false,
        }
    }

    /// Fetch a single byte of instruction data from memory
    pub fn fetch(&mut self) -> Result<u8, String> {
        let instruction = self.memory.read(self.pc as usize)?;
        self.pc = self.pc.wrapping_add(1); // Increment the program counter (with wrapping)
        Ok(instruction)
    }

    /// Fetch a 16-bit address from memory (two bytes in little-endian format)
    fn fetch_address(&mut self) -> Result<u16, String> {
        let low_byte = self.fetch()? as u16;
        let high_byte = self.fetch()? as u16;
        Ok((high_byte << 8) | low_byte) // Combine high and low bytes
    }

    /// Execute the given instruction based on its opcode
    pub fn execute(&mut self, instruction: u8) -> Result<(), String> {
        match instruction {
            // LOAD: Load a value from memory into the accumulator
            0x01 => {
                let address = self.fetch_address()?;
                self.acc = self.memory.read(address as usize)?;
            }

            // STORE: Store the accumulator value into a memory address
            0x02 => {
                let address = self.fetch_address()?;
                self.memory.write(address as usize, self.acc)?;
            }

            // ADD: Add a value from memory to the accumulator
            0x03 => {
                let address = self.fetch_address()?;
                let value = self.memory.read(address as usize)?;
                self.acc = self.acc.wrapping_add(value);
            }

            // SUB: Subtract a value from memory from the accumulator
            0x04 => {
                let address = self.fetch_address()?;
                let value = self.memory.read(address as usize)?;
                self.acc = self.acc.wrapping_sub(value);
            }

            // INC: Increment the accumulator by 1
            0x07 => {
                self.acc = self.acc.wrapping_add(1);
            }

            // DEC: Decrement the accumulator by 1
            0x08 => {
                self.acc = self.acc.wrapping_sub(1);
            }

            // AND: Logical AND between the accumulator and a memory value
            0x09 => {
                let address = self.fetch_address()?;
                let value = self.memory.read(address as usize)?;
                self.acc &= value;
            }

            // OR: Logical OR between the accumulator and a memory value
            0x0A => {
                let address = self.fetch_address()?;
                let value = self.memory.read(address as usize)?;
                self.acc |= value;
            }

            // XOR: Logical XOR between the accumulator and a memory value
            0x0B => {
                let address = self.fetch_address()?;
                let value = self.memory.read(address as usize)?;
                self.acc ^= value;
            }

            // JMP: Jump to the specified memory address
            0x10 => {
                let address = self.fetch_address()?;
                self.pc = address;
            }

            // JZ: Jump to an address if the accumulator is zero
            0x11 => {
                let address = self.fetch_address()?;
                if self.acc == 0 {
                    self.pc = address;
                }
            }

            // JNZ: Jump to an address if the accumulator is not zero
            0x12 => {
                let address = self.fetch_address()?;
                if self.acc != 0 {
                    self.pc = address;
                }
            }

            // LDA: Load a value directly into the accumulator
            0x13 => {
                self.acc = self.fetch()?;
            }

            // HALT: Stop the CPU
            0xFF => {
                self.halted = true;
            }

            // Handle unknown instructions
            _ => {
                return Err(format!("Unknown instruction: 0x{:02X} at PC: 0x{:04X}", instruction, self.pc));
            }
        }
        Ok(())
    }

    /// Run the CPU loop until the `halted` state is true
    pub fn run(&mut self) {
        while !self.halted {
            match self.fetch() {
                Ok(instruction) => {
                    if let Err(err) = self.execute(instruction) {
                        println!("Execution error: {}", err);
                        self.halted = true; // Stop the CPU on error
                    }
                }
                Err(err) => {
                    println!("Fetch error: {}", err);
                    self.halted = true; // Stop the CPU on error
                }
            }
        }
    }

    /// Debugging tool to print a chunk of memory content (hex values)
    #[allow(dead_code)]

    pub fn print_memory(&self, start: usize, count: usize) {
        for i in start..(start + count) {
            match self.memory.read(i) {
                Ok(value) => print!("{:02X} ", value),
                Err(err) => {
                    println!("Failed to read memory at 0x{:04X}: {}", i, err);
                    break;
                }
            }
            if (i - start + 1) % 16 == 0 {
                println!(); // Newline after 16 bytes
            }
        }
        println!(); // Final newline
    }
}