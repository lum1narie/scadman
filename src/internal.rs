use std::fmt::{Display, Formatter};

use crate::{scad_display::ScadDisplay, ScadObject, ScadObjectTrait, INDENT};

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
        lines.push("".to_string());
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
/// - If the child's representation starts with '{', the modifier is placed directly before the block
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
pub fn modifier_repr<T: ScadDisplay, U: ScadObjectTrait>(body: &T, child: &U) -> String {
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
pub fn block_repr<T: ScadObjectTrait>(objects: &[T]) -> String {
    let children_repr = objects
        .iter()
        .map(ScadObjectTrait::to_code)
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

/// Create a [`Vec<ScadOption>`] from key-value pairs with arbitrarily [`impl ScadDisplay`] value.
#[doc(hidden)]
#[macro_export]
macro_rules! __generate_scad_options {
    ( $(($name_req:expr_2021, $value_req:expr_2021)),*; $(;)? ) => {
        {
            vec![
                $($crate::internal::ScadOption::from_key_value($name_req, $value_req),)*
            ]
        }
    };
    ( $(($name_req:expr_2021, $value_req:expr_2021)),*; $(($name_opt:expr_2021, $value_opt:expr_2021)),+; ) => {
        {
            let mut opts: Vec<$crate::internal::ScadOption> = vec![
                $($crate::internal::ScadOption::from_key_value($name_req, $value_req),)*
            ];
            $(
                let maybe_opt = $crate::internal::ScadOption::from_key_value_option($name_opt, $value_opt);
                if let Some(opt) = maybe_opt {
                    opts.push(opt);
                }
            )+
                opts
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

/// implement [`ScadSentnece`] and [`ScadBuilder`] for certain type
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_builder_sentence {
    ( $type:ident ) => {
        paste::paste! {
            impl $crate::ScadSentence for $type {}

            impl $crate::ScadBuildable for $type {
                type Builder = [<$type Builder>];
            }

            impl $crate::ScadBuilder for [<$type Builder>] {
                type Target = $type;
                type Error = [<$type BuilderError>];
                fn build_scad(&self) -> Result<Self::Target, Self::Error> {
                    Self::build(&self)
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{common::Unit, ScadObjectDimensionType};

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

    struct ScadObjectMock(String);
    impl ScadObjectTrait for ScadObjectMock {
        fn to_code(&self) -> String {
            self.0.clone()
        }
        fn get_type(&self) -> ScadObjectDimensionType {
            ScadObjectDimensionType::ObjectMixed
        }
    }

    #[test]
    fn test_modifier_repr() {
        assert_eq!(
            modifier_repr(
                &ScadDisplayMock("mod()".to_string()),
                &ScadObjectMock("prim();\n".to_string())
            ),
            "mod()\n  prim();\n"
        );
        assert_eq!(
            modifier_repr(
                &ScadDisplayMock("mod()".to_string()),
                &ScadObjectMock("mod2()\n  prim();\n".to_string())
            ),
            "mod()\n  mod2()\n    prim();\n"
        );
        assert_eq!(
            modifier_repr(
                &ScadDisplayMock("mod()".to_string()),
                &ScadObjectMock("{\n  prim1();\n  prim2();\n}\n".to_string())
            ),
            "mod() {\n  prim1();\n  prim2();\n}\n"
        );
        assert_eq!(
            modifier_repr(
                &ScadDisplayMock("mod()".to_string()),
                &ScadObjectMock("/* comment */\n{\n  prim1();\n  prim2();\n}\n".to_string())
            ),
            "mod()\n  /* comment */\n  {\n    prim1();\n    prim2();\n  }\n"
        );
    }

    // TODO: test block_repr
    #[test]
    fn test_block_repr() {
        assert_eq!(
            block_repr(&[
                ScadObjectMock("prim1();\n".to_string()),
                ScadObjectMock("mod()\n  prim2();\n".to_string())
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
