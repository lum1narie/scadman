#![allow(unused_crate_dependencies)]
#![allow(missing_docs)]
#![allow(deprecated)]

#[cfg(test)]
mod tests {
    use scadman::prelude::*;
    use scadman::legacy::v0_1_1::*;
    use std::iter;

    // Helper function to create a basic 2D object (Square)
    fn sq(size: f64) -> ScadObject2D {
        primitive_2d(Square::build_with(|sb| {
            let _ = sb.size(size);
        }))
    }

    // Helper function to create a basic 3D object (Cube)
    fn cu(size: f64) -> ScadObject3D {
        primitive_3d(Cube::build_with(|cb| {
            let _ = cb.size(size);
        }))
    }

    #[test]
    fn test_translate3d_apply_to() {
        // Build a translate modifier and a cube primitive using existing helpers,
        // then apply the modifier via the new typed-wrapper method.
        let cube = primitive_3d(Cube::build_with(|cb| {
            let _ = cb.size(10.);
        }));
        // Should match existing modifier_3d output for translate applied to cube
        let translate = Translate3D::build_with(|tb| {
            let _ = tb.v([1., 2., 3.]);
        });
        assert_eq!(
            modifier_3d(translate, cube).to_code(),
            "translate([1, 2, 3])
  cube(size = 10);
"
        );
    }

    #[test]
    fn test_translate2d_apply_to() {
        let square = primitive_2d(Square::build_with(|sb| {
            let _ = sb.size(5.);
        }));
        let translate = Translate2D::build_with(|tb| {
            let _ = tb.v([3., 4.]);
        });
        assert_eq!(
            modifier_2d(translate, square).to_code(),
            "translate([3, 4])
  square(size = 5);
"
        );
    }

    #[test]
    fn test_add_2d() {
        let a = sq(10.0);
        let b = sq(20.0);
        let c = sq(30.0);
        let d = sq(40.0);

        // Object + Object
        let obj1 = a + b.clone();
        assert_eq!(
            obj1.to_code(),
            "union() {
  square(size = 10);
  square(size = 20);
}
"
        );

