use super::body::*;
use super::super::common::math::*;
use super::super::common::settings::*;
use super::super::particle::particle_system::*;

pub enum B2World {}

extern {
    fn b2World_CreateBody(world: *mut B2World, bd: *const BodyDef) -> *mut B2Body;
    fn b2World_CreateParticleSystem(world: *mut B2World, def: *const ParticleSystemDef) -> *mut B2ParticleSystem;
    fn b2World_Delete(world: *mut B2World);
    fn b2World_GetBodyCount(world: *const B2World) -> Int32;
    fn b2World_GetBodyList(world: *const B2World) -> *mut B2Body;
    fn b2World_GetGravity(world: *mut B2World) -> Vec2;
    fn b2World_New(gravity: *const Vec2) -> *mut B2World;
    fn b2World_Step(this: *mut B2World, timeStep: Float32, velocityIterations: Int32, positionIterations: Int32);
}

pub struct World {
	pub ptr: *mut B2World
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

    /// Create a particle system given a definition. No reference to the
    /// definition is retained.
    /// @warning This function is locked during callbacks.
    pub fn create_particle_system(&self, def: &ParticleSystemDef) -> ParticleSystem {
        unsafe {
            ParticleSystem { ptr: b2World_CreateParticleSystem(self.ptr, def) }
        }
    }

    pub fn get_body_count(&self) -> Int32 {
        unsafe {
            b2World_GetBodyCount(self.ptr)
        }
    }

    pub fn get_body_list(&self) -> Option<Body> {
        let ptr;
        unsafe {
            ptr = b2World_GetBodyList(self.ptr);
        }

        match ptr.is_null() {
            true => None,
            false => Some(Body { ptr: ptr })
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