use std::io::{self, BufRead};

fn main() -> io::Result<()>{
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	let mut buf = String::new();
	let mut line_num = 0;
	loop {
		let num = handle.read_line(&mut buf)?;
		if num == 0 {
			break;
		}

		let line = buf.trim();
		println!("{}. '{}'", line_num, line);

		line_num += 1;
		buf.clear();
	}

	Ok(())
}
