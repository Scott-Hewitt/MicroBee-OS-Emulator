pub struct Memory {
    pub(crate) data: Vec<u8>, // Memory stored as a vector of bytes
}

impl Memory {
    /// Create a new memory space with a given size (default 64KB)
    pub fn new(size: usize) -> Self {
        Memory {
            data: vec![0; size],
        }
    }

    /// Return the size of the memory
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Read a byte from a certain address
    pub fn read(&self, address: usize) -> Result<u8, String> {
        if address >= self.data.len() {
            Err(format!("Memory read out of bounds at address: {:04X}", address))
        } else {
            Ok(self.data[address])
        }
    }

    /// Write a byte to a certain address
    pub fn write(&mut self, address: usize, value: u8) -> Result<(), String> {
        if address >= self.data.len() {
            Err(format!("Memory write out of bounds at address: {:04X}", address))
        } else {
            self.data[address] = value;
            Ok(())
        }
    }

    /// Read a 16-bit value (2 bytes) from a certain address
    pub fn read_u16(&self, address: usize) -> Result<u16, String> {
        if address + 1 >= self.data.len() {
            Err(format!("Memory read_u16 out of bounds at address: {:04X}", address))
        } else {
            let low = self.data[address] as u16; // Read the lower 8 bits
            let high = self.data[address + 1] as u16; // Read the upper 8 bits
            Ok((high << 8) | low) // Combine to form a 16-bit value
        }
    }

    /// Write a 16-bit value (2 bytes) to a certain address
    pub fn write_u16(&mut self, address: usize, value: u16) -> Result<(), String> {
        if address + 1 >= self.data.len() {
            Err(format!("Memory write_u16 out of bounds at address: {:04X}", address))
        } else {
            let low = (value & 0x00FF) as u8; // Extract the lower 8 bits
            let high = ((value >> 8) & 0x00FF) as u8; // Extract the upper 8 bits
            self.data[address] = low; // Write the lower byte
            self.data[address + 1] = high; // Write the upper byte
            Ok(())
        }
    }
}