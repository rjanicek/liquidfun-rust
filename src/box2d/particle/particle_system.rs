use super::*;
use super::super::common::math::*;
use super::super::common::settings::*;
use std::slice;

#[repr(C)]
#[derive(Debug)]
pub struct ParticleSystemDef {
    /// Enable strict Particle/Body contact check.
    /// See SetStrictContactCheck for details.
    pub strict_contact_check: bool,

    /// Set the particle density.
    /// See SetDensity for details.
    pub density: Float32,

    /// Change the particle gravity scale. Adjusts the effect of the global
    /// gravity vector on particles. Default value is 1.0f.
    pub gravity_scale: Float32,

    /// Particles behave as circles with this radius. In Box2D units.
    pub radius: Float32,

    /// Set the maximum number of particles.
    /// By default, there is no maximum. The particle buffers can continue to
    /// grow while b2World's block allocator still has memory.
    /// See SetMaxParticleCount for details.
    pub max_count: Int32,

    /// Increases pressure in response to compression
    /// Smaller values allow more compression
    pub pressure_strength: Float32,

    /// Reduces velocity along the collision normal
    /// Smaller value reduces less
    pub damping_strength: Float32,

    /// Restores shape of elastic particle groups
    /// Larger values increase elastic particle velocity
    pub elastic_strength: Float32,

    /// Restores length of spring particle groups
    /// Larger values increase spring particle velocity
    pub spring_strength: Float32,

    /// Reduces relative velocity of viscous particles
    /// Larger values slow down viscous particles more
    pub viscous_strength: Float32,

    /// Produces pressure on tensile particles
    /// 0~0.2. Larger values increase the amount of surface tension.
    pub surface_tension_pressure_strength: Float32,

    /// Smoothes outline of tensile particles
    /// 0~0.2. Larger values result in rounder, smoother, water-drop-like
    /// clusters of particles.
    pub surface_tension_normal_strength: Float32,

    /// Produces additional pressure on repulsive particles
    /// Larger values repulse more
    /// Negative values mean attraction. The range where particles behave
    /// stably is about -0.2 to 2.0.
    pub repulsive_strength: Float32,

    /// Produces repulsion between powder particles
    /// Larger values repulse more
    pub powder_strength: Float32,

    /// Pushes particles out of solid particle group
    /// Larger values repulse more
    pub ejection_strength: Float32,

    /// Produces static pressure
    /// Larger values increase the pressure on neighboring partilces
    /// For a description of static pressure, see
    /// http://en.wikipedia.org/wiki/Static_pressure#Static_pressure_in_fluid_dynamics
    pub static_pressure_strength: Float32,

    /// Reduces instability in static pressure calculation
    /// Larger values make stabilize static pressure with fewer iterations
    pub static_pressure_relaxation: Float32,

    /// Computes static pressure more precisely
    /// See SetStaticPressureIterations for details
    pub static_pressure_iterations: Int32,

    /// Determines how fast colors are mixed
    /// 1.0f ==> mixed immediately
    /// 0.5f ==> mixed half way each simulation step (see b2World::Step())
    pub color_mixing_strength: Float32,

    /// Whether to destroy particles by age when no more particles can be
    /// created.  See #b2ParticleSystem::SetDestructionByAge() for
    /// more information.
    pub destroy_by_age: bool,

    /// Granularity of particle lifetimes in seconds.  By default this is
    /// set to (1.0f / 60.0f) seconds.  b2ParticleSystem uses a 32-bit signed
    /// value to track particle lifetimes so the maximum lifetime of a
    /// particle is (2^32 - 1) / (1.0f / lifetimeGranularity) seconds.
    /// With the value set to 1/60 the maximum lifetime or age of a particle is
    /// 2.27 years.
    pub lifetime_granularity: Float32,
}

impl Default for ParticleSystemDef {
    fn default () -> ParticleSystemDef {
        ParticleSystemDef {
            strict_contact_check: false,
            density: 1.0,
            gravity_scale: 1.0,
            radius: 1.0,
            max_count: 0,
            pressure_strength: 0.05,
            damping_strength: 1.0,
            elastic_strength: 0.25,
            spring_strength: 0.25,
            viscous_strength: 0.25,
            surface_tension_pressure_strength: 0.2,
            surface_tension_normal_strength: 0.2,
            repulsive_strength: 1.0,
            powder_strength: 0.5,
            ejection_strength: 0.5,
            static_pressure_strength: 0.2,
            static_pressure_relaxation: 0.2,
            static_pressure_iterations: 8,
            color_mixing_strength: 0.5,
            destroy_by_age: true,
            lifetime_granularity: 1.0 / 60.0,
        }
    }
}

pub enum B2ParticleSystem {}

extern {
    fn b2ParticleSystem_CreateParticle(ps: *mut B2ParticleSystem, pd: &B2ParticleDef) -> Int32;
    fn b2ParticleSystem_DestroyParticle(ps: *mut B2ParticleSystem, index: Int32);
    fn b2ParticleSystem_GetParticleCount(ps: *mut B2ParticleSystem) -> Int32;
    fn b2ParticleSystem_GetParticleFlags(ps: *mut B2ParticleSystem, index: Int32) -> UInt32;
    fn b2ParticleSystem_GetNext(ps: *mut B2ParticleSystem) -> *mut B2ParticleSystem;
    fn b2ParticleSystem_GetPositionBuffer(ps: *mut B2ParticleSystem) -> *mut Vec2;
}

#[allow(raw_pointer_derive)]
#[derive(Clone)]
pub struct ParticleSystem {
    pub ptr: *mut B2ParticleSystem
}

impl ParticleSystem {

    /// Create a particle whose properties have been defined.
    /// No reference to the definition is retained.
    /// A simulation step must occur before it's possible to interact with a
    /// newly created particle.  For example, DestroyParticleInShape() will
    /// not destroy a particle until b2World::Step() has been called.
    /// @warning This function is locked during callbacks.
    /// @return the index of the particle.
    pub fn create_particle(&self, pd: &ParticleDef) -> Int32 {
        unsafe {
            b2ParticleSystem_CreateParticle(self.ptr, &B2ParticleDef::from(pd))
        }
    }

    /// Destroy a particle.
    /// The particle is removed after the next simulation step (see
    /// b2World::Step()).
    pub fn destroy_particle(&self, index: Int32) {
        unsafe {
            b2ParticleSystem_DestroyParticle(self.ptr, index);
        }
    }

    /// Get the number of particles.
    pub fn get_particle_count(&self) -> Int32 {
        unsafe {
            b2ParticleSystem_GetParticleCount(self.ptr)
        }
    }

    /// Get flags for a particle. See the ParticleFlags struct.
    pub fn get_particle_flags(&self, index: Int32) -> Option<ParticleFlags> {
        unsafe {
            ParticleFlags::from_bits(b2ParticleSystem_GetParticleFlags(self.ptr, index))
        }
    }    

    /// Get the next particle-system in the world's particle-system list.
    pub fn get_next(&self) -> Option<ParticleSystem> {
        let ptr: *mut B2ParticleSystem;
        
        unsafe {
            ptr = b2ParticleSystem_GetNext(self.ptr);
        }

        if ptr.is_null() {
            None
        } else {
            Some(ParticleSystem { ptr: ptr })
        }
    }

    /// Get the position of each particle
    pub fn get_position_buffer(&self) -> &[Vec2] {
        unsafe {
            slice::from_raw_parts(b2ParticleSystem_GetPositionBuffer(self.ptr), self.get_particle_count() as usize)
        }
    }

}