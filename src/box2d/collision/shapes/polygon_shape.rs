use super::shape::*;
use super::super::super::common::settings::*;
use super::super::super::common::math::*;

enum B2PolygonShape {}

extern {
    fn b2PolygonShape_Delete(ptr: *mut B2PolygonShape);
    fn b2PolygonShape_GetVertex(ptr: *mut B2PolygonShape, index: Int32) -> &Vec2;
    fn b2PolygonShape_GetVertexCount(ptr: *const B2PolygonShape) -> Int32;
    fn b2PolygonShape_New() -> *mut B2PolygonShape;
    fn b2PolygonShape_SetAsBox(ptr: *mut B2PolygonShape, hx: Float32, hy: Float32);
    fn b2PolygonShape_Upcast(ptr: *mut B2PolygonShape) -> *mut B2Shape;
}

pub struct PolygonShape {
    ptr: *mut B2PolygonShape
}

pub fn from_shape(ptr: *mut B2Shape) -> PolygonShape {
    PolygonShape { ptr: ptr as *mut B2PolygonShape}
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

    pub fn delete(&self) {
        unsafe {
            b2PolygonShape_Delete(self.ptr);
        }
    }

    pub fn get_vertex(&self, index: Int32) -> &Vec2 {
        unsafe {
            b2PolygonShape_GetVertex(self.ptr, index)
        }
    }

    pub fn get_vertex_count(&self) -> Int32 {
        unsafe {
            b2PolygonShape_GetVertexCount(self.ptr)
        }
    }

    pub fn set_as_box(&mut self, hx:Float32, hy:Float32) {
        unsafe {
            b2PolygonShape_SetAsBox(self.ptr, hx, hy);
        }
    }
}

// This may not be a good idea, what if there are multiple references to same
// shape and one of them goes out of scope and is dropped? The remaining shape
// may cause use after free!
// impl Drop for PolygonShape {
//     fn drop(&mut self) {
//         unsafe {
//             b2PolygonShape_Delete(self.ptr);
//         }
//     }
// }