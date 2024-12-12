// https://adventofcode.com/2024/day/4

use itertools::*;

use super::*;

struct LetterBoard {
	content: Vec<u8>,
	size:(usize,usize),
}

impl LetterBoard {

	#[cfg(test)]
	fn lines(&self) -> impl Iterator<Item=&str> {
		use std::str::from_utf8;
		self.content.iter().as_slice().chunks(self.size.0).map(|c| from_utf8(c).unwrap())
	}
}

impl Display for LetterBoard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.lines().format("\n"))
	}
}

impl<'a> From<Input<'a>> for LetterBoard {

	fn from(input:Input) -> Self {

		let mut rows = input.lines().map(str::bytes).peekable();
		// measure line width
		let width = rows.peek().cloned().and_then(|l| Some(l.count())).unwrap();
		// linealize
		let content:Vec<u8> = rows.flatten().collect();
		// compute height
		let height = content.len() / width;

		Self { content, size: (width,height) }
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use aoc_driver::Part::*;

	const EXAMPLE_INPUT:&str =
		r###"
		MMMSXXMASM
		MSAMXMSMSA
		AMXSXMAAMM
		MSAMASMSMX
		XMASAMXAMM
		XXAMMXXAMA
		SMSMSASXSS
		SAXAMASAAA
		MAMMMXMMMM
		MXMXAXMASX
		"###;
}
