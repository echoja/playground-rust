use std::{iter::Sum, ops::Add};

#[derive(Debug)]
struct A {
	x: i32,
}

impl Add for A {
	type Output = A;

	fn add(self, rhs: Self) -> Self::Output {
		A { x: self.x + rhs.x }
	}
}

impl Sum for A {
	fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
		iter.fold(A { x: 0 }, |a, b| a + b)
	}
}

fn main() {
	let vec = vec![A { x: 1 }, A { x: 2 }, A { x: 3 }];
	let sum: A = vec.into_iter().sum();
	println!("sum: {:?}", sum);}

