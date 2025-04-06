use ambassador::Delegate;
use derive_builder::Builder;
use derive_more::derive::From;

use crate::{
    __generate_scad_options, __impl_scad3d,
    common::{Point3D, Unit},
    internal::generate_sentence_repr,
    scad_display::{ambassador_impl_ScadDisplay, Identifier, ScadDisplay},
    value_type::RoundSize,
};

/// Sphere object `sphere()` in SCAD.obj
#[derive(Builder, Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    /// Size of sphere.
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

__impl_scad3d!(Sphere);

impl SphereBuilder {
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
    /// # Arguments
    ///
    /// + `value` - `d` option in SCAD. This is the diameter of circle.
    pub fn d(&mut self, value: Unit) -> &mut Self {
        let new = self;
        new.size = Some(RoundSize::Diameter(value));
        new
    }
}

impl ScadDisplay for Sphere {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "sphere",
            __generate_scad_options!(
                (self.size.name(), self.size);
                ("$fa", self.fa), ("$fn", self.r#fn), ("$fs", self.fs);
            ),
        )
    }
}

/// Size of cube in SCAD.
#[derive(Copy, Clone, Debug, PartialEq, Delegate)]
#[delegate(ScadDisplay)]
pub enum CubeSize {
    /// Edges' length of square.
    /// `n` option in SCAD.
    N(Unit),
    /// `[x, y, z]` length of rectangle.
    /// `v` option in SCAD.
    V(Point3D),
}

impl From<Unit> for CubeSize {
    fn from(value: Unit) -> Self {
        Self::N(value)
    }
}

impl From<Point3D> for CubeSize {
    fn from(value: Point3D) -> Self {
        Self::V(value)
    }
}

impl From<[Unit; 3]> for CubeSize {
    fn from(value: [Unit; 3]) -> Self {
        let [x, y, z] = value;
        Self::V(Point3D::new(x, y, z))
    }
}

/// Cube object `cube()` in SCAD.
#[derive(Builder, Copy, Clone, Debug, PartialEq)]
pub struct Cube {
    /// Size of cube
    /// `n` or `v` option in SCAD.
    ///
    /// See also [`CubeSize`].
    #[builder(setter(into))]
    pub size: CubeSize,
    /// `center` option in SCAD.
    ///
    /// + `true` - square's origin is at center of square.
    /// + `false` - square's origin is at the point where
    ///     x, y, and z coordinate is the smallest.
    #[builder(setter(into, strip_option), default)]
    pub center: Option<bool>,
}

__impl_scad3d!(Cube);

impl ScadDisplay for Cube {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "cube",
            __generate_scad_options!(
                ("size", self.size);
                ("center", self.center);
            ),
        )
    }
}

/// Size of cylinder in SCAD.
///
/// See also [`RoundSize`].
#[derive(Copy, Clone, Debug, PartialEq, From)]
pub enum CylinderSize {
    /// Single size of cylinder.
    /// `r` or `d` option in SCAD.
    Single(RoundSize),
    /// Pair of size of cylinder.
    /// `r1|d1, r2|d2` option in SCAD.
    Double((RoundSize, RoundSize)),
}

/// Numbers to generate [`CylinderSize`].
///
/// The numbers are the length.
/// This type have no information about the size is radius or diameter.
#[derive(Copy, Clone, Debug, PartialEq, From)]
pub enum CylinderSizeEntry {
    /// Number to generate [`CylinderSize::Single`].
    Single(Unit),
    /// Pair of numbers to generate [`CylinderSize::Double`].
    Double([Unit; 2]),
}

/// Cylinder object `cylinder()` in SCAD.
#[derive(Builder, Copy, Clone, Debug, PartialEq)]
pub struct Cylinder {
    /// Height of cylinder.
    /// `h` option in SCAD.
    #[builder(setter(into))]
    pub h: Unit,
    /// Size of cylinder.
    /// `r` or `d` or `r1|d1, r2|d2` option in SCAD.
    ///
    /// See also [`CylinderSize`].
    #[builder(setter(custom))]
    pub size: CylinderSize,
    /// `center` option in SCAD.
    ///
    /// + `true` - sphere's z origin is at center of cylinder.
    /// + `false` - square's z origin is at the point where
    ///     z coordinate is the smallest.
    #[builder(setter(into, strip_option), default)]
    pub center: Option<bool>,
    /// `$fa` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub fa: Option<Unit>,
    /// `$fn` option in SCAD.$
    #[builder(setter(into, strip_option), default)]
    pub r#fn: Option<u64>,
    /// `$fs` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub fs: Option<Unit>,
}

