use ambassador::Delegate;
use derive_builder::Builder;
use derive_more::derive::From;
use nalgebra as na;

use crate::{
    __generate_scad_options,
    internal::generate_sentence_repr,
    scad_display::{ambassador_impl_ScadDisplay, ScadDisplay},
    value_type::Angle,
    AffineMatrix3D, Point3D, Unit, __impl_builder_sentence,
};

/// Translate modifier `translate()` in SCAD.
/// This Rust type is regarded as 3D object and only applys to 3D objects.
#[derive(Builder, Debug, Clone)]
pub struct Translate3D {
    /// Translation vector.
    /// `v` option in SCAD.
    #[builder(setter(into))]
    pub v: Point3D,
}

__impl_builder_sentence!(Translate3D);

impl ScadDisplay for Translate3D {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "translate",
            __generate_scad_options!(
                ("", self.v);;
            ),
        )
    }
}

/// Angle of rotate (3D) in SCAD.
///
/// `a` option in SCAD.
#[derive(Copy, Clone, Debug, PartialEq, From, Delegate)]
#[delegate(ScadDisplay)]
pub enum Rotate3DAngle {
    /// Rotation angle on `v`
    A(Angle),
    /// Rotation angles in `[x, y, z]` axes
    V(na::Vector3<Angle>),
}

/// Numbers to generate [`Rotate3DAngle`].
///
/// The numbers are the angle.
/// This type have no information about the angle is rad or deg.
#[derive(Copy, Clone, Debug, PartialEq, From)]
pub enum Rotate3DAngleEntry {
    /// Number to generate [`Rotate3DAngle::A`].
    Single(Unit),
    /// Pair of numbers to generate [`Rotate3DAngle::V`].
    Triple([Unit; 3]),
}

/// Rotate modifier `rotate()` in SCAD.
/// This Rust type is regarded as 3D object and only applys to 3D objects.
#[derive(Builder, Debug, Clone)]
pub struct Rotate3D {
    /// Rotation angle.
    /// `a` option in SCAD.
    ///
    /// See also [`AngleRotate3D`].
    #[builder(setter(custom))]
    pub a: Rotate3DAngle,
    /// Rotation axis.
    #[builder(setter(into, strip_option), default)]
    pub v: Option<Point3D>,
}

__impl_builder_sentence!(Rotate3D);

impl Rotate3DBuilder {
    /// Set rotation angle in degrees.
    ///
    /// # Arguments
    ///
    /// + `value` - The rotation angle in degrees.
    pub fn deg<T: Into<Rotate3DAngleEntry>>(&mut self, value: T) -> &mut Self {
        let new = self;
        new.a = match value.into() {
            Rotate3DAngleEntry::Single(a) => Some(Rotate3DAngle::A(Angle::Deg(a))),
            Rotate3DAngleEntry::Triple(a) => Some(Rotate3DAngle::V(na::Vector3::from_iterator(
                a.into_iter().map(Angle::Deg),
            ))),
        };
        new
    }

    /// Set rotation angle in radians.
    ///
    /// # Arguments
    ///
    /// + `value` - The rotation angle in radians.
    pub fn rad<T: Into<Rotate3DAngleEntry>>(&mut self, value: T) -> &mut Self {
        let new = self;
        new.a = match value.into() {
            Rotate3DAngleEntry::Single(a) => Some(Rotate3DAngle::A(Angle::Rad(a))),
            Rotate3DAngleEntry::Triple(a) => Some(Rotate3DAngle::V(na::Vector3::from_iterator(
                a.into_iter().map(Angle::Rad),
            ))),
        };
        new
    }
}

impl ScadDisplay for Rotate3D {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "rotate",
            __generate_scad_options!(
                ("a", self.a);
                ("v", self.v);
            ),
        )
    }
}

/// Scale modifier `scale()` in SCAD.
/// This Rust type is regarded as 3D object and only applys to 3D objects.
#[derive(Builder, Debug, Clone)]
pub struct Scale3D {
    /// Scaling vector.
    /// `v` option in SCAD.
    #[builder(setter(into))]
    pub v: Point3D,
}

__impl_builder_sentence!(Scale3D);

impl ScadDisplay for Scale3D {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "scale",
            __generate_scad_options!(
                ("", self.v);;
            ),
        )
    }
}

