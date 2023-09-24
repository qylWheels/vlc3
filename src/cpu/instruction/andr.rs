use crate::cpu::{
	register::Register,
	CPU,
};
use super::Instruction;
use super::opcode::OpCode;

pub struct Andr {
	dr: Register,
	sr1: Register,
	sr2: Register,
}

impl Andr {
	pub fn new(dr: Register, sr1: Register, sr2: Register) -> Self {
		Self {
			dr, sr1, sr2,
		}
	}
}

impl Instruction for Andr {
	fn execute(&self) {
		let result = CPU.read(self.sr1) & CPU.read(self.sr2);
		CPU.write(self.dr, result);
		CPU.update_condition_reg(result);
	}

	fn opcode(&self) -> OpCode {
		OpCode::ANDR
	}
}
