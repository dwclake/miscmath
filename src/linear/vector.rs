use super::*;
use super::CoordSystem::*;

use rand::Rng;
use std::ops::{Add, Div, Mul, Range, Sub};

/// A two dimensional mathematical vector
///
/// # Examples
///
/// ```
/// use miscmath::linear::vector::Vec2;
///
/// let a = Vec2::new();
///
/// assert!( ( a.x < 0.0000000001 ) && ( a.y < 0.0000000001 ) );
/// ```
///
#[derive( Copy, Clone, Debug )]
pub struct Vec2 {
	pub x: f32,
	pub y: f32,
	coord_system: CoordSystem,
}

/// Implements Add for Vec2
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
impl Add for Vec2 {
	type Output = Self;
	
	fn add( self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			coord_system: self.coord_system,
		}
	}
}

/// Implements Sub for Vec2
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
impl Sub for Vec2 {
	type Output = Self;
	
	fn sub( self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			coord_system: self.coord_system,
		}
	}
}

/// Implements Mul for Vec2
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
impl Mul for Vec2 {
	type Output = Self;
	
	fn mul( self, rhs: Self) -> Self::Output {
		Self {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
			coord_system: self.coord_system,
		}
	}
}

/// Implements Div for Vec2
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
impl Div for Vec2 {
	type Output = Self;
	
	fn div( self, rhs: Self) -> Self::Output {
		Self {
			x: self.x / rhs.x,
			y: self.y / rhs.y,
			coord_system: self.coord_system,
		}
	}
}

/// Implements PartialEq for Vec2
///
/// # Examples
///
/// ```
///  
///
///
/// ```
///
impl PartialEq for Vec2 {
	fn eq( &self, rhs: &Self ) -> bool {
		if (*self - *rhs).x.abs() < 0.0000000001  && (*self - *rhs).y.abs() < 0.0000000001 && self.coord_system == rhs.coord_system {
			true
		} else {
			false
		}
	}
}

impl Vec2 {
	
	/// Generates a new instance of Vec2 initialized to zero and returns it
	///
	/// # Examples
	///
	/// ```
	/// use miscmath::linear::vector::Vec2;
	///
	/// let a: Vec2 = Vec2::new();
	///
	/// assert!( ( a.x < 0.0000000001 ) && ( a.y < 0.0000000001 ) );
	/// ```
	///
	pub fn new( ) -> Vec2 {
		Vec2 { x: 0.0, y: 0.0,   coord_system: CARTESIAN }
	}
	
	/// Generates a new instance of Vec2 initialized to chosen values and returns it
	///
	/// # Examples
	///
	/// ```
	/// use miscmath::linear::{ CoordSystem::*, vector::Vec2 };
	///
	/// let a = Vec2::create( &5.6, &7.2 );
	///
	/// assert!( ( ( a.x - 5.6 ) < 0.0000000001 ) && ( ( a.y - 7.2 ) < 0.0000000001 ) );
	/// ```
	///
	pub fn create( x: &f32, y: &f32 ) -> Vec2 {
		let temp = Vec2 {
			x: *x,
			y: *y,
			coord_system: CARTESIAN,
		};
		temp
	}
	
	/// Generates a new instance of Vec2 initialized to random values in the range entered and returns it
	///
	/// # Examples
	///
	/// ```
	///  
	///
	/// 
	/// ```
	///
	pub fn from_rand_angle( range_x: &Range<f32>, mag: &Option<f32> ) -> Vec2 {
		let temp = Vec2::from_angle( &rand::thread_rng().gen_range( range_x.clone() ), mag );
		temp
	}
	
	/// Generates a new instance of Vec2 initialized to random values in the range entered and returns it
	///
	/// # Examples
	///
	/// ```
	///
	///
	///
	/// ```
	///
	pub fn create_random( range_x: &Range<f32> ) -> Vec2 {
		let temp = Vec2 {
			x: rand::thread_rng().gen_range( range_x.clone() ),
			y: rand::thread_rng().gen_range( range_x.clone() ),
			coord_system: CARTESIAN,
		};
		temp
	}
	
	/// Generates a new instance of Vec2 initialized to random values in the range entered and returns it
	///
	/// # Examples
	///
	/// ```
	///
	///
	///
	/// ```
	///
	pub fn create_random2( range_x: &Range<f32>, range_y: &Range<f32> ) -> Vec2 {
		let temp = Vec2 {
			x: rand::thread_rng().gen_range( range_x.clone() ),
			y: rand::thread_rng().gen_range( range_y.clone() ),
			coord_system: CARTESIAN,
		};
		temp
	}
	
