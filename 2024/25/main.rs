use std::io::{self, BufRead};

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut tmp = Vec::<u8>::new();
	let mut buf = String::new();
	let mut line_len = 0;
	let mut num_lines = 0;

	let mut locks = Vec::<Vec<usize>>::new();
	let mut keys = Vec::<Vec<usize>>::new();

	loop {
		buf.clear();
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}

		let line = buf.trim();
		if line.len() == 0 {
			num_lines = tmp.len() / line_len;

			match parse_map(&tmp, line_len) {
				(true, lock) => locks.push(lock),
				(false, key) => keys.push(key),
			}

			tmp = Vec::<u8>::new();
		} else {
			line_len = line.len();
			for c in line.chars() {
				tmp.push(c as u8);
			}
		}
	}

	match parse_map(&tmp, line_len) {
		(true, lock) => locks.push(lock),
		(false, key) => keys.push(key),
	}

	let mut p1_result = 0;

	for lock in locks {
		for key in &keys {
			let mut i = 0;

			while i < key.len() {
				if key[i] + lock[i] > num_lines {
					break;
				}

				i += 1;
			}

			if i == key.len() {
				p1_result += 1;
			}
		}
	}

	println!("part 1: {}", p1_result);
}

fn parse_map(arr: &Vec<u8>, line_len: usize) -> (bool, Vec<usize>) {
	let mut ret = vec![0; line_len];

	for i in 0..arr.len() {
		if arr[i] == b'#' {
			ret[i % line_len] += 1;
		}
	}

	let mut lock = true;
	for i in 0..line_len {
		lock = lock && (arr[i] == b'#');
	}

	return (lock, ret);
}
