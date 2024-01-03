#[derive(Debug)]
pub struct BigInt {
	pub data: Vec<u64>,
}

impl BigInt {
	fn new(x: u64) -> BigInt {
		match x {
			0 => BigInt { data: vec![] },
			n => BigInt { data: vec![n] },
		}
	}

	pub fn test_invariant(&self) -> bool {
		let n = self.data.len();
		match n {
			0 => true,
			num => self.data[num - 1] != 0,
		}
	}
}

impl Clone for BigInt {
	fn clone(&self) -> Self {
		Self {
			data: self.data.clone(),
		}
	}
}

use super::part03_exercise::SomethingOrNothing::{self, *};

impl<T: Clone> Clone for SomethingOrNothing<T> {
	fn clone(&self) -> Self {
		match *self {
			Something(ref v) => Something(v.clone()),
			Nothing => Nothing,
		}
	}
}

enum Variant {
	Number(i32),
	Text(String),
}

fn work_on_variant(mut var: Variant, text: String) {
	let mut ptr: &mut i32;
	match var {
		Variant::Number(ref mut n) => ptr = n,
		Variant::Text(_) => return,
	}

	// var = Variant::Text(text); // BAD!
	*ptr = 1337;
}
