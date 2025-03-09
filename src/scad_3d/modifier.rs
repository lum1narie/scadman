use std::ops::{Add, Mul, Sub};

use ambassador::Delegate;
use derive_builder::Builder;
use derive_more::derive::From;
use nalgebra as na;

use crate::{
    ScadObjectTrait, __generate_scad_options, __impl_scad3d,
    internal::generate_body,
    scad_display::{ambassador_impl_ScadDisplay, ScadDisplay},
    value_type::{Angle, Color},
    AffineMatrix3D, Point3D, Unit, __impl_modifier, __impl_modifier_to_code,
    scad_2d::ScadObject2D,
};

use super::ScadObject3D;

/// Give an implementation of a modifier 3D object
/// that has no parameters and is applied to 3D objects.
macro_rules! __impl_operator_3d {
    ( $type:ident, $name:expr_2021 ) => {
        #[doc = concat!($name,
        " modifier `", $name, "()` in SCAD.
        This Rust type is regarded as 3D object and only applys to 3D objects.")]
        #[allow(missing_debug_implementations)]
        #[derive(derive_builder::Builder, Debug, Clone)]
        pub struct $type {
            /// Children objects to apply this modifier.
            #[builder(setter(name = "apply_to", into), default)]
            pub children: Vec<$crate::scad_3d::ScadObject3D>,
        }
        $crate::__impl_scad3d!($type);

        impl $crate::ScadObjectTrait for $type {
            fn get_body(&self) -> String {
                generate_body(
                    $name,
                    $crate::__generate_scad_options!(
                        ;;
                    ),
                )
            }

            $crate::__impl_modifier_to_code!();
        }

        impl Default for $type {
            fn default() -> Self {
                Self::new()
            }
        }

        impl $type {
            /// generate new blank object
            pub const fn new() -> Self {
                Self {children: Vec::new()}
            }
        }

        $crate::__impl_modifier!($type, $crate::scad_3d::ScadObject3D);
    };
}

/// Translate modifier `translate()` in SCAD.
/// This Rust type is regarded as 3D object and only applys to 3D objects.
#[derive(Builder, Debug, Clone)]
pub struct Translate3D {
    /// Translation vector.
    /// `v` option in SCAD.
    #[builder(setter(into))]
    pub v: Point3D,
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into), default)]
    pub children: Vec<ScadObject3D>,
}

__impl_scad3d!(Translate3D);

impl ScadObjectTrait for Translate3D {
    fn get_body(&self) -> String {
        generate_body(
            "translate",
            __generate_scad_options!(
                ("", self.v);;
            ),
        )
    }

    __impl_modifier_to_code!();
}

__impl_modifier!(Translate3D, ScadObject3D);

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
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into), default)]
    pub children: Vec<ScadObject3D>,
}

__impl_scad3d!(Rotate3D);

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

impl ScadObjectTrait for Rotate3D {
    fn get_body(&self) -> String {
        generate_body(
            "rotate",
            __generate_scad_options!(
                ("a", self.a);
                ("v", self.v);
            ),
        )
    }

    __impl_modifier_to_code!();
}

__impl_modifier!(Rotate3D, ScadObject3D);

/// Scale modifier `scale()` in SCAD.
/// This Rust type is regarded as 3D object and only applys to 3D objects.
#[derive(Builder, Debug, Clone)]
pub struct Scale3D {
    /// Scaling vector.
    /// `v` option in SCAD.
    #[builder(setter(into))]
    pub v: Point3D,
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into), default)]
    pub children: Vec<ScadObject3D>,
}

__impl_scad3d!(Scale3D);

impl ScadObjectTrait for Scale3D {
    fn get_body(&self) -> String {
        generate_body(
            "scale",
            __generate_scad_options!(
                ("", self.v);;
            ),
        )
    }

    __impl_modifier_to_code!();
}

__impl_modifier!(Scale3D, ScadObject3D);

/// `auto` option in 3D resize modifier.
#[derive(Copy, Clone, Debug, PartialEq, Eq, From, Delegate)]
#[delegate(ScadDisplay)]
pub enum ResizeAuto {
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
    pub auto: Option<ResizeAuto>,
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into), default)]
    pub children: Vec<ScadObject3D>,
}

__impl_scad3d!(Resize3D);

impl ScadObjectTrait for Resize3D {
    fn get_body(&self) -> String {
        generate_body(
            "resize",
            __generate_scad_options!(
                ("", self.size);
                ("auto", self.auto);
            ),
        )
    }

    __impl_modifier_to_code!();
}

__impl_modifier!(Resize3D, ScadObject3D);

/// Mirror modifier `mirror()` in SCAD.
/// This Rust type is regarded as 3D object and only applys to 3D objects.
#[derive(Builder, Debug, Clone)]
pub struct Mirror3D {
    /// Normal vector of the mirror plane.
    #[builder(setter(into))]
    pub v: Point3D,
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into), default)]
    pub children: Vec<ScadObject3D>,
}

__impl_scad3d!(Mirror3D);

