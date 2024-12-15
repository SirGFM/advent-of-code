use std::io::{self, BufRead};
use std::fmt::Write;

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut p2_result = 0;

	let mut stones: Vec<u64>;

	// Read the input.
	let mut buf = String::new();
	buf.clear();
	handle.read_line(&mut buf).unwrap();

	let line = buf.trim();
	stones = line
		.split(" ")
		.map(|x| x.parse::<u64>().unwrap())
		.collect();

	for _ in 0..25 {
		let mut new_stones: Vec<u64> = Vec::<u64>::new();

		for stone in stones.iter() {
			let value = *stone;

			if value == 0 {
				new_stones.push(1);
			} else {
				buf.clear();
				write!(&mut buf, "{}", value).unwrap();

				//let digits = (value as f64).log10().ceil() as u32;
				let digits = buf.len() as u32;

				if digits % 2 == 0 {
					let div = (10 as u64).pow(digits / 2);

					new_stones.push(value / div);
					new_stones.push(value % div);
				} else {
					new_stones.push(value * 2024);
				}
			}
		}

		stones = new_stones;
	}

	println!("part 1: {}", stones.len());
	println!("part 2: {}", p2_result);
}

fn print_arr(arr: &Vec<u64>) {
	for v in arr.iter() {
		print!("{} ", *v);
	}
	println!("");
}
