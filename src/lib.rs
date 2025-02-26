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
        any_scads, any_scads2d, any_scads3d, objects2d, objects3d,
        scad_2d::{
            Circle, Color2D, Difference2D, Hull2D, Import2D, Intersection2D, Minkowski2D, Mirror2D,
            MultMatrix2D, Objects2D, Offset, Polygon, Resize2D, Rotate2D, Scale2D, Square, Text,
            Translate2D, Union2D,
        },
        scad_3d::{
            Color3D, Cube, Cylinder, Difference3D, Hull3D, Import3D, Intersection3D, LinearExtrude,
            Minkowski3D, Mirror3D, MultMatrix3D, Objects3D, Polyhedron, Resize3D, Rotate3D,
            RotateExtrude, Scale3D, Sphere, Surface, Translate3D, Union3D,
        },
        value_type::{RGB, RGBA},
        AffineMatrix2D, AffineMatrix3D, Container2D, Container3D, Point2D, Point3D, ScadObject,
        ScadObject2D, ScadObject3D, Unit,
    };
}
