use super::CoordSystem::*;
use super::*;

use rand::Rng;
use std::ops::{Add, Div, Mul, Range, Sub, AddAssign, SubAssign, MulAssign, DivAssign, Rem, RemAssign};

/// A two dimensional mathematical vector
///
/// # Examples
///
/// ```
/// use miscmath::prelude::*;
///
/// let a = vector::Vec2::new();
///
/// assert!( ( a.x < 0.00001 ) && ( a.y < 0.00001 ) );
/// ```
///
#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    ///
    pub x: f32,
    ///
    pub y: f32,
    ///
    coord_system: CoordSystem,
}

/// Implements Default for Vec2
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
impl Default for Vec2 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            coord_system: CARTESIAN,
        }
    }
}

/// Implements Add for Vec2
///
/// # Examples
///
/// ```
/// use miscmath::prelude::*;
/// 
/// let mut x = vector::Vec2::new();
/// let y = vector::Vec2::create( &5.0, &7.0);
/// x += y;
///
/// assert_eq!(x, y);
/// ```
///
impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            coord_system: self.coord_system,
        }
    }
}

/// Implements AddAssign for Vec2
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
impl AddAssign for Vec2 {

    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
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

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            coord_system: self.coord_system,
        }
    }
}

/// Implements SubAssign for Vec2
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
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

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            coord_system: self.coord_system,
        }
    }
}

/// Implements MulAssign for Vec2
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
impl MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
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

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            coord_system: self.coord_system,
        }
    }
}

/// Implements DivAssign for Vec2
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
impl DivAssign for Vec2 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

/// Implements Rem for Vec2
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
impl Rem for Vec2 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Vec2{
            x: self.x % rhs.x,
            y: self.y % rhs.y,
            coord_system: CARTESIAN,
        }
    }
}

/// Implements RemAssign for Vec2
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
impl RemAssign for Vec2 {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
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
    ///
    fn eq(&self, rhs: &Self) -> bool {
        if (*self - *rhs).x.abs() < 0.00001
            && (*self - *rhs).y.abs() < 0.00001
            && self.coord_system == rhs.coord_system
        {
            true
        } else {
            false
        }
    }

