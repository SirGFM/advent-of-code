use std::io::{self, BufRead};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct File {
	size: usize,
	data: Option<usize>,
}

impl File {
	fn new_data(size: usize, data: usize) -> File {
		File{
			size: size,
			data: Some(data),
		}
	}

	fn new_empty(size: usize) -> File {
		File{
			size: size,
			data: None,
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Node {
	file: File,
	idx: usize,
	next: Option<usize>,
	prev: Option<usize>,
}

impl Node {
	fn new(file: File, idx: usize, next: Option<usize>, prev: Option<usize>) -> Node{
		Node{
			file: file,
			idx: idx,
			next: next,
			prev: prev,
		}
	}
}

struct LinkedList {
	buf: Vec<Node>,
	head: Option<usize>,
	tail: Option<usize>,
}

impl LinkedList {
	fn new() -> LinkedList {
		LinkedList{
			buf: Vec::<Node>::new(),
			head: None,
			tail: None,
		}
	}

	fn to_data(&self, data: &mut Vec<usize>) {
		let mut i = 0;
		let mut node = self.get_head();
		while node.is_some() {
			let tmp = node.unwrap();

			for _ in 0..tmp.file.size {
				data[i] = match tmp.file.data {
					Some(value) => value + 1,
					None => 0,
				};

				i += 1;
			}

			node = self.get_next(tmp);
		}
	}

	fn append(&mut self, file: File) {
		if self.head.is_none() {
			let node = Node::new(file, 0, None, None);
			self.buf.push(node);
			self.head = Some(0);
			self.tail = Some(0);
		} else {
			let prev = self.tail.unwrap();
			self.insert_after(self.buf[prev], file);
		}
	}

	fn insert_after(&mut self, after: Node, file: File) {
		let idx = self.buf.len();

		let node = Node::new(file, idx, after.next, Some(after.idx));
		self.buf.push(node);

		self.buf[after.idx].next = Some(idx);

		match node.next {
			Some(next_idx) => self.buf[next_idx].prev = Some(idx),
			None => self.tail = Some(idx),
		}
	}

	fn refetch_node(&self, node: Node) -> Node {
		self.buf[node.idx]
	}

	fn update_node(&mut self, node: Node) {
		self.buf[node.idx] = node;

		if node.file.data.is_none() {
			match node.prev {
				Some(tmp) => {
					let prev = self.buf[tmp];

					if prev.file.data.is_none() {
						self.buf[node.idx].file.size += prev.file.size;
						self.remove_node(prev);
					}
				},
				_ => {/* Do nothing */},
			}

			match node.next {
				Some(tmp) => {
					let next = self.buf[tmp];

					if next.file.data.is_none() {
						self.buf[node.idx].file.size += next.file.size;
						self.remove_node(next);
					}
				},
				_ => {/* Do nothing */},
			}
		}
	}

	fn remove_node(&mut self, node: Node) {
		let node = self.refetch_node(node);
		match self.buf[node.idx].next {
			Some(idx) => self.buf[idx].prev = node.prev,
			None => self.tail = node.prev,
		}
		match self.buf[node.idx].prev {
			Some(idx) => self.buf[idx].next = node.next,
			None => self.head = node.next,
		}
	}

	fn get_head(&self) -> Option<Node> {
		match self.head {
			Some(idx) => Some(self.buf[idx]),
			None => None,
		}
	}

	fn get_tail(&self) -> Option<Node> {
		match self.tail {
			Some(idx) => Some(self.buf[idx]),
			None => None,
		}
	}

	fn get_next(&self, cur: Node) -> Option<Node> {
		let cur = self.refetch_node(cur);
		match cur.next {
			Some(idx) => Some(self.buf[idx]),
			None => None,
		}
	}

	fn get_prev(&self, cur: Node) -> Option<Node> {
		let cur = self.refetch_node(cur);
		match cur.prev {
			Some(idx) => Some(self.buf[idx]),
			None => None,
		}
	}
}

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();

	let mut files = LinkedList::new();
	let mut disk = Vec::<usize>::new();
	let mut non_empty = 0;

	loop {
		buf.clear();
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}

		let digits = buf.trim().chars().map(|x| ((x as u8) - b'0') as usize).collect::<Vec<usize>>();
		for i in 0..digits.len() {
			let size = digits[i];

			let digit: usize;
			if i % 2 == 0 {
				// File
				digit = i / 2 + 1;
				non_empty += size;
				files.append(File::new_data(size, digit - 1));
			} else {
				// Empty
				digit = 0;
				files.append(File::new_empty(size));
			}

			for _ in 0..size {
				disk.push(digit);
			}
		}
	}

	let mut end = disk.len() - 1;
	for i in 0..disk.len() {
		if i == non_empty {
			break;
		}

		while end > 0 && disk[end] == 0 {
			end -= 1;
		}
		if end == 0 {
			break;
		}

		if disk[i] == 0 {
			disk[i] = disk[end];
			disk[end] = 0;
			end -= 1;
		}
	}

	println!("part 1: {}", checksum(&disk));

	defrag_part2(&mut files);
	files.to_data(&mut disk);

	println!("part 2: {}", checksum(&disk));
}

fn checksum(disk: &Vec<usize>) -> usize {
	let mut res = 0;

	for i in 0..disk.len() {
		if disk[i] == 0 {
			continue;
		}
		res += i * (disk[i] - 1);
	}

	return res;
}

fn defrag_part2(files: &mut LinkedList) {
	let mut last = files.get_tail();

	loop {
		// Find the next node that may fit into an empty slot.
		while last.is_some() && last.unwrap().file.data.is_none() {
			last = files.get_prev(last.unwrap());
		}
		let mut last_node = match last {
			Some(node) => node,
			None => return,
		};
		let next_last = files.get_prev(last_node);

		// Find the first empty slot that would fit this file.
		let mut empty = files.get_head();
		while empty.is_some() {
			let tmp = empty.unwrap();

			if tmp.idx == last_node.idx ||
				(tmp.file.data.is_none() && tmp.file.size >= last_node.file.size) {

				break;
			}

			empty = files.get_next(tmp);
		}

		let mut empty = match empty {
			Some(tmp) => {
				if tmp.idx == last_node.idx {
					last = next_last;
					continue;
				}
				tmp
			},
			None => {
				last = next_last;
				continue;
			},
		};

		if empty.file.size > last_node.file.size {
			let remainder = File::new_empty(empty.file.size - last_node.file.size);
			files.insert_after(empty, remainder);
			empty = files.refetch_node(empty);

			empty.file.size = last_node.file.size;
		}
		empty.file.data = last_node.file.data;
		files.update_node(empty);

		last_node.file.data = None;
		files.update_node(last_node);

		last = next_last;
	}
}

fn print_disk(disk: &Vec<usize>) {
	for i in 0..disk.len() {
		if disk[i] == 0 {
			print!(".");
		} else {
			print!("{}", disk[i] - 1);
		}
	}
	println!("");
}

#[test]
fn test_ll() {
	let mut ll = LinkedList::new();

	ll.append(File::new_data(1, 0));
	assert_eq!(ll.head, Some(0));
	assert_eq!(ll.tail, Some(0));
	assert_eq!(ll.buf[0].file, File{size: 1, data: Some(0)});

	ll.append(File::new_empty(2));
	assert_eq!(ll.head, Some(0));
	assert_eq!(ll.tail, Some(1));
	assert_eq!(
		ll.buf[0],
		Node{
			file: File{size: 1, data: Some(0)},
			idx: 0,
			prev: None,
			next: Some(1),
		},
	);
	assert_eq!(
		ll.buf[1],
		Node{
			file: File{size: 2, data: None},
			idx: 1,
			prev: Some(0),
			next: None,
		},
	);

	let head = ll.get_head();
	assert_eq!(
		head,
		Some(
			Node{
				file: File{size: 1, data: Some(0)},
				idx: 0,
				prev: None,
				next: Some(1),
			},
		),
	);
	let head = head.unwrap();

	ll.insert_after(head, File::new_data(3, 4));
	assert_eq!(
		ll.buf[0],
		Node{
			file: File{size: 1, data: Some(0)},
			idx: 0,
			prev: None,
			next: Some(2),
		},
	);
	assert_eq!(
		ll.buf[2],
		Node{
			file: File{size: 3, data: Some(4)},
			idx: 2,
			prev: Some(0),
			next: Some(1),
		},
	);
	assert_eq!(
		ll.buf[1],
		Node{
			file: File{size: 2, data: None},
			idx: 1,
			prev: Some(2),
			next: None,
		},
	);

	let next = ll.get_next(head).unwrap();
	assert_eq!(
		next,
		Node{
			file: File{size: 3, data: Some(4)},
			idx: 2,
			prev: Some(0),
			next: Some(1),
		},
	);

	ll.insert_after(next, File::new_empty(5));
	assert_eq!(
		ll.buf[0],
		Node{
			file: File{size: 1, data: Some(0)},
			idx: 0,
			prev: None,
			next: Some(2),
		},
	);
	assert_eq!(
		ll.buf[2],
		Node{
			file: File{size: 3, data: Some(4)},
			idx: 2,
			prev: Some(0),
			next: Some(3),
		},
	);
	assert_eq!(
		ll.buf[3],
		Node{
			file: File{size: 5, data: None},
			idx: 3,
			prev: Some(2),
			next: Some(1),
		},
	);
	assert_eq!(
		ll.buf[1],
		Node{
			file: File{size: 2, data: None},
			idx: 1,
			prev: Some(3),
			next: None,
		},
	);

	ll.append(File::new_empty(4));
	assert_eq!(
		ll.buf[0],
		Node{
			file: File{size: 1, data: Some(0)},
			idx: 0,
			prev: None,
			next: Some(2),
		},
	);
	assert_eq!(
		ll.buf[2],
		Node{
			file: File{size: 3, data: Some(4)},
			idx: 2,
			prev: Some(0),
			next: Some(3),
		},
	);
	assert_eq!(
		ll.buf[3],
		Node{
			file: File{size: 5, data: None},
			idx: 3,
			prev: Some(2),
			next: Some(1),
		},
	);
	assert_eq!(
		ll.buf[1],
		Node{
			file: File{size: 2, data: None},
			idx: 1,
			prev: Some(3),
			next: Some(4),
		},
	);
	assert_eq!(
		ll.buf[4],
		Node{
			file: File{size: 4, data: None},
			idx: 4,
			prev: Some(1),
			next: None,
		},
	);

	let tail = ll.get_tail().unwrap();
	let empty = ll.get_prev(tail);
	assert_eq!(
		empty,
		Some(
			Node{
				file: File{size: 2, data: None},
				idx: 1,
				prev: Some(3),
				next: Some(4),
			},
		),
	);

	let empty = empty.unwrap();
	ll.update_node(empty);
	assert_eq!(
		ll.buf[0],
		Node{
			file: File{size: 1, data: Some(0)},
			idx: 0,
			prev: None,
			next: Some(2),
		},
	);
	assert_eq!(
		ll.buf[2],
		Node{
			file: File{size: 3, data: Some(4)},
			idx: 2,
			prev: Some(0),
			next: Some(1),
		},
	);
	assert_eq!(
		ll.buf[1],
		Node{
			file: File{size: 11, data: None},
			idx: 1,
			prev: Some(2),
			next: None,
		},
	);
	assert_eq!(ll.tail, Some(1));
}
