// https://adventofcode.com/2024/day/11

use std::{num::ParseIntError, ops::Deref};

use super::*;

#[derive(Debug,Clone,Copy)]
struct Stone(usize);
impl From<usize> for Stone {
	fn from(value: usize) -> Self {
		Self(value)
	}
}

impl Deref for Stone {
	type Target = usize;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl TryFrom<&str> for Stone {
	type Error = ParseIntError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		value.parse::<usize>().map(Self)
	}
}

impl From<Stone> for String {
	fn from(value: Stone) -> Self {
		(*value).to_string()
	}
}

impl Stone {
	pub fn try_split(&self) -> Option<(usize,usize)> {
		let v = self.0;
		if v < 10 {
			None
		} else {
			// 1 + n (where 10^n = v)
			let l10 = 1 + v.ilog10();
			if l10 % 2 == 0 {
				let n = 10usize.pow(l10/2);
				let upper = v/n;
				let lower = v%n;
				Some((upper,lower))
			} else {
				None
			}
		}
	}

	pub fn evolve(&self) -> Vec<Stone> {
		let v = self.0;
		if v == 0 {
			vec![Stone(1)]
		} else if let Some((a,b)) = self.try_split() {
			vec![Stone(a),Stone(b)]
		} else {
			vec![Stone(v*2024)]
		}
	}
}

struct Blinker(pub Vec<Stone>);
impl Blinker {
	pub fn stones(&self) -> &Vec<Stone> {
		&self.0
	}
}
impl Iterator for Blinker {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		let stones = self.0.iter().flat_map(Stone::evolve).collect_vec();
		let str = stones.iter().copied().map(String::from).join(" ");
		self.0 = stones;
		Some(str)
	}
}

impl From<&str> for Blinker {
	fn from(value: &str) -> Self {
		let stones = value.split(' ').map(|n| Stone::try_from(n).unwrap()).collect_vec();
		Self(stones)
	}
}

fn solve_1(input: &str) -> String {
	let line = Input(input).lines().next().unwrap();
	let blinker = Blinker::from(line);
	blinker.dropping(25).stones().len().to_string()
}

#[cfg(test)]
mod test {

	use super::*;
	use aoc_driver::Part::*;

	const INPUT_EXAMPLE:&str =
		r###"
		125 17
		"###;

	#[test]
	fn stone() {
		assert!(matches!(Stone(  10).try_split() , Some(( 1,0) )));
		assert!(matches!(Stone(  11).try_split() , Some(( 1,1) )));
		assert!(matches!(Stone(  19).try_split() , Some(( 1,9) )));
		assert!(matches!(Stone(1000).try_split() , Some((10,0) )));
		assert!(matches!(Stone(   0).try_split() , None));
		assert!(matches!(Stone(   1).try_split() , None));
		assert!(matches!(Stone(   9).try_split() , None));
		assert!(matches!(Stone( 111).try_split() , None));
	}

	#[test]
	fn part_1_example() {

		let line = Input(INPUT_EXAMPLE).lines().next().unwrap();
		let mut blinker = Blinker::from(line);

		let actual = blinker.next().unwrap_or_default();
		let expected = "253000 1 7";
		assert_eq!(actual, expected);

		let actual = blinker.next().unwrap_or_default();
		let expected = "253 0 2024 14168";
		assert_eq!(actual, expected);

		let actual = blinker.next().unwrap_or_default();
		let expected = "512072 1 20 24 28676032";
		assert_eq!(actual, expected);

		let actual = blinker.next().unwrap_or_default();
		let expected = "512 72 2024 2 0 2 4 2867 6032";
		assert_eq!(actual, expected);

		let actual = blinker.next().unwrap_or_default();
		let expected = "1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32";
		assert_eq!(actual, expected);

		let actual = blinker.next().unwrap_or_default();
		let expected = "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2";
		assert_eq!(actual,expected);

		let actual = blinker.dropping(25-6).stones().len();
		let expected = 55312;
		assert_eq!(actual, expected);
	}

	#[test]
	fn submit()-> Result<(), AppError> {
		try_submit(Day(11), solve_1, Part1)?;
		// try_submit(Day(11), solve_2, Part2)?;
		Ok(())
	}
}