/// `auto` option in 3D resize modifier.
#[derive(Copy, Clone, Debug, PartialEq, Eq, From, Delegate)]
#[delegate(ScadDisplay)]
pub enum ResizeAuto3D {
    /// Same value for all dimensions.
    B(bool),
    /// Values for each dimension.
    V([bool; 3]),
}

/// Resize modifier `resize()` in SCAD.
/// This Rust type is regarded as 3D object and only applys to 3D objects.
#[derive(Builder, Debug, Clone)]
pub struct Resize3D {
    /// New size.
    ///
    /// `0` means no change if the corresponding dimension of `auto` is `false`,
    /// or auto value if `true`.
    #[builder(setter(into))]
    pub size: Point3D,
    /// `auto` option in SCAD.
    ///
    /// See also [`ResizeAuto`].
    #[builder(setter(into, strip_option), default)]
    pub auto: Option<ResizeAuto3D>,
}

__impl_builder_sentence!(Resize3D);

impl ScadDisplay for Resize3D {
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
/// This Rust type is regarded as 3D object and only applys to 3D objects.
#[derive(Builder, Debug, Clone)]
pub struct Mirror3D {
    /// Normal vector of the mirror plane.
    #[builder(setter(into))]
    pub v: Point3D,
}

__impl_builder_sentence!(Mirror3D);

impl ScadDisplay for Mirror3D {
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
/// This Rust type is regarded as 3D object and only applys to 3D objects.
#[derive(Builder, Debug, Clone)]
pub struct MultMatrix3D {
    /// Affine transformation matrix for 3D vector.
    #[builder(setter(into))]
    pub m: AffineMatrix3D,
}

__impl_builder_sentence!(MultMatrix3D);

impl ScadDisplay for MultMatrix3D {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "multmatrix",
            __generate_scad_options!(
                ("m", self.m);;
            ),
        )
    }
}

/// Linear extrude modifier `linear_extrude()` in SCAD.
/// This Rust type is regarded as 3D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone)]
pub struct LinearExtrude {
    /// The length of the extruded object.
    ///
    /// `height` must be positive.
    #[builder(setter(into), default)]
    pub height: Unit,
    /// The vector that extrusion follows.
    #[builder(setter(into, strip_option), default)]
    pub v: Option<Point3D>,
    /// `center` option in SCAD.
    ///
    /// + `true` - Z range is from 0 to height.
    /// + `false` - Z range is -height/2 to height/2.
    #[builder(setter(into, strip_option), default)]
    pub center: Option<bool>,
    /// Twist degrees of through which the shape is extruded.
    ///
    /// Setting the parameter twist = 360 extrudes through one revolution.
    /// The twist direction follows the left hand rule.
    #[builder(setter(into, strip_option), default)]
    pub twist: Option<Unit>,
    /// `convexity` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub convexity: Option<u64>,
    /// The number of intermediate points along the Z axis of the extrusion.
    #[builder(setter(into, strip_option), default)]
    pub slices: Option<u64>,
    /// Scales value over the height of the extrusion.
    #[builder(setter(into, strip_option), default)]
    pub scale: Option<Unit>,
    /// `$fn` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub r#fn: Option<u64>,
}

__impl_builder_sentence!(LinearExtrude);

impl ScadDisplay for LinearExtrude {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "linear_extrude",
            __generate_scad_options!(
                ("height", self.height);
                ("v", self.v),
                ("center", self.center),
                ("twist", self.twist),
                ("convexity", self.convexity),
                ("slices", self.slices),
                ("scale", self.scale),
                ("$fn", self.r#fn);
            ),
        )
    }
}

