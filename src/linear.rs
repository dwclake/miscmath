pub mod matrix;
pub mod vector;

/// The type of coordinate system used
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum CoordSystem {
    ///
    CARTESIAN,
    ///
    POLAR,
}

/// Unitary float, a f32 which must be in the closed interval [0.0,1.0]
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UnitF {
    ///
    value: f32,
}

impl UnitF {
    /// Generates a new instance of UnitF
    ///
    /// # Examples
    ///
    /// ```
    ///
    ///
    ///
    /// ```
    ///
    pub fn new(value: f32) -> UnitF {
        if (0.0..1.0).contains(&value) {
            UnitF { value }
        } else {
            UnitF {
                value: value.clamp(0.0, 1.0),
            }
        }
    }

    /// Returns the value contain by self
    ///
    /// # Examples
    ///
    /// ```
    ///
    ///
    ///
    /// ```
    ///
    pub fn value(&self) -> f32 {
        self.value
    }
}
