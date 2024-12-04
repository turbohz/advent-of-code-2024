// https://adventofcode.com/2024/day/1

peg::parser!{
	grammar parse() for str {

		rule _ = [' ' | '\t']
		rule __ = _+
		rule ___ = _*

		rule digit() -> char = [c if c.is_ascii_digit()]

		rule id() -> usize
			= n:$(digit()+) {? n.parse().or(Err("Expected usize value")) }

		/// Matches a line from which two numbers separated
		/// by whitespace can be extracted
		rule id_pair() -> Option<[usize;2]>
			= ___ lid:id() __ rid:id() { Some([lid,rid]) }

		/// Matches an "empty" line
		rule empty() -> Option<[usize;2]> = ___ { None }

		pub rule pairs() -> Vec<[usize;2]>
			= r:( (id_pair() / empty()) ++ "\n" ) { r.into_iter().flatten().collect() }
	}
}

pub fn solve(input: &str) -> String {

	use itertools::Itertools as _;

	let pairs:Vec<[usize;2]> = parse::pairs(input).unwrap();

	let (mut left, mut right):(Vec<usize>,Vec<usize>) = pairs.iter().map(|[l,r]|(l,r)).unzip();
	left.sort();
	right.sort();

	let sum:usize = left.iter()
		.zip_eq(right)
		.map(|(l,r)| l.abs_diff(r))
		.sum();

	sum.to_string()
}

#[cfg(test)]
mod test {

	use super::*;
	use crate::AppError;

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
		crate::try_submit(1, &solve)
	}
}
