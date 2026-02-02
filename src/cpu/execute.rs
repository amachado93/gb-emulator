use super::{Cpu, instructions::*};
use crate::bus::Bus;

impl Cpu {
    pub fn step(&mut self, bus: &mut Bus) -> u8 {
        if self.halted {
            return 4; // HALT burns cycles
        }

        let opcode = bus.read8(self.regs.pc);
        self.regs.pc = self.regs.pc.wrapping_add(1);

        let instruction = decode(opcode);
        self.execute_instruction(instruction, bus)
    }

    fn execute_instruction(&mut self, instr: Instruction, bus: &mut Bus) -> u8 {
        match instr {
            Instruction::NOP => 4,
            Instruction::LDBCD16 => self.ld_bc_d16(bus),
            Instruction::LDBCA => self.ld_bc_a(bus),
            Instruction::INCBC => self.inc_bc(),
            Instruction::DECB => self.dec_b(),
            Instruction::INCHL => self.inc_hl(),
            Instruction::INCH => self.inc_h(),
            Instruction::LDDED16 => self.ld_de_d16(bus),
            Instruction::INCDE => self.inc_de(),
            Instruction::LDADE => self.ld_a_de(bus),
            Instruction::JRZR8 => self.jr_z_r8(bus),
            Instruction::JRNZR8 => self.jr_nz_r8(bus),
            Instruction::LDAHLINC => self.ld_a_hlinc(bus),
            Instruction::INCL => self.inc_c(),
            Instruction::ADD(target) => self.add(target),
            Instruction::ORC => self.or_c(),
            Instruction::LDIMM8(reg) => self.ld_imm8(reg, bus),
            Instruction::LDHLD16 => self.ld_hl_d16(bus),
            Instruction::LDHLPOSA => self.ld_hlpos_a(bus),
            Instruction::LDA8A => self.ld_a8_a(bus),
            Instruction::LDA16A => self.ld_a16_a(bus),
            Instruction::LDAA16 => self.ld_a_a16(bus),
            Instruction::JRR8 => self.jr_r8(bus),
            Instruction::LDRegReg(dst, src) => self.ld_reg_reg(dst, src, bus),
            Instruction::LDSPD16 => self.ld_sp_d16(bus),
            Instruction::CP(reg) => self.cp(reg, bus),
            Instruction::XORB => self.xor_b(),
            Instruction::XORC => self.xor_c(),
            Instruction::XORD => self.xor_d(),
            Instruction::XORE => self.xor_e(),
            Instruction::XORH => self.xor_h(),
            Instruction::XORL => self.xor_l(),
            Instruction::XORA => self.xor_a(),
            Instruction::POPBC => self.pop_bc(bus),
            Instruction::JPA16 => self.jp_a16(bus),
            Instruction::CALLNZA16 => self.call_nz_a16(bus),
            Instruction::PUSHBC => self.push_bc(bus),
            Instruction::ADDAD8 => self.add_a_d8(bus),
            Instruction::CALLA16 => self.call_a16(bus),
            Instruction::RET => self.ret(bus),
            Instruction::PUSHHL => self.push_hl(bus),
            Instruction::ANDD8 => self.and_d8(bus),
            Instruction::POPHL => self.pop_hl(bus),
            Instruction::DI => self.di(),
            Instruction::LDHAA8 => self.ldh_a_a8(bus),
            Instruction::POPAF => self.pop_af(bus),
            Instruction::PUSHAF => self.push_af(bus),
            Instruction::CPD8 => self.cp_d8(bus),
        }
    }

    fn ld_bc_d16(&mut self, bus: &mut Bus) -> u8 {
        // read 16-bit immediate (lil endian)
        let lo = bus.read8(self.regs.pc) as u16;
        self.regs.pc = self.regs.pc.wrapping_add(1);

        // then high
        let hi = bus.read8(self.regs.pc) as u16;
        self.regs.pc = self.regs.pc.wrapping_add(1);

        let value = (hi << 8) | lo;

        self.regs.set_bc(value);

        12
    }

    fn ld_bc_a(&mut self, bus: &mut Bus) -> u8 {
        let addr = self.regs.get_bc();
        bus.write8(addr, self.regs.a);

        8
    }

    fn inc_bc(&mut self) -> u8 {
        let data = self.regs.get_bc();
        let result = data.wrapping_add(1);

        self.regs.set_bc(result);

        8
    }

    fn dec_b(&mut self) -> u8 {
        let orig_val = self.regs.b;
        let result = self.regs.b.wrapping_sub(1);

        self.regs.b = result;

        // flags
        self.regs.set_z(result == 0);
        self.regs.set_n(true);
        self.regs.set_h((orig_val & 0x0F) == 0x0F);

        4
    }

