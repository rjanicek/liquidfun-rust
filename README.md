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

Thank You
---------
Erin Catto for [Box2D](https://github.com/erincatto/Box2D)

Google for [LiquidFun](https://github.com/google/liquidfun)

Nicolas Silva for [box2d.rs](https://github.com/nical/box2d.rs)