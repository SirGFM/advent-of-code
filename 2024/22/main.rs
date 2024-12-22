use std::io::{self, BufRead};
use std::collections;
use std::cmp;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Sequence {
	a: i64,
	b: i64,
	c: i64,
	d: i64,
}

impl Sequence {
	fn new(a: i64, b: i64, c: i64, d: i64) -> Sequence {
		Sequence{
			a: a,
			b: b,
			c: c,
			d: d,
		}
	}
}

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();

	let mut p1_result: u64 = 0;
	let mut sells = collections::HashMap::<Sequence, usize>::new();

	loop {
		buf.clear();
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}

		let mut vendor = collections::HashMap::<Sequence, usize>::new();
		let mut secret = buf.trim().parse::<u64>().unwrap();
		let mut diffs = Vec::<i64>::new();
		let mut last = get_digit(secret);

		for i in 0..2000 {
			secret = next(secret);

			let digit = get_digit(secret);
			let diff = digit - last;
			diffs.push(diff);
			if i >= 3 && digit > 0 {
				let instruction = Sequence::new(diffs[i-3], diffs[i-2], diffs[i-1], diffs[i]);

				if vendor.get(&instruction).is_none() {
					vendor.insert(instruction, digit as usize);
				}
			}

			last = digit;
		}

		for (k, v) in vendor.into_iter() {
			let old = sells.get(&k).or(Some(&0)).unwrap();
			sells.insert(k, old + v);
		}

		p1_result += secret;
	}

	let mut p2_result = 0;
	for (_, v) in sells.into_iter() {
		p2_result = cmp::max(v, p2_result);
	}

	println!("part 1: {}", p1_result);
	println!("part 2: {}", p2_result);
}

fn next(cur: u64) -> u64 {
	let tmp = cur * 64;
	let cur = (tmp ^ cur) & 0xff_ff_ff;

	let tmp = cur / 32;
	let cur = (tmp ^ cur) & 0xff_ff_ff;

	let tmp = cur << 11;
	let cur = (tmp ^ cur) & 0xff_ff_ff;

	return cur;
}

fn get_digit(value: u64) -> i64 {
	(value % 10) as i64
}
