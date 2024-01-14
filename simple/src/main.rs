mod gcd;
mod rust101;

use std::env;

#[derive(Clone)]
struct Point {
	x: f64,
	y: f64,
}

fn print_number(x: i32) {
	println!("x is {}", x);
}

// fn main() {
// 	let mut numbers: Vec<u64> = Vec::new();

// 	let num = 12_234_.56_f32;
// 	println!("Hello, world! {}", num);

// 	let moving = Some(vec![
// 		"hello".to_string(),
// 		"hello".to_string(),
// 		"hello".to_string(),
// 	]);

// 	if let Some(mut moving) = moving {
// 		println!("moving: {:?}", moving);
// 	}

// 	let a = &&&mut &&&mut 13;
// 	println!("a: {}", a);
// 	print_number(******a);

// 	for arg in env::args().skip(1) {
// 		numbers.push(arg.parse().expect("error parsing argument"))
// 	}

// 	if numbers.len() == 0 {
// 		eprintln!("Usage: gcd NUMBER ...");
// 		std::process::exit(1);
// 	}

// 	let first = numbers[0];
// 	let d = numbers[1..].iter().fold(first, |a, b| gcd::gcd(a, *b));

// 	println!("The greatest common divisor of {:?} is {}", numbers, d);
// }

// fn main() {
// 	let a = &&&mut &&&mut 13;
// 	println!("a: {}", a);
// 	print_number(******a);

// 	let mut b = &a;
// }

fn main() {
	let a;
	{
		let b = 10;
		a = &b;
		println!("a: {}", a);
	}
}
