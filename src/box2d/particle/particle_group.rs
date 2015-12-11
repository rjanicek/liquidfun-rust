pub enum B2ParticleGroup {}

#[allow(raw_pointer_derive)]
#[derive(Clone)]
pub struct ParticleGroup {
    pub ptr: *mut B2ParticleGroup
}