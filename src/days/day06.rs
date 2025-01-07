// https://adventofcode.com/2024/day/6

use std::iter::once;
use super::*;

#[derive(Debug,Clone,Copy,PartialEq,Eq,Ord,PartialOrd)]
pub enum Direction {
	North,
	East,
	South,
	West,
}

mod guard {

	use super::*;

	#[derive(Debug,Clone,Copy,PartialEq,Eq,Ord,PartialOrd)]
	pub struct State {
		pub location: Position,
		pub orientation: Direction,
	}

	pub struct Route<'a> {
		pub room: &'a Map,
		pub state: State,
		pub protocol: Box<dyn Iterator<Item=Direction>>
	}

	impl<'a> Route<'a> {
		pub fn new(room:&'a Map,start:Position) -> Self {
			use Direction::*;
			let mut protocol = Box::new([North,East,South,West].into_iter().cycle());
			let orientation = protocol.next().unwrap();
			let state = State { location: start, orientation };
			Route { room, protocol, state }
		}
	}

	impl<'a> Iterator for Route<'a> {
		type Item = State;

		fn next(&mut self) -> Option<Self::Item> {

			use Direction::*;

			let State { location: Position{x,y}, orientation } = self.state;

			// check exiting room condition

			let guard_exits = {

				let min = Position::zero();
				let max = self.room.last_position();

				orientation == North && y == min.y ||
				orientation == East  && x == max.x ||
				orientation == South && y == max.y ||
				orientation == West  && x == min.x
			};

			if guard_exits { None } else {

				// tries to take a step ahead

				let position_ahead = match orientation {
					North => Position{x,y:y-1},
					East  => Position{x:x+1,y},
					South => Position{x,y:y+1},
					West  => Position{x:x-1,y},
				};

				let obstructed = self.room[position_ahead] == b'#';

				if !obstructed {
					self.state.location = position_ahead;
				} else {
					// try with next orientation according to protocol
					//
					// NOTICE: this means that we will return repeated
					// consecutive positions every turn
					self.state.orientation = self.protocol.next().unwrap();
				}

				Some(self.state.to_owned())
			}
		}
	}
}

struct Simulation {
	room: Map,
	start: Position
}

impl Simulation {

	pub fn ins_obstacle(&mut self,p:Position) {
		self.room[p] = b'#';
	}

	pub fn del_obstacle(&mut self,p:Position) {
		self.room[p] = b'.';
	}

	pub fn iter(&self) -> impl Iterator<Item=guard::State> {
		self.into_iter()
			// Removes states that do not produce advance (rotating in place)
			// NOTICE: Luckily for us, `dedup_by` seems to keep
			// the last value, not the first.
			.dedup_by(|a,b| a.location == b.location)
	}
}

impl<'a, L:Iterator<Item=&'a str>+Clone> From<L> for Simulation {

	fn from(lines:L)-> Self {
		let room = Map::from(lines);
		let start = {
			let offset = room.iter().position(|item| item == b'^')
				.expect("There room data should have a '^' character somewhere");

			room.position_of(offset).unwrap()
		};

		Self { room, start }
	}
}

impl<'a> IntoIterator for &'a Simulation {
	type Item = guard::State;
	type IntoIter = Box<dyn Iterator<Item=guard::State> + 'a>;

	fn into_iter(self) -> Self::IntoIter {
		let route = guard::Route::new(&self.room,self.start);
		Box::new(once(route.state).chain(route))
	}
}

fn solve_1(input: &str) -> String {

	let simulation = Simulation::from(Input(input).lines());

	let mut offsets:Vec<usize> = simulation.iter()
		.map(|guard::State{location,..}| simulation.room.offset_of(location).unwrap())
		.collect();

	offsets.sort();
	offsets.iter()
		// remove repeated positions due to paths crossing
		.dedup()
		.count()
		.to_string()
}

fn solve_2(input: &str) -> String {

	use std::collections::BTreeSet;

	let mut simulation = Simulation::from(Input(input).lines());

	let mut guard_path = simulation.iter().map(|s| s.location).collect_vec();

	// Try only unique locations along the guard trail
	guard_path.sort();
	guard_path.into_iter().dedup()
		.filter(|&location| {

			// An obstruction cannot be placed at the start position
			if location == simulation.start { return false }

			let mut it_loops:bool = false;
			let mut visited = BTreeSet::new();

			simulation.ins_obstacle(location);

			for step in simulation.iter() {

				// A loop is detected when the guard re-visit
				// a location facing at the same direction

				if !visited.insert(step) {
					it_loops = true;
					break
				}
			}

			simulation.del_obstacle(location);

			return it_loops
		})
		.count()
		.to_string()
}

#[cfg(test)]
mod test {

	use super::*;
	use aoc_driver::Part::*;

	const INPUT_EXAMPLE:&str =
		r###"
		....#.....
		.........#
		..........
		..#.......
		.......#..
		..........
		.#..^.....
		........#.
		#.........
		......#...
		"###;

	#[test]
	fn part_1_example() {

		let expected = "41";
		let actual = solve_1(INPUT_EXAMPLE);

		assert_eq!(actual,expected);
	}

	#[test]
	fn part_2_example() {

		let expected = "6";
		let actual = solve_2(INPUT_EXAMPLE);

		assert_eq!(actual,expected);
	}

	#[test]
	fn submit()-> Result<(), AppError> {
		try_submit(Day(6), solve_1, Part1)?;
		try_submit(Day(6), solve_2, Part2)?;
		Ok(())
	}
}
