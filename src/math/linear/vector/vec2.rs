use rand::Rng;
use std::ops::RangeInclusive;
//use std::cmp::Ordering;

use crate::math::linear::{ *, System::* };

// Vec2 Struct
#[derive(Debug)]
pub struct Vec2 {
	pub x: f64,
	pub y: f64,
	system: System,
}

impl Vec2 {
	
	// new function
	// Generates a new instance of Vec2 initialized to zero and returns it
	pub fn new( ) -> Vec2 {
		Vec2 { x: 0.0, y: 0.0, system: CARTESIAN }
	}
	
	// create function
	// Generates a new instance of Vec2 initialized to chosen values and returns it
	pub fn create( x: &f64, y: &f64, system: &System ) -> Vec2 {
		let mut temp = Vec2 {
			x: *x,
			y: *y,
			system: *system
		};
		temp.swap_system( CARTESIAN );
		temp
	}
	
	// create_random function
	// Generates a new instance of Vec2 initialized to random values in the range entered and returns it
	pub fn create_random( range: &RangeInclusive<f64>, system: &System ) -> Vec2 {
		let mut temp = Vec2 {
			x: rand::thread_rng().gen_range( range.clone() ),
			y: rand::thread_rng().gen_range( range.clone() ),
			system: *system,
		};
		temp.swap_system( CARTESIAN );
		temp
	}
	
	// random_unit function
	// Generates a new instance of Vec2 initialized to a magnitude of 1 and a angle of random value in the range entered and returns it
	pub fn random_unit( range: &RangeInclusive<f64> ) -> Vec2 {
		let mut temp = Vec2::create( &1.0, &rand::thread_rng().gen_range( range.clone() ), &POLAR );
		temp.swap_system( CARTESIAN );
		temp
	}
	
	// unit function
	// Generates a new instance of Vec2 initialized as a unit vector of angle 0 and returns it
	pub fn unit( ) -> Vec2 {
		let temp = Vec2 { x: 1.0, y: 0.0, system: POLAR };
		temp
	}
	
	// add function
	// Adds the components from the rhs Vec2 to the corresponding components of self
	pub fn add( &mut self, rhs: &Vec2 ) -> Vec2 {
		let mut temp = Vec2::new( );
		self.x = self.x + rhs.x;
		self.y = self.y + rhs.y;
		temp.x = self.x;
		temp.y = self.y;
		temp
	}
	
	// dbg function
	// Prints debug information of self to terminal
	pub fn dbg( &self ) {
		dbg!( self );
	}
	
	// div function
	// Divides the components of self by a scalar value rhs
	pub fn div( &mut self, rhs: &f64 ) {
		self.x = self.x / rhs;
		self.y = self.y / rhs;
	}
	
	// mul function
	// Multiplies the components of self by a scalar value rhs
	pub fn mult( &mut self, rhs: &f64 ) {
		self.x = self.x * rhs;
		self.y = self.y * rhs;
	}
	
	// norm function
	// Normalizes the magnitude of self to 1, angle is unchanged
	pub fn norm( &mut self ) {
		self.swap_system( POLAR );
		self.x = 1.0;
		self.swap_system( CARTESIAN );
	}
	
	// rem function
	// Sets the components of self to the remainder of scalar division by rhs
	pub fn rem( &mut self, rhs: &f64 ) {
		self.x = self.x % rhs;
		self.y = self.y % rhs;
	}
	
	// sub function
	// Subtracts the components of the rhs Vec2 from the corresponding components of self
	pub fn sub( &mut self, rhs: &Vec2 ) {
		self.x = self.x - rhs.x;
		self.y = self.y - rhs.y;
	}
	
	// swap_system function
	// Converts components from cartesian to polar coordinates, and vice versa
	pub fn swap_system( &mut self, new_system: System ) {
		if self.system == POLAR && new_system == CARTESIAN {
			
			let x = self.x;
			self.x = ( (self.x).powi(2 ) + ( self.y ).powi(2 ) ).sqrt( );
			self.y = self.y.atan2(x );
			self.system = new_system;
			
		} else if self.system == CARTESIAN && new_system == POLAR {
			
			let theta = self.y;
			self.y = self.x * self.y.sin();
			self.x = self.x * theta.cos();
			self.system = new_system;
		}
	}
}