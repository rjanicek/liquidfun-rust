use super::super::super::super::box2d::collision::shapes::polygon_shape::*;
use super::super::super::super::box2d::common::math::*;
use super::super::super::super::box2d::common::settings::*;

pub struct VertexIterator<'a> {
	poly: &'a mut PolygonShape,
	vertex_count: Int32,
	index: Int32
}

impl PolygonShape {
	/// Get a vertex iterator.
	pub fn get_vertex_iterator(&mut self) -> VertexIterator {
		let vc = self.get_vertex_count();
		VertexIterator { poly: self, vertex_count: vc, index: 0 }
	}
}

impl<'a> Iterator for VertexIterator<'a> {
	type Item = Vec2;
	fn next(&mut self) -> Option<Vec2> {
		if self.index < self.vertex_count {
			self.index += 1;
			Some(self.poly.get_vertex(self.index - 1).clone())
		} else {
			None
		}
	}
}

// stab at non cloning version
// pub struct VertexIterator<'a> {
// 	poly: &'a mut PolygonShape,
// 	vertex_count: Int32,
// 	index: Int32
// }

// impl PolygonShape {
// 	/// Get a vertex iterator.
// 	pub fn get_vertex_iterator(&mut self) -> VertexIterator {
// 		let vc = self.get_vertex_count();
// 		VertexIterator { poly: self, vertex_count: vc, index: 0 }
// 	}
// }

// impl<'a> Iterator for VertexIterator<'a> {
// 	type Item = &'a Vec2;
// 	fn next(&mut self) -> Option<&'a Vec2> {
// 		if self.index < self.vertex_count {
// 			self.index += 1;
// 			Some(self.poly.get_vertex(self.index - 1))
// 		} else {
// 			None
// 		}
// 	}
// }
