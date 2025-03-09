//! A library for generating OpenSCAD code.

pub(crate) mod internal;
mod macros;

mod common;
pub use common::*;

pub mod scad_display;
pub mod value_type;

pub mod scad_2d;
pub mod scad_3d;

/// import `prelude::*` so you can be ready to code!
pub mod prelude {
    pub use crate::{
        scad_2d::ScadObject2D,
        scad_3d::ScadObject3D,
        value_type::{RGB, RGBA},
        AffineMatrix2D, AffineMatrix3D, Container2D, Container3D, Point2D, Point3D,
        ScadObjectTrait, Unit,
    };
}
