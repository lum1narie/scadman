//! 2D objects in SCAD.
use std::rc::Rc;

use ambassador::Delegate;
use derive_more::derive::From;

use crate::{
    common::DimensionMarker,
    internal::{block_repr, modifier_repr, primitive_repr},
    prelude::{
        Circle, Color, Difference, Hull, Import2D, Intersection, Minkowski, Mirror2D, MultMatrix2D,
        Offset, Polygon, Projection, Resize2D, Rotate2D, Scale2D, Square, Text, Translate2D, Union,
    },
    scad_display::{ambassador_impl_ScadDisplay, ScadDisplay},
};

/// A 2D object in SCAD.
#[derive(Debug, Clone, From)]
pub enum ScadObject2D {
    /// A primitive 2D object.
    Primitive(ScadPrimitive2D),
    /// A modifier 2D object.
    Modifier(ScadModifier2D),
    /// A block of 2D objects.
    Block(ScadBlock2D),
}

impl ScadDisplay for ScadObject2D {
    fn repr_scad(&self) -> String {
        match self {
            Self::Primitive(p) => p.repr_scad(),
            Self::Modifier(m) => m.repr_scad(),
            Self::Block(b) => b.repr_scad(),
        }
    }
}

/// A primitive 2D object in SCAD.
#[derive(Debug, Clone, From)]
pub struct ScadPrimitive2D {
    /// The body of the primitive.
    pub body: ScadPrimitiveBody2D,
}

impl ScadPrimitive2D {
    /// Creates a new [`ScadPrimitive2D`].
    pub const fn new(body: ScadPrimitiveBody2D) -> Self {
        Self { body }
    }
}

impl ScadDisplay for ScadPrimitive2D {
    fn repr_scad(&self) -> String {
        primitive_repr(&self.body)
    }
}

impl From<ScadPrimitive2D> for crate::common::ScadObjectGeneric<crate::common::D2> {
    fn from(val: ScadPrimitive2D) -> Self {
        let o = ScadObject2D::Primitive(val);
        let rc_o = Rc::new(o);
        let rc_impl = Rc::new(crate::common::ScadObjectImpl::Object2D(rc_o));
        Self::from_impl(rc_impl)
    }
}

/// A modifier for a 2D object in SCAD.
#[derive(Debug, Clone, From)]
pub struct ScadModifier2D {
    /// The body of the modifier.
    pub body: ScadModifierBody2D,
    /// The child object to be modified.
    pub child: Rc<crate::common::ScadObjectImpl>,
}

impl ScadModifier2D {
    /// Creates a new [`ScadModifier2D`] if the child's type matches the modifier's expected child type.
    ///
    /// # Returns
    ///
    /// + `Some(Self)`: The new object generated.
    /// + `None`: If type of `child`is not matched with `body`
    pub fn try_new(
        body: ScadModifierBody2D,
        child: Rc<crate::common::ScadObjectImpl>,
    ) -> Option<Self> {
        (child.get_type() == body.get_children_type()).then_some(Self { body, child })
    }
}

impl ScadDisplay for ScadModifier2D {
    fn repr_scad(&self) -> String {
        modifier_repr(&self.body, &self.child)
    }
}

/// A block of 2D objects in SCAD.
#[derive(Debug, Clone, From)]
pub struct ScadBlock2D {
    /// The objects in the block.
    pub objects: Vec<crate::common::ScadObjectImpl>,
}

impl ScadDisplay for ScadBlock2D {
    fn repr_scad(&self) -> String {
        block_repr(&self.objects)
    }
}

impl ScadBlock2D {
    /// Creates a new [`ScadBlock2D`] with the given objects if all objects are 2D.
    ///
    /// # Arguments
    ///
    /// * `objects` - A slice of objects to be included in the block
    ///
    /// # Returns
    ///
    /// * `Some(ScadBlock2D)` if all objects are 2D objects
    /// * `None` if any object is not a 2D object
    pub fn try_new(objects: &[crate::common::ScadObjectImpl]) -> Option<Self> {
        objects
            .iter()
            .all(|o| o.get_type() == DimensionMarker::Object2D)
            .then_some(Self {
                objects: objects.to_vec(),
            })
    }
}

