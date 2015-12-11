
#[repr(C)]
#[derive(Debug)]
pub enum Type
{
	e_circle = 0,
	e_edge = 1,
	e_polygon = 2,
	e_chain = 3,
	e_typeCount = 4
}

pub enum B2Shape {}

pub trait Shape {
	fn handle(&self) -> *mut B2Shape;
}
