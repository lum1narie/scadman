use ambassador::Delegate;
use derive_builder::Builder;
use derive_more::derive::From;

use crate::{
    common::{AffineMatrix2D, DimensionType as _, Point2D, Unit},
    internal::generate_sentence_repr,
    scad_display::{ambassador_impl_ScadDisplay, ScadDisplay},
    value_type::Angle,
};

use crate::common::D2;

macro_rules! __impl_apply_2d {
    ($mod_ty:ident) => {
        __impl_apply_to_modifier!(
            apply_to,
            $mod_ty,
            $crate::scad_2d::ScadModifierBody2D,
            $crate::common::ScadObjectGeneric<$crate::common::D2>, // Typed output object
            $crate::scad_2d::ScadModifier2D,
            $crate::scad_2d::ScadObject2D,
            $crate::common::ScadObjectImpl::Object2D,
            $crate::common::D2, // output_marker
            $crate::common::D2  // child_marker
        );
        __impl_panicking_modifier_methods!($mod_ty, D2);
    };
}

macro_rules! __impl_apply_3d {
    ($mod_ty:ident) => {
        __impl_apply_to_modifier!(
            apply_to,
            $mod_ty,
            $crate::scad_2d::ScadModifierBody2D, // Modifier body will be ScadModifierBody2D::Projection
            $crate::common::ScadObjectGeneric<$crate::common::D2>, // Typed output object
            $crate::scad_2d::ScadModifier2D,     // Concrete ScadModifier type
            $crate::scad_2d::ScadObject2D,       // Concrete ScadObject enum type
            $crate::common::ScadObjectImpl::Object2D, // ScadObjectImpl variant
            $crate::common::D2,                  // Output dimension marker (Projection outputs 2D)
            $crate::common::D3 // Child dimension marker (Projection takes 3D child)
        );
        __impl_panicking_modifier_methods!($mod_ty, D2);
    };
}

/// Translate modifier `translate()` in SCAD.
/// This Rust type is regarded as 2D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone, Copy)]
pub struct Translate2D {
    /// Translation vector.
    /// `v` option in SCAD.
    #[builder(setter(into))]
    pub v: Point2D,
}

__impl_builder_modifier!(Translate2D);
__impl_modifier_chaining!(Translate2D);
__impl_apply_2d!(Translate2D);

impl ScadDisplay for Translate2D {
    fn repr_scad(&self) -> String {
        generate_sentence_repr("translate", __generate_scad_options!(("", self.v);))
    }
}

/// Rotate modifier `rotate()` in SCAD.
/// This Rust type is regarded as 2D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone, Copy)]
pub struct Rotate2D {
    /// Rotation angle.
    /// `a` option in SCAD.
    ///
    /// See also [`Angle`].
    #[builder(setter(custom))]
    pub a: Angle,
}

__impl_builder_modifier!(Rotate2D);
__impl_modifier_chaining!(Rotate2D);
__impl_apply_2d!(Rotate2D);

impl Rotate2DBuilder {
    /// Set rotation angle in degrees.
    ///
    /// # Arguments
    ///
    /// + `value` - The rotation angle in degrees.
    pub const fn deg(&mut self, value: Unit) -> &mut Self {
        let new = self;
        new.a = Some(Angle::Deg(value));
        new
    }
    /// Set rotation angle in radians.
    ///
    /// # Arguments
    ///
    /// + `value` - The rotation angle in radians.
    pub const fn rad(&mut self, value: Unit) -> &mut Self {
        let new = self;
        new.a = Some(Angle::Rad(value));
        new
    }
}

impl ScadDisplay for Rotate2D {
    fn repr_scad(&self) -> String {
        generate_sentence_repr("rotate", __generate_scad_options!(("a", self.a);))
    }
}

/// Scale modifier `scale()` in SCAD.
/// This Rust type is regarded as 2D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone, Copy)]
pub struct Scale2D {
    /// Scaling vector.
    /// `v` option in SCAD.
    #[builder(setter(into))]
    pub v: Point2D,
}

__impl_builder_modifier!(Scale2D);
__impl_modifier_chaining!(Scale2D);
__impl_apply_2d!(Scale2D);

impl ScadDisplay for Scale2D {
    fn repr_scad(&self) -> String {
        generate_sentence_repr("scale", __generate_scad_options!(("", self.v);))
    }
}

/// `auto` option in 2D resize modifier.
#[derive(Copy, Clone, Debug, PartialEq, Eq, From, Delegate)]
#[delegate(ScadDisplay)]
pub enum ResizeAuto2D {
    /// Same value for all dimensions.
    B(bool),
    /// Values for each dimension.
    V([bool; 2]),
}

/// Resize modifier `resize()` in SCAD.
/// This Rust type is regarded as 2D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone, Copy)]
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
    pub auto: Option<ResizeAuto2D>,
}

__impl_builder_modifier!(Resize2D);
__impl_modifier_chaining!(Resize2D);
__impl_apply_2d!(Resize2D);

impl ScadDisplay for Resize2D {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "resize",
            __generate_scad_options!(
                ("", self.size); opt: (("auto", self.auto);)
            ),
        )
    }
}

