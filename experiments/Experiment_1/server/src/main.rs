mod server;

use crate::server::Server;

fn main() {
	println!("[INFO] Starting server.");
	
	let mut server = Server::new();
	server.start();
}