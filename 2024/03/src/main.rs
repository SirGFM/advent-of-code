use std::io::{self, BufRead};
use regex::Regex;

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
	let re_parts = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();

	let mut buf = String::new();
	let mut p1_result = 0;
	let mut p2_result = 0;

	let mut parse = true;
	loop {
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}

		let line = buf.trim();
		re_parts.captures_iter(line).for_each(|entry| {
			let token = entry.get(0).map_or("", |m| m.as_str());

			if token == "do()" {
				parse = true;
			} else if token == "don't()" {
				parse = false;
			} else {
				re.captures_iter(token).for_each(|cap| {
					let a = cap.get(1)
						.map_or(0, |m| m.as_str().parse::<i32>().unwrap());
					let b = cap.get(2)
						.map_or(0, |m| m.as_str().parse::<i32>().unwrap());

					p1_result += a * b;
					if parse {
						p2_result += a * b;
					}
				})
			}
		});

		buf.clear();
	}

	println!("part 1: {}", p1_result);
	println!("part 2: {}", p2_result);
}
