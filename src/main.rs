mod bus;
mod cpu;

use bus::Bus;
use cpu::Cpu;

fn main() {
    println!("GameBoy emulator loading...");

    let mut cpu = Cpu::new();
    let mut bus = Bus::new();

    cpu.reset();

    // quick sanity test
    bus.memory[0x0100] = 0x3E; // LD A, d8
    bus.memory[0x0101] = 0x0F;
    bus.memory[0x0102] = 0x06; // LD B, d8

    bus.memory[0x0103] = 0x01;
    bus.memory[0x0104] = 0x80; // ADD A. B
    bus.memory[0x0105] = 0x76; // HALT

    let mut step = 0;

    loop {
        let pc_before = cpu.regs.pc;
        let opcode = bus.read8(pc_before);

        let cycles = cpu.step(&mut bus);

        println!(
            "step {:02} | PC {:04X} | OP {:02X} | A {:02X} B {:02X} C {:02X} | F {:08b} | cycles {}",
            step, pc_before, opcode, cpu.regs.a, cpu.regs.b, cpu.regs.c, cpu.regs.f, cycles
        );

        step += 1;

        if cpu.halted {
            println!("\nCPU halted.");
            break;
        }
    }
}
