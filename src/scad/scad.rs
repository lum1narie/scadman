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

pub trait ScadObject2D: ScadObject + DynClone {}
pub trait ScadObject3D: ScadObject + DynClone {}
dyn_clone::clone_trait_object!(ScadObject2D);
dyn_clone::clone_trait_object!(ScadObject3D);

#[macro_export]
macro_rules! any_scads {
    [ $($scad:expr),* $(,)? ] => {
        vec![$(Box::new($scad) as Box<dyn ScadObject>),*]
    }
}
#[macro_export]
macro_rules! any_scads2d {
    [ $($scad:expr),* $(,)? ] => {
        vec![$(Box::new($scad) as Box<dyn ScadObject2D>),*]
    }
}
#[macro_export]
macro_rules! any_scads3d {
    [ $($scad:expr),* $(,)? ] => {
        vec![$(Box::new($scad) as Box<dyn ScadObject3D>),*]
    }
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
