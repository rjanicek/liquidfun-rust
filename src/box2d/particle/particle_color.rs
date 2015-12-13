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
    pub fn new(r: UInt8, g: UInt8, b: UInt8, a: UInt8) -> ParticleColor {
        unsafe {
            ParticleColor { ptr: b2ParticleColor_New(r, g, b, a) }
        }
    }

	pub fn zero() -> ParticleColor {
		ParticleColor::new(0, 0, 0, 0)
	}

    pub fn is_zero(&self) -> bool {
        unsafe {
            b2ParticleColor_IsZero(self.ptr)
        }
    }

	pub fn ptr(&self) -> *mut B2ParticleColor {
		self.ptr
	}

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