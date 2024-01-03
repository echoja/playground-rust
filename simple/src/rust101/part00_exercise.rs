// number or nothing

use std::fmt::Debug;

use self::NumberOrNothing::{Nothing, Number};

#[derive(Debug)]
enum NumberOrNothing {
	Number(i32),
	Nothing,
}

fn read_vec() -> Vec<i32> {
	vec![1, -5, 10, -10, 20, 30, 40, 50, 60]
}

fn vec_min(vec: Vec<i32>) -> NumberOrNothing {
	let mut result: NumberOrNothing = Nothing;

	for num in vec {
		match result {
			Number(prev_num) => {
				if prev_num > num {
					result = Number(num);
				}
			}
			Nothing => {
				result = Number(num);
			}
		}
	}

	result
}

impl PartialEq for NumberOrNothing {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Number(l0), Self::Number(r0)) => l0 == r0,
			_ => core::mem::discriminant(self) == core::mem::discriminant(other),
		}
	}
}

// impl Debug for NumberOrNothing {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		match self {
// 			Self::Number(arg0) => f.debug_tuple("Number").field(arg0).finish(),
// 			Self::Nothing => write!(f, "Nothing"),
// 		}
// 	}
// }

mod test {
	use super::vec_min;
	use super::NumberOrNothing::*;

	#[test]
	fn test_simple() {
		let result = vec_min(vec![3, 2, 1]);
		assert_eq!(result, Number(1));
	}

	#[test]
	fn test_nothing() {
		let result = vec_min(vec![]);
		assert_eq!(result, Nothing);
	}
}
