extern crate liquidfun;

use liquidfun::box2d::common::math::*;
use liquidfun::box2d::dynamics::body::*;
use liquidfun::box2d::dynamics::world::*;

#[test]
fn body_user_data() {

	let mut world = World::default();
	let mut body_def = BodyDef::default();
	
	let user_data = Vec2::new(6.0, 66.0);

	body_def.user_data = &user_data as *const Vec2 as usize;

	let body = world.create_body(&body_def);

	let body_user_data = unsafe { &*(body.get_user_data() as *const Vec2) };

	println!("{:?} == {:?}", user_data, body_user_data);

	assert_eq!(&user_data, body_user_data);
}