/// Mirror modifier `mirror()` in SCAD.
/// This Rust type is regarded as 2D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone, Copy)]
pub struct Mirror2D {
    /// Normal vector of the mirror plane.
    #[builder(setter(into))]
    pub v: Point2D,
}

__impl_builder_modifier!(Mirror2D);
__impl_modifier_chaining!(Mirror2D);
__impl_apply_2d!(Mirror2D);

impl ScadDisplay for Mirror2D {
    fn repr_scad(&self) -> String {
        generate_sentence_repr("mirror", __generate_scad_options!(("", self.v);))
    }
}

/// Affine tranformation modifier `multmatrix()` in SCAD.
/// This Rust type is regarded as 2D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone, Copy)]
pub struct MultMatrix2D {
    /// Affine transformation matrix for 2D vector.
    #[builder(setter(into))]
    pub m: AffineMatrix2D,
}

__impl_builder_modifier!(MultMatrix2D);
__impl_modifier_chaining!(MultMatrix2D);
__impl_apply_2d!(MultMatrix2D);

impl ScadDisplay for MultMatrix2D {
    fn repr_scad(&self) -> String {
        generate_sentence_repr("multmatrix", __generate_scad_options!(("m", self.m);))
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
#[derive(Builder, Debug, Clone, Copy)]
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

__impl_builder_modifier!(Offset);
__impl_modifier_chaining!(Offset);
__impl_apply_2d!(Offset);

impl OffsetBuilder {
    /// Set `r` option in SCAD.
    ///
    /// # Arguments
    ///
    /// + `value` - `r` option in SCAD. This is the radial offset.
    pub const fn r(&mut self, value: Unit) -> &mut Self {
        let new = self;
        new.size = Some(OffsetSize::R(value));
        new
    }
    /// Set `delta` option in SCAD.
    ///
    /// # Arguments
    ///
    /// + `value` - `delta` option in SCAD. This is the delta offset.
    pub const fn delta(&mut self, value: Unit) -> &mut Self {
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
                opt: (
                    ("chamfer", self.chamfer);
                    ("$fa", self.fa);
                    ("$fn", self.r#fn);
                    ("$fs", self.fs);
                )
            ),
        )
    }
}

/// Projection modifier `projection()` in SCAD.
/// This Rust type is regarded as 2D object and only applys to 3D objects.
#[derive(Builder, Debug, Clone, Copy)]
pub struct Projection {
    /// Flag to determine the shape should be cut at z = 0 or not.
    ///
    /// If `cut` is `true`, the shape is cut at z = 0.
    /// If `cut` is `false`, the shape is as projected.
    #[builder(setter(into, strip_option), default)]
    pub cut: Option<bool>,
}

__impl_builder_modifier!(Projection);
__impl_modifier_chaining!(Projection);
__impl_apply_3d!(Projection);

impl ScadDisplay for Projection {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "projection",
            __generate_scad_options!(opt: (("cut", self.cut);)),
        )
    }
}

#[cfg(test)]
mod tests {

    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn test_translate2d() {
        assert_eq!(
            Translate2DBuilder::default()
                .v([8., -4.])
                .build()
                .unwrap()
                .repr_scad(),
            "translate([8, -4])"
        );
    }

    #[test]
    fn test_rotate2d() {
        assert_eq!(
            Rotate2DBuilder::default()
                .deg(45.)
                .build()
                .unwrap()
                .repr_scad(),
            "rotate(a = 45)"
        );
        assert_eq!(
            Rotate2DBuilder::default()
                .rad(PI / 4.)
                .build()
                .unwrap()
                .repr_scad(),
            "rotate(a = 45)"
        );
    }

    #[test]
    fn test_mirror2d() {
        assert_eq!(
            Mirror2DBuilder::default()
                .v([1., -1.])
                .build()
                .unwrap()
                .repr_scad(),
            "mirror([1, -1])"
        );
    }

    #[test]
    fn test_scale2d() {
        assert_eq!(
            Scale2DBuilder::default()
                .v([3., 2.])
                .build()
                .unwrap()
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
            MultMatrix2DBuilder::default()
                .m(m)
                .build()
                .unwrap()
                .repr_scad(),
            "multmatrix(m = [[1, 2, 0, 3], [4, 5, 0, 6], [0, 0, 1, 0]])"
        );
    }

    #[test]
    fn test_offset() {
        assert_eq!(
            OffsetBuilder::default().r(1.).build().unwrap().repr_scad(),
            "offset(r = 1)"
        );
        assert_eq!(
            OffsetBuilder::default()
                .delta(2.)
                .build()
                .unwrap()
                .repr_scad(),
            "offset(delta = 2)"
        );
        assert_eq!(
            OffsetBuilder::default()
                .r(1.)
                .chamfer(true)
                .fs(10)
                .build()
                .unwrap()
                .repr_scad(),
            "offset(r = 1, chamfer = true, $fs = 10)"
        );
    }

    #[test]
    fn test_projection() {
        assert_eq!(
            ProjectionBuilder::default().build().unwrap().repr_scad(),
            "projection()"
        );
        assert_eq!(
            ProjectionBuilder::default()
                .cut(true)
                .build()
                .unwrap()
                .repr_scad(),
            "projection(cut = true)"
        );
    }
}