	/// Generates a new instance of Vec2 based on a entered angle with a magnitude of 1
	///
	/// # Examples
	///
	/// ```
	///  
	///
	///   
	/// ```
	///
	pub fn from_angle( theta: &f32, in_mag: &Option<f32> ) -> Vec2 {
		if let Some(mag) = in_mag {
			let mut temp = Vec2 { x: *mag, y: *theta, coord_system: POLAR };
			temp.swap_system( CARTESIAN );
			temp
		} else {
			let mut temp = Vec2 { x: 1.0, y: *theta, coord_system: POLAR };
			temp.swap_system( CARTESIAN );
			temp
		}
	}
	
	/// Generates a new instance of Vec2 initialized to a magnitude of 1 and a angle of random value in the range entered and returns it
	///
	/// # Examples
	///
	/// ```
	/// use std::f32::consts::TAU;
	/// use miscmath::linear::{ CoordSystem::*, vector::Vec2 };
	///
	/// let mut a = Vec2::random_unit( &( 0.0..TAU ) );
	///
	/// assert!( ( ( a.x - 1.0 ) < 0.0000000001 ) && ( a.y < TAU ) );
	/// ```
	///
	pub fn random_unit( range: &Range<f32> ) -> Vec2 {
		let temp = Vec2::from_angle(&rand::thread_rng().gen_range( range.clone() ), &None );
		temp
	}
	
	/// Generates a new instance of Vec2 initialized as a unit vector of angle 0 and returns it
	///
	/// # Examples
	///
	/// ```
	/// use miscmath::linear::{ CoordSystem::*, vector::Vec2 };
	///
	/// let mut b = Vec2::unit();
	///
	/// assert!( ( b.x - 1.0 < 0.0000000001 ) && ( b.y < 0.0000000001 ) );
	/// ```
	///
	pub fn unit( ) -> Vec2 {
		let temp = Vec2::from_angle( &0.0, &Some(1.0));
		temp
	}
	
	/// Adds the components from the rhs Vec2 to the corresponding components of self
	///
	/// # Examples
	///
	/// ```
	///  
	///
	///   
	/// ```
	///
	pub fn add( &mut self, rhs: &Vec2 ) {
		self.swap_system( CARTESIAN );
		self.x += rhs.x;
		self.y += rhs.y;
	}
	
	/// Calculates the angle between self and a passed in Vec2
	///
	/// # Examples
	///
	/// ```
	/// use miscmath::linear::{ CoordSystem::*, vector::Vec2 };
	///
	/// let mut a = Vec2::create( &5.6, &7.2 );
	/// let mut b = Vec2::unit();
	///
	/// let angle = a.angle_between( &mut b );
	///
	/// assert!( ( angle - 1.4731481877 ) < 0.0000000001 );
	/// ```
	///
	pub fn angle_between( &self, rhs: &Vec2 ) -> f32 {
		let dot = self.dot( &rhs );
		( dot / ( self.mag() * rhs.mag() ) ).acos()
	}
	
	/// Generates a new instance of Vec2 which is perpendicular to the self instance
	///
	/// # Examples
	///
	/// ```
	///  
	///
	///   
	/// ```
	///
	pub fn cross( &mut self ) {
		self.swap_system( CARTESIAN );
		let x = self.x;
		self.x = self.y;
		self.y = -x;
	}
	
	/// Calculates the distance between self and the Vec2 passed in
	///
	/// # Examples
	///
	/// ```
	///  
	///
	///   
	/// ```
	///
	pub fn dist( &self, rhs: &Vec2 ) -> f32 {
		( ( self.x - rhs.x ).powi(2) ) + ( ( self.y - rhs.y ).powi(2) )
	}
	
	/// Prints debug information of self to terminal
	///
	/// # Examples
	///
	/// ```
	///  
	///
	///   
	/// ```
	///
	pub fn dbg( &self ) {
		dbg!( self );
	}
	
	/// Calculates the dot product between self and a passed in Vec2
	///
	/// # Examples
	///
	/// ```
	///  
	///
	///   
	/// ```
	///
	pub fn dot( &self, rhs: &Vec2 ) -> f32 {
		( self.x * rhs.x ) + ( self.y * rhs.y )
	}
	
	/// Divides the components of self by a scalar value rhs
	///
	/// # Examples
	///
	/// ```
	///  
	///
	///   
	/// ```
	///
	pub fn div( &mut self, rhs: &f32 ) {
		self.x /= rhs;
		self.y /= rhs;
	}
	
	/// Linearly interpolates between self and a passed in Vec2
	///
	/// # Examples
	///
	/// ```
	/// use miscmath::linear::{ CoordSystem::*, vector::Vec2, UnitF };
	///
	///	let mut a = Vec2::create( &5.6, &7.2 );
	/// let mut b = Vec2::unit();
	///
	/// a.lerp( &b, UnitF::new( 0.5 ) );
	///
	/// assert!( ( ( a.x - 3.3 ) < 0.0000000001 ) && ( ( a.y - 3.6 ) < 0.0000000001 ) );
	/// ```
	///
	pub fn lerp( &mut self, rhs: &Vec2, amt: UnitF ) {
		self.swap_system( CARTESIAN );
		self.x = -( amt.value() - 1.0 ) * self.x + ( amt.value() * rhs.x );
		self.y = -( amt.value() - 1.0 ) * self.y + ( amt.value() * rhs.y );
	}
	
