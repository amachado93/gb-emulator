pub struct Bus {
    pub memory: [u8; 0x10000],

    // serial debug support
    serial_data: u8,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            memory: [0; 0x10000],
            serial_data: 0,
        }
    }

    pub fn read8(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write8(&mut self, addr: u16, value: u8) {
        match addr {
            // serial data register (SB)
            0xFF01 => {
                self.serial_data = value;
            }

            // serial control register (SC)
            0xFF02 => {
                // bit 7 set means "start transfer"
                if value == 0x81 {
                    print!("{}", self.serial_data as char);
                }
            }

            _ => {
                self.memory[addr as usize] = value;
            }
        }
    }
}
