use std::io::{self, BufRead};

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();
	let mut result_1: u64 = 0;

	loop {
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}
		let line = buf.trim().as_bytes();

		let mut first: u64 = 0;
		let mut second: u64 = 0;

		for i in 0..line.len() {
			let digit = (line[i] - b'0') as u64;

			if i != line.len() - 1 && digit > first {
				first = digit;
				second = 0;
			} else if digit > second {
				second = digit;
			}
		}

		let found = first * 10 + second;
		result_1 += found;

		buf.clear();
	}

	println!("part 1: {}", result_1);
}
