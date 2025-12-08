#![allow(unused_crate_dependencies)]
#![allow(missing_docs)]

#[cfg(test)]
mod tests {
    use scadman::prelude::*;

    #[test]
    fn test_primitive_scad_method() {
        assert_eq!(
            Square::build_with(|sb| {
                let _ = sb.size(10.0);
            })
            .to_code(),
            "square(size = 10);\n"
        );
        assert_eq!(
            Cube::build_with(|cb| {
                let _ = cb.size(5.0);
            })
            .to_code(),
            "cube(size = 5);\n"
        );
    }

    #[test]
    fn test_block() {
        assert_eq!(
            ScadObject2D::from([
                Square::build_with(|sb| {
                    let _ = sb.size(10.0);
                }),
                Translate2D::build_with(|tb| {
                    let _ = tb.v([30., 0.]);
                })
                .apply_to(Circle::build_with(|cb| {
                    let _ = cb.d(10.0);
                }))
            ])
            .to_code(),
            r"{
  square(size = 10);
  translate([30, 0])
    circle(d = 10);
}
"
        );
    }

    #[test]
    #[should_panic(
        expected = "A modifier cannot be converted to SCAD code directly without a child object. Use .apply_to() or similar methods."
    )]
    fn test_modifier_into_fail() {
        Into::<ScadObject2D>::into(Translate2D::build_with(|tb| {
            let _ = tb.v([1.0, 2.0]);
        }));
    }

    #[test]
    #[should_panic(
        expected = "A modifier cannot be converted to SCAD code directly without a child object. Use .apply_to() or similar methods."
    )]
    fn test_modifier_code_fail() {
        Translate2D::build_with(|tb| {
            let _ = tb.v([1.0, 2.0]);
        })
        .to_code();
    }

    #[test]
    fn test_modifier_apply_chaining() {
        let square_scad = Square::build_with(|sb| {
            let _ = sb.size(10.0);
        });

        // Chain modifier definitions
        let translated_modifier = Translate2D::build_with(|tb| {
            let _ = tb.v([10.0, 0.0]);
        });
        let rotated_modifier = Rotate2D::build_with(|rb| {
            let _ = rb.deg(45.0);
        });

        // Apply chained modifiers: first translate, then rotate
        let translated_rotated =
            rotated_modifier.apply_to(translated_modifier.apply_to(square_scad));

        assert_eq!(
            translated_rotated.to_code(),
            "rotate(a = 45)
  translate([10, 0])
    square(size = 10);
"
        );

        let cube_scad = Cube::build_with(|cb| {
            let _ = cb.size(5.0);
        });

        // Chain modifier definitions for 3D
        let translated_modifier_3d = Translate3D::build_with(|tb| {
            let _ = tb.v([1.0, 2.0, 3.0]);
        });
        let rotated_modifier_3d = Rotate3D::build_with(|rb| {
            let _ = rb.deg([0., 90., 0.]);
        });

        let translated_rotated_3d =
            rotated_modifier_3d.apply_to(translated_modifier_3d.apply_to(cube_scad));

        assert_eq!(
            translated_rotated_3d.to_code(),
            "rotate(a = [0, 90, 0])
  translate([1, 2, 3])
    cube(size = 5);
"
        );
    }

    #[test]
    fn test_modifier_apply_to_with_scad_able() {
        let square_scad = Square::build_with(|sb| {
            let _ = sb.size(10.0);
        });
        let translated = Translate2D::build_with(|tb| {
            let _ = tb.v([5.0, 5.0]);
        })
        .apply_to(square_scad);
        assert_eq!(
            translated.to_code(),
            "translate([5, 5])
  square(size = 10);
"
        );

        let cube_scad = Cube::build_with(|cb| {
            let _ = cb.size(10.0);
        });
        let rotated = Rotate3D::build_with(|rb| {
            let _ = rb.deg(45.0);
        })
        .apply_to(cube_scad);
        assert_eq!(
            rotated.to_code(),
            "rotate(a = 45)
  cube(size = 10);
"
        );
    }

    #[test]
    fn test_universal_modifier_scad_and_apply_to() {
        let square_scad = Square::build_with(|sb| {
            let _ = sb.size(10.0);
        });
        let cube_scad = Cube::build_with(|cb| {
            let _ = cb.size(10.0);
        });

        // Test Union 2D
        assert_eq!(
            Union::new().apply_to_2d(square_scad.clone()).to_code(),
            "union()
  square(size = 10);
"
        );
        // Test Union 3D
        assert_eq!(
            Union::new().apply_to_3d(cube_scad.clone()).to_code(),
            "union()
  cube(size = 10);
"
        );
        // Test Union Mixed (implicitly handled by .scad())
        assert_eq!(
            Union::new().apply_to_2d(square_scad.clone()).to_code(),
            "union()
  square(size = 10);
"
        );

        // Test Intersection 2D
        assert_eq!(
            Intersection::new().apply_to_2d(square_scad).to_code(),
            "intersection()
  square(size = 10);
"
        );
        // Test Intersection 3D
        assert_eq!(
            Intersection::new().apply_to_3d(cube_scad).to_code(),
            "intersection()
  cube(size = 10);
"
        );
    }
}
