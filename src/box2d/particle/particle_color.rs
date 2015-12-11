pub enum B2ParticleColor {}

extern {
	fn b2ParticleColor_Zero() -> *mut B2ParticleColor;
}


/// Small color object for each particle
// #[allow(raw_pointer_derive)]
// #[derive(Clone)]
// pub struct ParticleColor {
// 	pub ptr: *mut B2ParticleColor
// }

pub fn zero() -> *mut B2ParticleColor {
	unsafe {
		b2ParticleColor_Zero()
	}
}