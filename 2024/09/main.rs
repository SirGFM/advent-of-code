use std::io::{self, BufRead};

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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

	fn append(&mut self, file: File) {
		if self.head.is_none() {
			let node = Node::new(file, 0, None, None);
			self.buf.push(node);
			self.head = Some(0);
			self.tail = Some(0);
		} else {
			let prev = self.tail.unwrap();

			let idx = self.buf.len();
			self.buf[prev].next = Some(idx);

			let node = Node::new(file, idx, None, Some(prev));
			self.buf.push(node);
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

	fn update_node(&mut self, node: Node) {
		self.buf[node.idx] = node;
	}

	fn remove_node(&mut self, node: Node) {
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
		match cur.next {
			Some(idx) => Some(self.buf[idx]),
			None => None,
		}
	}

	fn get_prev(&self, cur: Node) -> Option<Node> {
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

	let mut p1_result = 0;
	for i in 0..disk.len() {
		if disk[i] == 0 {
			break;
		}
		p1_result += i * (disk[i] - 1);
	}

	println!("part 1: {}", p1_result);

	let mut last = files.get_tail();
	loop {
		// Find the next node that may fit into this empty slot.
		while last.is_some() && last.unwrap().file.data.is_none() {
			last = files.get_prev(last.unwrap());
		}
		let last_node = match last {
			Some(node) => node,
			None => break,
		};
		let next_last = files.get_next(last_node);

		// Find the first empty file that would fit this file.
		let mut empty = files.get_head();
		while empty.is_some() {
			let tmp = empty.unwrap();

			if tmp.idx == last_node.idx ||
				(tmp.file.data.is_none() && tmp.file.size >= last_node.file.size) {

				break;
			}

			empty = files.get_next(tmp);
		}
		if empty.is_none() {
			break;
		}
		let mut empty = empty.unwrap();

		if empty.idx == last_node.idx {
			continue;
		}

		files.remove_node(last_node);
		if last_node.file.size == empty.file.size {
			empty.file.data = last_node.file.data;
		} else {
			let remainder = File::new_empty(empty.file.size - last_node.file.size);
			files.insert_after(empty, remainder);

			empty.file.size = last_node.file.size;
		}
		files.update_node(empty);

		last = next_last;
	}

	//println!("part 2: {}", p2_result);
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
