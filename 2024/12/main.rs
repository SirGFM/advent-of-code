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

	let input_bytes = unsafe { input.as_bytes_mut() };

	for i in 0..input_bytes.len() {
		if input_bytes[i] != b'.' {
			let mut nodes = PointSet::new();

			let (area, perimeter) = bucket_fill(i % line_len, i / line_len, input_bytes, line_len, &mut nodes);
			p1_result += area * perimeter;
			//p2_result += area * sides.len();

			for point in nodes {
				input_bytes[point.x + point.y * line_len] = b'.';
			}
		}
	}

	println!("part 1: {}", p1_result);
	println!("part 2: {}", p2_result);
}

fn bucket_fill(x: usize, y: usize, data: &[u8], line_len: usize, visited: &mut PointSet) -> (usize, usize) {
	if visited.get(&Point::new(x, y)).is_some() {
		return (0, 0);
	}

	let mut area = 1;
	let mut perimeter = 4;
	let cur = data[x + line_len * y];

	visited.insert(Point::new(x, y));

	if x > 0 && data[x - 1 + line_len * y] == cur {
		let (tmp_area, tmp_perimeter) = bucket_fill(x - 1, y, data, line_len, visited);

		area += tmp_area;
		perimeter += tmp_perimeter;
		perimeter -= 1;
	}
	if x < line_len - 1 && data[x + 1 + line_len * y] == cur {
		let (tmp_area, tmp_perimeter) = bucket_fill(x + 1, y, data, line_len, visited);

		area += tmp_area;
		perimeter += tmp_perimeter;
		perimeter -= 1;
	}
	if y > 0 && data[x + line_len * (y - 1)] == cur {
		let (tmp_area, tmp_perimeter) = bucket_fill(x, y - 1, data, line_len, visited);

		area += tmp_area;
		perimeter += tmp_perimeter;
		perimeter -= 1;
	}
	if (y + 1) * line_len < data.len() && data[x + line_len * (y + 1)] == cur {
		let (tmp_area, tmp_perimeter) = bucket_fill(x, y + 1, data, line_len, visited);

		area += tmp_area;
		perimeter += tmp_perimeter;
		perimeter -= 1;
	}

	return (area, perimeter);
}
