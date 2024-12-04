// https://adventofcode.com/2024/day/1

pub fn solve(input: &str) -> String {
	"".into()
}

#[cfg(test)]
mod test {

	use super::*;
	use crate::AppError;

	#[test]
	fn test_submit()-> Result<(), AppError> {
		crate::try_submit(1, &solve)
	}
}
