use libc::size_t;
use std::mem::transmute;
use super::fixture::*;
use super::super::collision::shapes::shape::*;
use super::super::common::math::*;
use super::super::common::settings::*;
use super::world::*;

#[repr(C)]
#[derive(Debug)]
pub enum BodyType
{
	b2_staticBody = 0,
	b2_kinematicBody,
	b2_dynamicBody
}

#[repr(C)]
#[derive(Debug)]
pub struct BodyDef {
    pub body_type: BodyType,
    pub position: Vec2,
    pub angle: Float32,
    pub linear_velocity: Vec2,
    pub angular_velocity: Float32,
    pub linear_damping: Float32,
    pub angular_damping: Float32,
    pub allow_sleep: bool,
    pub awake: bool,
    pub fixed_rotation: bool,
    pub bullet: bool,
    pub active: bool,

    // usage:
    // 	move raw pointer in: &user_data as *const T as size_t;
    // 	dereference raw pointer out: *(user_data as *const T)
    pub user_data: size_t,

    pub gravity_scale : Float32,
}

impl Default for BodyDef {
	fn default () -> BodyDef {
    	BodyDef {
	        user_data: 0,
	        position: Vec2::default(),
	        angle: 0.0,
	        linear_velocity: Vec2::default(),
	        angular_velocity: 0.0,
	        linear_damping: 0.0,
	        angular_damping: 0.0,
	        allow_sleep: true,
	        awake: true,
	        fixed_rotation: false,
	        bullet: false,
	        body_type: BodyType::b2_staticBody,
	        active: true,
	        gravity_scale: 1.0,
	    }
  }
}

pub enum B2Body {}

extern {
    fn b2Body_CreateFixture_FromShape(this: *mut B2Body, shape: *const B2Shape, density: Float32) -> *mut B2Fixture;
    fn b2Body_CreateFixture(this: *mut B2Body, def: *mut FixtureDef) -> *mut B2Fixture;
    fn b2Body_GetAngle(this: *mut B2Body) -> Float32;
    fn b2Body_GetNext(this: *mut B2Body) -> *mut B2Body;
    fn b2Body_GetPosition(this: *mut B2Body) -> &Vec2;
    fn b2Body_GetUserData(this: *const B2Body) -> usize;
    fn b2Body_GetWorld(this: *const B2Body) -> *mut B2World; 
}

#[allow(raw_pointer_derive)]
#[derive(Clone)]
pub struct Body {
	pub ptr: *mut B2Body
}

impl Body {

    pub fn create_fixture(&self, def: &FixtureDef) -> Fixture {
        unsafe {
            Fixture { ptr: b2Body_CreateFixture(self.ptr, transmute(def)) }
        }
    }

    pub fn create_fixture_from_shape(&self, shape: &Shape, density: Float32) -> Fixture {
        unsafe {
            Fixture { ptr: b2Body_CreateFixture_FromShape(self.ptr, shape.handle(), density) }
        }
    }

    pub fn get_angle(&self) -> Float32 {
        unsafe {
            b2Body_GetAngle(self.ptr)
        }
    }

    pub fn get_next(&self) -> Option<Body> {
        let ptr: *mut B2Body;
        
        unsafe {
            ptr = b2Body_GetNext(self.ptr);
        }

        if ptr.is_null() {
            None
        } else {
            Some(Body { ptr: ptr })
        }        
    }

    pub fn get_position(&self) -> &Vec2 {
        unsafe {
            b2Body_GetPosition(self.ptr)
        }
    }

    pub fn get_user_data(&self) -> usize {
        unsafe {
            b2Body_GetUserData(self.ptr)
        }
    }

    pub fn get_world(&self) -> World {
        unsafe {
            World { ptr: b2Body_GetWorld(self.ptr) }
        }
    }

}