    fn inc_hl(&mut self) -> u8 {
        let data = self.regs.get_hl();
        let result = data.wrapping_add(1);

        self.regs.set_hl(result);

        8
    }

    fn inc_h(&mut self) -> u8 {
        let orig_val = self.regs.h;
        let result = self.regs.h.wrapping_add(1);

        self.regs.h = result;

        // flags
        self.regs.set_z(result == 0);
        self.regs.set_n(false);
        self.regs.set_h((orig_val & 0x0F) == 0x0F);

        4
    }

    fn ld_de_d16(&mut self, bus: &mut Bus) -> u8 {
        let lo = bus.read8(self.regs.pc) as u16;
        self.regs.pc = self.regs.pc.wrapping_add(1);

        let hi = bus.read8(self.regs.pc) as u16;
        self.regs.pc = self.regs.pc.wrapping_add(1);

        let value = (hi << 8) | lo;

        self.regs.set_de(value);

        12
    }

    fn inc_de(&mut self) -> u8 {
        let data = self.regs.get_de();
        let result = data.wrapping_add(1);

        self.regs.set_de(result);

        8
    }

    fn ld_a_de(&mut self, bus: &mut Bus) -> u8 {
        self.regs.a = bus.read8(self.regs.get_de());

        8
    }

    fn jr_z_r8(&mut self, bus: &mut Bus) -> u8 {
        // read signed 8-bit offset
        let offset = bus.read8(self.regs.pc) as i8;
        self.regs.pc = self.regs.pc.wrapping_add(1);

        // check Zero flag
        if self.regs.get_z() {
            // PC is already pointing to the next instruction
            self.regs.pc = self.regs.pc.wrapping_add(offset as u16);

            12
        } else {
            8
        }
    }

    // this can be read as - Jump if Zero flag is _not_ set
    fn jr_nz_r8(&mut self, bus: &mut Bus) -> u8 {
        let e = bus.read8(self.regs.pc) as i8;
        self.regs.pc = self.regs.pc.wrapping_add(1);

        if !self.regs.get_z() {
            self.regs.pc = self.regs.pc.wrapping_add(e as u16);
            12
        } else {
            8
        }
    }

    fn ld_a_hlinc(&mut self, bus: &mut Bus) -> u8 {
        let hl = self.regs.get_hl();

        self.regs.a = bus.read8(hl);
        self.regs.set_hl(hl.wrapping_add(1));

        8
    }

    fn inc_c(&mut self) -> u8 {
        let orig_val = self.regs.c;
        let result = self.regs.c.wrapping_add(1);
        self.regs.c = result;

        // flags
        self.regs.set_z(result == 0);
        self.regs.set_n(false);
        self.regs.set_h((orig_val & 0x0F) == 0x0F);

        4
    }

    fn add(&mut self, target: ArithmeticTarget) -> u8 {
        let value = match target {
            ArithmeticTarget::A => self.regs.a,
            ArithmeticTarget::B => self.regs.b,
            ArithmeticTarget::C => self.regs.c,
            ArithmeticTarget::D => self.regs.d,
            ArithmeticTarget::E => self.regs.e,
            ArithmeticTarget::H => self.regs.h,
            ArithmeticTarget::L => self.regs.l,
        };

        let a = self.regs.a;
        let result = a.wrapping_add(value);

        // flags
        self.regs.set_z(result == 0);
        self.regs.set_n(false);
        self.regs.set_h(((a & 0xF) + (value & 0xF)) > 0xF);
        self.regs.set_c((a as u16 + value as u16) > 0xFF);

        self.regs.a = result;

        4 // we return the number of cycles
    }

    fn or_c(&mut self) -> u8 {
        let result = self.regs.a | self.regs.c;
        self.regs.a = result;

        // flags are set after bitwise OR operation
        self.regs.set_z(result == 0);
        self.regs.set_n(false); // make sure n, h, and c are cleared
        self.regs.set_h(false);
        self.regs.set_c(false);

        4
    }

    fn ld_imm8(&mut self, reg: Register8, bus: &mut Bus) -> u8 {
        let value = bus.read8(self.regs.pc);
        self.regs.pc = self.regs.pc.wrapping_add(1);

        match reg {
            Register8::A => self.regs.a = value,
            Register8::B => self.regs.b = value,
            Register8::C => self.regs.c = value,
            Register8::D => self.regs.d = value,
            Register8::E => self.regs.e = value,
            Register8::H => self.regs.h = value,
            Register8::L => self.regs.l = value,
        }

        8
    }

