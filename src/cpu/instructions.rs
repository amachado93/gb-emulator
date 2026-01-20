#[derive(Debug)]
pub enum Instruction {
    ADD(ArithmeticTarget),
    LDIMM8(Register8),
}

#[derive(Debug, Copy, Clone)]
pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug, Copy, Clone)]
pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub fn decode(opcode: u8) -> Instruction {
    match opcode {
        // ADD A, r
        0x87 => Instruction::ADD(ArithmeticTarget::A),
        0x80 => Instruction::ADD(ArithmeticTarget::B),
        0x81 => Instruction::ADD(ArithmeticTarget::C),
        0x82 => Instruction::ADD(ArithmeticTarget::D),
        0x83 => Instruction::ADD(ArithmeticTarget::E),
        0x84 => Instruction::ADD(ArithmeticTarget::H),
        0x85 => Instruction::ADD(ArithmeticTarget::L),

        // LD r, d8
        0x06 => Instruction::LDIMM8(Register8::B),
        0x0E => Instruction::LDIMM8(Register8::C),
        0x16 => Instruction::LDIMM8(Register8::D),
        0x1E => Instruction::LDIMM8(Register8::E),
        0x26 => Instruction::LDIMM8(Register8::H),
        0x2E => Instruction::LDIMM8(Register8::L),
        0x3E => Instruction::LDIMM8(Register8::A),

        _ => panic!("Unimplemented opcode: 0x{:02X}", opcode),
    }
}
