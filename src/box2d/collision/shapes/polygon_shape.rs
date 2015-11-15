use super::shape::*;
use super::super::super::common::settings::*;

enum B2PolygonShape {}

extern {
    fn b2PolygonShape_Delete(ptr: *mut B2PolygonShape);
    fn b2PolygonShape_New() -> *mut B2PolygonShape;
    fn b2PolygonShape_SetAsBox(ptr: *mut B2PolygonShape, hx: Float32, hy: Float32);
    fn b2PolygonShape_Upcast(ptr: *mut B2PolygonShape) -> *mut B2Shape;
}

pub struct PolygonShape {
    ptr: *mut B2PolygonShape
}

impl Default for PolygonShape {
    fn default() -> PolygonShape {
        unsafe {
            PolygonShape { ptr: b2PolygonShape_New() }
        }
    }
}

// Is the up-cast even necessary? Can't we just use self.ptr directly?
impl Shape for PolygonShape {
    fn handle(&self) -> *mut B2Shape {
        unsafe {
            b2PolygonShape_Upcast(self.ptr)
        }
    }
}

impl PolygonShape {
    pub fn set_as_box(&mut self, hx:Float32, hy:Float32) {
        unsafe {
            b2PolygonShape_SetAsBox(self.ptr, hx, hy);
        }
    }    
}

impl Drop for PolygonShape {
    fn drop(&mut self) {
        unsafe {
            b2PolygonShape_Delete(self.ptr);
        }
    }
}