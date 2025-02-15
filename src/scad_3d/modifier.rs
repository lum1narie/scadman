

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{any_scads3d, scad_3d::{CubeBuilder, SphereBuilder}, value_type::{RGB, RGBA}, AffineMatrix3D, Point3D};
    use super::*;

    #[test]
    fn test_translate3d() {
        let children = any_scads3d![
            CubeBuilder::default().size(10.).build().unwrap(),
            SphereBuilder::default().r(5.).build().unwrap(),
        ];
        assert_eq!(
            Translate3DBuilder::default()
                .v(Point3D::new(8., -4., 6.))
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "translate([8, -4, 6]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_rotate3d() {
        let children = any_scads3d![
            CubeBuilder::default().size(10.).build().unwrap(),
            SphereBuilder::default().r(5.).build().unwrap(),
        ];
        assert_eq!(
            Rotate3DBuilder::default()
                .deg(45.)
                .apply_to(children.clone())
                .build()
                .unwrap()
                .to_code(),
            "rotate(45) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            Rotate3DBuilder::default()
                .rad(PI / 4.)
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "rotate(45) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_mirror3d() {
        let children = any_scads3d![
            CubeBuilder::default().size(10.).build().unwrap(),
            SphereBuilder::default().r(5.).build().unwrap(),
        ];
        assert_eq!(
            Mirror3DBuilder::default()
                .v(Point3D::new(1., -1., 0.))
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "mirror([1, -1, 0]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_scale3d() {
        let children = any_scads3d![
            CubeBuilder::default().size(10.).build().unwrap(),
            SphereBuilder::default().r(5.).build().unwrap(),
        ];
        assert_eq!(
            Scale3DBuilder::default()
                .v(Point3D::new(3., 2., 4.))
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "scale([3, 2, 4]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_resize3d() {
        let children = any_scads3d![
            CubeBuilder::default().size(10.).build().unwrap(),
            SphereBuilder::default().r(5.).build().unwrap(),
        ];
        let mut r1 = Resize3DBuilder::default();
        _ = r1.size(Point3D::new(3., 2., 1.)).apply_to(children);
        assert_eq!(
            r1.clone().build().unwrap().to_code(),
            "resize([3, 2, 1]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            r1.clone().auto(true).build().unwrap().to_code(),
            "resize([3, 2, 1], auto = true) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            r1.auto([true, false, true]).build().unwrap().to_code(),
            "resize([3, 2, 1], auto = [true, false, true]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_multimatrix2d() {
        let children = any_scads3d![
            CubeBuilder::default().size(10.).build().unwrap(),
            SphereBuilder::default().r(5.).build().unwrap(),
        ];
        let m = AffineMatrix3D::new(1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.);
        assert_eq!(
            MultMatrix3DBuilder::default()
                .m(m)
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "multmatrix(m = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_color3d() {
        let children = any_scads3d![
            CubeBuilder::default().size(10.).build().unwrap(),
            SphereBuilder::default().r(5.).build().unwrap(),
        ];
        assert_eq!(
            Color3DBuilder::default()
                .c(RGB::new(0.3, 0.5, 0.2))
                .apply_to(children.clone())
                .build()
                .unwrap()
                .to_code(),
            "color(c = [0.3, 0.5, 0.2]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            Color3DBuilder::default()
                .c(RGB::new(0.3, 0.5, 0.2))
                .a(1.0)
                .apply_to(children.clone())
                .build()
                .unwrap()
                .to_code(),
            "color(c = [0.3, 0.5, 0.2], a = 1) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            Color3DBuilder::default()
                .c(RGBA::new(0.3, 0.5, 0.2, 1.0))
                .apply_to(children.clone())
                .build()
                .unwrap()
                .to_code(),
            "color(c = [0.3, 0.5, 0.2, 1]) {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            Color3DBuilder::default()
                .c("#C0FFEE".to_string())
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "color(\"#C0FFEE\") {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_hull() {
        let children = any_scads3d![
            CubeBuilder::default().size(10.).build().unwrap(),
            SphereBuilder::default().r(5.).build().unwrap(),
        ];
        assert_eq!(
            Hull3DBuilder::default()
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "hull() {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_minkowski() {
        let children = any_scads3d![
            CubeBuilder::default().size(10.).build().unwrap(),
            SphereBuilder::default().r(5.).build().unwrap(),
        ];
        assert_eq!(
            Minkowski3DBuilder::default()
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "minkowski() {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_binary_op() {
        let children = any_scads3d![
            CubeBuilder::default().size(10.).build().unwrap(),
            SphereBuilder::default().r(5.).build().unwrap(),
        ];
        assert_eq!(
            Union3DBuilder::default()
                .apply_to(children.clone())
                .build()
                .unwrap()
                .to_code(),
            "union() {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            Difference3DBuilder::default()
                .apply_to(children.clone())
                .build()
                .unwrap()
                .to_code(),
            "difference() {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
        assert_eq!(
            Intersection3DBuilder::default()
                .apply_to(children)
                .build()
                .unwrap()
                .to_code(),
            "intersection() {\n  cube(size = 10);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_linear_extrude() {
        todo!()
    }

    #[test]
    fn test_rotate_extrude() {
        todo!()
    }

    #[test]
    fn test_multi_level() {
        let objs = any_scads3d![
            CubeBuilder::default().size(10.).build().unwrap(),
            SphereBuilder::default().r(5.).build().unwrap(),
        ];

        let scad = Rotate3DBuilder::default()
            .deg(45.)
            .apply_to(
                Translate3DBuilder::default()
                    .v(Point3D::new(8., -4., 6.))
                    .apply_to(objs.clone())
                    .build()
                    .unwrap()
                    .into(),
            )
            .build()
            .unwrap();

        assert_eq!(
            scad.to_code(),
            "rotate(45) {\n  translate([8, -4, 6]) {\n    cube(size = 10);\n    sphere(r = 5);\n  }\n}"
        );
    }
}
