use std::io::{self, BufRead};
use std::collections;

type PcList = collections::HashSet::<Vec<u8>>;

#[derive(PartialEq, Eq, Hash)]
struct Lan {
	nodes: Vec<Vec<u8>>,
}

impl Lan {
	fn new(a: &Vec<u8>, b: &Vec<u8>, c: &Vec<u8>) -> Lan {
		let mut tmp = vec![a.clone(), b.clone(), c.clone()];
		tmp.sort();

		return Lan{
			nodes: tmp,
		}
	}
}

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();
	let mut pcs = collections::HashMap::<Vec<u8>, PcList>::new();

	loop {
		buf.clear();
		let num = handle.read_line(&mut buf).unwrap();
		if num == 0 {
			break;
		}

		let pair = buf.trim()
			.split("-")
			.map(|x| x.as_bytes().to_vec())
			.collect::<Vec<Vec<u8>>>();

		insert(&mut pcs, pair[0].clone(), pair[1].clone());
		insert(&mut pcs, pair[1].clone(), pair[0].clone());
	}

	let mut starting_nodes = Vec::<Vec<u8>>::new();
	for pc in pcs.keys() {
		if pc[0] != b't' {
			continue;
		} else {
			starting_nodes.push(pc.clone());
		}
	}

	let mut found_lan = collections::HashSet::<Lan>::new();
	for a in starting_nodes {
		for b in pcs.get(&a).unwrap() {
			for c in pcs.get(b).unwrap() {
				if *c == a {
					continue;
				}

				for d in pcs.get(c).unwrap() {
					if *d == a {
						found_lan.insert(Lan::new(&a, b, c));
						break;
					}
				}
			}
		}
	}

	println!("part 1: {}", found_lan.len());
}

fn insert(pcs: &mut collections::HashMap<Vec<u8>, PcList>, pc: Vec<u8>, conn: Vec<u8>) {
	match pcs.get_mut(&pc) {
		Some(set) => { set.insert(conn); },
		None => { pcs.insert(pc, PcList::from([conn])); },
	}
}
