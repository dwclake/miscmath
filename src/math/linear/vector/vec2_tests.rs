#[cfg(test)]
mod vec2_tests {
	use std::f64::consts::*;
	use crate::math::linear::{System::*, UnitF, vector::vec2::Vec2};
	
	#[test]
	fn it_works( ) {
		
		/* Test for create function */
		let mut test = Vec2::create( &5.6, &7.2, &CARTESIAN );
		dbg!( &test );
		assert!( test.x - 5.6 < 0.0000000001 && test.y - 7.2 < 0.0000000001 );
		
		/* Test for new function */
		let test1 = Vec2::new();
		dbg!( &test1 );
		assert!( test1.x < 0.0000000001 && test1.y < 0.0000000001 );
		
		/* Test for random_unit function */
		let mut test1 = Vec2::random_unit( &( 0.0..TAU ) );
		test1.swap_system( POLAR );
		dbg!( &test1 );
		assert!( ( test1.x - 1.0 < 0.0000000001 ) && ( test1.y <= TAU) );
		
		/* Test for unit function */
		let mut test2 = Vec2::unit();
		test2.swap_system( POLAR );
		dbg!( &test2 );
		assert!( ( test2.x - 1.0 < 0.0000000001 ) && ( test2.y < 0.0000000001 ) );
		
		dbg!( test.angle_between( &test2 ) );
		assert!( test.angle_between( &test2 ) - 1.4731481877 < 0.0000000001 );
		
		test.lerp( &test2, UnitF::new( 0.5 ) );
		dbg!( &test );
		assert!( test.x - 3.3 < 0.0000000001 && test.y - 3.6 < 0.0000000001 );
	}
}