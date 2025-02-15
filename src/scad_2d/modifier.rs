use ambassador::Delegate;
use derive_builder::Builder;
use derive_more::derive::From;

use crate::{
    __generate_scad_options, __get_children_impl, __impl_scad2d,
    internal::generate_body,
    common::{AffineMatrix2D, Point2D, ScadObject, ScadObject2D, ScadObject3D, Unit},
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
        #[derive(Builder, Debug, Clone)]
        pub struct $type {
            /// Children objects to apply this modifier.
            #[builder(setter(name = "apply_to", into))]
            pub children: Vec<Box<dyn ScadObject2D>>,
        }
        $crate::__impl_scad2d!($type);

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
/// This Rust type is regarded as 2D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone)]
pub struct Translate2D {
    /// Translation vector.
    /// `v` option in SCAD.
    #[builder(setter(into))]
    pub v: Point2D,
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into))]
    pub children: Vec<Box<dyn ScadObject2D>>,
}

__impl_scad2d!(Translate2D);

impl ScadObject for Translate2D {
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
/// This Rust type is regarded as 2D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone)]
pub struct Rotate2D {
    /// Rotation angle.
    /// `a` option in SCAD.
    ///
    /// See also [`Angle`].
    #[builder(setter(custom))]
    pub a: Angle,
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into))]
    pub children: Vec<Box<dyn ScadObject2D>>,
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

impl ScadObject for Rotate2D {
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
/// This Rust type is regarded as 2D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone)]
pub struct Scale2D {
    /// Scaling vector.
    /// `v` option in SCAD.
    #[builder(setter(into))]
    pub v: Point2D,
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into))]
    pub children: Vec<Box<dyn ScadObject2D>>,
}

__impl_scad2d!(Scale2D);

impl ScadObject for Scale2D {
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
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into))]
    pub children: Vec<Box<dyn ScadObject2D>>,
}

__impl_scad2d!(Resize2D);

impl ScadObject for Resize2D {
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
/// This Rust type is regarded as 2D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone)]
pub struct Mirror2D {
    /// Normal vector of the mirror plane.
    #[builder(setter(into))]
    pub v: Point2D,
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into))]
    pub children: Vec<Box<dyn ScadObject2D>>,
}

__impl_scad2d!(Mirror2D);

impl ScadObject for Mirror2D {
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
/// This Rust type is regarded as 2D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone)]
pub struct MultMatrix2D {
    /// Affine transformation matrix for 2D vector.
    #[builder(setter(into))]
    pub m: AffineMatrix2D,
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into))]
    pub children: Vec<Box<dyn ScadObject2D>>,
}

__impl_scad2d!(MultMatrix2D);

impl ScadObject for MultMatrix2D {
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
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into))]
    pub children: Vec<Box<dyn ScadObject2D>>,
}

__impl_scad2d!(Color2D);

impl ScadObject for Color2D {
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
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into))]
    pub children: Vec<Box<dyn ScadObject2D>>,
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

impl ScadObject for Offset {
    fn get_body(&self) -> String {
        generate_body(
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
    __get_children_impl!();
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
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into))]
    pub children: Vec<Box<dyn ScadObject3D>>,
}

__impl_scad2d!(Projection);

impl ScadObject for Projection {
    fn get_body(&self) -> String {
        generate_body(
            "projection",
            __generate_scad_options!(
                ;("cut", self.cut);
            ),
        )
    }
    __get_children_impl!();
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;
    use crate::{
        any_scads2d, any_scads3d,
        scad_2d::{CircleBuilder, SquareBuilder},
        scad_3d::SphereBuilder,
        value_type::{RGB, RGBA},
    };

