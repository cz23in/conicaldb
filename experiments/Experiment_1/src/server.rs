

pub struct Server {

}

impl Server {
	pub fn new() -> Self {
		Self {}
	}
	
	pub fn start(&mut self) {
		self.start_io_thread();
		self.start_listening_thread();
		self.start_processing_thread();
	}
	
	fn start_listening_thread(&mut self) {
		println!("[INFO] listening_thread: Listening thread has started.");
	}
	
	fn start_io_thread(&mut self) {
		println!("[INFO] io_thread: IO thread has started.");
	}
	
	fn start_processing_thread(&mut self) {
		println!("[INFO] processing_thread: Processing thread has started.");
	}
}