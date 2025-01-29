// cube(size, center)
// cube([width,depth,height], center)
// cylinder(h,r|d,center)
// cylinder(h,r1|d1,r2|d2,center)
// polyhedron(points, faces, convexity)
// import("….ext", convexity)
// linear_extrude(height,center,convexity,twist,slices)
// rotate_extrude(angle,convexity)
// surface(file = "….ext",center,convexity)

use derive_builder::Builder;

use crate::scad::{ScadObject, ScadObject3D, Unit};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SphereSize {
    Radius(Unit),
    Diameter(Unit),
}

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Sphere {
    #[builder(setter(custom))]
    pub size: SphereSize,
    #[builder(setter(into, strip_option), default)]
    pub fa: Option<Unit>,
    #[builder(setter(into, strip_option), default)]
    pub r#fn: Option<u64>,
    #[builder(setter(into, strip_option), default)]
    pub fs: Option<Unit>,
}

impl SphereBuilder {
    pub fn r(&mut self, value: Unit) -> &mut Self {
        let new = self;
        new.size = Some(SphereSize::Radius(value));
        new
    }
    pub fn d(&mut self, value: Unit) -> &mut Self {
        let new = self;
        new.size = Some(SphereSize::Diameter(value));
        new
    }
}

impl ScadObject for Sphere {
    fn get_body(&self) -> String {
        let mut args: Vec<String> = Vec::new();
        let size_str: String = match self.size {
            SphereSize::Radius(r) => format!("r={}", r),
            SphereSize::Diameter(d) => format!("d={}", d),
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
        format!("sphere({})", args.join(", "))
    }
}

impl ScadObject3D for Sphere {}

#[cfg(test)]
mod tests {
    use crate::any_scads;

    use super::*;

    #[test]
    fn test_sphere() {
        assert_eq!(
            SphereBuilder::default().r(3.0).build().unwrap().to_code(),
            "sphere(r=3);"
        );
        assert_eq!(
            SphereBuilder::default().d(4.0).build().unwrap().to_code(),
            "sphere(d=4);"
        );
        assert_eq!(
            SphereBuilder::default()
                .r(3.0)
                .fa(0.5)
                .r#fn(20 as u64)
                .build()
                .unwrap()
                .to_code(),
            "sphere(r=3, $fa=0.5, $fn=20);"
        );
        assert_eq!(
            SphereBuilder::default()
                .r(3.0)
                .fs(40.)
                .fa(0.5)
                .build()
                .unwrap()
                .to_code(),
            "sphere(r=3, $fa=0.5, $fs=40);"
        );
        assert!(SphereBuilder::default()
            .fa(0.5)
            .r#fn(20 as u64)
            .fs(40.)
            .build()
            .is_err());
    }
}
