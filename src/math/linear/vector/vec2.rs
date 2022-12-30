use rand::Rng;
use std::ops::Range;

use crate::math::linear::{ *, System::* };

/* Vec2 Struct */
#[derive(Debug)]
pub struct Vec2 {
	pub x: f64,
	pub y: f64,
	system: System,
}

impl Vec2 {
	
	/* new function
	   Generates a new instance of Vec2 initialized to zero and returns it */
	pub fn new( ) -> Vec2 {
		Vec2 { x: 0.0, y: 0.0, system: CARTESIAN }
	}
	
	/* create function
	   Generates a new instance of Vec2 initialized to chosen values and returns it */
	pub fn create( x: &f64, y: &f64, system: &System ) -> Vec2 {
		let mut temp = Vec2 {
			x: *x,
			y: *y,
			system: *system
		};
		temp.swap_system( CARTESIAN );
		temp
	}
	
	/* create_random function
	   Generates a new instance of Vec2 initialized to random values in the range entered and returns it */
	pub fn create_random( range: &Range<f64>, system: &System ) -> Vec2 {
		let mut temp = Vec2 {
			x: rand::thread_rng().gen_range( range.clone() ),
			y: rand::thread_rng().gen_range( range.clone() ),
			system: *system,
		};
		temp.swap_system( CARTESIAN );
		temp
	}
	
	/* from_angle function
	   Generates a new instance of Vec2 based on a entered angle with a magnitude of 1 */
	pub fn from_angle( theta: &f64 ) -> Vec2 {
		let mut temp = Vec2::create( &1.0, theta, &POLAR );
		temp.swap_system( CARTESIAN );
		temp
	}
	
	/* random_unit function
	   Generates a new instance of Vec2 initialized to a magnitude of 1 and a angle of random value in the range entered and returns it */
	pub fn random_unit( range: &Range<f64> ) -> Vec2 {
		let mut temp = Vec2::create( &1.0, &rand::thread_rng().gen_range( range.clone() ), &POLAR );
		temp.swap_system( CARTESIAN );
		temp
	}
	
	/* unit function
	   Generates a new instance of Vec2 initialized as a unit vector of angle 0 and returns it */
	pub fn unit( ) -> Vec2 {
		let mut temp = Vec2 { x: 1.0, y: 0.0, system: POLAR };
		temp.swap_system( CARTESIAN );
		temp
	}
	
	/* add function
	   Adds the components from the rhs Vec2 to the corresponding components of self */
	pub fn add( &mut self, rhs: &Vec2 ) -> Vec2 {
		let mut temp = Vec2::new( );
		self.x += rhs.x;
		self.y += rhs.y;
		temp.x = self.x;
		temp.y = self.y;
		temp
	}
	
	/* angle_between
	   Calculates the angle between self and a passed in Vec2 */
	pub fn angle_between( &self, rhs: &Vec2 ) -> f64 {
		let dot = self.dot( &rhs );
		( dot / ( self.mag() * rhs.mag() ) ).acos()
	}
	
	/* cross function
	   Generates a new instance of Vec2 which is perpendicular to the self instance */
	pub fn cross( &mut self ) {
		let x = self.x;
		self.x = self.y;
		self.y = -x;
	}
	
	/* dist function
	   Calculates the distance between self and the Vec2 passed in */
	pub fn dist( &self, rhs: &Vec2 ) -> f64 {
		( ( self.x - rhs.x ).powi(2) ) + ( ( self.y - rhs.y ).powi(2) )
	}
	
	/* dbg function
	   Prints debug information of self to terminal */
	pub fn dbg( &self ) {
		dbg!( self );
	}
	
	/* dot function
	   Calculates the dot product between self and a passed in Vec2 */
	pub fn dot( &self, rhs: &Vec2 ) -> f64 {
		( self.x * rhs.x ) + ( self.y * rhs.y )
	}
	