    fn ld_hl_d16(&mut self, bus: &mut Bus) -> u8 {
        // read 16-bit immediate (lil endian)
        let lo = bus.read8(self.regs.pc) as u16;
        self.regs.pc = self.regs.pc.wrapping_add(1);

        let hi = bus.read8(self.regs.pc) as u16;
        self.regs.pc = self.regs.pc.wrapping_add(1);

        let value = (hi << 8) | lo;

        self.regs.set_hl(value);
        12
    }

    fn ld_a16_a(&mut self, bus: &mut Bus) -> u8 {
        // read 16-bit immediate (little endian)
        let lo = bus.read8(self.regs.pc) as u16;
        self.regs.pc = self.regs.pc.wrapping_add(1);

        let hi = bus.read8(self.regs.pc) as u16;
        self.regs.pc = self.regs.pc.wrapping_add(1);

        let addr = (hi << 8) | lo;

        bus.write8(addr, self.regs.a);

        16
    }

    fn ld_sp_d16(&mut self, bus: &mut Bus) -> u8 {
        // read 16-bit immediate address (little endian)
        let lo = bus.read8(self.regs.pc) as u16;
        self.regs.pc = self.regs.pc.wrapping_add(1);

        let hi = bus.read8(self.regs.pc) as u16;
        self.regs.pc = self.regs.pc.wrapping_add(1);

        let value = (hi << 8) | lo;

        self.regs.sp = value;

        12
    }

    fn ld_hlpos_a(&mut self, bus: &mut Bus) -> u8 {
        let hl = self.regs.get_hl();
        bus.write8(hl, self.regs.a);

        self.regs.set_hl(hl.wrapping_add(1));

        8
    }

    fn ld_a8_a(&mut self, bus: &mut Bus) -> u8 {
        let offset = bus.read8(self.regs.pc) as u16;
        self.regs.pc = self.regs.pc.wrapping_add(1);

        let addr = 0xFF00 | offset;

        bus.write8(addr, self.regs.a);

        12
    }

    fn ld_a_a16(&mut self, bus: &mut Bus) -> u8 {
        // read 16-bit immediate address (little endian)
        let lo = bus.read8(self.regs.pc) as u16;
        self.regs.pc = self.regs.pc.wrapping_add(1);

        let hi = bus.read8(self.regs.pc) as u16;
        self.regs.pc = self.regs.pc.wrapping_add(1);

        let addr = (hi << 8) | lo;

        // read from memory into A
        self.regs.a = bus.read8(addr);

        16
    }

    fn jr_r8(&mut self, bus: &mut Bus) -> u8 {
        // read signed 8-bit offset
        let offset = bus.read8(self.regs.pc) as i8;
        self.regs.pc = self.regs.pc.wrapping_add(1);

        // PC-relative jump

        self.regs.pc = self.regs.pc.wrapping_add(offset as u16);

        12
    }

    fn ld_reg_reg(&mut self, dst: Operand8, src: Operand8, bus: &mut Bus) -> u8 {
        let value = match src {
            Operand8::Reg(r) => self.regs.read_reg8(r),
            Operand8::IndHL => {
                let addr = self.regs.get_hl();
                bus.read8(addr)
            }
        };

        match dst {
            Operand8::Reg(r) => self.regs.write_reg8(r, value),
            Operand8::IndHL => {
                let addr = self.regs.get_hl();
                bus.write8(addr, value)
            }
        }

        // timing depends on whether (HL) was involved
        if matches!(src, Operand8::IndHL) || matches!(dst, Operand8::IndHL) {
            8
        } else {
            4
        }
    }

    fn cp(&mut self, op: Operand8, bus: &Bus) -> u8 {
        let value = match op {
            Operand8::Reg(r) => self.regs.read_reg8(r),
            Operand8::IndHL => {
                let addr = self.regs.get_hl();
                bus.read8(addr)
            }
        };

        let a = self.regs.a;
        let result = a.wrapping_sub(value);

        // Flags - this is important!!
        self.regs.set_z(result == 0);
        self.regs.set_n(true);
        self.regs.set_h((a & 0xF) < (value & 0xF));
        self.regs.set_c(a < value);

        // NOTE: A is not modified!

        // timing
        match op {
            Operand8::Reg(_) => 4,
            Operand8::IndHL => 8,
        }
    }

    fn xor_b(&mut self) -> u8 {
        let result = self.regs.a ^ self.regs.b;
        self.regs.a = result;

        // flags
        self.regs.set_z(result == 0);
        self.regs.set_n(false);
        self.regs.set_h(false);
        self.regs.set_c(false);

        4
    }

