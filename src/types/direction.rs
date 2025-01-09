use super::*;

#[derive(Debug,Clone,Copy,PartialEq,Eq,Ord,PartialOrd)]
pub enum Direction {
	North,
	East,
	South,
	West,
}

impl Into<V2> for Direction {
	fn into(self) -> V2 {
		use Direction::*;
		match self {
			North => V2{x: 0,y:-1},
			East  => V2{x: 1,y: 0},
			South => V2{x: 0,y: 1},
			West  => V2{x:-1,y: 0},
		}
	}
}
