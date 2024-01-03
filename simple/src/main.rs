mod gcd;
mod rust101;

use std::env;

fn main() {
	let mut numbers: Vec<u64> = Vec::new();

	for arg in env::args().skip(1) {
		numbers.push(arg.parse().expect("error parsing argument"))
	}

	if numbers.len() == 0 {
		eprintln!("Usage: gcd NUMBER ...");
		std::process::exit(1);
	}

	let first = numbers[0];
	let d = numbers[1..].iter().fold(first, |a, b| gcd::gcd(a, *b));

	println!("The greatest common divisor of {:?} is {}", numbers, d);
}
