use crate::{
	cpu::instruction::*,
	memory::MEMORY,
	optional_utils::summary::SUMMARY,
	parse::ARGS,
};
use instruction::Instruction;
use lazy_static::*;
use register::Register;
use std::{
	sync::{Arc, Mutex},
	time::Instant,
};

pub mod instruction;
pub mod register;

const REG_COUNT: usize = 11;

#[derive(Debug)]
struct CpuInner {
	regs: [u16; REG_COUNT],
	running: bool,
}

#[derive(Debug)]
pub struct Cpu {
	inner: Arc<Mutex<CpuInner>>,
}

lazy_static! {
	pub static ref CPU: Cpu = Cpu::new();
}

impl CpuInner {
	fn new() -> Self {
		Self {
			regs: [0, 0, 0, 0, 0, 0, 0, 0, 0x3000, 0, 0],
			running: true,
		}
	}
}

impl Cpu {
	fn new() -> Self {
		Self {
			inner: Arc::new(Mutex::new(CpuInner::new())),
		}
	}

	pub fn read(&self, which: Register) -> u16 {
		self.inner
			.lock()
			.unwrap()
			.regs[which as usize]
	}

	pub fn write(&self, which: Register, data: u16) {
		self.inner
			.lock()
			.unwrap()
			.regs[which as usize] = data;
	}

	pub fn is_running(&self) -> bool {
		self.inner
			.lock()
			.unwrap()
			.running
	}

	pub fn halt(&self) {
		self.inner
			.lock()
			.unwrap()
			.running = false;
	}

	pub fn fetch(&self) -> u16 {
		let raw_instr = MEMORY.read(self.read(Register::PC));
		self.write(
			Register::PC,
			self.read(Register::PC).wrapping_add(1)
		);
		raw_instr
	}

	/// sign extend 'low' bits of 'data' to 16-bit integer
	fn sign_extend_16(data: u16, low: u16) -> u16 {
		let mut mask = 0_u16;
		for i in 0..low {
			mask |= 1 << i;
		}

		match (data >> (low - 1)) & 1 {
			0 => data & mask,
			1 => (data & mask) | (0xffff << low),
			_ => unreachable!(),
		}
	}

	/// zero extend 'low' bits of 'data' to 16-bit integer
	fn zero_extend_16(data: u16, low: u16) -> u16 {
		let mut mask = 0_u16;
		for i in 0..low {
			mask |= 1 << i;
		}
		data & mask
	}

	pub fn update_condition_reg(&self, result: u16) {
		if result == 0 {
			self.write(Register::Cond, 0b010);	/* zero */
			return;
		}

		match (result >> 15) & 1 {
			0 => self.write(Register::Cond, 0b001),	/* positive */
			1 => self.write(Register::Cond, 0b100),	/* negative */
			_ => unreachable!(),
		}
	}

	pub fn decode(&self, raw_instr: u16) -> Box<dyn Instruction> {
		let opcode = raw_instr >> 12;

		match opcode {
			0b0000 => Self::decode_br(raw_instr),
			0b0010 => Self::decode_ld(raw_instr),
			0b0001 => Self::decode_add(raw_instr),
			0b0011 => Self::decode_st(raw_instr),
			0b0100 => Self::decode_jsr(raw_instr),
			0b0101 => Self::decode_and(raw_instr),
			0b0110 => Self::decode_ldr(raw_instr),
			0b0111 => Self::decode_str(raw_instr),
			0b1000 => Self::decode_rti(raw_instr),
			0b1001 => Self::decode_not(raw_instr),
			0b1010 => Self::decode_ldi(raw_instr),
			0b1011 => Self::decode_sti(raw_instr),
			0b1100 => Self::decode_jmp(raw_instr),
			0b1101 => Self::decode_res(raw_instr),
			0b1110 => Self::decode_lea(raw_instr),
			0b1111 => Self::decode_trap(raw_instr),
			_ => unreachable!(),
		}
	}

	fn decode_br(raw_instr: u16) -> Box<dyn Instruction> {
		let n = ((raw_instr >> 11) & 1) == 1;
		let z = ((raw_instr >> 10) & 1) == 1;
		let p = ((raw_instr >> 9) & 1) == 1;
		let offset = Self::sign_extend_16(raw_instr, 9);

		Box::new(br::Br::new(n, z, p, offset))
	}

	fn decode_add(raw_instr: u16) -> Box<dyn Instruction> {
		let dest_reg = Register::from((raw_instr >> 9) & 0b111);
		let src_reg1 = Register::from((raw_instr >> 6) & 0b111);
		let imm_flag = ((raw_instr >> 5) & 1) == 1;

		if imm_flag == false {
			let src_reg2 = Register::from(raw_instr & 0b111);

			Box::new(
				addr::Addr::new(dest_reg, src_reg1, src_reg2)
			)
		} else {
			let imm = Self::sign_extend_16(raw_instr, 5);

			Box::new(
				addi::Addi::new(dest_reg, src_reg1, imm)
			)
		}
	}

