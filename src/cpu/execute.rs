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
            Instruction::ADD(target) => self.add(target),
            Instruction::LDIMM8(reg) => self.id_imm8(reg, bus),
        }
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

        4
    }

    fn id_imm8(&mut self, reg: Register8, bus: &mut Bus) -> u8 {
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
}
