use std::io::{self, BufRead};

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();
	let mut pos: i32 = 50;
	let mut result_1: i32 = 0;
	let mut result_2: i32 = 0;

	loop {
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}
		let line = buf.trim();

		let num = line[1..].parse::<i32>().unwrap();
		let cmd = line.as_bytes()[0];

		let mut num = num;
		print!("{} ", pos);
		if cmd == b'L' {
			num *= -1;
			print!("<-{} ", num);
		} else if cmd == b'R' {
			print!("{}-> ", num);
		}

		let mut num_clicks = 0;
		while num < 0 {
			pos -= 1;
			num += 1;
			if pos == 0 {
				num_clicks += 1;
				result_2 += 1;
			} else if pos == -1 {
				pos = 99;
			}
		}

		while num > 0 {
			pos += 1;
			num -= 1;
			if pos == 100 {
				num_clicks += 1;
				result_2 += 1;
				pos = 0;
			}
		}
		println!("{} (clicks: {})", pos, num_clicks);

		if pos == 0 {
			result_1 += 1;
		}

		buf.clear();
	}

	println!("part 1: {}", result_1);
	println!("part 2: {}", result_2);
}