	fn decode_ld(raw_instr: u16) -> Box<dyn Instruction> {
		let dr = Register::from((raw_instr >> 9) & 0b111);
		let offset = Self::sign_extend_16(raw_instr, 9);

		Box::new(ld::Ld::new(dr, offset))
	}

	fn decode_st(raw_instr: u16) -> Box<dyn Instruction> {
		let sr = Register::from((raw_instr >> 9) & 0b111);
		let offset = Self::sign_extend_16(raw_instr, 9);

		Box::new(st::St::new(sr, offset))
	}

	fn decode_jsr(raw_instr: u16) -> Box<dyn Instruction> {
		let imm_flag: bool = ((raw_instr >> 11) & 1) == 1;

		if imm_flag == true {
			let offset = Self::sign_extend_16(raw_instr, 11);
			Box::new(jsr::Jsr::new(offset))
		} else {
			let base_reg = Register::from((raw_instr >> 6) & 0b111);
			Box::new(jsrr::Jsrr::new(base_reg))
		}
	}

	fn decode_and(raw_instr: u16) -> Box<dyn Instruction> {
		let dr = Register::from((raw_instr >> 9) & 0b111);
		let sr1 = Register::from((raw_instr >> 6) & 0b111);
		let imm_flag = ((raw_instr >> 5) & 1) == 1;

		if imm_flag == false {
			let sr2 = Register::from(raw_instr & 0b111);
			Box::new(andr::Andr::new(dr, sr1, sr2))
		} else {
			let imm = Self::sign_extend_16(raw_instr, 5);
			Box::new(andi::Andi::new(dr, sr1, imm))
		}
	}

	fn decode_ldr(raw_instr: u16) -> Box<dyn Instruction> {
		let dr = Register::from((raw_instr >> 9) & 0b111);
		let base_reg = Register::from((raw_instr >> 6) & 0b111);
		let offset = Self::sign_extend_16(raw_instr, 6);

		Box::new(ldr::Ldr::new(dr, base_reg, offset))
	}

	fn decode_str(raw_instr: u16) -> Box<dyn Instruction> {
		let sr = Register::from((raw_instr >> 9) & 0b111);
		let base_reg = Register::from((raw_instr >> 6) & 0b111);
		let offset = Self::sign_extend_16(raw_instr, 6);

		Box::new(str::Str::new(sr, base_reg, offset))
	}

	fn decode_rti(_raw_instr: u16) -> Box<dyn Instruction> {
		Box::new(rti::Rti::new())
	}

	fn decode_not(raw_instr: u16) -> Box<dyn Instruction> {
		let dr = Register::from((raw_instr >> 9) & 0b111);
		let sr = Register::from((raw_instr >> 6) & 0b111);

		Box::new(not::Not::new(dr, sr))
	}

	fn decode_ldi(raw_instr: u16) -> Box<dyn Instruction> {
		let dr = Register::from((raw_instr >> 9) & 0b111);
		let offset = Self::sign_extend_16(raw_instr, 9);

		Box::new(ldi::Ldi::new(dr, offset))
	}

	fn decode_sti(raw_instr: u16) -> Box<dyn Instruction> {
		let sr = Register::from((raw_instr >> 9) & 0b111);
		let offset = Self::sign_extend_16(raw_instr, 9);

		Box::new(sti::Sti::new(sr, offset))
	}

	fn decode_jmp(raw_instr: u16) -> Box<dyn Instruction> {
		let base_reg = Register::from((raw_instr >> 6) & 0b111);

		Box::new(jmp::Jmp::new(base_reg))
	}

	fn decode_res(_raw_instr: u16) -> Box<dyn Instruction> {
		Box::new(res::Res::new())
	}

	fn decode_lea(raw_instr: u16) -> Box<dyn Instruction> {
		let dr = Register::from((raw_instr >> 9) & 0b111);
		let offset = Self::sign_extend_16(raw_instr, 9);

		Box::new(lea::Lea::new(dr, offset))
	}

	fn decode_trap(raw_instr: u16) -> Box<dyn Instruction> {
		let trapvect = Self::zero_extend_16(raw_instr, 8);

		Box::new(trap::Trap::new(trapvect))
	}

	pub fn execute(&self, instr: Box<dyn Instruction>) {
		let mut begin = None;
		if ARGS.summary() == true {
			begin = Some(Instant::now());
		}

		instr.execute();

		let mut end = None;
		if ARGS.summary() == true {
			end = Some(Instant::now());
		}

		if ARGS.summary() == true {
			SUMMARY.add_record(
				instr.opcode(), 1, end.unwrap() - begin.unwrap()
			);
		}
	}
}
