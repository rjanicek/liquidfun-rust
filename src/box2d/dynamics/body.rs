use libc::size_t;
use std::mem::transmute;
use super::fixture::*;
use super::super::collision::shapes::shape::*;
use super::super::common::math::*;
use super::super::common::settings::*;
use super::world::*;

/// The body type.
/// static: zero mass, zero velocity, may be manually moved
/// kinematic: zero mass, non-zero velocity set by user, moved by solver
/// dynamic: positive mass, non-zero velocity determined by forces, moved by solver
#[repr(C)]
#[derive(Debug)]
pub enum BodyType {
	StaticBody = 0,
	KinematicBody,
	DynamicBody
}

/// A body definition holds all the data needed to construct a rigid body.
/// You can safely re-use body definitions. Shapes are added to a body after construction.
#[repr(C)]
#[derive(Debug)]
pub struct BodyDef {

    /// The body type: static, kinematic, or dynamic.
    /// Note: if a dynamic body would have zero mass, the mass is set to one.
    pub body_type: BodyType,

    /// The world position of the body. Avoid creating bodies at the origin
    /// since this can lead to many overlapping shapes.
    pub position: Vec2,

    /// The world angle of the body in radians.
    pub angle: Float32,

    /// The linear velocity of the body's origin in world co-ordinates.    
    pub linear_velocity: Vec2,

    /// The angular velocity of the body.    
    pub angular_velocity: Float32,

    /// Linear damping is use to reduce the linear velocity. The damping parameter
    /// can be larger than 1.0f but the damping effect becomes sensitive to the
    /// time step when the damping parameter is large.
    pub linear_damping: Float32,


    /// Angular damping is use to reduce the angular velocity. The damping parameter
    /// can be larger than 1.0f but the damping effect becomes sensitive to the
    /// time step when the damping parameter is large.
    pub angular_damping: Float32,

    /// Set this flag to false if this body should never fall asleep. Note that
    /// this increases CPU usage.
    pub allow_sleep: bool,

    /// Is this body initially awake or sleeping?
    pub awake: bool,

    /// Should this body be prevented from rotating? Useful for characters.
    pub fixed_rotation: bool,

    /// Is this a fast moving body that should be prevented from tunneling through
    /// other moving bodies? Note that all bodies are prevented from tunneling through
    /// kinematic and static bodies. This setting is only considered on dynamic bodies.
    /// @warning You should use this flag sparingly since it increases processing time.
    pub bullet: bool,

    /// Does this body start out active?
    pub active: bool,

    /// Use this to store application specific body data.
    pub user_data: size_t,

    /// Scale the gravity applied to this body.
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
	        body_type: BodyType::StaticBody,
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
    fn b2Body_GetFixtureList(this: *mut B2Body) -> *mut B2Fixture;
    fn b2Body_GetNext(this: *mut B2Body) -> *mut B2Body;
    fn b2Body_GetPosition(this: *mut B2Body) -> &Vec2;
    fn b2Body_GetUserData(this: *const B2Body) -> usize;
    fn b2Body_GetWorld(this: *const B2Body) -> *mut B2World; 
}

/// A rigid body. These are created via b2World::CreateBody.
#[allow(raw_pointer_derive)]
#[derive(Clone)]
pub struct Body {
	pub ptr: *mut B2Body
}

impl Body {

    /// Creates a fixture and attach it to this body. Use this function if you need
    /// to set some fixture parameters, like friction. Otherwise you can create the
    /// fixture directly from a shape.
    /// If the density is non-zero, this function automatically updates the mass of the body.
    /// Contacts are not created until the next time step.
    /// @param def the fixture definition.
    /// @warning This function is locked during callbacks.
    pub fn create_fixture(&self, def: &FixtureDef) -> Fixture {
        unsafe {
            Fixture { ptr: b2Body_CreateFixture(self.ptr, transmute(def)) }
        }
    }

    /// Creates a fixture from a shape and attach it to this body.
    /// This is a convenience function. Use FixtureDef if you need to set parameters
    /// like friction, restitution, user data, or filtering.
    /// If the density is non-zero, this function automatically updates the mass of the body.
    /// @param shape the shape to be cloned.
    /// @param density the shape density (set to zero for static bodies).
    /// @warning This function is locked during callbacks.
    pub fn create_fixture_from_shape(&self, shape: &Shape, density: f32) -> Fixture {
        unsafe {
            Fixture { ptr: b2Body_CreateFixture_FromShape(self.ptr, shape.handle(), density) }
        }
    }

    /// Get the angle in radians.
    /// @return the current world rotation angle in radians.
    pub fn get_angle(&self) -> f32 {
        unsafe {
            b2Body_GetAngle(self.ptr)
        }
    }

    /// Get the list of all fixtures attached to this body.
    pub fn get_fixture_list(&self) -> Option<Fixture> {
        let ptr;
        unsafe {
            ptr = b2Body_GetFixtureList(self.ptr);
        }

        if ptr.is_null() {
            None
        } else {
            Some(Fixture { ptr: ptr })
        }
    }    

    /// Get the next body in the world's body list.
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

    /// Get the world body origin position.
    /// @return the world position of the body's origin.
    pub fn get_position(&self) -> &Vec2 {
        unsafe {
            b2Body_GetPosition(self.ptr)
        }
    }

    /// Get the user data pointer that was provided in the body definition.
    pub fn get_user_data(&self) -> usize {
        unsafe {
            b2Body_GetUserData(self.ptr)
        }
    }

    /// Get the parent world of this body.
    pub fn get_world(&self) -> World {
        unsafe {
            World { ptr: b2Body_GetWorld(self.ptr) }
        }
    }

}
