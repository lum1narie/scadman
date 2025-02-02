use ambassador::delegatable_trait;
use derive_more::derive::From;

use super::{AffineMatrix2D, AffineMatrix3D, Container2D, Container3D, Unit};

#[delegatable_trait]
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

#[derive(Clone, Debug, From)]
pub struct Identifier(pub String);

impl ScadDisplay for Identifier {
    fn repr_scad(&self) -> String {
        self.0.clone()
    }
}

impl<T: ScadDisplay, const N: usize> ScadDisplay for [T; N] {
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
