use std::io::{self, BufRead};
use std::collections;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Point {
	x: i32,
	y: i32,
}

impl Point{
	fn new(x: i32, y: i32) -> Point {
		Point{
			x: x,
			y: y,
		}
	}

	fn reflect(&self, other: &Self) -> Point {
		let diff = other.sub(self);
		return self.sub(&diff);
	}

	fn sub(&self, other: &Self) -> Point {
		Point{
			x: self.x - other.x,
			y: self.y - other.y,
		}
	}
}

type Antennas = collections::HashMap::<u8, Vec<Point>>;

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut line_len = 0;
	let mut input = "".to_owned();

	let mut buf = String::new();

	let mut p1_nodes = collections::HashSet::<Point>::new();
	let mut p2_nodes = collections::HashSet::<Point>::new();
	let mut antennas = Antennas::new();
	let mut y: i32 = 0;

	// Read the map.
	loop {
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}

		let line = buf.trim();
		input.push_str(line);
		line_len = line.len();

		let mut x: i32 = 0;
		for c in line.bytes() {
			if c != b'.' {
				match antennas.get_mut(&c) {
					Some(tmp) => {
						tmp.push(Point::new(x, y));
					},
					None => {
						let mut tmp = Vec::<Point>::new();
						tmp.push(Point::new(x, y));
						antennas.insert(c, tmp);
					},
				}
			}
			x += 1;
		}

		buf.clear();
		y += 1;
	}

	let max_x = line_len as i32;
	let max_y = y;

	// Compare each antenna group.
	for list in antennas.values() {
		for i in 0..list.len()-1 {
			let p1 = &list[i];

			for j in i+1..list.len() {
				let p2 = &list[j];

				let mut tmp = p1.reflect(p2);
				if tmp.x >= 0 && tmp.x < max_x && tmp.y >= 0 && tmp.y < max_y {
					p1_nodes.insert(tmp.clone());
				}

				let diff = p2.sub(p1);
				while tmp.x >= 0 && tmp.x < max_x && tmp.y >= 0 && tmp.y < max_y {
					p2_nodes.insert(tmp.clone());
					tmp = tmp.sub(&diff);
				}

				let mut tmp = p2.reflect(p1);
				if tmp.x >= 0 && tmp.x < max_x && tmp.y >= 0 && tmp.y < max_y {
					p1_nodes.insert(tmp.clone());
				}

				let diff = p1.sub(p2);
				while tmp.x >= 0 && tmp.x < max_x && tmp.y >= 0 && tmp.y < max_y {
					p2_nodes.insert(tmp.clone());
					tmp = tmp.sub(&diff);
				}

				p2_nodes.insert(p1.clone());
				p2_nodes.insert(p2.clone());
			}
		}
	}

	println!("part 1: {}", p1_nodes.len());
	println!("part 2: {}", p2_nodes.len());
}
