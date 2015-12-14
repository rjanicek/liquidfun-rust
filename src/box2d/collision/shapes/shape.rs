//! A shape is used for collision detection. You can create a shape however you like.
//! Shapes used for simulation in b2World are created automatically when a b2Fixture
//! is created. Shapes may encapsulate a one or more child shapes.

#[repr(C)]
#[derive(Debug)]
pub enum Type
{
	Circle = 0,
	Edge = 1,
	Polygon = 2,
	Chain = 3,
	TypeCount = 4
}

pub enum B2Shape {}

pub trait Shape {
	fn handle(&self) -> *mut B2Shape;
}
