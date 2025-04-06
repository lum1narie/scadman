use ambassador::Delegate;
use derive_builder::Builder;
use derive_more::derive::From;

use crate::{
    AffineMatrix2D, Point2D, ScadSentence as _, Unit, __generate_scad_options, __impl_scad2d,
    internal::generate_sentence_repr,
    scad_display::{ambassador_impl_ScadDisplay, ScadDisplay},
    value_type::{Angle, Color},
};

/// Give an implementation of a modifier 2D object
/// that has no parameters and is applied to 2D objects.
macro_rules! __impl_operator_2d {
    ( $type:ident, $name:expr_2021 ) => {
        #[doc = concat!($name,
                                                            " modifier `", $name, "()` in SCAD.
        This Rust type is regarded as 2D object and only applys to 2D objects.")]
        #[allow(missing_debug_implementations)]
        #[allow(clippy::missing_const_for_fn)]
        #[derive(derive_builder::Builder, Debug, Clone)]
        pub struct $type {}

        $crate::__impl_scad2d!($type);

        impl $crate::scad_display::ScadDisplay for $type {
            fn repr_scad(&self) -> String {
                generate_sentence_repr($name, Vec::new())
            }
        }

        impl Default for $type {
            fn default() -> Self {
                Self::new()
            }
        }

        impl $type {
            /// generate new blank object
            pub const fn new() -> Self {
                Self {}
            }
        }
    };
}

/// Translate modifier `translate()` in SCAD.
/// This Rust type is regarded as 2D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone)]
pub struct Translate2D {
    /// Translation vector.
    /// `v` option in SCAD.
    #[builder(setter(into))]
    pub v: Point2D,
}

__impl_scad2d!(Translate2D);

impl ScadDisplay for Translate2D {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "translate",
            __generate_scad_options!(
                ("", self.v);;
            ),
        )
    }
}

/// Rotate modifier `rotate()` in SCAD.
/// This Rust type is regarded as 2D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone)]
pub struct Rotate2D {
    /// Rotation angle.
    /// `a` option in SCAD.
    ///
    /// See also [`Angle`].
    #[builder(setter(custom))]
    pub a: Angle,
}

__impl_scad2d!(Rotate2D);

impl Rotate2DBuilder {
    /// Set rotation angle in degrees.
    ///
    /// # Arguments
    ///
    /// + `value` - The rotation angle in degrees.
    pub fn deg(&mut self, value: Unit) -> &mut Self {
        let new = self;
        new.a = Some(Angle::Deg(value));
        new
    }
    /// Set rotation angle in radians.
    ///
    /// # Arguments
    ///
    /// + `value` - The rotation angle in radians.
    pub fn rad(&mut self, value: Unit) -> &mut Self {
        let new = self;
        new.a = Some(Angle::Rad(value));
        new
    }
}

impl ScadDisplay for Rotate2D {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "rotate",
            __generate_scad_options!(
                ("", self.a);;
            ),
        )
    }
}

/// Scale modifier `scale()` in SCAD.
/// This Rust type is regarded as 2D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone)]
pub struct Scale2D {
    /// Scaling vector.
    /// `v` option in SCAD.
    #[builder(setter(into))]
    pub v: Point2D,
}

__impl_scad2d!(Scale2D);

impl ScadDisplay for Scale2D {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "scale",
            __generate_scad_options!(
                ("", self.v);;
            ),
        )
    }
}

/// `auto` option in 2D resize modifier.
#[derive(Copy, Clone, Debug, PartialEq, Eq, From, Delegate)]
#[delegate(ScadDisplay)]
pub enum ResizeAuto {
    /// Same value for all dimensions.
    B(bool),
    /// Values for each dimension.
    V([bool; 2]),
}

/// Resize modifier `resize()` in SCAD.
/// This Rust type is regarded as 2D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone)]
pub struct Resize2D {
    /// New size.
    ///
    /// `0` means no change if the corresponding dimension of `auto` is `false`,
    /// or auto value if `true`.
    #[builder(setter(into))]
    pub size: Point2D,
    /// `auto` option in SCAD.
    ///
    /// See also [`ResizeAuto`].
    #[builder(setter(into, strip_option), default)]
    pub auto: Option<ResizeAuto>,
}

__impl_scad2d!(Resize2D);

impl ScadDisplay for Resize2D {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "resize",
            __generate_scad_options!(
                ("", self.size);
                ("auto", self.auto);
            ),
        )
    }
}

/// Mirror modifier `mirror()` in SCAD.
/// This Rust type is regarded as 2D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone)]
pub struct Mirror2D {
    /// Normal vector of the mirror plane.
    #[builder(setter(into))]
    pub v: Point2D,
}

__impl_scad2d!(Mirror2D);

