use core::panic;

#[derive(Debug)]
pub enum Instruction {
    NOP,
    LDBCD16,  // LD BC, d16
    LDBCA,    // LD (BC), A
    INCBC,    // INC BC
    JRNZR8,   // JR NZ, r8
    INCHL,    // INC HL
    JRZR8,    // JR Z, r8
    LDAHLINC, // LD A, (HL+)
    ADD(ArithmeticTarget),
    ORC, // OR C
    LDIMM8(Register8),
    LDA8A,     // LD (a8), A
    LDA16A,    // LD (a16), A
    LDAA16,    // LD A, (a16)
    LDHLD16,   // LD HL, d16
    JRR8,      // JR r8
    JPA16,     // JP a16
    CALLNZA16, // CALL NZ, a16
    POPBC,
    CALLA16, // CALL a16
    RET,
    LDRegReg(Operand8, Operand8),
    CP(Operand8),
    DI,
    LDSPD16,
    PUSHBC,
    POPHL,
    PUSHHL,
    ANDD8,
    LDHAA8,
    POPAF,
    PUSHAF,
    CPD8,
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

#[derive(Debug, Copy, Clone)]
pub enum Operand8 {
    Reg(Register8),
    IndHL,
}

pub fn decode(opcode: u8) -> Instruction {
    match opcode {
        0x00 => Instruction::NOP,
        0x01 => Instruction::LDBCD16,
        0x02 => Instruction::LDBCA,
        0x03 => Instruction::INCBC,

        0x20 => Instruction::JRNZR8,
        0x21 => Instruction::LDHLD16,
        0x23 => Instruction::INCHL,
        0x28 => Instruction::JRZR8,
        0x2A => Instruction::LDAHLINC,

        0xB1 => Instruction::ORC,
        // CP r (0xB8-0xBF)
        0xB8..=0xBF => {
            let reg = decode_reg(opcode & 0b111);
            Instruction::CP(reg)
        }

        // LD SP, d16
        0x31 => Instruction::LDSPD16,

        // LD r, r (0x40-0x7F, except 0x76)
        0x40..=0x7F if opcode != 0x76 => {
            let dst = decode_reg((opcode >> 3) & 0b111);
            let src = decode_reg(opcode & 0b111);
            Instruction::LDRegReg(dst, src)
        }

        // HALT
        0x76 => panic!("HALT not implemented here"),

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

        // LD (a8), A
        0xE0 => Instruction::LDA8A,

        // LD (a16), A
        0xEA => Instruction::LDA16A,

        // LD A, (a16)
        0xFA => Instruction::LDAA16,

        // JR r8
        0x18 => Instruction::JRR8,

        0xC1 => Instruction::POPBC,
        0xC3 => Instruction::JPA16,
        0xC4 => Instruction::CALLNZA16,
        0xC5 => Instruction::PUSHBC,

        // CALLA16
        0xCD => Instruction::CALLA16,

        0xC9 => Instruction::RET,

        // POP HL
        0xE1 => Instruction::POPHL,

        // PUSH HL
        0xE5 => Instruction::PUSHHL,
        0xE6 => Instruction::ANDD8,

        0xF0 => Instruction::LDHAA8,
        0xF1 => Instruction::POPAF,
        0xF3 => Instruction::DI,
        0xF5 => Instruction::PUSHAF,
        0xFE => Instruction::CPD8,

        _ => panic!("Unimplemented opcode: 0x{:02X}", opcode),
    }
}

fn decode_reg(bits: u8) -> Operand8 {
    match bits {
        0b000 => Operand8::Reg(Register8::B),
        0b001 => Operand8::Reg(Register8::C),
        0b010 => Operand8::Reg(Register8::D),
        0b011 => Operand8::Reg(Register8::E),
        0b100 => Operand8::Reg(Register8::H),
        0b101 => Operand8::Reg(Register8::L),
        0b111 => Operand8::Reg(Register8::A),
        0b110 => Operand8::IndHL,
        _ => unreachable!("Unsupported Operand8: 0b{:03b}...", bits),
    }
}
