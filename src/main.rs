use mathlib::math;

use mathlib::math::linear::{ System::*, vector::vec2::Vec2 };

fn main() {
	let x = Vec2::unit( );
	dbg!( &x );
	
	let y = Vec2::create( &0.5, &7.2, &POLAR );
	dbg!( &y );
	
	dbg!( x.angle_between( &y ) );
	
	dbg!( math::map(1.0, 0.0..1.0, 0.0..100.0 ) );
	dbg!( math::factorial( 5 ) );
}