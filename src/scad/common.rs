use std::fmt::Debug;

use dyn_clone::DynClone;
use nalgebra as na;

pub type Unit = f64;
pub type Point2D = na::Vector2<Unit>;
pub type Point3D = na::Vector3<Unit>;

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

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_scad_box {
    ( $type:ty ) => {
        impl From<$type> for Vec<Box<dyn ScadObject>> {
            fn from(value: $type) -> Self {
                vec![Box::new(value) as Box<dyn ScadObject>]
            }
        }
    };
}

pub trait ScadObject2D: ScadObject {}
pub trait ScadObject3D: ScadObject {}
dyn_clone::clone_trait_object!(ScadObject2D);
dyn_clone::clone_trait_object!(ScadObject3D);

#[macro_export]
macro_rules! any_scads {
    [ $($scad:expr),* $(,)? ] => {
        vec![$(Box::new($scad) as Box<dyn ScadObject>),*]
    };
}
#[macro_export]
macro_rules! any_scads2d {
    [ $($scad:expr),* $(,)? ] => {
        vec![$(Box::new($scad) as Box<dyn ScadObject2D>),*]
    };
}
#[macro_export]
macro_rules! any_scads3d {
    [ $($scad:expr),* $(,)? ] => {
        vec![$(Box::new($scad) as Box<dyn ScadObject3D>),*]
    };
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Angle {
    Deg(Unit),
    Rad(Unit),
}

impl Angle {
    pub fn deg(&self) -> Unit {
        match self {
            Angle::Deg(d) => *d,
            Angle::Rad(r) => r.to_degrees(),
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __get_children_impl {
    () => {
        fn get_children(&self) -> Option<Vec<String>> {
            Some(self.children.iter().map(|c| c.to_code()).collect())
        }
    };
}
