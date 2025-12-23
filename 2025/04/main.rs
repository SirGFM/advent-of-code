use std::io::{self, BufRead};

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();
	let mut input = "".to_owned();
	let mut line_len: usize = 0;
	let mut num_lines: usize = 0;

	let mut result_1: u64 = 0;
	let mut result_2: u64 = 0;

	loop {
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}

		let line = buf.trim();
		input.push_str(line);
		line_len = line.len();
		num_lines += 1;

		buf.clear();
	}

	let input = unsafe { input.as_bytes_mut() };

	for i in 0..input.len() {
		if input[i] == b'@' && count_boxes(&input, line_len, num_lines, i) < 5 {
			result_1 += 1;
		}
	}

	let mut last_result = result_1;

	while result_2 != last_result {
		last_result = result_2;

		for i in 0..input.len() {
			if input[i] == b'@' && count_boxes(&input, line_len, num_lines, i) < 5 {
				result_2 += 1;
				input[i] = b'.';
			}
		}
	}

	println!("part 1: {}", result_1);
	println!("part 2: {}", result_2);
}

fn count_boxes(input: &[u8], w: usize, h: usize, i: usize) -> u64 {
	let mut res = 0;
	let y = i / w;
	let x = i % w;

	if y > 0 {
		res += count_boxes_line(&input, w, x, y - 1);
	}
	res += count_boxes_line(&input, w, x, y);
	if y < h - 1 {
		res += count_boxes_line(&input, w, x, y + 1);
	}

	return res;
}

fn count_boxes_line(input: &[u8], w: usize, x: usize, y: usize) -> u64 {
	let mut res = 0;

	if x > 0 {
		res += tile_to_box_count(&input, w, x - 1, y);
	}
	res += tile_to_box_count(&input, w, x, y);
	if x < w - 1 {
		res += tile_to_box_count(&input, w, x + 1, y);
	}

	return res;
}

fn tile_to_box_count(input: &[u8], w: usize, x: usize, y: usize) -> u64 {
	if input[x + y * w] == b'@' {
		return 1;
	}
	return 0;
}
