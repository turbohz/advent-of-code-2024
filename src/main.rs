use std::{env, error::Error};
use aoc_driver::aoc_magic;

mod days;
use days::*;

fn main()->Result<(),Box<dyn Error>> {
	let cookie : String = env::var("COOKIE")?;
	aoc_magic!(&cookie, 2024:1:1, day01::solve)?;
	println!("All done!");
	Ok(())
}
