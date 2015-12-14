pub mod particle_color;
pub mod particle_group;
pub mod particle_system;

use libc::size_t;
use self::particle_color::*;
use self::particle_group::*;
use std::ptr;
use super::common::math::*;
use super::common::settings::*;

bitflags! {
	/// The particle type. Can be combined with the | operator.
    flags ParticleFlags: UInt32 {
		/// Water particle.
		const WATER_PARTICLE = 0,

		/// Removed after next simulation step.
		const ZOMBIE_PARTICLE = 1 << 1,

		/// Zero velocity.
		const WALL_PARTICLE = 1 << 2,
		/// With restitution from stretching.
		const SPRING_PARTICLE = 1 << 3,
		/// With restitution from deformation.
		const ELASTIC_PARTICLE = 1 << 4,
		/// With viscosity.
		const VISCOUS_PARTICLE = 1 << 5,
		/// Without isotropic pressure.
		const POWDER_PARTICLE = 1 << 6,
		/// With surface tension.
		const TENSILE_PARTICLE = 1 << 7,
		/// Mix color between contacting particles.
		const COLOR_MIXING_PARTICLE = 1 << 8,
		/// Call b2DestructionListener on destruction.
		const DESTRUCTION_LISTENER_PARTICLE = 1 << 9,
		/// Prevents other particles from leaking.
		const BARRIER_PARTICLE = 1 << 10,
		/// Less compressibility.
		const STATIC_PRESSURE_PARTICLE = 1 << 11,
		/// Makes pairs or triads with other particles.
		const REACTIVE_PARTICLE = 1 << 12,
		/// With high repulsive force.
		const REPULSIVE_PARTICLE = 1 << 13,
		/// Call b2ContactListener when this particle is about to interact with
		/// a rigid body or stops interacting with a rigid body.
		/// This results in an expensive operation compared to using
		/// b2_fixtureContactFilterParticle to detect collisions between
		/// particles.
		const FIXTURE_CONTACT_LISTENER_PARTICLE = 1 << 14,
		/// Call b2ContactListener when this particle is about to interact with
		/// another particle or stops interacting with another particle.
		/// This results in an expensive operation compared to using
		/// b2_particleContactFilterParticle to detect collisions between
		/// particles.
		const PARTICLE_CONTACT_LISTENER_PARTICLE = 1 << 15,
		/// Call b2ContactFilter when this particle interacts with rigid bodies.
		const FIXTURE_CONTACT_FILTER_PARTICLE = 1 << 16,
		/// Call b2ContactFilter when this particle interacts with other
		/// particles.
		const PARTICLE_CONTACT_FILTER_PARTICLE = 1 << 17,
    }
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
 	pub flags: ParticleFlags,

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
			flags: ParticleFlags::empty(),
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
		 	flags: pd.flags.bits(),
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