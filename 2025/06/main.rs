use std::io::{self, BufRead};

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let cmds: Vec::<&str>;
	let mut buf = String::new();
	let mut result_1: u64 = 0;
	let mut result_2: u64 = 0;

	let mut values = Vec::<Vec<u64>>::new();

	loop {
		handle.read_line(&mut buf).unwrap();
		let line = buf.trim();

		if line.as_bytes()[0] < b'0' || line.as_bytes()[0] > b'9' {
			cmds = line
				.split_ascii_whitespace()
				.collect();
			break;
		}

		let nums: Vec<_> = line
			.split_ascii_whitespace()
			.map(|x| x.parse::<u64>().unwrap())
			.collect();

		values.push(nums);

		buf.clear();
	}

	for i in 0..values[0].len() {
		for j in 1..values.len() {
			match cmds[i] {
				"+" => values[0][i] += values[j][i],
				"*" => values[0][i] *= values[j][i],
				_ => panic!("invalid op"),
			}
		}
	}

	for i in 0..values[0].len() {
		result_1 += values[0][i];
	}

	println!("part 1: {}", result_1);
	println!("part 2: {}", result_2);
}
