//! 3D objects in SCAD.

mod object;
pub use object::*;

mod modifier;
pub use modifier::*;

use crate::ScadObject3D;

#[derive(Clone, Debug, derive_more::Deref)]
pub struct Objects3D(pub Vec<Box<dyn ScadObject3D>>);

impl From<Vec<Box<dyn ScadObject3D>>> for Objects3D {
    fn from(value: Vec<Box<dyn ScadObject3D>>) -> Self {
        Objects3D(value)
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_scad3d {
    ( $type:ty ) => {
        $crate::__impl_scad_box!($type);
        impl ScadObject3D for $type {}
        impl From<$type> for $crate::scad_3d::Objects3D {
            fn from(value: $type) -> Self {
                $crate::scad_3d::Objects3D(vec![Box::new(value)])
            }
        }

        $crate::__build_with_impl!($type);
    };
}
