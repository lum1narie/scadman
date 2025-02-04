mod scad_display;
pub use scad_display::*;

mod value_type;
pub use value_type::*;

mod internal;
pub use internal::*;

use std::fmt::Debug;

use dyn_clone::DynClone;
use nalgebra as na;

/// Unit of length to write in SCAD code.
pub type Unit = f64;
/// Container type for 2D things.
pub type Container2D<T> = na::Vector2<T>;
/// Container type for 3D things.
pub type Container3D<T> = na::Vector3<T>;
/// Data type for 2D points.
pub type Point2D = Container2D<Unit>;
/// Data type for 3D points.
pub type Point3D = Container3D<Unit>;
/// Data type for Affine transformations in 2D.
pub type AffineMatrix2D = na::Matrix2x3<Unit>;
/// Data type for Affine transformations in 3D.
pub type AffineMatrix3D = na::Matrix3x4<Unit>;

const INDENT: usize = 2;

/// Trait for objects that can be written to SCAD code.
pub trait ScadObject: Debug + DynClone {
    /// Returns the body of the SCAD code.
    ///
    /// Body means the part that is not include
    /// neither any other objects nor semicolon.
    ///
    /// Ex.
    /// + circle may has a body such as `"circle(r = 1)"`
    /// + hull has a body of `"hull();"`
    ///
    /// # Returns
    ///
    /// SCAD code body as a [`String`].
    fn get_body(&self) -> String;

    /// Returns the SCAD code of the children of the object.
    ///
    /// Children means the objects in inner level of the object.
    ///
    /// Ex.
    /// + circle has no children, so it returns [`None`]
    /// + hull may has a body such as `Some(vec!["circle(r = 1)", "square(size = 2)"])`
    ///
    /// # Returns
    ///
    /// + [`Some<Vec<String>>`]: SCAD code of the children 
    /// + [`None`]: if the object has no children
    fn get_children(&self) -> Option<Vec<String>> {
        None
    }

    /// Returns the SCAD code of the object.
    ///
    /// Ex.
    /// + circle may returns such as `"circle(r = 1);"`
    /// + hull may returns such as `"hull(){\n  circle(r = 1);\n  square(size = 2);\n}"`
    ///
    /// # Returns
    ///
    /// SCAD code of the object as a [`String`].
    fn to_code(&self) -> String {
        let body = self.get_body();
        let code = match self.get_children() {
            Some(c) => {
                let unindented_str = c.join("\n");
                let children = unindented_str
                    .split('\n')
                    .map(|s| format!("{}{}", " ".repeat(INDENT), s))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("{body} {{\n{children}\n}}")
            }
            None => body + ";",
        };
        code
    }
}
dyn_clone::clone_trait_object!(ScadObject);

/// Trait for objects that shows 2D objects in SCAD.
pub trait ScadObject2D: ScadObject {}
/// Trait for objects that shows 3D objects in SCAD.
pub trait ScadObject3D: ScadObject {}
dyn_clone::clone_trait_object!(ScadObject2D);
dyn_clone::clone_trait_object!(ScadObject3D);
