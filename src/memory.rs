// src/memory.rs

pub struct Memory {
    pub(crate) data: Vec<u8>, // Memory stored as a vector of bytes
}
impl Memory {
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.data.len()
    }
}
impl Memory {
    /// Create a new memory space with a given size (default 64KB)
    pub fn new(size: usize) -> Self {
        Memory {
            data: vec![0; size],
        }
    }

    /// Read a byte from a certain address
    ///
    ///
    pub fn read(&self, address: usize) -> Result<u8, String> {
        if address >= self.data.len() {
            Err(format!("Memory read out of bounds at address: {:04X}", address))
        } else {
            Ok(self.data[address])
        }
    }

    pub fn write(&mut self, address: usize, value: u8) -> Result<(), String> {
        if address >= self.data.len() {
            Err(format!("Memory write out of bounds at address: {:04X}", address))
        } else {
            self.data[address] = value;
            Ok(())
        }
    }

}