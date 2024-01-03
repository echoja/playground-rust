mod gcd;
mod rust101;

// This decides which part is actually run.
fn main() {
	// rust101::part03_exercise::main();
	let a: u64 = 1 << 63;
	let aplus1: u64 = a.wrapping_add(1);
	println!("{}, {}, {}", a, aplus1, a.wrapping_add(a));
}
