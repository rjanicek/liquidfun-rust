pub mod particle_color;
pub mod particle_group;
pub mod particle_system;

use libc::size_t;
use self::particle_color::*;
use self::particle_group::*;
use std::ptr;
use super::common::math::*;
use super::common::settings::*;

/// The particle type. Can be combined with the | operator.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ParticleFlag {
	/// Water particle.
	WaterParticle = 0,
	/// Removed after next simulation step.
	ZombieParticle = 1 << 1,
	/// Zero velocity.
	WallParticle = 1 << 2,
	/// With restitution from stretching.
	SpringParticle = 1 << 3,
	/// With restitution from deformation.
	ElasticParticle = 1 << 4,
	/// With viscosity.
	ViscousParticle = 1 << 5,
	/// Without isotropic pressure.
	PowderParticle = 1 << 6,
	/// With surface tension.
	TensileParticle = 1 << 7,
	/// Mix color between contacting particles.
	ColorMixingParticle = 1 << 8,
	/// Call b2DestructionListener on destruction.
	DestructionListenerParticle = 1 << 9,
	/// Prevents other particles from leaking.
	BarrierParticle = 1 << 10,
	/// Less compressibility.
	StaticPressureParticle = 1 << 11,
	/// Makes pairs or triads with other particles.
	ReactiveParticle = 1 << 12,
	/// With high repulsive force.
	RepulsiveParticle = 1 << 13,
	/// Call b2ContactListener when this particle is about to interact with
	/// a rigid body or stops interacting with a rigid body.
	/// This results in an expensive operation compared to using
	/// b2_fixtureContactFilterParticle to detect collisions between
	/// particles.
	FixtureContactListenerParticle = 1 << 14,
	/// Call b2ContactListener when this particle is about to interact with
	/// another particle or stops interacting with another particle.
	/// This results in an expensive operation compared to using
	/// b2_particleContactFilterParticle to detect collisions between
	/// particles.
	ParticleContactListenerParticle = 1 << 15,
	/// Call b2ContactFilter when this particle interacts with rigid bodies.
	FixtureContactFilterParticle = 1 << 16,
	/// Call b2ContactFilter when this particle interacts with other
	/// particles.
	ParticleContactFilterParticle = 1 << 17,
}

#[repr(C)]
#[derive(Debug)]
pub struct B2ParticleDef {
 	flags: UInt32,
	position: Vec2,
	velocity: Vec2,
 	color: *mut B2ParticleColor,
	lifetime: Float32,
	user_data: size_t,
	group: *mut B2ParticleGroup,
}

/// A particle definition holds all the data needed to construct a particle.
/// You can safely re-use these definitions.
pub struct ParticleDef {
	/// \brief Specifies the type of particle (see #b2ParticleFlag).
	///
	/// A particle may be more than one type.
	/// Multiple types are chained by logical sums, for example:
	/// pd.flags = b2_elasticParticle | b2_viscousParticle
 	pub flags: Vec<ParticleFlag>,

	/// The world position of the particle.
	pub position: Vec2,

	/// The linear velocity of the particle in world co-ordinates.
	pub velocity: Vec2,

	/// The color of the particle.
 	pub color: ParticleColor,

	/// Lifetime of the particle in seconds.  A value <= 0.0f indicates a
	/// particle with infinite lifetime.
	pub lifetime: Float32,

	/// Use this to store application-specific body data.
	pub user_data: size_t,

	// An existing particle group to which the particle will be added.
	pub group: Option<ParticleGroup>,
}

impl Default for ParticleDef {
    fn default () -> ParticleDef {
        ParticleDef {
			flags: Vec::new(),
			position: Vec2::zero(),
			velocity: Vec2::zero(),
			color: ParticleColor::zero(),
			lifetime: 0.0,
			user_data: 0,
			group: None,
        }
    }
}

impl B2ParticleDef {
	fn from(pd: &ParticleDef) -> B2ParticleDef {
		B2ParticleDef {
		 	flags: pd.flags.iter().fold(0, |acc, flag| { acc | *flag as UInt32 }),
			position: pd.position.clone(),
			velocity: pd.velocity.clone(),
		 	color: pd.color.ptr(),
			lifetime: pd.lifetime,
			user_data: pd.user_data,
			group: match pd.group {
				Some(ref g) => g.ptr(),
				None => ptr::null_mut()
			}
		}
	}
}