impl ScadDisplay for Mirror2D {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "mirror",
            __generate_scad_options!(
                ("", self.v);;
            ),
        )
    }
}

/// Affine tranformation modifier `multmatrix()` in SCAD.
/// This Rust type is regarded as 2D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone)]
pub struct MultMatrix2D {
    /// Affine transformation matrix for 2D vector.
    #[builder(setter(into))]
    pub m: AffineMatrix2D,
}

__impl_scad2d!(MultMatrix2D);

impl ScadDisplay for MultMatrix2D {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "multmatrix",
            __generate_scad_options!(
                ("m", self.m);;
            ),
        )
    }
}

/// Color modifier `color()` in SCAD.
/// This Rust type is regarded as 2D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone)]
pub struct Color2D {
    /// Color.
    ///
    /// See also [`Color`].
    #[builder(setter(into))]
    pub c: Color,
    /// Alpha value.
    /// `a` option in SCAD.
    ///
    /// Set when the `color` is NOT [`Color::RGBA`].
    #[builder(setter(into, strip_option), default)]
    pub a: Option<Unit>,
}

__impl_scad2d!(Color2D);

impl ScadDisplay for Color2D {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "color",
            __generate_scad_options!(
                (self.c.name(), self.c.clone());
                ("a", self.a);
            ),
        )
    }
}

/// Size of offset modifier for SCAD
#[derive(Copy, Clone, Debug, PartialEq, Delegate)]
#[delegate(ScadDisplay)]
pub enum OffsetSize {
    /// Radius of the radial offset.
    R(Unit),
    /// Delta of the delta offset.
    Delta(Unit),
}

impl OffsetSize {
    /// Returns the name of the key in SCAD code
    ///
    /// # Returns
    ///
    /// The name of the key in SCAD code
    pub const fn name(self) -> &'static str {
        match self {
            Self::R(_) => "r",
            Self::Delta(_) => "delta",
        }
    }
}

/// Offset modifier `offset()` in SCAD.
#[derive(Builder, Debug, Clone)]
pub struct Offset {
    /// Size of the offset.
    /// `r` or `delta` option in SCAD.
    ///
    /// See also [`OffsetSize`].
    #[builder(setter(custom))]
    pub size: OffsetSize,
    /// Flag to determine the shape should be chamfered or not.
    /// `chamfer` option in SCAD.
    ///
    /// This parameter has no effect on radial offsets.
    #[builder(setter(into, strip_option), default)]
    pub chamfer: Option<bool>,
    /// `$fa` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub fa: Option<Unit>,
    /// `$fn` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub r#fn: Option<u64>,
    /// `$fs` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub fs: Option<Unit>,
}

__impl_scad2d!(Offset);

impl OffsetBuilder {
    /// Set `r` option in SCAD.
    ///
    /// # Arguments
    ///
    /// + `value` - `r` option in SCAD. This is the radial offset.
    pub fn r(&mut self, value: Unit) -> &mut Self {
        let new = self;
        new.size = Some(OffsetSize::R(value));
        new
    }
    /// Set `delta` option in SCAD.
    ///
    /// # Arguments
    ///
    /// + `value` - `delta` option in SCAD. This is the delta offset.
    pub fn delta(&mut self, value: Unit) -> &mut Self {
        let new = self;
        new.size = Some(OffsetSize::Delta(value));
        new
    }
}

impl ScadDisplay for Offset {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "offset",
            __generate_scad_options!(
                (self.size.name(), self.size);
                ("chamfer", self.chamfer),
                ("$fa", self.fa),
                ("$fn", self.r#fn),
                ("$fs", self.fs);
            ),
        )
    }
}

__impl_operator_2d!(Hull2D, "hull");
__impl_operator_2d!(Minkowski2D, "minkowski");
__impl_operator_2d!(Union2D, "union");
__impl_operator_2d!(Difference2D, "difference");
__impl_operator_2d!(Intersection2D, "intersection");

/// Projection modifier `projection()` in SCAD.
/// This Rust type is regarded as 2D object and only applys to 3D objects.
#[derive(Builder, Debug, Clone)]
pub struct Projection {
    /// Flag to determine the shape should be cut at z = 0 or not.
    ///
    /// If `cut` is `true`, the shape is cut at z = 0.
    /// If `cut` is `false`, the shape is as projected.
    #[builder(setter(into, strip_option), default)]
    pub cut: Option<bool>,
}

__impl_scad2d!(Projection);

impl ScadDisplay for Projection {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "projection",
            __generate_scad_options!(
                ;("cut", self.cut);
            ),
        )
    }
}

#[cfg(test)]
mod tests {

    use std::f64::consts::PI;

    use super::*;
    use crate::value_type::{RGB, RGBA};

