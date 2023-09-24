use enum_iterator::Sequence;

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
