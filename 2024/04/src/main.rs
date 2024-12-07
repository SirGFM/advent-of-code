use std::io::{self, BufRead};
use std::str;
use regex::Regex;

struct Converter {
	from: usize,
	to: usize,
}

struct Diagonal {
	rl: bool,
	init: bool,
	i: usize,
	x: usize,
	y: usize,
	to: usize,
	side: usize,
	max: usize,
}

impl Diagonal {
	fn new(side: usize, rl: bool) -> Diagonal {
		Diagonal{
			init: false,
			rl: rl,
			i: 0,
			x: 0,
			y: 0,
			to: 0,
			side: side,
			max: side * side,
		}
	}

	fn is_new_line(&self) -> bool {
		return self.y == 0 || self.x == self.side - 1;
	}
}

impl Iterator for Diagonal {
	type Item = Converter;

	fn next(&mut self) -> Option<Self::Item> {
		if !self.init {
			self.init = true;

			let mut from = 0;
			if self.rl {
				from = self.side - 1;
			}

			return Some(Self::Item{
				from: from,
				to: 0,
			})
		}

		self.i += 1;
		if self.x == self.side - 1 {
			let tmp = self.x;
			self.x = self.y + 1;
			self.y = tmp;
			self.to = 0;
		} else if self.y == 0 {
			self.y = self.x + 1;
			self.x = 0;
			self.to = 0;
		} else {
			self.x += 1;
			self.y -= 1;
			self.to += 1;
		}

		if self.i >= self.max {
			return None;
		} else {
			let mut x = self.x;
			if self.rl {
				x = self.side - self.x - 1;
			}

			return Some(Self::Item{
				from: x + self.y * self.side,
				to: self.to,
			})
		}
	}
}

// TODO: finish
fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let re = Regex::new(r"XMAS").unwrap();

	let mut line_len = 0;
	let mut input = "".to_owned();

	let mut buf = String::new();
	let mut p1_result = 0;
	let mut p2_result = 0;

	loop {
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}

		let line = buf.trim();
		input.push_str(line);
		line_len = line.len();

		buf.clear();
	}

	let input_bytes = input.as_bytes();
	let mut buf = vec!['.' as u8; line_len * 2];

	// Count horizontal lines.
	for i in 0..input.len() {
		let idx = i % line_len;

		buf[idx] = input_bytes[i];
		if idx == line_len - 1 {
			let tmp = str::from_utf8(&buf[0..line_len]).unwrap();
			let rev = tmp.chars().rev().collect::<String>();

			p1_result += re.captures_iter(&tmp).count();
			p1_result += re.captures_iter(&rev).count();
		}
	}

	// Count vertical lines.
	for i in 0..input.len() {
		let x = i % line_len;
		let y = i / line_len;

		buf[x] = input_bytes[y + x * line_len];
		if x == line_len - 1 {
			let tmp = str::from_utf8(&buf[0..line_len]).unwrap();
			let rev = tmp.chars().rev().collect::<String>();

			p1_result += re.captures_iter(&tmp).count();
			p1_result += re.captures_iter(&rev).count();
		}
	}

	// Count left-to-right diagonal.
	let mut diagonal = Diagonal::new(line_len, false);
	loop {
		let point = match diagonal.next() {
			Some(v) => v,
			None => break,
		};

		buf[point.to] = input_bytes[point.from];
		if diagonal.is_new_line() {
			let tmp = str::from_utf8(&buf[0..point.to+1]).unwrap();
			let rev = tmp.chars().rev().collect::<String>();

			p1_result += re.captures_iter(&tmp).count();
			p1_result += re.captures_iter(&rev).count();
		}
	}

	// Count right-to-left diagonal.
	let mut diagonal = Diagonal::new(line_len, true);
	loop {
		let point = match diagonal.next() {
			Some(v) => v,
			None => break,
		};

		buf[point.to] = input_bytes[point.from];
		if diagonal.is_new_line() {
			let tmp = str::from_utf8(&buf[0..point.to+1]).unwrap();
			let rev = tmp.chars().rev().collect::<String>();

			p1_result += re.captures_iter(&tmp).count();
			p1_result += re.captures_iter(&rev).count();
		}
	}

	// Count X-MAS.
	for p in input.match_indices("A") {
		let i = p.0;
		let x = i % line_len;
		let y = i / line_len;

		if x == 0 || y == 0 || x == line_len - 1 || y == line_len - 1 {
			continue;
		}

		let mut hor = vec!['.' as u8; 6];
		hor[0] = input_bytes[x - 1 + (y - 1) * line_len];
		hor[1] = input_bytes[x + y * line_len];
		hor[2] = input_bytes[x + 1 + (y + 1) * line_len];

		hor[3] = input_bytes[x - 1 + (y + 1) * line_len];
		hor[4] = input_bytes[x + y * line_len];
		hor[5] = input_bytes[x + 1 + (y - 1) * line_len];

		let mut ver = vec!['.' as u8; 6];
		ver[0] = input_bytes[x - 1 + (y - 1) * line_len];
		ver[1] = input_bytes[x + y * line_len];
		ver[2] = input_bytes[x + 1 + (y + 1) * line_len];
		ver[3] = input_bytes[x + 1 + (y - 1) * line_len];
		ver[4] = input_bytes[x + y * line_len];
		ver[5] = input_bytes[x - 1 + (y + 1) * line_len];

		let hor_str = str::from_utf8(&hor).unwrap();
		let ver_str = str::from_utf8(&ver).unwrap();

		if hor_str == "MASMAS" || hor_str == "SAMSAM" || ver_str == "MASMAS" || ver_str == "SAMSAM" {
			p2_result += 1;
		}
	}

	println!("part 1: {}", p1_result);
	println!("part 2: {}", p2_result);
}
