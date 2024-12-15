use std::io::{self, BufRead};
use std::collections;
use std::cmp;

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

	fn cmp(&self, other: &Self) -> cmp::Ordering {
		let diff: i32;

		if self.y != other.y {
			diff = (self.y as i32) - (other.y as i32);
		} else {
			diff = (self.x as i32) - (other.x as i32);
		}

		if diff < 0 {
			return cmp::Ordering::Less;
		} else if diff > 0 {
			return cmp::Ordering::Greater;
		} else {
			return cmp::Ordering::Equal;
		}
	}
}

impl Ord for Point {
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		self.cmp(other)
	}
}

impl PartialOrd for Point {
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		Some(self.cmp(other))
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

			for point in &nodes {
				// Convert to lower case.
				input_bytes[point.x + point.y * line_len] |= 0x20;
			}

			let sides = conv(input_bytes[i], &nodes, input_bytes, line_len);
			p2_result += area * sides;

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

fn conv(cur: u8, points: &PointSet, data: &[u8], line_len: usize) -> usize {
	let mut sides = 0;

	let mut v: Vec::<&Point> = points.into_iter().collect();
	v.sort();

	let num_lines = data.len() / line_len;
	for point in v {
		let mut same = vec![false; 9];

		let x = point.x;
		let y = point.y;

		// Inspect the surroudings for tiles that
		// are of the same type of the current tile.
		for j in 0..3 {
			let sy: i32 = (y as i32) + (j as i32) - 1;

			if sy < 0 || sy as usize >= num_lines {
				continue;
			}

			for i in 0..3 {
				let sx: i32 = (x as i32) + (i as i32) - 1;

				if sx < 0 || sx as usize >= line_len {
					continue;
				}

				let x = x + i - 1;
				let y = y + j - 1;
				let tile = data[x + y * line_len];
				same[i + j * 3] = cur == tile;
			}
		}

		if !same[1] && (same[2] || !same[5]) {
			// Top side.
			sides += 1;
		}
		if !same[7] && (same[8] || !same[5]) {
			// Bottom side.
			sides += 1;
		}
		if !same[3] && (same[6] || !same[7]) {
			// Left side.
			sides += 1;
		}
		if !same[5] && (same[8] || !same[7]) {
			// Right side.
			sides += 1;
		}
	}

	return sides;
}
