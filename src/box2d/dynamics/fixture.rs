use libc::size_t;
use super::super::collision::shapes::shape;
use super::super::common::settings::*;

#[repr(C)]
#[derive(Debug)]
pub struct Filter {
    pub category_bits: UInt16,
    pub mask_bits: UInt16,
    pub group_index: Int16,
}

impl Default for Filter {
	fn default() -> Filter {
		Filter {
            category_bits: 0x0001,
            mask_bits: 0xFFFF,
            group_index: 0
        }		
	}
}

#[repr(C)]
pub struct FixtureDef {
    pub shape: *mut shape::B2Shape,
    pub user_data: size_t,
    pub friction: Float32,
    pub restitution: Float32,
    pub density: Float32,
    pub is_sensor: bool,
    pub filter: Filter
}

impl FixtureDef {
    pub fn new(shape: &shape::Shape) -> FixtureDef {
        FixtureDef {
            shape: shape.handle(),
            user_data: 0,
            friction: 0.0,
            restitution: 0.0,
            density: 0.0,
            is_sensor: false,
            filter: Filter::default()
        }
    }
}

pub enum B2Fixture {}

extern {
    fn b2Fixture_GetNext(this: *mut B2Fixture) -> *mut B2Fixture;
    fn b2Fixture_GetShape(this: *mut B2Fixture) -> *mut shape::B2Shape;
    fn b2Fixture_GetType(this: *mut B2Fixture) -> shape::Type;
}

#[allow(raw_pointer_derive)]
#[derive(Clone)]
pub struct Fixture {
    pub ptr: *mut B2Fixture
}

impl Fixture {

    pub fn get_next(&self) -> Option<Fixture> {
        let ptr: *mut B2Fixture;
        
        unsafe {
            ptr = b2Fixture_GetNext(self.ptr);
        }

        if ptr.is_null() {
            None
        } else {
            Some(Fixture { ptr: ptr })
        }        
    }

    pub fn get_shape(&self) -> *mut shape::B2Shape {
        unsafe {
            return b2Fixture_GetShape(self.ptr);
        }
    }    

    pub fn get_type(&self) -> shape::Type {
        unsafe {
            b2Fixture_GetType(self.ptr)
        }
    }

}
