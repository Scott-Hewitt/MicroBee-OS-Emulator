use crate::memory::Memory; // Import the memory module

pub struct CPU {
    pub pc: u16,        // Program counter
    pub acc: u8,        // Accumulator register
    pub reg_a: u8,  // Additional register
    pub reg_b: u8,  // Additional register
    pub memory: Memory, // Memory module
    pub halted: bool,   // Halt flag to stop the CPU
    pub sp: u16,  //  Stack Pointer
    pub interrupts_enabled: bool, // New field to track interrupt state
}

impl CPU {
    /// Create a new instance of the CPU with a specified memory size
    pub fn new(memory_size: usize) -> Self {
        CPU {
            pc: 0,
            acc: 0,
            reg_a: 0,
            reg_b: 0,
            memory: Memory::new(memory_size),
            halted: false,
            sp: 0,
            interrupts_enabled: false, // Interrupts are initially disabled

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
    fn mov(&mut self) {
        self.reg_b = self.reg_a; // Example: Move reg_a's value to reg_b
    }

    fn mul(&mut self) {
        self.acc = self.reg_a.wrapping_mul(self.reg_b); // Handle overflow with wrapping
    }
    fn div(&mut self) -> Result<(), String> {
        if self.reg_b == 0 {
            return Err("Division by zero".to_string());
        }
        self.acc = self.reg_a / self.reg_b;
        Ok(())
    }
    fn cmp(&self) -> Result<(), String> {
        // Perform the comparison, but don't return an `i8`
        Ok(())
    }
    fn call(&mut self) -> Result<(), String> {
        let address = self.fetch_address()?; // Fetch target address (of type u16)
        if self.sp < 2 {                     // Ensure enough stack space
            return Err("Stack overflow - not enough space to push PC".to_string());
        }
        self.sp -= 2;                        // Decrement stack pointer
        self.memory.write_u16(self.sp as usize, self.pc)?; // Push PC onto the stack
        self.pc = address;                   // Set PC to the subroutine address (of type u16)
        Ok(())                               // Return success
    }
    fn ret(&mut self) -> Result<(), String> {
        self.sp += 2;                                // Increment stack pointer
        self.memory.write_u16(self.sp as usize, self.pc)?; // Pop PC
        Ok(())
    }
    fn jp(&mut self, condition: bool) {
        if condition {
            self.pc = self.fetch_address().unwrap();
        }
    }
    fn jn(&mut self, condition: bool) {
        if condition {
            self.pc = self.fetch_address().unwrap();
        }
    }
    fn int(&mut self) -> Result<(), String> {
        let interrupt_vector = self.fetch_address()?; // Safely fetch interrupt vector
        self.memory
            .write_u16(self.sp as usize, self.pc)
            .map_err(|_| "Failed to write to memory".to_string())?; // Handle memory write errors
        self.sp = self.sp.checked_sub(2).ok_or("Stack underflow")?; // Safely decrement SP
        self.pc = interrupt_vector; // Jump to interrupt handler
        Ok(())
    }
    fn cli(&mut self) {
        self.interrupts_enabled = false;
    }

    fn sei(&mut self) {
        self.interrupts_enabled = true;
    }
    fn push(&mut self, value: u8) -> Result<(), String> {
        self.sp = self.sp.checked_sub(1).ok_or("Stack underflow")?; // Safely decrement Stack Pointer
        self.memory
            .write(self.sp as usize, value) // Write value to memory
            .map_err(|e| format!("Failed to push value to memory: {}", e)) // Handle memory write errors
    }
    fn pop(&mut self) -> Result<u8, String> {
        // Implementation of popping data directly, no need to pass a mutable borrow of the target field
        let value = self.memory.read(self.sp as usize)?; // Example logic
        self.sp = self.sp.wrapping_add(1); // Modify the stack pointer
        Ok(value)
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
            0x14 => self.mov(),               // MOV instruction
            0x15 => self.mul(),               // MUL instruction
            0x16 => self.div()?,              // DIV instruction
            0x17 => self.cmp()?,               // CMP instruction
            0x18 => self.call()?,             // CALL instruction
            0x19 => self.ret()?,              // RET instruction
            0x1A => self.jp(true),            // JP (Jump if Positive)
            0x1B => self.jn(true),            // JN (Jump if Negative)
            0x1C => self.int()?,               // INT (Interrupt)
            0x1D => self.cli(),               // CLI (Disable Interrupts)
            0x1E => self.sei(),               // SEI (Enable Interrupts)
            0x1F => self.push(self.reg_a)?,    // PUSH reg_a
            0x20 => {
                let value = self.pop()?;      // First, pop the value from the stack
                self.reg_a = value;           // Then, assign it to reg_a
            }, // POP reg_a


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