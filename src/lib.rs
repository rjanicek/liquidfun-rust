//! Rust bindings for [LiquidFun](https://github.com/google/liquidfun/).
//!
//! [Repository](https://github.com/rjanicek/liquidfun-rust)
//!
//! Anything with a `B2` prefix indicates ffi boundry with the C++ API.
//!
//! User Data
//! ---------
//! Some objects in LiquidFun allow you to attach your own data to them. These are bound as `usize`. You can store raw pointers to your data there.
//!
//! usage:
//!  move raw pointer in: `&user_data as *const T as size_t;`
//!
//!  dereference raw pointer out: `*(user_data as *const T)`
//! 
//! example:  
//! ```
//! 	ground_body_def.user_data = &Vec2::new(6.0, 66.0) as *const Vec2 as usize;
//! ```
//!
//! Passing structs by value from C++ to Rust
//! -----------------------------------------
//! 
//! * make a new typedef struct with same name as C++ struct but with `c_` prefix
//! * add just the data parts from the C++ struct, make sure both struct members have the same order
//! 
//! ```cpp
//! // C++
//! struct b2Vec2 {
//! 	b2Vec2() {}
//! 	...
//! 	float32 x, y;
//! };
//! 
//! // new C struct
//! typedef struct c_b2Vec2 { float32 x, y; } c_b2Vec2;
//! ```
//! 
//! * make a casting function
//! * cast C++ struct to new C struct and pass back to rust
//! 
//! ```cpp
//! c_b2Vec2* cast(b2Vec2* v) {
//!     return reinterpret_cast<c_b2Vec2*>(v);
//! }
//! 
//! c_b2Vec2 sendStructFromCppToRustByValue() {
//! 	b2Vec2 tmp;
//!     return *cast(&tmp);
//! }
//! ```

extern crate libc;
#[macro_use]
extern crate bitflags;

pub mod box2d;
pub mod ext;