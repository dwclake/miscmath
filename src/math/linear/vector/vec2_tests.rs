#[cfg(test)]
mod vec2_tests {
	use std::f64::consts::*;
	use crate::math::linear::vector::vec2::*;
	use crate::math::linear::System::*;
	
	#[test]
	fn it_works( ) {
		
		// # Test for random_unit function
		let mut test = Vec2::random_unit( &(0.0..=TAU) );
		test.swap_system( POLAR );
		dbg!( &test );
		assert!( ( test.x - 1.0 < 0.0000000001 ) && ( test.y <= TAU) );
		
		// # Test for unit function
		let mut test = Vec2::unit();
		test.swap_system( POLAR );
		dbg!( &test );
		assert!( ( test.x - 1.0 < 0.0000000001 ) && ( test.y < 0.0000000001 ) );
	}
}