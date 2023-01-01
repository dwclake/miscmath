use miscmath::{ CoordSystem::*, vector::Vec2 };

fn main() {
	let x = Vec2::unit( );
	dbg!( &x );
	
	let y = Vec2::create( &0.5, &7.2, &POLAR );
	dbg!( &y );
	
	dbg!( y == y );
}