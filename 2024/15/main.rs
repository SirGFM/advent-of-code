use std::io::{self, BufRead};

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut line_len = 0;
	let mut input = "".to_owned();

	let mut buf = String::new();
	let mut p1_result = 0;
	let mut p2_result = 0;

	let mut x = 0;
	let mut y = 0;

	// Read the map.
	loop {
		buf.clear();
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}

		let line = buf.trim();
		if line.len() == 0 {
			break;
		}

		if x == 0 {
			let robot = line.find("@");
			if robot.is_some() {
				x = robot.unwrap();
			} else {
				y += 1;
			}
		}

		input.push_str(line);
		line_len = line.len();
	}
	let num_lines = input.len() / line_len;

	// Run instructions.
	let input_bytes = unsafe { input.as_bytes_mut() };
	input_bytes[x + y * line_len] = b'.';

	loop {
		buf.clear();
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}

		let line = buf.trim();
		for c in line.bytes() {
			let mut dx: i32 = 0;
			let mut dy: i32 = 0;

			match c {
				b'^' => dy = -1,
				b'v' => dy = 1,
				b'>' => dx = 1,
				b'<' => dx = -1,
				_ => {/* Do nothing */},
			}

			if try_move(x as i32, y as i32, dx, dy, input_bytes, line_len as i32, num_lines as i32) {
				x = ((x as i32) + dx) as usize;
				y = ((y as i32) + dy) as usize;
			}
		}
	}

	for i in 0..input_bytes.len() {
		if input_bytes[i] == b'O' {
			let x = i % line_len;
			let y = i / line_len;

			p1_result += x + y * 100;
		}
	}

	println!("part 1: {}", p1_result);
	println!("part 2: {}", p2_result);
}

fn try_move(x: i32, y: i32, dx: i32, dy: i32, data: &mut [u8], line_len: i32, num_lines: i32) -> bool {
	if x + dx < 0 || x + dx >= line_len || y + dy < 0 || y + dy >= num_lines {
		return false;
	}

	let x = x + dx;
	let y = y + dy;

	match data[(x + y * line_len) as usize] {
		b'.' => return true,
		b'#' => return false,
		b'O' => {
			return push_box(x, y, dx, dy, data, line_len, num_lines);
		},
		_ => panic!("invalid position"),
	}
}

fn push_box(x: i32, y: i32, dx: i32, dy: i32, data: &mut [u8], line_len: i32, num_lines: i32) -> bool {
	if x + dx < 0 || x + dx >= line_len || y + dy < 0 || y + dy >= num_lines {
		return false;
	}

	let old_idx = (x + y * line_len) as usize;

	let x = x + dx;
	let y = y + dy;

	let new_idx = (x + y * line_len) as usize;

	match data[new_idx] {
		b'#' => return false,
		b'.' => { /* Do nothing. */},
		b'O' => {
			if !push_box(x, y, dx, dy, data, line_len, num_lines) {
				return false;
			}
		},
		_ => panic!("invalid position"),
	}

	// Move the box.
	data[new_idx] = b'O';
	data[old_idx] = b'.';
	return true;
}
