use derive_builder::Builder;
use std::rc::Rc;

use crate::{
    __generate_scad_options, __impl_builder_sentence, __impl_modifier_chaining,
    common::{DimensionType as _, IntoScad, ScadObjectGeneric, ScadObjectImpl},
    internal::generate_sentence_repr,
    scad_2d::{ScadBlock2D, ScadModifier2D, ScadModifierBody2D, ScadObject2D},
    scad_3d::{ScadBlock3D, ScadModifier3D, ScadModifierBody3D, ScadObject3D},
    scad_display::ScadDisplay,
    scad_mixed::{ScadBlockMixed, ScadModifierBodyMixed, ScadModifierMixed, ScadObjectMixed},
    value_type::ScadColor,
    Unit,
};

/// Give an implementation of a modifier sentence without parameter.
macro_rules! __impl_operator {
    ( $type:ident, $name:expr_2021 ) => {
        // #[doc = concat!($name, " modifier `", $name, "()" in SCAD.)]
        #[allow(missing_debug_implementations)]
        #[allow(clippy::missing_const_for_fn)]
        #[allow(missing_copy_implementations)]
        #[derive(derive_builder::Builder, Debug, Clone, Copy)]
        pub struct $type {}

        $crate::__impl_builder_sentence!($type);

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

__impl_builder_sentence!(Color);
__impl_modifier_chaining!(Color);

impl IntoScad<crate::common::D2> for Color {
    fn scad(self) -> ScadObjectGeneric<crate::common::D2> {
        panic!("A modifier cannot be converted to SCAD code directly without a child object. Use .apply_to() or similar methods.")
    }
}

impl IntoScad<crate::common::D3> for Color {
    fn scad(self) -> ScadObjectGeneric<crate::common::D3> {
        panic!("A modifier cannot be converted to SCAD code directly without a child object. Use .apply_to() or similar methods.")
    }
}

impl IntoScad<crate::common::DMixed> for Color {
    fn scad(self) -> ScadObjectGeneric<crate::common::DMixed> {
        panic!("A modifier cannot be converted to SCAD code directly without a child object. Use .apply_to() or similar methods.")
    }
}

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
            $crate::common::ScadObjectMixed,
            $crate::scad_mixed::ScadModifierMixed,
            $crate::scad_mixed::ScadObjectMixed,
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
            $crate::common::ScadObject2D,
            $crate::scad_2d::ScadModifier2D,
            $crate::scad_2d::ScadObject2D,
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
            $crate::common::ScadObject3D,
            $crate::scad_3d::ScadModifier3D,
            $crate::scad_3d::ScadObject3D,
            $crate::common::ScadObjectImpl::Object3D,
            $crate::common::D3, // output_marker
            $crate::common::D3  // child_marker
        );
    };
}

macro_rules! __impl_apply_universal {
    ($mod_ty:ident) => {
        __impl_apply_2d!($mod_ty);
        __impl_apply_3d!($mod_ty);
        __impl_apply_mixed!($mod_ty);
    };
}

__impl_operator!(Hull, "hull");
__impl_modifier_chaining!(Hull);

impl IntoScad<crate::common::D2> for Hull {
    fn scad(self) -> ScadObjectGeneric<crate::common::D2> {
        panic!("A modifier cannot be converted to SCAD code directly without a child object. Use .apply_to() or similar methods.")
    }
}

impl IntoScad<crate::common::D3> for Hull {
    fn scad(self) -> ScadObjectGeneric<crate::common::D3> {
        panic!("A modifier cannot be converted to SCAD code directly without a child object. Use .apply_to() or similar methods.")
    }
}

impl IntoScad<crate::common::DMixed> for Hull {
    fn scad(self) -> ScadObjectGeneric<crate::common::DMixed> {
        panic!("A modifier cannot be converted to SCAD code directly without a child object. Use .apply_to() or similar methods.")
    }
}
__impl_operator!(Minkowski, "minkowski");
__impl_modifier_chaining!(Minkowski);

impl IntoScad<crate::common::D2> for Minkowski {
    fn scad(self) -> ScadObjectGeneric<crate::common::D2> {
        panic!("A modifier cannot be converted to SCAD code directly without a child object. Use .apply_to() or similar methods.")
    }
}

impl IntoScad<crate::common::D3> for Minkowski {
    fn scad(self) -> ScadObjectGeneric<crate::common::D3> {
        panic!("A modifier cannot be converted to SCAD code directly without a child object. Use .apply_to() or similar methods.")
    }
}

