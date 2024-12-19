use std::io::{self, BufRead};
use std::collections;
use std::cmp;
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};
use std::ops::Add;

#[repr(u8)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Direction {
	Up    = 0,
	Right = 1,
	Down  = 2,
	Left  = 3,
}

impl TryFrom<usize> for Direction {
	type Error = &'static str;

	fn try_from(value: usize) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Direction::Up),
			1 => Ok(Direction::Right),
			2 => Ok(Direction::Down),
			3 => Ok(Direction::Left),
			_ => Err("invalid value"),
		}
	}
}

impl Add<usize> for Direction {
	type Output = Direction;

	fn add(self, other: usize) -> Self::Output {
		Direction::try_from(((self as usize) + other) % 4).unwrap()
	}
}

impl Direction {
	fn forward(self, x: usize, y: usize) -> Option<(usize, usize)> {
		if (x == 0 && self == Direction::Left) ||
			(y == 0 && self == Direction::Up) {

			return None;
		}

		match self {
			Direction::Up => return Some((x, y - 1)),
			Direction::Right => return Some((x + 1, y)),
			Direction::Down => return Some((x, y + 1)),
			Direction::Left => return Some((x - 1, y)),
		}
	}

	fn backward(self, x: usize, y: usize) -> Option<(usize, usize)> {
		if (x == 0 && self == Direction::Right) ||
			(y == 0 && self == Direction::Down) {

			return None;
		}

		match self {
			Direction::Up => return Some((x, y + 1)),
			Direction::Right => return Some((x - 1, y)),
			Direction::Down => return Some((x, y - 1)),
			Direction::Left => return Some((x + 1, y)),
		}
	}
}

#[derive(PartialEq, Eq)]
struct NodeCost {
	cost: usize,
	x: usize,
	y: usize,
	dir: Direction,
}

impl NodeCost {
	fn new(cost: usize, x: usize, y: usize, dir: Direction) -> NodeCost {
		NodeCost{
			cost: cost,
			x: x,
			y: y,
			dir: dir,
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

	fn into_node(&self) -> Node {
		Node::new(self.x, self.y, self.dir, self.dir)
	}
}

impl Ord for NodeCost {
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		self.cmp(other)
	}
}

impl PartialOrd for NodeCost {
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		Some(self.cmp(other))
	}
}

#[derive(Debug)]
struct Node {
	x: usize,
	y: usize,
	dir: Direction,
	from: Direction,
}

impl Node {
	fn new(x: usize, y: usize, dir: Direction, from: Direction) -> Node {
		Node{
			x: x,
			y: y,
			dir: dir,
			from: from,
		}
	}

	fn next(&self) -> Vec<Node> {
		let mut next = Vec::<Node>::new();

		next.push(Node::new(self.x, self.y, self.dir + 1 % 4, self.dir));
		next.push(Node::new(self.x, self.y, self.dir + 3 % 4, self.dir));
		match self.dir.forward(self.x, self.y) {
			Some((x, y)) => next.push(Node::new(x, y, self.dir, self.dir)),
			None => {/*Do nothing*/},
		}

		return next;
	}

	fn prev(&self) -> Vec<Node> {
		let mut next = Vec::<Node>::new();

		next.push(Node::new(self.x, self.y, self.dir + 1 % 4, self.dir));
		next.push(Node::new(self.x, self.y, self.dir + 3 % 4, self.dir));
		match self.dir.backward(self.x, self.y) {
			Some((x, y)) => next.push(Node::new(x, y, self.dir, self.dir)),
			None => {/*Do nothing*/},
		}

		return next;
	}

	fn cost(&self) -> usize {
		if self.dir == self.from {
			return 1;
		} else {
			return 1000;
		}
	}

	fn get_node_cost(&self, cost: usize) -> NodeCost {
		NodeCost::new(cost, self.x, self.y, self.dir)
	}

