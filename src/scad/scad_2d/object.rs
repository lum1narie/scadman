use derive_builder::Builder;

use crate::{__impl_scad2d, scad::{Point2D, ScadObject, ScadObject2D, Unit}};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SquareSize {
    N(Unit),
    V(Point2D),
}

impl From<Point2D> for SquareSize {
    fn from(value: Point2D) -> Self {
        Self::V(value)
    }
}
impl From<Unit> for SquareSize {
    fn from(value: Unit) -> Self {
        Self::N(value)
    }
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
        let mut args: Vec<String> = Vec::new();
        let size_str: String = match self.size {
            SquareSize::N(n) => format!("size={}", n),
            SquareSize::V(v) => format!("size=[{}, {}]", v.x, v.y),
        };
        args.push(size_str);
        if let Some(c) = self.center {
            args.push(format!("center={}", c));
        }
        format!("square({})", args.join(", "))
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CircleSize {
    Radius(Unit),
    Diameter(Unit),
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
        let mut args: Vec<String> = Vec::new();
        let size_str: String = match self.size {
            CircleSize::Radius(r) => format!("r={}", r),
            CircleSize::Diameter(d) => format!("d={}", d),
        };
        args.push(size_str);
        if let Some(a) = self.fa {
            args.push(format!("$fa={}", a));
        }
        if let Some(n) = self.r#fn {
            args.push(format!("$fn={}", n));
        }
        if let Some(s) = self.fs {
            args.push(format!("$fs={}", s));
        }
        format!("circle({})", args.join(", "))
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

            for i in 0..pas.len() {
                for j in 0..pas[i].len() {
                    if pas[i][j] >= pts.len() {
                        return Some(Err(format!(
                            "path index out of bounds: [{}][{}]:{}",
                            i, j, pas[i][j]
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
        let mut args: Vec<String> = Vec::new();
        args.push(format!(
            "points=[{}]",
            self.points
                .iter()
                .map(|p| format!("[{}, {}]", p.x, p.y))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        if let Some(ps) = &self.paths {
            args.push(format!(
                "paths=[{}]",
                ps.iter()
                    .map(|p| format!(
                        "[{}]",
                        p.iter()
                            .map(|i| i.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        if let Some(c) = &self.convexity {
            args.push(format!("convexity={}", c));
        }
        format!("polygon({})", args.join(", "))
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
        let mut args: Vec<String> = Vec::new();
        args.push(format!("\"{}\"", self.text));
        if let Some(f) = &self.font {
            args.push(format!("font=\"{}\"", f));
        }
        if let Some(s) = &self.size {
            args.push(format!("size={}", s));
        }
        if let Some(h) = &self.halign {
            args.push(format!("halign=\"{}\"", h));
        }
        if let Some(v) = &self.valign {
            args.push(format!("valign=\"{}\"", v));
        }
        if let Some(sp) = &self.spacing {
            args.push(format!("spacing=\"{}\"", sp));
        }
        if let Some(d) = &self.direction {
            args.push(format!("direction=\"{}\"", d));
        }
        if let Some(l) = &self.language {
            args.push(format!("language=\"{}\"", l));
        }
        if let Some(s) = &self.script {
            args.push(format!("script=\"{}\"", s));
        }
        if let Some(f) = &self.r#fn {
            args.push(format!("$fn={}", f));
        }
        format!("text({})", args.join(", "))
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
        let mut args: Vec<String> = Vec::new();
        args.push(format!("\"{}\"", self.file));
        if let Some(c) = self.convexity {
            args.push(format!("convexity={}", c));
        }
        if let Some(id) = self.id {
            args.push(format!("id={}", id));
        }
        if let Some(l) = self.layer {
            args.push(format!("layer={}", l));
        }
        if let Some(a) = self.fa {
            args.push(format!("$fa={}", a));
        }
        if let Some(n) = self.r#fn {
            args.push(format!("$fn={}", n));
        }
        if let Some(s) = self.fs {
            args.push(format!("$fs={}", s));
        }
        format!("import({})", args.join(", "))
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
            "square(size=3);"
        );
        assert_eq!(
            SquareBuilder::default()
                .size(3.0)
                .center(true)
                .build()
                .unwrap()
                .to_code(),
            "square(size=3, center=true);"
        );
        assert_eq!(
            SquareBuilder::default()
                .size(Point2D::new(3., 2.))
                .build()
                .unwrap()
                .to_code(),
            "square(size=[3, 2]);"
        );
        assert_eq!(
            SquareBuilder::default()
                .size(Point2D::new(3., 2.))
                .center(true)
                .build()
                .unwrap()
                .to_code(),
            "square(size=[3, 2], center=true);"
        );
        assert!(SquareBuilder::default().center(true).build().is_err())
    }

    #[test]
    fn test_circle() {
        assert_eq!(
            CircleBuilder::default().r(3.0).build().unwrap().to_code(),
            "circle(r=3);"
        );
        assert_eq!(
            CircleBuilder::default().d(4.0).build().unwrap().to_code(),
            "circle(d=4);"
        );
        assert_eq!(
            CircleBuilder::default()
                .r(3.0)
                .fa(0.5)
                .r#fn(20 as u64)
                .build()
                .unwrap()
                .to_code(),
            "circle(r=3, $fa=0.5, $fn=20);"
        );
        assert_eq!(
            CircleBuilder::default()
                .r(3.0)
                .fs(40.)
                .fa(0.5)
                .build()
                .unwrap()
                .to_code(),
            "circle(r=3, $fa=0.5, $fs=40);"
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
            "polygon(points=[[1, 1], [-1, 2], [0, 0]]);"
        );
        assert_eq!(
            p0.clone()
                .paths(vec![vec![0, 2, 1]])
                .build()
                .unwrap()
                .to_code(),
            "polygon(points=[[1, 1], [-1, 2], [0, 0]], paths=[[0, 2, 1]]);"
        );
        assert_eq!(
            p0.clone().convexity(2 as u64).build().unwrap().to_code(),
            "polygon(points=[[1, 1], [-1, 2], [0, 0]], convexity=2);"
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
            "polygon(points=[[2, 0], [1, 1], [-1, 1], [1, 0], [0.5, 0.5], [-0.5, 0.5]], paths=[[0, 1, 2], [3, 4, 5]]);"
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
            "text(\"Hello World\", font=\"LiberationSans-Regular\");"
        );
        assert_eq!(
            TextBuilder::default()
                .text("Hello World")
                .size(3.0)
                .build()
                .unwrap()
                .to_code(),
            "text(\"Hello World\", size=3);"
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
            "import(\"shape.svg\", convexity=10);"
        );
    }
}
