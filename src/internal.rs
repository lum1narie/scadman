//! Internal module.
//! Macros in this module are not intended for public use,
//! despite being public.

use std::fmt::{Display, Formatter};

use crate::{
    common::{ScadObjectImpl, INDENT},
    scad_display::ScadDisplay,
};

/// Indent a string
///
/// # Arguments
///
/// + `s` - The string to be indented
/// + `indent` - The number of spaces to indent each line
///
/// # Returns
///
/// A new string with each line indented by the specified number of spaces
pub fn indent_str(s: &str, indent: usize) -> String {
    let mut lines: Vec<_> = s
        .lines()
        .map(|line| format!("{}{}", " ".repeat(indent), line))
        .collect();
    if s.ends_with('\n') {
        lines.push(String::new());
    }
    lines.join("\n")
}

/// Common code for primitive representation
///
/// # Arguments
///
/// + `body` - The SCAD sentence to be represented as a primitive
///
/// # Returns
///
/// A string representation of the primitive SCAD object, ending with a semicolon and newline
pub fn primitive_repr<T: ScadDisplay>(body: &T) -> String {
    format!("{};\n", body.repr_scad())
}

/// Represent a modifier with its child object in SCAD
///
/// - If the child's representation starts with '{{', the modifier is placed directly before the block
/// - Otherwise, the child is indented and placed on a new line after the modifier
///
/// # Arguments
///
/// * `body` - The modifier to be applied
/// * `child` - The child object to which the modifier is applied
///
/// # Returns
///
/// A string representation of the modifier and its child
pub fn modifier_repr<T: ScadDisplay>(body: &T, child: &ScadObjectImpl) -> String {
    let body_repr = body.repr_scad();
    let child_repr = child.to_code();
    if child_repr.chars().next().unwrap_or_default() == '{' {
        format!("{body_repr} {child_repr}")
    } else {
        let indented_child = indent_str(&child_repr, INDENT);
        format!("{body_repr}\n{indented_child}")
    }
}

/// Represent a block of SCAD objects
///
/// # Arguments
///
/// * `objects` - A slice of SCAD objects to be included in the block
///
/// # Returns
///
/// A string representation of the block, with objects indented and enclosed in curly braces
pub fn block_repr(objects: &[ScadObjectImpl]) -> String {
    let children_repr = objects
        .iter()
        .map(ScadObjectImpl::to_code)
        .collect::<String>();
    let indented_children = indent_str(&children_repr, INDENT);
    format!("{{\n{indented_children}}}\n")
}

/// Single option with a SCAD object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScadOption {
    /// Single value, no key
    Value(String),
    /// Key-value pair
    KeyValue((String, String)),
}

impl Display for ScadOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(v) => Display::fmt(&v, f),
            Self::KeyValue((k, v)) => {
                Display::fmt(&k, f)?;
                write!(f, " = ")?;
                Display::fmt(&v, f)
            }
        }
    }
}

impl ScadOption {
    /// Create a new option with a key-value pair.
    /// This function create an unnamed option if the `name` is empty string.
    ///
    /// # Arguments
    ///
    /// + `name` - name of the option, empty string for unnamed option
    /// + `value` - value of the option
    ///
    /// # Returns
    ///
    /// New [`ScadOption`]
    ///
    /// # Examples
    ///
    /// ```text
    /// use scadman::internal::ScadOption;
    /// assert_eq!(ScadOption::from_key_value("key", false),
    ///            ScadOption::KeyValue(("key".to_string(), "false".to_string())));
    /// assert_eq!(ScadOption::from_key_value("", true), ScadOption::Value("true".to_string()));
    /// ```
    pub fn from_key_value<T: ScadDisplay>(name: &str, value: T) -> Self {
        if name.is_empty() {
            Self::Value(value.repr_scad())
        } else {
            Self::KeyValue((name.to_string(), value.repr_scad()))
        }
    }

