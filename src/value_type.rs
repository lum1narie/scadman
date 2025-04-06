//! Types used to represent values in the library.

use ambassador::Delegate;
use derive_more::derive::From;
use nalgebra as na;

use crate::{
    common::Unit,
    scad_display::{ambassador_impl_ScadDisplay, ScadDisplay},
};

/// Vector representing an RGB color.
pub type RGB = na::Vector3<Unit>;
/// Vector representing an RGBA color.
pub type RGBA = na::Vector4<Unit>;

#[derive(Copy, Clone, Debug, PartialEq)]
/// Angle type for SCAD
pub enum Angle {
    /// Angle in degrees
    Deg(Unit),
    /// Angle in radians
    Rad(Unit),
}

impl Angle {
    /// Returns the angle in degrees
    ///
    /// # Returns
    ///
    /// The angle in degrees
    ///
    /// # Examples
    ///
    /// ```
    /// use scadman::{Unit, value_type::Angle};
    /// let d = Angle::Deg(90.0 as Unit);
    /// assert!((d.deg() - 90.0).abs() < 1e5);
    /// let r = Angle::Rad(std::f64::consts::PI as Unit / 2.);
    /// assert!((d.deg() - 90.0).abs() < 1e5);
    /// ```
    pub const fn deg(&self) -> Unit {
        match *self {
            Self::Deg(d) => d,
            Self::Rad(r) => r.to_degrees(),
        }
    }
}

impl ScadDisplay for Angle {
    fn repr_scad(&self) -> String {
        self.deg().repr_scad()
    }
}

#[derive(Clone, Debug, PartialEq, From, Delegate)]
#[delegate(ScadDisplay)]
/// Color type for SCAD
pub enum Color {
    /// Color in RGB format
    RGB(RGB),
    /// Color in RGBA format
    RGBA(RGBA),
    /// Named color
    Name(String),
}

impl ScadDisplay for RGBA {
    fn repr_scad(&self) -> String {
        format!(
            "[{}, {}, {}, {}]",
            self[0].repr_scad(),
            self[1].repr_scad(),
            self[2].repr_scad(),
            self[3].repr_scad()
        )
    }
}

impl Color {
    /// Returns the name of the key in SCAD code
    ///
    /// # Returns
    ///
    /// The name of the key in SCAD code
    pub const fn name(&self) -> &'static str {
        match *self {
            Self::Name(_) => "",
            _ => "c",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Delegate)]
#[delegate(ScadDisplay)]
/// Size of rounded shape type for SCAD
pub enum RoundSize {
    /// Radius of rounded shape
    Radius(Unit),
    /// Diameter of rounded shape
    Diameter(Unit),
}

impl RoundSize {
    /// Returns the name of the key in SCAD code
    ///
    /// # Returns
    ///
    /// The name of the key in SCAD code
    pub const fn name(&self) -> &'static str {
        match *self {
            Self::Radius(_) => "r",
            Self::Diameter(_) => "d",
        }
    }
}
