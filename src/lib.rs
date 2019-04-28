use std::env;
use std::error::Error;
use std::fs;
use std::collections::HashMap;


/// Run MapReduce process. Return Err if filename does not exist.
pub fn run<'a>(filename: &'a str) -> Result<(), Box<dyn Error>> {
	println!("{}", filename);
	let contents = fs::read_to_string(filename)?;
	let mut result = HashMap::<&str, usize>::new();

	if let Err(err) = count(&contents, &mut result) {
		Err(err)?
	}

	let mut result: Vec<_> = result.into_iter().collect();
	result.sort_by(|a, b| a.1.cmp(&b.1).reverse());

	for (ip, count) in result {
		println!("{} {}", ip, count);
	}

	Ok(())
}

/// Get filename from user argument. Return Err if filename is not given.
pub fn get_filename(mut args: env::Args) -> Result<String, &'static str> {
	// first argument is location of program being run
	args.next();
	let filename = match args.next() {
		Some(expr) => expr,
		None => return Err("Didn't get a filename"),
	};

	Ok(filename)
}

fn count<'a>(contents: &'a str, result: &mut HashMap<&'a str, usize>) -> Result<(), &'static str>{
	for line in contents.lines() {
		let key = match line.split_whitespace().next() {
			Some(expr) => expr,
			None => return Err("No content in a line"),
		};
		*result.entry(key).or_insert(0) += 1;
	}
	Ok(())
}

fn map() {
	unimplemented!()
}

fn reduce() {
	unimplemented!()
}

#[test]
fn blank_data() {
	let contents = "\n\
	\n\
	\n";
	let mut result = HashMap::<&str, usize>::new();
	assert_eq!(count(&contents, &mut result).unwrap_err(), "No content in a line");
}