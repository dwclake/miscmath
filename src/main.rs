use miscmath::prelude::*;

fn main() {
	let x = Vec2::unit( );
	dbg!( &x );
	
	let y = Vec2::create( &0.5, &7.2 );
	dbg!( &y );
	
	dbg!( y == y );
}