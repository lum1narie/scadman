#[cfg(test)]
mod tests {

    use std::{
        fs::{create_dir_all, File},
        io::Write as _,
        iter,
    };

    use scadman::prelude::*;

    const SMALL_OVERLAP: f64 = 0.025;

    const CLAMP_Z_SIZE: f64 = 45.;
    const CLAMP_PLATE_THICKNESS: f64 = 5.;
    const CLAMP_BACK_PLATE_THICKNESS: f64 = 5.;
    const CLAMP_UPPER_LENGTH: f64 = 30.;
    const CLAMP_SPAN: f64 = 19.;
    const CLAMP_LOWER_LENGTH: f64 = 10.;
    const CLAMP_CHAMFER_R: f64 = 2.;

    const CLAMP_NAIL_HEIGHT: f64 = 0.4;
    const CLAMP_NAIL_BASE_WIDTH: f64 = 3.6;
    const CLAMP_NAIL_TOP_WIDTH: f64 = 1.8;
    const CLAMP_NAIL_POS: [f64; 2] = [4.5, 20.];

    const HOOK_OUTER_R: f64 = 14.;
    const HOOK_INNER_R: f64 = 12.;
    const HOOK_INFILL_HEIGHT: f64 = 5.;
    const HOOK_LENGTH: f64 = 60.;
    const HOOK_END_R: f64 = 21.;
    const HOOK_END_LENGTH: f64 = 5.;

    fn generate_lattice_r_void(
        corner: &Point2D,
        r: f64,
        pos_x_out: bool,
        pos_y_out: bool,
        r#fn: u64,
    ) -> ScadObject {
        let outer = modifier_2d(
            Translate2D::build_with(|tb| {
                tb.v(corner
                    - r * if pos_x_out {
                        Point2D::x()
                    } else {
                        Point2D::zeros()
                    }
                    - r * if pos_y_out {
                        Point2D::y()
                    } else {
                        Point2D::zeros()
                    });
            }),
            primitive_2d(Square::build_with(|sb| {
                sb.size(r);
            })),
        );

        let inner = modifier_2d(
            Translate2D::build_with(|tb| {
                tb.v(corner
                    + if pos_x_out { -r } else { r } * Point2D::x()
                    + if pos_y_out { -r } else { r } * Point2D::y());
            }),
            primitive_2d(Circle::build_with(|cb| {
                cb.r(r).r#fn(r#fn);
            })),
        );

        outer - inner
    }

    fn generate_clamp() -> ScadObject {
        let shape_2d = {
            let body_x_a0: f64 = -CLAMP_UPPER_LENGTH - CLAMP_BACK_PLATE_THICKNESS;
            let body_x_a1: f64 = -CLAMP_LOWER_LENGTH - CLAMP_BACK_PLATE_THICKNESS;
            let body_x_a2: f64 = -CLAMP_BACK_PLATE_THICKNESS;
            let body_x_a3: f64 = 0.;
            let body_y_a0: f64 = 0.;
            let body_y_a1: f64 = CLAMP_PLATE_THICKNESS;
            let body_y_a2: f64 = CLAMP_SPAN + CLAMP_PLATE_THICKNESS;
            let body_y_a3: f64 = CLAMP_SPAN + 2. * CLAMP_PLATE_THICKNESS;
            let body_points = vec![
                [body_x_a3, body_y_a0],
                [body_x_a3, body_y_a3],
                [body_x_a0, body_y_a3],
                [body_x_a0, body_y_a2],
                [body_x_a2, body_y_a2],
                [body_x_a2, body_y_a1],
                [body_x_a1, body_y_a1],
                [body_x_a1, body_y_a0],
            ];
            let body = primitive_2d_commented(
                Polygon::build_with(|pb| {
                    pb.points(body_points);
                }),
                "body outer shape",
            );

            let body_rounded = modifier_2d_commented(
                Difference::new(),
                block_2d(&[
                    body,
                    generate_lattice_r_void(
                        &[body_x_a1, body_y_a1].into(),
                        CLAMP_CHAMFER_R,
                        false,
                        true,
                        64,
                    )
                    .commented("upper chamfer"),
                    generate_lattice_r_void(
                        &[body_x_a0, body_y_a2].into(),
                        CLAMP_CHAMFER_R,
                        false,
                        false,
                        64,
                    )
                    .commented("lower chamfer"),
                ]),
                "body rounded",
            );

            let tooth_x_a0: f64 = -CLAMP_NAIL_BASE_WIDTH;
            let tooth_x_a1: f64 = (-CLAMP_NAIL_TOP_WIDTH - CLAMP_NAIL_BASE_WIDTH) / 2.;
            let tooth_x_a2: f64 = (CLAMP_NAIL_TOP_WIDTH - CLAMP_NAIL_BASE_WIDTH) / 2.;
            let tooth_x_a3: f64 = 0.;
            let tooth_y_a0: f64 = -CLAMP_NAIL_HEIGHT;
            let tooth_y_a1: f64 = 0.;
            let tooth_y_a2: f64 = SMALL_OVERLAP;
            let tooth_points = vec![
                [tooth_x_a0, tooth_y_a1],
                [tooth_x_a1, tooth_y_a0],
                [tooth_x_a2, tooth_y_a0],
                [tooth_x_a3, tooth_y_a1],
                [tooth_x_a3, tooth_y_a2],
                [tooth_x_a0, tooth_y_a2],
            ];
            let tooth_shape = primitive_2d(Polygon::build_with(|pb| {
                pb.points(tooth_points);
            }));
            let teeth = CLAMP_NAIL_POS
                .iter()
                .map(|x| {
                    modifier_2d(
                        Translate2D::build_with(|tb| {
                            tb.v([body_x_a2 - x, body_y_a2]);
                        }),
                        tooth_shape.clone(),
                    )
                })
                .collect::<Vec<_>>();

            modifier_2d_commented(
                Union::new(),
                block_2d(
                    &iter::once(body_rounded)
                        .chain(teeth.into_iter())
                        .collect::<Vec<_>>(),
                ),
                "body with teeth",
            )
        };

        modifier_3d(
            LinearExtrude::build_with(|lb| {
                lb.height(CLAMP_Z_SIZE);
            }),
            shape_2d,
        )
    }

