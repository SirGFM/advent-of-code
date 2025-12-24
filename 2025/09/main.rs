use std::io::{self, BufRead};
use std::cmp::{max, min};

struct Point {
	x: u64,
	y: u64,
}

impl Point {
	fn new(x: u64, y: u64) -> Point {
		Point{
			x: x,
			y: y,
		}
	}

	fn area(&self, other: &Self) -> u64 {
		let x: u64 = 1 + max(self.x, other.x) - min(self.x, other.x);
		let y: u64 = 1 + max(self.y, other.y) - min(self.y, other.y);

		return x*y;
	}
}

/*
impl JBox {
	fn new(x: u64, y: u64, z: u64) -> JBox {
		JBox{
			at: Point::new(x, y, z),
			circuit: 0,
			pair: 0,
			dist: 0,
		}
	}

	fn sq_dist(&self, other: &Self) -> u64 {
		let x: u64 = max(self.at.x, other.at.x) - min(self.at.x, other.at.x);
		let y: u64 = max(self.at.y, other.at.y) - min(self.at.y, other.at.y);
		let z: u64 = max(self.at.z, other.at.z) - min(self.at.z, other.at.z);

		return x*x + y*y + z*z;
	}
}

impl PartialOrd for JBox {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		if self.dist < other.dist {
			Some(Ordering::Less)
		} else if self.dist > other.dist {
			Some(Ordering::Greater)
		} else if self.pair < other.pair {
			Some(Ordering::Less)
		} else if self.pair > other.pair {
			Some(Ordering::Greater)
		} else {
			Some(Ordering::Equal)
		}
	}
}

impl Ord for JBox {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}

impl PartialEq for JBox {
	fn eq(&self, other: &Self) -> bool {
		return self.dist == other.dist &&
			self.pair == other.pair;
	}
}

impl Eq for JBox {}
*/

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();
	let mut result_1: u64 = 0;
	let mut result_2: u64 = 0;

	let mut points = Vec::<Point>::new();

	loop {
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}

		let line: Vec<_> = buf
			.trim()
			.split(",")
			.map(|x| x.parse::<u64>().unwrap())
			.collect();

		points.push(Point::new(line[0], line[1]));

		buf.clear();
	}

	for i in 0..points.len()-1 {
		for j in 0..points.len() {
			let area = points[i].area(&points[j]);

			result_1 = max(area, result_1);
		}
	}

	println!("part 1: {}", result_1);
	println!("part 2: {}", result_2);
}
