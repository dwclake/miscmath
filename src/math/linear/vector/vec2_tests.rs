#[cfg(test)]
mod vec2_tests {
	use std::f64::consts::*;
	use crate::math::linear::{ System::*, vector::vec2::Vec2 };
	
	#[test]
	fn it_works( ) {
		
		/* Test for create function */
		let test = Vec2::create( &5.6, &7.2, &CARTESIAN );
		dbg!( &test );
		assert!( test.x - 5.6 < 0.0000000001 && test.y - 7.2 < 0.0000000001 );
		
		/* Test for new function */
		let test = Vec2::new();
		dbg!( &test );
		assert!( test.x < 0.0000000001 && test.y < 0.0000000001 );
		
		/* Test for random_unit function */
		let mut test = Vec2::random_unit( &( 0.0..=TAU ) );
		test.swap_system( POLAR );
		dbg!( &test );
		assert!( ( test.x - 1.0 < 0.0000000001 ) && ( test.y <= TAU) );
		
		/* Test for unit function */
		let mut test = Vec2::unit();
		test.swap_system( POLAR );
		dbg!( &test );
		assert!( ( test.x - 1.0 < 0.0000000001 ) && ( test.y < 0.0000000001 ) );
	}
}