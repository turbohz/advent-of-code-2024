use std::{fmt::Display, ops::{Deref, Index}};

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

#[derive(Debug, Clone, Copy)]
pub struct Field {
	pub height: u16,
	pub width: u16,
}

impl From<(u16,u16)> for Field {
	fn from(size: (u16,u16)) -> Self {
		Self { width: size.0, height: size.1 }
	}
}

impl Field {

	fn len(&self) -> usize {
		(self.width*self.height) as usize
	}

	pub fn size(&self)->(u16,u16) {
		(self.width,self.height)
	}

	#[inline]
	pub fn stride(&self) -> usize {
		self.width as usize
	}

	pub fn offset_of(&self, p:Position) -> Option<usize> {
		if p <= self.last_position() {
			let Position{x,y} = p;
			let stride:u16 = self.stride() as u16;
			(y*stride+x).try_into().ok()
		} else {
			None
		}
	}

	pub fn position_of(&self, offset:usize) -> Option<Position> {
		let stride = self.stride();
		if offset < self.len() {
			let x = (offset % stride).try_into().ok()?;
			let y = (offset / stride).try_into().ok()?;
			Some(Position{x,y})
		} else {
			None
		}
	}

	pub fn last_position(&self) -> Position {
		self.position_of(self.len()-1).unwrap()
	}
}

pub struct Map<T=u8> {
	field: Field,
	data: Vec<T>
}

impl<T:Copy> Map<T> {
	pub fn iter(&self) -> impl Iterator<Item=T> {
		self.data.iter().copied()
	}
}

impl<T> Deref for Map<T> {
	type Target = Field;

	fn deref(&self) -> &Self::Target {
		&self.field
	}
}

impl<'a, L:Iterator<Item=&'a str>+Clone> From<L> for Map<u8> {
	fn from(lines: L) -> Self {
		let mut rows = lines.map(str::bytes).peekable();
		let width:u16 = rows.peek().unwrap().len().try_into().unwrap();
		let data:Vec<u8> = rows.flatten().collect();
		let height:u16 = u16::try_from(data.len()).unwrap() / width;
		let field:Field = (width,height).into();
		Map { field, data }
	}
}

impl<T> IntoIterator for Map<T> {
	type Item = T;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.data.into_iter()
	}
}

impl<T> Index<Position> for Map<T> {
	type Output = T;
	fn index(&self, p: Position) -> &Self::Output {
		let offset = self.field.offset_of(p).unwrap();
		&self.data[offset]
	}
}
