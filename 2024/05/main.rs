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

	fn cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		if self.before.get(&other.page).is_some() {
			Some(cmp::Ordering::Less)
		} else if other.before.get(&self.page).is_some() {
			Some(cmp::Ordering::Greater)
		} else {
			None
		}
	}
}

impl Ord for Page {
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		self.cmp(other).unwrap()
	}
}

impl PartialOrd for Page {
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		self.cmp(other)
	}
}

impl PartialEq for Page {
	fn eq(&self, other: &Self) -> bool {
		return self.page == other.page
	}
}

impl Eq for Page {}

fn insert_page(pages: &mut PageDict, page: usize, value: Option<usize>) {
	match value {
		Some(v) => {
			match pages.get_mut(&page) {
				Some(tmp) => {
					tmp.add(v);
					pages.get_mut(&page).unwrap().before.get(&v).unwrap();
				},
				None => {
					let mut tmp = Page::new(page);
					tmp.add(v);
					pages.insert(page, tmp);
				},
			}
			insert_page(pages, v, None);
		},
		None => {
			if pages.get_mut(&page).is_none() {
				pages.insert(page, Page::new(page));
			}
		},
	}
}

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();
	let mut p1_result = 0;
	let mut p2_result = 0;

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

		insert_page(&mut pages, values[0], Some(values[1]));
	}

/* XXX: This doesn't sort nicely as I'd assumed...

	// Sort pages.
	let mut sorted_pages = Vec::<&Page>::new();
	for (_, val) in pages.iter() {
		sorted_pages.push(val);
	}
	sorted_pages.sort_unstable();

	let mut page_pos = vec![0 as usize; 100];
	for (i, cur) in sorted_pages.iter().enumerate() {
		page_pos[cur.page] = i;
	}
*/

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
		for i in 0..manual.len() {
			let cur = pages.get(&manual[i]).unwrap();

			for j in 0..manual.len() {
				if i == j {
					continue
				}

				let other = pages.get(&manual[j]).unwrap();
				inc = match cur.cmp(other) {
					Some(cmp::Ordering::Less) => i < j,
					Some(cmp::Ordering::Greater) => i > j,
					_ => false,
				};

				if !inc {
					break;
				}
			}

			if !inc {
				break;
			}
		}

		if inc {
			p1_result += manual[manual.len() / 2];
		} else {
			let mut manual_pages: Vec<&Page> = manual
				.into_iter()
				.map(|x| pages.get(&x).unwrap())
				.collect();

			manual_pages.sort();
			p2_result += manual_pages[manual_pages.len() / 2].page;
		}
	}

	println!("part 1: {}", p1_result);
	println!("part 2: {}", p2_result);
}
