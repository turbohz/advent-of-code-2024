mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

use std::fmt::Display;

pub(self) use itertools::Itertools;
pub(self) use types::*;

use aoc_driver::Part;
use peg::{error::ParseError, str::LineCol};

use super::*;

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

	// Return an iterator of lines in the input
	fn lines(&self) -> impl Clone + Iterator<Item=&'a str> + use<'a> {
		// clean up input comming from inline examples
		self.0.lines().map(str::trim_start).filter(|l| !l.is_empty())
	}

	/// Given a parser function, returns an iterator of parsed items of type T.
	/// Panics if parsing fails.
	fn parse_iter<T>(&self, parse:fn(&'a str) -> Result<T,ParseError<LineCol>>)-> impl Iterator<Item=T> + use<'a,T> {

		// return a lazy parsing iterator
		self.lines().map(move |l| {
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
