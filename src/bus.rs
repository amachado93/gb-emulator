pub struct Bus {
    pub memory: [u8; 0x10000],
}

impl Bus {
    pub fn new() -> Self {
        Self {
            memory: [0; 0x10000],
        }
    }

    pub fn read8(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write8(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }
}
