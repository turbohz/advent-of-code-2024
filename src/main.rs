use std::process::ExitCode;
use aoc_driver::aoc_magic;

pub use std::convert::identity;

#[cfg(test)]
mod days;

#[cfg(test)]
mod common;

pub const YEAR:i32 = 2024;

#[allow(unused)]
#[derive(Debug)]
pub enum AppError {
	BadConfiguration(String),
	IncorrectSolution(String)
}

pub fn cookie() -> Result<String,AppError> {
	std::env::var("COOKIE")
		.map_err(|e| {
			let msg = format!("Cookie error! {e:?}");
			AppError::BadConfiguration(msg)
		})
}

fn main() -> ExitCode {

	let cookie = cookie().unwrap();

	// Tests Creates `cache/` and `inputs/`
	let _ = aoc_magic!(&cookie, 2024:1:1, |_| "");

	eprintln!("Project configured.");
	eprintln!("Run `cargo test` to build and submit solutions");

	ExitCode::FAILURE
}