    fn generate_body() -> ScadObject {
        let hook_pos_y: f64 = CLAMP_SPAN / 2. + CLAMP_PLATE_THICKNESS;

        let hook = {
            let hook_outer = (modifier_3d(
                Translate3D::build_with(|tb| {
                    tb.v([0., 0., -SMALL_OVERLAP]);
                }),
                primitive_3d(Cylinder::build_with(|cb| {
                    cb.h(HOOK_LENGTH + 2. * SMALL_OVERLAP)
                        .r(HOOK_OUTER_R)
                        .r#fn(64_u64);
                })),
            ) + modifier_3d(
                Translate3D::build_with(|tb| {
                    tb.v([0., 0., HOOK_LENGTH]);
                }),
                primitive_3d(Cylinder::build_with(|cb| {
                    cb.h(HOOK_END_LENGTH).r(HOOK_END_R).r#fn(64_u64);
                })),
            ))
            .commented("hook outer");

            let hook_void = modifier_3d_commented(
                Translate3D::build_with(|tb| {
                    tb.v([0., 0., HOOK_INFILL_HEIGHT]);
                }),
                primitive_3d(Cylinder::build_with(|cb| {
                    cb.h(HOOK_LENGTH + HOOK_END_LENGTH - HOOK_INFILL_HEIGHT + SMALL_OVERLAP)
                        .r(HOOK_INNER_R)
                        .r#fn(6_u64);
                })),
                "hook void",
            );

            hook_outer - hook_void
        };

        generate_clamp()
            + modifier_3d(
                Translate3D::build_with(|tb| {
                    tb.v([-SMALL_OVERLAP, hook_pos_y, CLAMP_Z_SIZE / 2.]);
                }),
                modifier_3d(
                    Rotate3D::build_with(|rb| {
                        rb.deg([0., 90., 0.]);
                    }),
                    hook,
                ),
            )
    }

    #[test]
    fn test_clamp() {
        assert_eq!(
            generate_clamp().to_code(),
            r#"linear_extrude(height = 45)
  /* body with teeth */
  union() {
    /* body rounded */
    difference() {
      /* body outer shape */
      polygon(points = [[0, 0], [0, 29], [-35, 29], [-35, 24], [-5, 24], [-5, 5], [-15, 5], [-15, 0]]);
      /* upper chamfer */
      difference() {
        translate([-15, 3])
          square(size = 2);
        translate([-13, 3])
          circle(r = 2, $fn = 64);
      }
      /* lower chamfer */
      difference() {
        translate([-35, 24])
          square(size = 2);
        translate([-33, 26])
          circle(r = 2, $fn = 64);
      }
    }
    translate([-9.5, 24])
      polygon(points = [[-3.6, 0], [-2.7, -0.4], [-0.9, -0.4], [0, 0], [0, 0.025], [-3.6, 0.025]]);
    translate([-25, 24])
      polygon(points = [[-3.6, 0], [-2.7, -0.4], [-0.9, -0.4], [0, 0], [0, 0.025], [-3.6, 0.025]]);
  }
"#
        )
    }

    #[test]
    fn test_body() {
        assert_eq!(
            generate_body().to_code(),
            r#"union() {
  linear_extrude(height = 45)
    /* body with teeth */
    union() {
      /* body rounded */
      difference() {
        /* body outer shape */
        polygon(points = [[0, 0], [0, 29], [-35, 29], [-35, 24], [-5, 24], [-5, 5], [-15, 5], [-15, 0]]);
        /* upper chamfer */
        difference() {
          translate([-15, 3])
            square(size = 2);
          translate([-13, 3])
            circle(r = 2, $fn = 64);
        }
        /* lower chamfer */
        difference() {
          translate([-35, 24])
            square(size = 2);
          translate([-33, 26])
            circle(r = 2, $fn = 64);
        }
      }
      translate([-9.5, 24])
        polygon(points = [[-3.6, 0], [-2.7, -0.4], [-0.9, -0.4], [0, 0], [0, 0.025], [-3.6, 0.025]]);
      translate([-25, 24])
        polygon(points = [[-3.6, 0], [-2.7, -0.4], [-0.9, -0.4], [0, 0], [0, 0.025], [-3.6, 0.025]]);
    }
  translate([-0.025, 14.5, 22.5])
    rotate(a = [0, 90, 0])
      difference() {
        /* hook outer */
        union() {
          translate([0, 0, -0.025])
            cylinder(h = 60.05, r = 14, $fn = 64);
          translate([0, 0, 60])
            cylinder(h = 5, r = 21, $fn = 64);
        }
        /* hook void */
        translate([0, 0, 5])
          cylinder(h = 60.025, r = 12, $fn = 6);
      }
}
"#
        )
    }
}
