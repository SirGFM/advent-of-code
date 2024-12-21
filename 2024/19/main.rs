use std::io::{self, BufRead};
use std::str;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Node {
	value: u8,
	end: bool,
	next: Option<usize>,
	other: Option<usize>,
}

impl Node {
	fn new(value: u8, end: bool) -> Node {
		Node{
			value: value,
			end: end,
			next: None,
			other: None,
		}
	}

	fn new_connected(value: u8, end: bool, next: Option<usize>, other: Option<usize>) -> Node {
		Node{
			value: value,
			end: end,
			next: next,
			other: other,
		}
	}
}

struct Trie {
	buf: Vec<Node>,
}

impl Trie {
	fn new() -> Trie {
		Trie{
			buf: Vec::<Node>::new(),
		}
	}

	fn search(&self, pattern: &str) -> bool {
		let values = pattern.as_bytes();

		let mut last = 0;
		let mut idx = 0;
		let mut i = 0;

		while i < values.len() {
			last = idx;
			let node = &self.buf[idx];

			if values[i] == node.value {
				i += 1;
				match node.next {
					Some(tmp) => idx = tmp,
					None => break,
				}
			} else {
				match node.other {
					Some(tmp) => idx = tmp,
					None => return false,
				}
			}
		}

		return i == values.len() && self.buf[last].end
	}

	fn insert(&mut self, pattern: &str) {
		let mut last = 0;
		let mut idx = 0;

		let values = pattern.as_bytes();
		if self.buf.len() == 0 {
			let node = Node::new(values[0], values.len() == 1);
			self.buf.push(node);
			self.insert_next(&values[1..], 0);
			return;
		}

		let mut i = 0;

		while i < values.len() {
			last = idx;
			let node = &self.buf[idx];

			if values[i] == node.value {
				i += 1;
				match node.next {
					Some(tmp) => idx = tmp,
					None => {
						self.insert_next(&values[i..], idx);
						return;
					},
				}
			} else {
				match node.other {
					Some(tmp) => idx = tmp,
					None => {
						self.insert_other(&values[i..], idx);
						return;
					},
				}
			}
		}

		// Reached some node that is valid, but wasn't the end of a pattern.
		self.buf[last].end = true;
	}

	fn insert_other(&mut self, values: &[u8], idx: usize) {
		let new_idx = self.buf.len();
		self.buf[idx].other = Some(new_idx);

		let node = Node::new(values[0], values.len() == 1);
		self.buf.push(node);

		self.insert_next(&values[1..], new_idx);
	}

	fn insert_next(&mut self, values: &[u8], idx: usize) {
		let mut idx = idx;

		for i in 0..values.len() {
			let v = values[i];

			let new_idx = self.buf.len();
			self.buf[idx].next = Some(new_idx);

			let node = Node::new(v, i == values.len() - 1);
			self.buf.push(node);

			idx = new_idx;
		}
	}
}

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();
	let mut trie = Trie::new();

	// Read the patterns.
	buf.clear();
	handle.read_line(&mut buf).unwrap();

	for p in buf.trim().split(",").map(|x| x.trim()).collect::<Vec<&str>>() {
		trie.insert(p);
	}

	// Skip the empty line.
	buf.clear();
	handle.read_line(&mut buf).unwrap();

	let mut p1_result = 0;

	loop {
		buf.clear();
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}

		let line = buf.trim();
		if is_possible(&trie, line.as_bytes()) {
			p1_result += 1;
		}
	}

	println!("part 1: {}", p1_result);
}

fn is_possible(trie: &Trie, pattern: &[u8]) -> bool {
	for i in 1..pattern.len()+1 {
		let tmp = str::from_utf8(&pattern[..i]).unwrap();
		if trie.search(tmp) && is_possible(&trie, &pattern[i..]) {
			return true
		}
	}
	return pattern.len() == 0;
}