/// Rotate extrude modifier `rotate_extrude()` in SCAD.
/// This Rust type is regarded as 3D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone)]
pub struct RotateExtrude {
    /// The number of degrees to sweep.
    ///
    /// Starting at the positive X axis.
    /// The direction of the sweep follows the Right Hand Rule,
    /// hence a negative angle sweeps clockwise.
    #[builder(setter(into, strip_option), default)]
    pub angle: Option<Unit>,
    /// Specifies the starting angle of the extrusion,
    /// counter-clockwise from the positive X axis.
    ///
    /// Defaults to 0 if angle is specified, and 180 if not.
    #[builder(setter(into, strip_option), default)]
    pub start: Option<Unit>,
    /// `convexity` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub convexity: Option<u64>,
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

__impl_builder_sentence!(RotateExtrude);

impl ScadDisplay for RotateExtrude {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "rotate_extrude",
            __generate_scad_options!(
                ;
                ("angle", self.angle),
                ("start", self.start),
                ("convexity", self.convexity),
                ("$fa", self.fa),
                ("$fn", self.r#fn),
                ("$fs", self.fs);
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;
    use crate::ScadBuildable as _;

    #[test]
    fn test_translate3d() {
        assert_eq!(
            Translate3D::build_with(|tb| {
                let _ = tb.v([8., -4., 6.]);
            })
            .repr_scad(),
            "translate([8, -4, 6])"
        );
    }

    #[test]
    fn test_rotate3d() {
        assert_eq!(
            Rotate3D::build_with(|rb| {
                let _ = rb.deg([45., 0., 90.]);
            })
            .repr_scad(),
            "rotate(a = [45, 0, 90])"
        );
        assert_eq!(
            Rotate3D::build_with(|rb| {
                let _ = rb.rad([PI / 4., 0., PI / 2.]);
            })
            .repr_scad(),
            "rotate(a = [45, 0, 90])"
        );
        assert_eq!(
            Rotate3D::build_with(|rb| {
                let _ = rb.rad(PI / 4.).v([1., 1., 0.]);
            })
            .repr_scad(),
            "rotate(a = 45, v = [1, 1, 0])"
        );
    }

    #[test]
    fn test_mirror3d() {
        assert_eq!(
            Mirror3D::build_with(|mb| {
                let _ = mb.v([1., -1., 0.]);
            })
            .repr_scad(),
            "mirror([1, -1, 0])"
        );
    }

    #[test]
    fn test_scale3d() {
        assert_eq!(
            Scale3D::build_with(|sb| {
                let _ = sb.v([3., 2., 4.]);
            })
            .repr_scad(),
            "scale([3, 2, 4])"
        );
    }

    #[test]
    fn test_resize3d() {
        let mut r1 = Resize3DBuilder::default();
        _ = r1.size([3., 2., 1.]);
        assert_eq!(r1.clone().build().unwrap().repr_scad(), "resize([3, 2, 1])");
        assert_eq!(
            r1.clone().auto(true).build().unwrap().repr_scad(),
            "resize([3, 2, 1], auto = true)"
        );
        assert_eq!(
            r1.auto([true, false, true]).build().unwrap().repr_scad(),
            "resize([3, 2, 1], auto = [true, false, true])"
        );
    }

    #[test]
    fn test_multimatrix2d() {
        let m = AffineMatrix3D::new(1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.);
        assert_eq!(
            MultMatrix3D::build_with(|mb| {
                let _ = mb.m(m);
            })
            .repr_scad(),
            "multmatrix(m = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]])"
        );
    }

    #[test]
    fn test_linear_extrude() {
        assert_eq!(
            LinearExtrude::build_with(|lb| {
                let _ = lb.height(5.);
            })
            .repr_scad(),
            "linear_extrude(height = 5)"
        );
        assert_eq!(
            LinearExtrude::build_with(|lb| {
                let _ = lb.height(5.)
                    .v([0., 0.2, 1.])
                    .center(true)
                    .twist(180.)
                    .convexity(10_u64)
                    .slices(30_u64)
                    .scale(0.7)
                    .r#fn(20_u64)
                    ;
            })
            .repr_scad(),
            "linear_extrude(height = 5, v = [0, 0.2, 1], center = true, twist = 180, convexity = 10, slices = 30, scale = 0.7, $fn = 20)"
        );
    }

    #[test]
    fn test_rotate_extrude() {
        assert_eq!(
            RotateExtrude::build_with(|rb| {
                let _ = rb;
            })
            .repr_scad(),
            "rotate_extrude()"
        );
        assert_eq!(
            RotateExtrude::build_with(|rb| {
                let _ = rb.angle(180.).start(90.).convexity(10_u64).fa(5.);
            })
            .repr_scad(),
            "rotate_extrude(angle = 180, start = 90, convexity = 10, $fa = 5)"
        );
    }
}
