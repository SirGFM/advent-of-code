use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();

	let mut left: Vec<i32> = Vec::new();
	let mut right: Vec<i32> = Vec::new();

	loop {
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}

		let line = buf.trim();

		let mut iter = line.split_whitespace();
		let tmp = iter.next();
		match tmp {
			Some(txt) => {
				let num = txt.parse::<i32>().unwrap();
				left.push(num);
			}
			None => panic!("invalid line"),
		}

		let tmp = iter.next();
		match tmp {
			Some(txt) => {
				let num = txt.parse::<i32>().unwrap();
				right.push(num);
			}
			None => panic!("invalid line"),
		}

		buf.clear();
	}

	left.sort();
	right.sort();

	let mut result = 0;
	for i in 0..left.len() {
		result += (left[i] - right[i]).abs();
	}

	println!("part 1: {}", result);

	let mut count = HashMap::new();

	for num in right.iter() {
		match count.get(num) {
			Some(i) => count.insert(num, i + 1),
			None => count.insert(num, 1),
		};
	}

	result = 0;
	for i in 0..left.len() {
		let cur = left[i];
		match count.get(&cur) {
			Some(i) => result += cur * i,
			None => {},
		};
	}

	println!("part 2: {}", result);
}
