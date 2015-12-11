use super::super::super::box2d::dynamics::body::*;
use super::super::super::box2d::dynamics::world::*;

pub struct BodyIterator {
	body: Option<Body>,
	started: bool
}

impl World {
	/// Get a world body iterator.
	pub fn get_body_iterator(&mut self) -> BodyIterator {
		BodyIterator { body: self.get_body_list(), started: false }
	}
}

impl Iterator for BodyIterator {
	type Item = Body;
	fn next(&mut self) -> Option<Body> {
		if  !self.started {
			self.started = true;
		} else {
			self.body = match self.body {
				Some(ref x) => x.get_next(),
				None => None
			}
		}

		self.body.clone()
	}
}