	fn idx(&self, line_len: usize) -> usize {
		self.x + self.y * line_len
	}

	fn cost_idx(&self, line_len: usize) -> usize {
		(self.dir as usize) + self.idx(line_len) * 4
	}
}

impl PartialEq for Node {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x &&
			self.y == other.y &&
			self.dir == other.dir
	}
}

impl Eq for Node {}

impl Hash for Node {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.x.hash(state);
		self.y.hash(state);
		self.dir.hash(state);
	}
}

#[derive(PartialEq, Eq, Hash)]
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
}

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut line_len = 0;
	let mut input = "".to_owned();

	let mut buf = String::new();

	let mut sx = 0;
	let mut sy = 0;
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
		if line.len() == 0 {
			break;
		}

		if sx == 0 {
			let start = line.find("S");
			if start.is_some() {
				sx = start.unwrap();
			} else {
				sy += 1;
			}
		}
		if ex == 0 {
			let exit = line.find("E");
			if exit.is_some() {
				ex = exit.unwrap();
			} else {
				ey += 1;
			}
		}

		input.push_str(line);
		line_len = line.len();
	}
	let num_lines = input.len() / line_len;

	// Create a map of costs.
	let input_bytes = input.as_bytes();

	let mut cost: Vec<usize> = vec![0x7f_ff_ff_ff; input_bytes.len() * 4];
	let start = Node::new(sx, sy, Direction::Right, Direction::Right);
	cost[start.cost_idx(line_len)] = 0;

	let mut to_visit = collections::HashSet::<Node>::new();
	to_visit.insert(start);

	let mut visited = collections::HashSet::<Node>::new();

	let mut exit: Option<Node> = None;

	loop {
		if to_visit.len() == 0 {
			break;
		}

		let mut heap = collections::BinaryHeap::<NodeCost>::new();
		for node in &to_visit {
			let cost = cost[node.cost_idx(line_len)];
			heap.push(node.get_node_cost(cost));
		}

		let cur: Node = match heap.pop() {
			Some(got) => to_visit.take(&got.into_node()).unwrap(),
			None => panic!("empty heap"),
		};

		let cur_cost = cost[cur.cost_idx(line_len)];

		for node in cur.next() {
			if node.x >= line_len ||
				node.y >= num_lines ||
				input_bytes[node.idx(line_len)] == b'#' ||
				visited.get(&node).is_some() {

				continue;
			}

			let old_cost = cost[node.cost_idx(line_len)];
			let new_cost = cur_cost + node.cost();

			if new_cost < old_cost {
				if node.x == ex && node.y == ey {
					match &exit {
						Some(tmp) => {
							if cost[tmp.cost_idx(line_len)] < new_cost {
								Some(Node::new(node.x, node.y, node.dir, node.dir));
							}
						},
						None => exit = Some(Node::new(node.x, node.y, node.dir, node.dir)),
					}
				}

				cost[node.cost_idx(line_len)] = new_cost;
				to_visit.insert(node);
			}
		}

		visited.insert(cur);
	}

	let exit = exit.unwrap();
	println!("part 1: {}", cost[exit.cost_idx(line_len)]);

	// Visit every node in reverse order.
	let mut to_visit = collections::HashSet::<Node>::new();
	to_visit.insert(exit);

	let mut visited = collections::HashSet::<Point>::new();

	loop {
		if to_visit.len() == 0 {
			break;
		}

		let tmp: Vec<Node> = to_visit.drain().collect();
		for cur in tmp {
			let cur_cost = cost[cur.cost_idx(line_len)];

			for node in cur.prev() {
				if node.x >= line_len ||
					node.y >= num_lines ||
					input_bytes[node.idx(line_len)] == b'#' ||
					cost[node.cost_idx(line_len)] > cur_cost {

					continue;
				}

				to_visit.insert(node);
			}
			visited.insert(Point::new(cur.x, cur.y));
		}
	}

	println!("part 2: {}", visited.len());
}
