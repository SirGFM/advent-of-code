use std::io::{self, BufRead};

fn check_part1(entries: &Vec<i32>) -> bool {
	let mut last = -1;
	let mut dir = 0;

	for num in entries {
		if last != -1 {
			let cur = last - *num;
			if cur.abs() < 1 || cur.abs() > 3 {
				return false;
			}

			let cur_dir = cur / cur.abs();
			if dir != 0 && dir != cur_dir {
				return false;
			}
			dir = cur_dir;
		}

		last = *num;
	}

	return true;
}

fn check_part2(entries: &Vec<i32>, reverse: bool) -> bool {
	let mut last = -1;
	let mut dir = 0;
	let mut skipped = false;

	for i in 0..entries.len() {
		let num = entries[i];

		if last != -1 {
			let cur = last - num;
			if cur.abs() < 1 || cur.abs() > 3 {
				if skipped {
					if reverse {
						return false;
					}
					let mut tmp = entries.clone();
					tmp.reverse();
					return check_part2(&tmp, true);
				}
				skipped = true;
				continue;
			}

			let cur_dir = cur / cur.abs();
			if dir != 0 && dir != cur_dir {
				if skipped {
					if reverse {
						return false;
					}
					let mut tmp = entries.clone();
					tmp.reverse();
					return check_part2(&tmp, true);
				}
				skipped = true;
				continue;
			}
			dir = cur_dir;
		}

		last = num;
	}

	return true;
}

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut p1_result = 0;
	let mut p2_result = 0;
	let mut buf = String::new();
	loop {
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}

		let line = buf.trim();

		let entries: Vec<i32> = line
			.split_whitespace()
			.map(|x| x.parse::<i32>().unwrap())
			.collect();

		if check_part1(&entries) {
			p1_result += 1;
		}
		if check_part2(&entries, false) {
			p2_result += 1;
		}

		buf.clear();
	}

	println!("part 1: {}", p1_result);
	println!("part 2: {}", p2_result);
}
