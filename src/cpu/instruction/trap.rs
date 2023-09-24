use crate::cpu::{
	register::Register,
	CPU,
};
use crate::memory::MEMORY;
use std::io::{self, Write};
use super::Instruction;
use super::opcode::OpCode;

#[derive(Debug)]
pub struct Trap {
	trapvect: u16,
}

impl Trap {
	pub fn new(trapvect: u16) -> Self {
		Self {
			trapvect,
		}
	}
}

impl Instruction for Trap {
	fn execute(&self) {
		CPU.write(Register::R7, CPU.read(Register::PC));
		match self.trapvect {
			0x20 => handle_trap_getc(),	/* get character but not echo it */
			0x21 => handle_trap_out(),		/* output a character */
			0x22 => handle_trap_puts(),	/* output a word string */
			0x23 => handle_trap_in(),		/* get character and echo it */
			0x24 => handle_trap_putsp(),	/* output a byte string */
			0x25 => halt(),				/* halt the vm */
			_ => unreachable!(),
		}
	}

	fn opcode(&self) -> OpCode {
		OpCode::TRAP
	}
}

fn handle_trap_getc() {
	let ch;
	unsafe {
		ch = libc::getchar() as u16;
	}
	CPU.write(Register::R0, ch);
	CPU.update_condition_reg(ch);
}

fn handle_trap_out() {
	let ch = CPU.read(Register::R0) & 0xff;
	print!("{}", char::from(ch as u8));
	let _ = io::stdout().flush();
}

fn handle_trap_puts() {
	let start_addr = CPU.read(Register::R0);
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

fn handle_trap_in() {
	handle_trap_getc();
	handle_trap_out();
}

fn handle_trap_putsp() {
	let start_addr = CPU.read(Register::R0);
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

fn halt() {
	println!("HALT");
	CPU.halt();
}