    fn xor_c(&mut self) -> u8 {
        let result = self.regs.a ^ self.regs.c;
        self.regs.a = result;

        // flags
        self.regs.set_z(result == 0);
        self.regs.set_n(false);
        self.regs.set_h(false);
        self.regs.set_c(false);

        4
    }

    fn xor_d(&mut self) -> u8 {
        let result = self.regs.a ^ self.regs.d;
        self.regs.a = result;

        // flags
        self.regs.set_z(result == 0);
        self.regs.set_n(false);
        self.regs.set_h(false);
        self.regs.set_c(false);

        4
    }

    fn xor_e(&mut self) -> u8 {
        let result = self.regs.a ^ self.regs.e;
        self.regs.a = result;

        // flags
        self.regs.set_z(result == 0);
        self.regs.set_n(false);
        self.regs.set_h(false);
        self.regs.set_c(false);

        4
    }

    fn xor_h(&mut self) -> u8 {
        let result = self.regs.a ^ self.regs.h;
        self.regs.a = result;

        // flags
        self.regs.set_z(result == 0);
        self.regs.set_n(false);
        self.regs.set_h(false);
        self.regs.set_c(false);

        4
    }

    fn xor_l(&mut self) -> u8 {
        let result = self.regs.a ^ self.regs.l;
        self.regs.a = result;

        // flags
        self.regs.set_z(result == 0);
        self.regs.set_n(false);
        self.regs.set_h(false);
        self.regs.set_c(false);

        4
    }

    fn xor_a(&mut self) -> u8 {
        let result = self.regs.a ^ self.regs.a;
        self.regs.a = result;

        // flags
        self.regs.set_z(result == 0);
        self.regs.set_n(false);
        self.regs.set_h(false);
        self.regs.set_c(false);

        4
    }

    fn pop_bc(&mut self, bus: &mut Bus) -> u8 {
        let lo = bus.read8(self.regs.sp) as u16;
        self.regs.sp = self.regs.sp.wrapping_add(1);

        let hi = bus.read8(self.regs.sp) as u16;
        self.regs.sp = self.regs.sp.wrapping_add(1);

        let value = (hi << 8) | lo;
        self.regs.set_bc(value);

        12
    }

    fn jp_a16(&mut self, bus: &mut Bus) -> u8 {
        // read 16-bit immediate (lil endian)
        let lo = bus.read8(self.regs.pc) as u16;
        self.regs.pc = self.regs.pc.wrapping_add(1);

        let hi = bus.read8(self.regs.pc) as u16;
        self.regs.pc = self.regs.pc.wrapping_add(1);

        let addr = (hi << 8) | lo;

        // jump
        self.regs.pc = addr;

        16
    }

    fn call_nz_a16(&mut self, bus: &mut Bus) -> u8 {
        let lo = bus.read8(self.regs.pc) as u16;
        self.regs.pc = self.regs.pc.wrapping_add(1);

        let hi = bus.read8(self.regs.pc) as u16;
        self.regs.pc = self.regs.pc.wrapping_add(1);

        let target = (hi << 8) | lo;

        // push return address (PC after operands)
        let ret = self.regs.pc;

        if !self.regs.get_z() {
            // push high byte first
            self.regs.sp = self.regs.sp.wrapping_sub(1);
            bus.write8(self.regs.sp, (ret >> 8) as u8);

            // then low byte
            self.regs.sp = self.regs.sp.wrapping_sub(1);
            bus.write8(self.regs.sp, (ret & 0xFF) as u8);

            // jump
            self.regs.pc = target;

            24
        } else {
            12
        }
    }

    fn push_bc(&mut self, bus: &mut Bus) -> u8 {
        let b = self.regs.b;
        let c = self.regs.c;

        // push high byte first
        self.regs.sp = self.regs.sp.wrapping_sub(1);
        bus.write8(self.regs.sp, b);

        // then low byte
        self.regs.sp = self.regs.sp.wrapping_sub(1);
        bus.write8(self.regs.sp, c);

        16
    }

    fn add_a_d8(&mut self, bus: &mut Bus) -> u8 {
        let reg_a = self.regs.a;
        let n = bus.read8(self.regs.pc);
        self.regs.pc = self.regs.pc.wrapping_add(1);

        let result = reg_a.wrapping_add(n);
        self.regs.a = result;

        // flags
        self.regs.set_z(result == 0);
        self.regs.set_n(false);
        self.regs.set_h(((reg_a & 0x0F) + (n & 0x0F)) > 0x0F);
        self.regs.set_c((reg_a as u16 + n as u16) > 0xFF);

        8
    }

