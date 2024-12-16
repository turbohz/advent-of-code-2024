// https://adventofcode.com/2024/day/4

use itertools::*;
use super::*;

/// Represents a rectangle board of
/// letters, stored as bytes.
///
/// Since we want to search for words
/// in it, in different directions,
/// the board is surrounded by `Z`
/// bytes (which **must** not be
/// part of the search term)
/// to avoid matching across its edges.
///
struct LetterBoard<const Z:u8> {
	content: Vec<u8>,
	size:(usize,usize),
}

impl<const Z:u8> LetterBoard<Z> {

	fn horizontal<'a>(&'a self) -> impl Iterator<Item=u8> + 'a {
		let ref content = self.content;
		content.into_iter().copied()
	}

	fn diagonal_1<'a>(&'a self) -> impl Iterator<Item=u8> + 'a {
		let (width,height) = self.size;
		let len = self.content.len();
		let ref content = self.content;

		(0..len).map(move |i| {
			let col = i/height;
			let x = (col + i%height)%width;
			let y = i%height; // downward
			let offset = y*width+x;
			content[offset]
		})
	}

	fn diagonal_2<'a>(&'a self) -> impl Iterator<Item=u8> + 'a {
		let (width,height) = self.size;
		let len = self.content.len();
		let ref content = self.content;

		(0..len).map(move |i| {
			let col = i/height;
			let x = (col + i%height)%width;
			let y = (height-1)-(i%height); // upward
			let offset = y*width+x;
			content[offset]
		})
	}

	fn vertical<'a>(&'a self) -> impl Iterator<Item=u8> + 'a {

		let (width,height) = self.size;
		let len = self.content.len();
		let ref content = self.content;

		(0..len).map(move |i| {
			// sequence of offsets to the vec
			// to obtain the elements in a sequence per column
			let offset = i*width%len + i/height;
			content[offset]
		})
	}
}

impl<'a,const Z:u8> From<Input<'a>> for LetterBoard<Z> {

	fn from(input:Input) -> Self {

		use std::iter::once;
		use itertools::chain;

		let rows = input.lines().map(str::bytes);
		// clone iterator, measure line width, add one for the sentinel byte 'Z'
		let width = rows.clone().into_iter().take(1).flatten().count() + 2;

		let content:Vec<u8> = chain!(
			// initial line of 'Z' sentinel bytes
			repeat_n(Z, width),
			// add 'Z' sentinel byte at the end of each row
			rows.map(|row| chain!(once(Z),row,once(Z))).flatten(),
			// final line of 'Z' sentinel bytes
			repeat_n(Z, width)
		).collect();

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

	let board = LetterBoard::<b'.'>::from(Input(input));
	let word = "XMAS";

	let mut count = 0;
	count += count_matches(word.as_ref(), &mut board.horizontal());
	count += count_matches(word.as_ref(), &mut board.vertical());
	count += count_matches(word.as_ref(), &mut board.diagonal_1());
	count += count_matches(word.as_ref(), &mut board.diagonal_2());

	count.to_string()
}

#[cfg(test)]
mod test {

	use aoc_driver::Part::*;
	use super::*;

	impl<const Z:u8> Display for LetterBoard<Z> {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "{}", self.lines().format("\n"))
			}
	}

	impl<const Z:u8> LetterBoard<Z> {

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

		let board = LetterBoard::<b'.'>::from(Input(INPUT));
		assert_eq!(board.len(),30);

		// test horizontal

		// 5 rows of 6 items
		let expected = "...... .ABCD. .EFGH. .IJKL. ......".replace(" ", "");
		let actual = string(board.horizontal());
		assert_eq!(actual, expected);

		// test vertical

		// 6 columns of 5 items
		let expected = "..... .AEI. .BFJ. .CGK. .DHL. .....".replace(" ", "");
		let actual = string(board.vertical());
		assert_eq!(actual, expected);

		// test diagonals

		// LTR "downward" strips
		let expected = ".AFK. .BGL. .CH.. .D... ...I. ..EJ.".replace(" ", "");
		let actual = string(board.diagonal_1());
		assert_eq!(actual, expected);

		// LTR "upward" strips
		let expected = ".IFC. .JGD. .KH.. .L... ...A. ..EB.".replace(" ", "");
		let actual = string(board.diagonal_2());
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
