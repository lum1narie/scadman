use derive_builder::Builder;

use crate::{
    __generate_scad_options, __impl_scad_mixed, internal::generate_sentence_repr,
    scad_display::ScadDisplay, value_type::Color, ScadSentence as _, Unit,
};

/// Color modifier `color()` in SCAD.
/// This Rust type is regarded as Mixed object and only applys to mixed objects.
#[derive(Builder, Debug, Clone)]
pub struct ColorMixed {
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
}

__impl_scad_mixed!(ColorMixed);

impl ScadDisplay for ColorMixed {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_type::{RGB, RGBA};

    #[test]
    fn test_colormixed() {
        assert_eq!(
            ColorMixed::build_with(|cb| {
                let _ = cb.c(RGB::new(0.3, 0.5, 0.2));
            })
            .repr_scad(),
            "color(c = [0.3, 0.5, 0.2])"
        );
        assert_eq!(
            ColorMixed::build_with(|cb| {
                let _ = cb.c(RGB::new(0.3, 0.5, 0.2)).a(1.0);
            })
            .repr_scad(),
            "color(c = [0.3, 0.5, 0.2], a = 1)"
        );
        assert_eq!(
            ColorMixed::build_with(|cb| {
                let _ = cb.c(RGBA::new(0.3, 0.5, 0.2, 1.0));
            })
            .repr_scad(),
            "color(c = [0.3, 0.5, 0.2, 1])"
        );
        assert_eq!(
            ColorMixed::build_with(|cb| {
                let _ = cb.c("#C0FFEE".to_string());
            })
            .repr_scad(),
            "color(\"#C0FFEE\")"
        );
    }
}
