//! 2D objects in SCAD.
use std::rc::Rc;

use ambassador::Delegate;
use derive_more::derive::From;

use crate::{
    ambassador_impl_ScadCommentDisplay,
    internal::{block_repr, modifier_repr, primitive_repr},
    scad_display::{ambassador_impl_ScadDisplay, ScadDisplay},
    scad_sentence::{
        Circle, Color, Difference, Hull, Import2D, Intersection, Minkowski, Mirror2D, MultMatrix2D,
        Offset, Polygon, Projection, Resize2D, Rotate2D, Scale2D, Square, Text, Translate2D, Union,
    },
    ScadCommentDisplay, ScadObject, ScadObjectDimensionType, ScadObjectTrait,
};

/// A 2D object in SCAD.
#[derive(Debug, Clone, Delegate, From)]
#[delegate(ScadDisplay)]
#[delegate(ScadCommentDisplay)]
pub enum ScadObject2D<T: ScadObjectTrait> {
    /// A primitive 2D object.
    Primitive(ScadPrimitive2D),
    /// A modifier 2D object.
    Modifier(ScadModifier2D<T>),
    /// A block of 2D objects.
    Block(ScadBlock2D<T>),
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

impl ScadCommentDisplay for ScadPrimitive2D {}

/// A modifier for a 2D object in SCAD.
#[derive(Debug, Clone, From)]
pub struct ScadModifier2D<T: ScadObjectTrait> {
    /// The body of the modifier.
    pub body: ScadModifierBody2D,
    /// The child object to be modified.
    pub child: Rc<T>,
}

impl<T: ScadObjectTrait> ScadModifier2D<T> {
    /// Creates a new [`ScadModifier2D`] if the child's type matches the modifier's expected child type.
    ///
    /// # Returns
    ///
    /// + `Some(Self)`: The new object generated.
    /// + `None`: If type of `child`is not matched with `body`
    pub fn try_new(body: ScadModifierBody2D, child: Rc<T>) -> Option<Self> {
        (child.get_type() == body.get_children_type()).then_some(Self { body, child })
    }
}

impl<T: ScadObjectTrait> ScadDisplay for ScadModifier2D<T> {
    fn repr_scad(&self) -> String {
        modifier_repr(&self.body, &*self.child)
    }
}

impl<T: ScadObjectTrait> ScadCommentDisplay for ScadModifier2D<T> {}

/// A block of 2D objects in SCAD.
#[derive(Debug, Clone, From)]
pub struct ScadBlock2D<T: ScadObjectTrait> {
    /// The objects in the block.
    pub objects: Vec<T>,
}

impl<T: ScadObjectTrait> ScadBlock2D<T> {
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
    pub fn try_new(objects: &[T]) -> Option<Self> {
        objects
            .iter()
            .all(|o| o.get_type() == ScadObjectDimensionType::Object2D)
            .then_some(Self {
                objects: objects.to_vec(),
            })
    }
}

impl<T: ScadObjectTrait> ScadDisplay for ScadBlock2D<T> {
    fn repr_scad(&self) -> String {
        block_repr(&self.objects)
    }
}

impl<T: ScadObjectTrait> ScadCommentDisplay for ScadBlock2D<T> {}

/// A primitive sentences for 2D objects in SCAD.
#[derive(Debug, Clone, Delegate, From)]
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
    pub(crate) const fn get_children_type(&self) -> ScadObjectDimensionType {
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
            | Self::Union(_) => ScadObjectDimensionType::Object2D,
            Self::Projection(_) => ScadObjectDimensionType::Object3D,
        }
    }
}

macro_rules! __impl_from_for_primitive2d {
    ( $type:ty ) => {
        impl From<$type> for ScadPrimitive2D {
            fn from(value: $type) -> Self {
                Self { body: value.into() }
            }
        }
    };
}
__impl_from_for_primitive2d!(Circle);
__impl_from_for_primitive2d!(Import2D);
__impl_from_for_primitive2d!(Polygon);
__impl_from_for_primitive2d!(Square);
__impl_from_for_primitive2d!(Text);
