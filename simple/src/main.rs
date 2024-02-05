use std::fmt::Display;
use std::ops::Mul;
use std::{error::Error, fmt::Debug, hash::Hash, ops::Add};

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

trait Foo<T> {
	fn foo(&self) -> T;
}

impl Foo<i32> for i32 {
	fn foo(&self) -> i32 {
		println!("Foo for i32");
		*self
	}
}

// struct Person {
// 	age: u8,
// }

// impl Display for Person {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		write!(f, "person with age {}", self.age)
// 	}
// }

trait Barkable
where
	Self: Display,
{
	fn bark(&self);
}

impl<T: Display> Barkable for T {
	fn bark(&self) {
		println!("I'm {}, and I'm barking!!", self);
	}
}

fn main() {
	123.bark();
	3.0.bark();
	"hello".bark();
	let m = Message::Quit;
	m.bark();

	let p = Point { x: 1.0, y: 2.0 };

	let a = Some(5);
	if let Some(x) = &a {
		println!("x is {}", x);
	}

	match &p {
		Point { x, .. } => println!("x is not ref {}", x),
	}
	println!("1 == 2 = {}", 1 == 2);
	println!("1.eq(&2) = {}", 1.eq(&2));
	println!("{}", "abc".to_string() == "bcd".to_string());
}

fn foo<N: Mul<i32, Output = N>>(x: N) -> N {
	x * 100
}

#[derive(Debug)]
enum Message {
	Quit,
	Echo,
	Move,
}

impl Display for Message {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Message::Quit => write!(f, "Quit"),
			Message::Echo => write!(f, "Echo"),
			Message::Move => write!(f, "Move"),
		}
	}
}
