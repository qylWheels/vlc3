use crate::cpu::{
	register::Register,
	CPU,
};
use super::Instruction;
use super::opcode::OpCode;

pub struct Jsr {
	offset: u16,
}

impl Jsr {
	pub fn new(offset: u16) -> Self {
		Self {
			offset,
		}
	}
}

impl Instruction for Jsr {
	fn execute(&self) {
		CPU.write(Register::R7, CPU.read(Register::PC));
		CPU.write(
			Register::PC,
			CPU.read(Register::PC).wrapping_add(self.offset)
		);
	}

	fn opcode(&self) -> OpCode {
		OpCode::JSR
	}
}
