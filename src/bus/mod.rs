// Emulation for the intel 8080 bus (connects each device together)

pub struct Bus {
    pub memory: Vec<u8>,
}
use std::ops::Range;
impl Bus {
    pub fn new(memory: Vec<u8>) -> Self {
        Bus {
            memory
        }
    }
    
    pub fn read_range(&self, r: Range<usize>) -> &[u8] {
        &self.memory[r]
    }

    pub (crate) fn read(&self, pos: usize) -> u8 {
        self.memory[pos]
    }

    pub (crate) fn write(&mut self, value: u8, pos: usize) {
        self.memory[pos] = value;
    }
}


