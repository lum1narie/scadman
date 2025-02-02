use ambassador::Delegate;
use derive_builder::Builder;
use derive_more::derive::From;
use nalgebra as na;

use crate::{
    __generate_scad_options, __get_children_impl, __impl_scad2d,
    scad::{
        ambassador_impl_ScadDisplay, generate_body, AffineMatrix2D, Angle, Color, Point2D,
        ScadDisplay, ScadObject, ScadObject2D, ScadObject3D, Unit,
    },
};

macro_rules! __impl_operator_2d {
    ( $type:ident, $name:expr ) => {
        #[derive(Builder, Debug, Clone)]
        pub struct $type {
            #[builder(setter(name = "apply_to"))]
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

#[derive(Builder, Debug, Clone)]
pub struct Translate2D {
    pub v: Point2D,
    #[builder(setter(name = "apply_to"))]
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

#[derive(Builder, Debug, Clone)]
pub struct Rotate2D {
    #[builder(setter(custom))]
    pub a: Angle,
    #[builder(setter(name = "apply_to"))]
    pub children: Vec<Box<dyn ScadObject2D>>,
}

__impl_scad2d!(Rotate2D);

impl Rotate2DBuilder {
    pub fn deg(&mut self, value: Unit) -> &mut Self {
        let new = self;
        new.a = Some(Angle::Deg(value));
        new
    }
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

#[derive(Builder, Debug, Clone)]
pub struct Scale2D {
    pub v: Point2D,
    #[builder(setter(name = "apply_to"))]
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

#[derive(Copy, Clone, Debug, PartialEq, From, Delegate)]
#[delegate(ScadDisplay)]
pub enum ResizeAuto {
    B(bool),
    V([bool; 2]),
}

#[derive(Builder, Debug, Clone)]
pub struct Resize2D {
    pub size: Point2D,
    #[builder(setter(into, strip_option), default)]
    pub auto: Option<ResizeAuto>,
    #[builder(setter(name = "apply_to"))]
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

#[derive(Builder, Debug, Clone)]
pub struct Mirror2D {
    pub v: Point2D,
    #[builder(setter(name = "apply_to"))]
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

#[derive(Builder, Debug, Clone)]
pub struct MultMatrix2D {
    pub m: AffineMatrix2D,
    #[builder(setter(name = "apply_to"))]
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

#[derive(Builder, Debug, Clone)]
pub struct Color2D {
    #[builder(setter(into))]
    pub c: Color,
    #[builder(setter(into, strip_option), default)]
    pub a: Option<Unit>,
    #[builder(setter(name = "apply_to"))]
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

#[derive(Copy, Clone, Debug, PartialEq, Delegate)]
#[delegate(ScadDisplay)]
pub enum OffsetSize {
    R(Unit),
    Delta(Unit),
}

impl OffsetSize {
    pub fn name(self) -> &'static str {
        match self {
            OffsetSize::R(_) => "r",
            OffsetSize::Delta(_) => "delta",
        }
    }
}

#[derive(Builder, Debug, Clone)]
pub struct Offset {
    #[builder(setter(custom))]
    pub size: OffsetSize,
    #[builder(setter(into, strip_option), default)]
    pub chamfer: Option<bool>,
    #[builder(setter(into, strip_option), default)]
    pub fa: Option<Unit>,
    #[builder(setter(into, strip_option), default)]
    pub r#fn: Option<u64>,
    #[builder(setter(into, strip_option), default)]
    pub fs: Option<Unit>,
    #[builder(setter(name = "apply_to"))]
    pub children: Vec<Box<dyn ScadObject2D>>,
}

__impl_scad2d!(Offset);

impl OffsetBuilder {
    pub fn r(&mut self, value: Unit) -> &mut Self {
        let new = self;
        new.size = Some(OffsetSize::R(value));
        new
    }
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

#[derive(Builder, Debug, Clone)]
pub struct Projection {
    #[builder(setter(into, strip_option), default)]
    pub cut: Option<bool>,
    #[builder(setter(name = "apply_to"))]
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

    use crate::{
        any_scads2d, any_scads3d,
        scad::{CircleBuilder, SphereBuilder, SquareBuilder, RGB, RGBA},
    };

    use super::*;

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
        )
    }

    #[test]
    fn test_resize2d() {
        let children = any_scads2d![
            SquareBuilder::default().size(10.).build().unwrap(),
            CircleBuilder::default().r(5.).build().unwrap(),
        ];
        let r1 = {
            let mut r = Resize2DBuilder::default();
            r.size(Point2D::new(3., 2.)).apply_to(children);
            r
        };
        assert_eq!(
            r1.clone().build().unwrap().to_code(),
            "resize([3, 2]) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
        assert_eq!(
            r1.clone().auto(true).build().unwrap().to_code(),
            "resize([3, 2], auto = true) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
        assert_eq!(
            r1.clone()
                .auto([true, false])
                .build()
                .unwrap()
                .to_code(),
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
                    .into(),
            )
            .build()
            .unwrap();

        assert_eq!(
            scad.to_code(),
            "rotate(45) {\n  translate([8, -4]) {\n    square(size = 10);\n    circle(r = 5);\n  }\n}"
        );
    }
}
