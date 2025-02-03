use ambassador::Delegate;
use derive_builder::Builder;
use derive_more::derive::From;

use crate::{
    __generate_scad_options, __impl_scad3d,
    scad::{
        ambassador_impl_ScadDisplay, generate_body, Identifier, Point3D, RoundSize, ScadDisplay,
        ScadObject, ScadObject3D, Unit,
    },
};

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Sphere {
    #[builder(setter(custom))]
    pub size: RoundSize,
    #[builder(setter(into, strip_option), default)]
    pub fa: Option<Unit>,
    #[builder(setter(into, strip_option), default)]
    pub r#fn: Option<u64>,
    #[builder(setter(into, strip_option), default)]
    pub fs: Option<Unit>,
}

__impl_scad3d!(Sphere);

impl SphereBuilder {
    pub fn r(&mut self, value: Unit) -> &mut Self {
        let new = self;
        new.size = Some(RoundSize::Radius(value));
        new
    }
    pub fn d(&mut self, value: Unit) -> &mut Self {
        let new = self;
        new.size = Some(RoundSize::Diameter(value));
        new
    }
}

impl ScadObject for Sphere {
    fn get_body(&self) -> String {
        generate_body(
            "sphere",
            __generate_scad_options!(
                (self.size.name(), self.size);
                ("$fa", self.fa), ("$fn", self.r#fn), ("$fs", self.fs);
            ),
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq, From, Delegate)]
#[delegate(ScadDisplay)]
pub enum CubeSize {
    N(Unit),
    V(Point3D),
}

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Cube {
    #[builder(setter(into))]
    pub size: CubeSize,
    #[builder(setter(into, strip_option), default)]
    pub center: Option<bool>,
}

__impl_scad3d!(Cube);

impl ScadObject for Cube {
    fn get_body(&self) -> String {
        generate_body(
            "cube",
            __generate_scad_options!(
                ("", self.size);
                ("center", self.center);
            ),
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq, From)]
pub enum CylinderSize {
    Single(RoundSize),
    Double((RoundSize, RoundSize)),
}

#[derive(Copy, Clone, Debug, PartialEq, From)]
pub enum CylinderSizeEntry {
    Single(Unit),
    Double([Unit; 2]),
}

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Cylinder {
    #[builder(setter(into))]
    pub h: Unit,
    #[builder(setter(custom))]
    pub size: CylinderSize,
    #[builder(setter(into, strip_option), default)]
    pub center: Option<bool>,
    #[builder(setter(into, strip_option), default)]
    pub fa: Option<Unit>,
    #[builder(setter(into, strip_option), default)]
    pub r#fn: Option<u64>,
    #[builder(setter(into, strip_option), default)]
    pub fs: Option<Unit>,
}

impl CylinderBuilder {
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

impl ScadObject for Cylinder {
    fn get_body(&self) -> String {
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
                .map(|opt| opt.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[derive(Builder, Clone, Debug, PartialEq)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct Polyhedron {
    #[builder(setter(into))]
    pub points: Vec<Point3D>,
    #[builder(setter(into, strip_option), default)]
    pub paths: Option<Vec<Vec<usize>>>,
    #[builder(setter(into, strip_option), default)]
    pub convexity: Option<u64>,
}

__impl_scad3d!(Polyhedron);

impl PolyhedronBuilder {
    fn validate(&self) -> Result<(), String> {
        (|| -> Option<Result<(), String>> {
            let pts: Vec<Point3D> = self.points.clone()?;
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

impl ScadObject for Polyhedron {
    fn get_body(&self) -> String {
        generate_body(
            "polyhedron",
            __generate_scad_options!(
                ("points", self.points.clone());
                ("paths", self.paths.clone()), ("convexity", self.convexity);
            ),
        )
    }
}

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Import3D {
    #[builder(setter(into))]
    pub file: String,
    #[builder(setter(into, strip_option), default)]
    pub convexity: Option<u64>,
    #[builder(setter(into, strip_option), default)]
    pub fa: Option<Unit>,
    #[builder(setter(into, strip_option), default)]
    pub r#fn: Option<u64>,
    #[builder(setter(into, strip_option), default)]
    pub fs: Option<Unit>,
}

__impl_scad3d!(Import3D);

impl ScadObject for Import3D {
    fn get_body(&self) -> String {
        generate_body(
            "import",
            __generate_scad_options!(
                ("", self.file.clone());
                ("convexity", self.convexity),
                ("$fa", self.fa), ("$fn", self.r#fn), ("$fs", self.fs);
            ),
        )
    }
}

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Surface {
    #[builder(setter(into))]
    pub file: String,
    #[builder(setter(into, strip_option), default)]
    pub center: Option<bool>,
    #[builder(setter(into, strip_option), default)]
    pub invert: Option<bool>,
    #[builder(setter(into, strip_option), default)]
    pub convexity: Option<u64>,
}

__impl_scad3d!(Surface);

impl ScadObject for Surface {
    fn get_body(&self) -> String {
        generate_body(
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
    use super::*;

    #[test]
    fn test_sphere() {
        assert_eq!(
            SphereBuilder::default().r(3.0).build().unwrap().to_code(),
            "sphere(r = 3);"
        );
        assert_eq!(
            SphereBuilder::default().d(4.0).build().unwrap().to_code(),
            "sphere(d = 4);"
        );
        assert_eq!(
            SphereBuilder::default()
                .r(3.0)
                .fa(0.5)
                .r#fn(20 as u64)
                .build()
                .unwrap()
                .to_code(),
            "sphere(r = 3, $fa = 0.5, $fn = 20);"
        );
        assert_eq!(
            SphereBuilder::default()
                .r(3.0)
                .fs(40.)
                .fa(0.5)
                .build()
                .unwrap()
                .to_code(),
            "sphere(r = 3, $fa = 0.5, $fs = 40);"
        );
        assert!(SphereBuilder::default()
            .fa(0.5)
            .r#fn(20 as u64)
            .fs(40.)
            .build()
            .is_err());
    }

    #[test]
    fn test_cube() {
        assert_eq!(
            CubeBuilder::default().size(3.0).build().unwrap().to_code(),
            "cube(3);"
        );
        assert_eq!(
            CubeBuilder::default()
                .size(Point3D::new(4.0, 2.0, 3.0))
                .build()
                .unwrap()
                .to_code(),
            "cube([4, 2, 3]);"
        );
        assert_eq!(
            CubeBuilder::default()
                .size(3.0)
                .center(true)
                .build()
                .unwrap()
                .to_code(),
            "cube(3, center = true);"
        );
    }

    #[test]
    fn test_cylinder() {
        assert_eq!(
            CylinderBuilder::default().h(5.0).r(3.0).build().unwrap().to_code(),
            "cylinder(h = 5, r = 3);"
        );
        assert_eq!(
            CylinderBuilder::default().h(5.0).d(3.0).build().unwrap().to_code(),
            "cylinder(h = 5, d = 3);"
        );
        assert_eq!(
            CylinderBuilder::default().h(5.0).r([1.0, 2.0]).build().unwrap().to_code(),
            "cylinder(h = 5, r1 = 1, r2 = 2);"
        );
        assert_eq!(
            CylinderBuilder::default().h(5.0).d([1.0, 2.0]).build().unwrap().to_code(),
            "cylinder(h = 5, d1 = 1, d2 = 2);"
        );
        assert_eq!(
            CylinderBuilder::default().h(5.0).r(3.0).fa(2.0).build().unwrap().to_code(),
            "cylinder(h = 5, r = 3, $fa = 2);"
        );
    }

    #[test]
    fn test_polyhedron() {
        let p0 = {
            let mut p = PolyhedronBuilder::default();
            p.points(vec![
                Point3D::new(1., 1., 1.),
                Point3D::new(-1., 2., -1.),
                Point3D::new(0., 0., 0.),
            ]);
            p
        };
        assert_eq!(
            p0.clone().build().unwrap().to_code(),
            "polyhedron(points = [[1, 1, 1], [-1, 2, -1], [0, 0, 0]]);"
        );
        assert_eq!(
            p0.clone()
                .paths(vec![vec![0, 2, 1]])
                .build()
                .unwrap()
                .to_code(),
            "polyhedron(points = [[1, 1, 1], [-1, 2, -1], [0, 0, 0]], paths = [[0, 2, 1]]);"
        );
        assert_eq!(
            p0.clone().convexity(2 as u64).build().unwrap().to_code(),
            "polyhedron(points = [[1, 1, 1], [-1, 2, -1], [0, 0, 0]], convexity = 2);"
        );

        let p1 = {
            let mut p = PolyhedronBuilder::default();
            p.points(vec![
                Point3D::new(2., 0., 2.),
                Point3D::new(1., 1., 1.),
                Point3D::new(-1., 1., 0.),
                Point3D::new(1., 0., -1.),
                Point3D::new(0.5, 0.5, 0.7),
                Point3D::new(-0.5, 0.5, -0.3),
            ]);
            p
        };
        assert_eq!(
            p1.clone().paths(vec![vec![0, 1, 2], vec![3, 4, 5]]).build().unwrap().to_code(),
            "polyhedron(points = [[2, 0, 2], [1, 1, 1], [-1, 1, 0], [1, 0, -1], [0.5, 0.5, 0.7], [-0.5, 0.5, -0.3]], paths = [[0, 1, 2], [3, 4, 5]]);"
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
    fn test_import3d() {
        assert_eq!(
            Import3DBuilder::default()
                .file("shape.stl")
                .build()
                .unwrap()
                .to_code(),
            "import(\"shape.stl\");"
        );

        assert_eq!(
            Import3DBuilder::default()
                .file("shape.stl")
                .convexity(10 as u64)
                .build()
                .unwrap()
                .to_code(),
            "import(\"shape.stl\", convexity = 10);"
        );
    }

    #[test]
    fn test_surface() {
        assert_eq!(
            SurfaceBuilder::default()
                .file("shape.dat")
                .build()
                .unwrap()
                .to_code(),
            "surface(file = \"shape.dat\");"
        );

        assert_eq!(
            SurfaceBuilder::default()
                .file("shape.dat")
                .convexity(10 as u64)
                .center(true)
                .invert(true)
                .build()
                .unwrap()
                .to_code(),
            "surface(file = \"shape.dat\", center = true, invert = true, convexity = 10);"
        );
    }
}
