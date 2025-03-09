use std::ops::{Add, Mul, Sub};

use ambassador::Delegate;
use derive_builder::Builder;
use derive_more::derive::From;

use crate::{
    __generate_scad_options, __impl_modifier, __impl_modifier_to_code, __impl_scad2d,
    common::{AffineMatrix2D, Point2D, ScadObjectTrait, Unit},
    internal::generate_body,
    scad_3d::ScadObject3D,
    scad_display::{ambassador_impl_ScadDisplay, ScadDisplay},
    value_type::{Angle, Color},
};

use super::ScadObject2D;

/// Give an implementation of a modifier 2D object
/// that has no parameters and is applied to 2D objects.
macro_rules! __impl_operator_2d {
    ( $type:ident, $name:expr_2021 ) => {
        #[doc = concat!($name,
        " modifier `", $name, "()` in SCAD.
        This Rust type is regarded as 2D object and only applys to 2D objects.")]
        #[allow(missing_debug_implementations)]
        #[derive(derive_builder::Builder, Debug, Clone)]
        pub struct $type {
            /// Children objects to apply this modifier.
            #[builder(setter(name = "apply_to", into), default)]
            pub children: Vec<$crate::scad_2d::ScadObject2D>,
        }

        $crate::__impl_scad2d!($type);

        impl $crate::ScadObjectTrait for $type {
            fn get_body(&self) -> String {
                generate_body(
                    $name,
                    $crate::__generate_scad_options!(
                        ;;
                    ),
                )
            }

            $crate::__impl_modifier_to_code!();
        }

        impl $type {
            #[allow(dead_code)]
            const fn new() -> Self {
                Self {children: Vec::new()}
            }
        }

        $crate::__impl_modifier!($type, $crate::scad_2d::ScadObject2D);
    };
}

/// Translate modifier `translate()` in SCAD.
/// This Rust type is regarded as 2D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone)]
pub struct Translate2D {
    /// Translation vector.
    /// `v` option in SCAD.
    #[builder(setter(into))]
    pub v: Point2D,
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into), default)]
    pub children: Vec<ScadObject2D>,
}

__impl_scad2d!(Translate2D);

impl ScadObjectTrait for Translate2D {
    fn get_body(&self) -> String {
        generate_body(
            "translate",
            __generate_scad_options!(
                ("", self.v);;
            ),
        )
    }

    __impl_modifier_to_code!();
}

__impl_modifier!(Translate2D, ScadObject2D);

/// Rotate modifier `rotate()` in SCAD.
/// This Rust type is regarded as 2D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone)]
pub struct Rotate2D {
    /// Rotation angle.
    /// `a` option in SCAD.
    ///
    /// See also [`Angle`].
    #[builder(setter(custom))]
    pub a: Angle,
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into), default)]
    pub children: Vec<ScadObject2D>,
}

__impl_scad2d!(Rotate2D);

impl Rotate2DBuilder {
    /// Set rotation angle in degrees.
    ///
    /// # Arguments
    ///
    /// + `value` - The rotation angle in degrees.
    pub fn deg(&mut self, value: Unit) -> &mut Self {
        let new = self;
        new.a = Some(Angle::Deg(value));
        new
    }
    /// Set rotation angle in radians.
    ///
    /// # Arguments
    ///
    /// + `value` - The rotation angle in radians.
    pub fn rad(&mut self, value: Unit) -> &mut Self {
        let new = self;
        new.a = Some(Angle::Rad(value));
        new
    }
}

impl ScadObjectTrait for Rotate2D {
    fn get_body(&self) -> String {
        generate_body(
            "rotate",
            __generate_scad_options!(
                ("", self.a);;
            ),
        )
    }

    __impl_modifier_to_code!();
}

__impl_modifier!(Rotate2D, ScadObject2D);

/// Scale modifier `scale()` in SCAD.
/// This Rust type is regarded as 2D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone)]
pub struct Scale2D {
    /// Scaling vector.
    /// `v` option in SCAD.
    #[builder(setter(into))]
    pub v: Point2D,
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into), default)]
    pub children: Vec<ScadObject2D>,
}

