use std::fmt::{Debug, Display, Formatter};

use dyn_clone::DynClone;
use nalgebra as na;

pub type Unit = f64;
pub type Container2D<T> = na::Vector2<T>;
pub type Container3D<T> = na::Vector3<T>;
pub type Point2D = Container2D<Unit>;
pub type Point3D = Container3D<Unit>;
pub type AffineMatrix2D = na::Matrix2x3<Unit>;
pub type AffineMatrix3D = na::Matrix3x4<Unit>;
pub type RGB = na::Vector3<Unit>;
pub type RGBA = na::Vector4<Unit>;

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

pub trait ScadDisplay {
    fn repr_scad(&self) -> String;
}
macro_rules! __scad_display_as_string_impl {
    ( $type:ty ) => {
        impl ScadDisplay for $type {
            fn repr_scad(&self) -> String {
                self.to_string()
            }
        }
    };
}

const UNIT_PRECISION: usize = 8;
fn format_float(x: f64, n: usize) -> String {
    let mut s = format!("{0:.1$}", x, n);
    if s.contains('.') {
        while s.ends_with('0') {
            s.pop();
        }
        if s.ends_with('.') {
            s.pop();
        }
    }
    s
}

impl ScadDisplay for Unit {
    fn repr_scad(&self) -> String {
        format_float(*self, UNIT_PRECISION)
    }
}
__scad_display_as_string_impl!(u64);
__scad_display_as_string_impl!(usize);
__scad_display_as_string_impl!(bool);
impl ScadDisplay for String {
    fn repr_scad(&self) -> String {
        format!("\"{}\"", self.replace('"', "\\\""))
    }
}
impl<T: ScadDisplay> ScadDisplay for Container2D<T> {
    fn repr_scad(&self) -> String {
        format!("[{}, {}]", self[0].repr_scad(), self[1].repr_scad())
    }
}
impl<T: ScadDisplay> ScadDisplay for Container3D<T> {
    fn repr_scad(&self) -> String {
        format!(
            "[{}, {}, {}]",
            self[0].repr_scad(),
            self[1].repr_scad(),
            self[2].repr_scad()
        )
    }
}
impl<T: ScadDisplay> ScadDisplay for Vec<T> {
    fn repr_scad(&self) -> String {
        format!(
            "[{}]",
            self.iter()
                .map(|x| x.repr_scad())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
impl ScadDisplay for AffineMatrix3D {
    fn repr_scad(&self) -> String {
        format!(
            "[{}]",
            self.row_iter()
                .map(|row| format!(
                    "[{}]",
                    row.iter()
                        .map(|x| x.repr_scad())
                        .collect::<Vec<_>>()
                        .join(", ")
                ))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
impl ScadDisplay for AffineMatrix2D {
    fn repr_scad(&self) -> String {
        #[rustfmt::skip]
        let a3d = AffineMatrix3D::new(
            self[(0, 0)], self[(0, 1)], 0.0, self[(0, 2)],
            self[(1, 0)], self[(1, 1)], 0.0, self[(1, 2)],
            0.0,          0.0,          1.0, 0.0,
        );
        a3d.repr_scad()
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

pub enum ScadOption {
    Value(String),
    KeyValue((String, String)),
}

impl Display for ScadOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScadOption::Value(v) => std::fmt::Display::fmt(&v, f),
            ScadOption::KeyValue((k, v)) => {
                std::fmt::Display::fmt(&k, f)?;
                write!(f, " = ")?;
                std::fmt::Display::fmt(&v, f)
            }
        }
    }
}

impl ScadOption {
    pub fn from_key_value<T: ScadDisplay>(name: &str, value: T) -> Self {
        if name.is_empty() {
            Self::Value(value.repr_scad())
        } else {
            Self::KeyValue((name.to_string(), value.repr_scad()))
        }
    }

    pub fn from_key_value_option<T: ScadDisplay>(name: &str, value: Option<T>) -> Option<Self> {
        Some(Self::from_key_value(name, value?))
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __generate_scad_options {
    ( $(($name_req:expr, $value_req:expr)),*; $(;)? ) => {
        {
            vec![
                $($crate::scad::ScadOption::from_key_value($name_req, $value_req),)*
            ]
        }
    };
    ( $(($name_req:expr, $value_req:expr)),*; $(($name_opt:expr, $value_opt:expr)),+; ) => {
        {
            let mut opts: Vec<$crate::scad::ScadOption> = vec![
                $($crate::scad::ScadOption::from_key_value($name_req, $value_req),)*
            ];
            $(
                let maybe_opt = $crate::scad::ScadOption::from_key_value_option($name_opt, $value_opt);
                if let Some(opt) = maybe_opt {
                    opts.push(opt);
                }
            )*
                opts
        }
    };
}

pub(crate) fn generate_body(name: &str, opts: Vec<ScadOption>) -> String {
    let reprs = opts.iter().map(|o| o.to_string()).collect::<Vec<_>>();
    format!("{}({})", name, reprs.join(", "))
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
