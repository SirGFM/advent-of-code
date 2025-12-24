use std::io::{self, BufRead};

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();
	let mut result_1: u64 = 0;
	let mut result_2: u64 = 0;

	let mut input = Vec::<u8>::new();
	let mut w: usize = 0;

	loop {
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}

		let line = buf.trim().as_bytes();
		w = line.len();

		for b in line {
			input.push(*b)
		}

		buf.clear();
	}

	for i in w..input.len() {
		match input[i-w] {
			b'S' => input[i] = b'|',
			b'|' => {
				match input[i] {
					b'.' => input[i] = b'|',
					b'^' => {
						let x = i % w;

						if x > 0 {
							input[i-1] = b'|';
						}
						if x < w {
							input[i+1] = b'|';
						}
					},
					_ => {},
				}
			},
			_ => {},
		}
	}

	for i in w..input.len() {
		if input[i] == b'^' && input[i-w] == b'|' {
			result_1 += 1;
		}
	}

	println!("part 1: {}", result_1);
	println!("part 2: {}", result_2);
}