        // Union + Object
        let obj2 = obj1.clone() + c.clone();
        assert_eq!(
            obj2.to_code(),
            "union() {
  square(size = 10);
  square(size = 20);
  square(size = 30);
}
"
        );

        // Object + Union
        let obj3 = c.clone() + obj1.clone();
        assert_eq!(
            obj3.to_code(),
            "union() {
  square(size = 30);
  square(size = 10);
  square(size = 20);
}
"
        );

        // Union + Union
        let obj4 = obj1.clone() + (c.clone() + d);
        assert_eq!(
            obj4.to_code(),
            "union() {
  square(size = 10);
  square(size = 20);
  square(size = 30);
  square(size = 40);
}
"
        );

        // Union + Difference (Should flatten lhs, not rhs)
        let diff = b - c;
        let obj5 = obj1.clone() + diff.clone();
        assert_eq!(
            obj5.to_code(),
            "union() {
  square(size = 10);
  square(size = 20);
  difference() {
    square(size = 20);
    square(size = 30);
  }
}
"
        );

        // Difference + Union (Should flatten rhs, not lhs)
        let obj6 = diff + obj1;
        assert_eq!(
            obj6.to_code(),
            "union() {
  difference() {
    square(size = 20);
    square(size = 30);
  }
  square(size = 10);
  square(size = 20);
}
"
        );
    }

    #[test]
    fn test_add_3d() {
        let a = cu(10.0);
        let b = cu(20.0);
        let c = cu(30.0);
        let d = cu(40.0);

        // Object + Object
        let obj1 = a + b.clone();
        assert_eq!(
            obj1.to_code(),
            "union() {
  cube(size = 10);
  cube(size = 20);
}
"
        );

        // Union + Object
        let obj2 = obj1.clone() + c.clone();
        assert_eq!(
            obj2.to_code(),
            "union() {
  cube(size = 10);
  cube(size = 20);
  cube(size = 30);
}
"
        );

        // Object + Union
        let obj3 = c.clone() + obj1.clone();
        assert_eq!(
            obj3.to_code(),
            "union() {
  cube(size = 30);
  cube(size = 10);
  cube(size = 20);
}
"
        );

        // Union + Union
        let obj4 = obj1.clone() + (c.clone() + d);
        assert_eq!(
            obj4.to_code(),
            "union() {
  cube(size = 10);
  cube(size = 20);
  cube(size = 30);
  cube(size = 40);
}
"
        );

        // Union + Difference (Should flatten lhs, not rhs)
        let diff = b - c;
        let obj5 = obj1.clone() + diff.clone();
        assert_eq!(
            obj5.to_code(),
            "union() {
  cube(size = 10);
  cube(size = 20);
  difference() {
    cube(size = 20);
    cube(size = 30);
  }
}
"
        );

        // Difference + Union (Should flatten rhs, not lhs)
        let obj6 = diff + obj1;
        assert_eq!(
            obj6.to_code(),
            "union() {
  difference() {
    cube(size = 20);
    cube(size = 30);
  }
  cube(size = 10);
  cube(size = 20);
}
"
        );
    }

    #[test]
    fn test_sub_2d() {
        let a = sq(10.0);
        let b = sq(5.0);
        let c = sq(2.0);

        // Object - Object
        let obj1 = a.clone() - b.clone();
        assert_eq!(
            obj1.to_code(),
            "difference() {
  square(size = 10);
  square(size = 5);
}
"
        );

        // Difference - Object
        let obj2 = obj1.clone() - c.clone();
        assert_eq!(
            obj2.to_code(),
            "difference() {
  square(size = 10);
  square(size = 5);
  square(size = 2);
}
"
        );

        // Object - Difference (Should not flatten rhs)
        let obj3 = a - obj1.clone();
        assert_eq!(
            obj3.to_code(),
            "difference() {
  square(size = 10);
  difference() {
    square(size = 10);
    square(size = 5);
  }
}
"
        );

        // Difference - Union (Should not flatten rhs)
        let union = b + c;
        let obj4 = obj1.clone() - union.clone();
        assert_eq!(
            obj4.to_code(),
            "difference() {
  square(size = 10);
  square(size = 5);
  union() {
    square(size = 5);
    square(size = 2);
  }
}
"
        );

        // Union - Difference (Should not flatten anything)
        let obj5 = union - obj1;
        assert_eq!(
            obj5.to_code(),
            "difference() {
  union() {
    square(size = 5);
    square(size = 2);
  }
  difference() {
    square(size = 10);
    square(size = 5);
  }
}
"
        );
    }

    #[test]
    fn test_sub_3d() {
        let a = cu(10.0);
        let b = cu(5.0);
        let c = cu(2.0);

        // Object - Object
        let obj1 = a.clone() - b.clone();
        assert_eq!(
            obj1.to_code(),
            "difference() {
  cube(size = 10);
  cube(size = 5);
}
"
        );

        // Difference - Object
        let obj2 = obj1.clone() - c.clone();
        assert_eq!(
            obj2.to_code(),
            "difference() {
  cube(size = 10);
  cube(size = 5);
  cube(size = 2);
}
"
        );

        // Object - Difference (Should not flatten rhs)
        let obj3 = a - obj1.clone();
        assert_eq!(
            obj3.to_code(),
            "difference() {
  cube(size = 10);
  difference() {
    cube(size = 10);
    cube(size = 5);
  }
}
"
        );

        // Difference - Union (Should not flatten rhs)
        let union = b + c;
        let obj4 = obj1.clone() - union.clone();
        assert_eq!(
            obj4.to_code(),
            "difference() {
  cube(size = 10);
  cube(size = 5);
  union() {
    cube(size = 5);
    cube(size = 2);
  }
}
"
        );

        // Union - Difference (Should not flatten anything)
        let obj5 = union - obj1;
        assert_eq!(
            obj5.to_code(),
            "difference() {
  union() {
    cube(size = 5);
    cube(size = 2);
  }
  difference() {
    cube(size = 10);
    cube(size = 5);
  }
}
"
        );
    }

    #[test]
    fn test_mul_2d() {
        let a = sq(10.0);
        let b = sq(20.0);
        let c = sq(30.0);
        let d = sq(40.0);

        // Object * Object
        let obj1 = a * b;
        assert_eq!(
            obj1.to_code(),
            "intersection() {
  square(size = 10);
  square(size = 20);
}
"
        );

        // Intersection * Object
        let obj2 = obj1.clone() * c.clone();
        assert_eq!(
            obj2.to_code(),
            "intersection() {
  square(size = 10);
  square(size = 20);
  square(size = 30);
}
"
        );

        // Object * Intersection
        let obj3 = c.clone() * obj1.clone();
        assert_eq!(
            obj3.to_code(),
            "intersection() {
  square(size = 30);
  square(size = 10);
  square(size = 20);
}
"
        );

        // Intersection * Intersection
        let obj4 = obj1.clone() * (c.clone() * d.clone());
        assert_eq!(
            obj4.to_code(),
            "intersection() {
  square(size = 10);
  square(size = 20);
  square(size = 30);
  square(size = 40);
}
"
        );

        // Intersection * Union (Should flatten lhs, not rhs)
        let union = c + d;
        let obj5 = obj1.clone() * union.clone();
        assert_eq!(
            obj5.to_code(),
            "intersection() {
  square(size = 10);
  square(size = 20);
  union() {
    square(size = 30);
    square(size = 40);
  }
}
"
        );

        // Union * Intersection (Should flatten rhs, not lhs)
        let obj6 = union * obj1;
        assert_eq!(
            obj6.to_code(),
            "intersection() {
  union() {
    square(size = 30);
    square(size = 40);
  }
  square(size = 10);
  square(size = 20);
}
"
        );
    }

    #[test]
    fn test_mul_3d() {
        let a = cu(10.0);
        let b = cu(20.0);
        let c = cu(30.0);
        let d = cu(40.0);

        // Object * Object
        let obj1 = a * b;
        assert_eq!(
            obj1.to_code(),
            "intersection() {
  cube(size = 10);
  cube(size = 20);
}
"
        );

        // Intersection * Object
        let obj2 = obj1.clone() * c.clone();
        assert_eq!(
            obj2.to_code(),
            "intersection() {
  cube(size = 10);
  cube(size = 20);
  cube(size = 30);
}
"
        );

        // Object * Intersection
        let obj3 = c.clone() * obj1.clone();
        assert_eq!(
            obj3.to_code(),
            "intersection() {
  cube(size = 30);
  cube(size = 10);
  cube(size = 20);
}
"
        );

        // Intersection * Intersection
        let obj4 = obj1.clone() * (c.clone() * d.clone());
        assert_eq!(
            obj4.to_code(),
            "intersection() {
  cube(size = 10);
  cube(size = 20);
  cube(size = 30);
  cube(size = 40);
}
"
        );

        // Intersection * Union (Should flatten lhs, not rhs)
        let union = c + d;
        let obj5 = obj1.clone() * union.clone();
        assert_eq!(
            obj5.to_code(),
            "intersection() {
  cube(size = 10);
  cube(size = 20);
  union() {
    cube(size = 30);
    cube(size = 40);
  }
}
"
        );

        // Union * Intersection (Should flatten rhs, not lhs)
        let obj6 = union * obj1;
        assert_eq!(
            obj6.to_code(),
            "intersection() {
  union() {
    cube(size = 30);
    cube(size = 40);
  }
  cube(size = 10);
  cube(size = 20);
}
"
        );
    }

    ///
    /// ```compile_fail
    /// drop(sq(10.0) + cu(10.0));
    /// ```
    /// ```compile_fail
    /// drop(cu(10.0) + sq(10.0));
    /// ```
    /// ```compile_fail
    /// drop(sq(10.0) - cu(10.0));
    /// ```
    /// ```compile_fail
    /// drop(cu(10.0) - sq(10.0));
    /// ```
    /// ```compile_fail
    /// drop(sq(10.0) * cu(10.0));
    /// ```
    /// ```compile_fail
    /// drop(cu(10.0) * sq(10.0));
    /// ```
    #[test]
    fn test_op_fail() {}

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
    ) -> ScadObject2D {
        let outer = modifier_2d(
            Translate2D::build_with(|tb| {
                let _ = tb.v(corner
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
                let _ = sb.size(r);
            })),
        );

        let inner = modifier_2d(
            Translate2D::build_with(|tb| {
                let _ = tb.v(corner
                    + if pos_x_out { -r } else { r } * Point2D::x()
                    + if pos_y_out { -r } else { r } * Point2D::y());
            }),
            primitive_2d(Circle::build_with(|cb| {
                let _ = cb.r(r).r#fn(r#fn);
            })),
        );

        outer - inner
    }

    fn generate_clamp() -> ScadObject3D {
        let shape_2d = {
            let body_x_a0: f64 = -CLAMP_UPPER_LENGTH - CLAMP_BACK_PLATE_THICKNESS;
            let body_x_a1: f64 = -CLAMP_LOWER_LENGTH - CLAMP_BACK_PLATE_THICKNESS;
            let body_x_a2: f64 = -CLAMP_BACK_PLATE_THICKNESS;
            let body_x_a3: f64 = 0.;
            let body_y_a0: f64 = 0.;
            let body_y_a1: f64 = CLAMP_PLATE_THICKNESS;
            let body_y_a2: f64 = CLAMP_SPAN + CLAMP_PLATE_THICKNESS;
            let body_y_a3 = 2.0_f64.mul_add(CLAMP_PLATE_THICKNESS, CLAMP_SPAN);
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
                    let _ = pb.points(body_points);
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
                let _ = pb.points(tooth_points);
            }));
            let teeth = CLAMP_NAIL_POS
                .iter()
                .map(|x| {
                    modifier_2d(
                        Translate2D::build_with(|tb| {
                            let _ = tb.v([body_x_a2 - x, body_y_a2]);
                        }),
                        tooth_shape.clone(),
                    )
                })
                .collect::<Vec<_>>();

            modifier_2d_commented(
                Union::new(),
                block_2d(&iter::once(body_rounded).chain(teeth).collect::<Vec<_>>()),
                "body with teeth",
            )
        };

        modifier_3d(
            LinearExtrude::build_with(|lb| {
                let _ = lb.height(CLAMP_Z_SIZE);
            }),
            shape_2d,
        )
    }

    fn generate_body() -> ScadObject3D {
        let hook_pos_y: f64 = CLAMP_SPAN / 2. + CLAMP_PLATE_THICKNESS;

        let hook = {
            let hook_outer = (modifier_3d(
                Translate3D::build_with(|tb| {
                    let _ = tb.v([0., 0., -SMALL_OVERLAP]);
                }),
                primitive_3d(Cylinder::build_with(|cb| {
                    let _ = cb
                        .h(2.0_f64.mul_add(SMALL_OVERLAP, HOOK_LENGTH))
                        .r(HOOK_OUTER_R)
                        .r#fn(64_u64);
                })),
            ) + modifier_3d(
                Translate3D::build_with(|tb| {
                    let _ = tb.v([0., 0., HOOK_LENGTH]);
                }),
                primitive_3d(Cylinder::build_with(|cb| {
                    let _ = cb.h(HOOK_END_LENGTH).r(HOOK_END_R).r#fn(64_u64);
                })),
            ))
            .commented("hook outer");

            let hook_void = modifier_3d_commented(
                Translate3D::build_with(|tb| {
                    let _ = tb.v([0., 0., HOOK_INFILL_HEIGHT]);
                }),
                primitive_3d(Cylinder::build_with(|cb| {
                    let _ = cb
                        .h(HOOK_LENGTH + HOOK_END_LENGTH - HOOK_INFILL_HEIGHT + SMALL_OVERLAP)
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
                    let _ = tb.v([-SMALL_OVERLAP, hook_pos_y, CLAMP_Z_SIZE / 2.]);
                }),
                modifier_3d(
                    Rotate3D::build_with(|rb| {
                        let _ = rb.deg([0., 90., 0.]);
                    }),
                    hook,
                ),
            )
    }

    #[test]
    fn test_clamp() {
        assert_eq!(
            generate_clamp().to_code(),
            r"linear_extrude(height = 45)
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
"
        );
    }

    #[test]
    fn test_body() {
        assert_eq!(
            generate_body().to_code(),
            r"union() {
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
"
        );
    }
}
