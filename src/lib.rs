//! `scadman` is a Rust library designed for programmatically generating OpenSCAD code.
//!
//! It offers a type-safe and structured approach to defining 2D and 3D geometric
//! objects, applying transformations, and performing operations, ultimately outputting
//! valid OpenSCAD code.
//!
//! ## Key Features:
//! - **Generic Type System**: Leverages `ScadObjectGeneric<D>` to provide compile-time
//!   dimensional safety (2D, 3D, Mixed) while maintaining runtime flexibility.
//! - **Comprehensive API**: Supports a wide range of OpenSCAD primitives, modifiers,
//!   and block operations.
//! - **Builder Pattern**: Simplifies configuration of complex SCAD sentences.
//! - **Operator Overloading**: Enables intuitive use of `+`, `-`, `*` for boolean
//!   operations with runtime dimension checks.
//!
//! For detailed usage, examples, and an in-depth understanding of the type system,
//! please refer to the [README.md](https://github.com/lum1narie/scadman/blob/main/README.md).

#[macro_use]
pub(crate) mod internal;
mod macros;

#[allow(clippy::redundant_pub_crate)]
pub mod common;

pub mod scad_display;
pub mod value_type;

pub mod scad_sentence;

pub mod scad_2d;
pub mod scad_3d;
pub mod scad_mixed;

pub mod legacy;

/// import `prelude::*` so you can be ready to code!
///
/// This prelude intentionally exports a curated subset of the public API that
/// is stable after the runtime-refactor. It avoids exporting removed internal
/// types (like `ScadObjectBody`) while keeping the commonly used helpers.
pub mod prelude {
    pub use crate::{
        common::{
            AffineMatrix2D, AffineMatrix3D, Container2D, Container3D, DimensionMarker, Point2D,
            Point3D, ScadBuildable as _, ScadBuilder as _, ScadObject, ScadObject2D, ScadObject3D,
            ScadObjectGeneric, ScadObjectUntyped, Unit,
        },
        scad_2d::{
            ScadBlock2D, ScadModifier2D, ScadModifierBody2D, ScadPrimitive2D, ScadPrimitiveBody2D,
        },
        scad_3d::{
            ScadBlock3D, ScadModifier3D, ScadModifierBody3D, ScadPrimitive3D, ScadPrimitiveBody3D,
        },
        scad_mixed::{ScadBlockMixed, ScadModifierBodyMixed, ScadModifierMixed},
        scad_sentence::{
            Circle, Color, Cube, Cylinder, Difference, Hull, Import2D, Import3D, Intersection,
            LinearExtrude, Minkowski, Mirror2D, Mirror3D, MultMatrix2D, MultMatrix3D, Offset,
            Polygon, Polyhedron, Projection, Resize2D, Resize3D, Rotate2D, Rotate3D, RotateExtrude,
            Scale2D, Scale3D, Sphere, Square, Surface, Text, Translate2D, Translate3D, Union,
        },
        value_type::{RGB, RGBA},
    };
}
