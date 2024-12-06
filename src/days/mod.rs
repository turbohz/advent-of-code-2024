mod day01;

use super::*;
use peg::{error::ParseError, str::LineCol};

struct Input<'a>(&'a str);
impl<'a> Input<'a> {
	/// Given a parser function, returns an iterator of parsed items of type T.
	/// Panics if parsing fails.
	fn iter<T>(&self, parse:fn(&'a str) -> Result<T,ParseError<LineCol>>)-> impl Iterator<Item=T> + use<'a,T> {
		// clean up input comming from inline examples
		let raw_lines = self.0.lines().map(str::trim_start).filter(|l| !l.is_empty());
		// return a lazy parsing iterator
		raw_lines.map(move |l| parse(l).expect("Parser should not fail"))
	}
}

fn try_submit(day:usize, solver:Solver)->Result<(),AppError> {
	use aoc_driver::{calculate_and_post, Part::*};

	let cookie: String = cookie()?;

	calculate_and_post(
		&cookie, YEAR, day as i32,Part1,
		Some(format!("inputs/{day}.txt")),
		Some(format!("cache/{day}.json")),
		solver
	).map_err(|e| {
		let msg = format!("Solution for day {day} rejected: {e:?}");
		AppError::IncorrectSolution(msg)
	})
}
