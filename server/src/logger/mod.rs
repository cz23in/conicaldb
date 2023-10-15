pub struct Logger {}

impl Logger {
	pub fn new() -> Self {
		Logger {}
	}
	
	pub fn log_info(&mut self, message: String) {
		println!("[info] {}", message)
	}
}