    fn call_a16(&mut self, bus: &mut Bus) -> u8 {
        // read target address (lil endian)
        let lo = bus.read8(self.regs.pc) as u16;
        self.regs.pc = self.regs.pc.wrapping_add(1);

        let hi = bus.read8(self.regs.pc) as u16;
        self.regs.pc = self.regs.pc.wrapping_add(1);

        let target = (hi << 8) | lo;

        // push return address (PC after operands)
        let ret = self.regs.pc;

        // push high byte first
        self.regs.sp = self.regs.sp.wrapping_sub(1);
        bus.write8(self.regs.sp, (ret >> 8) as u8);

        // then low byte
        self.regs.sp = self.regs.sp.wrapping_sub(1);
        bus.write8(self.regs.sp, (ret & 0xFF) as u8);

        // jump
        self.regs.pc = target;

        24
    }

    fn and_d8(&mut self, bus: &mut Bus) -> u8 {
        let n = bus.read8(self.regs.pc);
        self.regs.pc = self.regs.pc.wrapping_add(1);

        let result = self.regs.a & n;
        self.regs.a = result;

        self.regs.set_z(result == 0);
        self.regs.set_n(false);
        self.regs.set_h(true);
        self.regs.set_c(false);

        8
    }

    fn pop_hl(&mut self, bus: &mut Bus) -> u8 {
        // pop low byte
        let lo = bus.read8(self.regs.sp) as u16;
        self.regs.sp = self.regs.sp.wrapping_add(1);

        // then high byte
        let hi = bus.read8(self.regs.sp) as u16;
        self.regs.sp = self.regs.sp.wrapping_add(1);

        let value = (hi << 8) | lo;

        self.regs.set_hl(value);

        12
    }

    fn push_hl(&mut self, bus: &mut Bus) -> u8 {
        let h = self.regs.h;
        let l = self.regs.l;

        // push high byte first
        self.regs.sp = self.regs.sp.wrapping_sub(1);
        bus.write8(self.regs.sp, h);

        // then low byte
        self.regs.sp = self.regs.sp.wrapping_sub(1);
        bus.write8(self.regs.sp, l);

        16
    }

    fn ret(&mut self, bus: &mut Bus) -> u8 {
        // pop low byte
        let lo = bus.read8(self.regs.sp) as u16;
        self.regs.sp = self.regs.sp.wrapping_add(1);

        // pop high byte
        let hi = bus.read8(self.regs.sp) as u16;
        self.regs.sp = self.regs.sp.wrapping_add(1);

        let addr = (hi << 8) | lo;

        self.regs.pc = addr;

        16
    }

    fn di(&mut self) -> u8 {
        self.ime = false;
        4
    }

    fn ldh_a_a8(&mut self, bus: &mut Bus) -> u8 {
        let lo = bus.read8(self.regs.pc) as u16;
        self.regs.pc = self.regs.pc.wrapping_add(1);

        let hi = 0xFF;
        let addr = (hi << 8) | lo;

        self.regs.a = bus.read8(addr);

        12
    }

    fn pop_af(&mut self, bus: &mut Bus) -> u8 {
        // pop lower byte
        let lo = bus.read8(self.regs.sp) as u16;
        self.regs.sp = self.regs.sp.wrapping_add(1);

        // pop high byte
        let hi = bus.read8(self.regs.sp) as u16;
        self.regs.sp = self.regs.sp.wrapping_add(1);

        let value = (hi << 8) | lo;
        self.regs.set_af(value);

        12
    }

    fn push_af(&mut self, bus: &mut Bus) -> u8 {
        let a = self.regs.a;
        let f = self.regs.f;

        // because F is the flags reg (aka restricted), ensure bits 3 to 0 are cleared
        let cleared_f = f & !0x0F;

        // first push high byte
        self.regs.sp = self.regs.sp.wrapping_sub(1);
        bus.write8(self.regs.sp, a);

        // then low byte
        self.regs.sp = self.regs.sp.wrapping_sub(1);
        bus.write8(self.regs.sp, cleared_f);

        16
    }

    fn cp_d8(&mut self, bus: &mut Bus) -> u8 {
        let n = bus.read8(self.regs.pc); // read the immediate data
        self.regs.pc = self.regs.pc.wrapping_add(1);

        let reg_a = self.regs.a;
        let result = reg_a.wrapping_sub(n);

        // set flags
        self.regs.set_z(result == 0);
        self.regs.set_n(true);
        self.regs.set_h((reg_a & 0x0F) < (n & 0x0F));
        self.regs.set_c((reg_a & 0xFF) < (n & 0xFF));

        8
    }
}
