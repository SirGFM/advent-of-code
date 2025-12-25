use std::ascii::{escape_default};
use std::collections::{HashMap};
use std::io::{self, BufRead};

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();
	let mut result_1: u64 = 0;

	let mut map_to = HashMap::<u32, Vec<u32>>::new();

	loop {
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}

		let input: Vec<_> = buf
			.trim()
			.split(":")
			.collect();

		let code = str2code(input[0]);
		let entries = input[1]
			.trim()
			.split(" ")
			.map(|x| str2code(x))
			.collect();

		map_to.insert(code, entries);

		buf.clear();
	}

	let start = str2code("you");
	let target = str2code("out");

	let mut nodes = HashMap::<u32, u64>::new();
	nodes.insert(start, 1);

	while nodes.len() > 0 {
		let mut next = HashMap::<u32, u64>::new();

		for (node, count) in &nodes {
			let count = *count;

			for neighbour in map_to.get(&node).unwrap() {
				let neighbour = *neighbour;

				if neighbour == target {
					result_1 += count;
				} else {
					let new_count = match next.get(&neighbour) {
						Some(old) => old + count,
						None => count,
					};
					next.insert(neighbour, new_count);
				}
			}
		}

		nodes = next;
	}

	println!("part 1: {}", result_1);
}

fn str2code(value: &str) -> u32 {
	let mut output: u32 = 0;

	for v in value.as_bytes() {
		output = output << 8 | (*v as u32);
	}

	return output;
}

fn print_code(value: u32) {
	print!(
		"{}{}{}",
		escape_default((value >> 16) as u8),
		escape_default((value >> 8) as u8),
		escape_default((value) as u8),
	);
}
