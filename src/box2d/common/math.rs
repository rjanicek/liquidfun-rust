use super::settings::*;

#[repr(C)]
#[derive(Debug, PartialEq, Default)]
pub struct Vec2 {
    pub x: Float32,
    pub y: Float32,
}

impl Vec2 {
	pub fn new(x: Float32, y: Float32) -> Vec2 {
		Vec2 {x: x, y: y}
	}

	pub fn set(&mut self, x: Float32, y: Float32) {
		self.x = x;
		self.y = y;
	}
}