__impl_scad2d!(Scale2D);

impl ScadObjectTrait for Scale2D {
    fn get_body(&self) -> String {
        generate_body(
            "scale",
            __generate_scad_options!(
                ("", self.v);;
            ),
        )
    }

    __impl_modifier_to_code!();
}

__impl_modifier!(Scale2D, ScadObject2D);

/// `auto` option in 2D resize modifier.
#[derive(Copy, Clone, Debug, PartialEq, Eq, From, Delegate)]
#[delegate(ScadDisplay)]
pub enum ResizeAuto {
    /// Same value for all dimensions.
    B(bool),
    /// Values for each dimension.
    V([bool; 2]),
}

/// Resize modifier `resize()` in SCAD.
/// This Rust type is regarded as 2D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone)]
pub struct Resize2D {
    /// New size.
    ///
    /// `0` means no change if the corresponding dimension of `auto` is `false`,
    /// or auto value if `true`.
    #[builder(setter(into))]
    pub size: Point2D,
    /// `auto` option in SCAD.
    ///
    /// See also [`ResizeAuto`].
    #[builder(setter(into, strip_option), default)]
    pub auto: Option<ResizeAuto>,
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into), default)]
    pub children: Vec<ScadObject2D>,
}

__impl_scad2d!(Resize2D);

impl ScadObjectTrait for Resize2D {
    fn get_body(&self) -> String {
        generate_body(
            "resize",
            __generate_scad_options!(
                ("", self.size);
                ("auto", self.auto);
            ),
        )
    }

    __impl_modifier_to_code!();
}

__impl_modifier!(Resize2D, ScadObject2D);

/// Mirror modifier `mirror()` in SCAD.
/// This Rust type is regarded as 2D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone)]
pub struct Mirror2D {
    /// Normal vector of the mirror plane.
    #[builder(setter(into))]
    pub v: Point2D,
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into), default)]
    pub children: Vec<ScadObject2D>,
}

__impl_scad2d!(Mirror2D);

impl ScadObjectTrait for Mirror2D {
    fn get_body(&self) -> String {
        generate_body(
            "mirror",
            __generate_scad_options!(
                ("", self.v);;
            ),
        )
    }

    __impl_modifier_to_code!();
}

__impl_modifier!(Mirror2D, ScadObject2D);

/// Affine tranformation modifier `multmatrix()` in SCAD.
/// This Rust type is regarded as 2D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone)]
pub struct MultMatrix2D {
    /// Affine transformation matrix for 2D vector.
    #[builder(setter(into))]
    pub m: AffineMatrix2D,
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into), default)]
    pub children: Vec<ScadObject2D>,
}

__impl_scad2d!(MultMatrix2D);

impl ScadObjectTrait for MultMatrix2D {
    fn get_body(&self) -> String {
        generate_body(
            "multmatrix",
            __generate_scad_options!(
                ("m", self.m);;
            ),
        )
    }

    __impl_modifier_to_code!();
}

__impl_modifier!(MultMatrix2D, ScadObject2D);

/// Color modifier `color()` in SCAD.
/// This Rust type is regarded as 2D object and only applys to 2D objects.
#[derive(Builder, Debug, Clone)]
pub struct Color2D {
    /// Color.
    ///
    /// See also [`Color`].
    #[builder(setter(into))]
    pub c: Color,
    /// Alpha value.
    /// `a` option in SCAD.
    ///
    /// Set when the `color` is NOT [`Color::RGBA`].
    #[builder(setter(into, strip_option), default)]
    pub a: Option<Unit>,
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into), default)]
    pub children: Vec<ScadObject2D>,
}

__impl_scad2d!(Color2D);

impl ScadObjectTrait for Color2D {
    fn get_body(&self) -> String {
        generate_body(
            "color",
            __generate_scad_options!(
                (self.c.name(), self.c.clone());
                ("a", self.a);
            ),
        )
    }

