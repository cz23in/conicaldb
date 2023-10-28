const MAX_BUFFER_SIZE: usize = 50;

pub struct Buffer {
	pub data: [u8; MAX_BUFFER_SIZE],
	pub index: usize
}

impl Buffer {
	pub fn new() -> Self {
		Buffer {
			data: [0; MAX_BUFFER_SIZE],
			index: 0
		}
	}

	pub fn drop_left(&mut self, n: usize) -> Option<Vec<u8>> {
		if n >= self.len() || n < 1 {
			return None
		}

		let mut saved_dropped = Vec::<u8>::new();
		for i in 0..n {
			saved_dropped.push(self.data[i]);
			if i + n < MAX_BUFFER_SIZE {
				self.data[i] = self.data[i + n];
			}
		}
		self.index -= n;

		Some(saved_dropped)
	}

	pub fn push_string(&mut self, target: String) -> bool {
		if (target.len() + self.index > MAX_BUFFER_SIZE) {
			return false;
		}

		let characters = target.as_bytes();
		for i in 0..characters.len() {
			self.data[self.index] = characters[i];
			self.index += 1;
		}

		true
	}

	pub fn len(&mut self) -> usize {
		self.index
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn buffer_to_text(buffer: &mut Buffer) -> String {
		let mut result = String::new();
		for i in 0..buffer.len() {
			result.push(buffer.data[i] as char);
		}
		result

	}

	#[test]
	fn buffer_test_initialization() {
		let mut buffer = Buffer::new();
		assert_eq!(buffer_to_text(&mut buffer), String::new());
	}

	#[test]
	fn buffer_test_push_simple() {
		let mut buffer = Buffer::new();
		let result = buffer.push_string(String::from("12345"));
		assert_eq!(result, true);
		assert_eq!(buffer_to_text(&mut buffer), String::from("12345"));
	}

	#[test]
	fn buffer_test_full() {
		let mut buffer = Buffer::new();
		let mut example_str = String::from("This is some example string.");
		buffer.push_string(example_str.clone());
		let mut bigger_string = String::new();
		for i in 0..MAX_BUFFER_SIZE {
			bigger_string.push('A');
		}
		let result = buffer.push_string(bigger_string);
		assert_eq!(result, false);
		assert_eq!(buffer_to_text(&mut buffer), example_str);
	}

	#[test]
	fn buffer_test_drop_more_than_has() {
		let mut buffer = Buffer::new();
		buffer.push_string(String::from("12345"));
		let result = buffer.drop_left(6);
		assert_eq!(result.is_some(), false);
		assert_eq!(buffer_to_text(&mut buffer), String::from("12345"));
	}
}