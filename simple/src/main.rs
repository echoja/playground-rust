use std::collections::{binary_heap, btree_map, btree_set, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
	println!("Hello, world!");
	test_collection_from_iter();
	test_collection_access();
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
