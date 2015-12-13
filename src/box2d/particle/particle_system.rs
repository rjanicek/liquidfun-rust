use super::*;
use super::super::common::math::*;
use super::super::common::settings::*;
use std::slice;

#[repr(C)]
#[derive(Debug)]
pub struct ParticleSystemDef {
    pub strict_contact_check: bool,
    pub density: Float32,
    pub gravity_scale: Float32,
    pub radius: Float32,
    pub max_count: Int32,
    pub pressure_strength: Float32,
    pub damping_strength: Float32,
    pub elastic_strength: Float32,
    pub spring_strength: Float32,
    pub viscous_strength: Float32,
    pub surface_tension_pressure_strength: Float32,
    pub surface_tension_normal_strength: Float32,
    pub repulsive_strength: Float32,
    pub powder_strength: Float32,
    pub ejection_strength: Float32,
    pub static_pressure_strength: Float32,
    pub static_pressure_relaxation: Float32,
    pub static_pressure_iterations: Int32,
    pub color_mixing_strength: Float32,
    pub destroy_by_age: bool,
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
    fn b2ParticleSystem_GetNext(ps: *mut B2ParticleSystem) -> *mut B2ParticleSystem;
    fn b2ParticleSystem_GetPositionBuffer(ps: *mut B2ParticleSystem) -> *mut Vec2;
}

#[allow(raw_pointer_derive)]
#[derive(Clone)]
pub struct ParticleSystem {
    pub ptr: *mut B2ParticleSystem
}

impl ParticleSystem {
    pub fn create_particle(&self, pd: &ParticleDef) -> Int32 {
        unsafe {
            b2ParticleSystem_CreateParticle(self.ptr, &B2ParticleDef::from(pd))
        }
    }

    pub fn destroy_particle(&self, index: Int32) {
        unsafe {
            b2ParticleSystem_DestroyParticle(self.ptr, index);
        }
    }

    pub fn get_particle_count(&self) -> Int32 {
        unsafe {
            b2ParticleSystem_GetParticleCount(self.ptr)
        }
    }

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

    pub fn get_position_buffer(&self) -> &[Vec2] {
        unsafe {
            slice::from_raw_parts(b2ParticleSystem_GetPositionBuffer(self.ptr), self.get_particle_count() as usize)
        }
    }

}