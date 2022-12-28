#[cfg(test)]
mod vec2_tests {
	use crate::math::*;

	#[test]
	fn it_works( ) {
		/*let result = vector2::add(&2, &2 );
		assert_eq!( result, 4 );*/
		println!( "test" );
		adds();
	}
}


use rand::Rng;
//use std::cmp::Ordering;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Coordinates {
	Cartesian,
	Polar,
}

#[derive(Debug)]
pub struct Vec2{
	pub x: f64,
	pub y: f64,
	pub system: Coordinates,
}

use Coordinates::*;

impl Vec2 {
	pub fn new( ) -> Self {
		Self { x: 0.0, y: 0.0, system: Cartesian }
	}

	pub fn create( x: f64, y: f64, system: Coordinates ) -> Self {
		Self { x, y, system }
	}

	pub fn create_random( min: f64, max: f64, system: Coordinates ) -> Self {
		Self {
			x: rand::thread_rng().gen_range( min..max ),
			y: rand::thread_rng().gen_range( min..max ),
			system,
		}
	}

	pub fn random_unit( min: f64, max: f64 ) -> Self {
		let mut temp= Vec2::create( 1.0, rand::thread_rng().gen_range( min..max ), Polar );
		temp.swap_system( Cartesian );
		temp
	}

	pub fn unit( ) -> Self {
		Self { x: 1.0, y: 1.0, system: Cartesian }
	}

	pub fn add( &mut self, rhs: &Vec2 ) /*-> Vec2*/ {
		//let mut temp = Vec2::new( );
		self.x = self.x + rhs.x;
		self.y = self.y + rhs.y;
		/*temp.x = self.x;
		temp.y = self.y;
		temp*/
	}

	pub fn dbg( &self, name: &str ) {
		println!( "{name}: " );
		dbg!( self );
	}

	pub fn div( &mut self, rhs: &f64 ) {
		self.x = self.x / rhs;
		self.y = self.y / rhs;
	}

	pub fn get_system( &self ) -> &str {
		let system :&str = match self.system {
			Cartesian => "Cartesian",
			Polar => "Polar",
		};
		system
	}

	pub fn mult( &mut self, rhs: &f64 ) {
		self.x = self.x * rhs;
		self.y = self.y * rhs;
	}

	pub fn norm( &mut self ) {
		self.swap_system( Polar );
		self.x = 1.0;
		self.swap_system( Cartesian );
	}

	pub fn rem( &mut self, rhs: &f64 ) {
		self.x = self.x % rhs;
		self.y = self.y % rhs;
	}

	pub fn sub( &mut self, rhs: &Vec2 ) {
		self.x = self.x - rhs.x;
		self.y = self.y - rhs.y;
	}

	pub fn swap_system( &mut self, new_system: Coordinates ) {
		if self.system == Polar && new_system == Cartesian {

			let x = self.x;
			self.x = ( (self.x).powi(2 ) + ( self.y ).powi(2 ) ).sqrt( );
			self.y = self.y.atan2(x );
			self.system = new_system;

		} else if self.system == Cartesian && new_system == Polar {

			let theta = self.y;
			self.y = self.x * self.y.sin();
			self.x = self.x * theta.cos();
			self.system = new_system;
		}
	}
}
