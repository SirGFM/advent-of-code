use std::io::{self, BufRead};
use std::collections;
use std::cmp;
use std::env;

#[derive(PartialEq, Eq)]
struct PointCost {
	cost: usize,
	x: usize,
	y: usize,
}

impl PointCost {
	fn new(cost: usize, x: usize, y: usize) -> PointCost {
		PointCost{
			cost: cost,
			x: x,
			y: y,
		}
	}

	fn cmp(&self, other: &Self) -> cmp::Ordering {
		// Binary heap retrieves the greatest value,
		// so cmp must be backwards.
		if self.cost > other.cost {
			cmp::Ordering::Less
		} else if self.cost < other.cost {
			cmp::Ordering::Greater
		} else {
			cmp::Ordering::Equal
		}
	}

	fn into_point(&self) -> Point {
		Point::new(self.x, self.y)
	}
}

impl Ord for PointCost {
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		self.cmp(other)
	}
}

impl PartialOrd for PointCost {
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		Some(self.cmp(other))
	}
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Point {
	x: usize,
	y: usize,
}

impl Point {
	fn new(x: usize, y: usize) -> Point {
		Point{
			x: x,
			y: y,
		}
	}

	fn neighbours(&self) -> Vec<Point> {
		let mut next = Vec::<Point>::new();

		if self.x > 0 {
			next.push(Point::new(self.x - 1, self.y));
		}
		if self.y > 0 {
			next.push(Point::new(self.x, self.y - 1));
		}
		next.push(Point::new(self.x + 1, self.y));
		next.push(Point::new(self.x, self.y + 1));

		return next;
	}

	fn cost(&self) -> usize {
		return 1;
	}

	fn get_point_cost(&self, cost: usize) -> PointCost {
		PointCost::new(cost, self.x, self.y)
	}

	fn idx(&self, line_len: usize) -> usize {
		self.x + self.y * line_len
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let num_bytes = args[1].parse::<usize>().unwrap();

	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();
	let mut points = Vec::<(usize, usize)>::new();

	let mut w = 0;
	let mut h = 0;

	// Read the map.
	loop {
		buf.clear();
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}

		let values: Vec<usize> = buf.trim()
			.split(",")
			.map(|x| x.parse::<usize>().unwrap())
			.collect();

		let x = values[0];
		let y = values[1];

		if w == 0 {
			w = x;
			h = y;
		} else {
			points.push((x, y));
		}
	}

	// Create the map and map of costs.
	let mut blocked: Vec<bool> = vec![false; w * h];
	let mut cost: Vec<usize> = vec![0x7f_ff_ff_ff; w * h];

	for i in 0..num_bytes {
		let (x, y) = points[i];
		blocked[x + y * w] = true;
	}

	let start = Point::new(0, 0);
	cost[start.idx(w)] = 0;

	let mut to_visit = collections::HashSet::<Point>::new();
	to_visit.insert(start);

	let mut visited = collections::HashSet::<Point>::new();

	let ex = w - 1;
	let ey = h - 1;
	let mut exit: Option<Point> = None;

	loop {
		if to_visit.len() == 0 {
			break;
		}

		let mut heap = collections::BinaryHeap::<PointCost>::new();
		for node in &to_visit {
			let cost = cost[node.idx(w)];
			heap.push(node.get_point_cost(cost));
		}

		let cur: Point = match heap.pop() {
			Some(got) => to_visit.take(&got.into_point()).unwrap(),
			None => panic!("empty heap"),
		};

		let cur_cost = cost[cur.idx(w)];

		for node in cur.neighbours() {
			if node.x >= w ||
				node.y >= h ||
				blocked[node.idx(w)] ||
				visited.get(&node).is_some() {

				continue;
			}

			let old_cost = cost[node.idx(w)];
			let new_cost = cur_cost + node.cost();

			if new_cost < old_cost {
				if node.x == ex && node.y == ey {
					exit = Some(Point::new(node.x, node.y));
				}

				cost[node.idx(w)] = new_cost;
				to_visit.insert(node);
			}
		}

		visited.insert(cur);
	}

	let exit = exit.unwrap();

	// Visit every node in reverse order.
	let mut to_visit = collections::HashSet::<Point>::new();
	to_visit.insert(exit);

	let mut fewest = collections::HashSet::<Point>::new();
	let mut visited = collections::HashSet::<Point>::new();

	loop {
		if to_visit.len() == 0 {
			break;
		}

		let tmp: Vec<Point> = to_visit.drain().collect();

		let cur = &tmp[0];
		let cur_cost = cost[cur.idx(w)];
		for node in cur.neighbours() {
			if node.x >= w ||
				node.y >= h ||
				blocked[node.idx(w)] ||
				cost[node.idx(w)] > cur_cost {

				continue;
			}

			to_visit.insert(node);
		}
		fewest.insert(Point::new(cur.x, cur.y));

		for cur in tmp {
			let cur_cost = cost[cur.idx(w)];

			for node in cur.neighbours() {
				if node.x >= w ||
					node.y >= h ||
					blocked[node.idx(w)] ||
					cost[node.idx(w)] > cur_cost {

					continue;
				}

				to_visit.insert(node);
			}
			visited.insert(Point::new(cur.x, cur.y));
		}
	}

	println!("part 1: {}", fewest.len() - 1);

	let mut i = num_bytes;
	while i < points.len() {
		let (x, y) = points[i];

		let tmp = Point::new(x, y);
		if !blocked[tmp.idx(w)] {
			blocked[tmp.idx(w)] = true;

			if !has_exit(w, h, &blocked) {
				break;
			}
		}

		i += 1;
	}

	println!("part 2: {},{}", points[i].0, points[i].1);
}

fn has_exit(w: usize, h: usize, blocked: &Vec<bool>) -> bool {
	let ex = w - 1;
	let ey = h - 1;

	let mut cost: Vec<usize> = vec![0x7f_ff_ff_ff; w * h];

	let start = Point::new(0, 0);
	cost[start.idx(w)] = 0;

	let mut to_visit = collections::HashSet::<Point>::new();
	to_visit.insert(start);

	let mut visited = collections::HashSet::<Point>::new();

	loop {
		if to_visit.len() == 0 {
			return false;
		}

		let mut heap = collections::BinaryHeap::<PointCost>::new();
		for node in &to_visit {
			let cost = cost[node.idx(w)];
			heap.push(node.get_point_cost(cost));
		}

		let cur: Point = match heap.pop() {
			Some(got) => to_visit.take(&got.into_point()).unwrap(),
			None => panic!("empty heap"),
		};

		let cur_cost = cost[cur.idx(w)];

		for node in cur.neighbours() {
			if node.x >= w ||
				node.y >= h ||
				blocked[node.idx(w)] ||
				visited.get(&node).is_some() {

				continue;
			}

			let old_cost = cost[node.idx(w)];
			let new_cost = cur_cost + node.cost();

			if new_cost < old_cost {
				if node.x == ex && node.y == ey {
					return true;
				}

				cost[node.idx(w)] = new_cost;
				to_visit.insert(node);
			}
		}

		visited.insert(cur);
	}
}
