use crate::cpu::instruction::opcode::OpCode;
use enum_iterator::all;
use lazy_static::*;
use std::{
	sync::{Arc, Mutex},
	time::Duration,
};

#[derive(Debug)]
struct ExecuteInfo {
	opcode: OpCode,
	times: usize,
	cost: Duration,
}

#[derive(Debug)]
struct SummaryInner {
	record: Vec<ExecuteInfo>,
}

#[derive(Debug)]
pub struct Summary {
	inner: Arc<Mutex<SummaryInner>>,
}

lazy_static! {
	pub static ref SUMMARY: Summary = Summary::new();
}

impl ExecuteInfo {
	fn new(opcode: OpCode, times: usize, cost: Duration) -> Self {
		Self {
			opcode, times, cost
		}
	}
}

impl SummaryInner {
	fn new() -> Self {
		Self {
			record: all::<OpCode>()
				.map(|opcode| {
					ExecuteInfo::new(opcode, 0, Duration::from_secs(0))
				})
				.collect::<Vec<_>>()
		}
	}
}

impl Summary {
	fn new() -> Self {
		Self {
			inner: Arc::new(Mutex::new(SummaryInner::new())),
		}
	}

	pub fn add_record(&self, opcode: OpCode, times: usize, cost: Duration) {
		let info = &mut self
			.inner
			.lock()
			.unwrap()
			.record[opcode as usize];
		info.times += times;
		info.cost += cost;
	}

	pub fn print_summary(&self) {
		println!("{:>20}{:>15}{:>15}", "Operation Type", "Calls", "Time");
		println!("{}", "-".repeat(60));

		let records = &self.inner.lock().unwrap().record;

		for record in records {
			println!(
				"{:>20}{:>15}{:>15}",
				String::from(record.opcode),
				record.times,
				format!("{} ms", record.cost.as_millis()),
			);
		}

		println!("{}", "-".repeat(60));

		println!(
			"{:>10}{:>10}{:>15}{:>15}",
			"Summary",
			"---",
			format!(
				"{}", records
					.iter()
					.map(|info| info.times)
					.sum::<usize>()
			),
			format!(
				"{} ms", records
					.iter()
					.map(|info| info.cost.as_millis())
					.sum::<u128>()
			),
		);
	}
}
