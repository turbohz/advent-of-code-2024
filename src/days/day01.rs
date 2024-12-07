// https://adventofcode.com/2024/day/1

use super::*;

type Pair = (usize,usize);

peg::parser!{

	grammar parse() for str {

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

pub fn solve(input: &str) -> String {

	let (mut left, mut right):(Vec<usize>,Vec<usize>) = Input(input).iter(parse::pair)
		.unzip();

	left.sort();
	right.sort();

	let sum:usize = left.iter()
		.zip(right)
		.map(|(l,r)| l.abs_diff(r))
		.sum();

	sum.to_string()
}

mod test {

	use super::*;

	#[test]
	fn test_example() {

		let input =
		r###"
		3   4
		4   3
		2   5
		1   3
		3   9
		3   3
		"###;

		let expected : &str = "11";
		let actual = solve(input);
		assert_eq!(actual, expected);
	}

	#[test]
	fn test_submit()-> Result<(), AppError> {
		try_submit(1, &solve)
	}
}
