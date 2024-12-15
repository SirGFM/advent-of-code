use std::io::{self, BufRead};
use regex::Regex;

#[derive(Clone)]
struct Params {
	x: u64,
	y: u64,
}

impl Params {
	fn new(x: u64, y: u64) -> Params {
		Params{
			x: x,
			y: y,
		}
	}
}

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let re = Regex::new(r"(\d+)[^\d]*(\d+)").unwrap();

	let mut buf = String::new();
	let mut p1_result = 0;
	let mut p2_result = 0;

	let mut i = 0;
	let mut data: Vec::<Params> = vec![Params::new(0, 0); 3];

	loop {
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}

		let line = buf.trim();
		if line.len() == 0 {
			continue;
		}

		let cap = re.captures(line).unwrap();
		let x = cap.get(1).unwrap().as_str().parse::<u64>().unwrap();
		let y = cap.get(2).unwrap().as_str().parse::<u64>().unwrap();

		data[i] = Params::new(x, y);

		buf.clear();

		i = (i + 1) % data.len();
		if i == 0 {
			let solution = solve(&data[0], &data[1], &data[2]);

			if solution.is_some() {
				let (a, b) = solution.unwrap();

				p1_result += a * 3 + b;
			}

			data[2] = Params::new(x + 10000000000000, y + 10000000000000);
			let solution = solve(&data[0], &data[1], &data[2]);

			if solution.is_some() {
				let (a, b) = solution.unwrap();

				p2_result += a * 3 + b;
			}
		}
	}

	println!("part 1: {}", p1_result);
	println!("part 2: {}", p2_result);
}

fn solve(a: &Params, b: &Params, want: &Params) -> Option<(u64, u64)> {
	let dividend: i64 = ((want.x * b.y) as i64) - ((want.y * b.x) as i64);
	let divisor: i64 = ((a.x * b.y) as i64) - ((a.y * b.x) as i64);

	if dividend % divisor != 0 {
		return None;
	}

	let p1 = (dividend / divisor) as u64;

	let dividend = want.x - p1 * a.x;
	let divisor = b.x;

	if dividend % divisor != 0 {
		return None;
	}

	let p2 = dividend / divisor;

	return Some((p1, p2));
}
