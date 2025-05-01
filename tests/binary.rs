#![allow(unused_crate_dependencies)]
#![allow(missing_docs)]

#[cfg(test)]
mod tests {
    use scadman::prelude::*;

    // Helper function to create a basic 2D object (Square)
    fn sq(size: f64) -> ScadObject {
        primitive_2d(Square::build_with(|sb| {
            let _ = sb.size(size);
        }))
    }

    // Helper function to create a basic 3D object (Cube)
    fn cu(size: f64) -> ScadObject {
        primitive_3d(Cube::build_with(|cb| {
            let _ = cb.size(size);
        }))
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
            "union() {\n  square(size = 10);\n  square(size = 20);\n}\n"
        );

        // Union + Object
        let obj2 = obj1.clone() + c.clone();
        assert_eq!(
            obj2.to_code(),
            "union() {\n  square(size = 10);\n  square(size = 20);\n  square(size = 30);\n}\n"
        );

        // Object + Union
        let obj3 = c.clone() + obj1.clone();
        assert_eq!(
            obj3.to_code(),
            "union() {\n  square(size = 30);\n  square(size = 10);\n  square(size = 20);\n}\n"
        );

        // Union + Union
        let obj4 = obj1.clone() + (c.clone() + d);
        assert_eq!(
        obj4.to_code(),
        "union() {\n  square(size = 10);\n  square(size = 20);\n  square(size = 30);\n  square(size = 40);\n}\n"
    );

        // Union + Difference (Should flatten lhs, not rhs)
        let diff = b - c;
        let obj5 = obj1.clone() + diff.clone();
        assert_eq!(
        obj5.to_code(),
        "union() {\n  square(size = 10);\n  square(size = 20);\n  difference() {\n    square(size = 20);\n    square(size = 30);\n  }\n}\n"
    );

        // Difference + Union (Should flatten rhs, not lhs)
        let obj6 = diff + obj1;
        assert_eq!(
        obj6.to_code(),
        "union() {\n  difference() {\n    square(size = 20);\n    square(size = 30);\n  }\n  square(size = 10);\n  square(size = 20);\n}\n"
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
            "union() {\n  cube(size = 10);\n  cube(size = 20);\n}\n"
        );

        // Union + Object
        let obj2 = obj1.clone() + c.clone();
        assert_eq!(
            obj2.to_code(),
            "union() {\n  cube(size = 10);\n  cube(size = 20);\n  cube(size = 30);\n}\n"
        );

        // Object + Union
        let obj3 = c.clone() + obj1.clone();
        assert_eq!(
            obj3.to_code(),
            "union() {\n  cube(size = 30);\n  cube(size = 10);\n  cube(size = 20);\n}\n"
        );

        // Union + Union
        let obj4 = obj1.clone() + (c.clone() + d);
        assert_eq!(
        obj4.to_code(),
        "union() {\n  cube(size = 10);\n  cube(size = 20);\n  cube(size = 30);\n  cube(size = 40);\n}\n"
    );

        // Union + Difference (Should flatten lhs, not rhs)
        let diff = b - c;
        let obj5 = obj1.clone() + diff.clone();
        assert_eq!(
        obj5.to_code(),
        "union() {\n  cube(size = 10);\n  cube(size = 20);\n  difference() {\n    cube(size = 20);\n    cube(size = 30);\n  }\n}\n"
    );

        // Difference + Union (Should flatten rhs, not lhs)
        let obj6 = diff + obj1;
        assert_eq!(
        obj6.to_code(),
        "union() {\n  difference() {\n    cube(size = 20);\n    cube(size = 30);\n  }\n  cube(size = 10);\n  cube(size = 20);\n}\n"
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
            "difference() {\n  square(size = 10);\n  square(size = 5);\n}\n"
        );

        // Difference - Object
        let obj2 = obj1.clone() - c.clone();
        assert_eq!(
            obj2.to_code(),
            "difference() {\n  square(size = 10);\n  square(size = 5);\n  square(size = 2);\n}\n"
        );

        // Object - Difference (Should not flatten rhs)
        let obj3 = a - obj1.clone();
        assert_eq!(
        obj3.to_code(),
        "difference() {\n  square(size = 10);\n  difference() {\n    square(size = 10);\n    square(size = 5);\n  }\n}\n"
    );

        // Difference - Union (Should not flatten rhs)
        let union = b + c;
        let obj4 = obj1.clone() - union.clone();
        assert_eq!(
        obj4.to_code(),
        "difference() {\n  square(size = 10);\n  square(size = 5);\n  union() {\n    square(size = 5);\n    square(size = 2);\n  }\n}\n"
    );

        // Union - Difference (Should not flatten anything)
        let obj5 = union - obj1;
        assert_eq!(
        obj5.to_code(),
        "difference() {\n  union() {\n    square(size = 5);\n    square(size = 2);\n  }\n  difference() {\n    square(size = 10);\n    square(size = 5);\n  }\n}\n"
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
            "difference() {\n  cube(size = 10);\n  cube(size = 5);\n}\n"
        );

        // Difference - Object
        let obj2 = obj1.clone() - c.clone();
        assert_eq!(
            obj2.to_code(),
            "difference() {\n  cube(size = 10);\n  cube(size = 5);\n  cube(size = 2);\n}\n"
        );

