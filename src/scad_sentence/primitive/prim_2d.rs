use ambassador::Delegate;
use derive_builder::Builder;

use crate::{
    __generate_scad_options, __impl_builder_sentence,
    common::{Point2D, Unit},
    internal::generate_sentence_repr,
    scad_display::{ambassador_impl_ScadDisplay, ScadDisplay},
    value_type::RoundSize,
};

/// Size of square in SCAD.
#[derive(Copy, Clone, Debug, PartialEq, Delegate)]
#[delegate(ScadDisplay)]
pub enum SquareSize {
    /// Edges' length of square.
    /// `n` option in SCAD.
    N(Unit),
    /// `[x, y]` length of rectangle.
    /// `v` option in SCAD.
    V(Point2D),
}

impl From<Unit> for SquareSize {
    fn from(value: Unit) -> Self {
        Self::N(value)
    }
}

impl From<Point2D> for SquareSize {
    fn from(value: Point2D) -> Self {
        Self::V(value)
    }
}

impl From<[Unit; 2]> for SquareSize {
    fn from(value: [Unit; 2]) -> Self {
        let [x, y] = value;
        Self::V(Point2D::new(x, y))
    }
}

/// Square object `square()` in SCAD.
#[derive(Builder, Copy, Clone, Debug, PartialEq)]
pub struct Square {
    /// Size of square.
    /// `n` or `v` option in SCAD.
    ///
    /// See also [`SquareSize`].
    #[builder(setter(into))]
    pub size: SquareSize,
    /// `center` option in SCAD.
    ///
    /// + `true` - square's origin is at center of square.
    /// + `false` - square's origin is at the point where
    ///     x and y coordinate is the smallest.
    #[builder(setter(into, strip_option), default)]
    pub center: Option<bool>,
}

__impl_builder_sentence!(Square);

impl ScadDisplay for Square {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "square",
            __generate_scad_options!(
                ("size", self.size);
                ("center", self.center);
            ),
        )
    }
}

/// Circle object `circle()` in SCAD.
#[derive(Builder, Copy, Clone, Debug, PartialEq)]
pub struct Circle {
    /// Size of circle.
    /// `r` or `d` option in SCAD.
    ///
    /// See also [`RoundSize`].
    #[builder(setter(custom))]
    pub size: RoundSize,
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

__impl_builder_sentence!(Circle);

impl CircleBuilder {
    /// Set `r` option in SCAD.
    ///
    /// # Arguments
    ///
    /// + `value` - `r` option in SCAD. This is the radius of circle.
    pub fn r(&mut self, value: Unit) -> &mut Self {
        let new = self;
        new.size = Some(RoundSize::Radius(value));
        new
    }
    /// Set `d` option in SCAD.
    ///
    ///
    /// # Arguments
    ///
    /// + `value` - `d` option in SCAD. This is the diameter of circle.
    pub fn d(&mut self, value: Unit) -> &mut Self {
        let new = self;
        new.size = Some(RoundSize::Diameter(value));
        new
    }
}

impl ScadDisplay for Circle {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "circle",
            __generate_scad_options!(
                (self.size.name(), self.size);
                ("$fa", self.fa), ("$fn", self.r#fn), ("$fs", self.fs);
            ),
        )
    }
}

/// Numbers to generate [`vec<Points2D>`].
#[derive(Clone, Debug, PartialEq, derive_more::Deref)]
pub struct VecPoint2DEntry(pub Vec<Point2D>);

impl From<Vec<[Unit; 2]>> for VecPoint2DEntry {
    fn from(value: Vec<[Unit; 2]>) -> Self {
        Self(value.into_iter().map(|[x, y]| Point2D::new(x, y)).collect())
    }
}

impl From<Vec<Point2D>> for VecPoint2DEntry {
    fn from(value: Vec<Point2D>) -> Self {
        Self(value)
    }
}

impl From<VecPoint2DEntry> for Vec<Point2D> {
    fn from(value: VecPoint2DEntry) -> Self {
        value.0
    }
}

