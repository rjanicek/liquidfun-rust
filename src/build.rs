extern crate gcc;

fn main() {
    gcc::Config::new()
    	.cpp(true)
	    .file("liquidfun-c/c_box2d.cpp")
	    .include("liquidfun-cpp")
	    .compile("libliquidfun.a");
}