	/* div function
	   Divides the components of self by a scalar value rhs */
	pub fn div( &mut self, rhs: &f64 ) {
		self.x /= rhs;
		self.y /= rhs;
	}
	
	/* lerp function
	   Linearly interpolates self with a passed in Vec2 */
	pub fn lerp( &mut self, rhs: &Vec2, amt: f64 ) {
		if amt <= 1.0 && amt >= 0.0 {
			self.x = -( amt - 1.0 ) * self.x + ( amt * rhs.x );
			self.y = -( amt - 1.0 ) * self.y + ( amt * rhs.y );
		} else {
			println!( "Interpolation amount passed in to function is not in the interval [0,1], so interpolation not calculated." );
		}
	}
	
	/* mag function
	   Calculates the magnitude of self */
	pub fn mag( &self ) -> f64 {
		( ( self.x ).powi(2) ).sqrt() + ( self.y ).powi(2)
	}
	
	/* magSq function
	   Calculates the magnitude squared of self */
	pub fn mag_sq( &self ) -> f64 {
		( self.x ).powi(2) + ( self.y ).powi(2)
	}
	
	/* mul function
	   Multiplies the components of self by a scalar value rhs */
	pub fn mult( &mut self, rhs: &f64 ) {
		self.x *= rhs;
		self.y *= rhs;
	}
	
	/* norm function
	   Normalizes the magnitude of self to 1, angle is unchanged */
	pub fn norm( &mut self ) {
		self.swap_system( POLAR );
		self.x = 1.0;
		self.swap_system( CARTESIAN );
	}
	
	/* rem function
	   Sets the components of self to the remainder of scalar division by rhs */
	pub fn rem( &mut self, rhs: &f64 ) {
		self.x %= rhs;
		self.y %= rhs;
	}
	
	/* rotate function
	   Rotates self by the angle entered */
	pub fn rotate( &mut self, theta: f64 ) {
		self.swap_system( POLAR );
		self.y += theta;
		self.swap_system( CARTESIAN );
	}
	
	/* set function
	   Sets components of vector to values entered */
	pub fn set( &mut self, input1: &f64, input2: &f64, system: &System ) {
		if *system == CARTESIAN {
			self.swap_system( CARTESIAN );
			self.x = *input1;
			self.y = *input2;
		} else {
			self.swap_system( POLAR );
			self.x = *input1;
			self.y = *input2;
			self.swap_system( CARTESIAN );
		}
	}
	
	/* set_mag function
	   Sets the magnitude of self */
	pub fn set_mag( &mut self, input: &f64 ) {
		self.swap_system( POLAR );
		self.x = *input;
		self.swap_system( CARTESIAN );
	}
	
	/* set_theta function
	   Sets the theta of self */
	pub fn set_theta( &mut self, input: &f64 ) {
		self.swap_system( POLAR );
		self.y = *input;
		self.swap_system( CARTESIAN );
	}
	
	/* sub function
	   Subtracts the components of the rhs Vec2 from the corresponding components of self */
	pub fn sub( &mut self, rhs: &Vec2 ) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
	
	/* system function
	   Returns the current coordinate system */
	pub fn system( &self ) -> System {
		self.system
	}
	
	/* swap_system function
	   Converts components from cartesian to polar coordinates, and vice versa */
	pub fn swap_system( &mut self, new_system: System ) {
		if self.system == POLAR && new_system == CARTESIAN {
			
			let x = self.x;
			self.x = ( (self.x).powi(2 ) + ( self.y ).powi(2 ) ).sqrt( );
			self.y = self.y.atan2(x );
			self.system = new_system;
			
		} else if self.system == CARTESIAN && new_system == POLAR {
			
			let theta = self.y;
			self.y = self.x * self.y.sin();
			self.x *= theta.cos();
			self.system = new_system;
		}
	}
	
	/* theta function
	   Calculates the theta of self */
	pub fn theta( &self ) -> f64 {
		( self.y ).atan2( self.x )
	}
}