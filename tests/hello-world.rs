extern crate libc;
extern crate liquidfun;

use liquidfun::box2d::collision::shapes::polygon_shape::*;
use liquidfun::box2d::common::math::*;
use liquidfun::box2d::dynamics::body::*;
use liquidfun::box2d::dynamics::fixture::*;
use liquidfun::box2d::dynamics::world::*;

#[test]
fn hello_world() {

	// Define the gravity vector.
	let gravity = Vec2::new(0.0, -10.0);

	// Construct a world object, which will hold and simulate the rigid bodies.
	let mut world = World::new(&gravity);

	// Define the ground body.
	let mut ground_body_def = BodyDef::default();
	ground_body_def.position.set(0.0, -10.0);

	// Call the body factory which allocates memory for the ground body
	// from a pool and creates the ground box shape (also from a pool).
	// The body is also added to the world.
	let ground_body = world.create_body(&ground_body_def);

	// Define the ground box shape.
	let mut ground_box = PolygonShape::default();

	// The extents are the half-widths of the box.
	ground_box.set_as_box(50.0, 10.0);

	// Add the ground fixture to the ground body.
	ground_body.create_fixture_from_shape(&ground_box, 0.0);

	// Define the dynamic body. We set its position and call the body factory.
	let mut body_def = BodyDef::default();
	body_def.body_type = BodyType::b2_dynamicBody;
	body_def.position.set(0.0, 4.0);
	let body = world.create_body(&body_def);

	// Define another box shape for our dynamic body.
	let mut dynamic_box = PolygonShape::default();
	dynamic_box.set_as_box(1.0, 1.0);

	// Define the dynamic body fixture.
	let mut fixture_def = FixtureDef::new(&dynamic_box);

	// Set the box density to be non-zero, so it will be dynamic.
	fixture_def.density = 1.0;

	// Override the default friction.
	fixture_def.friction = 0.3;

	// Add the shape to the body.
	body.create_fixture(&fixture_def);

	// Prepare for simulation. Typically we use a time step of 1/60 of a
	// second (60Hz) and 10 iterations. This provides a high quality simulation
	// in most game scenarios.
	let time_step = 1.0 / 60.0;
	let velocity_iterations = 6;
	let position_iterations = 2;

	// This is our little game loop.
	for _ in 0..60 {

		// Instruct the world to perform a single step of simulation.
		// It is generally best to keep the time step and iterations fixed.
		world.step(time_step, velocity_iterations, position_iterations);

		// Now print the position and angle of the body.
		let position = body.get_position();
		let angle = body.get_angle();

		println!("{:?} angle: {:?}", position, angle);
	}

	// When the world destructor is called, all bodies and joints are freed. This can
	// create orphaned pointers, so be careful about your world management.	

	// assert!(false);
}