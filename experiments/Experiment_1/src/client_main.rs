mod connection;
mod buffer;

use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;

use crate::connection::Connection;

fn main() {
	let command_queue = Arc::new(Mutex::new(Vec::<String>::new())); 

	let command_thread_command_queue_copy = Arc::clone(&command_queue);
	std::thread::spawn(move || {
		command_thread(command_thread_command_queue_copy)
	});
	
	let mut conn = Connection::connect("localhost:8080");	
	
	loop {
		let mut command_option: Option<String> = None;
		{
			let mut cq = command_queue.lock().unwrap();
			if !cq.is_empty() {
				command_option = Some(cq.get(0).unwrap().clone());
				cq.remove(0);
			}
		}

		match command_option {
			Some(s) => {
				conn.send_packet(s);
			}
			None => {}
		}
		
		conn.io_step();
		
		let read_packet = conn.read_packet();
		
		match read_packet {
			Some(packet) => {
				println!("Received packet: {}", packet);
			}
			None => {}
		}
	}
}

fn command_thread(command_queue: Arc<Mutex<Vec<String>>>) {
	loop {
		let mut command = String::new();
		print!("$ ");
		std::io::stdout().flush();
		std::io::stdin().read_line(&mut command).expect("Could not get user input");
		let mut cq = command_queue.lock().unwrap();
		println!("[Command Thread] Received {}", command);
		cq.push(command);
	}
}