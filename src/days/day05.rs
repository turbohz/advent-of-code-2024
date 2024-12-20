// https://adventofcode.com/2024/day/5

use std::ops::{Deref, DerefMut};

use super::*;

#[derive(Debug,Clone,Copy)]
struct Rule(u8,u8);

struct RuleSet(Vec<Rule>);

impl From<Vec<Rule>> for RuleSet {
	fn from(value: Vec<Rule>) -> Self {
		RuleSet(value)
	}
}

impl Deref for RuleSet {
	type Target = Vec<Rule>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for RuleSet {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl RuleSet {
	fn applicable_to<'a>(&'a self, update:&'a Update) -> impl Iterator<Item=&'a Rule> + Clone {
		self.iter().filter(|Rule(a,b)| {
			update.contains(&a) && update.contains(&b)
		})
	}
}

#[derive(Debug)]
struct Update(Vec<u8>);

impl Deref for Update {
	type Target = Vec<u8>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for Update {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

peg::parser!{

	grammar rules() for str {

		rule digit() -> char = [c if c.is_ascii_digit()]

		rule number() -> u8
			= n:$(digit()*<2>) {? n.parse().or(Err("Expected 2 digits usize value")) }

		pub rule pair() -> Rule
			= a:number() "|" b:number() { Rule(a,b) }
	}
}

peg::parser!{

	grammar update() for str {

		rule digit() -> char = [c if c.is_ascii_digit()]

		rule number() -> u8
			= n:$(digit()*<2>) {? n.parse().or(Err("Expected 2 digits usize value")) }

		pub rule pages() -> Update
			= p:(number() ** ",") { Update(p) }
	}
}

fn solve_1(input: &str) -> String {

	use core::iter::once;

	// empty line separating rules from
	// pages lists gets removed by lines()
	let mut lines = Input(input).lines();

	let mut rules:RuleSet = RuleSet(vec![]);

	let updates = loop {

		let line = lines.next()
			.expect("Rules should precede start of update lines (which breaks the loop");

		if let Ok(rule) = rules::pair(line) {
			rules.push(rule);
		} else {
			// The first update line has been
			// consumed, so must be reinserted
			break once(line).chain(lines)
				// create an iterator of Update
				.map(|line| update::pages(line).unwrap());
		}

	};

	updates
		.filter(|update| {
			let rules = rules.applicable_to(&update);

			update.is_sorted_by(|a,b| {
				rules.to_owned().any(|Rule(x,y)| {
					x == a && y == b
				})
			})
		})
		.map(|update| {
			let middle = (update.len()-1) / 2;
			update[middle] as usize
		})
		.sum::<usize>()
		.to_string()

}

mod test {

	use super::*;
	use aoc_driver::Part::*;

	const INPUT_EXAMPLE:&str =
		r###"
		47|53
		97|13
		97|61
		97|47
		75|29
		61|13
		75|53
		29|13
		97|29
		53|29
		61|53
		97|53
		61|29
		47|13
		75|47
		97|75
		47|61
		75|61
		47|29
		75|13
		53|13

		75,47,61,53,29
		97,61,53,29,13
		75,29,13
		75,97,47,61,53
		61,13,29
		97,13,75,29,47
		"###;

	#[test]
	fn part_1_example() {

		let expected : &str = "143";
		let actual:String = solve_1(INPUT_EXAMPLE);

		assert_eq!(actual, expected);
	}

	#[test]
	fn test_submit()-> Result<(), AppError> {
		try_submit(Day(5), solve_1, Part1)?;
		// try_submit(Day(4), solve_2, Part2)?;
		Ok(())
	}

}
