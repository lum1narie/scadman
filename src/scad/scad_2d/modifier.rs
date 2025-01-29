use derive_builder::Builder;

use crate::{
    __generate_scad_options, __get_children_impl, __impl_scad2d,
    scad::{generate_body, Angle, Point2D, ScadObject, ScadObject2D, ScadObject3D, Unit},
};

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
        scad::{CircleBuilder, SphereBuilder, SquareBuilder},
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
}
