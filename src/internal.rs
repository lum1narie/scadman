use std::fmt::{Display, Formatter};

use crate::scad_display::ScadDisplay;

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_scad_box {
    ( $type:ty ) => {
        impl From<$type> for Vec<Box<dyn ScadObject>> {
            fn from(value: $type) -> Self {
                vec![Box::new(value)]
            }
        }
    };
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
pub fn generate_body(name: &str, opts: Vec<ScadOption>) -> String {
    let reprs = opts.iter().map(ToString::to_string).collect::<Vec<_>>();
    format!("{}({})", name, reprs.join(", "))
}

/// Give a default implementation of [`ScadObject::get_children`].
///
/// This macro is for the [`impl ScadObject`] having `self.childern` as `Vec<Box<dyn ScadObject>>`.
#[doc(hidden)]
#[macro_export]
macro_rules! __get_children_impl {
    () => {
        fn get_children(&self) -> Option<Vec<String>> {
            Some(self.children.iter().map(|c| c.to_code()).collect())
        }
    };
}

/// TODO: doc
#[doc(hidden)]
#[macro_export]
macro_rules! __build_with_impl {
    ( $type:tt ) => {
        paste::paste! {
            impl $type {
                pub fn build_with(
                    builder_config: impl FnOnce(&mut [<$type Builder>])
                ) -> Self {
                    let mut builder = [<$type Builder>]::default();
                    let _ = builder_config(&mut builder);
                    builder.build().expect("required fields are not set")
                    }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::common::Unit;

    use super::*;

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
            generate_body("square", opts),
            "square(size = 1, center = true)"
        );
    }
}
