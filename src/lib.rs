//! A library for generating OpenSCAD code.

pub(crate) mod internal;
mod macros;

#[allow(clippy::redundant_pub_crate)]
mod common;
pub use common::*;

pub mod scad_display;
pub mod value_type;

pub mod scad_2d;
pub mod scad_3d;
pub mod scad_mixed;

pub mod scad_sentence;

/// import `prelude::*` so you can be ready to code!
pub mod prelude {
    // TODO:
    pub use crate::{
        scad_2d::ScadObject2D,
        scad_3d::ScadObject3D,
        scad_sentence::{
            Circle, Color, Cube, Cylinder, Difference, Hull, Import2D, Import3D, Intersection,
            LinearExtrude, Minkowski, Mirror2D, Mirror3D, MultMatrix2D, MultMatrix3D, Offset,
            Polygon, Polyhedron, Resize2D, Resize3D, Rotate2D, Rotate3D, RotateExtrude, Scale2D,
            Scale3D, Sphere, Square, Surface, Text, Translate2D, Translate3D, Union,
        },
        value_type::{RGB, RGBA},
        AffineMatrix2D, AffineMatrix3D, Container2D, Container3D, Point2D, Point3D,
        ScadBuildable as _, Unit,
    };
}