impl ScadObjectTrait for Mirror3D {
    fn get_body(&self) -> String {
        generate_body(
            "mirror",
            __generate_scad_options!(
                ("", self.v);;
            ),
        )
    }

    __impl_modifier_to_code!();
}

__impl_modifier!(Mirror3D, ScadObject3D);

/// Affine tranformation modifier `multmatrix()` in SCAD.
/// This Rust type is regarded as 3D object and only applys to 3D objects.
#[derive(Builder, Debug, Clone)]
pub struct MultMatrix3D {
    /// Affine transformation matrix for 3D vector.
    #[builder(setter(into))]
    pub m: AffineMatrix3D,
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into), default)]
    pub children: Vec<ScadObject3D>,
}

__impl_scad3d!(MultMatrix3D);

impl ScadObjectTrait for MultMatrix3D {
    fn get_body(&self) -> String {
        generate_body(
            "multmatrix",
            __generate_scad_options!(
                ("m", self.m);;
            ),
        )
    }

    __impl_modifier_to_code!();
}

__impl_modifier!(MultMatrix3D, ScadObject3D);

/// Color modifier `color()` in SCAD.
/// This Rust type is regarded as 3D object and only applys to 3D objects.
#[derive(Builder, Debug, Clone)]
pub struct Color3D {
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
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into), default)]
    pub children: Vec<ScadObject3D>,
}

__impl_scad3d!(Color3D);

impl ScadObjectTrait for Color3D {
    fn get_body(&self) -> String {
        generate_body(
            "color",
            __generate_scad_options!(
                (self.c.name(), self.c.clone());
                ("a", self.a);
            ),
        )
    }

    __impl_modifier_to_code!();
}

__impl_modifier!(Color3D, ScadObject3D);

__impl_operator_3d!(Hull3D, "hull");
__impl_operator_3d!(Minkowski3D, "minkowski");
__impl_operator_3d!(Union3D, "union");
__impl_operator_3d!(Difference3D, "difference");
__impl_operator_3d!(Intersection3D, "intersection");

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
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into), default)]
    pub children: Vec<ScadObject2D>,
}

__impl_scad3d!(LinearExtrude);

impl ScadObjectTrait for LinearExtrude {
    fn get_body(&self) -> String {
        generate_body(
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

    __impl_modifier_to_code!();
}

__impl_modifier!(LinearExtrude, ScadObject2D);

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
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into), default)]
    pub children: Vec<ScadObject2D>,
}

__impl_scad3d!(RotateExtrude);

