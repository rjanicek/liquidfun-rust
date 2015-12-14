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

/// A convex polygon. It is assumed that the interior of the polygon is to
/// the left of each edge.
/// Polygons have a maximum number of vertices equal to b2_maxPolygonVertices.
/// In most cases you should not need many vertices for a convex polygon.
pub struct PolygonShape {
    ptr: *mut B2PolygonShape,
    owned: bool,    
}

/// Cast a PolygonShape from a B2Shape.
pub fn from_shape(ptr: *mut B2Shape) -> PolygonShape {
    PolygonShape { ptr: ptr as *mut B2PolygonShape, owned: false}
}

impl Shape for PolygonShape {
    // Is the up-cast even necessary? Can't we just use self.ptr directly?
    fn handle(&self) -> *mut B2Shape {
        unsafe {
            b2PolygonShape_Upcast(self.ptr)
        }
    }
}

impl PolygonShape {

    /// Create a new PolygonShape.
    pub fn new() -> PolygonShape {
        unsafe {
            PolygonShape { ptr: b2PolygonShape_New(), owned: true }
        }
    }

    /// Get a vertex by index.
    pub fn get_vertex(&self, index: i32) -> &Vec2 {
        unsafe {
            b2PolygonShape_GetVertex(self.ptr, index)
        }
    }

    /// Get the vertex count.
    pub fn get_vertex_count(&self) -> i32 {
        unsafe {
            b2PolygonShape_GetVertexCount(self.ptr)
        }
    }

    /// Build vertices to represent an axis-aligned box centered on the local origin.
    /// @param hx the half-width.
    /// @param hy the half-height.
    pub fn set_as_box(&mut self, hx:f32, hy:f32) {
        unsafe {
            b2PolygonShape_SetAsBox(self.ptr, hx, hy);
        }
    }
}

impl Drop for PolygonShape {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                b2PolygonShape_Delete(self.ptr);
            }
        }
    }
}