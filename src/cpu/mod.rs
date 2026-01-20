pub mod execute;
pub mod instructions;
pub mod registers;

use registers::Registers;

pub struct Cpu {
    pub regs: Registers,
    pub halted: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            regs: Registers::default(),
            halted: false,
        }
    }

    pub fn reset(&mut self) {
        self.regs = Registers {
            pc: 0x0100,
            sp: 0xFFFE,
            ..Default::default()
        };
        self.halted = false;
    }
}
