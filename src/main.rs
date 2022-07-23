mod cpu;
use crate::cpu::instructions::Instructions;

fn main() {
  let mut mem: [u8; 256] = [0u8; 256];
  
  mem[0] = Instructions::MovLitR1 as u8;
  mem[1] = 0x10;
  
  mem[2] = Instructions::MovLitR2 as u8;
  mem[3] = 0x4;
  
  mem[4] = Instructions::AddRegReg as u8;
  mem[5] = cpu::Registers::R1 as u8;
  mem[6] = cpu::Registers::R2 as u8;
  
  let mut cpu = cpu::CPU::new(mem);
  
  cpu.debug();
  cpu.step();
  cpu.debug();
  cpu.step();
  cpu.debug();
  cpu.step();
  cpu.debug();
}
