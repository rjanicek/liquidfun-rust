extern crate gcc;

use std::env;
use std::path::{Path};

fn main() {
    let mut config = gcc::Config::new();

    config.cpp(true)
	    .file("liquidfun-c/c_box2d.cpp")
	    .include("liquidfun-cpp");

	// Include stlport if targeting Android.
	// http://stackoverflow.com/questions/23000748/arm-linux-androideabi-stl-compile-error
    match env::var("TARGET") {
    	Ok(ref target) if target == "arm-linux-androideabi" => {
			match env::var("NDK_HOME") {
				Ok(ndk_home) => {
					config.include(Path::new(&ndk_home).join("sources/cxx-stl/stlport/stlport"));
				}
				_ => panic!("Please set NDK_HOME environment variable to Android Native Development Kit path when targeting {:?}.", target)
			}
	    },
	    _ => {}
	}

   	config.compile("libliquidfun.a");
}
