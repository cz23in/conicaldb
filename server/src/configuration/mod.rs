pub struct Configuration {}

impl Configuration {
	pub fn new() -> Self {
		Configuration {}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn example_test_1() {
		assert_eq!(1, 1);
	}
}