use std::fmt::{Display, Formatter};

use super::ScadDisplay;

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_scad_box {
    ( $type:ty ) => {
        impl From<$type> for Vec<Box<dyn ScadObject>> {
            fn from(value: $type) -> Self {
                vec![Box::new(value) as Box<dyn ScadObject>]
            }
        }
    };
}

pub(crate) enum ScadOption {
    Value(String),
    KeyValue((String, String)),
}

impl Display for ScadOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScadOption::Value(v) => std::fmt::Display::fmt(&v, f),
            ScadOption::KeyValue((k, v)) => {
                std::fmt::Display::fmt(&k, f)?;
                write!(f, " = ")?;
                std::fmt::Display::fmt(&v, f)
            }
        }
    }
}

impl ScadOption {
    pub fn from_key_value<T: ScadDisplay>(name: &str, value: T) -> Self {
        if name.is_empty() {
            Self::Value(value.repr_scad())
        } else {
            Self::KeyValue((name.to_string(), value.repr_scad()))
        }
    }

    pub fn from_key_value_option<T: ScadDisplay>(name: &str, value: Option<T>) -> Option<Self> {
        Some(Self::from_key_value(name, value?))
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __generate_scad_options {
    ( $(($name_req:expr, $value_req:expr)),*; $(;)? ) => {
        {
            vec![
                $($crate::scad::ScadOption::from_key_value($name_req, $value_req),)*
            ]
        }
    };
    ( $(($name_req:expr, $value_req:expr)),*; $(($name_opt:expr, $value_opt:expr)),+; ) => {
        {
            let mut opts: Vec<$crate::scad::ScadOption> = vec![
                $($crate::scad::ScadOption::from_key_value($name_req, $value_req),)*
            ];
            $(
                let maybe_opt = $crate::scad::ScadOption::from_key_value_option($name_opt, $value_opt);
                if let Some(opt) = maybe_opt {
                    opts.push(opt);
                }
            )*
                opts
        }
    };
}

pub(crate) fn generate_body(name: &str, opts: Vec<ScadOption>) -> String {
    let reprs = opts.iter().map(|o| o.to_string()).collect::<Vec<_>>();
    format!("{}({})", name, reprs.join(", "))
}

#[doc(hidden)]
#[macro_export]
macro_rules! __get_children_impl {
    () => {
        fn get_children(&self) -> Option<Vec<String>> {
            Some(self.children.iter().map(|c| c.to_code()).collect())
        }
    };
}
