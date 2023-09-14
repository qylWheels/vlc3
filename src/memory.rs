use lazy_static::*;
use libc::{getchar, FD_SET, fd_set, timeval};
use std::{
	io,
	mem,
	os::fd::AsRawFd,
	sync::{Arc, Mutex},
};
use syscalls::{syscall, Sysno};

const MEMORY_SIZE: usize = 1 << 16;

#[derive(Debug)]
struct MemoryInner {
	mem: [u16; MEMORY_SIZE],
}

#[derive(Debug)]
pub struct Memory {
	inner: Arc<Mutex<MemoryInner>>,
}

lazy_static! {
	pub static ref MEMORY: Memory = Memory::new();
}

impl MemoryInner {
	fn new() -> Self {
		Self {
			mem: [0; MEMORY_SIZE],
		}
	}
}

impl Memory {
	fn new() -> Self {
		Self {
			inner: Arc::new(Mutex::new(MemoryInner::new())),
		}
	}

	fn check_key() -> bool {
		let mut readfds: fd_set;
		unsafe {
			readfds = mem::zeroed();
		}
		unsafe {
			FD_SET(io::stdin().as_raw_fd(), &mut readfds as *mut fd_set);
		}

		let mut timeout: timeval;
		unsafe {
			timeout = mem::zeroed();
		}
		timeout.tv_sec = 0;
		timeout.tv_usec = 0;

		let ret;
		unsafe {
			ret = syscall!(
				Sysno::select,
				1,
				&mut readfds as *mut fd_set,
				0,
				0,
				&mut timeout as *mut timeval
			).expect("failed to execute syscall");
		}
		ret != 0
	}

	pub fn read(&self, pos: u16) -> u16 {
		// check if is reading keyboard status register(a memory
		// mapped register)
		const MR_KBSR: u16 = 0xfe00;	// address of keyboard status register
		const MR_KBDR: u16 = 0xfe02;	// address of keyboard data register

		if pos == MR_KBSR {
			if Memory::check_key() {
				self.write(MR_KBSR, 1 << 15);
				self.write(MR_KBDR, unsafe { getchar() } as u16);
			} else {
				self.write(MR_KBSR, 0);
			}
		}

		self.inner
			.lock()
			.unwrap()
			.mem[pos as usize]
	}

	pub fn write(&self, pos: u16, data: u16) {
		self.inner
			.lock()
			.unwrap()
			.mem[pos as usize] = data;
	}
}
