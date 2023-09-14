use crate::cpu::CPU;
use crate::memory::MEMORY;
use ctrlc;
use lazy_static::*;
use std::{
	io,
	process::exit,
	os::fd::AsRawFd,
	sync::{Arc, Mutex},
};
use termios::*;

#[derive(Debug)]
struct VmInner {
	old_tio: Termios,
	new_tio: Termios,
}

#[derive(Debug)]
pub struct Vm {
	inner: Arc<Mutex<VmInner>>,
}

lazy_static! {
	pub static ref VM: Vm = Vm::new();
}

impl VmInner {
	fn new() -> Self {
		let old_tio = Termios::from_fd(io::stdin().as_raw_fd())
				.expect("failed to get terminal I/O structure");
		let mut new_tio = old_tio;
		new_tio.c_lflag &= !ICANON & !ECHO;

		Self {
			old_tio,
			new_tio
		}
	}
}

impl Vm {
	fn new() -> Self {
		Self {
			inner: Arc::new(Mutex::new(VmInner::new())),
		}
	}

	fn load(&self, byte_stream: Vec<u8>) {
		// to little endian
		let stream_u16 = byte_stream
			.chunks(2)
			.map(|two_bytes| {
				let big_endian_low = two_bytes[1] as u16;
				let big_endian_high = two_bytes[0] as u16;
				(big_endian_high << 8) | big_endian_low
			})
			.collect::<Vec<_>>();

		// copy u16s from [start, end) into MEMORY
		let start = stream_u16[0];
		stream_u16
			.iter()
			.enumerate()
			.skip(1)
			.for_each(|(idx, &data)| {
				MEMORY.write(start + idx as u16 - 1, data);
			});
	}

	pub fn init(&self, byte_stream: Vec<u8>) {
		// load byte stream into memory
		self.load(byte_stream);

		// initialize terminal
		Vm::disable_input_buffering();
	}

	fn disable_input_buffering() {
		let _ = tcsetattr(
			io::stdin().as_raw_fd(),
			TCSANOW,
			&VM
				.inner
				.lock()
				.unwrap()
				.new_tio,
		);
	}
	
	fn restore_input_buffering() {
		let _ = tcsetattr(
			io::stdin().as_raw_fd(),
			TCSANOW,
			&VM
				.inner
				.lock()
				.unwrap()
				.old_tio,
		);
	}

	fn handle_interrupt() {
		Vm::restore_input_buffering();
		println!();
		exit(-2);
	}

	pub fn run(&self) {
		// restore input buffering when SIGINT toggled
		ctrlc::set_handler(move || {
			Vm::handle_interrupt();
		}).expect("failed to set Ctrl-C handler");

		while CPU.is_running() {
			let raw_instr = CPU.fetch();
			let instr = CPU.decode(raw_instr);
			CPU.execute(instr);
		}

		// shutdown
		self.deinit();
	}

	fn deinit(&self) {
		Vm::restore_input_buffering();
	}
}