    ///
    fn ne(&self, rhs: &Self) -> bool {
        if (*self - *rhs).x.abs() > 0.00001 && (*self - *rhs).y.abs() > 0.00001 {
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
    /// use miscmath::prelude::*;
    ///
    /// let a = vector::Vec2::new();
    ///
    /// assert!( ( a.x < 0.00001 ) && ( a.y < 0.00001 ) );
    /// ```
    ///
    pub fn new() -> Vec2 {
        Vec2 {
            x: 0.0,
            y: 0.0,
            coord_system: CARTESIAN,
        }
    }

    /// Generates a new instance of Vec2 initialized to chosen values and returns it
    ///
    /// # Examples
    ///
    /// ```
    /// use miscmath::prelude::*;
    ///
    /// let a = vector::Vec2::create( &5.6, &7.2 );
    ///
    /// assert!( ( ( a.x - 5.6 ) < 0.00001 ) && ( ( a.y - 7.2 ) < 0.00001 ) );
    /// ```
    ///
    pub fn create(x: &f32, y: &f32) -> Vec2 {
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
    pub fn create_random(range_x: &Range<f32>) -> Vec2 {
        let temp = Vec2 {
            x: rand::thread_rng().gen_range(range_x.clone()),
            y: rand::thread_rng().gen_range(range_x.clone()),
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
    pub fn create_random2(range_x: &Range<f32>, range_y: &Range<f32>) -> Vec2 {
        let temp = Vec2 {
            x: rand::thread_rng().gen_range(range_x.clone()),
            y: rand::thread_rng().gen_range(range_y.clone()),
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
    pub fn from_angle(theta: &f32, in_mag: &Option<f32>) -> Vec2 {
        if let Some(mag) = in_mag {
            let mut temp = Vec2 {
                x: *mag,
                y: *theta,
                coord_system: POLAR,
            };
            temp.swap_system(CARTESIAN);
            temp
        } else {
            let mut temp = Vec2 {
                x: 1.0,
                y: *theta,
                coord_system: POLAR,
            };
            temp.swap_system(CARTESIAN);
            temp
        }
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
    pub fn from_rand_angle(range_x: &Range<f32>, mag: &Option<f32>) -> Vec2 {
        let temp = Vec2::from_angle(&rand::thread_rng().gen_range(range_x.clone()), mag);
        temp
    }

    /// Generates a new instance of Vec2 initialized to a magnitude of 1 and a angle of random value in the range entered and returns it
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f32::consts::TAU;
    /// use miscmath::prelude::*;
    ///
    /// let mut a = vector::Vec2::random_unit( &( 0.0..TAU ) );
    ///
    /// assert!( ( ( a.x - 1.0 ) < 0.00001 ) && ( a.y < TAU ) );
    /// ```
    ///
    pub fn random_unit(range: &Range<f32>) -> Vec2 {
        let temp = Vec2::from_angle(&rand::thread_rng().gen_range(range.clone()), &None);
        temp
    }

    /// Generates a new instance of Vec2 initialized as a unit vector of angle 0 and returns it
    ///
    /// # Examples
    ///
    /// ```
    /// use miscmath::prelude::*;
    ///
    /// let mut b = vector::Vec2::unit();
    ///
    /// assert!( ( b.x - 1.0 < 0.00001 ) && ( b.y < 0.00001 ) );
    /// ```
    ///
    pub fn unit() -> Vec2 {
        let temp = Vec2::from_angle(&0.0, &Some(1.0));
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
    pub fn add(&mut self, rhs: &Vec2) {
        self.swap_system(CARTESIAN);
        self.x += rhs.x;
        self.y += rhs.y;
    }

    /// Calculates the angle between self and a passed in Vec2
    ///
    /// # Examples
    ///
    /// ```
    /// use miscmath::prelude::*;
    ///
    /// let mut a = vector::Vec2::create( &5.6, &7.2 );
    /// let mut b = vector::Vec2::unit();
    ///
    /// let angle = a.angle_between( &mut b );
    ///
    /// assert!( ( angle - 1.4731481877 ) < 0.00001 );
    /// ```
    ///
    pub fn angle_between(&self, rhs: &Vec2) -> f32 {
        let dot = self.dot(&rhs);
        (dot / (self.mag() * rhs.mag())).acos()
    }

    ///
    ///
    /// # Examples
    ///
    /// ```
    ///
    ///
    ///
    /// ```
    ///
    pub fn constrain(&mut self, x_rng: &Range<f32>, y_rng: &Range<f32>) {
        self.x = self.x.clamp(x_rng.start, x_rng.end);
        self.y = self.y.clamp(y_rng.start, y_rng.end);
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
    pub fn cross(&mut self) {
        self.swap_system(CARTESIAN);
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
    pub fn dist(&self, rhs: &Vec2) -> f32 {
        ((self.x - rhs.x).powf(2.0)) + ((self.y - rhs.y).powf(2.0)).sqrt()
    }

    /// Calculates the distance squared between self and the Vec2 passed in
    ///
    /// # Examples
    ///
    /// ```
    ///  
    ///
    ///   
    /// ```
    ///
    pub fn dist_sq(&self, rhs: &Vec2) -> f32 {
        ((self.x - rhs.x).powf(2.0)) + ((self.y - rhs.y).powf(2.0))
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
    pub fn dbg(&self) {
        dbg!(self);
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
    pub fn dot(&self, rhs: &Vec2) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y)
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
    pub fn div(&mut self, rhs: &f32) {
        self.x /= rhs;
        self.y /= rhs;
    }

    /// Linearly interpolates between self and a passed in Vec2
    ///
    /// # Examples
    ///
    /// ```
    /// use miscmath::prelude::*;
    ///
    ///	let mut a = vector::Vec2::create( &5.6, &7.2 );
    /// let mut b = vector::Vec2::unit();
    ///
    /// a.lerp( &b, UnitF::new( 0.5 ) );
    ///
    /// assert!( ( ( a.x - 3.3 ) < 0.00001 ) && ( ( a.y - 3.6 ) < 0.00001 ) );
    /// ```
    ///
    pub fn lerp(&mut self, rhs: &Vec2, amt: UnitF) {
        self.swap_system(CARTESIAN);
        self.x = -(amt.value() - 1.0) * self.x + (amt.value() * rhs.x);
        self.y = -(amt.value() - 1.0) * self.y + (amt.value() * rhs.y);
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
    pub fn mag(&self) -> f32 {
        ((self.x).powf(2.0) + (self.y).powf(2.0)).sqrt()
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
    pub fn mag_sq(&self) -> f32 {
        (self.x).powf(2.0) + (self.y).powf(2.0)
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
    pub fn mult(&mut self, rhs: &f32) {
        self.swap_system(CARTESIAN);
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
    pub fn norm(&mut self) {
        self.swap_system(POLAR);
        self.x = 1.0;
        self.swap_system(CARTESIAN);
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
    pub fn rem(&mut self, rhs: &f32) {
        self.swap_system(CARTESIAN);
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
    pub fn rotate(&mut self, theta: f32) {
        self.swap_system(POLAR);
        self.y += theta;
        self.swap_system(CARTESIAN);
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
    pub fn set(&mut self, input1: &f32, input2: &f32, coord_system: &CoordSystem) {
        if *coord_system == CARTESIAN {
            self.swap_system(CARTESIAN);
            self.x = *input1;
            self.y = *input2;
        } else {
            self.swap_system(POLAR);
            self.x = *input1;
            self.y = *input2;
            self.swap_system(CARTESIAN);
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
    pub fn set_mag(&mut self, input: &f32) {
        self.swap_system(POLAR);
        self.x = *input;
        self.swap_system(CARTESIAN);
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
    pub fn set_theta(&mut self, input: &f32) {
        self.swap_system(POLAR);
        self.y = *input;
        self.swap_system(CARTESIAN);
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
    pub fn sub(&mut self, rhs: &Vec2) {
        self.swap_system(CARTESIAN);
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
    pub fn system(&self) -> CoordSystem {
        self.coord_system
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
    fn swap_system(&mut self, new_coord_system: CoordSystem) {
        if self.coord_system == CARTESIAN && new_coord_system == POLAR {
            let x = self.x;
            self.x = (self.x.powi(2) + self.y.powi(2)).sqrt();
            self.y = self.y.atan2(x);
            self.coord_system = new_coord_system;
        } else if self.coord_system == POLAR && new_coord_system == CARTESIAN {
            let theta = self.y;
            self.y = self.x * self.y.sin();
            self.x *= theta.cos();
            self.coord_system = new_coord_system;
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
    pub fn theta(&self) -> f32 {
        (self.y).atan2(self.x)
    }
}

/// A three dimensional mathematical vector
///
/// # Examples
///
/// ```
/// use miscmath::prelude::*;
///
/// let a = vector::Vec3::new();
///
/// assert!( ( a.x < 0.00001 ) && ( a.y < 0.00001 ) && ( a.z < 0.00001 ) );
/// ```
///
#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    ///
    pub x: f32,
    ///
    pub y: f32,
    ///
    pub z: f32,
    ///
    coord_system: CoordSystem,
}

/// Implements Default for Vec3
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            coord_system: CARTESIAN,
        }
    }
}

/// Implements Add for Vec3
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            coord_system: self.coord_system,
        }
    }
}

/// Implements AddAssign for Vec3
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

/// Implements Sub for Vec3
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            coord_system: self.coord_system,
        }
    }
}

/// Implements SubAssign for Vec3
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

/// Implements Mul for Vec3
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            coord_system: self.coord_system,
        }
    }
}

/// Implements MulAssign for Vec3
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

/// Implements Div for Vec3
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            coord_system: self.coord_system,
        }
    }
}

/// Implements DivAssign for Vec3
///
/// # Examples
///
/// ```
/// use miscmath::prelude::*;
/// 
/// let mut x = vector::Vec2::create( &100.0, &50.0 );
/// let y = vector::Vec2::create( &2.0, &5.0 );
/// let z = vector::Vec2::create( &50.0, &10.0 );
///
/// x /= y;
/// 
/// assert_eq!( x, z );
/// ```
///
impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

/// Implements Rem for Vec3
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
impl Rem for Vec3 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
            z: self.z % rhs.z,
            coord_system: self.coord_system,
        }
    }
}

/// Implements RemAssign for Vec3
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
impl RemAssign for Vec3 {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

/// Implements PartialEq for Vec3
///
/// # Examples
///
/// ```
///  
///
///
/// ```
///
impl PartialEq for Vec3 {
    ///
    fn eq(&self, rhs: &Self) -> bool {
        let temp = *self - *rhs;
        if temp.x.abs() < 0.00001
            && temp.y.abs() < 0.00001
            && temp.z.abs() < 0.00001
            && self.coord_system == rhs.coord_system
        {
            true
        } else {
            false
        }
    }

    ///
    fn ne(&self, rhs: &Self) -> bool {
        let temp = *self - *rhs;
        if temp.x.abs() > 0.00001 || temp.y.abs() > 0.00001 || temp.z.abs() > 0.00001 {
            true
        } else {
            false
        }
    }
}

impl Vec3 {
    /// Generates a new instance of Vec3 initialized to zero and returns it
    ///
    /// # Examples
    ///
    /// ```
    /// use miscmath::prelude::*;
    ///
    /// let a = vector::Vec3::new();
    ///
    /// assert!( ( a.x < 0.000001 ) && ( a.y < 0.000001 ) && ( a.z < 0.00001 ) );
    /// ```
    ///
    pub fn new() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            coord_system: CARTESIAN,
        }
    }

    /// Generates a new instance of Vec3 initialized to chosen values and returns it
    ///
    /// # Examples
    ///
    /// ```
    /// use miscmath::prelude::*;
    ///
    /// let a = vector::Vec3::create( &5.6, &7.2, &6.8 );
    ///
    /// assert!( ( ( a.x - 5.6 ) < 0.000001 ) && ( ( a.y - 7.2 ) < 0.000001 ) && ( ( a.z - 6.8 ) < 0.00001 ) );
    /// ```
    ///
    pub fn create(x: &f32, y: &f32, z: &f32) -> Vec3 {
        let temp = Vec3 {
            x: *x,
            y: *y,
            z: *z,
            coord_system: CARTESIAN,
        };
        temp
    }

    /// Generates a new instance of Vec3 initialized to random values in the range entered and returns it
    ///
    /// # Examples
    ///
    /// ```
    ///
    ///
    ///
    /// ```
    ///
    pub fn create_random(range_x: &Range<f32>) -> Vec3 {
        let temp = Vec3 {
            x: rand::thread_rng().gen_range(range_x.clone()),
            y: rand::thread_rng().gen_range(range_x.clone()),
            z: rand::thread_rng().gen_range(range_x.clone()),
            coord_system: CARTESIAN,
        };
        temp
    }

    /// Generates a new instance of Vec3 initialized to random values in the range entered and returns it
    ///
    /// # Examples
    ///
    /// ```
    ///
    ///
    ///
    /// ```
    ///
    pub fn create_random3(
        range_x: &Range<f32>,
        range_y: &Range<f32>,
        range_z: &Range<f32>,
    ) -> Vec3 {
        let temp = Vec3 {
            x: rand::thread_rng().gen_range(range_x.clone()),
            y: rand::thread_rng().gen_range(range_y.clone()),
            z: rand::thread_rng().gen_range(range_z.clone()),
            coord_system: CARTESIAN,
        };
        temp
    }

    /// Generates a new instance of Vec3 based on a entered angles with a magnitude of 1
    ///
    /// # Examples
    ///
    /// ```
    ///  
    ///
    ///   
    /// ```
    ///
    pub fn from_angle(theta: &f32, phi: &f32, in_mag: &Option<f32>) -> Vec3 {
        if let Some(mag) = in_mag {
            let mut temp = Vec3 {
                x: *mag,
                y: *theta,
                z: *phi,
                coord_system: POLAR,
            };
            temp.swap_system(CARTESIAN);
            temp
        } else {
            let mut temp = Vec3 {
                x: 1.0,
                y: *theta,
                z: *phi,
                coord_system: POLAR,
            };
            temp.swap_system(CARTESIAN);
            temp
        }
    }

    /// Generates a new instance of Vec3 initialized to random values in the range entered and returns it
    ///
    /// # Examples
    ///
    /// ```
    ///
    ///
    ///
    /// ```
    ///
    pub fn from_rand_angle(range: &Range<f32>, mag: &Option<f32>) -> Vec3 {
        let temp = Vec3::from_angle(
            &rand::thread_rng().gen_range(range.clone()),
            &rand::thread_rng().gen_range(range.clone()),
            mag,
        );
        temp
    }

    /// Generates a new instance of Vec3 initialized to a magnitude of 1 and angles of random value in the range entered and returns it
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f32::consts::TAU;
    /// use miscmath::prelude::*;
    ///
    /// let mut a = vector::Vec3::random_unit( &( 0.0..TAU ) );
    ///
    /// assert!( ( a.x - 1.0 < 0.00001 ) && ( a.y < TAU ) && ( a.z < TAU ) );
    /// ```
    ///
    pub fn random_unit(range: &Range<f32>) -> Vec3 {
        let temp = Vec3::from_angle(
            &rand::thread_rng().gen_range(range.clone()),
            &rand::thread_rng().gen_range(range.clone()),
            &None,
        );
        temp
    }

    /// Generates a new instance of Vec3 initialized as a unit vector with angles of 0 and returns it
    ///
    /// # Examples
    ///
    /// ```
    /// use miscmath::prelude::*;
    ///
    /// let mut b = vector::Vec3::unit();
    ///
    /// assert!( ( b.x < 0.00001 ) && ( b.y < 0.00001 ) && ( b.z - 1.0 < 0.0001 ) );
    /// ```
    ///
    pub fn unit() -> Vec3 {
        let temp = Vec3::from_angle(&0.0, &0.0, &Some(1.0));
        temp
    }

    /// Adds the components from the rhs Vec3 to the corresponding components of self
    ///
    /// # Examples
    ///
    /// ```
    ///  
    ///
    ///   
    /// ```
    ///
    pub fn add(&mut self, rhs: &Vec3) {
        self.swap_system(CARTESIAN);
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }

    /// Calculates the angle between self and a passed in Vec3
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    ///
    pub fn angle_between(&self, rhs: &Vec3) -> f32 {
        let dot = self.dot(&rhs);
        (dot / (self.mag() * rhs.mag())).acos()
    }

    ///
    ///
    /// # Examples
    ///
    /// ```
    ///
    ///
    ///
    /// ```
    ///
    pub fn constrain(&mut self, x_rng: &Range<f32>, y_rng: &Range<f32>, z_rng: &Range<f32>) {
        self.x = self.x.clamp(x_rng.start, x_rng.end);
        self.y = self.y.clamp(y_rng.start, y_rng.end);
        self.z = self.z.clamp(z_rng.start, z_rng.end);
    }

    /// Generates a new instance of Vec3 which is perpendicular to the self instance
    ///
    /// # Examples
    ///
    /// ```
    ///  
    ///
    ///   
    /// ```
    ///
    pub fn cross(&mut self, rhs: &Vec3) -> Vec3 {
        self.swap_system(CARTESIAN);

        let x = (self.y * rhs.z) - (self.z * rhs.y);
        let y = (self.z * rhs.x) - (self.x * rhs.z);
        let z = (self.x * rhs.y) - (self.y * rhs.x);

        Vec3 {
            x,
            y,
            z,
            coord_system: CARTESIAN,
        }
    }

    /// Calculates the distance between self and the Vec3 passed in
    ///
    /// # Examples
    ///
    /// ```
    ///
    ///
    ///
    /// ```
    ///
    pub fn dist(&self, rhs: &Vec3) -> f32 {
        let distance = ((self.x - rhs.x).powf(2.0))
            + ((self.y - rhs.y).powf(2.0))
            + ((self.z - rhs.z).powf(2.0));

        distance.sqrt()
    }

    /// Calculates the distance squared between self and the Vec3 passed in
    ///
    /// # Examples
    ///
    /// ```
    ///  
    ///
    ///   
    /// ```
    ///
    pub fn dist_sq(&self, rhs: &Vec3) -> f32 {
        let distance = ((self.x - rhs.x).powf(2.0))
            + ((self.y - rhs.y).powf(2.0))
            + ((self.z - rhs.z).powf(2.0));

        distance
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
    pub fn dbg(&self) {
        dbg!(self);
    }

    /// Calculates the dot product between self and a passed in Vec3
    ///
    /// # Examples
    ///
    /// ```
    ///  
    ///
    ///   
    /// ```
    ///
    pub fn dot(&self, rhs: &Vec3) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
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
    pub fn div(&mut self, rhs: &f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }

    /// Linearly interpolates between self and a passed in Vec3
    ///
    /// # Examples
    ///
    /// ```
    /// use miscmath::prelude::*;
    ///
    ///	let mut a = vector::Vec3::create( &5.6, &7.2, &6.8 );
    /// let mut b = vector::Vec3::new();
    ///
    /// a.lerp( &b, UnitF::new( 0.5 ) );
    ///
    /// assert!( ( ( a.x - 2.8 ) < 0.00001 ) && ( ( a.y - 3.6 ) < 0.00001 ) && ( a.z - 3.4 ) < 0.00001 );
    /// ```
    ///
    pub fn lerp(&mut self, rhs: &Vec3, amt: UnitF) {
        self.swap_system(CARTESIAN);

        self.x = -(amt.value() - 1.0) * self.x + (amt.value() * rhs.x);
        self.y = -(amt.value() - 1.0) * self.y + (amt.value() * rhs.y);
        self.z = -(amt.value() - 1.0) * self.z + (amt.value() * rhs.z);
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
    pub fn mag(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
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
    pub fn mag_sq(&self) -> f32 {
        self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)
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
    pub fn mult(&mut self, rhs: &f32) {
        self.swap_system(CARTESIAN);
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
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
    pub fn norm(&mut self) {
        self.swap_system(POLAR);
        self.x = 1.0;
        self.swap_system(CARTESIAN);
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
    pub fn phi(&self) -> f32 {
        self.x * (self.y).cos()
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
    pub fn rem(&mut self, rhs: &f32) {
        self.swap_system(CARTESIAN);
        self.x %= rhs;
        self.y %= rhs;
        self.z %= rhs;
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
    pub fn rotate(&mut self, theta: &f32, phi: &f32) {
        self.swap_system(POLAR);
        self.y += *theta;
        self.z += *phi;
        self.swap_system(CARTESIAN);
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
    pub fn set(&mut self, input1: &f32, input2: &f32, input3: &f32, coord_system: &CoordSystem) {
        if *coord_system == CARTESIAN {
            self.swap_system(CARTESIAN);
            self.x = *input1;
            self.y = *input2;
            self.z = *input3;
        } else {
            self.swap_system(POLAR);
            self.x = *input1;
            self.y = *input2;
            self.z = *input3;
            self.swap_system(CARTESIAN);
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
    pub fn set_mag(&mut self, input: &f32) {
        self.swap_system(POLAR);
        self.x = *input;
        self.swap_system(CARTESIAN);
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
    pub fn set_theta(&mut self, input: &f32) {
        self.swap_system(POLAR);
        self.y = *input;
        self.swap_system(CARTESIAN);
    }

    /// Sets the phi of self
    ///
    /// # Examples
    ///
    /// ```
    ///  
    ///
    ///   
    /// ```
    ///
    pub fn set_phi(&mut self, input: &f32) {
        self.swap_system(POLAR);
        self.z = *input;
        self.swap_system(CARTESIAN);
    }

    /// Subtracts the components of the rhs Vec3 from the corresponding components of self
    ///
    /// # Examples
    ///
    /// ```
    ///  
    ///
    ///   
    /// ```
    ///
    pub fn sub(&mut self, rhs: &Vec3) {
        self.swap_system(CARTESIAN);
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
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
    pub fn system(&self) -> CoordSystem {
        self.coord_system
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
    fn swap_system(&mut self, new_coord_system: CoordSystem) {
        if self.coord_system == CARTESIAN && new_coord_system == POLAR {
            let x = self.x;
            let y = self.y;

            self.x = (self.x.powf(2.0) + self.y.powf(2.0)).sqrt();
            self.y = self.z.atan2(x);
            self.z = y.atan2(x);
            self.coord_system = new_coord_system;
        } else if self.coord_system == POLAR && new_coord_system == CARTESIAN {
            let mag = self.x;
            let theta = self.y;

            self.y = self.x * self.y.sin() * self.z.cos();
            self.x = self.x * theta.sin() * self.z.sin();
            self.z = mag * theta.cos();
            self.coord_system = new_coord_system;
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
    pub fn theta(&self) -> f32 {
        (self.z).atan2(self.x)
    }
}