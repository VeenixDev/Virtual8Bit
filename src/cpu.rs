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
  
  pub fn set_register(&mut self, reg: Registers, value: u8) -> () {
  	self.registers[reg as usize] = value;
  }
  
  pub fn fetch(&mut self) -> u8 {
  	let next_instruction_address = self.get_register(Registers::IP);
  	let instruction = self.memory[usize::from(next_instruction_address)];
  	self.set_register(Registers::IP, next_instruction_address + 1);
  	instruction
  }
  
  pub fn execute(&mut self, instruction: u8) -> () {
    match instruction {
      // Move literal to r1
      x if x == instructions::Instructions::MovLitR1 as u8 => {
        let literal: u8 = self.fetch();
        self.set_register(Registers::R1, literal);
        return ();
      },
      x if x == instructions::Instructions::MovLitR2 as u8 => {
        let literal: u8 = self.fetch();
        self.set_register(Registers::R2, literal);
        return ();
      },
      // Add register to register
      x if x == instructions::Instructions::AddRegReg as u8 => {
        let r1: u8 = self.fetch();
        let r2: u8 = self.fetch();
        let register_value1: u8 = self.registers[usize::from(r1)];
        let register_value2: u8 = self.registers[usize::from(r2)];
        self.set_register(Registers::ACC, register_value1 + register_value2);
      	return ();
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
