// https://adventofcode.com/2024/day/2

use std::cmp::Ordering;

use super::*;

type Levels = Vec<usize>;

peg::parser!{

	grammar line() for str {
		rule _ = [' ']
		rule __ = _+

		rule digit() -> char = [c if c.is_ascii_digit()]

		rule level() -> usize
			= n:$(digit()+) {? n.parse().or(Err("Expected usize value")) }

		/// Matches a line with at least a pair of numbers,
		/// separated by spaces
		pub rule levels() -> Levels
			= ( level() **<2,> _ )
	}
}

pub fn solve_1(input: &str) -> String {

	type Pair = (usize,usize);

	#[derive(Clone, Copy, Debug)]
	struct Delta {
		diff:usize,
		ord:Ordering,
	}

	impl From<Pair> for Delta {
		fn from((n1,n2):Pair) -> Self {
			Delta {
				diff: n2.abs_diff(n1),
				ord: n2.cmp(&n1)
			}
		}
	}

	use itertools::*;

	let lines = Input(input).parse_iter(line::levels);

	const SAFE_RANGE:std::ops::RangeInclusive<usize> = 1..=3;

	let valid_count = lines.map(|levels| {
		// Convert pairs of level values into Delta value
		let mut deltas = levels.into_iter().tuple_windows::<Pair>().map(Delta::from);
		// Try to convert sequence into a single ordering value, if deltas are within a safe range
		deltas.try_fold(Ordering::Equal, |overall_ord, Delta { diff, ord }| {
			if SAFE_RANGE.contains(&diff) && (overall_ord == Ordering::Equal || overall_ord == ord ) {
				Some(ord)
			} else {
				None
			}
		})
	}).flatten().count();

	valid_count.to_string()
}

mod test {

	use super::*;
	use aoc_driver::Part::*;

	const EXAMPLE_INPUT:&str  =
		r###"
		7 6 4 2 1
		1 2 7 8 9
		9 7 6 2 1
		1 3 2 4 5
		8 6 4 4 1
		1 3 6 7 9
		"###;

	#[test]
	fn test_1_example() {

		let expected : &str = "2";
		let actual = solve_1(EXAMPLE_INPUT);
		assert_eq!(actual, expected);
	}

	#[test]
	fn test_submit()-> Result<(), AppError> {
		try_submit(Day(2), solve_1, Part1)?;
		Ok(())
	}
}