impl CylinderBuilder {
    /// Set `r` or `r1, r2` option in SCAD.
    ///
    /// # Arguments
    ///
    /// + `value` - `r` or `r1, r2` option in SCAD. This is the radius of cylinder.
    pub fn r<T: Into<CylinderSizeEntry>>(&mut self, value: T) -> &mut Self {
        let new = self;
        new.size = match value.into() {
            CylinderSizeEntry::Single(r) => Some(CylinderSize::Single(RoundSize::Radius(r))),
            CylinderSizeEntry::Double([r1, r2]) => Some(CylinderSize::Double((
                RoundSize::Radius(r1),
                RoundSize::Radius(r2),
            ))),
        };
        new
    }
    /// Set `d` or `d1, d2` option in SCAD.
    ///
    /// # Arguments
    ///
    /// + `value` - `d` or `d1, d2` option in SCAD. This is the diameter of cylinder.
    pub fn d<T: Into<CylinderSizeEntry>>(&mut self, value: T) -> &mut Self {
        let new = self;
        new.size = match value.into() {
            CylinderSizeEntry::Single(d) => Some(CylinderSize::Single(RoundSize::Diameter(d))),
            CylinderSizeEntry::Double([d1, d2]) => Some(CylinderSize::Double((
                RoundSize::Diameter(d1),
                RoundSize::Diameter(d2),
            ))),
        };
        new
    }
}

__impl_scad3d!(Cylinder);