/// Polygon object `polygon()` in SCAD.
#[derive(Builder, Clone, Debug, PartialEq)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct Polygon {
    /// Verticies of polygon.
    /// `points` option in SCAD.
    #[builder(setter(custom))]
    pub points: Vec<Point2D>,
    /// Edges of polygon.
    /// `paths` option in SCAD.
    ///
    /// Each element is a path. Each element of a path shows the index of a point.
    #[builder(setter(into, strip_option), default)]
    pub paths: Option<Vec<Vec<usize>>>,
    /// `convexity` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub convexity: Option<u64>,
}

__impl_builder_sentence!(Polygon);

impl PolygonBuilder {
    /// Check if `paths` is in the range of `points`'s indicies.
    fn validate(&self) -> Result<(), String> {
        (|| -> Option<Result<(), String>> {
            let pts: Vec<Point2D> = self.points.clone()?;
            let pas: Vec<Vec<usize>> = self.paths.clone()??;

            for (i, pa) in pas.into_iter().enumerate() {
                for (j, vtx) in pa.into_iter().enumerate() {
                    if vtx >= pts.len() {
                        return Some(Err(format!("path index out of bounds: [{i}][{j}]:{vtx}")));
                    }
                }
            }

            Some(Ok(()))
        })()
        .unwrap_or(Ok(()))
    }

    /// Set `points` option in SCAD.
    ///
    /// # Arguments
    ///
    /// + `value` - `points` option in SCAD. This is the verticies of polygon.
    pub fn points<T: Into<VecPoint2DEntry>>(&mut self, value: T) -> &mut Self {
        let new = self;
        let entry: VecPoint2DEntry = value.into();
        new.points = Some(entry.into());
        new
    }
}

impl ScadDisplay for Polygon {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "polygon",
            __generate_scad_options!(
                ("points", self.points.clone());
                ("paths", self.paths.clone()), ("convexity", self.convexity);
            ),
        )
    }
}

/// Text object `text()` in SCAD.
#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Text {
    /// Text to show.
    #[builder(setter(into))]
    pub text: String,
    /// Font size of text.
    /// `size` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub size: Option<Unit>,
    /// Font of text.
    /// `font` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub font: Option<String>,
    /// Horizontal alignment of text.
    /// Possible values are `"left"`, `"center"`, and `"right"`.
    /// `halign` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub halign: Option<String>,
    /// Vertical alignment of text.
    /// Possible values are `"top"`, `"center"`, `"baseline"`, and `"bottom"`.
    /// `valign` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub valign: Option<String>,
    /// Spacing of text.
    /// `spacing` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub spacing: Option<String>,
    /// Direction of text.
    /// Possible values are `"ltr"`, `"rtl"`, `"ttb"`, and `"btt"`.
    /// `direction` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub direction: Option<String>,
    /// Language of text. (e.g., `"en"`, `"ar"`, `"ch"`).
    /// `language` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub language: Option<String>,
    /// Script of text. (e.g., `"latin"`, `"arabic"`, `"hani"`)
    /// `script` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub script: Option<String>,
    /// `$fn` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub r#fn: Option<u64>,
}

__impl_builder_sentence!(Text);

impl ScadDisplay for Text {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
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

/// SCAD object imported from external file.
/// `import()` in SCAD.
/// This Rust type is regarded as 2D object.
#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Import2D {
    /// Path of the external file.
    #[builder(setter(into))]
    pub file: String,
    /// `convexity` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub convexity: Option<u64>,
    /// Id of an element or group to import.
    /// For SVG import only,
    /// `id` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub id: Option<u64>,
    /// Specify a specific layer to import.
    /// For DXF and SVG import only.
    /// `layer` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub layer: Option<u64>,
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

__impl_builder_sentence!(Import2D);

impl ScadDisplay for Import2D {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
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
    use crate::ScadBuildable as _;

    use super::*;

