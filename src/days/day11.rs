// https://adventofcode.com/2024/day/11

use std::{num::ParseIntError, ops::Deref};

use super::*;

trait Evolve: Copy + From<usize> + ToString {
	fn evolve(&self) -> Evolution<Self>;
}

// The length is provided when it's created
#[derive(Debug,Clone,Copy,PartialEq)]
struct SizedStone(usize,u8);

impl TryFrom<&str> for SizedStone {
	type Error = ();

	fn try_from(input: &str) -> Result<Self, Self::Error> {
		let len = u8::try_from(input.len()).map_err(|_|())?;
		let val = str::parse(input).map_err(|_|())?;
		Ok(SizedStone(val,len))
	}
}

impl From<usize> for SizedStone {
	fn from(value: usize) -> Self {
		let len = 1 + if value >= 10 { value.ilog10() as u8 } else { 0 };
		Self(value,len)
	}
}

impl From<(usize,u8)> for SizedStone {
	fn from((val,len): (usize,u8)) -> Self {
		Self(val,len)
	}
}

impl SizedStone {
	#[inline]
	pub fn try_split(&self) -> Option<((usize,u8),usize)> {

		let SizedStone(val,len) = self;

		if len % 2 == 0 {
			let half = len/2;
			let n = 10usize.pow(half as u32);
			let upper = val/n;
			let lower = val%n;
			Some((
				(upper, half),
				// leading zeroes are eliminated!
				// or it can be just 0
				lower
			))
		} else {
			None
		}
	}
}

impl Evolve for SizedStone {
	fn evolve(&self) -> Evolution<Self> {
		use Evolution::*;
		let v = self.0;
		if v == 0 {
			Single(Self(1,1))
		} else if let Some((fst,snd)) = self.try_split() {
			Couple(fst.into(),snd.into())
		} else {
			Single(Self::from(v*2024))
		}
	}
}

impl Display for SizedStone {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"{}",self.0)
	}
}

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

enum Evolution<T:Copy> {
	Empty(),
	Single(T),
	Couple(T,T)
}

impl<T:Copy> Iterator for Evolution<T> {
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
	#[inline]
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
}

impl Display for Stone {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"{}",self.0)
	}
}

impl Evolve for Stone {
	fn evolve(&self) -> Evolution<Stone> {
		use Evolution::*;
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

struct Blinker<T> {
	seed: Vec<T>
}

impl<T:Evolve> Blinker<T> {

	fn count<const TIMES:u8>(self) -> usize {

		fn walk<T:Evolve,const MAX:u8>(t:impl Iterator<Item=T>,i:u8) -> usize {
			if i < MAX {
				t.map(|s| walk::<T,MAX>(s.evolve(),i+1)).sum()
			} else {
				t.count()
			}
		}

		self.seed
			.into_iter()
			.map(|stone| walk::<T,TIMES>(Evolution::Single(stone),0))
			.sum::<usize>()
	}

	fn collect<const TIMES:u8>(self) -> String {

		fn walk<T:Evolve,const MAX:u8>(t:impl Iterator<Item=T>,i:u8) -> Vec<T> {
			if i < MAX {
				t.map(|s| walk::<T,MAX>(s.evolve(),i+1)).flatten().collect_vec()
			} else {
				t.collect_vec()
			}
		}

		self.seed
			.into_iter()
			.flat_map(|stone| walk::<T,TIMES>(Evolution::Single(stone),0))
			.map(|s| s.to_string())
			.join(" ")
	}
}

impl<T:Evolve> From<&str> for Blinker<T> {
	fn from(value: &str) -> Self {
		let seed = value.split(' ').map(|n| str::parse::<usize>(n).unwrap().into()).collect_vec();
		Self { seed }
	}
}

fn solve_1(input: &str) -> String {
	let line = Input(input).lines().next().unwrap();
	let blinker:Blinker<Stone> = line.into();
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
		let mut empty = Evolution::<Stone>::Empty();
		assert_eq!(empty.next(),None);

		let mut single = Evolution::Single(Stone(0));
		assert_eq!(single.next(),Some(Stone(0)));
		assert_eq!(single.next(),None);

		let mut couple = Evolution::Couple(Stone(0),Stone(1));
		assert_eq!(couple.next(),Some(Stone(0)));
		assert_eq!(couple.next(),Some(Stone(1)));
		assert_eq!(couple.next(),None);
	}

	#[test]
	fn part_1_example() {

		let line = Input(INPUT_EXAMPLE).lines().next().unwrap();

		let blinker:Blinker<Stone> = line.into();
		let expected = "253000 1 7";
		let actual = blinker.collect::<1>();
		assert_eq!(actual, expected);

		let blinker:Blinker<Stone> = line.into();
		let expected = "253 0 2024 14168";
		let actual = blinker.collect::<2>();
		assert_eq!(actual, expected);

		let blinker:Blinker<Stone> = line.into();
		let expected = "512072 1 20 24 28676032";
		let actual = blinker.collect::<3>();
		assert_eq!(actual, expected);

		let blinker:Blinker<Stone> = line.into();
		let expected = "512 72 2024 2 0 2 4 2867 6032";
		let actual = blinker.collect::<4>();
		assert_eq!(actual, expected);

		let blinker:Blinker<Stone> = line.into();
		let expected = "1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32";
		let actual = blinker.collect::<5>();
		assert_eq!(actual, expected);

		let blinker:Blinker<Stone> = line.into();
		let expected = "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2";
		let actual = blinker.collect::<6>();
		assert_eq!(actual, expected);

		// Counts

		let blinker:Blinker<Stone> = line.into();
		let expected = 22;
		let actual = blinker.count::<6>();
		assert_eq!(actual, expected);

		let blinker:Blinker<Stone> = line.into();
		let expected = 55312;
		let actual = blinker.count::<25>();
		assert_eq!(actual, expected);

		let blinker:Blinker<SizedStone> = line.into();
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
