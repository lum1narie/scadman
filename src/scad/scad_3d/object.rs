// cylinder(h,r|d,center)
// cylinder(h,r1|d1,r2|d2,center)
// polyhedron(points, faces, convexity)
// import("….ext", convexity)
// linear_extrude(height,center,convexity,twist,slices)
// rotate_extrude(angle,convexity)
// surface(file = "….ext",center,convexity)

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
}
