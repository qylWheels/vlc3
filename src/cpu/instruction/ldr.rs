use crate::cpu::{
	register::Register,
	CPU,
};
use crate::memory::MEMORY;
use super::Instruction;
use super::opcode::OpCode;

pub struct Ldr {
	dr: Register,
	base_reg: Register,
	offset: u16,
}

impl Ldr {
	pub fn new(dr: Register, base_reg: Register, offset: u16) -> Self {
		Self {
			dr, base_reg, offset,
		}
	}
}

impl Instruction for Ldr {
	fn execute(&self) {
		let data = MEMORY.read(
			CPU.read(self.base_reg).wrapping_add(self.offset)
		);
		CPU.write(self.dr, data);
		CPU.update_condition_reg(data);
	}

	fn opcode(&self) -> OpCode {
		OpCode::LDR
	}
}