    __impl_modifier_to_code!();
}

__impl_modifier!(Color2D, ScadObject2D);

/// Size of offset modifier for SCAD
#[derive(Copy, Clone, Debug, PartialEq, Delegate)]
#[delegate(ScadDisplay)]
pub enum OffsetSize {
    /// Radius of the radial offset.
    R(Unit),
    /// Delta of the delta offset.
    Delta(Unit),
}

impl OffsetSize {
    /// Returns the name of the key in SCAD code
    ///
    /// # Returns
    ///
    /// The name of the key in SCAD code
    pub const fn name(self) -> &'static str {
        match self {
            Self::R(_) => "r",
            Self::Delta(_) => "delta",
        }
    }
}

/// Offset modifier `offset()` in SCAD.
#[derive(Builder, Debug, Clone)]
pub struct Offset {
    /// Size of the offset.
    /// `r` or `delta` option in SCAD.
    ///
    /// See also [`OffsetSize`].
    #[builder(setter(custom))]
    pub size: OffsetSize,
    /// Flag to determine the shape should be chamfered or not.
    /// `chamfer` option in SCAD.
    ///
    /// This parameter has no effect on radial offsets.
    #[builder(setter(into, strip_option), default)]
    pub chamfer: Option<bool>,
    /// `$fa` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub fa: Option<Unit>,
    /// `$fn` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub r#fn: Option<u64>,
    /// `$fs` option in SCAD.
    #[builder(setter(into, strip_option), default)]
    pub fs: Option<Unit>,
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into), default)]
    pub children: Vec<ScadObject2D>,
}

__impl_scad2d!(Offset);

impl OffsetBuilder {
    /// Set `r` option in SCAD.
    ///
    /// # Arguments
    ///
    /// + `value` - `r` option in SCAD. This is the radial offset.
    pub fn r(&mut self, value: Unit) -> &mut Self {
        let new = self;
        new.size = Some(OffsetSize::R(value));
        new
    }
    /// Set `delta` option in SCAD.
    ///
    /// # Arguments
    ///
    /// + `value` - `delta` option in SCAD. This is the delta offset.
    pub fn delta(&mut self, value: Unit) -> &mut Self {
        let new = self;
        new.size = Some(OffsetSize::Delta(value));
        new
    }
}

impl ScadObjectTrait for Offset {
    fn get_body(&self) -> String {
        generate_body(
            "offset",
            __generate_scad_options!(
                (self.size.name(), self.size);
                ("chamfer", self.chamfer),
                ("$fa", self.fa),
                ("$fn", self.r#fn),
                ("$fs", self.fs);
            ),
        )
    }

    __impl_modifier_to_code!();
}

__impl_modifier!(Offset, ScadObject2D);

__impl_operator_2d!(Hull2D, "hull");
__impl_operator_2d!(Minkowski2D, "minkowski");
__impl_operator_2d!(Union2D, "union");
__impl_operator_2d!(Difference2D, "difference");
__impl_operator_2d!(Intersection2D, "intersection");

/// Projection modifier `projection()` in SCAD.
/// This Rust type is regarded as 2D object and only applys to 3D objects.
#[derive(Builder, Debug, Clone)]
pub struct Projection {
    /// Flag to determine the shape should be cut at z = 0 or not.
    ///
    /// If `cut` is `true`, the shape is cut at z = 0.
    /// If `cut` is `false`, the shape is as projected.
    #[builder(setter(into, strip_option), default)]
    pub cut: Option<bool>,
    /// Children objects to apply this modifier.
    #[builder(setter(name = "apply_to", into), default)]
    pub children: Vec<ScadObject3D>,
}

__impl_scad2d!(Projection);

impl ScadObjectTrait for Projection {
    fn get_body(&self) -> String {
        generate_body(
            "projection",
            __generate_scad_options!(
                ;("cut", self.cut);
            ),
        )
    }

