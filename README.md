LiquidFun Rust
==============

Rust bindings for [LiquidFun](https://github.com/google/liquidfun/).

* Model LiquidFun C++ API as closely as possible in idiomatic Rust.
* Documentation for C++ API should be usable for Rust API.
* Document places where Rust API varries from C++ API.

Usage
-----
First, add the following to your `Cargo.toml`:

```toml
[dependencies]
liquidfun = "*"
```

Next, add this to your crate root:

```rust
extern crate liquidfun;
```

Example
-------

```rust
extern crate liquidfun;

use liquidfun::box2d::common::math::*;
use liquidfun::box2d::dynamics::world::*;

#[test]
fn hello_world() {

	// Define the gravity vector.
	let gravity = Vec2::new(0.0, -10.0);

	// Construct a world object, which will hold and simulate the rigid bodies.
	let mut world = World::new(&gravity);
	assert_eq!(gravity, world.get_gravity());

}

```

Status
------

The LiquidFun Hello World example compiles and runs. More bindings will be added as needed by projects that depend on LiquidFun Rust.

User Data
---------
Some objects in LiquidFun allow you to attach your own data to them. These are bound as `usize`. You can store raw pointers to your data there.

```
	ground_body_def.user_data = &Vec2::new(6.0, 66.0) as *const Vec2 as usize;
```

Passing structs by value from C++ to Rust
-----------------------------------------

* make a new typedef struct with same name as C++ struct but with `c_` prefix
* add just the data parts from the C++ struct, make sure both struct members have the same order

```cpp
// C++
struct b2Vec2 {
	b2Vec2() {}
	...
	float32 x, y;
};

// new C struct
typedef struct c_b2Vec2 { float32 x, y; } c_b2Vec2;
```

* make a casting function
* cast C++ struct to new C struct and pass back to rust

```cpp
c_b2Vec2* cast(b2Vec2* v) {
    return reinterpret_cast<c_b2Vec2*>(v);
}

c_b2Vec2 sendStructFromCppToRustByValue() {
	b2Vec2 tmp;
    return *cast(&tmp);
}
```

References
----------
[LiquidFun Programmer's Guide](https://google.github.io/liquidfun/Programmers-Guide.html)
[LiquidFun API Documentation](https://google.github.io/liquidfun/API-Ref.html)

Thank You
---------
Erin Catto for [Box2D](https://github.com/erincatto/Box2D)

Google for [LiquidFun](https://github.com/google/liquidfun)

Nicolas Silva for [box2d.rs](https://github.com/nical/box2d.rs)