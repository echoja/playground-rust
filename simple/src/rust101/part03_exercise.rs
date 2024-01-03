// number or nothing

use std::{
	fmt::{Debug, Display},
	io::{self, prelude::*},
};

use self::SomethingOrNothing::{Nothing, Something};

#[derive(Debug, PartialEq)]
pub enum SomethingOrNothing<T> {
	Something(T),
	Nothing,
}

fn read_vec() -> Vec<i32> {
	let mut result = Vec::<i32>::new();
	let stdin = io::stdin();
	println!("Enter a list of numbers, one per line. End with Ctrl-D (Linux) or Ctrl-Z (Windows).");

	for line in stdin.lock().lines() {
		let line = line.unwrap();
		match line.trim().parse::<i32>() {
			Ok(n) => result.push(n),
			Err(_) => todo!(),
		}
	}

	result
}

fn vec_min(vec: Vec<i32>) -> SomethingOrNothing<i32> {
	let mut result: SomethingOrNothing<i32> = Nothing;

	for num in vec {
		match result {
			Something(prev_num) => {
				if prev_num > num {
					result = Something(num);
				}
			}
			Nothing => {
				result = Something(num);
			}
		}
	}

	result
}

trait Printable {
	fn print(self);
}

impl<T: Display> Printable for SomethingOrNothing<T> {
	fn print(self) {
		match self {
			Something(n) => println!("value: [SomethingOrNothing {}]", n),
			Nothing => println!("value: NOTHING"),
		}
	}
}

pub fn main() {
	let vec = read_vec();
	let min = vec_min(vec);
	min.print();
}

mod test {
	use super::vec_min;
	use super::SomethingOrNothing::*;

	#[test]
	fn test_simple() {
		let result = vec_min(vec![3, 2, 1]);
		assert_eq!(result, Something(1));
	}

	#[test]
	fn test_nothing() {
		let result = vec_min(vec![]);
		assert_eq!(result, Nothing);
	}
}
