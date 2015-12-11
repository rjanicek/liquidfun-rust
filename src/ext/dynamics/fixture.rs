use super::super::super::box2d::dynamics::fixture::*;
use super::super::super::box2d::dynamics::body::*;

pub struct FixtureIterator {
	fixture: Option<Fixture>,
	started: bool
}

impl Body {
	pub fn get_fixture_iterator(&mut self) -> FixtureIterator {
		FixtureIterator { fixture: self.get_fixture_list(), started: false }
	}
}

impl Iterator for FixtureIterator {
	type Item = Fixture;
	fn next(&mut self) -> Option<Fixture> {
		if  !self.started {
			self.started = true;
		} else {
			self.fixture = match self.fixture {
				Some(ref x) => x.get_next(),
				None => None
			}
		}

		self.fixture.clone()
	}
}