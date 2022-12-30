pub mod vec2;
pub mod vec3;
mod vec2_tests;
mod vec3_tests;


use super::*;

/* */
pub enum Vector {
	Vec2d( f64, f64, System ),
	Vec3d( f64, f64, f64, System ),
}

impl Vector {

}