    __impl_modifier_to_code!();
}

__impl_modifier!(Projection, ScadObject3D);

impl Add for ScadObject2D {
    type Output = Union2D;

    fn add(self, rhs: Self) -> Self::Output {
        Union2D::build_with(|ub| {
            let _ = ub.apply_to(vec![self, rhs]);
        })
    }
}
impl Mul for ScadObject2D {
    type Output = Intersection2D;

    fn mul(self, rhs: Self) -> Self::Output {
        Intersection2D::build_with(|ib| {
            let _ = ib.apply_to(vec![self, rhs]);
        })
    }
}
impl Sub for ScadObject2D {
    type Output = Difference2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Difference2D::build_with(|db| {
            let _ = db.apply_to(vec![self, rhs]);
        })
    }
}

#[cfg(test)]
mod tests {

    use std::f64::consts::PI;

    use super::*;
    use crate::{
        objects_2d, objects_3d,
        scad_2d::{Circle, Square},
        scad_3d::Sphere,
        value_type::{RGB, RGBA},
        ScadModifier as _,
    };

    fn get_children() -> Vec<ScadObject2D> {
        objects_2d![
            Square::build_with(|sb| {
                let _ = sb.size(10.);
            }),
            Circle::build_with(|cb| {
                let _ = cb.r(5.);
            }),
        ]
    }

