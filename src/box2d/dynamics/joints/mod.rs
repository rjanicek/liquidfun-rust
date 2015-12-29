use libc::size_t;
use super::super::dynamics::body::{Body};

pub mod revolute_joint;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum JointType {
	UnknownJoint = 0,
	RevoluteJoint,
	PrismaticJoint,
	DistanceJoint,
	PulleyJoint,
	MouseJoint,
	GearJoint,
	WheelJoint,
    WeldJoint,
	FrictionJoint,
	RopeJoint,
	MotorJoint
}

/// Joint definitions are used to construct joints.
#[derive(Debug)]
pub struct JointDef {
	pub joint_type: JointType,

	/// Use this to attach application specific data to your joints.
	pub user_data: size_t,

	/// The first attached body.
	pub body_a: Option<Body>,

	/// The second attached body.
	pub body_b: Option<Body>,

	/// Set this flag to true if the attached bodies should collide.
	pub collide_connected: bool,
}

impl Default for JointDef {
	fn default() -> JointDef {
		JointDef {
			joint_type: JointType::UnknownJoint,
			user_data: 0,
			body_a: None,
			body_b: None,
			collide_connected: false,
		}
	}
}