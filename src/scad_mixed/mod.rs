//! Mixed objects in SCAD.
use std::rc::Rc;

use ambassador::Delegate;
use derive_more::derive::From;

use crate::{
    ambassador_impl_ScadCommentDisplay,
    internal::{block_repr, modifier_repr},
    scad_display::{ambassador_impl_ScadDisplay, ScadDisplay},
    ScadCommentDisplay, ScadObject, ScadObjectDimensionType, ScadObjectTrait,
};

mod modifier;
pub use modifier::*;

/// A Mixed object in SCAD.
#[derive(Debug, Clone, Delegate, From)]
#[delegate(ScadDisplay)]
#[delegate(ScadCommentDisplay)]
pub enum ScadObjectMixed {
    Modifier(ScadModifierMixed<ScadObject>),
    Block(ScadBlockMixed),
}

#[derive(Debug, Clone, From)]
pub struct ScadModifierMixed<T: ScadObjectTrait> {
    pub body: ScadModifierBodyMixed,
    pub child: Rc<T>,
}

impl<T: ScadObjectTrait> ScadModifierMixed<T> {
    pub const fn new(body: ScadModifierBodyMixed, child: Rc<T>) -> Self {
        Self { body, child }
    }

    pub fn set_child(&mut self, child: Rc<T>) {
        self.child = child;
    }
}

impl<T: ScadObjectTrait> ScadDisplay for ScadModifierMixed<T> {
    fn repr_scad(&self) -> String {
        modifier_repr(&self.body, &*self.child)
    }
}

impl<T: ScadObjectTrait> ScadCommentDisplay for ScadModifierMixed<T> {}

#[derive(Debug, Clone, From)]
pub struct ScadBlockMixed {
    pub objects: Vec<ScadObject>,
}

impl ScadBlockMixed {
    pub fn new(objects: &[ScadObject]) -> Self {
        Self {
            objects: objects.to_vec(),
        }
    }
}

impl ScadDisplay for ScadBlockMixed {
    fn repr_scad(&self) -> String {
        block_repr(&self.objects)
    }
}

impl ScadCommentDisplay for ScadBlockMixed {}

#[derive(Debug, Clone, Delegate, From)]
#[delegate(ScadDisplay)]
pub enum ScadModifierBodyMixed {
    /// `color()` in SCAD.
    Color(ColorMixed),
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
