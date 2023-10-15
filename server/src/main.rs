mod conical_error;
mod sql;
mod configuration;

use crate::configuration::Configuration;

fn main() {
	let _configuration: Configuration = Configuration::new();
}