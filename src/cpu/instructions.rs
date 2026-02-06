use core::panic;

#[derive(Debug)]
pub enum Instruction {
    NOP,
    LDBCD16, // LD BC, d16
    LDBCA,   // LD (BC), A
    INCBC,   // INC BC
    DECB,
    LDDED16,
    INCDE,
    JRNZR8, // JR NZ, r8
    INCHL,  // INC HL
    INCH,
    JRZR8, // JR Z, r8
    LDADE,
    LDAHLINC, // LD A, (HL+)
    INCL,
    ADD(ArithmeticTarget),
    LDIMM8(Register8),
    LDA8A,     // LD (a8), A
    LDA16A,    // LD (a16), A
    LDAA16,    // LD A, (a16)
    LDHLD16,   // LD HL, d16
    LDHLPOSA,  // LD (HL+), A
    LDHLNEGA,  // LD (HL-), A
    JRR8,      // JR r8
    JPA16,     // JP a16
    CALLNZA16, // CALL NZ, a16
    XORB,
    XORC,
    XORD,
    XORE,
    XORH,
    XORL,
    XORA,
    POPBC,
    CALLA16, // CALL a16
    RET,
    LDRegReg(Operand8, Operand8),
    ORB,
    ORC,
    ORD,
    ORE,
    ORH,
    ORL,
    ORHL,
    ORA,
    CP(Operand8),
    DI,
    LDSPD16,
    PUSHBC,
    ADDAD8,
    SUBD8,
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
        0x05 => Instruction::DECB,

        0x11 => Instruction::LDDED16,
        0x13 => Instruction::INCDE,
        0x1A => Instruction::LDADE,

        0x20 => Instruction::JRNZR8,
        0x21 => Instruction::LDHLD16,
        0x22 => Instruction::LDHLPOSA,
        0x23 => Instruction::INCHL,
        0x24 => Instruction::INCH,
        0x28 => Instruction::JRZR8,
        0x2A => Instruction::LDAHLINC,
        0x2C => Instruction::INCL,

        // CP r (0xB8-0xBF)
        0xB8..=0xBF => {
            let reg = decode_reg(opcode & 0b111);
            Instruction::CP(reg)
        }

        // LD SP, d16
        0x31 => Instruction::LDSPD16,
        0x32 => Instruction::LDHLNEGA,

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

        0xA8 => Instruction::XORB,
        0xA9 => Instruction::XORC,
        0xAA => Instruction::XORD,
        0xAB => Instruction::XORE,
        0xAC => Instruction::XORH,
        0xAD => Instruction::XORL,
        0xAF => Instruction::XORA,

        0xB0 => Instruction::ORB,
        0xB1 => Instruction::ORC,
        0xB3 => Instruction::ORD,
        0xB2 => Instruction::ORE,
        0xB4 => Instruction::ORH,
        0xB5 => Instruction::ORL,
        0xB6 => Instruction::ORHL,
        0xB7 => Instruction::ORA,

        0xC1 => Instruction::POPBC,
        0xC3 => Instruction::JPA16,
        0xC4 => Instruction::CALLNZA16,
        0xC5 => Instruction::PUSHBC,
        0xC6 => Instruction::ADDAD8,

        0xC9 => Instruction::RET,
        // CALLA16
        0xCD => Instruction::CALLA16,

        0xD6 => Instruction::SUBD8,

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
