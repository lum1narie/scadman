//! 3D objects in SCAD.
use std::rc::Rc;

use ambassador::Delegate;
use derive_more::derive::From;

use crate::{
    common::{DimensionMarker, D3},
    internal::{block_repr, modifier_repr, primitive_repr},
    prelude::{
        Color, Cube, Cylinder, Difference, Hull, Import3D, Intersection, LinearExtrude, Minkowski,
        Mirror3D, MultMatrix3D, Polyhedron, Resize3D, Rotate3D, RotateExtrude, Scale3D, Sphere,
        Surface, Translate3D, Union,
    },
    scad_display::{ambassador_impl_ScadDisplay, ScadDisplay},
};

/// A 3D object in SCAD.
#[derive(Debug, Clone, From)]
pub enum ScadObject3D {
    /// A primitive 3D object.
    Primitive(ScadPrimitive3D),
    /// A modifier 3D object.
    Modifier(ScadModifier3D),
    /// A block of 3D objects.
    Block(ScadBlock3D),
}

impl ScadDisplay for ScadObject3D {
    fn repr_scad(&self) -> String {
        match self {
            ScadObject3D::Primitive(p) => p.repr_scad(),
            ScadObject3D::Modifier(m) => m.repr_scad(),
            ScadObject3D::Block(b) => b.repr_scad(),
        }
    }
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

impl From<ScadPrimitive3D> for crate::common::ScadObjectGeneric<crate::common::D3> {
    fn from(val: ScadPrimitive3D) -> crate::common::ScadObjectGeneric<crate::common::D3> {
        let o = ScadObject3D::Primitive(val);
        let rc_o = Rc::new(o);
        let rc_impl = Rc::new(crate::common::ScadObjectImpl::Object3D(rc_o));
        crate::common::ScadObjectGeneric::from_impl(rc_impl)
    }
}

/// A modifier for a 3D object in SCAD.
#[derive(Debug, Clone, From)]
pub struct ScadModifier3D {
    /// The body of the modifier.
    pub body: ScadModifierBody3D,
    /// The child object to be modified.
    pub child: Rc<crate::common::ScadObjectImpl>,
}

impl ScadModifier3D {
    /// Creates a new [`ScadModifier3D`] if the child's type matches the modifier's expected child type.
    ///
    /// # Returns
    ///
    /// + `Some(Self)`: The new object generated.
    /// + `None`: If type of `child`is not matched with `body`
    pub fn try_new(
        body: ScadModifierBody3D,
        child: Rc<crate::common::ScadObjectImpl>,
    ) -> Option<Self> {
        (child.get_type() == body.get_children_type()).then_some(Self { body, child })
    }
}

impl ScadDisplay for ScadModifier3D {
    fn repr_scad(&self) -> String {
        modifier_repr(&self.body, &self.child)
    }
}

/// A block of 3D objects in SCAD.
#[derive(Debug, Clone, From)]
pub struct ScadBlock3D {
    /// The objects in the block.
    pub objects: Vec<crate::common::ScadObjectImpl>,
}

impl ScadDisplay for ScadBlock3D {
    fn repr_scad(&self) -> String {
        block_repr(&self.objects)
    }
}

impl ScadBlock3D {
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
    pub fn try_new(objects: &[crate::common::ScadObjectImpl]) -> Option<Self> {
        objects
            .iter()
            .all(|o| o.get_type() == DimensionMarker::Object3D)
            .then_some(Self {
                objects: objects.to_vec(),
            })
    }
}

__impl_into_scad_for_collection_with_try_new!(
    crate::common::D3,
    ScadBlock3D,
    ScadObject3D,
    crate::common::ScadObjectImpl::Object3D,
    "Internal error: Vec<T> or &[T] for D3 should always produce a valid ScadBlock3D"
);

/// A primitive sentences for 3D objects in SCAD.
#[derive(Debug, Clone, Delegate)] // Removed From
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

// Manual From implementations
impl From<Cube> for ScadPrimitiveBody3D {
    fn from(val: Cube) -> Self {
        ScadPrimitiveBody3D::Cube(val)
    }
}

impl From<Cylinder> for ScadPrimitiveBody3D {
    fn from(val: Cylinder) -> Self {
        ScadPrimitiveBody3D::Cylinder(val)
    }
}

impl From<Import3D> for ScadPrimitiveBody3D {
    fn from(val: Import3D) -> Self {
        ScadPrimitiveBody3D::Import(val)
    }
}

impl From<Polyhedron> for ScadPrimitiveBody3D {
    fn from(val: Polyhedron) -> Self {
        ScadPrimitiveBody3D::Polyhedron(val)
    }
}

impl From<Sphere> for ScadPrimitiveBody3D {
    fn from(val: Sphere) -> Self {
        ScadPrimitiveBody3D::Sphere(val)
    }
}

impl From<Surface> for ScadPrimitiveBody3D {
    fn from(val: Surface) -> Self {
        ScadPrimitiveBody3D::Surface(val)
    }
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
    pub(crate) const fn get_children_type(&self) -> DimensionMarker {
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
            | Self::Union(_) => DimensionMarker::Object3D,
            Self::LinearExtrude(_) | Self::RotateExtrude(_) => DimensionMarker::Object2D,
        }
    }
}

impl From<ScadObject3D> for crate::common::ScadObjectGeneric<crate::common::D3> {
    fn from(val: ScadObject3D) -> crate::common::ScadObjectGeneric<crate::common::D3> {
        let rc_o = Rc::new(val);
        let rc_impl = Rc::new(crate::common::ScadObjectImpl::Object3D(rc_o));
        crate::common::ScadObjectGeneric::from_impl(rc_impl)
    }
}
