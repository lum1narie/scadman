use std::ops::{Add, Mul, Sub};

use ambassador::Delegate;
use derive_builder::Builder;
use derive_more::derive::From;
use nalgebra as na;

use crate::{
    ScadObject, __generate_scad_options, __get_children_impl, __impl_scad3d,
    internal::generate_body,
    scad_2d::Objects2D,
    scad_display::{ambassador_impl_ScadDisplay, ScadDisplay},
    value_type::{Angle, Color},
    AffineMatrix3D, Point3D, ScadObject2D, ScadObject3D, Unit,
};

use super::Objects3D;

/// Give an implementation of a modifier 3D object
/// that has no parameters and is applied to 3D objects.
macro_rules! __impl_operator_3d {
    ( $type:ident, $name:expr_2021 ) => {
        #[doc = concat!($name,
        " modifier `", $name, "()` in SCAD.
        This Rust type is regarded as 3D object and only applys to 3D objects.")]
        #[allow(missing_debug_implementations)]
        #[derive(Builder, Debug, Clone)]
        pub struct $type {
            /// Children objects to apply this modifier.
            #[builder(setter(name = "apply_to", into))]
            pub children: Objects3D,
        }
        $crate::__impl_scad3d!($type);

        impl ScadObject for $type {
            fn get_body(&self) -> String {
                generate_body(
                    $name,
                    __generate_scad_options!(
                        ;;
                    ),
                )
            }
            $crate::__get_children_impl!();
        }
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
    #[builder(setter(name = "apply_to", into))]
    pub children: Objects3D,
}

__impl_scad3d!(Translate3D);

impl ScadObject for Translate3D {
    fn get_body(&self) -> String {
        generate_body(
            "translate",
            __generate_scad_options!(
                ("", self.v);;
            ),
        )
    }
    __get_children_impl!();
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
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into))]
    pub children: Objects3D,
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

impl ScadObject for Rotate3D {
    fn get_body(&self) -> String {
        generate_body(
            "rotate",
            __generate_scad_options!(
                ("a", self.a);
                ("v", self.v);
            ),
        )
    }
    __get_children_impl!();
}

/// Scale modifier `scale()` in SCAD.
/// This Rust type is regarded as 3D object and only applys to 3D objects.
#[derive(Builder, Debug, Clone)]
pub struct Scale3D {
    /// Scaling vector.
    /// `v` option in SCAD.
    #[builder(setter(into))]
    pub v: Point3D,
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into))]
    pub children: Objects3D,
}

__impl_scad3d!(Scale3D);

impl ScadObject for Scale3D {
    fn get_body(&self) -> String {
        generate_body(
            "scale",
            __generate_scad_options!(
                ("", self.v);;
            ),
        )
    }
    __get_children_impl!();
}

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
    #[builder(setter(name = "apply_to", into))]
    pub children: Objects3D,
}

__impl_scad3d!(Resize3D);

impl ScadObject for Resize3D {
    fn get_body(&self) -> String {
        generate_body(
            "resize",
            __generate_scad_options!(
                ("", self.size);
                ("auto", self.auto);
            ),
        )
    }
    __get_children_impl!();
}

/// Mirror modifier `mirror()` in SCAD.
/// This Rust type is regarded as 3D object and only applys to 3D objects.
#[derive(Builder, Debug, Clone)]
pub struct Mirror3D {
    /// Normal vector of the mirror plane.
    #[builder(setter(into))]
    pub v: Point3D,
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into))]
    pub children: Objects3D,
}

__impl_scad3d!(Mirror3D);

impl ScadObject for Mirror3D {
    fn get_body(&self) -> String {
        generate_body(
            "mirror",
            __generate_scad_options!(
                ("", self.v);;
            ),
        )
    }
    __get_children_impl!();
}

/// Affine tranformation modifier `multmatrix()` in SCAD.
/// This Rust type is regarded as 3D object and only applys to 3D objects.
#[derive(Builder, Debug, Clone)]
pub struct MultMatrix3D {
    /// Affine transformation matrix for 3D vector.
    #[builder(setter(into))]
    pub m: AffineMatrix3D,
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into))]
    pub children: Objects3D,
}

__impl_scad3d!(MultMatrix3D);

impl ScadObject for MultMatrix3D {
    fn get_body(&self) -> String {
        generate_body(
            "multmatrix",
            __generate_scad_options!(
                ("m", self.m);;
            ),
        )
    }
    __get_children_impl!();
}

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
    #[builder(setter(name = "apply_to", into))]
    pub children: Objects3D,
}

