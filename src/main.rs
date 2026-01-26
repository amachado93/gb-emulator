use std::fs;

mod bus;
mod cpu;

use bus::Bus;
use cpu::Cpu;

fn load_rom(bus: &mut Bus, path: &str) {
    let rom = fs::read(path).expect("Failed to read rom...");
    for (i, byte) in rom.iter().enumerate() {
        bus.memory[i] = *byte;
    }
}

fn main() {
    println!("GameBoy emulator loading...");

    let mut cpu = Cpu::new();
    let mut bus = Bus::new();

    load_rom(&mut bus, "roms/cpu_instrs/cpu_instrs.gb");
    cpu.reset();

    let mut cycles = 0;

    loop {
        let step_cycles = cpu.step(&mut bus);
        cycles += step_cycles as u64;

        // TEMP: break if emulator locks up
        if cycles > 50_000_000 {
            println!("TIMEOUT!");
            break;
        }
    }
}
