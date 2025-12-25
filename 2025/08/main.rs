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

	for i in 0..points.len()-1 {
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
		add_conn(&conns[i], &mut circuits, &mut helper);
	}

	let mut num_conns = Vec::<usize>::new();
	for circuit in &circuits {
		num_conns.push(circuit.len());
	}

	num_conns.sort();
	num_conns.reverse();
	result_1 = num_conns[0] * num_conns[1] * num_conns[2];

	for i in num_conn..conns.len() {
		if is_single_circuit(&circuits, points.len()) {
			let conn = &conns[i-1];
			result_2 = conn.to.x * conn.from.x;
			break;
		}

		add_conn(&conns[i], &mut circuits, &mut helper);
	}

	println!("part 1: {}", result_1);
	println!("part 2: {}", result_2);
}

fn is_single_circuit(circuits: &Vec<Vec<Point>>, target: usize) -> bool {
	for i in 0..circuits.len()-1 {
		if circuits[i].len() == 0 {
			continue;
		} else if circuits[i].len() != target {
			return false;
		}

		for j in i+1..circuits.len() {
			if circuits[j].len() != 0 {
				return false;
			}
		}

		return true;
	}

	return false;
}

fn add_conn(conn: &Conn, circuits: &mut Vec<Vec<Point>>, helper: &mut HashMap<Point, usize>) {
	let has_from = helper.get(&conn.from).is_some();
	let has_to = helper.get(&conn.to).is_some();

	if has_from && has_to {
		let from = *helper.get(&conn.from).unwrap();
		let to = *helper.get(&conn.to).unwrap();

		if from == to {
			// Do nothing!
		} else if circuits[to].len() < circuits[from].len() {
			merge(to, from, circuits, helper);
		} else {
			merge(from, to, circuits, helper);
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

fn merge(old: usize, new: usize, circuits: &mut Vec<Vec<Point>>, helper: &mut HashMap<Point, usize>) {
	let src_idx: usize;
	let src: &mut [Vec<Point>];
	let dst_idx: usize;
	let dst: &mut [Vec<Point>];

	if old < new {
		src_idx = old;
		dst_idx = new - (old + 1);
		(src, dst) = circuits.split_at_mut(src_idx + 1);
	} else {
		dst_idx = new;
		src_idx = old - (new + 1);
		(dst, src) = circuits.split_at_mut(dst_idx + 1);
	}

	for point in &src[src_idx] {
		helper.insert(*point, new);
		dst[dst_idx].push(*point);
	}
	circuits[old] = Vec::<Point>::new();
}
