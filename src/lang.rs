use std::fmt;

#[derive(Debug)]
enum TokenType {
  LITERAL,
  REGISTER,
  INSTRUCTION,
}

struct Token {
  r#type: TokenType,
  value: String
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(type: {:?}, value: {:?})", self.r#type, self.value)
    }
}

pub fn compile(file_name: &String) {
	let token1: Token = Token { r#type: TokenType::INSTRUCTION, value: String::from("MovRegReg") };
	let token2: Token = Token { r#type: TokenType::LITERAL, value: String::from("3") };
	let token3: Token = Token { r#type: TokenType::REGISTER, value: String::from("R1") };
	
	println!("{}", token1);
	println!("{}", token2);
	println!("{}", token3);
	
	println!("Done compiling {:?}", file_name);
}
