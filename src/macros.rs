//! A collection of helper macros exported for extenal use.

/// A macro for creating a [`Vec<ScadObject2D>`] from each variant.
#[macro_export]
macro_rules! objects_2d {
    [ $($scad:expr_2021),* $(,)? ] => {
        {
            vec![$($crate::scad_2d::ScadObject2D::from($scad)),*]
        }
    };
}

/// A macro for creating a [`Vec<ScadObject3D>`] from each variant.
#[macro_export]
macro_rules! objects_3d {
    [ $($scad:expr_2021),* $(,)? ] => {
        {
            vec![$($crate::scad_3d::ScadObject3D::from($scad)),*]
        }
    };
}
