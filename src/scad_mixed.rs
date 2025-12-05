//! Mixed objects in SCAD.
use std::rc::Rc;

use ambassador::Delegate;
use derive_more::derive::From;

use crate::{
    internal::{block_repr, modifier_repr},
    scad_display::{ambassador_impl_ScadDisplay, ScadDisplay},
    scad_sentence::{Color, Difference, Hull, Intersection, Minkowski, Union},
    DimensionMarker,
};

/// A Mixed object in SCAD.
#[derive(Debug, Clone, Delegate, From)]
#[delegate(ScadDisplay)]
pub enum ScadObjectMixed {
    /// A modifier mixed object.
    Modifier(ScadModifierMixed),
    /// A block of mixed objects.
    Block(ScadBlockMixed),
}

/// A modifier for a mixed object in SCAD.
#[derive(Debug, Clone, From)]
pub struct ScadModifierMixed {
    /// The body of the modifier.
    pub body: ScadModifierBodyMixed,
    /// The child object to be modified.
    pub child: Rc<crate::common::ScadObjectImpl>,
}

impl ScadModifierMixed {
    /// Creates a new [`ScadModifierMixed`].
    pub const fn new(
        body: ScadModifierBodyMixed,
        child: Rc<crate::common::ScadObjectImpl>,
    ) -> Self {
        Self { body, child }
    }

    /// Creates a new [`ScadModifierMixed`] if the child's type matches the modifier's
    /// expected child type. Returns `Some(Self)` when the types match, otherwise
    /// `None`.
    pub fn try_new(
        body: ScadModifierBodyMixed,
        child: Rc<crate::common::ScadObjectImpl>,
    ) -> Option<Self> {
        (child.get_type() == body.get_children_type()).then_some(Self { body, child })
    }
}

impl ScadDisplay for ScadModifierMixed {
    fn repr_scad(&self) -> String {
        modifier_repr(&self.body, &self.child)
    }
}



/// A block of mixed objects in SCAD.
#[derive(Debug, Clone, From)]
pub struct ScadBlockMixed {
    /// The objects in the block.
    pub objects: Vec<crate::common::ScadObjectImpl>,
}

impl ScadBlockMixed {
    /// Creats a new [`ScadBlockMixed`].
    pub fn new(objects: &[crate::common::ScadObjectImpl]) -> Self {
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



/// A modifier sentences for mixed objects in SCAD.
#[derive(Debug, Clone, Delegate, From)]
#[delegate(ScadDisplay)]
pub enum ScadModifierBodyMixed {
    /// `color()` in SCAD.
    Color(Color),
    Hull(Hull),
    Minkowski(Minkowski),
    Union(Union),
    Difference(Difference),
    Intersection(Intersection),
}

impl ScadModifierBodyMixed {
    pub(crate) const fn get_children_type(&self) -> DimensionMarker {
        DimensionMarker::ObjectMixed
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_scad_mixed {
    ( $type:ident ) => {
        $crate::__impl_builder_sentence!($type);
    };
}