    #[test]
    fn test_translate2d() {
        let children = get_children();
        assert_eq!(
            Translate2D::build_with(|tb| {
                let _ = tb.v([8., -4.]);
            })
            .apply_to(&children)
            .to_code(),
            "translate([8, -4]) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
    }

    #[test]
    fn test_rotate2d() {
        let children = get_children();
        assert_eq!(
            Rotate2D::build_with(|rb| {
                let _ = rb.deg(45.);
            })
            .apply_to(&children)
            .to_code(),
            "rotate(45) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
        assert_eq!(
            Rotate2D::build_with(|rb| {
                let _ = rb.rad(PI / 4.);
            })
            .apply_to(&children)
            .to_code(),
            "rotate(45) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
    }

    #[test]
    fn test_mirror2d() {
        let children = get_children();
        assert_eq!(
            Mirror2D::build_with(|mb| {
                let _ = mb.v([1., -1.]);
            })
            .apply_to(&children)
            .to_code(),
            "mirror([1, -1]) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
    }

    #[test]
    fn test_scale2d() {
        let children = get_children();
        assert_eq!(
            Scale2D::build_with(|sb| {
                let _ = sb.v([3., 2.]);
            })
            .apply_to(&children)
            .to_code(),
            "scale([3, 2]) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
    }

    #[test]
    fn test_resize2d() {
        let children = get_children();
        let mut r1 = Resize2DBuilder::default();
        _ = r1.size([3., 2.]).apply_to(children);
        assert_eq!(
            r1.clone().build().unwrap().to_code(),
            "resize([3, 2]) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
        assert_eq!(
            r1.clone().auto(true).build().unwrap().to_code(),
            "resize([3, 2], auto = true) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
        assert_eq!(
            r1.auto([true, false]).build().unwrap().to_code(),
            "resize([3, 2], auto = [true, false]) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
    }

    #[test]
    fn test_multimatrix2d() {
        let children = get_children();
        let m = AffineMatrix2D::new(1., 2., 3., 4., 5., 6.);
        assert_eq!(
            MultMatrix2D::build_with(|mb| {
                let _ = mb.m(m);
            })
            .apply_to(&children)
            .to_code(),
            "multmatrix(m = [[1, 2, 0, 3], [4, 5, 0, 6], [0, 0, 1, 0]]) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
    }

    #[test]
    fn test_color2d() {
        let children = get_children();
        assert_eq!(
            Color2D::build_with(|cb| {
                let _ = cb.c(RGB::new(0.3, 0.5, 0.2));
            })
            .apply_to(&children)
            .to_code(),
            "color(c = [0.3, 0.5, 0.2]) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
        assert_eq!(
            // Color2DBuilder::default()
            //     .c(RGB::new(0.3, 0.5, 0.2))
            //     .a(1.0)
            //     .apply_to(children.clone())
            //     .build()
            //     .unwrap()
            //     .to_code(),
            Color2D::build_with(|cb| {
                let _ = cb.c(RGB::new(0.3, 0.5, 0.2)).a(1.0);
            })
            .apply_to(&children)
            .to_code(),
            "color(c = [0.3, 0.5, 0.2], a = 1) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
        assert_eq!(
            Color2D::build_with(|cb| {
                let _ = cb.c(RGBA::new(0.3, 0.5, 0.2, 1.0));
            })
            .apply_to(&children)
            .to_code(),
            "color(c = [0.3, 0.5, 0.2, 1]) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
        assert_eq!(
            Color2D::build_with(|cb| {
                let _ = cb.c("#C0FFEE".to_string());
            })
            .apply_to(&children)
            .to_code(),
            "color(\"#C0FFEE\") {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
    }

    #[test]
    fn test_offset() {
        let children = get_children();
        assert_eq!(
            Offset::build_with(|ob| {
                let _ = ob.r(1.);
            })
            .apply_to(&children)
            .to_code(),
            "offset(r = 1) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
        assert_eq!(
            Offset::build_with(|ob| {
                let _ = ob.delta(2.);
            })
            .apply_to(&children)
            .to_code(),
            "offset(delta = 2) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
        assert_eq!(
            Offset::build_with(|ob| {
                let _ = ob.r(1.).chamfer(true).fs(10);
            })
            .apply_to(&children)
            .to_code(),
            "offset(r = 1, chamfer = true, $fs = 10) {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
    }

    #[test]
    fn test_hull() {
        let children = get_children();
        assert_eq!(
            Hull2D::new().apply_to(&children).to_code(),
            "hull() {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
    }

    #[test]
    fn test_minkowski() {
        let children = get_children();
        assert_eq!(
            Minkowski2D::new().apply_to(&children).to_code(),
            "minkowski() {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
    }

    #[test]
    fn test_binary_op() {
        let children = get_children();
        assert_eq!(
            Union2D::new().apply_to(&children).to_code(),
            "union() {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
        assert_eq!(
            Difference2D::new().apply_to(&children).to_code(),
            "difference() {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
        assert_eq!(
            Intersection2D::new().apply_to(&children).to_code(),
            "intersection() {\n  square(size = 10);\n  circle(r = 5);\n}"
        );

        assert_eq!(
            (children[0].clone() + children[1].clone()).to_code(),
            "union() {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
        assert_eq!(
            (children[0].clone() - children[1].clone()).to_code(),
            "difference() {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
        assert_eq!(
            (children[0].clone() * children[1].clone()).to_code(),
            "intersection() {\n  square(size = 10);\n  circle(r = 5);\n}"
        );
    }

    #[test]
    fn test_projection() {
        let children = objects_3d![Sphere::build_with(|sb| {
            let _ = sb.r(10.);
        })];

        assert_eq!(
            Projection::build_with(|_| {}).apply_to(&children).to_code(),
            "projection() {\n  sphere(r = 10);\n}"
        );
        assert_eq!(
            Projection::build_with(|pb| {
                let _ = pb.cut(true);
            })
            .apply_to(&children)
            .to_code(),
            "projection(cut = true) {\n  sphere(r = 10);\n}"
        );
    }

    #[test]
    fn test_multilevel() {
        let objs = get_children();
        let scad = Rotate2D::build_with(|rb| {
            let _ = rb
                .deg(45.)
                .apply_to(objects_2d![Translate2D::build_with(|tb| {
                    let _ = tb.v([8., -4.]).apply_to(objs);
                })]);
        });
        assert_eq!(
            scad.to_code(),
            "rotate(45) {\n  translate([8, -4]) {\n    square(size = 10);\n    circle(r = 5);\n  }\n}"
        );
    }
}
