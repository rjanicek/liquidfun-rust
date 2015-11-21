use super::super::box2d::common::math::*;
use super::super::box2d::dynamics::world::*;

impl Default for World {
	fn default() -> World {
		let gravity = Vec2::new(0.0, -10.0);
		World::new(&gravity)
	}
}