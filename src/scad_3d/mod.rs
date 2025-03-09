//! 3D objects in SCAD.
use ambassador::Delegate;
use derive_more::derive::From;

use crate::ambassador_impl_ScadObjectTrait;
use crate::ScadObjectTrait;

mod object;
pub use object::*;

mod modifier;
pub use modifier::*;

/// A 3D object in SCAD.
#[derive(Debug, Clone, Delegate, From)]
#[delegate(ScadObjectTrait)]
pub enum ScadObject3D {
    /// `cube()` in SCAD.
    Cube(Cube),
    /// `cylinder()` in SCAD.
    Cylinder(Cylinder),
    /// `polyhedron()` in SCAD.
    Polyhedron(Polyhedron),
    /// `sphere()` in SCAD.
    Sphere(Sphere),
    /// `surface()` in SCAD.
    Surface(Surface),
    /// `color()` in SCAD.
    Color(Color3D),
    /// `difference()` in SCAD.
    Difference(Difference3D),
    /// `hull()` in SCAD.
    Hull(Hull3D),
    /// `import()` in SCAD.
    Import(Import3D),
    /// `intersection()` in SCAD.
    Intersection(Intersection3D),
    /// `linear_extrude()` in SCAD.
    LinearExtrude(LinearExtrude),
    /// `minkowski()` in SCAD.
    Minkowski(Minkowski3D),
    /// `mirror()` in SCAD.
    Mirror(Mirror3D),
    /// `multmatrix()` in SCAD.
    MultMatrix(MultMatrix3D),
    /// `resize()` in SCAD.
    Resize(Resize3D),
    /// `rotate()` in SCAD.
    Rotate(Rotate3D),
    /// `rotate_extrude()` in SCAD.
    RotateExtrude(RotateExtrude),
    /// `scale()` in SCAD.
    Scale(Scale3D),
    /// `translate()` in SCAD.
    Translate(Translate3D),
    /// `union()` in SCAD.
    Union(Union3D),
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_scad3d {
    ( $type:ident ) => {
        $crate::__build_with_impl!($type);
    };
}
