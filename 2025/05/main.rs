use std::io::{self, BufRead};

struct Range {
	min: u64,
	max: u64,
}

impl Range{
	fn new(min: u64, max: u64) -> Range {
		Range{
			min: min,
			max: max,
		}
	}

	fn is_fresh(&self, value: u64) -> bool {
		return value >= self.min && value <= self.max;
	}
}

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();
	let mut ranges = Vec::<Range>::new();

	let mut result_1: u64 = 0;

	loop {
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}
		let line = buf.trim();

		if line.len() == 0 {
			break;
		}

		let range: Vec<_> = line.split("-").collect();
		ranges.push(Range::new(range[0].parse::<u64>().unwrap(), range[1].parse::<u64>().unwrap()));

		buf.clear();
	}
	buf.clear();

	loop {
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}
		let line = buf.trim();

		let value = line.parse::<u64>().unwrap();
		for range in &ranges {
			if range.is_fresh(value) {
				result_1 += 1;
				break;
			}
		}

		buf.clear();
	}

	println!("part 1: {}", result_1);
}