    #[test]
    fn test_square() {
        assert_eq!(
            Square::build_with(|b| {
                let _ = b.size(3.);
            })
            .repr_scad(),
            "square(size = 3)"
        );
        assert_eq!(
            Square::build_with(|b| {
                let _ = b.size(3.).center(true);
            })
            .repr_scad(),
            "square(size = 3, center = true)"
        );
        assert_eq!(
            Square::build_with(|b| {
                let _ = b.size([3., 2.]);
            })
            .repr_scad(),
            "square(size = [3, 2])"
        );
        assert_eq!(
            Square::build_with(|b| {
                let _ = b.size(Point2D::new(3., 2.));
            })
            .repr_scad(),
            "square(size = [3, 2])"
        );
        assert_eq!(
            Square::build_with(|b| {
                let _ = b.size([3., 2.]).center(true);
            })
            .repr_scad(),
            "square(size = [3, 2], center = true)"
        );
        drop(SquareBuilder::default().center(true).build().unwrap_err());
    }

    #[test]
    fn test_circle() {
        assert_eq!(
            Circle::build_with(|b| {
                let _ = b.r(3.);
            })
            .repr_scad(),
            "circle(r = 3)"
        );
        assert_eq!(
            Circle::build_with(|b| {
                let _ = b.d(4.);
            })
            .repr_scad(),
            "circle(d = 4)"
        );
        assert_eq!(
            Circle::build_with(|b| {
                let _ = b.r(3.).fa(0.5).r#fn(20_u64);
            })
            .repr_scad(),
            "circle(r = 3, $fa = 0.5, $fn = 20)"
        );
        assert_eq!(
            Circle::build_with(|b| {
                let _ = b.r(3.).fs(40).fa(0.5);
            })
            .repr_scad(),
            "circle(r = 3, $fa = 0.5, $fs = 40)"
        );
        drop(
            CircleBuilder::default()
                .fa(0.5)
                .r#fn(20_u64)
                .fs(40.)
                .build()
                .unwrap_err(),
        );
    }

    #[test]
    fn test_polygon() {
        let mut p0 = PolygonBuilder::default();
        _ = p0.points(vec![
            Point2D::new(1., 1.),
            Point2D::new(-1., 2.),
            Point2D::new(0., 0.),
        ]);
        assert_eq!(
            p0.build().unwrap().repr_scad(),
            "polygon(points = [[1, 1], [-1, 2], [0, 0]])"
        );
        assert_eq!(
            p0.clone()
                .paths(vec![vec![0, 2, 1]])
                .build()
                .unwrap()
                .repr_scad(),
            "polygon(points = [[1, 1], [-1, 2], [0, 0]], paths = [[0, 2, 1]])"
        );
        assert_eq!(
            p0.convexity(2_u64).build().unwrap().repr_scad(),
            "polygon(points = [[1, 1], [-1, 2], [0, 0]], convexity = 2)"
        );

        let mut p1 = PolygonBuilder::default();
        _ = p1.points(vec![
            [2., 0.],
            [1., 1.],
            [-1., 1.],
            [1., 0.],
            [0.5, 0.5],
            [-0.5, 0.5],
        ]);
        assert_eq!(
            p1.clone().paths([vec![0, 1, 2], vec![3, 4, 5]]).build().unwrap().repr_scad(),
            "polygon(points = [[2, 0], [1, 1], [-1, 1], [1, 0], [0.5, 0.5], [-0.5, 0.5]], paths = [[0, 1, 2], [3, 4, 5]])"
        );
        assert_eq!(
            p1.paths([vec![0, 1, 2], vec![6, 4, 5]])
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
            Text::build_with(|tb| {
                let _ = tb.text("Hello World");
            })
            .repr_scad(),
            "text(\"Hello World\")"
        );
        assert_eq!(
            Text::build_with(|tb| {
                let _ = tb.text("Hello World").font("LiberationSans-Regular");
            })
            .repr_scad(),
            "text(\"Hello World\", font = \"LiberationSans-Regular\")"
        );
        assert_eq!(
            Text::build_with(|tb| {
                let _ = tb.text("Hello World").size(3.);
            })
            .repr_scad(),
            "text(\"Hello World\", size = 3)"
        );
    }

    #[test]
    fn test_import2d() {
        assert_eq!(
            Import2D::build_with(|ib| {
                let _ = ib.file("shape.svg");
            })
            .repr_scad(),
            "import(\"shape.svg\")"
        );

        assert_eq!(
            Import2D::build_with(|ib| {
                let _ = ib.file("shape.svg").convexity(10_u64);
            })
            .repr_scad(),
            "import(\"shape.svg\", convexity = 10)"
        );
    }
}