__impl_scad3d!(Color3D);

impl ScadObject for Color3D {
    fn get_body(&self) -> String {
        generate_body(
            "color",
            __generate_scad_options!(
                (self.c.name(), self.c.clone());
                ("a", self.a);
            ),
        )
    }
    __get_children_impl!();
}

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
    #[builder(setter(name = "apply_to", into))]
    pub children: Objects2D,
}

__impl_scad3d!(LinearExtrude);

impl ScadObject for LinearExtrude {
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
    __get_children_impl!();
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
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into))]
    pub children: Objects2D,
}

__impl_scad3d!(RotateExtrude);

impl ScadObject for RotateExtrude {
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
    __get_children_impl!();
}

impl Add for Objects3D {
    type Output = Union3D;

    fn add(self, rhs: Self) -> Self::Output {
        Union3D::build_with(|ub| {
            let _ = ub.apply_to(Objects3D(
                self.iter().chain(rhs.iter()).cloned().collect::<Vec<_>>(),
            ));
        })
    }
}
impl Mul for Objects3D {
    type Output = Intersection3D;

    fn mul(self, rhs: Self) -> Self::Output {
        Intersection3D::build_with(|ib| {
            let _ = ib.apply_to(Objects3D(
                self.iter().chain(rhs.iter()).cloned().collect::<Vec<_>>(),
            ));
        })
    }
}
impl Sub for Objects3D {
    type Output = Difference3D;