impl ScadObjectTrait for RotateExtrude {
    fn get_body(&self) -> String {
        generate_body(
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

    __impl_modifier_to_code!();
}

__impl_modifier!(RotateExtrude, ScadObject2D);

impl Add for ScadObject3D {
    type Output = Union3D;

    fn add(self, rhs: Self) -> Self::Output {
        Union3D::build_with(|ub| {
            let _ = ub.apply_to(vec![self, rhs]);
        })
    }
}
impl Mul for ScadObject3D {
    type Output = Intersection3D;

    fn mul(self, rhs: Self) -> Self::Output {
        Intersection3D::build_with(|ib| {
            let _ = ib.apply_to(vec![self, rhs]);
        })
    }
}
impl Sub for ScadObject3D {
    type Output = Difference3D;

    fn sub(self, rhs: Self) -> Self::Output {
        Difference3D::build_with(|db| {
            let _ = db.apply_to(vec![self, rhs]);
        })
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;
    use crate::{
        objects_2d, objects_3d,
        scad_2d::Square,
        scad_3d::{Cube, Sphere},
        value_type::{RGB, RGBA},
        ScadModifier as _,
    };

    fn get_children() -> Vec<ScadObject3D> {
        objects_3d![
            Cube::build_with(|cb| {
                let _ = cb.size(10.);
            }),
            Sphere::build_with(|cb| {
                let _ = cb.r(5.);
            }),
        ]
    }

    #[test]
    fn test_translate3d() {
        let children = get_children();
        assert_eq!(
            Translate3D::build_with(|tb| {
                let _ = tb.v([8., -4., 6.]);
            })
            .apply_to(&children)
            .to_code(),
            "translate([8, -4, 6]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_rotate3d() {
        let children = get_children();
        assert_eq!(
            Rotate3D::build_with(|rb| {
                let _ = rb.deg([45., 0., 90.]);
            })
            .apply_to(&children)
            .to_code(),
            "rotate(a = [45, 0, 90]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            Rotate3D::build_with(|rb| {
                let _ = rb.rad([PI / 4., 0., PI / 2.]);
            })
            .apply_to(&children)
            .to_code(),
            "rotate(a = [45, 0, 90]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            Rotate3D::build_with(|rb| {
                let _ = rb.rad(PI / 4.).v([1., 1., 0.]);
            })
            .apply_to(&children)
            .to_code(),
            "rotate(a = 45, v = [1, 1, 0]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_mirror3d() {
        let children = get_children();
        assert_eq!(
            Mirror3D::build_with(|mb| {
                let _ = mb.v([1., -1., 0.]);
            })
            .apply_to(&children)
            .to_code(),
            "mirror([1, -1, 0]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_scale3d() {
        let children = get_children();
        assert_eq!(
            Scale3D::build_with(|sb| {
                let _ = sb.v([3., 2., 4.]);
            })
            .apply_to(&children)
            .to_code(),
            "scale([3, 2, 4]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_resize3d() {
        let children = get_children();
        let mut r1 = Resize3DBuilder::default();
        _ = r1.size([3., 2., 1.]).apply_to(children);
        assert_eq!(
            r1.clone().build().unwrap().to_code(),
            "resize([3, 2, 1]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            r1.clone().auto(true).build().unwrap().to_code(),
            "resize([3, 2, 1], auto = true) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            r1.auto([true, false, true]).build().unwrap().to_code(),
            "resize([3, 2, 1], auto = [true, false, true]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_multimatrix2d() {
        let children = get_children();
        let m = AffineMatrix3D::new(1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.);
        assert_eq!(
            MultMatrix3D::build_with(|mb| {
            let _ =     mb.m(m);
            }).apply_to(&children).to_code(),
            "multmatrix(m = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_color3d() {
        let children = get_children();
        assert_eq!(
            Color3D::build_with(|cb| {
                let _ = cb.c(RGB::new(0.3, 0.5, 0.2));
            })
            .apply_to(&children)
            .to_code(),
            "color(c = [0.3, 0.5, 0.2]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            Color3D::build_with(|cb| {
                let _ = cb.c(RGB::new(0.3, 0.5, 0.2)).a(1.0);
            })
            .apply_to(&children)
            .to_code(),
            "color(c = [0.3, 0.5, 0.2], a = 1) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            Color3D::build_with(|cb| {
                let _ = cb.c(RGBA::new(0.3, 0.5, 0.2, 1.0));
            })
            .apply_to(&children)
            .to_code(),
            "color(c = [0.3, 0.5, 0.2, 1]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            Color3D::build_with(|cb| {
                let _ = cb.c("#C0FFEE".to_string());
            })
            .apply_to(&children)
            .to_code(),
            "color(\"#C0FFEE\") {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_hull() {
        let children = get_children();
        assert_eq!(
            Hull3D::new().apply_to(&children).to_code(),
            "hull() {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_minkowski() {
        let children = get_children();
        assert_eq!(
            Minkowski3D::new().apply_to(&children).to_code(),
            "minkowski() {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_binary_op() {
        let children = get_children();
        assert_eq!(
            Union3D::new().apply_to(&children).to_code(),
            "union() {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            Difference3D::new().apply_to(&children).to_code(),
            "difference() {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            Intersection3D::new().apply_to(&children).to_code(),
            "intersection() {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );

        assert_eq!(
            (children[0].clone() + children[1].clone()).to_code(),
            "union() {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            (children[0].clone() - children[1].clone()).to_code(),
            "difference() {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            (children[0].clone() * children[1].clone()).to_code(),
            "intersection() {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_linear_extrude() {
        let children = objects_2d![Square::build_with(|sb| {
            let _ = sb.size(10.);
        })];
        assert_eq!(
            LinearExtrude::build_with(|lb| {
                let _ = lb.height(5.);
            })
            .apply_to(&children)
            .to_code(),
            "linear_extrude(height = 5) {\n  square(size = 10);\n}"
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
                .r#fn(20_u64);
            })
            .apply_to(&children)
            .to_code(),
            "linear_extrude(height = 5, v = [0, 0.2, 1], center = true, twist = 180, convexity = 10, slices = 30, scale = 0.7, $fn = 20) {\n  square(size = 10);\n}"
        );
    }

    #[test]
    fn test_rotate_extrude() {
        let children = objects_2d![Square::build_with(|sb| {
            let _ = sb.size(10.);
        })];
        assert_eq!(
            RotateExtrude::build_with(|_| {})
                .apply_to(&children)
                .to_code(),
            "rotate_extrude() {\n  square(size = 10);\n}"
        );
        assert_eq!(
            RotateExtrude::build_with(|rb| {
                let _ = rb.angle(180.)
                    .start(90.)
                    .convexity(10_u64)
                    .fa(5.);
                })
            .apply_to(&children)
            .to_code(),
            "rotate_extrude(angle = 180, start = 90, convexity = 10, $fa = 5) {\n  square(size = 10);\n}"
        );
    }

    #[test]
    fn test_multi_level() {
        let objs = get_children();
        let scad = Rotate3D::build_with(|rb| {
            let _ = rb
                .deg([45., 0., 90.])
                .apply_to(objects_3d![Translate3D::build_with(|tb| {
                    let _ = tb.v([8., -4., 6.]).apply_to(objs);
                })]);
        });

        assert_eq!(
            scad.to_code(),
            "rotate(a = [45, 0, 90]) {\n  translate([8, -4, 6]) {\n    cube(size = 10);\n    sphere(r = 5);\n  }\n}"
        );
    }
}
