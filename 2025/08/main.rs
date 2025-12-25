use std::io::{self, BufRead};
use std::cmp::{max, min, Ordering};
use std::collections::{HashMap};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Point {
	x: u64,
	y: u64,
	z: u64,
}

impl Point {
	fn new(x: u64, y: u64, z: u64) -> Point {
		Point{
			x: x,
			y: y,
			z: z,
		}
	}

	fn sq_dist(&self, other: &Self) -> u64 {
		let x: u64 = max(self.x, other.x) - min(self.x, other.x);
		let y: u64 = max(self.y, other.y) - min(self.y, other.y);
		let z: u64 = max(self.z, other.z) - min(self.z, other.z);

		return x*x + y*y + z*z;
	}
}

struct Conn {
	from: Point,
	to: Point,
	dist: u64,
}

impl Conn {
	fn new(from: Point, to: Point) -> Conn {
		let dist = from.sq_dist(&to);

		Conn{
			from: from,
			to: to,
			dist: dist,
		}
	}
}

impl PartialOrd for Conn {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		if self.dist < other.dist {
			Some(Ordering::Less)
		} else if self.dist > other.dist {
			Some(Ordering::Greater)
		} else {
			Some(Ordering::Equal)
		}
	}
}

impl Ord for Conn {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}

impl PartialEq for Conn {
	fn eq(&self, other: &Self) -> bool {
		return self.dist == other.dist
	}
}

impl Eq for Conn {}

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();
	let result_1: usize;
	let mut result_2: u64 = 0;

	let mut points = Vec::<Point>::new();
	let mut num_conn: usize = 0;

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

		if line.len() == 1 {
			num_conn = line[0] as usize;
		} else {
			points.push(Point::new(line[0], line[1], line[2]));
		}

		buf.clear();
	}

	if num_conn == 0 {
		num_conn = points.len();
	}

	let mut conns = Vec::<Conn>::new();

	for i in 1..points.len()-1 {
		let from = points[i].clone();

		for j in i+1..points.len() {
			let to = points[j].clone();

			conns.push(Conn::new(from, to));
		}
	}

	conns.sort();

	let mut circuits = Vec::<Vec<Point>>::new();
	let mut helper = HashMap::<Point, usize>::new();

	for i in 0..num_conn {
		let conn = &conns[i];

		let has_from = helper.get(&conn.from).is_some();
		let has_to = helper.get(&conn.to).is_some();

		if has_from && has_to {
			let from = *helper.get(&conn.from).unwrap();
			let to = *helper.get(&conn.to).unwrap();

			if from == to {
				// Do nothing!
			} else if circuits[to].len() < circuits[from].len() {
				for point in circuits[to].clone() {
					helper.insert(point, from);
					circuits[from].push(point);
				}
				circuits[to] = Vec::<Point>::new();
			} else {
				for point in circuits[from].clone() {
					helper.insert(point, to);
					circuits[to].push(point);
				}
				circuits[from] = Vec::<Point>::new();
			}
		} else if has_from {
			let circuit = *helper.get(&conn.from).unwrap();
			helper.insert(conn.to, circuit);
			circuits[circuit].push(conn.to);
		} else if has_to {
			let circuit = *helper.get(&conn.to).unwrap();
			helper.insert(conn.from, circuit);
			circuits[circuit].push(conn.from);
		} else {
			helper.insert(conn.from, circuits.len());
			helper.insert(conn.to, circuits.len());
			circuits.push(vec![conn.from, conn.to]);
		}
	}

	let mut num_conns = Vec::<usize>::new();
	for circuit in circuits {
		num_conns.push(circuit.len());
	}

	num_conns.sort();
	num_conns.reverse();
	result_1 = num_conns[0] * num_conns[1] * num_conns[2];

	println!("part 1: {}", result_1);
	println!("part 2: {}", result_2);
}
