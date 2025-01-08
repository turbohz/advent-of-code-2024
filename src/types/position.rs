use std::fmt::Display;
use super::*;

#[derive(Debug,Clone,Copy,PartialEq,Eq,Default,Ord,PartialOrd)]
pub struct Position { pub x:u16, pub y:u16 }

impl Display for Position {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"{},{}",self.x,self.y)
	}
}

impl Position {
	pub fn zero() -> Self {
		Self{x:0,y:0}
	}
}

impl Into<(u16,u16)> for Position {
	fn into(self) -> (u16,u16) {
		(self.x,self.y)
	}
}

impl From<V2> for Position {
	fn from(v2: V2) -> Self {
		let V2{x,y} = v2;
		Position{x: x as u16, y: y as u16}
	}
}
