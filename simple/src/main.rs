use std::collections::{binary_heap, btree_map, btree_set, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
	println!("Hello, world!");
}

#[test]
fn test_heap_peek_mut() {
	let mut heap = BinaryHeap::new();
	heap.push(1);
	heap.push(5);
	heap.push(2);
	heap.push(3);
	let a = heap.peek_mut().unwrap();
	assert_eq!(*a, 5);
}

#[test]
fn test_collection_from_iter() {
	let hash_map1: HashMap<_, _> = HashMap::from_iter(vec![
		(1, "kim"),
		(5, "park"),
		(2, "lee"),
		(4, "choi"),
		(3, "go"),
	]);
	let hash_map2: HashMap<_, _> =
		vec![(1, "kim"), (5, "park"), (2, "lee"), (4, "choi"), (3, "go")]
			.into_iter()
			.collect();

	println!("hash_map1: {:#?}", hash_map1);
	println!("hash_map2: {:#?}", hash_map2);
}

#[test]
fn test_collection_access() {
	let mut vec = vec![1, 5, 2, 4, 3];
	let mut vec_deque = VecDeque::from(vec![1, 5, 2, 4, 3]);
	let mut heap = BinaryHeap::from(vec![1, 5, 2, 4, 3]);
	let mut hash_set: HashSet<i32> = vec![1, 5, 2, 4, 3].into_iter().collect();
	let mut hash_map: HashMap<i32, &str> =
		vec![(1, "kim"), (5, "park"), (2, "lee"), (4, "choi"), (3, "go")]
			.into_iter()
			.collect();
	let mut btree_set: std::collections::BTreeSet<i32> = vec![1, 5, 2, 4, 3].into_iter().collect();
	let mut btree_map: std::collections::BTreeMap<i32, &str> =
		vec![(1, "kim"), (5, "park"), (2, "lee"), (4, "choi"), (3, "go")]
			.into_iter()
			.collect();
}

#[test]
fn test_custom_set() {
	#[derive(Debug)]
	struct Person {
		name: String,
		age: u32,
	}

	impl PartialEq for Person {
		fn eq(&self, other: &Self) -> bool {
			self.name == other.name
		}
	}

	impl Eq for Person {}

	impl std::hash::Hash for Person {
		fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
			self.name.hash(state);
		}
	}

	let mut set = HashSet::new();
	set.insert(Person {
		name: "kim".to_string(),
		age: 20,
	});
	println!("{:?}", set);
}

#[test]
fn test_two_sum() {
	struct Solution;

	impl Solution {
		pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
			let mut map = HashMap::new();
			for (i, &num) in nums.iter().enumerate() {
				if let Some(&j) = map.get(&(target - num)) {
					return vec![j as i32, i as i32];
				}
				map.insert(num, i);
			}
			vec![]
		}
	}

	let nums = vec![2, 7, 11, 15];
	let target = 9;
	let result = Solution::two_sum(nums, target);
	assert_eq!(result, vec![0, 1]);

	let nums = vec![3, 2, 4];
	let target = 6;
	let result = Solution::two_sum(nums, target);
	assert_eq!(result, vec![1, 2]);

	let nums = vec![3, 3];
	let target = 6;
	let result = Solution::two_sum(nums, target);
	assert_eq!(result, vec![0, 1]);
}

macro_rules! my_vec {
	( $( $x:expr ),* ) => {
		{
			let mut temp_vec = Vec::new();
			$(
				temp_vec.push($x);
			)*
			temp_vec
		}
	};
	( $( $x:expr ),+ ,) => {
		my_vec![$($x),+]
	};

	( $( $x:ident )comma+) => {
		my_vec![$($x),+]
	};
}

#[test]
fn test_my_vec() {
	let v = my_vec![1, 2, 3];
	assert_eq!(v, vec![1, 2, 3]);

	let v2 = my_vec![1, 2, 3,];
	assert_eq!(v2, vec![1, 2, 3]);

	let a = 1;
	let b = 2;
	let c = 3;
	let v3 = my_vec![a comma b comma c];
	assert_eq!(v3, vec![1, 2, 3]);
}

macro_rules! mac_test1 {
	( $x:expr) => {
		$x
	};
	( myname is $x:expr) => {
		$x
	};
}

#[test]
fn test_macro() {
	let a = 10;
	let b = mac_test1!(a);
	assert_eq!(b, 10);

	let c = mac_test1!(myname is a);
	assert_eq!(c, 10);
}

macro_rules! complain {
	( $x:expr ) => {{
		println!("Oh no! {}", $x);
	}};
	(user : $userid:tt, $msg:expr) => {{
		println!("User {} says: {}", $userid, $msg);
	}};
}

#[test]
fn test_macro2() {
	complain!("I'm sad");
	complain!(user: "kim", "I'm happy");
}
