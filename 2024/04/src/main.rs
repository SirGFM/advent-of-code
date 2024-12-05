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
			side: side,
			max: side * side,
		}
	}

	fn is_new_line(&self) -> bool {
		return self.x == 0 || self.y == self.side - 1;
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
		} else if self.y == 0 {
			self.y = self.x + 1;
			self.x = 0;
		} else {
			self.x += 1;
			self.y -= 1;
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
				to: self.i,
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

	// Read left-to-right.
	let input_bytes = input.as_bytes();
	let mut lr_vec = vec!['.' as u8; input_bytes.len() + line_len];
	for i in 0..input.len() {
		let x = i % line_len;
		let y = i / line_len;

		// Separate adjacent lines.
		lr_vec[1 + x + y * line_len] = input_bytes[i];
	}
	let lr = str::from_utf8(&lr_vec).unwrap();

	// Convert to up-down.
	let mut ud_vec = vec!['.' as u8; input_bytes.len() + line_len];
	for i in 0..input.len() {
		let x = i % line_len;
		let y = i / line_len;

		// Separate adjacent lines.
		ud_vec[x + (y + 1) * line_len] = input_bytes[y + x * line_len];
	}
	let ud = str::from_utf8(&ud_vec).unwrap();

	// Convert to left-to-right diagonal.
	let mut tmp = Diagonal::new(line_len, false);
	let mut offset = 0;
	let mut diag_lr_vec = vec!['.' as u8; input_bytes.len() + line_len * 2];
	loop {
		let point = match tmp.next() {
			Some(v) => v,
			None => break,
		};

		// Separate adjacent lines.
		if tmp.is_new_line() && point.from != 0 {
			offset += 1;
		}

		diag_lr_vec[point.to + offset] = input_bytes[point.from];
	}
	let diag_lr = str::from_utf8(&diag_lr_vec).unwrap();

	// Convert to right-to-left diagonal.
	let mut offset = 0;
	let mut tmp = Diagonal::new(line_len, true);
	let mut diag_rl_vec = vec!['.' as u8; input_bytes.len() + line_len * 2];
	loop {
		let point = match tmp.next() {
			Some(v) => v,
			None => break,
		};

		// Separate adjacent lines.
		if tmp.is_new_line() && point.from != 0 {
			offset += 1;
		}

		diag_rl_vec[point.to + offset] = input_bytes[point.from];
	}
	let diag_rl = str::from_utf8(&diag_rl_vec).unwrap();

	// Invert everything.
	let diag_rl_rev = diag_rl.chars().rev().collect::<String>();
	let diag_lr_rev = diag_lr.chars().rev().collect::<String>();
	let du = ud.chars().rev().collect::<String>();
	let rl = lr.chars().rev().collect::<String>();

	p1_result += re.captures_iter(&lr).count();
	p1_result += re.captures_iter(&rl).count();
	p1_result += re.captures_iter(&ud).count();
	p1_result += re.captures_iter(&du).count();
	p1_result += re.captures_iter(&diag_lr).count();
	p1_result += re.captures_iter(&diag_lr_rev).count();
	p1_result += re.captures_iter(&diag_rl).count();
	p1_result += re.captures_iter(&diag_rl_rev).count();

	println!("part 1: {}", p1_result);
	println!("part 2: {}", p2_result);
}
