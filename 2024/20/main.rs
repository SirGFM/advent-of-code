use std::io::{self, BufRead};
use std::collections;
use std::cmp;

#[derive(PartialEq, Eq, Hash)]
struct CheatPos {
	from_x: usize,
	from_y: usize,
	to_x: usize,
	to_y: usize,
	saved: usize,
}

impl CheatPos {
	fn new(from_x: usize, from_y: usize, to_x: usize, to_y: usize, saved: usize) -> CheatPos {
		CheatPos{
			from_x: from_x,
			from_y: from_y,
			to_x: to_x,
			to_y: to_y,
			saved: saved,
		}
	}
}

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
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();

	let mut blocked: Vec<bool> = Vec::<bool>::new();
	let mut w = 0;

	let mut ex = 0;
	let mut ey = 0;

	// Read the map.
	loop {
		buf.clear();
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}

		let line = buf.trim();
		for c in line.chars() {
			if c == 'E' {
				ex = blocked.len() % w;
			}

			if c == '#' {
				blocked.push(true);
			} else {
				blocked.push(false);
			}
		}

		if ex == 0 {
			ey += 1;
		}

		w = line.len();
	}

	let w = w;
	let h = blocked.len() / w;

	// Create the map and map of costs.
	let mut cost: Vec<usize> = vec![0x7f_ff_ff_ff; w * h];

	let exit = Point::new(ex, ey);
	cost[exit.idx(w)] = 0;

	let mut to_visit = collections::HashSet::<Point>::new();
	to_visit.insert(exit);

	let mut visited = collections::HashSet::<Point>::new();

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
				cost[node.idx(w)] = new_cost;
				to_visit.insert(node);
			}
		}

		visited.insert(cur);
	}

	let mut p1_result = 0;
	for y in 0..h {
		for x in 0..w {
			let cur = Point::new(x, y);

			if blocked[cur.idx(w)] {
				continue
			}

			for cheat in check_cheat(&cur, w, h, &blocked, &cost) {
				if cheat.saved >= 100 {
					p1_result += 1;
				}
			}
		}
	}

	println!("part 1: {}", p1_result);

	let mut found = collections::HashSet::<CheatPos>::new();

	for y in 0..h {
		for x in 0..w {
			let cur = Point::new(x, y);

			if blocked[cur.idx(w)] {
				continue
			}

			check_cheat20(&cur, w, h, &blocked, &cost, &mut found);
		}
	}

	let mut p2_result = 0;
	for node in found.into_iter() {
		if node.saved >= 100 {
			p2_result += 1;
		}
	}
	println!("part 2: {}", p2_result);
}

fn check_cheat(cur: &Point, w: usize, h: usize, blocked: &Vec<bool>, cost: &Vec<usize>) -> Vec<CheatPos> {
	let mut ret: Vec<CheatPos> = Vec::<CheatPos>::new();
	let cur_cost = cost[cur.idx(w)];

	for node in cur.neighbours() {
		if node.x >= w || node.y >= h || !blocked[node.idx(w)] {
			continue;
		}

		let dx: i32 = (node.x as i32) - (cur.x as i32);
		let dy: i32 = (node.y as i32) - (cur.y as i32);

		let next_x: i32 = (node.x as i32) + dx;
		let next_y: i32 = (node.y as i32) + dy;
		if next_x < 0 || next_x >= (w as i32) || next_y < 0 || next_y >= (h as i32) {
			continue;
		}

		let next = Point::new(next_x as usize, next_y as usize);
		if blocked[next.idx(w)] {
			continue;
		}

		let next_cost = cost[next.idx(w)];
		if next_cost < cur_cost {
			// Extra 2 because of the cost of going through the wall.
			let new_point = CheatPos::new(cur.x, cur.y, next.x, next.y, cur_cost - next_cost - 2);
			ret.push(new_point);
		}
	}

	return ret;
}

fn check_cheat20(
	cur: &Point,
	w: usize,
	h: usize,
	blocked: &Vec<bool>,
	cost: &Vec<usize>,
	found: &mut collections::HashSet::<CheatPos>,
) {
	let cur_cost = cost[cur.idx(w)];

	for y in -20..21_i32 {
		let new_y = (cur.y as i32) + y;
		if new_y < 0 || new_y >= h as i32 {
			continue;
		}
		let new_y = new_y as usize;

		for x in -20..21_i32 {
			let delta = (y.abs() + x.abs()) as usize;
			if delta < 2 || delta > 20 {
				continue;
			}

			let new_x = (cur.x as i32) + x;
			if new_x < 0 || new_x >= w as i32 {
				continue;
			}
			let new_x = new_x as usize;

			let next = Point::new(new_x, new_y);
			if blocked[next.idx(w)] {
				continue;
			}

			let next_cost = cost[next.idx(w)];
			if next_cost < cur_cost && cur_cost - next_cost > delta {
				let new_point = CheatPos::new(cur.x, cur.y, next.x, next.y, cur_cost - next_cost - delta);
				found.insert(new_point);
			}
		}
	}
}
