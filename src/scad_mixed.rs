//! Mixed objects in SCAD.
use std::rc::Rc;

use ambassador::Delegate;
use derive_more::derive::From;

use crate::{
    common::DimensionMarker,
    internal::{block_repr, modifier_repr},
    prelude::{Color, Difference, Hull, Intersection, Minkowski, Union},
    scad_display::{ambassador_impl_ScadDisplay, ScadDisplay},
};

/// A Mixed object in SCAD.
#[derive(Debug, Clone, From)]
pub enum ScadObjectMixed {
    /// A modifier mixed object.
    Modifier(ScadModifierMixed),
    /// A block of mixed objects.
    Block(ScadBlockMixed),
}

impl ScadDisplay for ScadObjectMixed {
    fn repr_scad(&self) -> String {
        match self {
            Self::Modifier(m) => m.repr_scad(),
            Self::Block(b) => b.repr_scad(),
        }
    }
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

impl ScadDisplay for ScadBlockMixed {
    fn repr_scad(&self) -> String {
        block_repr(&self.objects)
    }
}

impl ScadBlockMixed {
    /// Creats a new [`ScadBlockMixed`].
    pub fn new(objects: &[crate::common::ScadObjectImpl]) -> Self {
        Self {
            objects: objects.to_vec(),
        }
    }

    /// Creates a new [`ScadBlockMixed`] with the given objects if all objects are Mixed.
    ///
    /// # Arguments
    ///
    /// * `objects` - A slice of objects to be included in the block
    ///
    /// # Returns
    ///
    /// * `Some(ScadBlockMixed)` if all objects are Mixed objects
    /// * `None` if any object is not a Mixed object
    pub fn try_new(objects: &[crate::common::ScadObjectImpl]) -> Option<Self> {
        objects
            .iter()
            .all(|o| o.get_type() == DimensionMarker::ObjectMixed)
            .then_some(Self {
                objects: objects.to_vec(),
            })
    }
}

__impl_from_scad_for_collection_with_try_new!(
    crate::common::DMixed,
    ScadBlockMixed,
    ScadObjectMixed,
    crate::common::ScadObjectImpl::ObjectMixed,
    "Internal error: Vec<T> or &[T] for DMixed should always produce a valid ScadBlockMixed"
);

impl From<ScadObjectMixed> for crate::common::ScadObjectGeneric<crate::common::DMixed> {
    fn from(val: ScadObjectMixed) -> Self {
        let rc_o = Rc::new(val);
        let rc_impl = Rc::new(crate::common::ScadObjectImpl::ObjectMixed(rc_o));
        Self::from_impl(rc_impl)
    }
}

/// A modifier sentences for mixed objects in SCAD.
#[derive(Debug, Clone, Delegate, From)]
#[delegate(ScadDisplay)]
pub enum ScadModifierBodyMixed {
    /// `color()` in SCAD.
    Color(Color),
    /// `hull()` in SCAD.
    Hull(Hull),
    /// `minkowski()` in SCAD.
    Minkowski(Minkowski),
    /// `union()` in SCAD.
    Union(Union),
    /// `difference()` in SCAD.
    Difference(Difference),
    /// `intersection()` in SCAD.
    Intersection(Intersection),
}

impl ScadModifierBodyMixed {
    pub(crate) const fn get_children_type(&self) -> DimensionMarker {
        DimensionMarker::ObjectMixed
    }
}
