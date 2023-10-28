use std::net::TcpStream;
use std::io::{Read, Write};

use crate::buffer::Buffer;

const PACKET_SIZE: usize = 1028;

pub struct Connection {
	pub tcp_stream: TcpStream,
	pub incoming_buffer: Buffer,
	pub outgoing_buffer: Buffer,
	pub incoming_queue: Vec<String>
}

impl Connection {
	pub fn connect(connection_string: &str) -> Self {
		let connection_result = TcpStream::connect(connection_string);
		let mut connection = connection_result.expect("Could not connect.");
		connection.set_nonblocking(true).expect("Could not set to non-blocking.");
		Self {
			tcp_stream: connection,
			incoming_buffer: Buffer::new(),
			outgoing_buffer: Buffer::new(),
			incoming_queue: Vec::<String>::new()
		}
	}

	pub fn io_step(&mut self) {
		self.attempt_read();
		self.attempt_write();
	}

	fn attempt_read(&mut self) {
		let mut buffer = [0; PACKET_SIZE];
		let read_result = self.tcp_stream.read(&mut buffer);

		if read_result.is_ok() {
			let n = read_result.unwrap();
			if n > 0 {
				let buffer_subset = &buffer[0..n].to_vec();
				let read_string: &str = std::str::from_utf8(buffer_subset).unwrap();
				self.incoming_buffer.push_string(String::from(read_string));
			}
		}
	}

	fn attempt_write(&mut self) {
		let mut to_send_size = self.outgoing_buffer.len();
		
		if to_send_size <= 0 {
			return;
		}
		
		// Cap the sending of packets to packet_size.
		if to_send_size > PACKET_SIZE {
			to_send_size = PACKET_SIZE;
		}
		
		let packet = self.outgoing_buffer.drop_left(to_send_size);
		self.tcp_stream.write_all(packet.as_slice()).expect("Could not write.");
	}
	
	pub fn send_packet(&mut self, packet: String) {
		self.outgoing_buffer.push_string(packet);
		self.outgoing_buffer.push_string(String::from("\n"));
	}
	
	pub fn read_packet(&mut self) -> Option<String> {
		if self.incoming_queue.is_empty() {
			return None;
		}
		
		let queued_item = self.incoming_queue[0].clone();
		self.incoming_queue.remove(0);
	
		Some(queued_item)
	} 
}
