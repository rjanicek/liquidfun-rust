use super::super::super::common::math::*;
use super::super::super::common::settings::*;
use super::super::super::dynamics::body::Body;

use super::{JointType, JointDef};

/// Revolute joint definition. This requires defining an
/// anchor point where the bodies are joined. The definition
/// uses local anchor points so that the initial configuration
/// can violate the constraint slightly. You also need to
/// specify the initial relative angle for joint limits. This
/// helps when saving and loading a game.
/// The local anchor points are measured from the body's origin
/// rather than the center of mass because:
/// 1. you might not know where the center of mass will be.
/// 2. if you add/remove shapes from a body and recompute the mass,
///    the joints will be broken.
#[derive(Debug)]
pub struct RevoluteJointDef {

	/// Initialize the bodies, anchors, and reference angle using a world
	/// anchor point.
	// void Initialize(b2Body* bodyA, b2Body* bodyB, const b2Vec2& anchor);

	/// The local anchor point relative to bodyA's origin.
	pub local_anchor_a: Vec2,

	/// The local anchor point relative to bodyB's origin.
	pub local_anchor_b: Vec2,

	/// The bodyB angle minus bodyA angle in the reference state (radians).
	pub reference_angle: Float32,

	/// A flag to enable joint limits.
	pub enable_limit: bool,

	/// The lower angle for the joint limit (radians).
	pub lower_angle: Float32,

	/// The upper angle for the joint limit (radians).
	pub upper_angle: Float32,

	/// A flag to enable the joint motor.
	pub enable_motor: bool,

	/// The desired motor speed. Usually in radians per second.
	pub motor_speed: Float32,

	/// The maximum motor torque used to achieve the desired motor speed.
	/// Usually in N-m.
	pub max_motor_torque: Float32,	
}

impl RevoluteJointDef {
	pub fn new () -> (JointDef, RevoluteJointDef) {
		(
			JointDef { joint_type: JointType::RevoluteJoint, ..JointDef::default() }, 
	    	RevoluteJointDef {
				local_anchor_a: Vec2::zero(),
				local_anchor_b: Vec2::zero(),
				reference_angle: 0.0,
				lower_angle: 0.0,
				upper_angle: 0.0,
				max_motor_torque: 0.0,
				motor_speed: 0.0,
				enable_limit: false,
				enable_motor: false,
		    }
	    )
    }

    pub fn initialize(&mut self, joint: &mut JointDef, body_a: Body, body_b: Body, anchor: &Vec2) {
		self.local_anchor_a = body_a.get_local_point(anchor);
		self.local_anchor_b = body_b.get_local_point(anchor);
		self.reference_angle = body_b.get_angle() - body_a.get_angle();
		joint.body_a = Some(body_a);
		joint.body_b = Some(body_b);
    }
}

pub enum B2RevoluteJoint {}

extern {
    fn b2RevoluteJoint_SetMotorSpeed(this: *mut B2RevoluteJoint, speed: Float32);
}


/// A revolute joint constrains two bodies to share a common point while they
/// are free to rotate about the point. The relative rotation about the shared
/// point is the joint angle. You can limit the relative rotation with
/// a joint limit that specifies a lower and upper angle. You can use a motor
/// to drive the relative rotation about the shared point. A maximum motor torque
/// is provided so that infinite forces are not generated.
#[allow(raw_pointer_derive)]
#[derive(Clone, Debug)]
pub struct RevoluteJoint {
	pub ptr: *mut B2RevoluteJoint
}

impl RevoluteJoint {

	/// Set the motor speed in radians per second.
    pub fn set_motor_speed(&self, speed: f32) {
        unsafe {
            b2RevoluteJoint_SetMotorSpeed(self.ptr, speed);
        }
    }

}