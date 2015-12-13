extern crate liquidfun;

use liquidfun::box2d::dynamics::world;
use liquidfun::box2d::particle::*;
use liquidfun::box2d::particle::particle_system::*;

#[test]
fn create_a_zero_particle_color() {
	let pc = particle_color::ParticleColor::zero();
	assert!(pc.is_zero());
}

#[test]
fn create_and_destroy_a_particle() {
	let mut world = world::World::default();
	let particle_system_def = ParticleSystemDef::default();
	let particle_system = world.create_particle_system(&particle_system_def);

   	let mut pd = ParticleDef::default();
   	pd.flags = vec![ParticleFlag::ElasticParticle];
   	pd.color.set(0, 0, 255, 256);
   	pd.position.set(10.0, 0.0);
   	assert_eq!(particle_system.get_particle_count(), 0);
   	let temp_index = particle_system.create_particle(&pd);
   	assert_eq!(particle_system.get_particle_count(), 1);
   	particle_system.destroy_particle(temp_index);
}