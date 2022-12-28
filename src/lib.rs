#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works( ) {
        /*let result = vector2::add(&2, &2 );
        assert_eq!( result, 4 );*/
    }
}

pub mod mathlib
{
    use Coordinates::*;
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

    impl Vec2 {
        fn new( ) -> Self {
            Self { x: 0.0, y: 0.0, system: Cartesian }
        }

        fn create( x: f64, y: f64, system: Coordinates ) -> Self {
            Self { x, y, system }
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

        fn get_system( &self ) -> &str {
            let system :&str = match self.system {
                Cartesian => "Cartesian",
                Polar => "Polar",
            };
            system
        }
    }
}
