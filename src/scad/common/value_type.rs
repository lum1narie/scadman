use ambassador::Delegate;
use derive_more::derive::From;
use nalgebra as na;

use super::{ambassador_impl_ScadDisplay, ScadDisplay, Unit};

pub type RGB = na::Vector3<Unit>;
pub type RGBA = na::Vector4<Unit>;

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

impl ScadDisplay for Angle {
    fn repr_scad(&self) -> String {
        self.deg().repr_scad()
    }
}

#[derive(Clone, Debug, PartialEq, From, Delegate)]
#[delegate(ScadDisplay)]
pub enum Color {
    RGB(RGB),
    RGBA(RGBA),
    Name(String),
}

impl ScadDisplay for RGBA {
    fn repr_scad(&self) -> String {
        format!(
            "[{}, {}, {}, {}]",
            self[0].repr_scad(),
            self[1].repr_scad(),
            self[2].repr_scad(),
            self[3].repr_scad()
        )
    }
}

impl Color {
    pub fn name(&self) -> &'static str {
        match self {
            Color::Name(_) => "",
            _ => "c",
        }
    }
}