    #[test]
    fn test_translate2d() {
        assert_eq!(
            Translate2D::build_with(|tb| {
                let _ = tb.v([8., -4.]);
            })
            .repr_scad(),
            "translate([8, -4])"
        );
    }

    #[test]
    fn test_rotate2d() {
        assert_eq!(
            Rotate2D::build_with(|rb| {
                let _ = rb.deg(45.);
            })
            .repr_scad(),
            "rotate(45)"
        );
        assert_eq!(
            Rotate2D::build_with(|rb| {
                let _ = rb.rad(PI / 4.);
            })
            .repr_scad(),
            "rotate(45)"
        );
    }

    #[test]
    fn test_mirror2d() {
        assert_eq!(
            Mirror2D::build_with(|mb| {
                let _ = mb.v([1., -1.]);
            })
            .repr_scad(),
            "mirror([1, -1])"
        );
    }

    #[test]
    fn test_scale2d() {
        assert_eq!(
            Scale2D::build_with(|sb| {
                let _ = sb.v([3., 2.]);
            })
            .repr_scad(),
            "scale([3, 2])"
        );
    }

    #[test]
    fn test_resize2d() {
        let mut r1 = Resize2DBuilder::default();
        _ = r1.size([3., 2.]);
        assert_eq!(r1.clone().build().unwrap().repr_scad(), "resize([3, 2])");
        assert_eq!(
            r1.clone().auto(true).build().unwrap().repr_scad(),
            "resize([3, 2], auto = true)"
        );
        assert_eq!(
            r1.auto([true, false]).build().unwrap().repr_scad(),
            "resize([3, 2], auto = [true, false])"
        );
    }

    #[test]
    fn test_multimatrix2d() {
        let m = AffineMatrix2D::new(1., 2., 3., 4., 5., 6.);
        assert_eq!(
            MultMatrix2D::build_with(|mb| {
                let _ = mb.m(m);
            })
            .repr_scad(),
            "multmatrix(m = [[1, 2, 0, 3], [4, 5, 0, 6], [0, 0, 1, 0]])"
        );
    }

    #[test]
    fn test_color2d() {
        assert_eq!(
            Color2D::build_with(|cb| {
                let _ = cb.c(RGB::new(0.3, 0.5, 0.2));
            })
            .repr_scad(),
            "color(c = [0.3, 0.5, 0.2])"
        );
        assert_eq!(
            // Color2DBuilder::default()
            //     .c(RGB::new(0.3, 0.5, 0.2))
            //     .a(1.0)
            //     .apply_to(children.clone())
            //     .build()
            //     .unwrap()
            //     .repr_scad(),
            Color2D::build_with(|cb| {
                let _ = cb.c(RGB::new(0.3, 0.5, 0.2)).a(1.0);
            })
            .repr_scad(),
            "color(c = [0.3, 0.5, 0.2], a = 1)"
        );
        assert_eq!(
            Color2D::build_with(|cb| {
                let _ = cb.c(RGBA::new(0.3, 0.5, 0.2, 1.0));
            })
            .repr_scad(),
            "color(c = [0.3, 0.5, 0.2, 1])"
        );
        assert_eq!(
            Color2D::build_with(|cb| {
                let _ = cb.c("#C0FFEE".to_string());
            })
            .repr_scad(),
            "color(\"#C0FFEE\")"
        );
    }

    #[test]
    fn test_offset() {
        assert_eq!(
            Offset::build_with(|ob| {
                let _ = ob.r(1.);
            })
            .repr_scad(),
            "offset(r = 1)"
        );
        assert_eq!(
            Offset::build_with(|ob| {
                let _ = ob.delta(2.);
            })
            .repr_scad(),
            "offset(delta = 2)"
        );
        assert_eq!(
            Offset::build_with(|ob| {
                let _ = ob.r(1.).chamfer(true).fs(10);
            })
            .repr_scad(),
            "offset(r = 1, chamfer = true, $fs = 10)"
        );
    }

    #[test]
    fn test_hull() {
        assert_eq!(Hull2D::new().repr_scad(), "hull()");
    }

    #[test]
    fn test_minkowski() {
        assert_eq!(Minkowski2D::new().repr_scad(), "minkowski()");
    }

    #[test]
    fn test_binary_op() {
        assert_eq!(Union2D::new().repr_scad(), "union()");
        assert_eq!(Difference2D::new().repr_scad(), "difference()");
        assert_eq!(Intersection2D::new().repr_scad(), "intersection()");
    }

    #[test]
    fn test_projection() {
        assert_eq!(
            Projection::build_with(|pb| {
                let _ = pb;
            })
            .repr_scad(),
            "projection()"
        );
        assert_eq!(
            Projection::build_with(|pb| {
                let _ = pb.cut(true);
            })
            .repr_scad(),
            "projection(cut = true)"
        );
    }
}
