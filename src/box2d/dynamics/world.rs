use super::body::*;
use super::super::common::math::*;
use super::super::common::settings::*;

enum B2World {}

extern {
    fn b2World_CreateBody(world: *mut B2World, bd: *const BodyDef) -> *mut B2Body;
    fn b2World_Delete(world: *mut B2World);
    fn b2World_GetGravity(world: *mut B2World) -> Vec2;
    fn b2World_New(gravity: *const Vec2) -> *mut B2World;
    fn b2World_Step(this: *mut B2World, timeStep: Float32, velocityIterations: Int32, positionIterations: Int32);
}

pub struct World {
	ptr: *mut B2World
}

impl World {
    pub fn new(gravity: &Vec2) -> World {
        unsafe {
            World { ptr: b2World_New(gravity) }
        }
    }

    pub fn create_body(&mut self, def: &BodyDef) -> Body {
        unsafe {
            Body { ptr: b2World_CreateBody(self.ptr, def) }
        }
    }

    pub fn get_gravity(&mut self) -> Vec2 {
    	unsafe {
    		b2World_GetGravity(self.ptr)
    	}
    }

    pub fn step(&mut self, time_step: Float32, velocity_iterations: Int32, position_iterations: Int32) {
        unsafe {
            b2World_Step(self.ptr, time_step, velocity_iterations, position_iterations);
        }
    }    

}

impl Drop for World {
    fn drop(&mut self) {
        unsafe {
            b2World_Delete(self.ptr);
        }
    }
}