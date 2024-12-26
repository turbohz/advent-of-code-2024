// https://adventofcode.com/2024/day/6

use std::{iter::{repeat,once}, ops::Index};
use super::*;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Direction {
	North,
	East,
	South,
	West,
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
struct Position(usize,usize);

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
struct GuardState {
	location: Position,
	orientation: Direction,
}

impl Default for GuardState {
	fn default() -> Self {
		Self { location: Position(0,0), orientation: Direction::North }
	}
}

#[derive(Debug, Clone, Copy)]
struct Field {
	pub width: usize,
	pub height: usize,
}

impl From<(usize,usize)> for Field {
	fn from(size: (usize,usize)) -> Self {
		Self { width: size.0, height: size.1 }
	}
}

impl Field {

	pub fn size(&self)->(usize,usize) {
		(self.width,self.height)
	}

	#[inline]
	pub fn stride(&self) -> usize {
		self.width
	}

	pub fn offset_of(&self, Position(x,y):Position) -> usize {
		y*self.stride()+x
	}

	pub fn position_of(&self, offset:usize) -> Position {
		let stride = self.stride();
		Position(offset % stride, offset / stride)
	}

	pub fn last_position(&self) -> Position {
		Position(self.width-1,self.height-1)
	}
}

struct Room {
	field: Field,
	data: Vec<u8>
}

impl Room {

	fn at(&self, position:Position) -> Option<u8> {
		let (w,h) = self.field.size();
		let Position(x,y) = position;
		if x<w && y<h {
			Some(self[position])
		} else {
			None
		}
	}

	pub fn size(&self)->(usize,usize) {
		self.field.size()
	}

	pub fn start_position(&self)-> Position {

		let offset = self.data.iter().position(|&item| item == b'^')
			.expect("There room data should have a '^' character somewhere");

		self.field.position_of(offset)
	}
}

impl Index<Position> for Room {
	type Output = u8;
	fn index(&self, p: Position) -> &Self::Output {
		&self.data[self.field.offset_of(p)]
	}
}

struct Simulation<'a> {
	room: Room,
	guard_state: GuardState,
	guard_protocol: Box<dyn 'a+Iterator<Item=Direction>>,
}

impl<'a, L:Iterator<Item=&'a str>+Clone> From<L> for Simulation<'a> {

	fn from(lines:L)-> Self {

		let rows = lines.map(str::bytes);
		// clone iterator, measure line width
		let width = rows.clone().into_iter().take(1).flatten().count();

		let data:Vec<u8> = rows.flatten().collect();
		let height = data.len() / width;
		let room = Room { field: (width,height).into(), data };

		let initial_guard_state = {
			GuardState { location: room.start_position(), ..Default::default() }
		};

		Self {
			room,
			guard_state: initial_guard_state,
			guard_protocol: Box::new(repeat(Direction::North)),
		}
	}
}

impl<'a> Simulation<'a> {

	pub fn set_protocol<P:'a+Iterator<Item=Direction>>(&mut self, protocol:P) {
		self.guard_protocol = Box::new(protocol)
	}

	pub fn start_position(&self)->Position {
		return self.room.start_position();
	}

	pub fn count_locations(&mut self)->usize {

		let start = self.start_position();
		let field = self.room.field.clone();

		// NOTICE:
		// the initial position must be added manually
		let mut offsets:Vec<usize> = once(start).chain(self).map(|p| {
			field.offset_of(p)
		}).collect();

		offsets.sort();
		offsets.iter().dedup().count()
	}
}

impl<'a> Iterator for Simulation<'a> {
	type Item = Position;

	fn next(&mut self) -> Option<Self::Item> {

		use Direction::*;

		let GuardState { location: Position(x,y), orientation } = self.guard_state;

		// check exiting room condition

		let guard_exits = {

			let Position(min_x,min_y) = Position(0,0);
			let Position(max_x,max_y) = self.room.field.last_position();

			orientation == North && y == min_y ||
			orientation == East  && x == max_x ||
			orientation == South && y == max_y ||
			orientation == West  && x == min_x
		};

		if guard_exits { None } else {

			let position_ahead = match orientation {
				North => Position(x,y-1),
				East  => Position(x+1,y),
				South => Position(x,y+1),
				West  => Position(x-1,y),
			};

			let obstructed = self.room.at(position_ahead) == Some(b'#');

			if !obstructed {
				self.guard_state.location = position_ahead;
			} else {
				// try with next orientation
				self.guard_state.orientation = self.guard_protocol.next().unwrap();
			}

			Some(self.guard_state.location)
		}
	}
}

fn solve_1(input: &str) -> String {

	use Direction::*;

	let protocol = [North,East,South,West].into_iter().cycle();
	let mut simulation = Simulation::from(Input(input).lines());
	simulation.set_protocol(protocol);
	simulation.count_locations().to_string()
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
	fn simulation_initialization() {

		let simulation = Simulation::from(Input(INPUT_EXAMPLE).lines());

		assert_eq!(simulation.room.size(), (10,10));

		let expected = GuardState { location: Position(4,6), orientation: Direction::North };
		let actual = simulation.guard_state;

		assert_eq!(actual,expected);
	}

	#[test]
	fn part_1_example() {

		use Direction::*;

		let protocol = [North,East,South,West].into_iter().cycle();

		let mut simulation = Simulation::from(Input(INPUT_EXAMPLE).lines());
		simulation.set_protocol(protocol);

		let expected = 41;
		let actual = simulation.count_locations();

		assert_eq!(actual,expected);
	}

	#[test]
	fn submit()-> Result<(), AppError> {
		try_submit(Day(6), solve_1, Part1)?;
		// try_submit(Day(6), solve_2, Part2)?;
		Ok(())
	}
}
