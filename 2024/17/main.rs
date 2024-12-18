use std::io::{self, BufRead};
use std::convert::TryFrom;

#[derive(Debug)]
enum Opcode {
	Adv,
	Bxl,
	Bst,
	Jnz,
	Bxc,
	Out,
	Bdv,
	Cdv,
	Nop,
}

impl TryFrom<u8> for Opcode {
	type Error = &'static str;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			b'0' => Ok(Opcode::Adv),
			b'1' => Ok(Opcode::Bxl),
			b'2' => Ok(Opcode::Bst),
			b'3' => Ok(Opcode::Jnz),
			b'4' => Ok(Opcode::Bxc),
			b'5' => Ok(Opcode::Out),
			b'6' => Ok(Opcode::Bdv),
			b'7' => Ok(Opcode::Cdv),
			_ => Err("invalid opcode"),
		}
	}
}

#[derive(Debug)]
enum Operand {
	Value(usize),
	RegA,
	RegB,
	RegC,
	Reserved,
}

impl TryFrom<u8> for Operand {
	type Error = &'static str;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			b'0' | b'1' | b'2' | b'3'=> Ok(Operand::Value((value - b'0') as usize)),
			b'4' => Ok(Operand::RegA),
			b'5' => Ok(Operand::RegB),
			b'6' => Ok(Operand::RegC),
			b'7' => Ok(Operand::Reserved),
			_ => Err("invalid operand"),
		}
	}
}

impl Operand {
	fn get(&self, reg_a: usize, reg_b: usize, reg_c: usize) -> usize {
		match self {
			Operand::Value(value) => *value,
			Operand::RegA => reg_a,
			Operand::RegB => reg_b,
			Operand::RegC => reg_c,
			Operand::Reserved => panic!("invalud operand"),
		}
	}

	fn literal(&self) -> usize {
		match self{
			Operand::Value(value) => *value,
			Operand::RegA => 4,
			Operand::RegB => 5,
			Operand::RegC => 6,
			Operand::Reserved => 7,
		}
	}
}

impl Opcode {
	fn exec(&self, value: Operand, reg_a: usize, reg_b: usize, reg_c: usize) -> (i32, usize, usize, usize, i32) {
		let mut new_a = reg_a;
		let mut new_b = reg_b;
		let mut new_c = reg_c;
		let mut new_ptr: i32 = -1;
		let mut out: i32 = -1;

		match self {
			Opcode::Adv => {
				new_a = reg_a / (1 << value.get(reg_a, reg_b, reg_c));
			},
			Opcode::Bxl => {
				new_b = reg_b ^ value.literal();
			},
			Opcode::Bst => {
				new_b = value.get(reg_a, reg_b, reg_c) & 0x7;
			},
			Opcode::Jnz => {
				if reg_a != 0 {
					new_ptr = value.literal() as i32;
				}
			},
			Opcode::Bxc => {
				new_b = reg_b ^ reg_c;
			},
			Opcode::Out => {
				out = (value.get(reg_a, reg_b, reg_c) & 0x7) as i32;
			},
			Opcode::Bdv => {
				new_b = reg_a / (1 << value.get(reg_a, reg_b, reg_c));
			},
			Opcode::Cdv => {
				new_c = reg_a / (1 << value.get(reg_a, reg_b, reg_c));
			},
			Opcode::Nop => panic!("invalid opcode"),
		}

		//println!("a:{reg_a} b:{reg_b} c:{reg_c} - {:?} {:?} - pos:{new_ptr} a:{new_a} b:{new_b} c:{new_c} out:{out}", self, value);

		return (new_ptr, new_a, new_b, new_c, out);
	}
}

fn main() {
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();
	let mut p2_result = 0;

	let mut reg_a;
	let mut reg_b;
	let mut reg_c;

	let mut cmd: Opcode = Opcode::Nop;

	reg_a = read_line(&mut buf, &mut handle)[2].parse::<usize>().unwrap();
	reg_b = read_line(&mut buf, &mut handle)[2].parse::<usize>().unwrap();
	reg_c = read_line(&mut buf, &mut handle)[2].parse::<usize>().unwrap();

	read_line(&mut buf, &mut handle);

	let prog: &str = read_line(&mut buf, &mut handle)[1];
	let prog_bytes = prog.as_bytes();

	let mut out = Vec::<usize>::new();

	let mut ptr = 0;
	while ptr < prog_bytes.len() {
		if ptr % 4 == 0 {
			cmd = Opcode::try_from(prog_bytes[ptr]).unwrap();
			ptr += 2
		} else if ptr % 4 == 2 {
			let value = Operand::try_from(prog_bytes[ptr]).unwrap();

			let new_ptr: i32;
			let new_out: i32;
			(new_ptr, reg_a, reg_b, reg_c, new_out) = cmd.exec(value, reg_a, reg_b, reg_c);

			if new_ptr >= 0 {
				ptr = new_ptr as usize;
			} else {
				ptr += 2;
			}

			if new_out >= 0 {
				out.push(new_out as usize);
			}
		}
	}

	print!("part 1: ");

	for v in out.into_iter() {
		print!("{v},");
	}

	println!("");
}

fn read_line<'a, T>(buf: &'a mut String, handle: &'a mut T) -> Vec<&'a str>
where
	T: BufRead
{
	buf.clear();
	handle.read_line(buf).unwrap();
	let v: Vec<&str> = buf.trim()
		.split(" ")
		.collect();

	return v;
}
