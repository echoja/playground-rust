use std::{cmp, ops};

use super::part05_exercise::BigInt;

fn overflowing_add(a: u64, b: u64, carry: bool) -> (u64, bool) {
	let sum = a.wrapping_add(b);
	if sum >= a {
		let carry_sum = sum.wrapping_add(if carry { 1 } else { 0 });
		(carry_sum, if carry_sum >= sum { false } else { true })
	} else {
		(sum + if carry { 1 } else { 0 }, true)
	}
}

#[test]
fn test_overflowing_add() {
	assert_eq!(overflowing_add(10, 100, false), (110, false));
	assert_eq!(overflowing_add(10, 100, true), (111, false));
	assert_eq!(overflowing_add(1 << 63, 1 << 63, false), (0, true));
	assert_eq!(overflowing_add(1 << 63, 1 << 63, true), (1, true));
	assert_eq!(overflowing_add(1 << 63, (1 << 63) - 1, true), (0, true));
}

pub trait Minimum {
	fn min<'a>(&'a self, other: &'a Self) -> &'a Self;
}

impl PartialEq for BigInt {
	fn eq(&self, other: &Self) -> bool {
		self.data == other.data
	}
}

impl ops::Add<BigInt> for BigInt {
	type Output = BigInt;

	fn add(self, rhs: BigInt) -> Self::Output {
		let max_len = cmp::max(self.data.len(), rhs.data.len());
		let mut result_vec: Vec<u64> = Vec::with_capacity(max_len);
		let mut carry = false; /* the current carry bit */
		for i in 0..max_len {
			let lhs_val = if i < self.data.len() { self.data[i] } else { 0 };
			let rhs_val = if i < rhs.data.len() { rhs.data[i] } else { 0 };
			let (sum, new_carry) = overflowing_add(lhs_val, rhs_val, carry);
			result_vec.push(sum);
			carry = new_carry;
		}

		if carry {
			result_vec.push(1);
		}

		BigInt { data: result_vec }
	}
}

impl ops::Add<&BigInt> for &BigInt {
	type Output = BigInt;

	fn add(self, rhs: &BigInt) -> Self::Output {
		let left_copied = self.clone();
		let right_copied = rhs.clone();
		left_copied + right_copied
	}
}
// impl<'a, 'b> ops::Add<&'a BigInt> for &'b BigInt {
// 	type Output = BigInt;

// 	fn add(self, rhs: &'a BigInt) -> Self::Output {
// 		let left_copied = self.clone();
// 		let right_copied = rhs.clone();
// 		left_copied + right_copied
// 	}
// }

impl Minimum for BigInt {
	fn min<'a>(&'a self, other: &'a Self) -> &'a Self {
		debug_assert!(self.test_invariant() && other.test_invariant());

		if self.data.len() < other.data.len() {
			self
		} else if self.data.len() > other.data.len() {
			other
		} else {
			let mut result: &BigInt = other;
			for i in 0..self.data.len() {
				if self.data[i] < other.data[i] {
					result = self;
				} else if self.data[i] > other.data[i] {
					result = other;
				}
			}
			result
		}
	}
}

pub fn vec_min<T: Minimum>(v: &Vec<T>) -> Option<&T> {
	let mut min: Option<&T> = None;
	for e in v {
		min = Some(match min {
			None => e,
			Some(n) => n.min(e),
		});
	}
	min
}

mod test {
	use super::{BigInt, Minimum};

	mod add {
		use super::{BigInt, Minimum};

		#[test]
		fn general() {
			let a = BigInt {
				data: vec![5, 2, 3],
			};
			let b = BigInt {
				data: vec![5, 2, 4],
			};
			let result = a + b;
			assert_eq!(
				result,
				BigInt {
					data: vec![10, 4, 7]
				}
			)
		}

		#[test]
		fn general_ref() {
			let a = BigInt {
				data: vec![5, 2, 3],
			};
			let b = BigInt {
				data: vec![5, 2, 4],
			};
			let result = &a + &b;
			assert_eq!(
				result,
				BigInt {
					data: vec![10, 4, 7]
				}
			)
		}

		#[test]
		fn overflow1() {
			let result = BigInt {
				data: vec![1 << 63],
			} + BigInt {
				data: vec![1 << 63],
			};
			assert_eq!(result, BigInt { data: vec![0, 1] })
		}

		#[test]
		fn overflow2() {
			let result = BigInt {
				data: vec![u64::MAX, u64::MAX, u64::MAX],
			} + BigInt { data: vec![1] };
			assert_eq!(
				result,
				BigInt {
					data: vec![0, 0, 0, 1]
				}
			)
		}
	}

	#[test]
	fn test_general() {
		let a = BigInt {
			data: vec![5, 2, 3],
		};
		let b = BigInt {
			data: vec![5, 2, 4],
		};
		let result = a.min(&b);
	}

	#[test]
	fn test_equal_general() {
		let a = BigInt {
			data: vec![5, 2, 3],
		};
		let b = BigInt {
			data: vec![5, 2, 3],
		};
		assert_eq!(a, b);
	}

	#[test]

	fn test_not_equal_general() {
		let a = BigInt {
			data: vec![5, 2, 3],
		};
		let b = BigInt {
			data: vec![5, 2, 4],
		};
		assert_ne!(a, b);
	}
}
