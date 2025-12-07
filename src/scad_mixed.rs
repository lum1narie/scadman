//! Mixed objects in SCAD.
use std::rc::Rc;

use ambassador::Delegate;
use derive_more::derive::From;

use crate::{
    common::{DMixed, DimensionMarker},
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
            ScadObjectMixed::Modifier(m) => m.repr_scad(),
            ScadObjectMixed::Block(b) => b.repr_scad(),
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

impl From<ScadObjectMixed> for crate::common::ScadObjectGeneric<crate::common::DMixed> {
    fn from(val: ScadObjectMixed) -> crate::common::ScadObjectGeneric<crate::common::DMixed> {
        let rc_o = Rc::new(val);
        let rc_impl = Rc::new(crate::common::ScadObjectImpl::ObjectMixed(rc_o));
        crate::common::ScadObjectGeneric::from_impl(rc_impl)
    }
}
