pub enum B2ParticleGroup {}

/// A group of particles. b2ParticleGroup::CreateParticleGroup creates these.
#[allow(raw_pointer_derive)]
#[derive(Clone)]
pub struct ParticleGroup {
    pub ptr: *mut B2ParticleGroup
}

impl ParticleGroup {

	/// Get ParticleGroup's raw pointer.
	pub fn ptr(&self) -> *mut B2ParticleGroup {
		self.ptr
	}
}