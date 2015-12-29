extern crate liquidfun;

use liquidfun::box2d::collision::shapes::polygon_shape::*;
use liquidfun::box2d::dynamics::body::*;
use liquidfun::box2d::dynamics::joints::*;
use liquidfun::box2d::dynamics::world::World;

#[test]
fn create_revolute_joint() {

	let mut world = World::default();

	let bd = BodyDef::default();
	let body_a = world.create_body(&bd);
	let body_b = world.create_body(&bd);

	let mut shape = PolygonShape::new();
	shape.set_as_box(1.0, 1.0);
	body_a.create_fixture_from_shape(&shape, 0.0);
	body_b.create_fixture_from_shape(&shape, 0.0);

	let mut jd = revolute_joint::RevoluteJointDef::new();
	jd.0.body_a = Some(body_a);
	jd.0.body_b = Some(body_b);
	jd.1.local_anchor_a.set(0.0, 0.0);
	jd.1.local_anchor_b.set(0.0, 0.0);
	jd.1.reference_angle = 0.0;
	jd.1.motor_speed = 1.0;
	jd.1.max_motor_torque = 1e7;
	jd.1.enable_motor = true;
	let joint = world.create_revolute_joint(&jd);

	assert_eq!(world.get_joint_count(), 1);
}