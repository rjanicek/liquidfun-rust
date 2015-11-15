
pub enum B2Shape {}

pub trait Shape {
	fn handle(&self) -> *mut B2Shape;
}