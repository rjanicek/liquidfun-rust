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
    fn b2World_GetParticleSystemList(world: *const B2World) -> *mut B2ParticleSystem;
    fn b2World_New(gravity: *const Vec2) -> *mut B2World;
    fn b2World_Step(this: *mut B2World, timeStep: Float32, velocityIterations: Int32, positionIterations: Int32);
}

/// The world class manages all physics entities, dynamic simulation,
/// and asynchronous queries. The world also contains efficient memory
/// management facilities.
pub struct World {
	pub ptr: *mut B2World
}

impl World {

    /// Construct a world object.
    /// @param gravity the world gravity vector.
    pub fn new(gravity: &Vec2) -> World {
        unsafe {
            World { ptr: b2World_New(gravity) }
        }
    }

    /// Create a rigid body given a definition. No reference to the definition
    /// is retained.
    /// @warning This function is locked during callbacks.
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

    /// Get the number of bodies.
    pub fn get_body_count(&self) -> i32 {
        unsafe {
            b2World_GetBodyCount(self.ptr)
        }
    }

    /// Get the world body list. With the returned body, use b2Body::GetNext to get
    /// the next body in the world list. A NULL body indicates the end of the list.
    /// @return the head of the world body list.
    pub fn get_body_list(&self) -> Option<Body> {
        let ptr;
        unsafe {
            ptr = b2World_GetBodyList(self.ptr);
        }

        if ptr.is_null() {
            None
        } else {
            Some(Body { ptr: ptr })
        }
    }

    /// Get the world particle-system list. With the returned body, use
    /// b2ParticleSystem::GetNext to get the next particle-system in the world
    /// list. A NULL particle-system indicates the end of the list.
    /// @return the head of the world particle-system list.
    pub fn get_particle_system_list(&self) -> Option<ParticleSystem> {
        let ptr;
        unsafe {
            ptr = b2World_GetParticleSystemList(self.ptr);
        }

        if ptr.is_null() {
            None
        } else {
            Some(ParticleSystem { ptr: ptr })
        }
    }

    /// Get the global gravity vector.
    pub fn get_gravity(&mut self) -> Vec2 {
    	unsafe {
    		b2World_GetGravity(self.ptr)
    	}
    }

    /// Take a time step. This performs collision detection, integration,
    /// and constraint solution.
    /// @param timeStep the amount of time to simulate, this should not vary.
    /// @param velocityIterations for the velocity constraint solver.
    /// @param positionIterations for the position constraint solver.
    pub fn step(&mut self, time_step: f32, velocity_iterations: i32, position_iterations: i32) {
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