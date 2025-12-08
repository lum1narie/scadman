use derive_builder::Builder;

use crate::{
    common::{DimensionType as _, Unit},
    internal::generate_sentence_repr,
    scad_display::ScadDisplay,
    value_type::ScadColor,
};

/// Give an implementation of a modifier sentence without parameter.
macro_rules! __impl_operator {
    ( $type:ident, $name:expr_2021 ) => {
        #[allow(missing_debug_implementations)]
        #[allow(clippy::missing_const_for_fn)]
        #[allow(missing_copy_implementations)]
        #[derive(derive_builder::Builder, Debug, Clone, Copy)]
        #[doc = concat!($name, " modifier `", $name, "() in SCAD.")]
        pub struct $type {}

        $crate::__impl_builder_modifier!($type);

        impl $crate::scad_display::ScadDisplay for $type {
            fn repr_scad(&self) -> String {
                generate_sentence_repr($name, Vec::new())
            }
        }

        impl Default for $type {
            fn default() -> Self {
                Self::new()
            }
        }

        impl $type {
            /// generate new blank object
            pub const fn new() -> Self {
                Self {}
            }
        }
    };
}

/// Color modifier `color()` in SCAD.
/// This Rust type is regarded as Mixed object and only applys to mixed objects.
#[derive(Builder, Debug, Clone)]
pub struct Color {
    /// Color.
    ///
    /// See also [`ScadColor`].
    #[builder(setter(into))]
    pub c: ScadColor,
    /// Alpha value.
    /// `a` option in SCAD.
    ///
    /// Set when the `color` is NOT [`ScadColor::RGBA`].
    #[builder(setter(into, strip_option), default)]
    pub a: Option<Unit>,
}

__impl_builder_modifier!(Color);
__impl_modifier_chaining!(Color);

impl ScadDisplay for Color {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "color",
            __generate_scad_options!(
                (self.c.name(), self.c.clone()); opt: (("a", self.a);)
            ),
        )
    }
}

// Implement apply_to for mixed modifiers. These modifiers operate on mixed
// (untyped) objects, so use the mixed-scoped helper.
macro_rules! __impl_apply_mixed {
    ($mod_ty:ident) => {
        $crate::__impl_apply_to_modifier!(
            apply_to,
            $mod_ty,
            $crate::scad_mixed::ScadModifierBodyMixed,
            $crate::common::ScadObjectGeneric<$crate::common::DMixed>, // Typed output object
            $crate::scad_mixed::ScadModifierMixed,
            $crate::scad_mixed::ScadObjectMixed, // Enum definition remains
            $crate::common::ScadObjectImpl::ObjectMixed,
            $crate::common::DMixed, // output_marker
            $crate::common::DMixed  // child_marker
        );
    };
}

macro_rules! __impl_apply_2d {
    ($mod_ty:ident) => {
        $crate::__impl_apply_to_modifier!(
            apply_to_2d,
            $mod_ty,
            $crate::scad_2d::ScadModifierBody2D,
            $crate::common::ScadObjectGeneric<$crate::common::D2>, // Typed output object
            $crate::scad_2d::ScadModifier2D,
            $crate::scad_2d::ScadObject2D, // Enum definition remains
            $crate::common::ScadObjectImpl::Object2D,
            $crate::common::D2, // output_marker
            $crate::common::D2  // child_marker
        );
    };
}

macro_rules! __impl_apply_3d {
    ($mod_ty:ident) => {
        $crate::__impl_apply_to_modifier!(
            apply_to_3d,
            $mod_ty,
            $crate::scad_3d::ScadModifierBody3D,
            $crate::common::ScadObjectGeneric<$crate::common::D3>, // Typed output object
            $crate::scad_3d::ScadModifier3D,
            $crate::scad_3d::ScadObject3D, // Enum definition remains
            $crate::common::ScadObjectImpl::Object3D,
            $crate::common::D3, // output_marker
            $crate::common::D3  // child_marker
        );
    };
}

__impl_operator!(Hull, "hull");
__impl_modifier_chaining!(Hull);

__impl_operator!(Minkowski, "minkowski");
__impl_modifier_chaining!(Minkowski);

__impl_operator!(Union, "union");
__impl_modifier_chaining!(Union);

__impl_operator!(Difference, "difference");
__impl_modifier_chaining!(Difference);

__impl_operator!(Intersection, "intersection");
__impl_modifier_chaining!(Intersection);

__impl_apply_2d!(Color);
__impl_apply_3d!(Color);
__impl_apply_mixed!(Color);
__impl_apply_2d!(Hull);
__impl_apply_3d!(Hull);
__impl_apply_mixed!(Hull);
__impl_apply_2d!(Minkowski);
__impl_apply_3d!(Minkowski);
__impl_apply_mixed!(Minkowski);
__impl_apply_2d!(Union);
__impl_apply_3d!(Union);
__impl_apply_mixed!(Union);
__impl_apply_2d!(Difference);
__impl_apply_3d!(Difference);
__impl_apply_mixed!(Difference);
__impl_apply_2d!(Intersection);
__impl_apply_3d!(Intersection);
__impl_apply_mixed!(Intersection);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::ScadBuildable as _,
        scad_sentence::{Cube, Square},
        value_type::{RGB, RGBA},
    };

    #[test]
    fn test_colormixed() {
        assert_eq!(
            ColorBuilder::default()
                .c(RGB::new(0.3, 0.5, 0.2))
                .build()
                .unwrap()
                .repr_scad(),
            "color(c = [0.3, 0.5, 0.2])"
        );
        assert_eq!(
            ColorBuilder::default()
                .c(RGB::new(0.3, 0.5, 0.2))
                .a(1.0)
                .build()
                .unwrap()
                .repr_scad(),
            "color(c = [0.3, 0.5, 0.2], a = 1)"
        );
        assert_eq!(
            ColorBuilder::default()
                .c(RGBA::new(0.3, 0.5, 0.2, 1.0))
                .build()
                .unwrap()
                .repr_scad(),
            "color(c = [0.3, 0.5, 0.2, 1])"
        );
        assert_eq!(
            ColorBuilder::default()
                .c("#C0FFEE".to_string())
                .build()
                .unwrap()
                .repr_scad(),
            "color(\"#C0FFEE\")"
        );
    }

    #[test]
    fn test_hull() {
        assert_eq!(Hull::new().repr_scad(), "hull()");
    }

    #[test]
    fn test_minkowski() {
        assert_eq!(Minkowski::new().repr_scad(), "minkowski()");
    }

    #[test]
    fn test_binary_op() {
        assert_eq!(Union::new().repr_scad(), "union()");
        assert_eq!(Difference::new().repr_scad(), "difference()");
        assert_eq!(Intersection::new().repr_scad(), "intersection()");
    }

    #[test]
    fn test_apply_to_2d() {
        let square = Square::build_with(|b| {
            let _ = b.size(10.0);
        });
        let unioned = Union::new().apply_to_2d(square);
        assert_eq!(unioned.to_code(), "union()\n  square(size = 10);\n");
    }

    #[test]
    fn test_apply_to_3d() {
        let cube = Cube::build_with(|b| {
            let _ = b.size(10.0);
        });
        let hulled = Hull::new().apply_to_3d(cube);
        assert_eq!(hulled.to_code(), "hull()\n  cube(size = 10);\n");
    }
}
