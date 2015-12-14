use super::super::common::settings::*;

pub enum B2ParticleColor {}

extern {
	fn b2ParticleColor_New(r: UInt8, g: UInt8, b: UInt8, a: UInt8) -> *mut B2ParticleColor;
	fn b2ParticleColor_Delete(p: *mut B2ParticleColor);
    fn b2ParticleColor_Set(p: *mut B2ParticleColor, r: UInt8, g: UInt8, b: UInt8, a: UInt8);
    fn b2ParticleColor_IsZero(p: *mut B2ParticleColor) -> bool;
}


/// Small color object for each particle
#[allow(raw_pointer_derive)]
#[derive(Clone)]
pub struct ParticleColor {
	pub ptr: *mut B2ParticleColor
}

impl ParticleColor {

    /// Constructor with four elements: r (red), g (green), b (blue), and a
    /// (opacity).
    /// Each element can be specified 0 to 255.
    pub fn new(r: UInt8, g: UInt8, b: UInt8, a: UInt8) -> ParticleColor {
        unsafe {
            ParticleColor { ptr: b2ParticleColor_New(r, g, b, a) }
        }
    }

    /// Create a ParticleColor with zero values.
	pub fn zero() -> ParticleColor {
		ParticleColor::new(0, 0, 0, 0)
	}

    /// True when all four color elements equal 0. When true, a particle color
    /// buffer isn't allocated by CreateParticle().
    pub fn is_zero(&self) -> bool {
        unsafe {
            b2ParticleColor_IsZero(self.ptr)
        }
    }

    /// Get ParticleColor's raw pointer.
	pub fn ptr(&self) -> *mut B2ParticleColor {
		self.ptr
	}

    /// Sets color for current object using the four elements described above.
    pub fn set(&self, r: UInt8, g: UInt8, b: UInt8, a: UInt8) {
        unsafe {
            b2ParticleColor_Set(self.ptr, r, g, b, a);
        }
    }

}

impl Drop for ParticleColor {
    fn drop(&mut self) {
        unsafe {
            b2ParticleColor_Delete(self.ptr);
        }
    }
}