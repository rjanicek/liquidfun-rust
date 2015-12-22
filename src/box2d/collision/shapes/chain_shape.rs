use std::slice;
use super::shape::*;
use super::super::super::common::math::*;
use super::super::super::common::settings::*;

enum B2ChainShape {}

extern {
    fn b2ChainShape_Delete(ptr: *mut B2ChainShape);
    fn b2ChainShape_New() -> *mut B2ChainShape;
    fn b2ChainShape_Upcast(ptr: *mut B2ChainShape) -> *mut B2Shape;
    fn b2ChainShape_CreateChain(ptr: *mut B2ChainShape, vertices: *const Vec2, count: Int32);
    fn b2ChainShape_GetChildCount(ptr: *mut B2ChainShape) -> Int32;
    fn b2ChainShape_m_count(ptr: *mut B2ChainShape) -> Int32;
    fn b2ChainShape_m_vertices(ptr: *mut B2ChainShape) -> *const Vec2;
}

/// A chain shape is a free form sequence of line segments.
/// The chain has two-sided collision, so you can use inside and outside collision.
/// Therefore, you may use any winding order.
/// Since there may be many vertices, they are allocated using b2Alloc.
/// Connectivity information is used to create smooth collisions.
/// WARNING: The chain will not collide properly if there are self-intersections.
pub struct ChainShape {
    ptr: *mut B2ChainShape,
    owned: bool,    
}

/// Cast a ChainShape from a B2Shape.
pub fn from_shape(ptr: *mut B2Shape) -> ChainShape {
    ChainShape { ptr: ptr as *mut B2ChainShape, owned: false}
}

impl Shape for ChainShape {
    // Is the up-cast even necessary? Can't we just use self.ptr directly?
    fn handle(&self) -> *mut B2Shape {
        unsafe {
            b2ChainShape_Upcast(self.ptr)
        }
    }
}

impl ChainShape {

    /// Create a new ChainShape.
    pub fn new() -> ChainShape {
        unsafe {
            ChainShape { ptr: b2ChainShape_New(), owned: true }
        }
    }

	/// Create a chain with isolated end vertices.
	/// @param vertices an array of vertices, these are copied
	/// @param count the vertex count
    pub fn create_chain(&mut self, vertices: &[Vec2], count: i32) {
    	unsafe {
    		b2ChainShape_CreateChain(self.ptr, vertices.as_ptr(), count);
    	}
    }
	
	/// @see b2Shape::GetChildCount
    pub fn get_child_count(&self) -> i32 {
        unsafe {
            b2ChainShape_GetChildCount(self.ptr)
        }
    }

	/// The vertex count.
 	pub fn get_vertex_count(&self) -> i32 {
        unsafe {
            b2ChainShape_m_count(self.ptr)
        }
    }

	/// The vertices. Owned by this class.
 	pub fn get_vertices(&self) -> &[Vec2] {
        unsafe {
			slice::from_raw_parts(b2ChainShape_m_vertices(self.ptr), self.get_vertex_count() as usize)
        }
    }

}

/// The destructor frees the vertices using b2Free.
impl Drop for ChainShape {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                b2ChainShape_Delete(self.ptr);
            }
        }
    }
}