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
#[derive(Debug)]
pub enum ParticleFlag {
	/// Water particle.
	b2_waterParticle = 0,
	/// Removed after next simulation step.
	b2_zombieParticle = 1 << 1,
	/// Zero velocity.
	b2_wallParticle = 1 << 2,
	/// With restitution from stretching.
	b2_springParticle = 1 << 3,
	/// With restitution from deformation.
	b2_elasticParticle = 1 << 4,
	/// With viscosity.
	b2_viscousParticle = 1 << 5,
	/// Without isotropic pressure.
	b2_powderParticle = 1 << 6,
	/// With surface tension.
	b2_tensileParticle = 1 << 7,
	/// Mix color between contacting particles.
	b2_colorMixingParticle = 1 << 8,
	/// Call b2DestructionListener on destruction.
	b2_destructionListenerParticle = 1 << 9,
	/// Prevents other particles from leaking.
	b2_barrierParticle = 1 << 10,
	/// Less compressibility.
	b2_staticPressureParticle = 1 << 11,
	/// Makes pairs or triads with other particles.
	b2_reactiveParticle = 1 << 12,
	/// With high repulsive force.
	b2_repulsiveParticle = 1 << 13,
	/// Call b2ContactListener when this particle is about to interact with
	/// a rigid body or stops interacting with a rigid body.
	/// This results in an expensive operation compared to using
	/// b2_fixtureContactFilterParticle to detect collisions between
	/// particles.
	b2_fixtureContactListenerParticle = 1 << 14,
	/// Call b2ContactListener when this particle is about to interact with
	/// another particle or stops interacting with another particle.
	/// This results in an expensive operation compared to using
	/// b2_particleContactFilterParticle to detect collisions between
	/// particles.
	b2_particleContactListenerParticle = 1 << 15,
	/// Call b2ContactFilter when this particle interacts with rigid bodies.
	b2_fixtureContactFilterParticle = 1 << 16,
	/// Call b2ContactFilter when this particle interacts with other
	/// particles.
	b2_particleContactFilterParticle = 1 << 17,
}

/// A particle definition holds all the data needed to construct a particle.
/// You can safely re-use these definitions.
#[repr(C)]
#[derive(Debug)]
pub struct ParticleDef {
	/// \brief Specifies the type of particle (see #b2ParticleFlag).
	///
	/// A particle may be more than one type.
	/// Multiple types are chained by logical sums, for example:
	/// pd.flags = b2_elasticParticle | b2_viscousParticle
 	flags: UInt32,

	/// The world position of the particle.
	position: Vec2,

	/// The linear velocity of the particle in world co-ordinates.
	velocity: Vec2,

	/// The color of the particle.
 	color: *mut B2ParticleColor,

	/// Lifetime of the particle in seconds.  A value <= 0.0f indicates a
	/// particle with infinite lifetime.
	lifetime: Float32,

	/// Use this to store application-specific body data.
	user_data: size_t,

	// An existing particle group to which the particle will be added.
	group: *mut B2ParticleGroup,
}

impl Default for ParticleDef {
    fn default () -> ParticleDef {
        ParticleDef {
			flags: 0,
			position: Vec2::zero(),
			velocity: Vec2::zero(),
			color: particle_color::zero(),
			lifetime: 0.0,
			user_data: 0,
			group: ptr::null_mut(),
        }
    }
}