// https://adventofcode.com/2024/day/8

use std::ops::{Add, Deref, Neg, Sub};

use super::*;

#[derive(Clone, Copy)]
struct Antenna {
	frequency:u8,
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
			Ok(Antenna { frequency: code, location: Default::default() })
		} else {
			Err(())
		}
	}
}

impl From<Antenna> for V2 {
	fn from(antenna: Antenna) -> Self {
		V2::from(antenna.location)
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

	pub fn frequencies<'a>(&'a self) -> impl Iterator<Item=u8> + 'a {
		self.antennae().map(|a| a.frequency).sorted().dedup()
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct V2(isize,isize);

impl Neg for V2 {
	type Output = V2;

	fn neg(self) -> Self::Output {
		V2(-self.0,-self.1)
	}
}

impl Add for V2 {
	type Output = V2;

	fn add(self, rhs: Self) -> Self::Output {
		let V2(x1,y1) = self;
		let V2(x2,y2) = rhs;
		V2(x1+x2,y1+y2)
	}
}

impl Sub for V2 {
	type Output = V2;

	fn sub(self, rhs: Self) -> Self::Output {
		self + (-rhs)
	}
}

impl From<Position> for V2 {
	fn from(pos: Position) -> Self {
		let Position(x,y) = pos;
		V2(x as isize, y as isize)
	}
}

fn solve_1(input: &str) -> String {

	let city = City(Map::from(Input(input).lines()));

	// Gather all pairs of antennae with the same frequency

	let antenna_combos = city.frequencies().flat_map(|c| {
		city.antennae().filter(move |a| a.frequency == c).combinations(2)
	});

	// Find both potential anti-nodes for each pair

	let anti_nodes = antenna_combos.flat_map(|c| {
		c.into_iter().map(V2::from).permutations(2).map(|vecs| vecs[1] - vecs[0] + vecs[1])
	});

	// Reduce the list of potential anti-nodes
	// Must be inside the bounds, and they must
	// be counted once (no dupes)

	let Position(max_x,max_y) = city.field.last_position();

	anti_nodes
		.sorted()
		.dedup()
		.filter(|v| match *v {

			V2(x,_) if x < 0 => false,
			V2(x,_) if x > max_x as isize => false,

			V2(_,y) if y < 0 => false,
			V2(_,y) if y > max_y as isize => false,

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
