//! Trait and types that can be represented as a string in SCAD.

use ambassador::delegatable_trait;
use derive_more::derive::From;

use crate::common::{AffineMatrix2D, AffineMatrix3D, Container2D, Container3D, Unit};

/// Trait for types that can be represented as a string in SCAD.
#[delegatable_trait]
pub(crate) trait ScadDisplay {
    /// Returns a string representation of the type in SCAD.
    ///
    /// # Returns
    ///
    /// A [`String`] representation of the type in SCAD.
    fn repr_scad(&self) -> String;
}

/// A macro for implementing [`ScadDisplay`] for a types.
/// This macro gives [`repr_scad`] just as a [`to_string`] implementation.
macro_rules! __scad_display_as_string_impl {
    ( $type:ty ) => {
        impl ScadDisplay for $type {
            fn repr_scad(&self) -> String {
                self.to_string()
            }
        }
    };
}

/// Precision of [`Unit`].
/// This represents the number of decimal places in a decimal number.
const UNIT_PRECISION: usize = 8;

/// Formats a floating point number as a [`String`].
/// This function rounds a float in [`UNIT_PRECISION`] decimal places.
fn format_float(x: f64, n: usize) -> String {
    let mut s = format!("{x:.n$}");
    if s.contains('.') {
        while s.ends_with('0') {
            if s.pop().is_none() {
                break;
            }
        }
        if s.ends_with('.') {
            _ = s.pop();
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

/// A type for representing an identifier in SCAD.
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
                .map(ScadDisplay::repr_scad)
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
                .map(ScadDisplay::repr_scad)
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
                        .map(ScadDisplay::repr_scad)
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