        // Object - Difference (Should not flatten rhs)
        let obj3 = a - obj1.clone();
        assert_eq!(
        obj3.to_code(),
        "difference() {\n  cube(size = 10);\n  difference() {\n    cube(size = 10);\n    cube(size = 5);\n  }\n}\n"
    );

        // Difference - Union (Should not flatten rhs)
        let union = b + c;
        let obj4 = obj1.clone() - union.clone();
        assert_eq!(
        obj4.to_code(),
        "difference() {\n  cube(size = 10);\n  cube(size = 5);\n  union() {\n    cube(size = 5);\n    cube(size = 2);\n  }\n}\n"
    );

        // Union - Difference (Should not flatten anything)
        let obj5 = union - obj1;
        assert_eq!(
        obj5.to_code(),
        "difference() {\n  union() {\n    cube(size = 5);\n    cube(size = 2);\n  }\n  difference() {\n    cube(size = 10);\n    cube(size = 5);\n  }\n}\n"
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
            "intersection() {\n  square(size = 10);\n  square(size = 20);\n}\n"
        );

        // Intersection * Object
        let obj2 = obj1.clone() * c.clone();
        assert_eq!(
        obj2.to_code(),
        "intersection() {\n  square(size = 10);\n  square(size = 20);\n  square(size = 30);\n}\n"
    );

        // Object * Intersection
        let obj3 = c.clone() * obj1.clone();
        assert_eq!(
        obj3.to_code(),
        "intersection() {\n  square(size = 30);\n  square(size = 10);\n  square(size = 20);\n}\n"
    );

        // Intersection * Intersection
        let obj4 = obj1.clone() * (c.clone() * d.clone());
        assert_eq!(
        obj4.to_code(),
        "intersection() {\n  square(size = 10);\n  square(size = 20);\n  square(size = 30);\n  square(size = 40);\n}\n"
    );

        // Intersection * Union (Should flatten lhs, not rhs)
        let union = c + d;
        let obj5 = obj1.clone() * union.clone();
        assert_eq!(
        obj5.to_code(),
        "intersection() {\n  square(size = 10);\n  square(size = 20);\n  union() {\n    square(size = 30);\n    square(size = 40);\n  }\n}\n"
    );

        // Union * Intersection (Should flatten rhs, not lhs)
        let obj6 = union * obj1;
        assert_eq!(
        obj6.to_code(),
        "intersection() {\n  union() {\n    square(size = 30);\n    square(size = 40);\n  }\n  square(size = 10);\n  square(size = 20);\n}\n"
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
            "intersection() {\n  cube(size = 10);\n  cube(size = 20);\n}\n"
        );

        // Intersection * Object
        let obj2 = obj1.clone() * c.clone();
        assert_eq!(
            obj2.to_code(),
            "intersection() {\n  cube(size = 10);\n  cube(size = 20);\n  cube(size = 30);\n}\n"
        );

        // Object * Intersection
        let obj3 = c.clone() * obj1.clone();
        assert_eq!(
            obj3.to_code(),
            "intersection() {\n  cube(size = 30);\n  cube(size = 10);\n  cube(size = 20);\n}\n"
        );

        // Intersection * Intersection
        let obj4 = obj1.clone() * (c.clone() * d.clone());
        assert_eq!(
        obj4.to_code(),
        "intersection() {\n  cube(size = 10);\n  cube(size = 20);\n  cube(size = 30);\n  cube(size = 40);\n}\n"
    );

        // Intersection * Union (Should flatten lhs, not rhs)
        let union = c + d;
        let obj5 = obj1.clone() * union.clone();
        assert_eq!(
        obj5.to_code(),
        "intersection() {\n  cube(size = 10);\n  cube(size = 20);\n  union() {\n    cube(size = 30);\n    cube(size = 40);\n  }\n}\n"
    );

        // Union * Intersection (Should flatten rhs, not lhs)
        let obj6 = union * obj1;
        assert_eq!(
        obj6.to_code(),
        "intersection() {\n  union() {\n    cube(size = 30);\n    cube(size = 40);\n  }\n  cube(size = 10);\n  cube(size = 20);\n}\n"
    );
    }

    #[test]
    #[should_panic(expected = "`Object2D + Object3D` is not allowed")]
    fn test_add_dimension_mismatch() {
        drop(sq(10.0) + cu(10.0));
    }

    #[test]
    #[should_panic(expected = "`Object3D + Object2D` is not allowed")]
    fn test_add_dimension_mismatch_rev() {
        drop(cu(10.0) + sq(10.0));
    }

    #[test]
    #[should_panic(expected = "`Object2D - Object3D` is not allowed")]
    fn test_sub_dimension_mismatch() {
        drop(sq(10.0) - cu(10.0));
    }

    #[test]
    #[should_panic(expected = "`Object3D - Object2D` is not allowed")]
    fn test_sub_dimension_mismatch_rev() {
        drop(cu(10.0) - sq(10.0));
    }

    #[test]
    #[should_panic(expected = "`Object2D * Object3D` is not allowed")]
    fn test_mul_dimension_mismatch() {
        drop(sq(10.0) * cu(10.0));
    }

    #[test]
    #[should_panic(expected = "`Object3D * Object2D` is not allowed")]
    fn test_mul_dimension_mismatch_rev() {
        drop(cu(10.0) * sq(10.0));
    }
}
