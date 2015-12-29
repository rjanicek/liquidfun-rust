extern crate liquidfun;

use liquidfun::box2d::dynamics::world;
use liquidfun::box2d::particle::*;
use liquidfun::box2d::particle::particle_system::*;
use liquidfun::box2d::common::math::*;

#[test]
fn create_a_zero_particle_color() {
	let pc = particle_color::ParticleColor::zero();
	assert!(pc.is_zero());
}

#[test]
fn create_and_destroy_a_particle() {
	let world = world::World::default();
	let particle_system_def = ParticleSystemDef::default();
	let particle_system = world.create_particle_system(&particle_system_def);

	let pd = ParticleDef::default();
	assert_eq!(particle_system.get_particle_count(), 0);
	let temp_index = particle_system.create_particle(&pd);
	assert_eq!(particle_system.get_particle_count(), 1);
	particle_system.destroy_particle(temp_index);
}

#[test]
fn set_and_get_particle_flags() {
   let world = world::World::default();
   let particle_system_def = ParticleSystemDef::default();
   let particle_system = world.create_particle_system(&particle_system_def);

   let mut pd = ParticleDef::default();
   pd.flags = ZOMBIE_PARTICLE;
   let particle_index = particle_system.create_particle(&pd);
   let flags = particle_system.get_particle_flags(particle_index);
   assert!(flags.is_some());
   assert_eq!(flags.unwrap(), ZOMBIE_PARTICLE);
}

#[test]
fn get_position_buffer() {
	let world = world::World::default();
	let particle_system_def = ParticleSystemDef::default();
	let particle_system = world.create_particle_system(&particle_system_def);

   	let mut pd = ParticleDef::default();
   	pd.position.set(6.0, 6.0);
   	particle_system.create_particle(&pd);
   	let position_buffer = particle_system.get_position_buffer();
      assert_eq!(position_buffer.len(), 1);
   	assert_eq!(position_buffer[0], Vec2 { x: 6.0, y: 6.0});
}