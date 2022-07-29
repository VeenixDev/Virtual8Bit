pub mod instructions;

pub enum Registers {
  IP = 0, ACC = 1,
  R1 = 2, R2 = 3, R3 = 4, R4 = 5
}

pub struct CPU {
  registers: [u8; 6],
  memory: [u8; 256],
}

impl CPU {
  pub fn new(memory: [u8; 256]) -> Self {
    Self { memory: memory, registers: [0u8; 6] }
  }
  
  pub fn get_register(&self, reg: Registers) -> u8 {
    self.registers[reg as usize]
  }
  
  pub fn get_register_from_address(&self, reg: u8) -> u8 {
  	self.registers[usize::from(reg)]
  }
  
  pub fn set_register(&mut self, reg: Registers, value: u8) -> () {
  	self.registers[reg as usize] = value & 0xff;
  }
  
  pub fn fetch(&mut self) -> u8 {
  	let next_instruction_address = self.get_register(Registers::IP);
  	let instruction = self.memory[usize::from(next_instruction_address)];
  	self.set_register(Registers::IP, next_instruction_address + 1);
  	instruction
  }
  
  pub fn execute(&mut self, instruction: u8) -> () {
    match instruction {
      x if x == instructions::Instructions::MovLitR1 as u8 => {
        let literal: u8 = self.fetch();
        self.set_register(Registers::R1, literal);
      },
      x if x == instructions::Instructions::MovLitR2 as u8 => {
        let literal: u8 = self.fetch();
        self.set_register(Registers::R2, literal);
      },
      x if x == instructions::Instructions::MovLitR3 as u8 => {
      	let literal: u8 = self.fetch();
      	self.set_register(Registers::R3, literal);
      },
      x if x == instructions::Instructions::MovLitR4 as u8 => {
      	let literal: u8 = self.fetch();
      	self.set_register(Registers::R4, literal);
      },
      x if x == instructions::Instructions::MovRegR1 as u8 => {
      	let reg: u8 = self.fetch();
      	let register_value: u8 = self.get_register_from_address(reg);
      	self.set_register(Registers::R1, register_value);
      },
      x if x == instructions::Instructions::MovRegR2 as u8 => {
      	let reg: u8 = self.fetch();
      	let register_value: u8 = self.get_register_from_address(reg);
      	self.set_register(Registers::R2, register_value);
      },
      x if x == instructions::Instructions::MovRegR3 as u8 => {
      	let reg: u8 = self.fetch();
      	let register_value: u8 = self.get_register_from_address(reg);
      	self.set_register(Registers::R3, register_value);
      },
      x if x == instructions::Instructions::MovRegR4 as u8 => {
      	let reg: u8 = self.fetch();
      	let register_value: u8 = self.get_register_from_address(reg);
      	self.set_register(Registers::R4, register_value);
      },
      x if x == instructions::Instructions::AddRegReg as u8 => {
        let r1: u8 = self.fetch();
        let r2: u8 = self.fetch();
        let register_value1: u8 = self.get_register_from_address(r1);
        let register_value2: u8 = self.get_register_from_address(r2);
        self.set_register(Registers::ACC, register_value1 + register_value2);
      },
      x if x == instructions::Instructions::AddRegLit as u8 => {
      	let reg: u8 = self.fetch();
      	let lit: u8 = self.fetch();
      	let register_value: u8 = self.get_register_from_address(reg);
      	self.set_register(Registers::ACC, register_value + lit);
      },
      x if x == instructions::Instructions::OrRegReg as u8 => {
      	let r1: u8 = self.fetch();
      	let r2: u8 = self.fetch();
      	let register_value1: u8 = self.get_register_from_address(r1);
      	let register_value2: u8 = self.get_register_from_address(r2);
      	self.set_register(Registers::ACC, register_value1 | register_value2);
      },
      x if x == instructions::Instructions::OrRegLit as u8 => {
      	let reg: u8 = self.fetch();
      	let lit: u8 = self.fetch();
      	let register_value: u8 = self.get_register_from_address(reg);
      	self.set_register(Registers::ACC, register_value | lit);
      },
      x if x == instructions::Instructions::AndRegReg as u8 => {
      	let r1: u8 = self.fetch();
      	let r2: u8 = self.fetch();
      	let register_value1: u8 = self.get_register_from_address(r1);
      	let register_value2: u8 = self.get_register_from_address(r2);
      	self.set_register(Registers::ACC, register_value1 & register_value2);
      },
      x if x == instructions::Instructions::AndRegLit as u8 => {
      	let reg: u8 = self.fetch();
      	let lit: u8 = self.fetch();
      	let register_value: u8 = self.get_register_from_address(reg);
      	self.set_register(Registers::ACC, register_value & lit);
      },
      x if x == instructions::Instructions::XorRegReg as u8 => {
      	let r1: u8 = self.fetch();
      	let r2: u8 = self.fetch();
      	let register_value1: u8 = self.get_register_from_address(r1);
      	let register_value2: u8 = self.get_register_from_address(r2);
      	self.set_register(Registers::ACC, register_value1 ^ register_value2);
      },
      x if x == instructions::Instructions::XorRegLit as u8 => {
      	let reg: u8 = self.fetch();
      	let lit: u8 = self.fetch();
      	let register_value: u8 = self.get_register_from_address(reg);
      	self.set_register(Registers::ACC, register_value ^ lit);
      },
      x if x == instructions::Instructions::NotReg as u8 => {
      	let reg: u8 = self.fetch();
      	let register_value: u8 = self.get_register_from_address(reg);
      	self.set_register(Registers::ACC, !register_value);
      },
      x if x == instructions::Instructions::NotLit as u8 => {
      	let lit: u8 = self.fetch();
      	self.set_register(Registers::ACC, !lit);
      },
      x if x == instructions::Instructions::SubRegReg as u8 => {
      	let r1 = self.fetch();
      	let r2 = self.fetch();
      	let register_value1 = self.get_register_from_address(r1);
      	let register_value2 = self.get_register_from_address(r2);
      	self.set_register(Registers::ACC, register_value1 - register_value2);
      },
      x if x == instructions::Instructions::SubRegLit as u8 => {
      	let reg = self.fetch();
      	let lit = self.fetch();
      	let register_value = self.get_register_from_address(reg);
      	self.set_register(Registers::ACC, register_value - lit);
      },
      x if x == instructions::Instructions::SubLitReg as u8 => {
      	let reg = self.fetch();
      	let lit = self.fetch();
      	let register_value = self.get_register_from_address(reg);
      	self.set_register(Registers::ACC, lit - register_value);
      },
      x if x == instructions::Instructions::MulRegReg as u8 => {
      	let r1 = self.fetch();
      	let r2 = self.fetch();
      	let register_value1 = self.get_register_from_address(r1);
      	let register_value2 = self.get_register_from_address(r2);
      	self.set_register(Registers::ACC, register_value1 * register_value2);
      },
      x if x == instructions::Instructions::MulRegLit as u8 => {
      	let reg = self.fetch();
      	let lit = self.fetch();
      	let register_value = self.get_register_from_address(reg);
      	self.set_register(Registers::ACC, register_value * lit);
      },
      x if x == instructions::Instructions::DivRegReg as u8 => {
      	let r1 = self.fetch();
      	let r2 = self.fetch();
      	let register_value1 = self.get_register_from_address(r1);
      	let register_value2 = self.get_register_from_address(r2);
      	self.set_register(Registers::ACC, register_value1 / register_value2);
      },
      x if x == instructions::Instructions::DivRegLit as u8 => {
      	let reg = self.fetch();
      	let lit = self.fetch();
      	let register_value = self.get_register_from_address(reg);
      	self.set_register(Registers::ACC, register_value / lit);
      },
      x if x == instructions::Instructions::DivLitReg as u8 => {
      	let reg = self.fetch();
      	let lit = self.fetch();
      	let register_value = self.get_register_from_address(reg);
      	self.set_register(Registers::ACC, lit / register_value);
      },
      x if x == instructions::Instructions::ModRegReg as u8 => {
      	let r1 = self.fetch();
      	let r2 = self.fetch();
      	let register_value1 = self.get_register_from_address(r1);
      	let register_value2 = self.get_register_from_address(r2);
      	self.set_register(Registers::ACC, register_value1 % register_value2);
      },
      x if x == instructions::Instructions::ModRegLit as u8 => {
      	let reg = self.fetch();
      	let lit = self.fetch();
      	let register_value = self.get_register_from_address(reg);
      	self.set_register(Registers::ACC, register_value % lit);
      },
      x if x == instructions::Instructions::ModLitReg as u8 => {
      	let reg = self.fetch();
      	let lit = self.fetch();
      	let register_value = self.get_register_from_address(reg);
      	self.set_register(Registers::ACC, lit % register_value);
      },
      _ => { return () }
    }
  }
  
  pub fn step(&mut self) -> () {
    let instruction = self.fetch();
    self.execute(instruction)
  }
  
  pub fn debug(&self) -> () {
    println!("ip:  0x{:0>2X}", self.get_register(Registers::IP));
    println!("acc: 0x{:0>2X}", self.get_register(Registers::ACC));
    println!("r1:  0x{:0>2X}", self.get_register(Registers::R1));
    println!("r2:  0x{:0>2X}", self.get_register(Registers::R2));
    println!("r3:  0x{:0>2X}", self.get_register(Registers::R3));
    println!("r4:  0x{:0>2X}", self.get_register(Registers::R4));
    println!("");
  }
}