	/// Calculates the magnitude of self
	///
	/// # Examples
	///
	/// ```
	///  
	///
	///   
	/// ```
	///
	pub fn mag( &self ) -> f32 {
		( ( self.x ).powi(2) ).sqrt() + ( self.y ).powi(2)
	}
	
	/// Calculates the magnitude squared of self
	///
	/// # Examples
	///
	/// ```
	///  
	///
	///   
	/// ```
	///
	pub fn mag_sq( &self ) -> f32 {
		( self.x ).powi(2) + ( self.y ).powi(2)
	}
	
	/// Multiplies the components of self by a scalar value rhs
	///
	/// # Examples
	///
	/// ```
	///  
	///
	///   
	/// ```
	///
	pub fn mult( &mut self, rhs: &f32 ) {
		self.swap_system( CARTESIAN );
		self.x *= rhs;
		self.y *= rhs;
	}
	
	/// Normalizes the magnitude of self to 1, angle is unchanged
	///
	/// # Examples
	///
	/// ```
	///  
	///
	///   
	/// ```
	///
	pub fn norm( &mut self ) {
		self.swap_system( POLAR );
		self.x = 1.0;
		self.swap_system( CARTESIAN );
	}
	
	/// Sets the components of self to the remainder of scalar division by rhs
	///
	/// # Examples
	///
	/// ```
	///  
	///
	///   
	/// ```
	///
	pub fn rem( &mut self, rhs: &f32 ) {
		self.swap_system( CARTESIAN );
		self.x %= rhs;
		self.y %= rhs;
	}
	
	/// Rotates self by the angle entered
	///
	/// # Examples
	///
	/// ```
	///  
	///
	///   
	/// ```
	///
	pub fn rotate( &mut self, theta: f32 ) {
		self.swap_system( POLAR );
		self.y += theta;
		self.swap_system( CARTESIAN );
	}
	
	/// Sets components of vector to values entered
	///
	/// # Examples
	///
	/// ```
	///  
	///
	///   
	/// ```
	///
	pub fn set( &mut self, input1: &f32, input2: &f32,   coord_system: &CoordSystem ) {
		if *coord_system == CARTESIAN {
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
	
	/// Sets the magnitude of self
	///
	/// # Examples
	///
	/// ```
	///  
	///
	///   
	/// ```
	///
	pub fn set_mag( &mut self, input: &f32 ) {
		self.swap_system( POLAR );
		self.x = *input;
		self.swap_system( CARTESIAN );
	}
	
	/// Sets the theta of self
	///
	/// # Examples
	///
	/// ```
	///  
	///
	///   
	/// ```
	///
	pub fn set_theta( &mut self, input: &f32 ) {
		self.swap_system( POLAR );
		self.y = *input;
		self.swap_system( CARTESIAN );
	}
	
	/// Subtracts the components of the rhs Vec2 from the corresponding components of self
	///
	/// # Examples
	///
	/// ```
	///  
	///
	///   
	/// ```
	///
	pub fn sub( &mut self, rhs: &Vec2 ) {
		self.swap_system( CARTESIAN );
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
	
	/// Returns the current coordinate system
	///
	/// # Examples
	///
	/// ```
	///  
	///
	///   
	/// ```
	///
	pub fn system( &self ) -> CoordSystem {
		self .coord_system
	}
	
	/// Converts components from cartesian to polar coordinates, and vice versa
	///
	/// # Examples
	///
	/// ```
	///  
	///
	///   
	/// ```
	///
	fn swap_system( &mut self, new_coord_system: CoordSystem ) {
		if self .coord_system == CARTESIAN &&  new_coord_system == POLAR {
			
			let x = self.x;
			self.x = ( (self.x).powi(2 ) + ( self.y ).powi(2 ) ).sqrt( );
			self.y = self.y.atan2(x );
			self .coord_system =  new_coord_system;
			
		} else if self .coord_system == POLAR &&  new_coord_system == CARTESIAN {
			
			let theta = self.y;
			self.y = self.x * self.y.sin();
			self.x *= theta.cos();
			self .coord_system =  new_coord_system;
		}
	}
	
	/// Calculates the theta of self
	///
	/// # Examples
	///
	/// ```
	///  
	///
	///   
	/// ```
	///
	pub fn theta( &self ) -> f32 {
		( self.y ).atan2( self.x )
	}
}