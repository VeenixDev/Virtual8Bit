use std::env;
mod cpu;
mod lang;
use crate::cpu::instructions::Instructions;

fn run_machine() {
	let mut mem: [u8; 256] = [0u8; 256];
  
  mem[0] = Instructions::MovLitR1 as u8;
  mem[1] = 0x10;
  
  mem[2] = Instructions::MovLitR2 as u8;
  mem[3] = 0x4;
  
  mem[4] = Instructions::AddRegReg as u8;
  mem[5] = cpu::Registers::R1 as u8;
  mem[6] = cpu::Registers::R2 as u8;
  
  mem[7] = Instructions::MovRegR3 as u8;
  mem[8] = cpu::Registers::ACC as u8;
  
  mem[9] = Instructions::ModRegLit as u8;
  mem[10] = cpu::Registers::R3 as u8;
  mem[11] = 0x3;
  
  let mut cpu = cpu::CPU::new(mem);
  
  cpu.debug();
  cpu.step();
  cpu.debug();
  cpu.step();
  cpu.debug();
  cpu.step();
  cpu.debug();
  cpu.step();
  cpu.debug();
  cpu.step();
  cpu.debug();
}

fn main() {
	let args: Vec<String> = env::args().collect();
	
	match args.get(1).unwrap_or(&String::from("")).to_owned() {
	  x if x == String::from("run") => {
	  	run_machine();
	  },
	  x if x == String::from("compile") => {	  	
	  	if args.len() != 3 {	  		
	  		println!("Invalid file_name");
	  		return ();
	  	}
	  	let file_name: &String = args.get(2).unwrap();
	  	
	  	lang::compile(file_name);
	  	println!("Compile {}!", args.get(2).unwrap());	
	  },
	  _ => {
	  	println!("Usage: cargo run <run|compile> [filename]");
	  	return ();
	  },
	};
}