#[test]
fn test_trie() {
	let mut trie = Trie::new();

	trie.insert("r");
	assert_eq!(trie.buf, vec![Node::new(b'r', true)]);
	assert_eq!(trie.search("r"), true);
	assert_eq!(trie.search("wr"), false);

	trie.insert("wr");
	assert_eq!(
		trie.buf,
		vec![
			Node::new_connected(b'r', true, None, Some(1)),
			Node::new_connected(b'w', false, Some(2), None),
			Node::new(b'r', true),
		],
	);
	assert_eq!(trie.search("r"), true);
	assert_eq!(trie.search("w"), false);
	assert_eq!(trie.search("wr"), true);
	assert_eq!(trie.search("b"), false);

	trie.insert("b");
	assert_eq!(
		trie.buf,
		vec![
			Node::new_connected(b'r', true, None, Some(1)),
			Node::new_connected(b'w', false, Some(2), Some(3)),
			Node::new_connected(b'r', true, None, None),
			Node::new(b'b', true),
		],
	);
	assert_eq!(trie.search("r"), true);
	assert_eq!(trie.search("wr"), true);
	assert_eq!(trie.search("b"), true);
	assert_eq!(trie.search("g"), false);

	trie.insert("g");
	assert_eq!(
		trie.buf,
		vec![
			Node::new_connected(b'r', true, None, Some(1)),
			Node::new_connected(b'w', false, Some(2), Some(3)),
			Node::new_connected(b'r', true, None, None),
			Node::new_connected(b'b', true, None, Some(4)),
			Node::new(b'g', true),
		],
	);
	assert_eq!(trie.search("r"), true);
	assert_eq!(trie.search("wr"), true);
	assert_eq!(trie.search("b"), true);
	assert_eq!(trie.search("g"), true);
	assert_eq!(trie.search("bwu"), false);

	trie.insert("bwu");
	assert_eq!(
		trie.buf,
		vec![
			Node::new_connected(b'r', true, None, Some(1)),
			Node::new_connected(b'w', false, Some(2), Some(3)),
			Node::new_connected(b'r', true, None, None),
			Node::new_connected(b'b', true, Some(5), Some(4)),
			Node::new(b'g', true),
			Node::new_connected(b'w', false, Some(6), None),
			Node::new(b'u', true),
		],
	);
	assert_eq!(trie.search("r"), true);
	assert_eq!(trie.search("wr"), true);
	assert_eq!(trie.search("b"), true);
	assert_eq!(trie.search("g"), true);
	assert_eq!(trie.search("bw"), false);
	assert_eq!(trie.search("bwu"), true);
	assert_eq!(trie.search("rb"), false);

	trie.insert("rb");
	assert_eq!(
		trie.buf,
		vec![
			Node::new_connected(b'r', true, Some(7), Some(1)),
			Node::new_connected(b'w', false, Some(2), Some(3)),
			Node::new_connected(b'r', true, None, None),
			Node::new_connected(b'b', true, Some(5), Some(4)),
			Node::new(b'g', true),
			Node::new_connected(b'w', false, Some(6), None),
			Node::new(b'u', true),
			Node::new(b'b', true),
		],
	);
	assert_eq!(trie.search("r"), true);
	assert_eq!(trie.search("wr"), true);
	assert_eq!(trie.search("b"), true);
	assert_eq!(trie.search("g"), true);
	assert_eq!(trie.search("bw"), false);
	assert_eq!(trie.search("bwu"), true);
	assert_eq!(trie.search("rb"), true);
	assert_eq!(trie.search("gb"), false);

	trie.insert("gb");
	assert_eq!(
		trie.buf,
		vec![
			Node::new_connected(b'r', true, Some(7), Some(1)),
			Node::new_connected(b'w', false, Some(2), Some(3)),
			Node::new_connected(b'r', true, None, None),
			Node::new_connected(b'b', true, Some(5), Some(4)),
			Node::new_connected(b'g', true, Some(8), None),
			Node::new_connected(b'w', false, Some(6), None),
			Node::new(b'u', true),
			Node::new(b'b', true),
			Node::new(b'b', true),
		],
	);
	assert_eq!(trie.search("r"), true);
	assert_eq!(trie.search("wr"), true);
	assert_eq!(trie.search("b"), true);
	assert_eq!(trie.search("g"), true);
	assert_eq!(trie.search("bw"), false);
	assert_eq!(trie.search("bwu"), true);
	assert_eq!(trie.search("rb"), true);
	assert_eq!(trie.search("gb"), true);
	assert_eq!(trie.search("br"), false);

	trie.insert("br");
	assert_eq!(
		trie.buf,
		vec![
			Node::new_connected(b'r', true, Some(7), Some(1)),
			Node::new_connected(b'w', false, Some(2), Some(3)),
			Node::new_connected(b'r', true, None, None),
			Node::new_connected(b'b', true, Some(5), Some(4)),
			Node::new_connected(b'g', true, Some(8), None),
			Node::new_connected(b'w', false, Some(6), Some(9)),
			Node::new(b'u', true),
			Node::new(b'b', true),
			Node::new(b'b', true),
			Node::new(b'r', true),
		],
	);
	assert_eq!(trie.search("r"), true);
	assert_eq!(trie.search("wr"), true);
	assert_eq!(trie.search("b"), true);
	assert_eq!(trie.search("g"), true);
	assert_eq!(trie.search("bw"), false);
	assert_eq!(trie.search("bwu"), true);
	assert_eq!(trie.search("rb"), true);
	assert_eq!(trie.search("gb"), true);
	assert_eq!(trie.search("br"), true);

	trie.insert("bw");
	assert_eq!(
		trie.buf,
		vec![
			Node::new_connected(b'r', true, Some(7), Some(1)),
			Node::new_connected(b'w', false, Some(2), Some(3)),
			Node::new_connected(b'r', true, None, None),
			Node::new_connected(b'b', true, Some(5), Some(4)),
			Node::new_connected(b'g', true, Some(8), None),
			Node::new_connected(b'w', true, Some(6), Some(9)),
			Node::new(b'u', true),
			Node::new(b'b', true),
			Node::new(b'b', true),
			Node::new(b'r', true),
		],
	);
	assert_eq!(trie.search("r"), true);
	assert_eq!(trie.search("wr"), true);
	assert_eq!(trie.search("b"), true);
	assert_eq!(trie.search("g"), true);
	assert_eq!(trie.search("bw"), true);
	assert_eq!(trie.search("bwu"), true);
	assert_eq!(trie.search("rb"), true);
	assert_eq!(trie.search("gb"), true);
	assert_eq!(trie.search("br"), true);
}
