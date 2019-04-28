use std::env;
use std::process;

use map_reduce;

fn main() {
	let filename = map_reduce::get_filename(env::args()).unwrap_or_else(|err| {
		eprintln!("Problem parsing arguments: {}", err);
		process::exit(1);
	});
	if let Err(e) = map_reduce::run(&filename) {
		eprintln!("Application error: {}", e);
		process::exit(1);
	}
}