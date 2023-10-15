use std::sync::Arc;
use std::sync::Mutex;

use crate::logger::Logger;

pub struct Server {
	logger: Arc<Mutex<Logger>> 
}

impl Server {
	pub fn new(logger: Arc<Mutex<Logger>>) -> Self {
		Server {
			logger: logger
		}
	}
	pub fn start(&mut self) {
		self.logger.lock().unwrap().log_info(String::from("Starting server"));
	}
}