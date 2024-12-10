// https://adventofcode.com/2024/day/3

// The example input might lead you to believe that
// you'll get a single line, but the real input
// spans multiple lines (contains new line characters)

use super::*;

#[derive(Debug)]
enum Op {
	MulAllow(bool),
	Mul(usize,usize),
}

enum Chunk {
	Op(Op),
	Noop
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

		rule mul() -> Op
			= "mul(" a:number() "," b:number() ")" { Op::Mul(a,b) }

		rule do () -> Op
			= "do()" { Op::MulAllow(true) }

		rule dont () -> Op
			= "don't()" { Op::MulAllow(false) }

		rule op () -> Chunk
			= op:(do() / dont() / mul()) { Chunk::Op(op) }

		// A useless char, which should be discarded
		rule noop() -> Chunk
		= [_] { Chunk::Noop }

		// Consumes a chunk, returning a possible op
		// and maybe more data
		pub rule next_op() -> (Option<Op>,Option<&'input str>)
			= chunk:(op() / noop()) rest:$([_]*)? { (chunk.into(),rest) }
	}
}

fn ops_from(mut input: &str) -> impl Iterator<Item=Op> {

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

	ops.into_iter()
}

pub fn solve_1(input: &str) -> String {

	let lines = Input(input).lines();

	lines.map(|input| {

		ops_from(input).flat_map(|op| {
			#[allow(unreachable_patterns)]
			match op {
				Op::Mul(a,b) => Some(a*b),
				_ => None
			}
		}).sum::<usize>()


	}).sum::<usize>().to_string()
}

pub fn solve_2(input: &str) -> String {

	let lines = Input(input).lines();

	// mull allowed state must be carried over
	// different lines
	let mut mul_allowed = true;

	lines.map(|input| {

		let mut sum:usize = 0;

		for op in ops_from(input) {
			match op {
				Op::MulAllow(allow) => mul_allowed = allow,
				Op::Mul(a,b) if mul_allowed => sum += a*b,
				_ => ()
			}
		}

		sum

	}).sum::<usize>().to_string()
}

mod test {

	use super::*;
	use aoc_driver::Part::*;

	#[test]
	fn part_1_example() {

		const INPUT:&str =
			r###"
			xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
			"###;

		let expected : &str = "161";
		let actual:String = solve_1(INPUT);

		assert_eq!(actual, expected);
	}

	#[test]
	fn part_2_example() {

		const INPUT:&str =
			r###"
			xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
			"###;

		let expected : &str = "48";
		let actual:String = solve_2(INPUT);

		assert_eq!(actual, expected);
	}

	#[test]
	fn test_submit()-> Result<(), AppError> {
		try_submit(Day(3), solve_1, Part1)?;
		try_submit(Day(3), solve_2, Part2)?;
		Ok(())
	}
}