    #[test]
    fn test_translate2d() {
        let children = any_scads2d![
            SquareBuilder::default().size(10.).build().unwrap(),
            CircleBuilder::default().r(5.).build().unwrap(),
        ];
        assert_eq!(
            Translate2DBuilder::default()
                .v(Point2D::new(8., -4.))
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "translate([8, -4]) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
    }

    #[test]
    fn test_rotate2d() {
        let children = any_scads2d![
            SquareBuilder::default().size(10.).build().unwrap(),
            CircleBuilder::default().r(5.).build().unwrap(),
        ];
        assert_eq!(
            Rotate2DBuilder::default()
                .deg(45.)
                .apply_to(children.clone())
                .build()
                .unwrap()
                .to_code(),
            "rotate(45) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
        assert_eq!(
            Rotate2DBuilder::default()
                .rad(PI / 4.)
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "rotate(45) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
    }

    #[test]
    fn test_mirror2d() {
        let children = any_scads2d![
            SquareBuilder::default().size(10.).build().unwrap(),
            CircleBuilder::default().r(5.).build().unwrap(),
        ];
        assert_eq!(
            Mirror2DBuilder::default()
                .v(Point2D::new(1., -1.))
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "mirror([1, -1]) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
    }

    #[test]
    fn test_scale2d() {
        let children = any_scads2d![
            SquareBuilder::default().size(10.).build().unwrap(),
            CircleBuilder::default().r(5.).build().unwrap(),
        ];
        assert_eq!(
            Scale2DBuilder::default()
                .v(Point2D::new(3., 2.))
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "scale([3, 2]) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
    }

    #[test]
    fn test_resize2d() {
        let children = any_scads2d![
            SquareBuilder::default().size(10.).build().unwrap(),
            CircleBuilder::default().r(5.).build().unwrap(),
        ];
        let mut r1 = Resize2DBuilder::default();
        _ = r1.size(Point2D::new(3., 2.)).apply_to(children);
        assert_eq!(
            r1.clone().build().unwrap().to_code(),
            "resize([3, 2]) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
        assert_eq!(
            r1.clone().auto(true).build().unwrap().to_code(),
            "resize([3, 2], auto = true) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
        assert_eq!(
            r1.auto([true, false]).build().unwrap().to_code(),
            "resize([3, 2], auto = [true, false]) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
    }

    #[test]
    fn test_multimatrix2d() {
        let children = any_scads2d![
            SquareBuilder::default().size(10.).build().unwrap(),
            CircleBuilder::default().r(5.).build().unwrap(),
        ];
        let m = AffineMatrix2D::new(1., 2., 3., 4., 5., 6.);
        assert_eq!(
            MultMatrix2DBuilder::default()
                .m(m)
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "multmatrix(m = [[1, 2, 0, 3], [4, 5, 0, 6], [0, 0, 1, 0]]) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
    }

    #[test]
    fn test_color2d() {
        let children = any_scads2d![
            SquareBuilder::default().size(10.).build().unwrap(),
            CircleBuilder::default().r(5.).build().unwrap(),
        ];
        assert_eq!(
            Color2DBuilder::default()
                .c(RGB::new(0.3, 0.5, 0.2))
                .apply_to(children.clone())
                .build()
                .unwrap()
                .to_code(),
            "color(c = [0.3, 0.5, 0.2]) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
        assert_eq!(
            Color2DBuilder::default()
                .c(RGB::new(0.3, 0.5, 0.2))
                .a(1.0)
                .apply_to(children.clone())
                .build()
                .unwrap()
                .to_code(),
            "color(c = [0.3, 0.5, 0.2], a = 1) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
        assert_eq!(
            Color2DBuilder::default()
                .c(RGBA::new(0.3, 0.5, 0.2, 1.0))
                .apply_to(children.clone())
                .build()
                .unwrap()
                .to_code(),
            "color(c = [0.3, 0.5, 0.2, 1]) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
        assert_eq!(
            Color2DBuilder::default()
                .c("#C0FFEE".to_string())
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "color(\"#C0FFEE\") {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
    }

    #[test]
    fn test_offset() {
        let children = any_scads2d![
            SquareBuilder::default().size(10.).build().unwrap(),
            CircleBuilder::default().r(5.).build().unwrap(),
        ];
        assert_eq!(
            OffsetBuilder::default()
                .r(1.)
                .apply_to(children.clone())
                .build()
                .unwrap()
                .to_code(),
            "offset(r = 1) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
        assert_eq!(
            OffsetBuilder::default()
                .delta(1.)
                .apply_to(children.clone())
                .build()
                .unwrap()
                .to_code(),
            "offset(delta = 1) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
        assert_eq!(
            OffsetBuilder::default()
                .r(1.)
                .chamfer(true)
                .fs(10)
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "offset(r = 1, chamfer = true, $fs = 10) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
    }

    #[test]
    fn test_hull() {
        let children = any_scads2d![
            SquareBuilder::default().size(10.).build().unwrap(),
            CircleBuilder::default().r(5.).build().unwrap(),
        ];
        assert_eq!(
            Hull2DBuilder::default()
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "hull() {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
    }

    #[test]
    fn test_minkowski() {
        let children = any_scads2d![
            SquareBuilder::default().size(10.).build().unwrap(),
            CircleBuilder::default().r(5.).build().unwrap(),
        ];
        assert_eq!(
            Minkowski2DBuilder::default()
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "minkowski() {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
    }

    #[test]
    fn test_binary_op() {
        let children = any_scads2d![
            SquareBuilder::default().size(10.).build().unwrap(),
            CircleBuilder::default().r(5.).build().unwrap(),
        ];
        assert_eq!(
            Union2DBuilder::default()
                .apply_to(children.clone())
                .build()
                .unwrap()
                .to_code(),
            "union() {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
        assert_eq!(
            Difference2DBuilder::default()
                .apply_to(children.clone())
                .build()
                .unwrap()
                .to_code(),
            "difference() {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
        assert_eq!(
            Intersection2DBuilder::default()
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "intersection() {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
    }

    #[test]
    fn test_projection() {
        let children = any_scads3d![SphereBuilder::default().r(10.).build().unwrap()];
        assert_eq!(
            ProjectionBuilder::default()
                .apply_to(children.clone())
                .build()
                .unwrap()
                .to_code(),
            "projection() {\n  sphere(r = 10);\n}"
        );
        assert_eq!(
            ProjectionBuilder::default()
                .cut(true)
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "projection(cut = true) {\n  sphere(r = 10);\n}"
        );
    }

    #[test]
    fn test_multilevel() {
        let objs = any_scads2d![
            SquareBuilder::default().size(10.).build().unwrap(),
            CircleBuilder::default().r(5.).build().unwrap(),
        ];

        let scad = Rotate2DBuilder::default()
            .deg(45.)
            .apply_to(
                Translate2DBuilder::default()
                    .v(Point2D::new(8., -4.))
                    .apply_to(objs.clone())
                    .build()
                    .unwrap()
            )
            .build()
            .unwrap();

        assert_eq!(
            scad.to_code(),
            "rotate(45) {\n  translate([8, -4]) {\n    square(size = 10);\n    circle(r = 5);\n  }\n}"
        );
    }
}
