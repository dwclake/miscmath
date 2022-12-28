#[cfg(test)]
mod vec2_tests {
	//use super::*;

	#[test]
	fn it_works( ) {
		/*let result = vector2::add(&2, &2 );
		assert_eq!( result, 4 );*/
		println!( "test" );
	}
}

pub mod vec2
{
	use rand::Rng;
	use std::cmp::Ordering;
	use raylib::ffi::system;

	/*pub fn add(left: &usize, right: &usize) -> usize {
		let angle: f32 = 1.67;
		let mut _x: Vec<usize> = Vec::new( );
		_x.push(1);
		_x.push( *left );
		_x.push( *right );
		_x.push(angle.cos() as usize );
		_x.pop( );
		left + right
	}*/
	#[derive(Debug)]
	#[derive(PartialEq)]
	enum Coordinates {
		Cartesian,
		Polar,
	}

	#[derive(Debug)]
	struct Vec2{
		x: f64,
		y: f64,
		system: Coordinates,
	}

	use Coordinates::*;

	impl Vec2 {
		fn new( ) -> Self {
			Self { x: 0.0, y: 0.0, system: Cartesian }
		}

		fn create( x: f64, y: f64, system: Coordinates ) -> Self {
			Self { x, y, system }
		}

		fn create_random( min: f64, max: f64, system: Coordinates ) -> Self {
			Self {
				x: rand::thread_rng().gen_range( min..max ),
				y: rand::thread_rng().gen_range( min..max ),
				system,
			}
		}

		fn random_unit( min: f64, max: f64 ) -> Self {
			let mut temp= Vec2::create( 1.0, rand::thread_rng().gen_range( min..max ), Polar );
			temp.swap_system( Cartesian );
			temp
		}

		fn unit( ) -> Self {
			Self { x: 1.0, y: 1.0, system: Cartesian }
		}

		fn add( &mut self, rhs: &Vec2 ) /*-> Vec2*/ {
			//let mut temp = Vec2::new( );
			self.x = self.x + rhs.x;
			self.y = self.y + rhs.y;
			/*temp.x = self.x;
			temp.y = self.y;
			temp*/
		}

		fn dbg( &self, name: &str ) {
			println!( "{name}: " );
			dbg!( self );
		}

		fn div( &mut self, rhs: &f64 ) {
			self.x = self.x / rhs;
			self.y = self.y / rhs;
		}

		fn get_system( &self ) -> &str {
			let system :&str = match self.system {
				Cartesian => "Cartesian",
				Polar => "Polar",
			};
			system
		}

		fn mult( &mut self, rhs: &f64 ) {
			self.x = self.x * rhs;
			self.y = self.y * rhs;
		}

		fn norm( &mut self ) {
			self.swap_system( Polar );
			self.x = 1.0;
			self.swap_system( Cartesian );
		}

		fn rem( &mut self, rhs: &f64 ) {
			self.x = self.x % rhs;
			self.y = self.y % rhs;
		}

		fn sub( &mut self, rhs: &Vec2 ) {
			self.x = self.x - rhs.x;
			self.y = self.y - rhs.y;
		}

		fn swap_system( &mut self, new_system: Coordinates ) {
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
}