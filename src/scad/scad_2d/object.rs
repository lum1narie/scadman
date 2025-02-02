use ambassador::Delegate;
use derive_builder::Builder;
use derive_more::derive::From;

use crate::{
    __generate_scad_options, __impl_scad2d,
    scad::{
        ambassador_impl_ScadDisplay, generate_body, Point2D, ScadDisplay, ScadObject, ScadObject2D,
        Unit,
    },
};

#[derive(Copy, Clone, Debug, PartialEq, From, Delegate)]
#[delegate(ScadDisplay)]
pub enum SquareSize {
    N(Unit),
    V(Point2D),
}

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Square {
    #[builder(setter(into))]
    pub size: SquareSize,
    #[builder(setter(into, strip_option), default)]
    pub center: Option<bool>,
}

__impl_scad2d!(Square);

impl ScadObject for Square {
    fn get_body(&self) -> String {
        generate_body(
            "square",
            __generate_scad_options!(
                ("size", self.size);
                ("center", self.center);
            ),
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Delegate)]
#[delegate(ScadDisplay)]
pub enum CircleSize {
    Radius(Unit),
    Diameter(Unit),
}

impl CircleSize {
    fn name(&self) -> &'static str {
        match self {
            CircleSize::Radius(_) => "r",
            CircleSize::Diameter(_) => "d",
        }
    }
}

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Circle {
    #[builder(setter(custom))]
    pub size: CircleSize,
    #[builder(setter(into, strip_option), default)]
    pub fa: Option<Unit>,
    #[builder(setter(into, strip_option), default)]
    pub r#fn: Option<u64>,
    #[builder(setter(into, strip_option), default)]
    pub fs: Option<Unit>,
}

__impl_scad2d!(Circle);

impl CircleBuilder {
    pub fn r(&mut self, value: Unit) -> &mut Self {
        let new = self;
        new.size = Some(CircleSize::Radius(value));
        new
    }
    pub fn d(&mut self, value: Unit) -> &mut Self {
        let new = self;
        new.size = Some(CircleSize::Diameter(value));
        new
    }
}

impl ScadObject for Circle {
    fn get_body(&self) -> String {
        generate_body(
            "circle",
            __generate_scad_options!(
                (self.size.name(), self.size);
                ("$fa", self.fa), ("$fn", self.r#fn), ("$fs", self.fs);
            ),
        )
    }
}

#[derive(Builder, Clone, Debug, PartialEq)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct Polygon {
    pub points: Vec<Point2D>,
    #[builder(setter(into, strip_option), default)]
    pub paths: Option<Vec<Vec<usize>>>,
    #[builder(setter(into, strip_option), default)]
    pub convexity: Option<u64>,
}

__impl_scad2d!(Polygon);

impl PolygonBuilder {
    fn validate(&self) -> Result<(), String> {
        (|| -> Option<Result<(), String>> {
            let pts: Vec<Point2D> = self.points.clone()?;
            let pas: Vec<Vec<usize>> = self.paths.clone()??;

            for (i, pa) in pas.into_iter().enumerate() {
                for (j, vtx) in pa.into_iter().enumerate() {
                    if vtx >= pts.len() {
                        return Some(Err(format!(
                            "path index out of bounds: [{}][{}]:{}",
                            i, j, vtx
                        )));
                    }
                }
            }

            Some(Ok(()))
        })()
        .unwrap_or(Ok(()))
    }
}

impl ScadObject for Polygon {
    fn get_body(&self) -> String {
        generate_body(
            "polygon",
            __generate_scad_options!(
                ("points", self.points.clone());
                ("paths", self.paths.clone()), ("convexity", self.convexity);
            ),
        )
    }
}

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Text {
    #[builder(setter(into))]
    pub text: String,
    #[builder(setter(into, strip_option), default)]
    pub size: Option<Unit>,
    #[builder(setter(into, strip_option), default)]
    pub font: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub halign: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub valign: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub spacing: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub direction: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub language: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub script: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub r#fn: Option<u64>,
}

__impl_scad2d!(Text);

