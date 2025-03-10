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
    /// `projection()` in SCAD.
    Projection(Projection),
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
        paste::paste! {
            impl $crate::ScadBuildable for $type {
                type Builder = [<$type Builder>];
                type Enum = super::ScadObject2D;
            }

            impl $crate::ScadBuilder for [<$type Builder>] {
                type Target = $type;
                type Error = [<$type BuilderError>];
                fn build_scad(&self) -> Result<Self::Target, Self::Error> {
                    Self::build(&self)
                }
            }
        }
    };
}
