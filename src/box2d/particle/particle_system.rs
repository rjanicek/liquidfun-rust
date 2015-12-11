use super::super::common::settings::*;

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

#[allow(raw_pointer_derive)]
#[derive(Clone)]
pub struct ParticleSystem {
    pub ptr: *mut B2ParticleSystem
}