use std::io::{self, BufRead};

enum Operation {
	Sum,
	Mult,
	Concat,
}

fn run(target: usize, cur: usize, op: Operation, rem: &[usize], concat: bool) -> bool {
	if rem.len() > 1 && cur >= target {
		return false;
	}

	match op {
		Operation::Sum => {
			if rem.len() == 1 {
				return cur + rem[0] == target
			} else {
				return run(target, cur + rem[0], Operation::Sum, &rem[1..], concat) ||
					run(target, cur + rem[0], Operation::Mult, &rem[1..], concat) ||
					(concat && run(target, cur + rem[0], Operation::Concat, &rem[1..], concat));
			}
		},
		Operation::Mult => {
			if rem.len() == 1 {
				return cur * rem[0] == target
			} else {
				return run(target, cur * rem[0], Operation::Sum, &rem[1..], concat) ||
					run(target, cur * rem[0], Operation::Mult, &rem[1..], concat) ||
					(concat && run(target, cur * rem[0], Operation::Concat, &rem[1..], concat));
			}
		},
		Operation::Concat => {
			let mut mult = 10;
			while rem[0] / mult > 0 {
				mult *= 10;
			}

			let cur = cur * mult;
			return run(target, cur, Operation::Sum, &rem, concat);
		}
	}
}

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();
	let mut p1_result = 0;
	let mut p2_result = 0;

	// Read the map.
	loop {
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}

		let line = buf.trim();
		let sep = line.find(":").unwrap();

		let target = line[..sep].trim()
			.parse::<usize>()
			.unwrap();
		let values: Vec<usize> = line[sep+1..].trim()
			.split(" ")
			.map(|x| x.parse::<usize>().unwrap())
			.collect();

		if run(target, values[0], Operation::Sum, &values[1..], false)  ||
			run(target, values[0], Operation::Mult, &values[1..], false)  {

			p1_result += target;
			p2_result += target;
		} else if
			run(target, values[0], Operation::Sum, &values[1..], true)  ||
			run(target, values[0], Operation::Mult, &values[1..], true)  ||
			run(target, values[0], Operation::Concat, &values[1..], true) {

			p2_result += target;
		}

		buf.clear();
	}

	println!("part 1: {}", p1_result);
	println!("part 2: {}", p2_result);
}
