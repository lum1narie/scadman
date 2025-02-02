mod scad_display;
pub use scad_display::*;

mod value_type;
pub use value_type::*;

mod internal;
pub(crate) use internal::*;

use std::fmt::Debug;

use dyn_clone::DynClone;
use nalgebra as na;

pub type Unit = f64;
pub type Container2D<T> = na::Vector2<T>;
pub type Container3D<T> = na::Vector3<T>;
pub type Point2D = Container2D<Unit>;
pub type Point3D = Container3D<Unit>;
pub type AffineMatrix2D = na::Matrix2x3<Unit>;
pub type AffineMatrix3D = na::Matrix3x4<Unit>;

const INDENT: usize = 2;

pub trait ScadObject: Debug + DynClone {
    fn get_body(&self) -> String;
    fn get_children(&self) -> Option<Vec<String>> {
        None
    }
    fn to_code(&self) -> String {
        let body = self.get_body();
        let code = match self.get_children() {
            Some(c) => {
                let unindented_str = c.join("\n");
                let children = unindented_str
                    .split("\n")
                    .map(|s| format!("{}{}", " ".repeat(INDENT), s))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("{} {{\n{}\n}}", body, children)
            }
            None => body + ";",
        };
        code
    }
}
dyn_clone::clone_trait_object!(ScadObject);

pub trait ScadObject2D: ScadObject {}
pub trait ScadObject3D: ScadObject {}
dyn_clone::clone_trait_object!(ScadObject2D);
dyn_clone::clone_trait_object!(ScadObject3D);
