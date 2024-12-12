// https://adventofcode.com/2024/day/1

use super::*;

type Pair = (usize,usize);

peg::parser!{

	grammar line() for str {

		rule _ = [' ' | '\t']
		rule __ = _+

		rule digit() -> char = [c if c.is_ascii_digit()]

		rule id() -> usize
			= n:$(digit()+) {? n.parse().or(Err("Expected usize value")) }

		/// Matches a line from which two numbers separated
		/// by whitespace can be extracted
		pub rule pair() -> Pair
			= lid:id() __ rid:id() { (lid,rid) }
	}
}

fn solve_1(input: &str) -> String {

	let (mut left, mut right):(Vec<usize>,Vec<usize>) = Input(input).parse_iter(line::pair)
		.unzip();

	left.sort();
	right.sort();

	let sum:usize = left.iter()
		.zip(right)
		.map(|(l,r)| l.abs_diff(r))
		.sum();

	sum.to_string()
}

fn solve_2(input: &str) -> String {

	let (left, right):(Vec<usize>,Vec<usize>) = Input(input).parse_iter(line::pair)
		.unzip();

	let result:usize = left.iter()
		.map(|l| l * right.iter().filter(|r| *r == l).count())
		.sum();

	result.to_string()
}

mod test {

	use super::*;
	use aoc_driver::Part::*;

	const EXAMPLE_INPUT:&str  =
		r###"
		3   4
		4   3
		2   5
		1   3
		3   9
		3   3
		"###;

	#[test]
	fn test_1_example() {

		let expected : &str = "11";
		let actual = solve_1(EXAMPLE_INPUT);
		assert_eq!(actual, expected);
	}

	#[test]
	fn test_2_example() {

		let expected : &str = "31";
		let actual = solve_2(EXAMPLE_INPUT);
		assert_eq!(actual, expected);
	}

	#[test]
	fn test_submit()-> Result<(), AppError> {
		try_submit(Day(1), solve_1, Part1)?;
		try_submit(Day(1), solve_2, Part2)?;
		Ok(())
	}
}
