use std::io::{self, BufRead};
use regex::Regex;

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

	let mut buf = String::new();
	let mut p2_result = 0;

	let mut width = 0;
	let mut height = 0;

	let mut quadrants = vec![0; 4];

	loop {
		buf.clear();
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}

		let line = buf.trim();

		if width == 0 {
			let values: Vec<u64> = line
				.split(",")
				.map(|x| x.parse::<u64>().unwrap())
				.collect();
			assert_eq!(values.len(), 2);

			width = values[0];
			height = values[1];
			continue;
		}

		let cap = re.captures(line).unwrap();
		let sx = cap.get(1).unwrap().as_str().parse::<u64>().unwrap();
		let sy = cap.get(2).unwrap().as_str().parse::<u64>().unwrap();

		let ivx = cap.get(3).unwrap().as_str().parse::<i64>().unwrap();
		let ivy = cap.get(4).unwrap().as_str().parse::<i64>().unwrap();

		let vx = ((ivx + (width as i64)) as u64) % width;
		let vy = ((ivy + (height as i64)) as u64) % height;

		let x = (sx + vx * 100) % width;
		let y = (sy + vy * 100) % height;

		if x < width / 2 {
			if y < height / 2 {
				quadrants[0] += 1;
			} else if y > height / 2 {
				quadrants[1] += 1;
			}
		} else if x > width / 2 {
			if y < height / 2 {
				quadrants[2] += 1;
			} else if y > height / 2 {
				quadrants[3] += 1;
			}
		}
	}

	let p1_result = quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3];

	println!("part 1: {}", p1_result);
	println!("part 2: {}", p2_result);
}
