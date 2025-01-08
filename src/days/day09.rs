// https://adventofcode.com/2024/day/9

use std::ops::RangeInclusive;
use std::{num::NonZero, ops::{Deref, DerefMut}};

use super::*;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
struct FileId(NonZero<u16>);

impl TryFrom<u16> for FileId {
	type Error = ();

	fn try_from(n: u16) -> Result<Self, Self::Error> {
		match n {
			..u16::MAX => Ok(FileId(NonZero::new(n+1).unwrap())),
			u16::MAX   => Err(())
		}
	}
}

impl From<&FileId> for u16 {
	fn from(fid: &FileId) -> Self {
		fid.0.get()-1
	}
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Block {
	Unused,
	Used(FileId)
}

impl Block {

	fn set_file_id(&mut self,fid:FileId) {
		*self = Self::Used(fid)
	}

	fn get_file_id(&self) -> Option<FileId> {
		match self {
			Self::Used(fid) => Some(*fid),
			Self::Unused => None
		}
	}

	fn clear(&mut self) {
		*self = Self::Unused
	}

	fn used(&self) -> bool {
		matches!(self,Self::Used(_))
	}
}

struct Disk(Vec<Block>);

#[derive(Debug)]
struct Span {
	kind: Block,
	range: RangeInclusive<usize>
}

impl Span {
	pub fn len(&self) -> usize {
		1 + self.range.end() - self.range.start()
	}
}

impl Disk {
	pub fn new(map:&str) -> Self {
		// Precompute expanded size
		let cap = map.chars().fold(0,|sum,c| sum + c.to_digit(10).unwrap() as usize);
		// Expand map into individual blocks
		map.chars()
			.chunks(2)
			.into_iter()
			.enumerate()
			.fold(Disk(Vec::with_capacity(cap)),|mut disk,(i,mut chunk)| {
				// initialize used blocks
				if let Some(chr) = chunk.next() {
					let count = chr.to_digit(10).unwrap() as usize;
					let file_id = FileId::try_from(i as u16).unwrap();
					let new_len = disk.len() + count;
					disk.resize_with(new_len,||Block::Used(file_id));
				}
				// initialize unused blocks
				// The examples do not have a last chunk of unused
				if let Some(chr) = chunk.next() {
					let count = chr.to_digit(10).unwrap() as usize;
					let new_len = disk.len() + count;
					disk.resize_with(new_len,||Block::Unused);
				}
				disk
			})
	}

	pub fn pack(&mut self) {
		let mut i1 = 0;
		let mut i2 = self.len()-1;

		while i1 < i2 {

			if self[i2].used() && !self[i1].used() {
				let fid = self[i2].get_file_id().unwrap();
				self[i1].set_file_id(fid);
				self[i2].clear();
			}

			if matches!(self[i1],Block::Used(_)) { i1 += 1 };
			if matches!(self[i2],Block::Unused)  { i2 -= 1 };
		}
	}

	pub fn defrag(&mut self) {

		let files = self.spans().filter(|Span{kind,..}| matches!(kind,Block::Used(_))).collect_vec();

		for file in files.iter().rev() {
			let relocate_to = {
				let mut free_spans = self.spans().filter(|Span{kind,..}| matches!(kind,Block::Unused));
				free_spans.find(|span| span.len() >= file.len())
			};

			if let Some(dst) = relocate_to {
				// Can only move to the left!
				if dst.range.start() <= file.range.start() {
					self.move_blocks(file.range.clone(), *dst.range.start());
				}
			}
		}
	}

	pub fn spans(&self) -> impl Iterator<Item=Span> {
		self.iter().copied().enumerate().batching(|it| {
			let head = it.next()?;
			let (start,kind) = head;
			let (end,_) = it.take_while_ref(|(_,blk)| *blk == kind).last().unwrap_or(head);
			let span = Span { kind, range:start..=end };
			Some(span)
		})
	}

	fn move_blocks(&mut self, src:RangeInclusive<usize>,to:usize) {
		// Not really :D
		let len = src.end() - src.start();
		let dst = to..=to+len;
		for (s,d) in src.into_iter().zip(dst.into_iter()) {
			self[d] = self[s];
			self[s] = Block::Unused;
		}
	}

	fn checksum(&self) -> usize {
		self.iter()
			.enumerate()
			.map(|(i,b)|
				match b {
					Block::Unused => 0,
					Block::Used(fid) => i * u16::from(fid) as usize
				}
			).sum()
	}
}

impl Deref for Disk {
	type Target = Vec<Block>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for Disk {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

fn solve_1(input: &str) -> String {

	let map = Input(input).lines().take(1).collect::<String>();
	let mut disk = Disk::new(&map);
	disk.pack();
	disk.checksum().to_string()
}

fn solve_2(input: &str) -> String {

	let map = Input(input).lines().take(1).collect::<String>();
	let mut disk = Disk::new(&map);
	disk.defrag();
	disk.checksum().to_string()
}

#[cfg(test)]
mod test {

	use super::*;
	use aoc_driver::Part::*;

	impl Disk {
		fn show(&self,sep:&str) -> String {
			self.iter().map(|block| {
				match block {
					Block::Unused => '.',
					Block::Used(fid) => u16::from(fid).to_string().chars().next().unwrap()
				}
			}).join(sep)
		}
	}

	const INPUT_EXAMPLE:&str =
		r###"
		2333133121414131402
		"###;

	#[test]
	fn disk() {

		let map = Input(INPUT_EXAMPLE).lines().take(1).collect::<String>();
		let mut disk = Disk::new(&map);

		// Test initialization

		let actual = disk.show("");
		let expected = "00...111...2...333.44.5555.6666.777.888899";

		assert_eq!(actual,expected);

		// Moving blocks

		disk.move_blocks(40..=41, 2);

		let expected = "0099.111...2...333.44.5555.6666.777.8888..";
		let actual = disk.show("");

		assert_eq!(actual,expected);
	}

	#[test]
	fn part_1_example() {

		let map = Input(INPUT_EXAMPLE).lines().take(1).collect::<String>();
		let mut disk = Disk::new(&map);

		disk.pack();
		let actual = disk.show("");
		let expected = "0099811188827773336446555566..............";

		assert_eq!(actual,expected);

		let actual = disk.checksum().to_string();
		let expected = "1928";

		assert_eq!(actual,expected);
	}

	#[test]
	fn part_2_example() {
		let map = Input(INPUT_EXAMPLE).lines().take(1).collect::<String>();
		let mut disk = Disk::new(&map);
		disk.defrag();

		let expected = "00992111777.44.333....5555.6666.....8888..";
		let actual = disk.show("");

		assert_eq!(actual,expected);

	}

	#[test]
	fn submit()-> Result<(), AppError> {
		try_submit(Day(9), solve_1, Part1)?;
		try_submit(Day(9), solve_2, Part2)?;
		Ok(())
	}
}
