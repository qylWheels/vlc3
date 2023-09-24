use crate::cpu::{
	register::Register,
	CPU,
};
use super::Instruction;
use super::opcode::OpCode;

pub struct Br {
	n: bool,
	z: bool,
	p: bool,
	offset: u16,
}

impl Br {
	pub fn new(n: bool, z: bool, p: bool, offset: u16) -> Self {
		Self {
			n, z, p, offset,
		}
	}
}

impl Instruction for Br {
	fn execute(&self) {
		let cond = CPU.read(Register::Cond);
		if (self.n && (cond == 1 << 2))
		|| (self.z && (cond == 1 << 1))
		|| (self.p && (cond == 1 << 0)) {
			CPU.write(
				Register::PC,
				CPU.read(Register::PC).wrapping_add(self.offset),
			);
		}
	}

	fn opcode(&self) -> OpCode {
		OpCode::BR
	}
}
