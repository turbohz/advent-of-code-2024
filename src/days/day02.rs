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

type Pair = (usize,usize);

#[derive(Clone, Copy, Debug)]
struct Delta {
	dif:usize,
	ord:Ordering,
}

impl From<Pair> for Delta {
	fn from((n1,n2):Pair) -> Self {
		Delta {
			dif: n2.abs_diff(n1),
			ord: n2.cmp(&n1)
		}
	}
}

struct Deltas<'a>(Box<dyn Iterator<Item=Delta> + 'a>);

impl<'a,T:Iterator<Item=&'a usize> + 'a> From<T> for Deltas<'a> {
	fn from(value: T) -> Self {
		let i = value.into_iter().copied().tuple_windows::<Pair>().map(Delta::from);
		Self(Box::new(i))
	}
}

impl<'a> Iterator for Deltas<'a> {
	type Item = Delta;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next()
	}
}

const SAFETY_RANGE:std::ops::RangeInclusive<usize> = 1..=3;

pub fn is_safe<'a>(report: impl IntoIterator<Item=&'a usize> + 'a) -> bool {

	let mut deltas = Deltas::from(report.into_iter());

	deltas.try_fold(Ordering::Equal, |ord_prev, Delta { dif, ord }| {

		let safe = SAFETY_RANGE.contains(&dif) && (ord_prev == Ordering::Equal || ord_prev == ord );

		if safe { Some(ord) } else { None }

	}).is_some() // it folded
}

fn solve_1(input: &str) -> String {

	let lines = Input(input).parse_iter(line::levels);

	let safe_reports = lines.filter(|l| is_safe(l));

	safe_reports.count().to_string()
}

fn solve_2(input: &str) -> String {

	let lines = Input(input).parse_iter(line::levels);

	let safe_reports = lines.filter(|full_report| {

		// try full report

		is_safe(full_report) || {

			// try variations of the report, removing a single element

			let enum_report = full_report.into_iter().enumerate();

			for i in 0..full_report.len() {

				let variation = enum_report.clone()
					.filter_map(|(at,v)| if at != i { Some(v)} else { None });

				if is_safe(variation) { return true }
			}

			false
		}
	});

	safe_reports.count().to_string()
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
	fn part_1_example() {

		let expected : &str = "2";
		let actual = solve_1(EXAMPLE_INPUT);
		assert_eq!(actual, expected);
	}

	#[test]
	fn part_2_example() {

		let expected : &str = "4";
		let actual = solve_2(EXAMPLE_INPUT);
		assert_eq!(actual, expected);
	}

	#[test]
	fn submit()-> Result<(), AppError> {
		try_submit(Day(2), solve_1, Part1)?;
		try_submit(Day(2), solve_2, Part2)?;
		Ok(())
	}
}
