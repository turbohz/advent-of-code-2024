// https://adventofcode.com/2024/day/7

use super::*;

#[derive(Debug,Clone,Copy)]
enum Op {
	Plus,
	Times
}

fn op_combinations(n:usize) -> impl Iterator<Item=Vec<Op>> {
	use std::iter::repeat_n;
	repeat_n([Op::Plus,Op::Times].into_iter(),n).multi_cartesian_product()
}

#[derive(Debug)]
struct Equation {
	target: usize,
	operands: Vec<usize>
}

impl Equation {
	pub fn try_solve_with(&self,ops:&Vec<Op>) -> Result<usize,()> {
		let &first = self.operands.first().expect("There should be a first operand");
		let result = self.operands.iter()
			.skip(1).zip(ops)
			.fold(first,|acc,(n,o)|
				match o {
					Op::Plus => acc + n,
					Op::Times => acc * n
				}.to_owned()
			);

		if result == self.target {
			Ok(result)
		} else {
			Err(())
		}
	}
}

peg::parser!{

	grammar line() for str {

		rule _ = [' ']
		rule __ = _+

		rule digit() -> char = [c if c.is_ascii_digit()]

		rule number() -> usize
			= n:$(digit()+) {? n.parse().or(Err("Expected usize value")) }

		/// Matches a line with a target value, followed by ':',
		/// and a list of whitespace separated numbers
		pub rule equation() -> Equation
			= target:number() ":" _ operands:(number() **<2,> _) { Equation { target, operands } }
	}
}

fn solve_1(input: &str) -> String {

	let equations = Input(input).parse_iter(line::equation);

	equations.filter_map(|eq| {
		let found = op_combinations(eq.operands.len()- 1)
			.any(|ref ops| eq.try_solve_with(ops).is_ok());
	 	if found { Some(eq.target) } else { None }
	})
	.sum::<usize>()
	.to_string()
}

mod test {

	use super::*;
	use aoc_driver::Part::*;

	const INPUT_EXAMPLE:&str =
		r###"
		190: 10 19
		3267: 81 40 27
		83: 17 5
		156: 15 6
		7290: 6 8 6 15
		161011: 16 10 13
		192: 17 8 14
		21037: 9 7 18 13
		292: 11 6 16 20
		"###;

	#[test]
	fn test() {
		let expected = "3749";
		let actual = solve_1(INPUT_EXAMPLE);
		assert_eq!(actual,expected);
	}

	#[test]
	fn submit()-> Result<(), AppError> {
		try_submit(Day(7), solve_1, Part1)?;
		// try_submit(Day(7), solve_2, Part2)?;
		Ok(())
	}
}