    fn sub(self, rhs: Self) -> Self::Output {
        Difference3D::build_with(|db| {
            let _ = db.apply_to(Objects3D(
                self.iter().chain(rhs.iter()).cloned().collect::<Vec<_>>(),
            ));
        })
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;
    use crate::{
        any_scads2d, any_scads3d,
        scad_2d::SquareBuilder,
        scad_3d::{Cube, CubeBuilder, Sphere, SphereBuilder},
        value_type::{RGB, RGBA},
    };

    #[test]
    fn test_translate3d() {
        let children = any_scads3d![
            CubeBuilder::default().size(10.).build().unwrap(),
            SphereBuilder::default().r(5.).build().unwrap(),
        ];
        assert_eq!(
            Translate3DBuilder::default()
                .v([8., -4., 6.])
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "translate([8, -4, 6]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_rotate3d() {
        let children = any_scads3d![
            Cube::build_with(|cb| {
                let _ = cb.size(10.);
            }),
            Sphere::build_with(|sb| {
                let _ = sb.r(5.);
            }),
        ];
        assert_eq!(
            Rotate3D::build_with(|rb| {
                let _ = rb.deg([45., 0., 90.]).apply_to(children.clone());
            })
            .to_code(),
            "rotate(a = [45, 0, 90]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            Rotate3D::build_with(|rb| {
                let _ = rb.rad([PI / 4., 0., PI / 2.]).apply_to(children.clone());
            })
            .to_code(),
            "rotate(a = [45, 0, 90]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            Rotate3D::build_with(|rb| {
                let _ = rb.rad(PI / 4.).v([1., 1., 0.]).apply_to(children);
            })
            .to_code(),
            "rotate(a = 45, v = [1, 1, 0]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_mirror3d() {
        let children = any_scads3d![
            CubeBuilder::default().size(10.).build().unwrap(),
            SphereBuilder::default().r(5.).build().unwrap(),
        ];
        assert_eq!(
            Mirror3DBuilder::default()
                .v([1., -1., 0.])
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "mirror([1, -1, 0]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_scale3d() {
        let children = any_scads3d![
            CubeBuilder::default().size(10.).build().unwrap(),
            SphereBuilder::default().r(5.).build().unwrap(),
        ];
        assert_eq!(
            Scale3DBuilder::default()
                .v([3., 2., 4.])
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "scale([3, 2, 4]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_resize3d() {
        let children = any_scads3d![
            CubeBuilder::default().size(10.).build().unwrap(),
            SphereBuilder::default().r(5.).build().unwrap(),
        ];
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
        let children = any_scads3d![
            CubeBuilder::default().size(10.).build().unwrap(),
            SphereBuilder::default().r(5.).build().unwrap(),
        ];
        let m = AffineMatrix3D::new(1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.);
        assert_eq!(
            MultMatrix3DBuilder::default()
                .m(m)
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "multmatrix(m = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_color3d() {
        let children = any_scads3d![
            CubeBuilder::default().size(10.).build().unwrap(),
            SphereBuilder::default().r(5.).build().unwrap(),
        ];
        assert_eq!(
            Color3DBuilder::default()
                .c(RGB::new(0.3, 0.5, 0.2))
                .apply_to(children.clone())
                .build()
                .unwrap()
                .to_code(),
            "color(c = [0.3, 0.5, 0.2]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            Color3DBuilder::default()
                .c(RGB::new(0.3, 0.5, 0.2))
                .a(1.0)
                .apply_to(children.clone())
                .build()
                .unwrap()
                .to_code(),
            "color(c = [0.3, 0.5, 0.2], a = 1) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            Color3DBuilder::default()
                .c(RGBA::new(0.3, 0.5, 0.2, 1.0))
                .apply_to(children.clone())
                .build()
                .unwrap()
                .to_code(),
            "color(c = [0.3, 0.5, 0.2, 1]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            Color3DBuilder::default()
                .c("#C0FFEE".to_string())
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "color(\"#C0FFEE\") {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_hull() {
        let children = any_scads3d![
            CubeBuilder::default().size(10.).build().unwrap(),
            SphereBuilder::default().r(5.).build().unwrap(),
        ];
        assert_eq!(
            Hull3DBuilder::default()
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "hull() {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_minkowski() {
        let children = any_scads3d![
            CubeBuilder::default().size(10.).build().unwrap(),
            SphereBuilder::default().r(5.).build().unwrap(),
        ];
        assert_eq!(
            Minkowski3DBuilder::default()
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "minkowski() {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_binary_op() {
        let children = any_scads3d![
            CubeBuilder::default().size(10.).build().unwrap(),
            SphereBuilder::default().r(5.).build().unwrap(),
        ];
        assert_eq!(
            Union3DBuilder::default()
                .apply_to(children.clone())
                .build()
                .unwrap()
                .to_code(),
            "union() {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            Difference3DBuilder::default()
                .apply_to(children.clone())
                .build()
                .unwrap()
                .to_code(),
            "difference() {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            Intersection3DBuilder::default()
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "intersection() {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_linear_extrude() {
        let children = any_scads2d![SquareBuilder::default().size(10.).build().unwrap()];
        assert_eq!(
            LinearExtrudeBuilder::default()
                .apply_to(children.clone())
                .height(5.)
                .build()
                .unwrap()
                .to_code(),
            "linear_extrude(height = 5) {\n  square(size = 10);\n}"
        );
        assert_eq!(
            LinearExtrudeBuilder::default()
                .apply_to(children)
                .height(5.)
                .v([0., 0.2, 1.])
                .center(true)
                .twist(180.)
                .convexity(10_u64)
                .slices(30_u64)
                .scale(0.7)
                .r#fn(20_u64)
                .build()
                .unwrap()
                .to_code(),
            "linear_extrude(height = 5, v = [0, 0.2, 1], center = true, twist = 180, convexity = 10, slices = 30, scale = 0.7, $fn = 20) {\n  square(size = 10);\n}"
        );
    }

    #[test]
    fn test_rotate_extrude() {
        let children = any_scads2d![SquareBuilder::default().size(10.).build().unwrap()];
        assert_eq!(
            RotateExtrudeBuilder::default()
                .apply_to(children.clone())
                .build()
                .unwrap()
                .to_code(),
            "rotate_extrude() {\n  square(size = 10);\n}"
        );
        assert_eq!(
            RotateExtrudeBuilder::default()
                .apply_to(children.clone())
                .angle(180.)
                .start(90.)
                .convexity(10_u64)
                .fa(5.)
                .build()
                .unwrap()
                .to_code(),
            "rotate_extrude(angle = 180, start = 90, convexity = 10, $fa = 5) {\n  square(size = 10);\n}"
        );
    }

    #[test]
    fn test_multi_level() {
        let objs = any_scads3d![
            CubeBuilder::default().size(10.).build().unwrap(),
            SphereBuilder::default().r(5.).build().unwrap(),
        ];

        let scad = Rotate3DBuilder::default()
            .deg([45., 0., 90.])
            .apply_to(
                Translate3DBuilder::default()
                    .v([8., -4., 6.])
                    .apply_to(objs.clone())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        assert_eq!(
            scad.to_code(),
            "rotate(a = [45, 0, 90]) {\n  translate([8, -4, 6]) {\n    cube(size = 10);\n    sphere(r = 5);\n  }\n}"
        );
    }
}