impl ScadObject for Text {
    fn get_body(&self) -> String {
        generate_body(
            "text",
            __generate_scad_options!(
                ("", self.text.clone());
                ("font", self.font.clone()),
                ("size", self.size),
                ("halign", self.halign.clone()),
                ("valign", self.valign.clone()),
                ("spacing", self.spacing.clone()),
                ("direction", self.direction.clone()),
                ("language", self.language.clone()),
                ("script", self.script.clone()),
                ("$fn", self.r#fn);
            ),
        )
    }
}

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Import2D {
    #[builder(setter(into))]
    pub file: String,
    #[builder(setter(into, strip_option), default)]
    pub convexity: Option<u64>,
    #[builder(setter(into, strip_option), default)]
    pub id: Option<u64>,
    #[builder(setter(into, strip_option), default)]
    pub layer: Option<u64>,
    #[builder(setter(into, strip_option), default)]
    pub fa: Option<Unit>,
    #[builder(setter(into, strip_option), default)]
    pub r#fn: Option<u64>,
    #[builder(setter(into, strip_option), default)]
    pub fs: Option<Unit>,
}

__impl_scad2d!(Import2D);

impl ScadObject for Import2D {
    fn get_body(&self) -> String {
        generate_body(
            "import",
            __generate_scad_options!(
                ("", self.file.clone());
                ("convexity", self.convexity), ("id", self.id),
                ("layer", self.layer),
                ("$fa", self.fa), ("$fn", self.r#fn), ("$fs", self.fs);
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square() {
        assert_eq!(
            SquareBuilder::default()
                .size(3.0)
                .build()
                .unwrap()
                .to_code(),
            "square(size = 3);"
        );
        assert_eq!(
            SquareBuilder::default()
                .size(3.0)
                .center(true)
                .build()
                .unwrap()
                .to_code(),
            "square(size = 3, center = true);"
        );
        assert_eq!(
            SquareBuilder::default()
                .size(Point2D::new(3., 2.))
                .build()
                .unwrap()
                .to_code(),
            "square(size = [3, 2]);"
        );
        assert_eq!(
            SquareBuilder::default()
                .size(Point2D::new(3., 2.))
                .center(true)
                .build()
                .unwrap()
                .to_code(),
            "square(size = [3, 2], center = true);"
        );
        assert!(SquareBuilder::default().center(true).build().is_err())
    }

    #[test]
    fn test_circle() {
        assert_eq!(
            CircleBuilder::default().r(3.0).build().unwrap().to_code(),
            "circle(r = 3);"
        );
        assert_eq!(
            CircleBuilder::default().d(4.0).build().unwrap().to_code(),
            "circle(d = 4);"
        );
        assert_eq!(
            CircleBuilder::default()
                .r(3.0)
                .fa(0.5)
                .r#fn(20 as u64)
                .build()
                .unwrap()
                .to_code(),
            "circle(r = 3, $fa = 0.5, $fn = 20);"
        );
        assert_eq!(
            CircleBuilder::default()
                .r(3.0)
                .fs(40.)
                .fa(0.5)
                .build()
                .unwrap()
                .to_code(),
            "circle(r = 3, $fa = 0.5, $fs = 40);"
        );
        assert!(CircleBuilder::default()
            .fa(0.5)
            .r#fn(20 as u64)
            .fs(40.)
            .build()
            .is_err());
    }

    #[test]
    fn test_polygon() {
        let p0 = {
            let mut p = PolygonBuilder::default();
            p.points(vec![
                Point2D::new(1., 1.),
                Point2D::new(-1., 2.),
                Point2D::new(0., 0.),
            ]);
            p
        };
        assert_eq!(
            p0.clone().build().unwrap().to_code(),
            "polygon(points = [[1, 1], [-1, 2], [0, 0]]);"
        );
        assert_eq!(
            p0.clone()
                .paths(vec![vec![0, 2, 1]])
                .build()
                .unwrap()
                .to_code(),
            "polygon(points = [[1, 1], [-1, 2], [0, 0]], paths = [[0, 2, 1]]);"
        );
        assert_eq!(
            p0.clone().convexity(2 as u64).build().unwrap().to_code(),
            "polygon(points = [[1, 1], [-1, 2], [0, 0]], convexity = 2);"
        );

        let p1 = {
            let mut p = PolygonBuilder::default();
            p.points(vec![
                Point2D::new(2., 0.),
                Point2D::new(1., 1.),
                Point2D::new(-1., 1.),
                Point2D::new(1., 0.),
                Point2D::new(0.5, 0.5),
                Point2D::new(-0.5, 0.5),
            ]);
            p
        };
        assert_eq!(
            p1.clone().paths(vec![vec![0, 1, 2], vec![3, 4, 5]]).build().unwrap().to_code(),
            "polygon(points = [[2, 0], [1, 1], [-1, 1], [1, 0], [0.5, 0.5], [-0.5, 0.5]], paths = [[0, 1, 2], [3, 4, 5]]);"
        );
        assert_eq!(
            p1.clone()
                .paths(vec![vec![0, 1, 2], vec![6, 4, 5]])
                .build()
                .err()
                .map(|e| e.to_string())
                .unwrap_or_default(),
            "path index out of bounds: [1][0]:6"
        );
    }

    #[test]
    fn test_text() {
        assert_eq!(
            TextBuilder::default()
                .text("Hello World")
                .build()
                .unwrap()
                .to_code(),
            "text(\"Hello World\");"
        );
        assert_eq!(
            TextBuilder::default()
                .text("Hello World")
                .font("LiberationSans-Regular")
                .build()
                .unwrap()
                .to_code(),
            "text(\"Hello World\", font = \"LiberationSans-Regular\");"
        );
        assert_eq!(
            TextBuilder::default()
                .text("Hello World")
                .size(3.0)
                .build()
                .unwrap()
                .to_code(),
            "text(\"Hello World\", size = 3);"
        );
    }

    #[test]
    fn test_import2d() {
        assert_eq!(
            Import2DBuilder::default()
                .file("shape.svg")
                .build()
                .unwrap()
                .to_code(),
            "import(\"shape.svg\");"
        );

        assert_eq!(
            Import2DBuilder::default()
                .file("shape.svg")
                .convexity(10 as u64)
                .build()
                .unwrap()
                .to_code(),
            "import(\"shape.svg\", convexity = 10);"
        );
    }
}
