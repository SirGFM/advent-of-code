use std::io::{self, BufRead};

const EXP10: [u64; 11] = [
	0,
	10,
	100,
	1000,
	10000,
	100000,
	1000000,
	10000000,
	100000000,
	1000000000,
	10000000000,
];

fn log10(num: u64) -> usize {
	if num == 0 {
		return 0;
	} else if num < 10 {
		return 1;
	} else if num < 100 {
		return 2;
	} else if num < 1000 {
		return 3;
	} else if num < 10000 {
		return 4;
	} else if num < 100000 {
		return 5;
	} else if num < 1000000 {
		return 6;
	} else if num < 10000000 {
		return 7;
	} else if num < 100000000 {
		return 8;
	} else if num < 1000000000 {
		return 9;
	} else if num < 10000000000 {
		return 10;
	} else if num < 100000000000 {
		return 11;
	}
	panic!("cannot calculate log10 of number")
}

fn get_div(num: u64) -> Vec<usize> {
	let mut arr = Vec::<usize>::new();
	let base = log10(num);

	let mut log = base - 1;
	while log > 0 {
		if base % log == 0 {
			arr.push(log);
		}
		log -= 1;
	}

	return arr;
}

fn gen_sep_lookup() -> Vec<Vec<usize>> {
	let mut arr = Vec::<Vec<usize>>::new();

	arr.push(Vec::<usize>::new());
	for i in 1..EXP10.len() {
		arr.push(get_div(EXP10[i] - 1));
	}

	return arr
}

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();

	handle.read_line(&mut buf).unwrap();
	let line = buf.trim();

	let lookup = gen_sep_lookup();

	let mut result_1 = 0;
	let mut result_2 = 0;

	for entry in line.split(',') {
		let values: Vec<u64> = entry
			.split("-")
			.map(|x| x.parse::<u64>().unwrap())
			.collect();

		for num in values[0]..(values[1]+1) {
			if !valid_p1(num) {
				result_1 += num;
				result_2 += num;
			} else if !valid_p2(num, &lookup) {
				result_2 += num;
			}
		}
	}

	println!("result 1: {}", result_1);
	println!("result 2: {}", result_2);
}

fn valid_p1(num: u64) -> bool {
	let log = log10(num);

	if (log & 1) == 1 {
		// Number of odd number of digits can't be divided in half.
		return true
	}
	let div = EXP10[log / 2];

	return (num / div) != (num % div);
}

fn valid_p2(num: u64, lookup: &Vec<Vec<usize>>) -> bool {
	let i = log10(num);

	for log in &lookup[i] {
		let div = EXP10[*log];

		if !is_p2_valid(num, div) {
			return false;
		}
	}

	return true;
}

fn is_p2_valid(num: u64, div: u64) -> bool {
	let base = num % div;

	let mut num = num;
	while num > 0 {
		num = num / div;

		let rem = num % div;
		if rem != base && num != 0 {
			return true;
		}
	}

	return false;
}
