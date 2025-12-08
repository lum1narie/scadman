#![allow(unused_crate_dependencies)]
#![allow(missing_docs)]

#[cfg(test)]
mod tests {
    use scadman::prelude::*;

    // Helper function to create a basic 2D object (Square)
    fn sq(size: f64) -> Square {
        Square::build_with(|sb| {
            let _ = sb.size(size);
        })
    }

    // Helper function to create a basic 3D object (Cube)
    fn cu(size: f64) -> Cube {
        Cube::build_with(|cb| {
            let _ = cb.size(size);
        })
    }

    #[test]
    fn test_add_2d() {
        let a = ScadObject2D::from(sq(10.0));
        let b = ScadObject2D::from(sq(20.0));
        let c = ScadObject2D::from(sq(30.0));
        let d = ScadObject2D::from(sq(40.0));

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
        let a = ScadObject3D::from(cu(10.0));
        let b = ScadObject3D::from(cu(20.0));
        let c = ScadObject3D::from(cu(30.0));
        let d = ScadObject3D::from(cu(40.0));

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
        let a = ScadObject2D::from(sq(10.0));
        let b = ScadObject2D::from(sq(5.0));
        let c = ScadObject2D::from(sq(2.0));

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
        let a = ScadObject3D::from(cu(10.0));
        let b = ScadObject3D::from(cu(5.0));
        let c = ScadObject3D::from(cu(2.0));

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
        let a = ScadObject2D::from(sq(10.0));
        let b = ScadObject2D::from(sq(20.0));
        let c = ScadObject2D::from(sq(30.0));
        let d = ScadObject2D::from(sq(40.0));

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
        let a = ScadObject3D::from(cu(10.0));
        let b = ScadObject3D::from(cu(20.0));
        let c = ScadObject3D::from(cu(30.0));
        let d = ScadObject3D::from(cu(40.0));

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
}
