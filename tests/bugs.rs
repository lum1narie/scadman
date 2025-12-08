#![allow(unused_crate_dependencies)]
#![allow(missing_docs)]

#[cfg(test)]
mod tests {
    use scadman::prelude::*;

    pub fn cuboid_from_to(p0: Point3D, p1: Point3D) -> ScadObject3D {
        let from = Point3D::new(p0.x.min(p1.x), p0.y.min(p1.y), p0.z.min(p1.z));
        let to = Point3D::new(p0.x.max(p1.x), p0.y.max(p1.y), p0.z.max(p1.z));

        let c = Cube::build_with(|cb| {
            let _ = cb.size(to - from).center(false);
        });
        let norm = from.norm().abs() <= 1e-6;

        if norm {
            c
        } else {
            Translate3D::build_with(|tb| {
                let _ = tb.v(from);
            })
            .apply_to(c)
        }
        .commented(&format!(
            "cuboid_from_to([{}, {}, {}], [{}, {}, {}])",
            p0.x, p0.y, p0.z, p1.x, p1.y, p1.z
        ))
    }

    // 0.2.0
    #[test]
    fn test_bug_dup_comment() {
        let p0 = Point3D::new(1., 4., 3.);
        let p1 = Point3D::new(5., -2., 7.);
        let c = cuboid_from_to(p0, p1);

        assert_eq!(
            c.to_code(),
            r"/* cuboid_from_to([1, 4, 3], [5, -2, 7]) */
translate([1, -2, 3])
  cube(size = [4, 6, 4], center = false);
"
        );
    }

    // 0.2.0 ~ 0.2.1
    #[test]
    fn test_bug_op_merge() {
        let sq = Square::build_with(|sb| {
            let _ = sb.size([2.0, 2.0]);
        });
        let sqs = (0..=3)
            .map(|i| {
                Translate2D::build_with(|tb| {
                    let _ = tb.v([Unit::from(i) * 4., 0.]);
                })
                .apply_to(sq.clone())
            })
            .collect::<Vec<_>>();

        let b0 = ScadObject2D::from(&sqs[0..=1]);
        let b1 = ScadObject2D::from(&sqs[2..=3]);

        assert_eq!(
            (b0 + b1).to_code(),
            r"union() {
  {
    translate([0, 0])
      square(size = [2, 2]);
    translate([4, 0])
      square(size = [2, 2]);
  }
  {
    translate([8, 0])
      square(size = [2, 2]);
    translate([12, 0])
      square(size = [2, 2]);
  }
}
"
        );
    }
}
