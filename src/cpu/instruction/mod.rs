pub mod opcode;

pub mod addr;
pub mod addi;
pub mod andr;
pub mod andi;
pub mod br;
pub mod jmp;
pub mod jsr;
pub mod jsrr;
pub mod ld;
pub mod ldi;
pub mod ldr;
pub mod lea;
pub mod not;
pub mod res;
pub mod rti;
pub mod st;
pub mod sti;
pub mod str;
pub mod trap;

use opcode::OpCode;

pub trait Instruction {
	fn execute(&self);
	fn opcode(&self) -> OpCode;
}
