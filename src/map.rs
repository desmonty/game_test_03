use rand::seq::SliceRandom;
use rand::thread_rng;
use std::slice::Iter;

/*
	Idea enum.
	Used to know which room we are going to see.
*/
pub const NUM_IDEAS: usize = 5;
#[derive(Debug, Copy, Clone)]
pub enum Idea {
	Deny,
	Rage,
	Negociation,
	Depression,
	Acceptation,
}

impl Idea {
	pub fn iterator() -> Iter<'static, Idea> {
		static IDEAS: [Idea; NUM_IDEAS] = [
			Idea::Deny,
			Idea::Rage,
			Idea::Negociation,
			Idea::Depression,
			Idea::Acceptation
		];
		IDEAS.into_iter()
	}
}


/*
	EmotionalContext enum.
	Used to know in each kind of mindset we should see the room.
	This can be seen as a filter.
*/
#[derive(Debug, Copy, Clone)]
pub enum EmotionalContext {
	Reality,
	Love,
	Logic,
}


/*
	Room struct. Used to know where we are currently.
*/
#[derive(Debug, Copy, Clone)]
pub struct Room {
	pub idea: Idea,
	pub emotional_context: EmotionalContext,
}


/*
	Map struct. Used to known which room we are in
	and deal with the transition between them.
*/
#[derive(Debug, Copy, Clone)]
pub struct Map {
	pub room: Room,
	pub ideas: [Idea; NUM_IDEAS],
	pub ideas_horizontal: usize[NUM_IDEAS],
	pub ideas_vertical: usize[NUM_IDEAS],
	indice: usize,
}

impl Map {
	pub fn new() -> Map {
		let mut index: usize = 0;
		let mut ideas = [Idea::Deny; NUM_IDEAS];
		let mut ideas_horizontal: usize[NUM_IDEAS] = [0..NUM_IDEAS];
		let mut ideas_vertical: usize[NUM_IDEAS] = [0..NUM_IDEAS];

		// for idea in Idea::iterator() {
		// 	ideas[index] = *idea;
		// 	index += 1;
		// }
		
		/*
			We shuffle everything, we are lost.
			Even if you know how it works, you'll have difficulty
			to understand where you are exactly.
		*/
		let mut rng = thread_rng();
		ideas_horizontal.shuffle(&mut rng);

		rng = thread_rng();
		ideas_vertical.shuffle(&mut rng);

		/*
			And we start in the first room 
			with nothing but reality
			obviously.
		*/
		let room = Room {
			idea: ideas[0],
			emotional_context: EmotionalContext::Reality,
		};

		let indice: usize = 0;

		Map {
			room,
			ideas,
			indice
		}
	}

	pub fn move_left(&mut self) -> Room {
		if self.indice >= NUM_IDEAS {
			self.indice = 0;
		}

		if self.indice == 0 {
			self.indice = NUM_IDEAS - 1;
		} else {
			self.indice = self.indice - 1;
		}

		self.room.idea = self.ideas[self.indice];

		self.room
	}
	pub fn move_right(&mut self) -> Room {
		if self.indice >= NUM_IDEAS {
			self.indice = 0;
		}

		if self.indice == NUM_IDEAS - 1 {
			self.indice = 0;
		} else {
			self.indice = self.indice + 1;
		}

		self.room.idea = self.ideas[self.indice];

		self.room
	}
	pub fn move_up(&mut self) -> Room {
		if self.indice >= NUM_IDEAS {
			self.indice = 0;
		}

		if self.indice ==  {
			self.indice = NUM_IDEAS - 1;
		} else {
			self.indice = self.indice - 1;
		}

		self.room.idea = self.ideas[self.indice];

		self.room
	}

}