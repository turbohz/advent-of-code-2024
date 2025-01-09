use std::ops::{Deref, Index, IndexMut};

use super::*;

pub struct Map<T=u8> {
	field: Field,
	data: Vec<T>
}

impl<T:Copy> Map<T> {
	pub fn iter(&self) -> impl Iterator<Item=T> {
		self.data.iter().copied()
	}

	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
		self.data.iter_mut()
	}
}

impl<T> Deref for Map<T> {
	type Target = Field;

	fn deref(&self) -> &Self::Target {
		&self.field
	}
}

impl<'a,T:From<u8>,L:Iterator<Item=&'a str>+Clone> From<L> for Map<T> {
	fn from(lines: L) -> Self {
		let mut rows = lines.map(str::bytes).peekable();
		let width:u16 = rows.peek().unwrap().len().try_into().unwrap();
		let data:Vec<T> = rows.flatten().map(T::from).collect();
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

impl<T> IndexMut<Position> for Map<T> {
	fn index_mut(&mut self, p: Position) -> &mut Self::Output {
		let offset = self.field.offset_of(p).unwrap();
		&mut self.data[offset]
	}
}
