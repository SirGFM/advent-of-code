use std::io::{self, BufRead};

#[derive(Clone)]
struct Node {
	value: u8,
	count: u64,
}

impl Node {
	fn new(value: u8) -> Node {
		Node{
			value: value,
			count: 0,
		}
	}

	fn beam(&mut self, parent: &Self) {
		if self.value == b'|' {
			self.count += parent.count;
		} else {
			self.value = b'|';
			if parent.value == b'|' {
				self.count = parent.count;
			} else {
				self.count = 1;
			}
		}
	}
}

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();
	let mut result_1: u64 = 0;
	let mut result_2: u64 = 0;

	let mut input = Vec::<Node>::new();
	let mut w: usize = 0;

	loop {
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}

		let line = buf.trim().as_bytes();
		w = line.len();

		for b in line {
			input.push(Node::new(*b));
		}

		buf.clear();
	}

	for i in w..input.len() {
		let parent = &input[i-w].clone();

		match parent.value {
			b'S' => input[i].beam(parent),
			b'|' => {
				match input[i].value {
					b'.' => input[i].beam(parent),
					b'^' => {
						let x = i % w;

						if x > 0 {
							input[i-1].beam(parent);
						}
						if x < w {
							input[i+1].beam(parent);
						}
					},
					b'|' => input[i].beam(parent),
					_ => {},
				}
			},
			_ => {},
		}
	}

	for i in w..input.len() {
		if input[i].value == b'^' && input[i-w].value == b'|' {
			result_1 += 1;
		}
	}

	for i in input.len()-w..input.len() {
		if input[i].value == b'|' {
			result_2 += input[i].count;
		}
	}

	println!("part 1: {}", result_1);
	println!("part 2: {}", result_2);
}