impl ScadDisplay for Cylinder {
    fn repr_scad(&self) -> String {
        let size_str = match self.size {
            CylinderSize::Single(size) => format!("{} = {}", size.name(), size.repr_scad()),
            CylinderSize::Double((size1, size2)) => format!(
                "{}1 = {}, {}2 = {}",
                size1.name(),
                size1.repr_scad(),
                size2.name(),
                size2.repr_scad()
            ),
        };
        let opts = __generate_scad_options!(
            ("h", self.h),
            ("", Identifier(size_str));
            ("center", self.center),
            ("$fa", self.fa), ("$fn", self.r#fn), ("$fs", self.fs);
        );
        format!(
            "cylinder({})",
            opts.iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

/// Numbers to generate [`vec<Points3D>`].
#[derive(Clone, Debug, PartialEq, derive_more::Deref)]
pub struct VecPoint3DEntry(pub Vec<Point3D>);

impl From<Vec<[Unit; 3]>> for VecPoint3DEntry {
    fn from(value: Vec<[Unit; 3]>) -> Self {
        Self(
            value
                .into_iter()
                .map(|[x, y, z]| Point3D::new(x, y, z))
                .collect(),
        )
    }
}

impl From<Vec<Point3D>> for VecPoint3DEntry {
    fn from(value: Vec<Point3D>) -> Self {
        Self(value)
    }
}

impl From<VecPoint3DEntry> for Vec<Point3D> {
    fn from(value: VecPoint3DEntry) -> Self {
        value.0
    }
}

/// Polyhedron object `polyhedron()` in SCAD.
#[derive(Builder, Clone, Debug, PartialEq)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct Polyhedron {
    /// Verticies of polyhedron.
    /// `points` option in SCAD.
    #[builder(setter(custom))]
    pub points: Vec<Point3D>,
    /// Faces of polyhedron.
    /// `faces` option in SCAD.
    ///
    /// Each element is a face.
    /// Each two consecutive elements of a face are the faces. (include (last, first))
    /// Each element of a path shows the index of a point.
    #[builder(setter(into, strip_option), default)]
    pub faces: Option<Vec<Vec<usize>>>,
    /// `convexity` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub convexity: Option<u64>,
}

__impl_scad3d!(Polyhedron);

impl PolyhedronBuilder {
    /// Check if `faces` is in the range of `points`'s indicies.
    fn validate(&self) -> Result<(), String> {
        (|| -> Option<Result<(), String>> {
            let pts: Vec<Point3D> = self.points.clone()?;
            let pas: Vec<Vec<usize>> = self.faces.clone()??;

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
    pub fn points<T: Into<VecPoint3DEntry>>(&mut self, value: T) -> &mut Self {
        let new = self;
        let entry: VecPoint3DEntry = value.into();
        new.points = Some(entry.into());
        new
    }
}

impl ScadDisplay for Polyhedron {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "polyhedron",
            __generate_scad_options!(
                ("points", self.points.clone());
                ("faces", self.faces.clone()), ("convexity", self.convexity);
            ),
        )
    }
}

/// SCAD object imported from external file.
/// `import()` in SCAD.
/// This Rust type is regarded as 3D object.
#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Import3D {
    /// Path of the external file.
    #[builder(setter(into))]
    pub file: String,
    /// `convexity` option in SCAD.:bp
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

__impl_scad3d!(Import3D);

impl ScadDisplay for Import3D {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "import",
            __generate_scad_options!(
                ("", self.file.clone());
                ("convexity", self.convexity),
                ("$fa", self.fa), ("$fn", self.r#fn), ("$fs", self.fs);
            ),
        )
    }
}

/// SCAD object from heightmap information from text or image files
/// `surface()` in SCAD.
#[derive(Builder, Clone, Debug, PartialEq, Eq)]
pub struct Surface {
    /// Path of the external file.
    #[builder(setter(into))]
    pub file: String,
    /// `center` option in SCAD.
    ///
    /// + `true` - Object's xy origin is at center of it.
    /// + `false` - Object's xy origin is at the point where
    ///     x and y coordinate is the smallest.
    #[builder(setter(into, strip_option), default)]
    pub center: Option<bool>,
    /// Inverts how the color values of imported images are translated into height values.
    /// `invert` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub invert: Option<bool>,
    /// `convexity` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub convexity: Option<u64>,
}

__impl_scad3d!(Surface);

impl ScadDisplay for Surface {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "surface",
            __generate_scad_options!(
                ("file", self.file.clone());
                ("center", self.center),
                ("invert", self.invert),
                ("convexity", self.convexity);
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::ScadSentence as _;

    use super::*;

    #[test]
    fn test_sphere() {
        assert_eq!(
            Sphere::build_with(|sb| {
                let _ = sb.r(3.0);
            })
            .repr_scad(),
            "sphere(r = 3)"
        );
        assert_eq!(
            Sphere::build_with(|sb| {
                let _ = sb.d(4.0);
            })
            .repr_scad(),
            "sphere(d = 4)"
        );
        assert_eq!(
            Sphere::build_with(|sb| {
                let _ = sb.r(3.0).fa(0.5).r#fn(20_u64);
            })
            .repr_scad(),
            "sphere(r = 3, $fa = 0.5, $fn = 20)"
        );
        assert_eq!(
            Sphere::build_with(|sb| {
                let _ = sb.r(3.0).fs(40.).fa(0.5);
            })
            .repr_scad(),
            "sphere(r = 3, $fa = 0.5, $fs = 40)"
        );
        let _x = SphereBuilder::default()
            .fa(0.5)
            .r#fn(20_u64)
            .fs(40.)
            .build()
            .unwrap_err();
    }

    #[test]
    fn test_cube() {
        assert_eq!(
            Cube::build_with(|cb| {
                let _ = cb.size(3.0);
            })
            .repr_scad(),
            "cube(size = 3)"
        );
        assert_eq!(
            Cube::build_with(|cb| {
                let _ = cb.size([4.0, 2.0, 3.0]);
            })
            .repr_scad(),
            "cube(size = [4, 2, 3])"
        );
        assert_eq!(
            Cube::build_with(|cb| {
                let _ = cb.size(Point3D::new(4.0, 2.0, 3.0));
            })
            .repr_scad(),
            "cube(size = [4, 2, 3])"
        );
        assert_eq!(
            Cube::build_with(|cb| {
                let _ = cb.size(3.0).center(true);
            })
            .repr_scad(),
            "cube(size = 3, center = true)"
        );
    }

    #[test]
    fn test_cylinder() {
        assert_eq!(
            Cylinder::build_with(|cb| {
                let _ = cb.h(5.0).r(3.0);
            })
            .repr_scad(),
            "cylinder(h = 5, r = 3)"
        );
        assert_eq!(
            Cylinder::build_with(|cb| {
                let _ = cb.h(5.0).d(3.0);
            })
            .repr_scad(),
            "cylinder(h = 5, d = 3)"
        );
        assert_eq!(
            Cylinder::build_with(|cb| {
                let _ = cb.h(5.0).r([1.0, 2.0]);
            })
            .repr_scad(),
            "cylinder(h = 5, r1 = 1, r2 = 2)"
        );
        assert_eq!(
            Cylinder::build_with(|cb| {
                let _ = cb.h(5.0).d([1.0, 2.0]);
            })
            .repr_scad(),
            "cylinder(h = 5, d1 = 1, d2 = 2)"
        );
        assert_eq!(
            Cylinder::build_with(|cb| {
                let _ = cb.h(5.0).r(3.0).fa(2.0);
            })
            .repr_scad(),
            "cylinder(h = 5, r = 3, $fa = 2)"
        );
    }

    #[test]
    fn test_polyhedron() {
        let mut p0 = PolyhedronBuilder::default();
        _ = p0.points(vec![
            Point3D::new(1., 1., 1.),
            Point3D::new(-1., 2., -1.),
            Point3D::new(0., 0., 0.),
        ]);
        assert_eq!(
            p0.build().unwrap().repr_scad(),
            "polyhedron(points = [[1, 1, 1], [-1, 2, -1], [0, 0, 0]])"
        );
        assert_eq!(
            p0.clone()
                .faces(vec![vec![0, 2, 1]])
                .build()
                .unwrap()
                .repr_scad(),
            "polyhedron(points = [[1, 1, 1], [-1, 2, -1], [0, 0, 0]], faces = [[0, 2, 1]])"
        );
        assert_eq!(
            p0.convexity(2_u64).build().unwrap().repr_scad(),
            "polyhedron(points = [[1, 1, 1], [-1, 2, -1], [0, 0, 0]], convexity = 2)"
        );

        let mut p1 = PolyhedronBuilder::default();
        _ = p1.points(vec![
            [2., 0., 2.],
            [1., 1., 1.],
            [-1., 1., 0.],
            [1., 0., -1.],
            [0.5, 0.5, 0.7],
            [-0.5, 0.5, -0.3],
        ]);
        assert_eq!(
            p1.faces([vec![0, 1, 2], vec![3, 4, 5]]).build().unwrap().repr_scad(),
            "polyhedron(points = [[2, 0, 2], [1, 1, 1], [-1, 1, 0], [1, 0, -1], [0.5, 0.5, 0.7], [-0.5, 0.5, -0.3]], faces = [[0, 1, 2], [3, 4, 5]])"
        );
        assert_eq!(
            p1.clone()
                .faces([vec![0, 1, 2], vec![6, 4, 5]])
                .build()
                .err()
                .map(|e| e.to_string())
                .unwrap_or_default(),
            "path index out of bounds: [1][0]:6"
        );
    }

    #[test]
    fn test_import3d() {
        assert_eq!(
            Import3D::build_with(|ib| {
                let _ = ib.file("shape.stl");
            })
            .repr_scad(),
            "import(\"shape.stl\")"
        );

        assert_eq!(
            Import3D::build_with(|ib| {
                let _ = ib.file("shape.stl").convexity(10_u64);
            })
            .repr_scad(),
            "import(\"shape.stl\", convexity = 10)"
        );
    }

    #[test]
    fn test_surface() {
        assert_eq!(
            Surface::build_with(|sb| {
                let _ = sb.file("shape.dat");
            })
            .repr_scad(),
            "surface(file = \"shape.dat\")"
        );

        assert_eq!(
            Surface::build_with(|sb| {
                let _ = sb
                    .file("shape.dat")
                    .convexity(10_u64)
                    .center(true)
                    .invert(true);
            })
            .repr_scad(),
            "surface(file = \"shape.dat\", center = true, invert = true, convexity = 10)"
        );
    }
}
