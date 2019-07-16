mod map;

use crate::map::*;

fn main() {
	let mut mind = Map::new();
	println!("mind: {:?}\n", mind);

	let new_room = mind.move_left();
	println!("new_room: {:?}", new_room);
}