use std::{fmt::Display, ops::Index};

#[derive(Debug,Clone,Copy,PartialEq,Eq,Default,Ord,PartialOrd)]
pub struct Position(pub usize, pub usize);

impl Display for Position {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"{},{}",self.0,self.1)
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Field {
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

pub struct Map<T=u8> {
	pub field: Field,
	pub data: Vec<T>
}

impl<'a, L:Iterator<Item=&'a str>+Clone> From<L> for Map<u8> {
	fn from(lines: L) -> Self {
		let mut rows = lines.map(str::bytes).peekable();
		let width = rows.peek().unwrap().len();
		let data:Vec<u8> = rows.flatten().collect();
		let height = data.len() / width;
		Map { field: (width,height).into(), data }
	}
}

impl<T> Index<Position> for Map<T> {
	type Output = T;
	fn index(&self, p: Position) -> &Self::Output {
		&self.data[self.field.offset_of(p)]
	}
}
