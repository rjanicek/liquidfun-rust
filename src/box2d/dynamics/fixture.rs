use libc::size_t;
use super::super::collision::shapes::shape::*;
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
    pub shape: *mut B2Shape,
    pub user_data: size_t,
    pub friction: Float32,
    pub restitution: Float32,
    pub density: Float32,
    pub is_sensor: bool,
    pub filter: Filter
}

impl FixtureDef {
    pub fn new(shape: &Shape) -> FixtureDef {
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

pub struct Fixture {
    pub ptr: *mut B2Fixture
}