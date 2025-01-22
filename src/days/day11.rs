// https://adventofcode.com/2024/day/11

use std::{num::ParseIntError, ops::Deref};

use super::*;

#[derive(Debug,Clone,Copy,PartialEq)]
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

enum Iter<T:Copy> {
	Empty(),
	Single(T),
	Couple(T,T)
}

impl<T:Copy> Iterator for Iter<T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		match self {
			Self::Empty() => None
			,
			Self::Single(single) => {
				let v = *single;
				*self = Self::Empty();
				Some(v)
			}
			,
			Self::Couple(fst,snd) => {
				let v = *fst;
				*self = Self::Single(*snd);
				Some(v)
			}
		}
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
			let digits = 1 + v.ilog10();
			if digits % 2 == 0 {
				let n = 10usize.pow(digits/2);
				let upper = v/n;
				let lower = v%n;
				Some((upper,lower))
			} else {
				None
			}
		}
	}

	pub fn evolve(&self) -> impl Iterator<Item=Stone> {
		use Iter::*;
		let v = self.0;
		if v == 0 {
			Single(Stone(1))
		} else if let Some((a,b)) = self.try_split() {
			Couple(Stone(a),Stone(b))
		} else {
			Single(Stone(v*2024))
		}
	}
}

struct Blinker {
	seed: Vec<Stone>
}

impl Blinker {

	fn count<const TIMES:u8>(self) -> usize {

		fn walk<const MAX:u8>(t:impl Iterator<Item=Stone>,i:u8) -> usize {
			if i < MAX - 1 {
				t.map(|s| walk::<MAX>(s.evolve(),i+1)).sum()
			} else {
				t.count()
			}
		}

		self.seed
			.into_iter()
			.map(|stone| walk::<TIMES>(stone.evolve(),0))
			.sum::<usize>()
	}

	fn collect<const TIMES:u8>(self) -> String {

		fn walk<const MAX:u8>(t:impl Iterator<Item=Stone>,i:u8) -> Vec<Stone> {
			if i < MAX - 1 {
				t.map(|s| walk::<MAX>(s.evolve(),i+1)).flatten().collect_vec()
			} else {
				t.collect_vec()
			}
		}

		self.seed
			.into_iter()
			.flat_map(|stone| walk::<TIMES>(stone.evolve(),0))
			.map(String::from)
			.join(" ")
	}
}

impl From<&str> for Blinker {
	fn from(value: &str) -> Self {
		let seed = value.split(' ').map(|n| Stone::try_from(n).unwrap()).collect_vec();
		Self { seed }
	}
}

fn solve_1(input: &str) -> String {
	let line = Input(input).lines().next().unwrap();
	let blinker:Blinker = line.into();
	blinker.count::<25>().to_string()
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
	fn iter() {
		let mut empty = Iter::<Stone>::Empty();
		assert_eq!(empty.next(),None);

		let mut single = Iter::Single(Stone(0));
		assert_eq!(single.next(),Some(Stone(0)));
		assert_eq!(single.next(),None);

		let mut couple = Iter::Couple(Stone(0),Stone(1));
		assert_eq!(couple.next(),Some(Stone(0)));
		assert_eq!(couple.next(),Some(Stone(1)));
		assert_eq!(couple.next(),None);
	}

	#[test]
	fn part_1_example() {

		let line = Input(INPUT_EXAMPLE).lines().next().unwrap();

		let blinker:Blinker = line.into();
		let expected = "253000 1 7";
		let actual = blinker.collect::<1>();
		assert_eq!(actual, expected);

		let blinker:Blinker = line.into();
		let expected = "253 0 2024 14168";
		let actual = blinker.collect::<2>();
		assert_eq!(actual, expected);

		let blinker:Blinker = line.into();
		let expected = "512072 1 20 24 28676032";
		let actual = blinker.collect::<3>();
		assert_eq!(actual, expected);

		let blinker:Blinker = line.into();
		let expected = "512 72 2024 2 0 2 4 2867 6032";
		let actual = blinker.collect::<4>();
		assert_eq!(actual, expected);

		let blinker:Blinker = line.into();
		let expected = "1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32";
		let actual = blinker.collect::<5>();
		assert_eq!(actual, expected);

		let blinker:Blinker = line.into();
		let expected = "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2";
		let actual = blinker.collect::<6>();
		assert_eq!(actual, expected);

		// Counts

		let blinker:Blinker = line.into();
		let expected = 22;
		let actual = blinker.count::<6>();
		assert_eq!(actual, expected);

		let blinker:Blinker = line.into();
		let expected = 55312;
		let actual = blinker.count::<25>();
		assert_eq!(actual, expected);
	}

	#[test]
	fn submit()-> Result<(), AppError> {
		try_submit(Day(11), solve_1, Part1)?;
		// try_submit(Day(11), solve_2, Part2)?;
		Ok(())
	}
}
