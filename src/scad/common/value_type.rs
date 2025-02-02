use super::{ScadDisplay, Unit};

use nalgebra as na;

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

#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    RGB(RGB),
    RGBA(RGBA),
    Name(String),
}

impl From<RGB> for Color {
    fn from(rgb: RGB) -> Self {
        Color::RGB(rgb)
    }
}
impl From<RGBA> for Color {
    fn from(rgba: RGBA) -> Self {
        Color::RGBA(rgba)
    }
}
impl From<String> for Color {
    fn from(name: String) -> Self {
        Color::Name(name)
    }
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

impl ScadDisplay for Color {
    fn repr_scad(&self) -> String {
        match self {
            Color::RGB(rgb) => rgb.repr_scad(),
            Color::RGBA(rgba) => rgba.repr_scad(),
            Color::Name(name) => name.repr_scad(),
        }
    }
}
