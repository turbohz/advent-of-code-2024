// https://adventofcode.com/2024/day/3

use super::*;

#[derive(Debug)]
struct Mul(usize,usize);
impl Mul {
	pub fn compute(&self)->usize {
		self.0 * self.1
	}
}

peg::parser!{

	grammar memory() for str {

		rule digit() -> char = [c if c.is_ascii_digit()]

		rule number() -> usize
			= n:$(digit()*<1,3>) {? n.parse().or(Err("Expected usize value")) }

		rule mul() -> Mul
			= "mul(" a:number() "," b:number() ")" { Mul(a,b) }

		rule corruption_span()
			// Does NOT start with a mul() and is a sequence
			// of zero or more "non-followed-by mul" characters,
			// and a single "followeded-by mul character
			= !mul() ([_]!mul())* [_]&mul()

		/// Matches an optional `corruption_span` sequence followed by a
		/// valid `mul` operator, and what comes after (if anything)
		pub rule next_mul() -> (Mul,Option<&'input str>)
			= corruption_span()? m:mul() rest:$([_]*)? { (m,rest) }
	}
}

pub fn solve_1(input: &str) -> String {

	// The example input might lead you to believe that
	// you'll get a single line, but the real input
	// spans multiple lines (contains new line characters)

	let input = Input(input).lines();

	input.map(|mut l| {

		let mut sum:usize = 0;

		// parse iteratively, obtaining and executing the next mul
		// accumulating its values

		loop {

			let Ok((mul,maybe_rest)) = memory::next_mul(l) else {
				// We are done: `l` contains leftovers without a `mul` op
				break sum;
			};

			sum += mul.compute();

			if let Some(rest) = maybe_rest {
				l = rest;
			} else {
				// We are done: no more str to parse, it ended with a valid `mul` op
				break sum;
			}
		}
	}).sum::<usize>().to_string()
}

mod test {

	use super::*;
	use aoc_driver::Part::*;

	const EXAMPLE_INPUT:&str =
		r###"
		xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
		"###;

	#[test]
	fn test_1_example() {

		let expected : &str = "161";
		let actual:String = solve_1(EXAMPLE_INPUT);

		assert_eq!(actual, expected);
	}

	#[test]
	fn test_submit()-> Result<(), AppError> {
		try_submit(Day(3), solve_1, Part1)?;
		Ok(())
	}
}
