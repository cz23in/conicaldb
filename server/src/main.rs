mod conical_error;
mod sql;
mod configuration;
mod server;
mod logger;

use std::sync::Arc;
use std::sync::Mutex;

use crate::configuration::Configuration;
use crate::server::Server;
use crate::logger::Logger;

fn main() {
	let _configuration: Configuration = Configuration::new();
	let logger: Arc<Mutex<Logger>> = Arc::new(Mutex::new(Logger::new()));
	let mut server: Server = Server::new(Arc::clone(&logger));
	server.start();
}