    /// Create a new option with a key-value pair.
    /// This function returns `None` if the `value` is `None`.
    /// This function create an unnamed option if the `name` is empty string.
    ///
    /// # Argument
    ///
    /// + `name` - name of the option, empty string for unnamed option
    /// + `value` - value of the option, `None` to fail.
    ///
    /// # Returns
    ///
    /// + [`ScadOption`] - if the `value` is [`Some<T>`]
    /// + [`None`] - if the `value` is [`None`]
    ///
    /// # Examples
    ///
    /// ```text
    /// use scadman::internal::ScadOption;
    /// assert_eq!(ScadOption::from_key_value_option("key", Some(false)),
    ///            Some(ScadOption::KeyValue(("key".to_string(), "false".to_string()))));
    /// assert_eq!(ScadOption::from_key_value_option::<bool>("key", None), None);
    /// ```
    pub fn from_key_value_option<T: ScadDisplay>(name: &str, value: Option<T>) -> Option<Self> {
        Some(Self::from_key_value(name, value?))
    }
}

/// Create a [`Vec<ScadOption>`] from key-value pairs with arbitrarily
/// [`impl ScadDisplay`] value.
///
/// This macro accepts the common calling forms used across the codebase:
/// - `__generate_scad_options!( (name, value) );`
/// - `__generate_scad_options!( (req1, v1); (req2, v2); );`
/// - `__generate_scad_options!( (req...), ; (opt1, v1); (opt2, v2); );`
/// - It also tolerates `;;` (empty optional list) and trailing semicolons.
///
/// To avoid parsing ambiguity between overlapping matcher arms, the macro
/// matches the key/name positions as `expr` and provides only a single
/// semicolon-separated pattern. This form matches callers such as:
///   ("size", self.size) ; ("center", self.center);
#[doc(hidden)]
#[macro_export]
macro_rules! __generate_scad_options {
    // Semicolon-separated form only: required ; optional
    // Use `expr` for key positions to allow literals, method calls and
    // identifiers uniformly. Optional pairs are placed into an explicit
    // `opt: ( ... )` group to avoid parsing ambiguity between the
    // required and optional sequences.
    ( $( ($name:expr_2021, $value:expr_2021) );* $(;)? $( opt: ( $( ($oname:expr_2021, $ovalue:expr_2021) );* $(;)? ) )? ) => {
        {
            let mut opts: Vec<$crate::internal::ScadOption> = Vec::new();
            $(
                opts.push($crate::internal::ScadOption::from_key_value(&$name, $value));
            )*
            $(
                $(
                    if let Some(opt) = $crate::internal::ScadOption::from_key_value_option(&$oname, $ovalue) {
                        opts.push(opt);
                    }
                )*
            )?
            opts
        }
    };

    // Fallback: no options
    () => {
        {
            Vec::<$crate::internal::ScadOption>::new()
        }
    };
}

/// Generate a SCAD code for a SCAD object with [`ScadOption`]s.
///
/// # Arguments
///
/// + `name` - name of the SCAD object
/// + `opts` - [`ScadOption`]s of the SCAD object
///
/// # Returns
///
/// SCAD code of the SCAD object
///
/// # Examples
///
/// ```text
/// use scadman::{scad::Unit, internal::ScadOption};
/// let opts = vec![
///    ScadOption::from_key_value("size", 1 as Unit),
///    ScadOption::from_key_value("center", true),
/// ];
/// assert_eq!(generate_body("square", opts), "square(size = 1, center = true)");
/// ```
pub fn generate_sentence_repr(name: &str, opts: Vec<ScadOption>) -> String {
    let reprs = opts.iter().map(ToString::to_string).collect::<Vec<_>>();
    format!("{}({})", name, reprs.join(", "))
}

