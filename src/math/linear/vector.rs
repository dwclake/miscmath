pub mod vec2;
mod vec2_tests;

use super::*;

pub enum Vector {
	Vec2d( f64, f64, System ),
	Vec3d( f64, f64, f64, System ),
}

impl Vector {

}