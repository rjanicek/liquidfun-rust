extern crate liquidfun;

use liquidfun::box2d::dynamics::world;
use liquidfun::box2d::particle::ParticleDef;
use liquidfun::box2d::particle::particle_system::*;

#[test]
fn create_a_particle_system() {
	let mut world = world::World::default();
	let particle_system_def = ParticleSystemDef::default();
	let particle_system = world.create_particle_system(&particle_system_def);
}

#[test]
fn create_and_destroy_a_particle() {
	let mut world = world::World::default();
	let particle_system_def = ParticleSystemDef::default();
	let particle_system = world.create_particle_system(&particle_system_def);

   	let pd = ParticleDef::default();

   // pd.flags = b2_elasticParticle;
   // pd.color.Set(0, 0, 255, 255);
   // pd.position.Set(i, 0);
   // int tempIndex = m_particleSystem->CreateParticle(pd);
   // m_particleSystem->DestroyParticle(tempIndex);	
}