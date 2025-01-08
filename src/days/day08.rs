// https://adventofcode.com/2024/day/8

use std::ops::Deref;

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
		let position_of = |o| self.position_of(o);
		self.iter().enumerate()
			.filter_map(move |(offset,byte)| {
				Antenna::try_from(byte)
					.map(|a| a.at_location(position_of(offset).unwrap()))
					.ok()
			})
	}

	pub fn frequencies<'a>(&'a self) -> impl Iterator<Item=u8> + 'a {
		self.antennae().map(|a| a.frequency).sorted().dedup()
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

	let max:V2 = city.last_position().into();
	let min:V2 = V2::zero();

	anti_nodes
		.sorted()
		.dedup()
		.filter(|v| match *v {

			V2{x,..} if x < min.x => false,
			V2{x,..} if x > max.x => false,

			V2{y,..} if y < min.y => false,
			V2{y,..} if y > max.y => false,

			_ => true
		})
		.count()
		.to_string()

}

fn solve_2(input: &str) -> String {

	let city = City(Map::from(Input(input).lines()));

	fn antinodes(a:V2,b:V2,city:&City) -> impl IntoIterator<Item=Position> {

		let max:V2 = city.last_position().into();
		let min:V2 = V2::zero();
		let inc = b-a;
		let mut antinode = b;
		// antinodes generated from antennae a -> b
		// that fall inside the city limits
		std::iter::from_fn(move || {
			antinode = antinode+inc;
			match antinode {
				V2{x,..} if x < min.x => None,
				V2{x,..} if x > max.x => None,

				V2{y,..} if y < min.x => None,
				V2{y,..} if y > max.y => None,

				_ => Some(antinode)
			}
		})
		// Plus the two antennae
		.chain([a,b].into_iter()).map(Position::from)
	}

	// Gather all pairs of antennae with the same frequency

	let antenna_combos = city.frequencies().flat_map(|c| {
		city.antennae().filter(move |a| a.frequency == c).combinations(2)
	});

	let anti_nodes = antenna_combos.flat_map(|c| {
		c.into_iter().map(V2::from).permutations(2).flat_map(|vecs| antinodes(vecs[0], vecs[1], &city))
	});

	anti_nodes
		.sorted()
		.dedup()
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
	fn part_2_example() {

		let expected = "34";
		let actual = solve_2(INPUT_EXAMPLE);

		assert_eq!(actual,expected);
	}

	#[test]
	fn submit()-> Result<(), AppError> {
		try_submit(Day(8), solve_1, Part1)?;
		try_submit(Day(8), solve_2, Part2)?;
		Ok(())
	}
}
