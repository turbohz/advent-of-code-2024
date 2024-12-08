mod day01;
mod day02;

use std::fmt::Display;

use super::*;
use aoc_driver::Part;
use peg::{error::ParseError, str::LineCol};

#[derive(Clone, Copy)]
struct Day(usize);

impl Into<i32> for Day {
    fn into(self) -> i32 {
        self.0 as i32
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    	write!(f, "{}", self.0)
    }
}

struct Input<'a>(&'a str);
impl<'a> Input<'a> {
	/// Given a parser function, returns an iterator of parsed items of type T.
	/// Panics if parsing fails.
	fn iter<T>(&self, parse:fn(&'a str) -> Result<T,ParseError<LineCol>>)-> impl Iterator<Item=T> + use<'a,T> {
		// clean up input comming from inline examples
		let raw_lines = self.0.lines().map(str::trim_start).filter(|l| !l.is_empty());
		// return a lazy parsing iterator
		raw_lines.map(move |l| {
			parse(l)
				.inspect_err(|e| eprintln!("Failed parsing {l}: {e}"))
				.expect("Parser should not fail")
		})
	}
}

fn try_submit(day:Day, solver:fn(&str)->String, part:Part)->Result<(),AppError> {

	let cookie: String = cookie()?;

	aoc_driver::calculate_and_post(
		&cookie, YEAR, day, part,
		Some(format!("inputs/{day}.txt")),
		Some(format!("cache/{day}.json")),
		solver
	).map_err(|e| {
		let msg = format!("Solution for day {day} rejected: {e:?}");
		AppError::IncorrectSolution(msg)
	})
}
