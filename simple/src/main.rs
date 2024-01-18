use std::{error::Error, ops::Add};

mod gcd;
mod rust101;

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

// struct S<'a> {
// 	x: &'a i32,
// 	y: &'a i32,
// }

// fn main() {
// 	let x = 10;
// 	let r;
// 	{
// 		let y = 20;
// 		{
// 			let s = S { x: &x, y: &y };
// 			r = s.x;
// 		}
// 	}
// 	// println!("r: {}", r);
// }

#[derive(Debug)]
struct Person {
	age: u8,
}

impl Add<u8> for Person {
	type Output = Person;

	fn add(self, rhs: u8) -> Self::Output {
		Person {
			age: self.age + rhs,
		}
	}
}

fn main() {
	let person = Person { age: 10 };
	let person = person + 20;
	println!("person: {:?}", person);
}

fn say_hello(out: &mut dyn std::io::Write) -> std::io::Result<()> {
	out.write_all(b"hello world\n")?;
	out.flush()?;
	Ok(())
}
