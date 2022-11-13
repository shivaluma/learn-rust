fn main() {
	// let bind a value to a variable
    let action = std::env::args().nth(1).expect("Please specify an action");
	let item = std::env::args().nth(2).expect("Please specify an item");

	println!("{:?}, {:?}", action, item);
}
