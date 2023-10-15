use std::sync::Arc;
use std::sync::Mutex;
use std::net::TcpListener;
use std::io::Write;

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
		self.start_client_io_thread();
		self.start_accept_thread();

		self.logger.lock().unwrap().log_info(String::from("Server has started."));
	}

	fn start_client_io_thread(&mut self) {
		let logger_clone = Arc::clone(&self.logger);
		std::thread::spawn(move || {
			logger_clone.lock().unwrap().log_info(String::from("Client IO Thread started."));
		});
	}

	fn start_accept_thread(&mut self) {
		let logger_clone = Arc::clone(&self.logger);
		std::thread::spawn(move || {
			logger_clone.lock().unwrap().log_info(String::from("Accept thread started."));

			let listener_result = TcpListener::bind("127.0.0.1:8080");

			if !listener_result.is_ok() {
				logger_clone.lock().unwrap().panic(String::from("Could not bind to given address."));
			}

			let listener = listener_result.unwrap();

			for stream in listener.incoming() {
				stream.unwrap().write_all("HTTP/1.1 200 OK\nContent-Type: text/html;\n\nHello".as_bytes()).unwrap();
			}
		}).join().unwrap();
	}
}