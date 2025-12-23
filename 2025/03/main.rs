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
		let line = buf.trim().as_bytes();

		result_1 += calculate(&line, 2);
		result_2 += calculate(&line, 12);

		buf.clear();
	}

	println!("part 1: {}", result_1);
	println!("part 2: {}", result_2);
}

fn calculate(line: &[u8], num_digits: usize) -> u64 {
	let mut digits: Vec<usize> = Vec::<usize>::new();

	for i in 0..num_digits {
		digits.push(line.len() - (num_digits - 1 - i));
	}

	get_largest(&line, &mut digits, 0);
	return to_number(&line, &digits);
}

fn get_largest(line: &[u8], digits: &mut[usize], end: usize) {
	let mut i = digits[0];

	while i > end {
		let digit = line[digits[0] - 1];
		let cur = line[i - 1];

		if cur >= digit {
			digits[0] = i;

			if digits.len() > 1 {
				get_largest(&line, &mut digits[1..], i);
			}
		}

		i -= 1;
	}
}

fn to_number(line: &[u8], digits: &[usize]) -> u64 {
	let mut ret: u64 = 0;

	for i in digits {
		ret = ret * 10 + ((line[i - 1] - b'0') as u64);
	}

	return ret;
}
