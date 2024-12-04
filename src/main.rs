use std::{
	error::Error,
	process::ExitCode
};

mod days;

const YEAR:i32 = 2024;

#[derive(Debug)]
enum AppError {
	BadConfiguration(String),
	IncorrectSolution(String)
}

pub type Solver = &'static dyn Fn(&str)->String;

fn try_submit(day:usize, solver:Solver)->Result<(),AppError> {
	use aoc_driver::{calculate_and_post, Part::*};
	use std::env;

	let cookie: String = env::var("COOKIE")
		.map_err(|e| {
			let msg = format!("Cookie error! {e:?}");
			AppError::BadConfiguration(msg)
		})?;

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

fn main() -> ExitCode {

	eprintln!("You're not supposed to run this program!");
	eprintln!("Execute the test suite with: cargo test");

	ExitCode::FAILURE
}
