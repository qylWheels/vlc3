use optional_utils::summary::SUMMARY;
use parse::ARGS;
use std::fs;
use vm::VM;

mod cpu;
mod memory;
mod optional_utils;
mod parse;
mod vm;

fn main() {
	// parse
	ARGS.parse();
	let path = ARGS.path().unwrap();

	// vm, run!
	match fs::read(&path) {
		Ok(byte_stream) => VM.init(byte_stream),
		Err(e) => panic!(
			"An error occured when opening file: {}({})",
			path,
			e,
		),
	}
	VM.run();

	// print summary if relative option is specified
	if ARGS.summary() {
		SUMMARY.print_summary();
	}
}
