use std::io::{self, BufRead};

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();

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
			} else {
				// Empty
				digit = 0;
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
