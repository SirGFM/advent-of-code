use std::io::{self, BufRead};
use std::cmp;

#[derive(Clone, Debug)]
struct Range {
	min: u64,
	max: u64,
}

impl Range{
	fn new(min: u64, max: u64) -> Range {
		Range{
			min: min,
			max: max,
		}
	}

	fn is_fresh(&self, value: u64) -> bool {
		return value >= self.min && value <= self.max;
	}

	fn merge(&self, other: &Self) -> Option<Self> {
		if self.min <= other.min && self.max >= other.max {
			// sMin --- oMin --- oMax --- sMax
			Some(self.clone())
		} else if self.min > other.min && self.max < other.max {
			// oMin --- sMin --- sMax --- oMax
			Some(other.clone())
		} else if self.min < other.min && self.max < other.max {
			if self.max >= other.min {
				// sMin --- oMin --- sMax --- oMax
				Some(Range::new(self.min, other.max))
			} else {
				// sMin --- sMax --- oMin --- oMax
				None
			}
		} else if self.min > other.min && self.max > other.max {
			if self.min <= other.max {
				// oMin --- sMin --- oMax --- sMax
				Some(Range::new(other.min, self.max))
			} else {
				// oMin --- oMax --- sMin --- sMax
				None
			}
		} else {
			None
		}
	}

	fn len(&self) -> u64 {
		self.max - self.min + 1
	}
}

impl PartialOrd for Range {
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		if self.min < other.min {
			Some(cmp::Ordering::Less)
		} else if self.min > other.min {
			Some(cmp::Ordering::Greater)
		} else {
			if self.max < other.max {
				Some(cmp::Ordering::Less)
			} else if self.max > self.max {
				Some(cmp::Ordering::Greater)
			} else {
				Some(cmp::Ordering::Equal)
			}
		}
	}
}

impl Ord for Range {
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		self.partial_cmp(other).unwrap()
	}
}

impl PartialEq for Range {
	fn eq(&self, other: &Self) -> bool {
		return self.min == other.min && self.max == other.max
	}
}

impl Eq for Range {}

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();
	let mut ranges = Vec::<Range>::new();

	let mut result_1: u64 = 0;
	let mut result_2: u64 = 0;

	loop {
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}
		let line = buf.trim();

		if line.len() == 0 {
			break;
		}

		let range: Vec<_> = line.split("-").collect();
		ranges.push(Range::new(range[0].parse::<u64>().unwrap(), range[1].parse::<u64>().unwrap()));

		buf.clear();
	}
	buf.clear();

	loop {
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}
		let line = buf.trim();

		let value = line.parse::<u64>().unwrap();
		for range in &ranges {
			if range.is_fresh(value) {
				result_1 += 1;
				break;
			}
		}

		buf.clear();
	}

	// Part 2:
	// 1. Sort ranges
	// 2. Normalize ranges (i.e., merge adjacent ranges)
	// 3. Reverse
	// 4. Normalize again
	// 5. Count diff (inclusive) in ranges
	ranges.sort();

	ranges = normalize_ranges(&ranges);

	ranges.reverse();
	ranges = normalize_ranges(&ranges);

	for range in ranges {
		result_2 += range.len();
	}

	println!("part 1: {}", result_1);
	println!("part 2: {}", result_2);
}

fn normalize_ranges(src: &[Range]) -> Vec<Range> {
	let mut normalized = Vec::<Range>::new();
	let mut ranges = Vec::<Range>::new();

	for range in src {
		ranges.push(range.clone());
	}

	for i in 0..ranges.len()-1 {
		match ranges[i].merge(&ranges[i+1]) {
			Some(merged) => ranges[i+1] = merged,
			None => normalized.push(ranges[i].clone()),
		}
	}
	normalized.push(ranges[ranges.len()-1].clone());

	return normalized;
}
