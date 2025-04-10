//! Mixed objects in SCAD.
use std::rc::Rc;

use ambassador::Delegate;
use derive_more::derive::From;

use crate::{
    ambassador_impl_ScadCommentDisplay,
    internal::{block_repr, modifier_repr},
    scad_display::{ambassador_impl_ScadDisplay, ScadDisplay},
    scad_sentence::Color,
    ScadCommentDisplay, ScadObjectDimensionType, ScadObjectTrait,
};

/// A Mixed object in SCAD.
#[derive(Debug, Clone, Delegate, From)]
#[delegate(ScadDisplay)]
#[delegate(ScadCommentDisplay)]
pub enum ScadObjectMixed<T: ScadObjectTrait> {
    /// A modifier mixed object.
    Modifier(ScadModifierMixed<T>),
    /// A block of mixed objects.
    Block(ScadBlockMixed<T>),
}

/// A modifier for a mixed object in SCAD.
#[derive(Debug, Clone, From)]
pub struct ScadModifierMixed<T: ScadObjectTrait> {
    /// The body of the modifier.
    pub body: ScadModifierBodyMixed,
    /// The child object to be modified.
    pub child: Rc<T>,
}

impl<T: ScadObjectTrait> ScadModifierMixed<T> {
    /// Creates a new [`ScadModifierMixed`].
    pub const fn new(body: ScadModifierBodyMixed, child: Rc<T>) -> Self {
        Self { body, child }
    }
}

impl<T: ScadObjectTrait> ScadDisplay for ScadModifierMixed<T> {
    fn repr_scad(&self) -> String {
        modifier_repr(&self.body, &*self.child)
    }
}

impl<T: ScadObjectTrait> ScadCommentDisplay for ScadModifierMixed<T> {}

/// A block of mixed objects in SCAD.
#[derive(Debug, Clone, From)]
pub struct ScadBlockMixed<T: ScadObjectTrait> {
    /// The objects in the block.
    pub objects: Vec<T>,
}

impl<T: ScadObjectTrait> ScadBlockMixed<T> {
    /// Creats a new [`ScadBlockMixed`].
    pub fn new(objects: &[T]) -> Self {
        Self {
            objects: objects.to_vec(),
        }
    }
}

impl<T: ScadObjectTrait> ScadDisplay for ScadBlockMixed<T> {
    fn repr_scad(&self) -> String {
        block_repr(&self.objects)
    }
}

impl<T: ScadObjectTrait> ScadCommentDisplay for ScadBlockMixed<T> {}

/// A modifier sentences for mixed objects in SCAD.
#[derive(Debug, Clone, Delegate, From)]
#[delegate(ScadDisplay)]
pub enum ScadModifierBodyMixed {
    /// `color()` in SCAD.
    Color(Color),
}

impl ScadModifierBodyMixed {
    pub(crate) const fn get_children_type(&self) -> ScadObjectDimensionType {
        ScadObjectDimensionType::ObjectMixed
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_scad_mixed {
    ( $type:ident ) => {
        $crate::__impl_builder_sentence!($type);
    };
}
