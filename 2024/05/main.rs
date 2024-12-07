use std::io::{self, BufRead};
use std::collections;
use std::cmp;

struct Page {
	page: usize,
	before: collections::HashMap::<usize, bool>,
}

type PageDict = collections::HashMap::<usize, Page>;

impl Page {
	fn new(page: usize) -> Page {
		Page {
			page: page,
			before: collections::HashMap::<usize, bool>::new(),
		}
	}

	fn add(&mut self, value: usize) {
		self.before.insert(value, true);
	}

	fn cmp(&self, other: &Self) -> cmp::Ordering {
		if self.before.get(&other.page).is_some() {
			cmp::Ordering::Less
		} else if other.before.get(&self.page).is_some() {
			cmp::Ordering::Greater
		} else if self.page < other.page {
			cmp::Ordering::Less
		} else {
			cmp::Ordering::Greater
		}
	}
}

impl Ord for Page {
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		self.cmp(other)
	}
}

impl PartialOrd for Page {
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for Page {
	fn eq(&self, other: &Self) -> bool {
		return self.page == other.page
	}
}

impl Eq for Page {}

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();
	let mut p1_result = 0;

	let mut pages = PageDict::new();

	// Read the page instructions.
	loop {
		buf.clear();
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}

		let line = buf.trim();
		if line.len() == 0 {
			break;
		}

		let values: Vec<usize> = line
			.split("|")
			.map(|x| x.parse::<usize>().unwrap())
			.collect();
		assert_eq!(values.len(), 2);

		match pages.get_mut(&values[0]) {
			Some(page) => { page.add(values[1]); },
			None => {
				let mut page = Page::new(values[0]);
				page.add(values[1]);
				pages.insert(values[0], page);

				pages.insert(values[1], Page::new(values[1]));
			},
		}
	}

	// Sort pages.
	let mut sorted_pages = Vec::<&Page>::new();
	for (_, val) in pages.iter() {
		if val.before.len() == 0 {
			println!("{}", val.page)
		}
		sorted_pages.push(val);
	}
	sorted_pages.sort_unstable();

	let mut page_pos = vec![0 as usize; 100];
	for (i, cur) in sorted_pages.iter().enumerate() {
		page_pos[cur.page] = i;
	}

	// Read manuals.
	loop {
		buf.clear();
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}

		let manual: Vec<usize> = buf.trim()
			.split(",")
			.map(|x| x.parse::<usize>().unwrap())
			.collect();

		let mut inc = true;
		let mut last: usize = 0;
		for i in 0..manual.len() {
			let page = manual[i];

			if page_pos[page] < last {
				inc = false;
				break;
			}

			last = page_pos[page]
		}

		if !inc {
			continue;
		}

		p1_result += manual[manual.len() / 2];
	}

	println!("part 1: {}", p1_result);
}
