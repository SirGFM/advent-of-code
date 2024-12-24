use std::io::{self, BufRead};
use std::collections;

enum Operation {
	And,
	Xor,
	Or,
}

impl Operation {
	fn from_str(value: &str) -> Operation {
		match value {
			"AND" => Operation::And,
			"XOR" => Operation::Xor,
			"OR" => Operation::Or,
			_ => panic!("invalid operation"),
		}
	}

	fn exec(&self, a: u8, b: u8) -> u8 {
		match self {
			Operation::And => a & b,
			Operation::Xor => a ^ b,
			Operation::Or => a | b,
		}
	}
}

struct Input {
	name: Vec<u8>,
	value: u8,
}

impl Input {
	fn new(name: Vec<u8>, value: u8) -> Input {
		Input{
			name: name,
			value: value,
		}
	}
}

struct Gate {
	idx: usize,
	op: Operation,
	lhs: Vec<u8>,
	lhs_value: Option<u8>,
	rhs: Vec<u8>,
	rhs_value: Option<u8>,
	out: Vec<u8>,
	out_value: Option<u8>,
}

impl Gate {
	fn new(idx: usize, op: Operation, lhs: Vec<u8>, rhs: Vec<u8>, out: Vec<u8>) -> Gate{
		Gate{
			idx: idx,
			op: op,
			lhs: lhs,
			lhs_value: None,
			rhs: rhs,
			rhs_value: None,
			out: out,
			out_value: None,
		}
	}

	fn set_lhs(&mut self, lhs: u8) -> Option<u8> {
		if self.lhs_value.is_some() {
			return None;
		}

		self.lhs_value = Some(lhs);

		match self.rhs_value {
			Some(rhs) => self.out_value = Some(self.op.exec(lhs, rhs)),
			None => { /* Do nothing */ },
		}

		return self.out_value
	}

	fn set_rhs(&mut self, rhs: u8) -> Option<u8> {
		if self.rhs_value.is_some() {
			return None;
		}

		self.rhs_value = Some(rhs);

		match self.lhs_value {
			Some(lhs) => self.out_value = Some(self.op.exec(lhs, rhs)),
			None => { /* Do nothing */ },
		}

		return self.out_value
	}
}

struct GateList {
	gates: Vec<Gate>,
	input2gate: collections::HashMap<Vec<u8>, Vec<usize>>,
}

impl GateList {
	fn new() -> GateList {
		GateList{
			gates: Vec::<Gate>::new(),
			input2gate: collections::HashMap::<Vec<u8>, Vec<usize>>::new(),
		}
	}

	fn push(&mut self, op: Operation, lhs: Vec<u8>, rhs: Vec<u8>, out: Vec<u8>) {
		let idx = self.gates.len();

		self.gates.push(Gate::new(idx, op, lhs.clone(), rhs.clone(), out));
		self.set_input(lhs, idx);
		self.set_input(rhs, idx);
	}

	fn set_input(&mut self, name: Vec<u8>, idx: usize) {
		match self.input2gate.get_mut(&name) {
			Some(list) => { list.push(idx); } ,
			None => { self.input2gate.insert(name, vec![idx]); } ,
		}
	}

	fn exec(&mut self, input: &Input) -> Vec<Input> {
		let mut ret = Vec::<Input>::new();

		let gates = self.input2gate.get(&input.name);
		if gates.is_none() {
			return ret;
		}

		for idx in gates.unwrap() {
			let gate = &mut self.gates[*idx];

			let out: Option<u8>;
			if gate.lhs == input.name {
				out = gate.set_lhs(input.value);
			} else if gate.rhs == input.name {
				out = gate.set_rhs(input.value);
			} else {
				out = None;
			}

			if out.is_some() {
				let out = out.unwrap();
				ret.push(Input::new(gate.out.clone(), out));
				gate.out_value = Some(out);
			}
		}

		return ret;
	}
}

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();

	let mut gate_list = GateList::new();
	let mut inputs = collections::LinkedList::<Input>::new();

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

		let parts = line.split(": ").collect::<Vec<&str>>();
		let name = parts[0].as_bytes().to_vec();
		let value = parts[1].parse::<u8>().unwrap();

		inputs.push_back(Input::new(name, value));
	}

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

		let parts = line.split(" ").collect::<Vec<&str>>();

		let lhs = parts[0].as_bytes().to_vec();
		let op = Operation::from_str(parts[1]);
		let rhs = parts[2].as_bytes().to_vec();
		let out = parts[4].as_bytes().to_vec();

		gate_list.push(op, lhs, rhs, out);
	}

	let mut output = Vec::<u8>::new();

	while inputs.len() > 0 {
		let input = inputs.pop_front().unwrap();

		for out in gate_list.exec(&input) {
			if out.name[0] == b'z' {
				let idx = ((out.name[1] - b'0') as usize) * 10 +
					((out.name[2] - b'0') as usize);

				while idx >= output.len() {
					output.push(0);
				}
				output[idx] = out.value;
			}
			inputs.push_back(out);
		}
	}

	let mut p1_result = 0;
	for i in 0..output.len() {
		let value = output[output.len() - i - 1] as usize;

		p1_result = (p1_result << 1) | value;
	}
	println!("part 1: {}", p1_result);
}
