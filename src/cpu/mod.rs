use crate::memory::MEMORY;
use crate::optional_utils::summary::SUMMARY;
use crate::parse::ARGS;
use instruction::{Instruction, OpCode};
use lazy_static::*;
use libc;
use register::Register;
use std::{
	io::{self, Write},
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

	fn read(&self, which: Register) -> u16 {
		self.inner
			.lock()
			.unwrap()
			.regs[which as usize]
	}

	fn write(&self, which: Register, data: u16) {
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

	fn update_condition_reg(&self, result: u16) {
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

	pub fn decode(&self, raw_instr: u16) -> Instruction {
		let opcode = raw_instr >> 12;

		match opcode {
			0b0000 => Cpu::decode_br(raw_instr),
			0b0001 => Cpu::decode_add(raw_instr),
			0b0010 => Cpu::decode_ld(raw_instr),
			0b0011 => Cpu::decode_st(raw_instr),
			0b0100 => Cpu::decode_jsr(raw_instr),
			0b0101 => Cpu::decode_and(raw_instr),
			0b0110 => Cpu::decode_ldr(raw_instr),
			0b0111 => Cpu::decode_str(raw_instr),
			0b1000 => Cpu::decode_rti(raw_instr),
			0b1001 => Cpu::decode_not(raw_instr),
			0b1010 => Cpu::decode_ldi(raw_instr),
			0b1011 => Cpu::decode_sti(raw_instr),
			0b1100 => Cpu::decode_jmp(raw_instr),
			0b1101 => Cpu::decode_res(raw_instr),
			0b1110 => Cpu::decode_lea(raw_instr),
			0b1111 => Cpu::decode_trap(raw_instr),
			_ => unreachable!(),
		}
	}

	fn decode_br(raw_instr: u16) -> Instruction {
		let n = ((raw_instr >> 11) & 1) == 1;
		let z = ((raw_instr >> 10) & 1) == 1;
		let p = ((raw_instr >> 9) & 1) == 1;
		let offset = Self::sign_extend_16(raw_instr, 9);

		Instruction::new(
			OpCode::BR,
			None,
			[None, None, None],
			Some(offset),
			[Some(n), Some(z), Some(p)],
		)
	}

	fn decode_add(raw_instr: u16) -> Instruction {
		let dest_reg = Register::from((raw_instr >> 9) & 0b111);
		let src_reg1 = Register::from((raw_instr >> 6) & 0b111);
		let imm_flag = ((raw_instr >> 5) & 1) == 1;

		if imm_flag == false {
			let src_reg2 = Register::from(raw_instr & 0b111);

			Instruction::new(
				OpCode::ADDR,
				Some(imm_flag),
				[Some(dest_reg), Some(src_reg1), Some(src_reg2)],
				None,
				[None, None, None],
			)
		} else {
			let imm = Self::sign_extend_16(raw_instr, 5);

			Instruction::new(
				OpCode::ADDI,
				Some(imm_flag),
				[Some(dest_reg), Some(src_reg1), None],
				Some(imm),
				[None, None, None],
			)
		}
	}

	fn decode_ld(raw_instr: u16) -> Instruction {
		let dr = Register::from((raw_instr >> 9) & 0b111);
		let offset = Self::sign_extend_16(raw_instr, 9);

		Instruction::new(
			OpCode::LD,
			None,
			[Some(dr), None, None],
			Some(offset),
			[None, None, None],
		)
	}

	fn decode_st(raw_instr: u16) -> Instruction {
		let sr = Register::from((raw_instr >> 9) & 0b111);
		let offset = Self::sign_extend_16(raw_instr, 9);

		Instruction::new(
			OpCode::ST,
			None,
			[Some(sr), None, None],
			Some(offset),
			[None, None, None],
		)
	}

	fn decode_jsr(raw_instr: u16) -> Instruction {
		let imm_flag: bool = ((raw_instr >> 11) & 1) == 1;

		if imm_flag == true {
			let offset = Self::sign_extend_16(raw_instr, 11);
			Instruction::new(
				OpCode::JSR,
				Some(imm_flag),
				[None, None, None],
				Some(offset),
				[None, None, None],
			)
		} else {
			let base_reg = Register::from((raw_instr >> 6) & 0b111);
			Instruction::new(
				OpCode::JSRR,
				Some(imm_flag),
				[Some(base_reg), None, None],
				None,
				[None, None, None],
			)
		}
	}

	fn decode_and(raw_instr: u16) -> Instruction {
		let dr = Register::from((raw_instr >> 9) & 0b111);
		let sr1 = Register::from((raw_instr >> 6) & 0b111);
		let imm_flag = ((raw_instr >> 5) & 1) == 1;

		if imm_flag == false {
			let sr2 = Register::from(raw_instr & 0b111);
			Instruction::new(
				OpCode::ANDR,
				Some(imm_flag),
				[Some(dr), Some(sr1), Some(sr2)],
				None,
				[None, None, None],
			)
		} else {
			let imm = Self::sign_extend_16(raw_instr, 5);
			Instruction::new(
				OpCode::ANDI,
				Some(imm_flag),
				[Some(dr), Some(sr1), None],
				Some(imm),
				[None, None, None],
			)
		}
	}

	fn decode_ldr(raw_instr: u16) -> Instruction {
		let dr = Register::from((raw_instr >> 9) & 0b111);
		let base_reg = Register::from((raw_instr >> 6) & 0b111);
		let offset = Self::sign_extend_16(raw_instr, 6);

		Instruction::new(
			OpCode::LDR,
			None,
			[Some(dr), Some(base_reg), None],
			Some(offset),
			[None, None, None],
		)
	}

	fn decode_str(raw_instr: u16) -> Instruction {
		let sr = Register::from((raw_instr >> 9) & 0b111);
		let base_reg = Register::from((raw_instr >> 6) & 0b111);
		let offset = Self::sign_extend_16(raw_instr, 6);

		Instruction::new(
			OpCode::STR,
			None,
			[Some(sr), Some(base_reg), None],
			Some(offset),
			[None, None, None],
		)
	}

	fn decode_rti(_raw_instr: u16) -> Instruction {
		Instruction::new(
			OpCode::RTI,
			None,
			[None, None, None],
			None,
			[None, None, None],
		)
	}

	fn decode_not(raw_instr: u16) -> Instruction {
		let dr = Register::from((raw_instr >> 9) & 0b111);
		let sr = Register::from((raw_instr >> 6) & 0b111);

		Instruction::new(
			OpCode::NOT,
			None,
			[Some(dr), Some(sr), None],
			None,
			[None, None, None],
		)
	}

	fn decode_ldi(raw_instr: u16) -> Instruction {
		let dr = Register::from((raw_instr >> 9) & 0b111);
		let offset = Self::sign_extend_16(raw_instr, 9);

		Instruction::new(
			OpCode::LDI,
			None,
			[Some(dr), None, None],
			Some(offset),
			[None, None, None]
		)
	}

	fn decode_sti(raw_instr: u16) -> Instruction {
		let sr = Register::from((raw_instr >> 9) & 0b111);
		let offset = Self::sign_extend_16(raw_instr, 9);

		Instruction::new(
			OpCode::STI,
			None,
			[Some(sr), None, None],
			Some(offset),
			[None, None, None],
		)
	}

	fn decode_jmp(raw_instr: u16) -> Instruction {
		let base_reg = Register::from((raw_instr >> 6) & 0b111);

		Instruction::new(
			OpCode::JMP,
			None,
			[Some(base_reg), None, None],
			None,
			[None, None, None],
		)
	}

	fn decode_res(_raw_instr: u16) -> Instruction {
		Instruction::new(
			OpCode::RTI,
			None,
			[None, None, None],
			None,
			[None, None, None],
		)
	}

	fn decode_lea(raw_instr: u16) -> Instruction {
		let dr = Register::from((raw_instr >> 9) & 0b111);
		let offset = Self::sign_extend_16(raw_instr, 9);

		Instruction::new(
			OpCode::LEA,
			None,
			[Some(dr), None, None],
			Some(offset),
			[None, None, None],
		)
	}

	fn decode_trap(raw_instr: u16) -> Instruction {
		let trapvect = Self::zero_extend_16(raw_instr, 8);

		Instruction::new(
			OpCode::TRAP,
			None,
			[None, None, None],
			Some(trapvect),
			[None, None, None],
		)
	}

	pub fn execute(&self, instr: Instruction) {
		let mut begin = None;
		if ARGS.summary() == true {
			begin = Some(Instant::now());
		}

		let opcode = instr.opcode();
		match opcode {
			OpCode::ADDR => self.execute_addr(instr),
			OpCode::ADDI => self.execute_addi(instr),
			OpCode::ANDR => self.execute_andr(instr),
			OpCode::ANDI => self.execute_andi(instr),
			OpCode::BR => self.execute_br(instr),
			OpCode::JMP => self.execute_jmp(instr),
			OpCode::JSR => self.execute_jsr(instr),
			OpCode::JSRR => self.execute_jsrr(instr),
			OpCode::LD => self.execute_ld(instr),
			OpCode::LDI => self.execute_ldi(instr),
			OpCode::LDR => self.execute_ldr(instr),
			OpCode::LEA => self.execute_lea(instr),
			OpCode::NOT => self.execute_not(instr),
			OpCode::RES => self.execute_res(instr),
			OpCode::RTI => self.execute_rti(instr),
			OpCode::ST => self.execute_st(instr),
			OpCode::STI => self.execute_sti(instr),
			OpCode::STR => self.execute_str(instr),
			OpCode::TRAP => self.handle_trap(instr),
			_ => unimplemented!(),
		}

		let mut end = None;
		if ARGS.summary() == true {
			end = Some(Instant::now());
		}

		if ARGS.summary() == true {
			SUMMARY.add_record(
				opcode, 1, end.unwrap() - begin.unwrap()
			);
		}
	}

	fn execute_addr(&self, instr: Instruction) {
		let regs = instr.regs();
		let (dr, sr1, sr2) = (
			regs[0].unwrap(),
			regs[1].unwrap(),
			regs[2].unwrap(),
		);

		let result = self.read(sr1).wrapping_add(self.read(sr2));
		self.write(dr, result);
		self.update_condition_reg(result);
	}

	fn execute_addi(&self, instr: Instruction) {
		let regs = instr.regs();
		let (dr, sr1, imm) = (
			regs[0].unwrap(),
			regs[1].unwrap(),
			instr.imm().unwrap(),
		);

		let result = self.read(sr1).wrapping_add(imm);
		self.write(dr, result);
		self.update_condition_reg(result);
	}

	fn execute_andr(&self, instr: Instruction) {
		let regs = instr.regs();
		let (dr, sr1, sr2) = (
			regs[0].unwrap(),
			regs[1].unwrap(),
			regs[2].unwrap(),
		);

		let result = self.read(sr1) & self.read(sr2);
		self.write(dr, result);
		self.update_condition_reg(result);
	}

	fn execute_andi(&self, instr: Instruction) {
		let regs = instr.regs();
		let (dr, sr1) = (regs[0].unwrap(), regs[1].unwrap());
		let imm = instr.imm().unwrap();

		let result = self.read(sr1) & imm;
		self.write(dr, result);
		self.update_condition_reg(result);
	}

	fn execute_br(&self, instr: Instruction) {
		let cond = self.read(Register::Cond);
		let nzp = instr.nzp();
		let (n, z, p) = (
			nzp[0].unwrap(),
			nzp[1].unwrap(),
			nzp[2].unwrap(),
		);
		let offset = instr.imm().unwrap();

		if (n && (cond == 1 << 2)) || (z && (cond == 1 << 1)) || (p && (cond == 1 << 0)) {
			self.write(
				Register::PC,
				self.read(Register::PC).wrapping_add(offset)
			);
		}
	}

	fn execute_jmp(&self, instr: Instruction) {
		let target_addr = self.read(
			instr.regs()[0].unwrap()
		);

		self.write(Register::PC, target_addr);
	}

	fn execute_jsr(&self, instr: Instruction) {
		self.write(Register::R7, self.read(Register::PC));
		let offset = instr.imm().unwrap();
		self.write(
			Register::PC,
			self.read(Register::PC).wrapping_add(offset)
		);
	}

	fn execute_jsrr(&self, instr: Instruction) {
		let base_reg = instr.regs()[0].unwrap();
		let target = self.read(base_reg);
		self.write(Register::R7, self.read(Register::PC));
		self.write(Register::PC, target);
	}

	fn execute_ld(&self, instr: Instruction) {
		let dr = instr.regs()[0].unwrap();
		let offset = instr.imm().unwrap();
		let target_addr = self.read(Register::PC).wrapping_add(offset);
		let data = MEMORY.read(target_addr);
		self.write(dr, data);
		self.update_condition_reg(data);
	}

	fn execute_ldi(&self, instr: Instruction) {
		let target_addr = MEMORY.read(
			self.read(Register::PC).wrapping_add(instr.imm().unwrap())
		);

		let result = MEMORY.read(target_addr);
		
		let dr = instr.regs()[0].unwrap();
		self.write(dr, result);

		self.update_condition_reg(result);
	}

	fn execute_ldr(&self, instr: Instruction) {
		let regs = instr.regs();
		let (dr, base_reg) = (
			regs[0].unwrap(),
			regs[1].unwrap()
		);
		let offset = instr.imm().unwrap();
		let data = MEMORY.read(
			self.read(base_reg).wrapping_add(offset)
		);
		self.write(dr, data);
		self.update_condition_reg(data);
	}

	fn execute_lea(&self, instr: Instruction) {
		let dr = instr.regs()[0].unwrap();
		let offset = instr.imm().unwrap();
		let result = self.read(Register::PC).wrapping_add(offset);
		self.write(dr, result);
		self.update_condition_reg(result);
	}

	fn execute_not(&self, instr: Instruction) {
		let regs = instr.regs();
		let (dr, sr) = (regs[0].unwrap(), regs[1].unwrap());
		let result = !self.read(sr);
		self.write(dr, result);
		self.update_condition_reg(result);
	}

	fn execute_res(&self, _instr: Instruction) {
		unimplemented!("This operation isn't allowed in vlc3");
	}

	fn execute_rti(&self, _instr: Instruction) {
		unimplemented!("This operation isn't allowed in vlc3");
	}

	fn execute_st(&self, instr: Instruction) {
		let sr = instr.regs()[0].unwrap();
		let offset = instr.imm().unwrap();
		let addr = self.read(Register::PC).wrapping_add(offset);
		MEMORY.write(addr, self.read(sr));
	}

	fn execute_sti(&self, instr: Instruction) {
		let sr = instr.regs()[0].unwrap();
		let offset = instr.imm().unwrap();

		let target_addr = MEMORY
			.read(self.read(Register::PC))
			.wrapping_add(offset);

		MEMORY.write(target_addr, self.read(sr));
	}

	fn execute_str(&self, instr: Instruction) {
		let regs = instr.regs();
		let (sr, base_reg) = (
			regs[0].unwrap(),
			regs[1].unwrap(),
		);
		let offset = instr.imm().unwrap();

		let addr = self.read(base_reg).wrapping_add(offset);
		MEMORY.write(addr, self.read(sr));
	}

	fn handle_trap(&self, instr: Instruction) {
		self.write(Register::R7, self.read(Register::PC));
		let trapvect = instr.imm().unwrap();

		match trapvect {
			0x20 => self.handle_trap_getc(),	/* get character but not echo it */
			0x21 => self.handle_trap_out(),		/* output a character */
			0x22 => self.handle_trap_puts(),	/* output a word string */
			0x23 => self.handle_trap_in(),		/* get character and echo it */
			0x24 => self.handle_trap_putsp(),	/* output a byte string */
			0x25 => self.halt(),				/* halt the vm */
			_ => unreachable!(),
		}
	}

	fn handle_trap_getc(&self) {
		let ch;
		unsafe {
			ch = libc::getchar() as u16;
		}
		self.write(Register::R0, ch);
		self.update_condition_reg(ch);
	}

	fn handle_trap_out(&self) {
		let ch = self.read(Register::R0) & 0xff;
		print!("{}", char::from(ch as u8));
		let _ = io::stdout().flush();
	}

	fn handle_trap_puts(&self) {
		let start_addr = self.read(Register::R0);
		let s = (start_addr..)
			.take_while(|&addr| {
				0 != MEMORY.read(addr)
			})
			.map(|addr| {
				let ch = MEMORY.read(addr);
				char::from(ch as u8)
			})
			.collect::<String>();
		print!("{s}");
		let _ = io::stdout().flush();
	}

	fn handle_trap_in(&self) {
		self.handle_trap_getc();
		self.handle_trap_out();
	}

	fn handle_trap_putsp(&self) {
		let start_addr = self.read(Register::R0);
		let s = (start_addr..)
			.take_while(|&addr| 0 != {
				MEMORY.read(addr)
			})
			.map(|num| {
				let c1 = char::from((num & 0xff) as u8);
				let c2 = char::from((num >> 8) as u8);
				format!("{}{}", c1.to_string(), c2.to_string())
			})
			.collect::<String>();
		print!("{s}");
		let _ = io::stdout().flush();
	}

	fn halt(&self) {
		println!("HALT");
		self.inner
			.lock()
			.unwrap()
			.running = false;
	}
}
