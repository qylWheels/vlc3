use argparse::{
	ArgumentParser,
	StoreTrue,
	StoreOption,
};
use lazy_static::*;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct ArgumentInner {
	// path
	path: Option<String>,

	// options
	summary: bool,
}

#[derive(Debug)]
pub struct Argument {
	inner: Arc<Mutex<ArgumentInner>>,
}

lazy_static! {
	pub static ref ARGS: Argument = Argument::new();
}

impl ArgumentInner {
	fn new() -> Self {
		Self {
			path: None,
			summary: false,
		}
	}
}

impl Argument {
	fn new() -> Self {
		Self {
			inner: Arc::new(Mutex::new(ArgumentInner::new())),
		}
	}

	pub fn path(&self) -> Option<String> {
		self
			.inner
			.lock()
			.unwrap()
			.path
			.clone()
	}

	pub fn summary(&self) -> bool {
		self
			.inner
			.lock()
			.unwrap()
			.summary
	}

	pub(super) fn parse(&self) {
		let mut summary = false;
		let mut path = Some(String::new());

		// nmd, use braces to limit ArgumentParser's scope to
		// ensure that &mut summary and &mut path live long enouth!
		// RIDICULOUS!!!
		{
			let mut parser = ArgumentParser::new();
			parser.set_description("Emulate lc-3 environment.");
			parser.refer(&mut summary)
				.add_option(
					&["-s", "--summary"],
					StoreTrue,
					"Print summary after program exited"
				);
			
			parser.refer(&mut path).add_argument(
					"PROGRAM",
					StoreOption,
					"Path to program"
				);
			parser.parse_args_or_exit();
		}

		(*ARGS.inner.lock().unwrap()).summary = summary;
		(*ARGS.inner.lock().unwrap()).path = path;
	}
}
