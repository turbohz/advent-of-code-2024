// https://adventofcode.com/2024/day/8

use std::ops::{Add, Deref, Neg, Sub};

use super::*;

#[derive(Clone, Copy)]
struct Antenna {
	code:u8,
	location:Position
}

impl Antenna {
	pub fn at_location(self, location:Position) -> Self {
		Self { location, ..self}
	}
}

impl TryFrom<u8> for Antenna {
	type Error = ();

	fn try_from(code: u8) -> Result<Self, Self::Error> {
		if code.is_ascii_alphanumeric() {
			Ok(Antenna { code, location: Default::default() })
		} else {
			Err(())
		}
	}
}

impl From<Antenna> for Vec {
	fn from(antenna: Antenna) -> Self {
		Vec::from(antenna.location)
	}
}

struct City(Map<u8>);

impl Deref for City {
	type Target = Map;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl City {
	pub fn antennae<'a>(&'a self) -> impl Iterator<Item=Antenna> + 'a {
		let position_of = |o| self.field.position_of(o);
		self.data.iter().enumerate()
			.filter_map(move |(offset,&byte)| {
				Antenna::try_from(byte)
					.map(|a| a.at_location(position_of(offset)))
					.ok()
			})
	}

	pub fn codes<'a>(&'a self) -> impl Iterator<Item=u8> + 'a {
		self.antennae().map(|a| a.code).sorted().dedup()
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Vec(isize,isize);

impl Neg for Vec {
	type Output = Vec;

	fn neg(self) -> Self::Output {
		Vec(-self.0,-self.1)
	}
}

impl Add for Vec {
	type Output = Vec;

	fn add(self, rhs: Self) -> Self::Output {
		let Vec(x1,y1) = self;
		let Vec(x2,y2) = rhs;
		Vec(x1+x2,y1+y2)
	}
}

impl Sub for Vec {
	type Output = Vec;

	fn sub(self, rhs: Self) -> Self::Output {
		self + (-rhs)
	}
}

impl From<Position> for Vec {
	fn from(pos: Position) -> Self {
		let Position(x,y) = pos;
		Vec(x as isize, y as isize)
	}
}

fn solve_1(input: &str) -> String {

	let city = City(Map::from(Input(input).lines()));

	// Gather all pairs of antennae with the same frequency

	let antenna_combos = city.codes().flat_map(|c| {
		city.antennae().filter(move |a| a.code == c).combinations(2)
	});

	// Find both potential anti-nodes for each pair

	let anti_nodes = antenna_combos.flat_map(|c| {
		c.into_iter().map(Vec::from).permutations(2).map(|vecs| vecs[1] - vecs[0] + vecs[1])
	});

	// Reduce the list of potential anti-nodes
	// Must be inside the bounds, and they must
	// be counted once (no dupes)

	let Position(max_x,max_y) = city.field.last_position();

	anti_nodes
		.sorted()
		.dedup()
		.filter(|v| match *v {

			Vec(x,_) if x < 0 => false,
			Vec(x,_) if x > max_x as isize => false,

			Vec(_,y) if y < 0 => false,
			Vec(_,y) if y > max_y as isize => false,

			_ => true
		})
		.count()
		.to_string()

}

mod test {

	use super::*;
	use aoc_driver::Part::*;

	const INPUT_EXAMPLE:&str =
		r###"
		............
		........0...
		.....0......
		.......0....
		....0.......
		......A.....
		............
		............
		........A...
		.........A..
		............
		............
		"###;

	#[test]
	fn part_1_example() {

		let expected = "14";
		let actual = solve_1(INPUT_EXAMPLE);

		assert_eq!(actual,expected);
	}

	#[test]
	fn submit()-> Result<(), AppError> {
		try_submit(Day(8), solve_1, Part1)?;
		// try_submit(Day(6), solve_2, Part2)?;
		Ok(())
	}
}
