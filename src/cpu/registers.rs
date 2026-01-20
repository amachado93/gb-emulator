#[derive(Default)]
pub struct Registers {
    pub a: u8,
    pub f: u8, // flags
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

const Z: u8 = 0b1000_0000;
const N: u8 = 0b0100_0000;
const H: u8 = 0b0010_0000;
const C: u8 = 0b0001_0000;

impl Registers {
    // 16 bit register helpers

    // NOTE:
    // we treat the first register as a u16 and then shift 8 positions
    // so that it's occuping the most significant byte position.
    // then we bitwise OR the other register. the result is
    // a two byte number with the contents of the first register
    // in the MSB position and the other register occuping LSB
    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | self.f as u16
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = (value & 0xF0) as u8; // important! mask lower nibble for F register
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    // flag helpers
    pub fn get_z(&self) -> bool {
        self.f & Z != 0
    }
    pub fn set_z(&mut self, v: bool) {
        self.f = if v { self.f | Z } else { self.f & !Z }
    }

    pub fn get_n(&self) -> bool {
        self.f & N != 0
    }
    pub fn set_n(&mut self, v: bool) {
        self.f = if v { self.f | N } else { self.f & !N }
    }

    pub fn get_h(&self) -> bool {
        self.f & H != 0
    }
    pub fn set_h(&mut self, v: bool) {
        self.f = if v { self.f | H } else { self.f & !H }
    }

    pub fn get_c(&self) -> bool {
        self.f & C != 0
    }
    pub fn set_c(&mut self, v: bool) {
        self.f = if v { self.f | C } else { self.f & !C }
    }
}

#[derive(Default)]
pub struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION
            | (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION
            | (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}
