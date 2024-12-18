// https://adventofcode.com/2024/day/4

use std::iter::once;
use itertools::*;
use super::*;

/// Represents a rectangle board of
/// letters, stored as bytes.
///
struct LetterBoard {
	content: Vec<u8>,
	size:(usize,usize),
}

impl LetterBoard {

	fn horizontal<'a>(&'a self,fence:u8) -> impl Iterator<Item=u8> + 'a {
		let ref content = self.content;
		let (width,_) = self.size;
		// add sentinel value at end of each row
		content.chunks(width).map(move |chunk| chunk.iter().copied().chain(once(fence))).flatten()
	}

	fn diagonal_1<'a>(&'a self, fence:u8) -> impl Iterator<Item=u8> + 'a {
		let (width,height) = self.size;
		let ref content = self.content;

		//increased lengths to account for fences
		let fwidth = width+1;
		let fheight = height+1;

		(0..fwidth*fheight).map(move |i| {
			// increase each cycle
			let col = i/fheight;
			let x = (col + i%fheight)%fwidth;
			let y = i%fheight; // downward

			// fence bytes at right and bottom
			if x == width || y == height {
				fence
			} else {
				let offset = y*width+x;
				content[offset]
			}
		})
	}

	fn diagonal_2<'a>(&'a self,fence:u8) -> impl Iterator<Item=u8> + 'a {
		let (width,height) = self.size;
		let ref content = self.content;

		//increased lengths to account for fences
		let fwidth = width+1;
		let fheight = height+1;

		(0..fwidth*fheight).map(move |i| {
			// increase each cycle
			let col = i/fheight;
			let x = (col + i%fheight)%fwidth;
			let y = (fheight-1)-(i%fheight); // upward

			// fence bytes at right and top
			if x == width || y == 0 {
				fence
			} else {
				// correct y to ignore top
				let offset = (y-1)*width+x;
				content[offset]
			}
		})
	}

	fn vertical<'a>(&'a self,fence:u8) -> impl Iterator<Item=u8> + 'a {

		let (width,height) = self.size;
		let ref content = self.content;

		(0..width).cartesian_product(0..=height).map(move |(x,y)| {
			if y == height {
				fence
			} else{
				let offset = y*width+x;
				content[offset]
			}
		})
	}
}


impl<'a> From<Input<'a>> for LetterBoard {

	fn from(input:Input) -> Self {

		let rows = input.lines().map(str::bytes);
		// clone iterator, measure line width
		let width = rows.clone().into_iter().take(1).flatten().count();

		let content:Vec<u8> = rows.flatten().collect();

		// compute height
		let height = content.len() / width;

		Self { content, size: (width,height) }
	}
}

fn solve_1(input: &str) -> String {
	//
	fn count_matches(word:&[u8], input:&mut impl Iterator<Item=u8>) -> usize {

		let forward:Vec<u8>  = word.into();
		let backward:Vec<u8> = forward.iter().rev().copied().collect();

		// load bytes of first possible match into a "buffer"
		let mut window:Vec<u8> = input.take(word.len()).collect();
		let mut count:usize = 0;

		loop {

			if window == forward || window == backward { count += 1 }

			// fetch new byte
			let buffer = &mut window;

			let Some(c) = input.next() else {
				// Done
				break count
			};

			// update buffer
			buffer.rotate_left(1);
			if let Some(last) = buffer.last_mut() { *last = c };
		}
	}

	let board = LetterBoard::from(Input(input));
	let word = "XMAS";

	let mut count = 0;
	const FENCE:u8 = b'.';
	count += count_matches(word.as_ref(), &mut board.horizontal(FENCE));
	count += count_matches(word.as_ref(), &mut board.vertical(FENCE));
	count += count_matches(word.as_ref(), &mut board.diagonal_1(FENCE));
	count += count_matches(word.as_ref(), &mut board.diagonal_2(FENCE));

	count.to_string()
}

#[cfg(test)]
mod test {

	use aoc_driver::Part::*;
	use super::*;

	impl Display for LetterBoard {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "{}", self.lines().format("\n"))
			}
	}

	impl LetterBoard {

		fn lines(&self) -> impl Iterator<Item=&str> {
			use std::str::from_utf8;
			self.content.iter().as_slice().chunks(self.size.0).map(|c| from_utf8(c).unwrap())
		}

		fn len(&self) -> usize {
			self.content.len()
		}
	}

	pub fn string<'a>(i:impl Iterator<Item=u8> + 'a) -> String {
		use core::str::from_utf8;
		from_utf8(i.collect::<Vec<_>>().as_slice()).unwrap().to_string()
	}

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

	#[test]
	fn test_letterboard() {

		const INPUT:&str =
			r###"
			ABCD
			EFGH
			IJKL
			"###;

		let board = LetterBoard::from(Input(INPUT));
		assert_eq!(board.len(),12);

		// test horizontal

		// 5 rows of 6 items
		let expected = "ABCD.EFGH.IJKL.";
		let actual   = string(board.horizontal(b'.'));
		assert_eq!(actual, expected);

		// test vertical

		// 6 columns of 5 items
		let expected = "AEI.BFJ.CGK.DHL.";
		let actual   = string(board.vertical(b'.'));
		assert_eq!(actual, expected);

		// test diagonals

		// LTR "downward" strips
		let expected = "AFK.BGL.CH..D.I..EJ.";
		let actual   = string(board.diagonal_1(b'.'));
		assert_eq!(actual, expected);

		// LTR "upward" strips
		let expected = "IFC.JGD.KH..L.A..EB.";
		let actual   = string(board.diagonal_2(b'.'));
		assert_eq!(actual, expected);
	}

	#[test]
	fn part_1_example() {
		let expected : &str = "18";
		let actual:String = solve_1(EXAMPLE_INPUT);

		assert_eq!(actual, expected);
	}

	#[test]
	fn test_submit()-> Result<(), AppError> {
		try_submit(Day(4), solve_1, Part1)?;
		// try_submit(Day(4), solve_2, Part2)?;
		Ok(())
	}
}
