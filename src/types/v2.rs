use std::ops::{Add, Neg, Sub};
use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct V2 { pub x:i32, pub y:i32 }


impl V2 {
	pub fn zero() -> Self {
		Self{x:0,y:0}
	}
}

impl Neg for V2 {
	type Output = V2;

	fn neg(self) -> Self::Output {
		V2{x:-self.x,y:-self.y}
	}
}

impl Add for V2 {
	type Output = V2;

	fn add(self, rhs: Self) -> Self::Output {
		let V2{x:x1,y:y1} = self;
		let V2{x:x2,y:y2} = rhs;
		V2{x:x1+x2,y:y1+y2}
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
		let Position{x,y} = pos;
		V2{x:x as i32, y:y as i32}
	}
}
