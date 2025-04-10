//! 3D objects in SCAD.
use std::rc::Rc;

use ambassador::Delegate;
use derive_more::derive::From;

use crate::{
    ambassador_impl_ScadCommentDisplay,
    internal::{block_repr, modifier_repr, primitive_repr},
    scad_display::{ambassador_impl_ScadDisplay, ScadDisplay},
    scad_sentence::{
        Color, Cube, Cylinder, Difference, Hull, Import3D, Intersection, LinearExtrude, Minkowski,
        Mirror3D, MultMatrix3D, Polyhedron, Resize3D, Rotate3D, RotateExtrude, Scale3D, Sphere,
        Surface, Translate3D, Union,
    },
    ScadCommentDisplay, ScadObject, ScadObjectDimensionType, ScadObjectTrait,
};

/// A 3D object in SCAD.
#[derive(Debug, Clone, Delegate, From)]
#[delegate(ScadDisplay)]
#[delegate(ScadCommentDisplay)]
pub enum ScadObject3D<T: ScadObjectTrait> {
    /// A primitive 3D object.
    Primitive(ScadPrimitive3D),
    /// A modifier 3D object.
    Modifier(ScadModifier3D<T>),
    /// A block of 3D objects.
    Block(ScadBlock3D<T>),
}

/// A primitive 3D object in SCAD.
#[derive(Debug, Clone, From)]
pub struct ScadPrimitive3D {
    /// The body of the primitive.
    pub body: ScadPrimitiveBody3D,
}

impl ScadPrimitive3D {
    /// Creates a new [`ScadPrimitive3D`].
    pub const fn new(body: ScadPrimitiveBody3D) -> Self {
        Self { body }
    }
}

impl ScadDisplay for ScadPrimitive3D {
    fn repr_scad(&self) -> String {
        primitive_repr(&self.body)
    }
}

impl ScadCommentDisplay for ScadPrimitive3D {}

/// A modifier for a 3D object in SCAD.
#[derive(Debug, Clone, From)]
pub struct ScadModifier3D<T: ScadObjectTrait> {
    /// The body of the modifier.
    pub body: ScadModifierBody3D,
    /// The child object to be modified.
    pub child: Rc<T>,
}

impl<T: ScadObjectTrait> ScadModifier3D<T> {
    /// Creates a new [`ScadModifier3D`] if the child's type matches the modifier's expected child type.
    ///
    /// # Returns
    ///
    /// + `Some(Self)`: The new object generated.
    /// + `None`: If type of `child`is not matched with `body`
    pub fn try_new(body: ScadModifierBody3D, child: Rc<T>) -> Option<Self> {
        (child.get_type() == body.get_children_type()).then_some( Self { body, child })
    }
}

impl<T: ScadObjectTrait> ScadDisplay for ScadModifier3D<T> {
    fn repr_scad(&self) -> String {
        modifier_repr(&self.body, &*self.child)
    }
}

impl<T: ScadObjectTrait> ScadCommentDisplay for ScadModifier3D<T> {}

/// A block of 3D objects in SCAD.
#[derive(Debug, Clone, From)]
pub struct ScadBlock3D<T: ScadObjectTrait> {
    /// The objects in the block.
    pub objects: Vec<T>,
}

impl<T: ScadObjectTrait> ScadBlock3D<T> {
    /// Creates a new [`ScadBlock3D`] with the given objects if all objects are 3D.
    ///
    /// # Arguments
    ///
    /// * `objects` - A slice of objects to be included in the block
    ///
    /// # Returns
    ///
    /// * `Some(ScadBlock3D)` if all objects are 3D objects
    /// * `None` if any object is not a 3D object
    pub fn try_new(objects: &[T]) -> Option<Self> {
        objects
            .iter()
            .all(|o| o.get_type() == ScadObjectDimensionType::Object3D)
            .then_some(Self {
                objects: objects.to_vec(),
            })
    }
}

impl<T: ScadObjectTrait> ScadDisplay for ScadBlock3D<T> {
    fn repr_scad(&self) -> String {
        block_repr(&self.objects)
    }
}

impl<T: ScadObjectTrait> ScadCommentDisplay for ScadBlock3D<T> {}

/// A primitive sentences for 3D objects in SCAD.
#[derive(Debug, Clone, Delegate, From)]
#[delegate(ScadDisplay)]
pub enum ScadPrimitiveBody3D {
    /// `cube()` in SCAD.
    Cube(Cube),
    /// `cylinder()` in SCAD.
    Cylinder(Cylinder),
    /// `import()` in SCAD.
    Import(Import3D),
    /// `polyhedron()` in SCAD.
    Polyhedron(Polyhedron),
    /// `sphere()` in SCAD.
    Sphere(Sphere),
    /// `surface()` in SCAD.
    Surface(Surface),
}

/// A modifier sentences for 3D objects in SCAD.
#[derive(Debug, Clone, Delegate, From)]
#[delegate(ScadDisplay)]
pub enum ScadModifierBody3D {
    /// `color()` in SCAD.
    Color(Color),
    /// `difference()` in SCAD.
    Difference(Difference),
    /// `hull()` in SCAD.
    Hull(Hull),
    /// `intersection()` in SCAD.
    Intersection(Intersection),
    /// `linear_extrude()` in SCAD.
    LinearExtrude(LinearExtrude),
    /// `minkowski()` in SCAD.
    Minkowski(Minkowski),
    /// `mirror()` in SCAD.
    Mirror(Mirror3D),
    /// `multmatrix()` in SCAD.
    MultMatrix(MultMatrix3D),
    /// `resize()` in SCAD.
    Resize(Resize3D),
    /// `rotate()` in SCAD.
    Rotate(Rotate3D),
    /// `rotate_extrude()` in SCAD.
    RotateExtrude(RotateExtrude),
    /// `scale()` in SCAD.
    Scale(Scale3D),
    /// `translate()` in SCAD.
    Translate(Translate3D),
    /// `union()` in SCAD.
    Union(Union),
}

impl ScadModifierBody3D {
    pub(crate) const fn get_children_type(&self) -> ScadObjectDimensionType {
        match self {
            Self::Color(_)
            | Self::Difference(_)
            | Self::Hull(_)
            | Self::Intersection(_)
            | Self::Minkowski(_)
            | Self::Mirror(_)
            | Self::MultMatrix(_)
            | Self::Resize(_)
            | Self::Rotate(_)
            | Self::Scale(_)
            | Self::Translate(_)
            | Self::Union(_) => ScadObjectDimensionType::Object3D,
            Self::LinearExtrude(_) | Self::RotateExtrude(_) => ScadObjectDimensionType::Object2D,
        }
    }
}

macro_rules! __impl_from_for_primitive3d {
    ( $type:ty ) => {
        impl From<$type> for ScadPrimitive3D {
            fn from(value: $type) -> Self {
                Self { body: value.into() }
            }
        }
    };
}

__impl_from_for_primitive3d!(Cube);
__impl_from_for_primitive3d!(Cylinder);
__impl_from_for_primitive3d!(Import3D);
__impl_from_for_primitive3d!(Polyhedron);
__impl_from_for_primitive3d!(Sphere);
__impl_from_for_primitive3d!(Surface);