/// Macro to implement chaining `apply()` for modifiers.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_modifier_chaining {
    ($mod_ty:ident) => {
        impl $mod_ty {
            /// Apply this modifier and return itself for chaining.
            pub const fn apply(self) -> Self {
                self
            }
        }
    };
}

/// Macro to implement panicking `to_code` and `From` for modifiers.
/// This ensures that attempting to treat a modifier as a complete `ScadObject`
/// without a child will cause a runtime panic, as expected by some tests.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_panicking_modifier_methods {
    ($mod_ty:ident, $dim_marker:ty) => {
        impl $mod_ty {
            pub fn to_code(&self) -> String {
                panic!(
                    "A modifier cannot be converted to SCAD code directly without a child object. Use .apply_to() or similar methods."
                );
            }
        }

        impl From<$mod_ty> for $crate::common::ScadObjectGeneric<$dim_marker> {
            fn from(_: $mod_ty) -> Self {
                panic!(
                    "A modifier cannot be converted to SCAD code directly without a child object. Use .apply_to() or similar methods."
                );
            }
        }
    };
}

/// Macro to implement `apply_to` for modifiers.
///
/// Parameters:
/// - $method_name: name of the method (e.g., `apply_to`)
/// - $mod_ty: modifier sentence type (e.g., `Translate3D`)
/// - $body_ty: concrete ScadModifierBody type path (e.g., `crate::scad_3d::ScadModifierBody3D`)
/// - $typed_output_obj: typed wrapper return type (e.g., `crate::common::ScadObject3D`)
/// - $mod_impl_ty: concrete ScadModifier type path (e.g., `crate::scad_3d::ScadModifier3D`)
/// - $obj_enum_ty: concrete ScadObject enum type (e.g., `crate::scad_3d::ScadObject3D`)
/// - $impl_variant: ScadObjectImpl variant constructor path
///   (e.g., `crate::common::ScadObjectImpl::Object3D` or `::Object2D`)
/// - $output_marker: dimension marker type for the output (e.g., `crate::common::D3` / `D2`)
/// - $child_marker: dimension marker type for the child (e.g., `crate::common::D3` / `D2`)
///
/// The generated method accepts `child: impl Into<ScadObjectGeneric<$child_marker>>`
/// so callers can pass either the typed wrapper, builder results that implement
/// `Scad`, or the legacy untyped runtime `ScadObject`.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_apply_to_modifier {
    (
        $method_name:ident,
        $mod_ty:ident,
        $body_ty:path,
        $typed_output_obj:path,
        $mod_impl_ty:path,
        $obj_enum_ty:path,
        $impl_variant:path,
        $output_marker:path,
        $child_marker:path
    ) => {
        impl $mod_ty {
            pub fn $method_name(
                self,
                child: impl Into<$crate::common::ScadObjectGeneric<$child_marker>>,
            ) -> $typed_output_obj {
                use std::rc::Rc;

                // Convert the input into the appropriate typed wrapper. This may perform a runtime
                // check and panic if dimensions mismatch (preserving existing behavior).
                let untyped_child_generic: $crate::common::ScadObjectGeneric<$child_marker> =
                    child.into();

                // Extract the inner runtime implementation Rc to pass to the modifier.
                let child_impl_rc: Rc<$crate::common::ScadObjectImpl> =
                    Rc::clone(&untyped_child_generic.inner);

                // Assert correct dimension at runtime as before.
                assert!(
                    child_impl_rc.get_type() == <$child_marker>::MARKER,
                    "Modifier requires: {:?}, but received: {:?}",
                    <$child_marker>::MARKER,
                    child_impl_rc.get_type()
                );

                // Convert this sentence into the modifier body expected by the target scad module.
                let body: $body_ty = self.into();

                // Try to construct the modifier; keep the existing panic message on failure.
                let m = <$mod_impl_ty>::try_new(body.clone(), child_impl_rc).unwrap_or_else(|| {
                    panic!(
                        "Modifier {:?} requires: {:?}",
                        body,
                        body.get_children_type()
                    )
                });

                // Wrap into the concrete object enum and then into the runtime implementation variant.
                let o = <$obj_enum_ty>::Modifier(m);
                let rc_impl = Rc::new($impl_variant(Rc::new(o)));

                // Convert rc_impl into the typed wrapper using from_impl and the provided marker.
                $crate::common::ScadObjectGeneric::<$output_marker>::from_impl(rc_impl)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_builder_primitive {
    ( $type:ident, $dim_marker:ty ) => {
        paste::paste! {
            impl $crate::common::ScadSentence for $type {
            }

            impl $crate::common::ScadBuildable for $type {
                type Target = $crate::common::ScadObjectGeneric<$dim_marker>;
                type Builder = [<$type Builder>];
            }

            impl $crate::common::ScadBuilder for [<$type Builder>] {
                type Sentence = $type;
                type Error = [<$type BuilderError>];
                fn build_scad(&self) -> Result<Self::Sentence, Self::Error> {
                    Self::build(&self)
                }
            }
        }
    };
}

/// implement [`ScadSentnece`] and [`ScadBuilder`] for certain type
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_builder_modifier {
    ( $type:ident ) => {
        paste::paste! {
            impl $crate::common::ScadSentence for $type {
            }

            impl $crate::common::ScadBuildable for $type {
                type Target = $type;
                type Builder = [<$type Builder>];
            }

            impl $crate::common::ScadBuilder for [<$type Builder>] {
                type Sentence = $type;
                type Error = [<$type BuilderError>];
                fn build_scad(&self) -> Result<Self::Sentence, Self::Error> {
                    Self::build(&self)
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_from_scad_for_collection_with_try_new {
    (
        $dim_marker:ty,         // e.g., D2
        $scad_block_ty:ty,      // e.g., ScadBlock2D
        $scad_object_enum_ty:ty,// e.g., ScadObject2D
        $scad_object_impl_variant:path, // e.g., ScadObjectImpl::Object2D
        $expect_msg:literal     // e.g., "Internal error: ..."
    ) => {
        impl<T> From<&[T]> for $crate::common::ScadObjectGeneric<$dim_marker>
        where
            T: Into<$crate::common::ScadObjectGeneric<$dim_marker>> + Clone,
        {
            fn from(item: &[T]) -> Self {
                let objects_rc_impl: Vec<std::rc::Rc<$crate::common::ScadObjectImpl>> =
                    item.iter().map(|item| item.clone().into().inner).collect();
                let objects_impl: Vec<$crate::common::ScadObjectImpl> = objects_rc_impl
                    .into_iter()
                    .map(|rc| Rc::unwrap_or_clone(rc))
                    .collect();

                let block = <$scad_block_ty>::try_new(&objects_impl).expect($expect_msg);

                let o = <$scad_object_enum_ty>::Block(block);
                let rc_impl = std::rc::Rc::new($scad_object_impl_variant(std::rc::Rc::new(o)));
                $crate::common::ScadObjectGeneric::from_impl(rc_impl)
            }
        }

        impl<T, const N: usize> From<[T; N]> for $crate::common::ScadObjectGeneric<$dim_marker>
        where
            T: Into<$crate::common::ScadObjectGeneric<$dim_marker>> + Clone,
        {
            fn from(item: [T; N]) -> Self {
                item[..].into()
            }
        }

        impl<T> From<Vec<T>> for $crate::common::ScadObjectGeneric<$dim_marker>
        where
            T: Into<$crate::common::ScadObjectGeneric<$dim_marker>> + Clone,
        {
            fn from(item: Vec<T>) -> Self {
                item.as_slice().into()
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{common::ScadObjectImpl, common::Unit};

    use super::*;

    #[test]
    fn test_indent_str() {
        assert_eq!(indent_str("hello\nworld!\n", 3), "   hello\n   world!\n");
        assert_eq!(
            indent_str("   hello\n  world!", 2),
            "     hello\n    world!"
        );
        assert_eq!(indent_str("\n\n\n", 1), " \n \n \n");
    }

    struct ScadDisplayMock(String);
    impl ScadDisplay for ScadDisplayMock {
        fn repr_scad(&self) -> String {
            self.0.clone()
        }
    }

    #[test]
    fn test_primitive_repr() {
        assert_eq!(
            primitive_repr(&ScadDisplayMock("prim()".to_string())),
            "prim();\n"
        );
    }

    #[derive(Clone)]
    struct ScadObjectMock(String);
    impl ScadObjectMock {
        fn inner(self) -> ScadObjectImpl {
            struct Inner(String);
            impl crate::common::ScadObjectReprMixed for Inner {
                fn to_code(&self) -> String {
                    self.0.clone()
                }
                fn as_any(&self) -> &dyn std::any::Any {
                    &self.0
                }
            }
            ScadObjectImpl::ObjectMixed(Rc::new(Inner(self.0)))
        }
    }

    #[test]
    fn test_modifier_repr() {
        assert_eq!(
            modifier_repr(
                &ScadDisplayMock("mod()".to_string()),
                &ScadObjectMock("prim();\n".to_string()).inner()
            ),
            "mod()\n  prim();\n"
        );
        assert_eq!(
            modifier_repr(
                &ScadDisplayMock("mod()".to_string()),
                &ScadObjectMock("mod2()\n  prim();\n".to_string()).inner()
            ),
            "mod()\n  mod2()\n    prim();\n"
        );
        assert_eq!(
            modifier_repr(
                &ScadDisplayMock("mod()".to_string()),
                &ScadObjectMock("{\n  prim1();\n  prim2();\n}\n".to_string()).inner()
            ),
            "mod() {\n  prim1();\n  prim2();\n}\n"
        );
        assert_eq!(
            modifier_repr(
                &ScadDisplayMock("mod()".to_string()),
                &ScadObjectMock("/* comment */\n{\n  prim1();\n  prim2();\n}\n".to_string())
                    .inner()
            ),
            "mod()\n  /* comment */\n  {\n    prim1();\n    prim2();\n  }\n"
        );
        assert_eq!(
            modifier_repr(
                &ScadDisplayMock("mod1()".to_string()),
                &ScadObjectMock(
                    "{\n  /* comment */\n  mod2(){\n    prim2();\n  }\n}\n".to_string()
                )
                .inner()
            ),
            "mod1() {\n  /* comment */\n  mod2(){\n    prim2();\n  }\n}\n"
        );
    }

    // TODO: test block_repr
    #[test]
    fn test_block_repr() {
        assert_eq!(
            block_repr(&[
                ScadObjectMock("prim1();\n".to_string()).inner(),
                ScadObjectMock("mod()\n  prim2();\n".to_string()).inner()
            ]),
            "{\n  prim1();\n  mod()\n    prim2();\n}\n"
        );
    }

    #[test]
    fn test_scad_option() {
        assert_eq!(
            ScadOption::from_key_value("key", false),
            ScadOption::KeyValue(("key".to_string(), "false".to_string()))
        );
        assert_eq!(
            ScadOption::from_key_value("", true),
            ScadOption::Value("true".to_string())
        );

        assert_eq!(
            ScadOption::from_key_value_option("key", Some(false)),
            Some(ScadOption::KeyValue((
                "key".to_string(),
                "false".to_string()
            )))
        );
        assert_eq!(ScadOption::from_key_value_option::<bool>("key", None), None);
    }

    #[test]
    fn test_generate_body() {
        let opts = vec![
            ScadOption::from_key_value("size", Unit::from(1)),
            ScadOption::from_key_value("center", true),
        ];
        assert_eq!(
            generate_sentence_repr("square", opts),
            "square(size = 1, center = true)"
        );
    }
}
