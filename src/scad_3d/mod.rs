//! 3D objects in SCAD.

mod object;
pub use object::*;

mod modifier;
pub use modifier::*;

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_scad3d {
    ( $type:ty ) => {
        $crate::__impl_scad_box!($type);
        impl ScadObject3D for $type {}
        impl From<$type> for Vec<Box<dyn ScadObject3D>> {
            fn from(value: $type) -> Self {
                vec![Box::new(value)]
            }
        }

        $crate::__build_with_impl!($type);
    };
}
