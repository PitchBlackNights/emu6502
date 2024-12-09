use emu6502::cpu::CPU;
use emu6502::mem::Mem;

fn main() {
    let mut mem: Mem = Mem::new();
    let mut cpu: CPU = CPU::new();
    cpu.force_reset(&mut mem);
}