impl IntoScad<crate::common::DMixed> for Minkowski {
    fn scad(self) -> ScadObjectGeneric<crate::common::DMixed> {
        panic!("A modifier cannot be converted to SCAD code directly without a child object. Use .apply_to() or similar methods.")
    }
}
__impl_operator!(Union, "union");
__impl_modifier_chaining!(Union);

impl IntoScad<crate::common::D2> for Union {
    fn scad(self) -> ScadObjectGeneric<crate::common::D2> {
        panic!("A modifier cannot be converted to SCAD code directly without a child object. Use .apply_to() or similar methods.")
    }
}

impl IntoScad<crate::common::D3> for Union {
    fn scad(self) -> ScadObjectGeneric<crate::common::D3> {
        panic!("A modifier cannot be converted to SCAD code directly without a child object. Use .apply_to() or similar methods.")
    }
}

impl IntoScad<crate::common::DMixed> for Union {
    fn scad(self) -> ScadObjectGeneric<crate::common::DMixed> {
        panic!("A modifier cannot be converted to SCAD code directly without a child object. Use .apply_to() or similar methods.")
    }
}
__impl_operator!(Difference, "difference");
__impl_modifier_chaining!(Difference);

impl IntoScad<crate::common::D2> for Difference {
    fn scad(self) -> ScadObjectGeneric<crate::common::D2> {
        panic!("A modifier cannot be converted to SCAD code directly without a child object. Use .apply_to() or similar methods.")
    }
}

impl IntoScad<crate::common::D3> for Difference {
    fn scad(self) -> ScadObjectGeneric<crate::common::D3> {
        panic!("A modifier cannot be converted to SCAD code directly without a child object. Use .apply_to() or similar methods.")
    }
}

impl IntoScad<crate::common::DMixed> for Difference {
    fn scad(self) -> ScadObjectGeneric<crate::common::DMixed> {
        panic!("A modifier cannot be converted to SCAD code directly without a child object. Use .apply_to() or similar methods.")
    }
}
__impl_operator!(Intersection, "intersection");
__impl_modifier_chaining!(Intersection);

impl IntoScad<crate::common::D2> for Intersection {
    fn scad(self) -> ScadObjectGeneric<crate::common::D2> {
        panic!("A modifier cannot be converted to SCAD code directly without a child object. Use .apply_to() or similar methods.")
    }
}

impl IntoScad<crate::common::D3> for Intersection {
    fn scad(self) -> ScadObjectGeneric<crate::common::D3> {
        panic!("A modifier cannot be converted to SCAD code directly without a child object. Use .apply_to() or similar methods.")
    }
}

impl IntoScad<crate::common::DMixed> for Intersection {
    fn scad(self) -> ScadObjectGeneric<crate::common::DMixed> {
        panic!("A modifier cannot be converted to SCAD code directly without a child object. Use .apply_to() or similar methods.")
    }
}

__impl_apply_universal!(Color);
__impl_apply_universal!(Hull);
__impl_apply_universal!(Minkowski);
__impl_apply_universal!(Union);
__impl_apply_universal!(Difference);
__impl_apply_universal!(Intersection);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        prelude::{primitive_2d, primitive_3d},
        scad_sentence::{Cube, Square},
        value_type::{RGB, RGBA},
        ScadBuildable as _,
    };

    #[test]
    fn test_colormixed() {
        assert_eq!(
            Color::build_with(|cb| {
                let _ = cb.c(RGB::new(0.3, 0.5, 0.2));
            })
            .repr_scad(),
            "color(c = [0.3, 0.5, 0.2])"
        );
        assert_eq!(
            Color::build_with(|cb| {
                let _ = cb.c(RGB::new(0.3, 0.5, 0.2)).a(1.0);
            })
            .repr_scad(),
            "color(c = [0.3, 0.5, 0.2], a = 1)"
        );
        assert_eq!(
            Color::build_with(|cb| {
                let _ = cb.c(RGBA::new(0.3, 0.5, 0.2, 1.0));
            })
            .repr_scad(),
            "color(c = [0.3, 0.5, 0.2, 1])"
        );
        assert_eq!(
            Color::build_with(|cb| {
                let _ = cb.c("#C0FFEE".to_string());
            })
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
        let square = primitive_2d(Square::build_with(|b| {
            let _ = b.size(10.0);
        }));
        let unioned = Union::new().apply_to_2d(square);
        assert_eq!(unioned.to_code(), "union()\n  square(size = 10);\n");
    }

    #[test]
    fn test_apply_to_3d() {
        let cube = primitive_3d(Cube::build_with(|b| {
            let _ = b.size(10.0);
        }));
        let hulled = Hull::new().apply_to_3d(cube);
        assert_eq!(hulled.to_code(), "hull()\n  cube(size = 10);\n");
    }
}
