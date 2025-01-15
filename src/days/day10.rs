// https://adventofcode.com/2024/day/10

use std::{fmt::Debug, ops::Deref};

use super::*;

/// Paths can turn in 90 degrees only
/// in horizontal and vertical directions

/// A trail is a path from level 0 to 9 increasing by 1
///
/// A trailhead is any location with height 0
/// Its score is the number of trails (with slope 1)
/// that reach the top (a 9 of height)

/// Strategy:
/// - Find trailheads level locations
/// - scan around for locations with level + 1
/// - Follow along until you're on a level 9 or
///   there's nowhere else to go

#[derive(Debug,Default,Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
struct Level(u8);

impl Level {
	const MIN:Level = Level(0);
	const MAX:Level = Level(9);
}

impl Deref for Level {
	type Target = u8;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl Display for Level {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"{}",self.0)
	}
}

impl Level {

	fn next(&self) -> Option<Self> {
		if *self < Self::MAX {
			Some(Level(**self+1))
		} else {
			None
		}
	}
}

impl From<u8> for Level {
	fn from(value:u8) -> Self {
		// value is expected to be an ASCII digit
		assert!((b'0'..=b'9').contains(&value));
		let num = value-b'0';
		Level(num)
	}
}

struct TopographicMap(Map<Level>);
impl Deref for TopographicMap {
	type Target = Map<Level>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl TopographicMap {
	pub fn at(&self,coord: impl Into<V2>) -> Option<Spot> {
		let coord:V2 = coord.into();
		if self.contains(coord) {
			let location:Position = coord.try_into().ok()?;
			let level:Level = self[location];
			Some(Spot { location, level })
		} else {
			None
		}
	}

	/// Returns locations of trailheads
	fn trailheads(&self) -> impl Iterator<Item=Spot> {
		self.iter()
			.enumerate()
			.filter_map(|(i,level)| {
				if level == Level::MIN {
					let location = self.position_of(i).unwrap();
					let spot = Spot { location,level };
					Some(spot)
				} else {
					None
				}
			})
	}

	fn paths(&self,spot:Spot) -> impl Iterator<Item=Spot> {
		const DIRECTIONS:[Direction;4] = const {
			use Direction::*;
			[North,East,South,West]
		};

		let next_level = spot.level.next();
		let coord:V2 = spot.location.into();

		DIRECTIONS.into_iter()
			// empty() if no next level ↴
			.zip(std::iter::repeat_n(next_level,DIRECTIONS.len()).flatten())
			.filter_map(move |(dir,lvl)| {
				let delta:V2 = dir.into();
				let next = coord + delta;
				// A spot there, which is next level
				self.at(next).filter(|spot| spot.level == lvl)
			})
	}
}

impl Display for TopographicMap {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let csize:usize = self.stride().into();
		let msg = self.iter().chunks(csize).into_iter()
			.map(|cnk| {
				cnk.into_iter().flat_map(|level| char::from_digit(*level as u32,10)).join("")
			})
			.collect_vec()
		;
		write!(f, "{:#?}", msg)
	}
}

impl<'a, L:Iterator<Item=&'a str>+Clone> From<L> for TopographicMap {
	fn from(lines: L) -> Self {
		let map:Map<Level> = lines.into();
		Self(map)
	}
}

#[derive(Clone, Copy)]
struct Spot {
	location: Position,
	level: Level,
}

impl Display for Spot {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"[{}]({},{})",self.level,self.location.x,self.location.y)
	}
}

impl Debug for Spot {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"{}",self)
	}
}

#[cfg(test)]
mod test {

	use super::*;
	use aoc_driver::Part::*;

	const INPUT_EXAMPLE:&str =
		r###"
		89010123
		78121874
		87430965
		96549874
		45678903
		32019012
		01329801
		10456732
		"###;

	#[test]
	fn topographic_map() {

		let map = TopographicMap::from(Input(INPUT_EXAMPLE).lines());

		// top-left corner
		let spot = map.at(V2{x:0,y:0}).unwrap();
		let actual = map.paths(spot).map(|s| s.location).at_most_one().ok().flatten();
		let expected = Some(Position{x:1,y:0});
		assert_eq!(actual,expected);

		// a nine
		let spot = map.at(V2{x:1,y:0}).unwrap();
		let mut actual = map.paths(spot).map(|s| s.location);
		assert!(actual.next().is_none());

		// no suitable neighbors
		let spot = map.at(V2{x:0,y:7}).unwrap();
		let mut actual = map.paths(spot).map(|s| s.location);
		assert!(actual.next().is_none());

		// bottom right corner
		let spot = map.at(V2{x:7,y:7}).unwrap();
		let actual = map.paths(spot).map(|s| s.location).at_most_one().ok().flatten();
		let expected = Some(Position{x:6,y:7});
		assert_eq!(actual,expected);

		// some trailhead
		let spot = map.at(V2{x:0,y:6}).unwrap();
		let actual = map.paths(spot).map(|s| s.location).collect_tuple();
		let expected = Some((Position{x:1,y:6},Position{x:0,y:7}));
		assert_eq!(actual,expected);

		let trailheads = map.trailheads().collect_vec();

		assert_eq!(trailheads.len(),9);
		assert!(trailheads.iter().all(|s| s.level == Level::MIN));

		let locations = trailheads.into_iter().map(|s| s.location).collect_vec();

		assert!(locations.contains(&Position{x:2,y:0}));
		assert!(locations.contains(&Position{x:4,y:0}));

		assert!(locations.contains(&Position{x:4,y:2}));

		assert!(locations.contains(&Position{x:6,y:4}));

		assert!(locations.contains(&Position{x:2,y:5}));
		assert!(locations.contains(&Position{x:5,y:5}));

		assert!(locations.contains(&Position{x:0,y:6}));
		assert!(locations.contains(&Position{x:6,y:6}));

		assert!(locations.contains(&Position{x:1,y:7}));
	}
}
