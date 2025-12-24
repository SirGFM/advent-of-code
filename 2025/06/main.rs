use std::io::{self, BufRead};

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();
	let mut result_1: u64 = 0;
	let mut result_2: u64 = 0;

	let cmds: Vec::<&str>;
	let mut values = Vec::<Vec<u64>>::new();

	let mut full = Vec::<u8>::new();
	let mut line_len: usize;

	loop {
		handle.read_line(&mut buf).unwrap();

		// Store the entire input for part 2.
		let full_line = buf.trim_end_matches('\n').trim_end_matches('\r');
		line_len = full_line.len();
		for b in full_line.as_bytes() {
			full.push(*b);
		}

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

	// Part 2:

	// Rotate the input,
	// causing the operation to be on the first line
	// of each group.
	let mut rotated = vec![0 as u8; full.len()];
	let w = line_len;
	let h = full.len() / w;
	for i in 0..full.len() {
		let y = i / w;
		let x = i % w;

		rotated[y + x * h] = full[i];
	}

	let mut cur: u64 = 0;
	let mut cmd: u8 = 0;

	let (w, h) = (h, w);
	for y in 0..h {
		let i = y * w;
		let mut j = i + w;

		// Detect the first line by checking for the operator.
		let tmp = rotated[j - 1];
		if tmp == b'*' || tmp == b'+' {
			j -= 1;
			cmd = tmp;

			match cmd {
				b'*' => cur = 1,
				b'+' => cur = 0,
				_ => panic!("impossible match"),
			}
		}

		let mut tmp: u64 = 0;
		str::from_utf8(&rotated[i..j])
			.unwrap()
			.trim()
			.split_ascii_whitespace()
			.for_each(|x| {
				tmp = x.parse::<u64>().unwrap();

				match cmd {
					b'*' => cur *= tmp,
					b'+' => cur += tmp,
					_ => panic!("invalid cmd"),
				}
			});

		// If no number was found, then this is an empty line.
		// Thus, accumulate the result.
		if tmp == 0 {
			result_2 += cur;
			cur = 0;
		}
	}
	result_2 += cur;

	println!("part 1: {}", result_1);
	println!("part 2: {}", result_2);
}
