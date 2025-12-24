use std::io::{self, BufRead};

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();
	let mut result_1: u64 = 0;
	let mut result_2: u64 = 0;

	loop {
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}
		let input: Vec<_> = buf
			.trim()
			.split(" ")
			.collect();

		result_1 += solve(&input);

		buf.clear();
	}

	println!("part 1: {}", result_1);
	println!("part 2: {}", result_2);
}

fn solve(input: &Vec<&str>) -> u64 {
	let target = get_target(input[0]);

	let mut cmds = Vec::<u32>::new();
	for value in &input[1..input.len()-1] {
		cmds.push(get_cmd(value));
	}

	let mut attempts: u64 = 1;
	let mut states = vec![0; 1];
	loop {
		let mut tmp = Vec::<u32>::new();

		for state in states {
			for cmd in &cmds {
				let new_state = state ^ *cmd;
				if new_state == target {
					return attempts;
				}

				tmp.push(new_state);
			}
		}

		states = tmp;
		attempts += 1;
	}
}

fn get_target(input: &str) -> u32 {
	let input = &input[1..input.len()-1];

	let mut target: u32 = 0;
	let mut idx: u32 = 0;

	for b in input.as_bytes() {
		if *b == b'#' {
			target |= 1 << idx;
		}

		idx += 1;
	}

	return target;
}

fn get_cmd(input: &str) -> u32 {
	let input = &input[1..input.len()-1];
	let mut bitmask: u32 = 0;

	input
		.split(",")
		.for_each(|x| {
			let value = x.parse::<u32>().unwrap();

			bitmask |= 1 << value;
		});

	return bitmask;
}
