//! 2D objects in SCAD.
use ambassador::Delegate;
use derive_more::derive::From;

use crate::ambassador_impl_ScadObjectTrait;
use crate::ScadObjectTrait;

mod object;
pub use object::*;

mod modifier;
pub use modifier::*;

/// A 2D object in SCAD.
#[derive(Debug, Clone, Delegate, From)]
#[delegate(ScadObjectTrait)]
pub enum ScadObject2D {
    /// `circle()` in SCAD.
    Circle(Circle),
    /// `polygon()` in SCAD.
    Polygon(Polygon),
    /// `square()` in SCAD.
    Square(Square),
    /// `text()` in SCAD.
    Text(Text),
    /// `color()` in SCAD.
    Color(Color2D),
    /// `difference()` in SCAD.
    Difference(Difference2D),
    /// `hull()` in SCAD.
    Hull(Hull2D),
    /// `import()` in SCAD.
    Import(Import2D),
    /// `intersection()` in SCAD.
    Intersection(Intersection2D),
    /// `minkowski()` in SCAD.
    Minkowski(Minkowski2D),
    /// `mirror()` in SCAD.
    Mirror(Mirror2D),
    /// `multmatrix()` in SCAD.
    MultMatrix(MultMatrix2D),
    /// `offset()` in SCAD.
    Offset(Offset),
    /// `resize()` in SCAD.
    Resize(Resize2D),
    /// `rotate()` in SCAD.
    Rotate(Rotate2D),
    /// `scale()` in SCAD.
    Scale(Scale2D),
    /// `translate()` in SCAD.
    Translate(Translate2D),
    /// `union()` in SCAD.
    Union(Union2D),
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_scad2d {
    ( $type:ident ) => {
        $crate::__build_with_impl!($type);
    };
}
