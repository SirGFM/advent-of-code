use std::io::{self, BufRead};

fn print_data(rx: usize, ry: usize, data: &[u8], line_len: usize) {
	for i in 0..data.len() {
		let x = i % line_len;
		let y = i / line_len;

		if x == 0 {
			println!("");
		}

		if x == rx && y == ry {
			print!("@");
		} else {
			print!("{}", data[i] as char);
		}
	}

	println!("\n");
}

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

	let input_bytes = unsafe { input.as_bytes_mut() };
	input_bytes[x + y * line_len] = b'.';

	// Convert the map.
	let mut p2_data = vec![b'.'; input_bytes.len() * 2];
	for i in 0..input_bytes.len() {
		match input_bytes[i] {
			b'.' => { /* Do nothing */ },
			b'#' => {
				p2_data[i * 2] = b'#';
				p2_data[i * 2 + 1] = b'#';
			},
			b'O' => {
				p2_data[i * 2] = b'[';
				p2_data[i * 2 + 1] = b']';
			},
			_ => panic!("invalid position"),
		}
	}

	// Run instructions.
	let mut p1_x = x;
	let mut p1_y = y;
	let mut p2_x = x * 2;
	let mut p2_y = y;

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

			if try_move(p1_x as i32, p1_y as i32, dx, dy, input_bytes, line_len as i32, num_lines as i32) {
				p1_x = ((p1_x as i32) + dx) as usize;
				p1_y = ((p1_y as i32) + dy) as usize;
			}

			if try_move(p2_x as i32, p2_y as i32, dx, dy, &mut p2_data, (line_len * 2) as i32, num_lines as i32) {
				p2_x = ((p2_x as i32) + dx) as usize;
				p2_y = ((p2_y as i32) + dy) as usize;
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

	for i in 0..p2_data.len() {
		if p2_data[i] == b'[' {
			let x = i % (line_len * 2);
			let y = i / (line_len * 2);

			p2_result += x + y * 100;
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

	let tile = data[(x + y * line_len) as usize];
	match tile {
		b'.' => return true,
		b'#' => return false,
		b'O' => {
			return push_box(x, y, dx, dy, data, line_len, num_lines);
		},
		b'[' | b']' => {
			let ok = push_box(x, y, dx, dy, data, line_len, num_lines);
			if ok && dy != 0 {
				let off: i32 = match tile {
					b'[' => 1,
					b']' => -1,
					_ =>  panic!("invalid case"),
				};

				force_push(x + off, y, dy, data, line_len, num_lines);
			}
			return ok;
		},
		_ => panic!("invalid position"),
	}
}

fn push_box(x: i32, y: i32, dx: i32, dy: i32, data: &mut [u8], line_len: i32, num_lines: i32) -> bool {
	if x + dx < 0 || x + dx >= line_len || y + dy < 0 || y + dy >= num_lines {
		return false;
	}

	let old_idx = (x + y * line_len) as usize;
	let old_tile = data[old_idx];

	if dy != 0 && (old_tile == b'[' || old_tile == b']') {
		let off: i32 = match old_tile {
			b'[' => 1,
			b']' => -1,
			_ =>  panic!("invalid case"),
		};

		if !simulate_push(x, y, dy, data, line_len, num_lines) ||
			!simulate_push(x + off, y, dy, data, line_len, num_lines) {

			return false;
		}
	}

	let x = x + dx;
	let y = y + dy;

	let new_idx = (x + y * line_len) as usize;
	let new_tile = data[new_idx];

	match new_tile {
		b'#' => return false,
		b'.' => { /* Do nothing. */},
		b'O' => {
			if !push_box(x, y, dx, dy, data, line_len, num_lines) {
				return false;
			}
		},
		b'[' | b']' => {
			if dx != 0 {
				if !push_box(x, y, dx, dy, data, line_len, num_lines) {
					return false;
				}
			} else if dy != 0 {
				let off: i32 = match new_tile {
					b'[' => 1,
					b']' => -1,
					_ =>  panic!("invalid case"),
				};

				force_push(x, y, dy, data, line_len, num_lines);
				force_push(x + off, y, dy, data, line_len, num_lines);
			}
		},
		_ => panic!("invalid position"),
	}

	// Move the box.
	data[new_idx] = old_tile;
	data[old_idx] = b'.';
	return true;
}

fn simulate_push(x: i32, y: i32, dy: i32, data: &mut [u8], line_len: i32, num_lines: i32) -> bool {
	if y + dy < 0 || y + dy >= num_lines {
		return false;
	}

	let y = y + dy;

	let new_idx = (x + y * line_len) as usize;

	match data[new_idx] {
		b'#' => return false,
		b'.' => return true,
		b'[' => {
			return simulate_push(x, y, dy, data, line_len, num_lines) &&
				simulate_push(x + 1, y, dy, data, line_len, num_lines);
		},
		b']' => {
			return simulate_push(x, y, dy, data, line_len, num_lines) &&
				simulate_push(x - 1, y, dy, data, line_len, num_lines);
		},
		_ => panic!("invalid position"),
	}
}

fn force_push(x: i32, y: i32, dy: i32, data: &mut [u8], line_len: i32, num_lines: i32) {
	let old_idx = (x + y * line_len) as usize;
	let old_tile = data[old_idx];

	let y = y + dy;

	let new_idx = (x + y * line_len) as usize;

	match data[new_idx] {
		b'.' => { /* Do nothing. */},
		b'[' => {
			force_push(x, y, dy, data, line_len, num_lines);
			force_push(x + 1, y, dy, data, line_len, num_lines);
		},
		b']' => {
			force_push(x, y, dy, data, line_len, num_lines);
			force_push(x - 1, y, dy, data, line_len, num_lines);
		},
		_ => panic!("invalid position"),
	}

	// Move the box.
	data[new_idx] = old_tile;
	data[old_idx] = b'.';
}
