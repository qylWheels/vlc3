use super::Instruction;
use super::opcode::OpCode;

pub struct Rti;

impl Rti {
	pub fn new() -> Self {
		Self
	}
}

impl Instruction for Rti {
	fn execute(&self) {
		unimplemented!("This operation isn't allowed in vlc3");
	}

	fn opcode(&self) -> OpCode {
		OpCode::RTI
	}
}
