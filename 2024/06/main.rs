use std::io::{self, BufRead};
use std::str;

// '.' == 0b0010_1110

#[repr(u8)]
#[derive(Clone)]
#[derive(PartialEq)]
enum Direction {
	Up    = 0b1000_0000,
	Down  = 0b0100_0000,
	Left  = 0b0001_0000,
	Right = 0b0000_0001,
}

impl Direction {
	fn next(&self, x: i32, y: i32) -> (i32, i32) {
		match self {
			Direction::Up => (x, y - 1),
			Direction::Down => (x, y + 1),
			Direction::Left => (x - 1, y),
			Direction::Right => (x + 1, y),
		}
	}

	fn rotate(self) -> Direction {
		match self {
			Direction::Up => Direction::Right,
			Direction::Down => Direction::Left,
			Direction::Left => Direction::Up,
			Direction::Right => Direction::Down,
		}
	}
}

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut line_len: i32 = 0;
	let mut input = "".to_owned();

	let mut buf = String::new();
	let mut p2_result = 0;

	// Read the map.
	loop {
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}

		let line = buf.trim();
		input.push_str(line);
		line_len = line.len() as i32;

		buf.clear();
	}

	let init = input.find("^").unwrap() as i32;

	let input = input.replace("^", ".");
	let input_bytes = input.as_bytes();

	let mut data = vec![0 as u8; input.len()];
	data.copy_from_slice(&input_bytes);
	let p1_result = check(&mut data, line_len, init, true).0;

	for i in 0..input.len() {
		let mut data = vec![0 as u8; input.len()];
		data.copy_from_slice(&input_bytes);

		if data[i] != b'.' {
			continue;
		}
		data[i] = b'#';
		data[init as usize] = b'.';

		if check(&mut data, line_len, init, false).1 {
			p2_result += 1;
		}
	}

	println!("part 1: {}", p1_result);
	println!("part 2: {}", p2_result);
}

fn check(data: &mut [u8], line_len: i32, init: i32, get_count: bool) -> (usize, bool) {
	let init_x: i32 = init % line_len;
	let init_y: i32 = init / line_len;

	let mut x = init_x;
	let mut y = init_y;
	let mut dir = Direction::Up;

	loop {
		if !get_count && (data[(x + y * line_len) as usize] & (dir.clone() as u8)) != 0 {
			// Found a loop.
			return (0, true);
		}

		let (next_x, next_y) = dir.next(x, y);

		if next_x < 0 || next_x >= line_len || next_y < 0 || next_y >= line_len {
			// Exited the loop.
			if get_count {
				return (str::from_utf8(&data).unwrap().split("X").collect::<Vec<_>>().len(), false);
			} else {
				return (0, false);
			}
		}

		if data[(next_x + next_y * line_len) as usize] == b'#' {
			if !get_count {
				data[(x + y * line_len) as usize] |= dir.clone() as u8;
			}
			dir = dir.rotate();
		} else {
			if get_count {
				data[(x + y * line_len) as usize] = b'X';
			} else {
				data[(x + y * line_len) as usize] |= dir.clone() as u8;
			}

			x = next_x;
			y = next_y;
		}
	}
}
