use std::io::{self, BufRead};
use std::collections;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Point {
	x: usize,
	y: usize,
}

type PointSet = collections::HashSet::<Point>;

impl Point{
	fn new(x: usize, y: usize) -> Point {
		Point{
			x: x,
			y: y,
		}
	}
}

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut line_len = 0;
	let mut input = "".to_owned();

	let mut buf = String::new();
	let mut p1_result = 0;
	let mut p2_result = 0;

	// Read the map.
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

	for i in 0..input.len() {
		if input_bytes[i] == b'0' {
			let mut nodes = PointSet::new();

			p2_result += inspect(i % line_len, i / line_len, input_bytes, line_len, &mut nodes);
			p1_result += nodes.len();
		}
	}

	println!("part 1: {}", p1_result);
	println!("part 2: {}", p2_result);
}

fn inspect(x: usize, y: usize, data: &[u8], line_len: usize, nodes: &mut PointSet) -> usize {
	let mut found = 0;
	let cur = data[x + line_len * y];

	if cur == b'9' {
		nodes.insert(Point::new(x, y));
		return 1;
	}

	if x > 0 && data[x - 1 + line_len * y] == cur + 1 {
		found += inspect(x - 1, y, data, line_len, nodes);
	}
	if x < line_len - 1 && data[x + 1 + line_len * y] == cur + 1 {
		found += inspect(x + 1, y, data, line_len, nodes);
	}
	if y > 0 && data[x + line_len * (y - 1)] == cur + 1 {
		found += inspect(x, y - 1, data, line_len, nodes);
	}
	if (y + 1) * line_len < data.len() && data[x + line_len * (y + 1)] == cur + 1 {
		found += inspect(x, y + 1, data, line_len, nodes);
	}

	return found;
}
