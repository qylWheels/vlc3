use enum_iterator::Sequence;
use super::register::Register;

#[derive(Clone, Copy, Debug, Sequence)]
pub enum OpCode {
	ADDR,
	ADDI,
	ANDR,
	ANDI,
	BR,
	JMP,
	JSR,
	JSRR,
	LD,
	LDI,
	LDR,
	LEA,
	NOT,
	RES,	/* reserved, unused */
	RET,
	RTI,	/* unused */
	ST,
	STI,
	STR,
	TRAP,
}

impl From<OpCode> for String {
	fn from(value: OpCode) -> String {
		match value {
			OpCode::ADDR => String::from("ADDR"),
			OpCode::ADDI => String::from("ADDI"),
			OpCode::ANDR => String::from("ANDR"),
			OpCode::ANDI => String::from("ANDI"),
			OpCode::BR => String::from("BR"),
			OpCode::JMP => String::from("JMP"),
			OpCode::JSR => String::from("JSR"),
			OpCode::JSRR => String::from("JSRR"),
			OpCode::LD => String::from("LD"),
			OpCode::LDI => String::from("LDI"),
			OpCode::LDR => String::from("LDR"),
			OpCode::LEA => String::from("LEA"),
			OpCode::NOT => String::from("NOT"),
			OpCode::RES => String::from("RES"),
			OpCode::RET => String::from("RET"),
			OpCode::RTI => String::from("RTI"),
			OpCode::ST => String::from("ST"),
			OpCode::STI => String::from("STI"),
			OpCode::STR => String::from("STR"),
			OpCode::TRAP => String::from("TRAP"),
		}
	}
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Instruction {
	opcode: OpCode,
	imm_flag: Option<bool>,
	regs: [Option<Register>; 3],
	imm: Option<u16>,
	nzp: [Option<bool>; 3],
}

#[allow(dead_code)]
impl Instruction {
	pub fn new(
		opcode: OpCode,
		imm_flag: Option<bool>,
		regs: [Option<Register>; 3],
		imm: Option<u16>,
		nzp: [Option<bool>; 3],
	) -> Self {
		Self { opcode, imm_flag, regs, imm, nzp }
	}

	pub fn opcode(&self) -> OpCode {
		self.opcode
	}

	pub fn imm_flag(&self) -> Option<bool> {
		self.imm_flag
	}

	pub fn regs(&self) -> [Option<Register>; 3] {
		self.regs
	}

	pub fn imm(&self) -> Option<u16> {
		self.imm
	}

	pub fn nzp(&self) -> [Option<bool>; 3] {
		self.nzp
	}
}

