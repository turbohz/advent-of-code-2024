// https://adventofcode.com/2024/day/3

use super::*;

#[derive(Debug)]
struct Mul(usize,usize);
impl Mul {
	pub fn compute(&self)->usize {
		self.0 * self.1
	}
}

#[derive(Debug)]
enum Op {
	Mul(usize,usize),
}

enum Chunk {
	Op(Op),
	Noise
}

impl Into<Option<Op>> for Chunk {
	fn into(self) -> Option<Op> {
		match self {
			Chunk::Op(op) => Some(op),
			_ => None
		}
	}
}

peg::parser!{

	grammar memory() for str {

		rule digit() -> char = [c if c.is_ascii_digit()]

		rule number() -> usize
			= n:$(digit()*<1,3>) {? n.parse().or(Err("Expected usize value")) }

		// A chunk that contains an Op
		rule mul() -> Chunk
			= "mul(" a:number() "," b:number() ")" { Chunk::Op(Op::Mul(a,b)) }

		// A useless char, which should be discarded
		rule noise() -> Chunk
			= [_] { Chunk::Noise }

		// Consumes a chunk, returning a possible op
		// and maybe more data
		pub rule next_op() -> (Option<Op>,Option<&'input str>)
			= chunk:(mul() / noise()) rest:$([_]*)? { (chunk.into(),rest) }
	}
}

pub fn solve_1(input: &str) -> String {

	// The example input might lead you to believe that
	// you'll get a single line, but the real input
	// spans multiple lines (contains new line characters)

	let lines = Input(input).lines();

	lines.map(|mut input| {

		let mut ops:Vec<Op> = vec![];

		while !input.is_empty() {

			let r = memory::next_op(input).unwrap();

			if let (Some(op),_) = r {
				ops.push(op);
			}

			if let (_,Some(rest)) = r {
				input = rest
			}
		}

		ops.iter().map(|op| {
			#[allow(unreachable_patterns)]
			match op {
				Op::Mul(a,b) => a*b,
				_ => panic!("Unsupported Op: {op:?}")
			}
		}).sum::<usize>()


	}).sum::<usize>().to_string()
}

mod test {

	use super::*;
	use aoc_driver::Part::*;

	const EXAMPLE_INPUT:&str =
		r###"
		xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
		"###;

	#[test]
	fn test_1_example() {

		let expected : &str = "161";
		let actual:String = solve_1(EXAMPLE_INPUT);

		assert_eq!(actual, expected);
	}

	#[test]
	fn test_submit()-> Result<(), AppError> {
		try_submit(Day(3), solve_1, Part1)?;
		Ok(())
	}
}
