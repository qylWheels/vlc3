use super::Instruction;
use super::opcode::OpCode;

pub struct Res;

impl Res {
	pub fn new() -> Self {
		Self
	}
}

impl Instruction for Res {
	fn execute(&self) {
		unimplemented!("This operation isn't allowed in vlc3");
	}

	fn opcode(&self) -> OpCode {
		OpCode::RES
	}
}
