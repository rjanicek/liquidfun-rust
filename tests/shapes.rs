extern crate liquidfun;

use liquidfun::box2d::collision::shapes::chain_shape::*;
use liquidfun::box2d::common::math::*;

#[test]
fn create_chain() {

	// This a chain shape with isolated vertices

	let vs = [
		Vec2::new(1.7, 0.0),
		Vec2::new(1.0, 0.25),
		Vec2::new(0.0, 0.0),
		Vec2::new(-1.7, 0.4),
	];

	let mut chain = ChainShape::new();
	chain.create_chain(&vs, vs.len() as i32);
	assert_eq!(chain.get_vertex_count(), 4);

	let vertices = chain.get_vertices();
	assert_eq!(vertices.len(), 4);
	assert_eq!(vertices, vs);
}