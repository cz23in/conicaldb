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

	pub fn drop_left(&mut self, n: usize) -> Vec<u8> {
		let mut result = Vec::<u8>::new();
		for i in n..self.data.len() {
			result.push(self.data[i-n]);
			self.data[i - n] = self.data[i];
		}
		self.index -= n;
		result
	}

	pub fn push_string(&mut self, target: String) {
		let characters = target.as_bytes();
		for i in 0..characters.len() {
			if self.index == self.data.len() {
				break;
			}
			self.data[self.index] = characters[i];
			self.index += 1;
		}
	}

	pub fn len(&mut self) -> usize {
		self.index
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn buffer_to_text(buffer: &Buffer) -> String {
		let data_buffer: Vec<u8> = buffer.data[0..buffer.index].to_vec();
		let result = std::str::from_utf8(&data_buffer).expect("Could not extract data from buffer.");
		String::from(result)
	}

	#[test]
	fn buffer_test_1() {
		let mut buffer = Buffer::new();
		assert_eq!(buffer_to_text(&buffer), String::from(""));

		buffer.push_string(String::from("12345"));
		assert_eq!(buffer_to_text(&buffer), String::from("12345"));

		let drop_lefted = buffer.drop_left(3);
		let compare_str: &str = "123";
		let compare_vec: Vec<u8> = compare_str.as_bytes().to_vec();
		assert_eq!(drop_lefted, compare_vec);

		assert_eq!(buffer_to_text(&buffer), String::from("123"));
	}
}