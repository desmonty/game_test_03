use std::slice::Iter;

/*
	Idea enum.
	Used to know which room we are going to see.
*/
pub const NUM_IDEAS: usize = 5;
#[derive(Debug)]
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
#[derive(Debug)]
pub enum EmotionalContext {
	Reality,
	Love,
	Logic,
}