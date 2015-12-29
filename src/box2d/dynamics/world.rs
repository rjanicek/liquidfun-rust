use libc::size_t;
use std::ptr;
use super::body::*;
use super::super::common::math::*;
use super::super::common::settings::*;
use super::super::particle::particle_system::*;
use super::joints;

pub enum B2World {}

extern {
    fn b2World_CreateBody(world: *mut B2World, bd: *const BodyDef) -> *mut B2Body;
    fn b2World_CreateParticleSystem(world: *mut B2World, def: *const ParticleSystemDef) -> *mut B2ParticleSystem;
    fn b2World_Delete(world: *mut B2World);
    fn b2World_GetBodyCount(world: *const B2World) -> Int32;
    fn b2World_GetJointCount(world: *const B2World) -> Int32;
    fn b2World_GetBodyList(world: *const B2World) -> *mut B2Body;
    fn b2World_GetGravity(world: *mut B2World) -> Vec2;
    fn b2World_GetParticleSystemList(world: *const B2World) -> *mut B2ParticleSystem;
    fn b2World_New(gravity: *const Vec2) -> *mut B2World;
    fn b2World_Step(this: *mut B2World, timeStep: Float32, velocityIterations: Int32, positionIterations: Int32);

    fn b2World_CreateRevoluteJoint(
        world: *mut B2World,
        
        joint_type: joints::JointType,
        user_data: size_t,
        body_a: *mut B2Body,
        body_b: *mut B2Body,
        collide_connected: bool,

        local_anchor_a: Vec2,
        local_anchor_b: Vec2,
        reference_angle: Float32,
        enable_limit: bool,
        lower_angle: Float32,
        upper_angle: Float32,
        enable_motor: bool,
        motor_speed: Float32,
        max_motor_torque: Float32
    ) -> *mut joints::revolute_joint::B2RevoluteJoint;

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

    /// Create a revolute joint to constrain bodies together. No reference to the definition
    /// is retained. This may cause the connected bodies to cease colliding.
    /// @warning This function is locked during callbacks.
    pub fn create_revolute_joint(&mut self, def: &(joints::JointDef, joints::revolute_joint::RevoluteJointDef)) -> joints::revolute_joint::RevoluteJoint {
        unsafe {
            joints::revolute_joint::RevoluteJoint {ptr: b2World_CreateRevoluteJoint(
                self.ptr,
                def.0.joint_type,
                def.0.user_data,
                match def.0.body_a {
                    Some(ref b) =>b.ptr,
                    None => ptr::null_mut()
                },
                match def.0.body_b {
                    Some(ref b) =>b.ptr,
                    None => ptr::null_mut()
                },
                def.0.collide_connected,
                def.1.local_anchor_a,
                def.1.local_anchor_b,
                def.1.reference_angle,
                def.1.enable_limit,
                def.1.lower_angle,
                def.1.upper_angle,
                def.1.enable_motor,
                def.1.motor_speed,
                def.1.max_motor_torque
            )}
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

    /// Get the number of joints.
    pub fn get_joint_count(&self) -> i32 {
        unsafe {
            b2World_GetJointCount(self.ptr)
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