__impl_into_scad_for_collection_with_try_new!(
    crate::common::D2,
    ScadBlock2D,
    ScadObject2D,
    crate::common::ScadObjectImpl::Object2D,
    "Internal error: Vec<T> or &[T] for D2 should always produce a valid ScadBlock2D"
);

impl From<ScadObject2D> for crate::common::ScadObjectGeneric<crate::common::D2> {
    fn from(val: ScadObject2D) -> Self {
        let rc_o = Rc::new(val);
        let rc_impl = Rc::new(crate::common::ScadObjectImpl::Object2D(rc_o));
        Self::from_impl(rc_impl)
    }
}

/// A primitive sentences for 2D objects in SCAD.
#[derive(Debug, Clone, Delegate)] // Removed From
#[delegate(ScadDisplay)]
pub enum ScadPrimitiveBody2D {
    /// `circle()` in SCAD.
    Circle(Circle),
    /// `import()` in SCAD.
    Import(Import2D),
    /// `polygon()` in SCAD.
    Polygon(Polygon),
    /// `square()` in SCAD.
    Square(Square),
    /// `text()` in SCAD.
    Text(Text),
}

// Manual From implementations
impl From<Circle> for ScadPrimitiveBody2D {
    fn from(val: Circle) -> Self {
        Self::Circle(val)
    }
}

impl From<Import2D> for ScadPrimitiveBody2D {
    fn from(val: Import2D) -> Self {
        Self::Import(val)
    }
}

impl From<Polygon> for ScadPrimitiveBody2D {
    fn from(val: Polygon) -> Self {
        Self::Polygon(val)
    }
}

impl From<Square> for ScadPrimitiveBody2D {
    fn from(val: Square) -> Self {
        Self::Square(val)
    }
}

impl From<Text> for ScadPrimitiveBody2D {
    fn from(val: Text) -> Self {
        Self::Text(val)
    }
}

/// A modifier sentences for 2D objects in SCAD.
#[derive(Debug, Clone, Delegate, From)]
#[delegate(ScadDisplay)]
pub enum ScadModifierBody2D {
    /// `color()` in SCAD.
    Color(Color),
    /// `difference()` in SCAD.
    Difference(Difference),
    /// `hull()` in SCAD.
    Hull(Hull),
    /// `intersection()` in SCAD.
    Intersection(Intersection),
    /// `minkowski()` in SCAD.
    Minkowski(Minkowski),
    /// `mirror()` in SCAD.
    Mirror(Mirror2D),
    /// `multmatrix()` in SCAD.
    MultMatrix(MultMatrix2D),
    /// `offset()` in SCAD.
    Offset(Offset),
    /// `projection()` in SCAD.
    Projection(Projection),
    /// `resize()` in SCAD.
    Resize(Resize2D),
    /// `rotate()` in SCAD.
    Rotate(Rotate2D),
    /// `scale()` in SCAD.
    Scale(Scale2D),
    /// `translate()` in SCAD.
    Translate(Translate2D),
    /// `union()` in SCAD.
    Union(Union),
}

impl ScadModifierBody2D {
    /// Gets the expected child type for this modifier.
    pub(crate) const fn get_children_type(&self) -> DimensionMarker {
        match self {
            Self::Color(_)
            | Self::Difference(_)
            | Self::Hull(_)
            | Self::Intersection(_)
            | Self::Minkowski(_)
            | Self::Mirror(_)
            | Self::MultMatrix(_)
            | Self::Offset(_)
            | Self::Resize(_)
            | Self::Rotate(_)
            | Self::Scale(_)
            | Self::Translate(_)
            | Self::Union(_) => DimensionMarker::Object2D,
            Self::Projection(_) => DimensionMarker::Object3D,
        }
    }
}
