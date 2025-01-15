use super::{Position, V2};

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

	pub fn contains(&self, coord:V2) -> bool {
		let V2{x,y} = coord;
		x >= 0 && y >= 0 &&
		x < (self.width  as i32) &&
		y < (self.height as i32)
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
