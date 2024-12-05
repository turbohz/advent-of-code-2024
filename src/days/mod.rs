mod day01;

use super::*;

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
