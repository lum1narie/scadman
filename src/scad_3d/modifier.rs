use ambassador::Delegate;
use derive_builder::Builder;
use derive_more::derive::From;

use crate::{
    ScadObject, __generate_scad_options, __get_children_impl, __impl_scad3d,
    internal::generate_body,
    scad_display::{ambassador_impl_ScadDisplay, ScadDisplay},
    value_type::{Angle, Color},
    AffineMatrix3D, Point3D, ScadObject3D, Unit,
};

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
            #[builder(setter(name = "apply_to"))]
            pub children: Vec<Box<dyn ScadObject3D>>,
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
    #[builder(setter(name = "apply_to"))]
    pub children: Vec<Box<dyn ScadObject3D>>,
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

/// Rotate modifier `rotate()` in SCAD.
/// This Rust type is regarded as 3D object and only applys to 3D objects.
#[derive(Builder, Debug, Clone)]
pub struct Rotate3D {
    /// Rotation angle.
    /// `a` option in SCAD.
    ///
    /// See also [`Angle`].
    #[builder(setter(custom))]
    pub a: Angle,
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to"))]
    pub children: Vec<Box<dyn ScadObject3D>>,
}

__impl_scad3d!(Rotate3D);

impl Rotate3DBuilder {
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

impl ScadObject for Rotate3D {
    fn get_body(&self) -> String {
        generate_body(
            "rotate",
            __generate_scad_options!(
                ("", self.a);;
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
    #[builder(setter(name = "apply_to"))]
    pub children: Vec<Box<dyn ScadObject3D>>,
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
    #[builder(setter(name = "apply_to"))]
    pub children: Vec<Box<dyn ScadObject3D>>,
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
    #[builder(setter(name = "apply_to"))]
    pub children: Vec<Box<dyn ScadObject3D>>,
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
    #[builder(setter(name = "apply_to"))]
    pub children: Vec<Box<dyn ScadObject3D>>,
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
    #[builder(setter(name = "apply_to"))]
    pub children: Vec<Box<dyn ScadObject3D>>,
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

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    use crate::{
        any_scads3d,
        scad_3d::{CubeBuilder, SphereBuilder},
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
                .v(Point3D::new(8., -4., 6.))
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
            CubeBuilder::default().size(10.).build().unwrap(),
            SphereBuilder::default().r(5.).build().unwrap(),
        ];
        assert_eq!(
            Rotate3DBuilder::default()
                .deg(45.)
                .apply_to(children.clone())
                .build()
                .unwrap()
                .to_code(),
            "rotate(45) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            Rotate3DBuilder::default()
                .rad(PI / 4.)
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "rotate(45) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
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
                .v(Point3D::new(1., -1., 0.))
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
                .v(Point3D::new(3., 2., 4.))
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
        _ = r1.size(Point3D::new(3., 2., 1.)).apply_to(children);
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
        todo!()
    }

    #[test]
    fn test_rotate_extrude() {
        todo!()
    }

    #[test]
    fn test_multi_level() {
        let objs = any_scads3d![
            CubeBuilder::default().size(10.).build().unwrap(),
            SphereBuilder::default().r(5.).build().unwrap(),
        ];

        let scad = Rotate3DBuilder::default()
            .deg(45.)
            .apply_to(
                Translate3DBuilder::default()
                    .v(Point3D::new(8., -4., 6.))
                    .apply_to(objs.clone())
                    .build()
                    .unwrap()
                    .into(),
            )
            .build()
            .unwrap();

        assert_eq!(
            scad.to_code(),
            "rotate(45) {\n  translate([8, -4, 6]) {\n    cube(size = 10);\n    sphere(r = 5);\n  }\n}"
        );
    }
}
