//! 2D objects in SCAD.

mod object;
pub use object::*;

mod modifier;
pub use modifier::*;

use crate::ScadObject2D;

#[derive(Clone, Debug, derive_more::Deref)]
pub struct Objects2D(pub Vec<Box<dyn ScadObject2D>>);

impl From<Vec<Box<dyn ScadObject2D>>> for Objects2D {
    fn from(value: Vec<Box<dyn ScadObject2D>>) -> Self {
        Objects2D(value)
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_scad2d {
    ( $type:ty ) => {
        $crate::__impl_scad_box!($type);
        impl ScadObject2D for $type {}
        impl From<$type> for $crate::scad_2d::Objects2D {
            fn from(value: $type) -> Self {
                $crate::scad_2d::Objects2D(vec![Box::new(value)])
            }
        }

        $crate::__build_with_impl!($type);
    };
}
