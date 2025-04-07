use derive_builder::Builder;

use crate::{
    __generate_scad_options, internal::generate_sentence_repr, scad_display::ScadDisplay,
    value_type::ScadColor, Unit, __impl_builder_sentence,
};

/// Give an implementation of a modifier sentence without parameter.
macro_rules! __impl_operator {
    ( $type:ident, $name:expr_2021 ) => {
        #[doc = concat!($name, " modifier `", $name, "()` in SCAD.")]
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

impl ScadDisplay for Color {
    fn repr_scad(&self) -> String {
        generate_sentence_repr(
            "color",
            __generate_scad_options!(
                (self.c.name(), self.c.clone());
                ("a", self.a);
            ),
        )
    }
}

__impl_operator!(Hull, "hull");
__impl_operator!(Minkowski, "minkowski");
__impl_operator!(Union, "union");
__impl_operator!(Difference, "difference");
__impl_operator!(Intersection, "intersection");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
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
}
