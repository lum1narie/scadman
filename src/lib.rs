//! A library for generating OpenSCAD code.

pub(crate) mod internal;
mod macros;

#[allow(clippy::redundant_pub_crate)]
mod common;
use std::rc::Rc;

pub use common::*;
use scad_2d::{
    ScadBlock2D, ScadModifier2D, ScadModifierBody2D, ScadPrimitive2D,
    ScadPrimitiveBody2D, ScadObject2D,
};
use scad_3d::{
    ScadBlock3D, ScadModifier3D, ScadModifierBody3D, ScadPrimitive3D,
    ScadPrimitiveBody3D, ScadObject3D,
};
use scad_mixed::{ScadBlockMixed, ScadModifierBodyMixed, ScadModifierMixed, ScadObjectMixed};

use crate::scad_display::ScadDisplay;

pub mod scad_display;
pub mod value_type;

pub mod scad_2d;
pub mod scad_3d;
pub mod scad_mixed;

pub mod scad_sentence;

/// import `prelude::*` so you can be ready to code!
///
/// This prelude intentionally exports a curated subset of the public API that
/// is stable after the runtime-refactor. It avoids exporting removed internal
/// types (like `ScadObjectBody`) while keeping the commonly used helpers.
pub mod prelude {
    pub use crate::{
        // factory helpers
        block_2d, block_2d_commented, block_3d, block_3d_commented, block_mixed,
        block_mixed_commented, modifier_2d, modifier_2d_commented, modifier_3d,
        modifier_3d_commented, modifier_mixed, modifier_mixed_commented, primitive_2d,
        primitive_2d_commented, primitive_3d, primitive_3d_commented, try_block_2d,
        try_block_2d_commented, try_block_3d, try_block_3d_commented, try_modifier_2d,
        try_modifier_2d_commented, try_modifier_3d, try_modifier_3d_commented,
        // core object and helpers
        ScadObject, ScadObjectGeneric, ScadObjectUntyped, ScadObjectDimensionType,
        ScadBuildable as _, ScadBuilder as _,
        // geometry / value types
        value_type::{RGB, RGBA},
        AffineMatrix2D, AffineMatrix3D, Container2D, Container3D, Point2D, Point3D, Unit,
        // sentence and module types (re-export for convenience)
        scad_2d::{ScadBlock2D, ScadModifier2D, ScadModifierBody2D, ScadObject2D, ScadPrimitive2D,
                  ScadPrimitiveBody2D},
        scad_3d::{ScadBlock3D, ScadModifier3D, ScadModifierBody3D, ScadObject3D, ScadPrimitive3D,
                  ScadPrimitiveBody3D},
        scad_mixed::{ScadBlockMixed, ScadModifierBodyMixed, ScadModifierMixed, ScadObjectMixed},
        scad_sentence::{
            Circle, Color, Cube, Cylinder, Difference, Hull, Import2D, Import3D, Intersection,
            LinearExtrude, Minkowski, Mirror2D, Mirror3D, MultMatrix2D, MultMatrix3D, Offset,
            Polygon, Polyhedron, Resize2D, Resize3D, Rotate2D, Rotate3D, RotateExtrude, Scale2D,
            Scale3D, Sphere, Square, Surface, Text, Translate2D, Translate3D, Union,
        },
    };
}

// Helper adapters to produce runtime ScadObjectImpl from existing concrete enums.
// These adapter types implement the small object-safe traits defined in common.rs
// by delegating to the existing repr_scad / to_code paths.
use crate::common::ScadObjectImpl as ImplEnum;

struct Adapter2D(ScadObject2D);

impl ScadObjectRepr2D for Adapter2D {
    fn to_code(&self) -> String {
        // Delegate to the existing 2D enum's repr_scad implementation.
        self.0.repr_scad()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        &self.0
    }
}

impl ScadObjectReprMixed for Adapter2D {
    fn to_code(&self) -> String {
        // existing repr_scad implementations already produce full code (including
        // semicolons or block braces). Use them directly.
        self.0.repr_scad()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        &self.0
    }
}

struct Adapter3D(ScadObject3D);

impl ScadObjectRepr3D for Adapter3D {
    fn to_code(&self) -> String {
        // Delegate to the existing 3D enum's repr_scad implementation.
        self.0.repr_scad()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        &self.0
    }
}

impl ScadObjectReprMixed for Adapter3D {
    fn to_code(&self) -> String {
        self.0.repr_scad()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        &self.0
    }
}

struct AdapterMixed(ScadObjectMixed);
impl ScadObjectReprMixed for AdapterMixed {
    fn to_code(&self) -> String {
        self.0.repr_scad()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        &self.0
    }
}

/// Wrap a 2D concrete enum into the runtime ScadObject (mixed untyped alias).
fn wrap_2d(o: ScadObject2D) -> ScadObject {
    let rc_impl = Rc::new(ImplEnum::Object2D(Rc::new(Adapter2D(o))));
    // ScadObject is a type alias to ScadObjectGeneric<DMixed>, use the ctor present
    ScadObject::from_impl(rc_impl)
}

/// Wrap a 3D concrete enum into the runtime ScadObject.
fn wrap_3d(o: ScadObject3D) -> ScadObject {
    let rc_impl = Rc::new(ImplEnum::Object3D(Rc::new(Adapter3D(o))));
    ScadObject::from_impl(rc_impl)
}

/// Wrap a mixed concrete enum into the runtime ScadObject.
fn wrap_mixed(o: scad_mixed::ScadObjectMixed) -> ScadObject {
    let rc_impl = Rc::new(ImplEnum::ObjectMixed(Rc::new(AdapterMixed(o))));
    ScadObject::from_impl(rc_impl)
}

// 2D generating functions
// ----------------------------------------

/// Creates a 2D primitive [`ScadObject`] from the given input.
///
/// # Arguments
///
/// + `sentence` - A value that can be converted into a [`ScadPrimitiveBody2D`]
///
/// # Returns
///
/// A [`ScadObject`] representing the 2D primitive
pub fn primitive_2d<T: Into<ScadPrimitiveBody2D>>(sentence: T) -> ScadObject {
    let s: ScadPrimitiveBody2D = sentence.into();
    let p: ScadPrimitive2D = s.into();
    let o = ScadObject2D::Primitive(p);
    wrap_2d(o)
}

/// Creates a 2D primitive [`ScadObject`] with a comment.
///
/// # Arguments
///
/// + `sentence` - A value that can be converted into a [`ScadPrimitiveBody2D`]
/// + `comment` - A string slice to add a comment to the [`ScadObject`]
///
/// # Returns
///
/// A [`ScadObject`] representing the 2D primitive with an attached comment
pub fn primitive_2d_commented<T: Into<ScadPrimitiveBody2D>>(
    sentence: T,
    comment: &str,
) -> ScadObject {
    let s: ScadPrimitiveBody2D = sentence.into();
    let p: ScadPrimitive2D = s.into();
    let o = ScadObject2D::Primitive(p);
    let mut obj = wrap_2d(o);
    obj = obj.commented(comment);
    obj
}

/// Attempts to create a 2D modifier [`ScadObject`] with a child object.
///
/// # Arguments
///
/// + `sentence` - A value that can be converted into a [`ScadModifierBody2D`]
/// + `child` - The child [`ScadObject`] to be modified
///
/// # Returns
///
/// An optional [`ScadObject`] representing the 2D modifier, or [`None`] if creation fails
pub fn try_modifier_2d<T: Into<ScadModifierBody2D>>(
    sentence: T,
    child: ScadObject,
) -> Option<ScadObject> {
    let s: ScadModifierBody2D = sentence.into();
    // child.inner is Rc<ScadObjectImpl>; pass a clone of that Rc to the modifier
    let child_impl_rc: Rc<crate::common::ScadObjectImpl> = Rc::clone(&child.inner);
    let m = scad_2d::ScadModifier2D::try_new(s, child_impl_rc)?;
    let o = scad_2d::ScadObject2D::Modifier(m);
    Some(wrap_2d(o))
}

/// Creates a 2D modifier [`ScadObject`] with a child object.
///
/// # Arguments
///
/// + `sentence` - A value that can be converted into a [`ScadModifierBody2D`]
/// + `child` - The child [`ScadObject`] to be modified
///
/// # Panics
///
/// Panics if the modifier cannot be created with the given child
///
/// # Returns
///
/// A [`ScadObject`] representing the 2D modifier
pub fn modifier_2d<T: Into<ScadModifierBody2D>>(sentence: T, child: ScadObject) -> ScadObject {
    let s: ScadModifierBody2D = sentence.into();
    let child_impl_rc: Rc<crate::common::ScadObjectImpl> = Rc::clone(&child.inner);
    let m = scad_2d::ScadModifier2D::try_new(s.clone(), child_impl_rc)
        .unwrap_or_else(|| panic!("Modifier {:?} requires: {:?}", s, s.get_children_type()));
    let o = scad_2d::ScadObject2D::Modifier(m);
    wrap_2d(o)
}

/// Attempts to create a 2D modifier [`ScadObject`] with a child object and a comment.
///
/// # Arguments
///
/// + `sentence` - A value that can be converted into a [`ScadModifierBody2D`]
/// + `child` - The child [`ScadObject`] to be modified
/// + `comment` - A string slice to add a comment to the [`ScadObject`]
///
/// # Returns
///
/// An optional [`ScadObject`] representing the 2D modifier with an attached comment, or [`None`] if creation fails
pub fn try_modifier_2d_commented<T: Into<ScadModifierBody2D>>(
    sentence: T,
    child: ScadObject,
    comment: &str,
) -> Option<ScadObject> {
    let s: ScadModifierBody2D = sentence.into();
    let child_impl_rc: Rc<crate::common::ScadObjectImpl> = Rc::clone(&child.inner);
    let m = scad_2d::ScadModifier2D::try_new(s, child_impl_rc)?;
    let o = scad_2d::ScadObject2D::Modifier(m);
    let mut obj = wrap_2d(o);
    obj = obj.commented(comment);
    Some(obj)
}

/// Creates a 2D modifier [`ScadObject`] with a child object and a comment.
///
/// # Arguments
///
/// + `sentence` - A value that can be converted into a [`ScadModifierBody2D`]
/// + `child` - The child [`ScadObject`] to be modified
/// + `comment` - A string slice to add a comment to the [`ScadObject`]
///
/// # Panics
///
/// Panics if the modifier cannot be created with the given child
///
/// # Returns
///
/// A [`ScadObject`] representing the 2D modifier with an attached comment
pub fn modifier_2d_commented<T: Into<ScadModifierBody2D>>(
    sentence: T,
    child: ScadObject,
    comment: &str,
) -> ScadObject {
    let mut obj = modifier_2d(sentence, child);
    obj = obj.commented(comment);
    obj
}

/// Attempts to create a 2D block [`ScadObject`] from a slice of [`ScadObject`]s.
///
/// # Arguments
///
/// + `objects` - A slice of [`ScadObject`]s to be grouped into a block
///
/// # Returns
///
/// An optional [`ScadObject`] representing the 2D block, or [`None`] if creation fails
pub fn try_block_2d(objects: &[ScadObject]) -> Option<ScadObject> {
    // convert to owned ScadObjectImpl vector for scad_2d::ScadBlock2D::try_new
    let impls: Vec<crate::common::ScadObjectImpl> =
        objects.iter().map(|o| o.inner.as_ref().clone()).collect();
    let c = scad_2d::ScadBlock2D::try_new(&impls)?;
    let o = scad_2d::ScadObject2D::Block(c);
    Some(wrap_2d(o))
}

/// Creates a 2D block [`ScadObject`] from a slice of [`ScadObject`]s.
///
/// # Arguments
///
/// + `objects` - A slice of [`ScadObject`]s to be grouped into a block
///
/// # Panics
///
/// Panics if the block cannot be created from the given objects
///
/// # Returns
///
/// A [`ScadObject`] representing the 2D block
pub fn block_2d(objects: &[ScadObject]) -> ScadObject {
    let impls: Vec<crate::common::ScadObjectImpl> =
        objects.iter().map(|o| o.inner.as_ref().clone()).collect();
    let c = scad_2d::ScadBlock2D::try_new(&impls)
        .expect("Objects in blocks needs to be ScadObject2D");
    let o = scad_2d::ScadObject2D::Block(c);
    wrap_2d(o)
}

/// Attempts to create a 2D block [`ScadObject`] from a slice of [`ScadObject`]s with a comment.
///
/// # Arguments
///
/// + `objects` - A slice of [`ScadObject`]s to be grouped into a block
/// + `comment` - A string slice to add a comment to the [`ScadObject`]
///
/// # Returns
///
/// An optional [`ScadObject`] representing the 2D block with an attached comment, or [`None`] if creation fails
pub fn try_block_2d_commented(objects: &[ScadObject], comment: &str) -> Option<ScadObject> {
    let mut obj = try_block_2d(objects)?;
    obj = obj.commented(comment);
    Some(obj)
}

/// Creates a 2D block [`ScadObject`] from a slice of [`ScadObject`]s with a comment.
///
/// # Arguments
///
/// + `objects` - A slice of [`ScadObject`]s to be grouped into a block
/// + `comment` - A string slice to add a comment to the [`ScadObject`]
///
/// # Panics
///
/// Panics if the block cannot be created from the given objects
///
/// # Returns
///
/// A [`ScadObject`] representing the 2D block with an attached comment
pub fn block_2d_commented(objects: &[ScadObject], comment: &str) -> ScadObject {
    let mut obj = block_2d(objects);
    obj = obj.commented(comment);
    obj
}

// 3D generating functions
// ----------------------------------------

/// Creates a 3D primitive [`ScadObject`] from the given input.
///
/// # Arguments
///
/// + `sentence` - A value that can be converted into a [`ScadPrimitiveBody3D`]
///
/// # Returns
///
/// A [`ScadObject`] representing the 3D primitive
pub fn primitive_3d<T: Into<ScadPrimitiveBody3D>>(sentence: T) -> ScadObject {
    let s: ScadPrimitiveBody3D = sentence.into();
    let p: ScadPrimitive3D = s.into();
    let o = scad_3d::ScadObject3D::Primitive(p);
    wrap_3d(o)
}

/// Creates a 3D primitive [`ScadObject`] with a comment.
///
/// # Arguments
///
/// + `sentence` - A value that can be converted into a [`ScadPrimitiveBody3D`]
/// + `comment` - A string slice to add a comment to the [`ScadObject`]
///
/// # Returns
///
/// A [`ScadObject`] representing the 3D primitive with an attached comment
pub fn primitive_3d_commented<T: Into<ScadPrimitiveBody3D>>(
    sentence: T,
    comment: &str,
) -> ScadObject {
    let mut obj = primitive_3d(sentence);
    obj = obj.commented(comment);
    obj
}

/// Attempts to create a 3D modifier [`ScadObject`] with a child object.
///
/// # Arguments
///
/// + `sentence` - A value that can be converted into a [`ScadModifierBody3D`]
/// + `child` - The child [`ScadObject`] to be modified
///
/// # Returns
///
/// An optional [`ScadObject`] representing the 3D modifier, or [`None`] if creation fails
pub fn try_modifier_3d<T: Into<ScadModifierBody3D>>(
    sentence: T,
    child: ScadObject,
) -> Option<ScadObject> {
    let s: ScadModifierBody3D = sentence.into();
    let child_impl_rc: Rc<crate::common::ScadObjectImpl> = Rc::clone(&child.inner);
    let m = scad_3d::ScadModifier3D::try_new(s, child_impl_rc)?;
    let o = scad_3d::ScadObject3D::Modifier(m);
    Some(wrap_3d(o))
}

/// Creates a 3D modifier [`ScadObject`] with a child object.
///
/// # Arguments
///
/// + `sentence` - A value that can be converted into a [`ScadModifierBody3D`]
/// + `child` - The child [`ScadObject`] to be modified
///
/// # Panics
///
/// Panics if the modifier cannot be created with the given child
///
/// # Returns
///
/// A [`ScadObject`] representing the 3D modifier
pub fn modifier_3d<T: Into<ScadModifierBody3D>>(sentence: T, child: ScadObject) -> ScadObject {
    let s: ScadModifierBody3D = sentence.into();
    let child_impl_rc: Rc<crate::common::ScadObjectImpl> = Rc::clone(&child.inner);
    let m = scad_3d::ScadModifier3D::try_new(s.clone(), child_impl_rc)
        .unwrap_or_else(|| panic!("Modifier {:?} requires: {:?}", s, s.get_children_type()));
    let o = scad_3d::ScadObject3D::Modifier(m);
    wrap_3d(o)
}

/// Attempts to create a 3D modifier [`ScadObject`] with a child object and a comment.
///
/// # Arguments
///
/// + `sentence` - A value that can be converted into a [`ScadModifierBody3D`]
/// + `child` - The child [`ScadObject`] to be modified
/// + `comment` - A string slice to add a comment to the [`ScadObject`]
///
/// # Returns
///
/// An optional [`ScadObject`] representing the 3D modifier with an attached comment, or [`None`] if creation fails
pub fn try_modifier_3d_commented<T: Into<ScadModifierBody3D>>(
    sentence: T,
    child: ScadObject,
    comment: &str,
) -> Option<ScadObject> {
    let mut obj = try_modifier_3d(sentence, child)?;
    obj = obj.commented(comment);
    Some(obj)
}

/// Creates a 3D modifier [`ScadObject`] with a child object and a comment.
///
/// # Arguments
///
/// + `sentence` - A value that can be converted into a [`ScadModifierBody3D`]
/// + `child` - The child [`ScadObject`] to be modified
/// + `comment` - A string slice to add a comment to the [`ScadObject`]
///
/// # Panics
///
/// Panics if the modifier cannot be created with the given child
///
/// # Returns
///
/// A [`ScadObject`] representing the 3D modifier with an attached comment
pub fn modifier_3d_commented<T: Into<ScadModifierBody3D>>(
    sentence: T,
    child: ScadObject,
    comment: &str,
) -> ScadObject {
    let mut obj = modifier_3d(sentence, child);
    obj = obj.commented(comment);
    obj
}

/// Attempts to create a 3D block [`ScadObject`] from a slice of [`ScadObject`]s.
///
/// # Arguments
///
/// + `objects` - A slice of [`ScadObject`]s to be grouped into a block
///
/// # Returns
///
/// An optional [`ScadObject`] representing the 3D block, or [`None`] if creation fails
pub fn try_block_3d(objects: &[ScadObject]) -> Option<ScadObject> {
    let impls: Vec<crate::common::ScadObjectImpl> =
        objects.iter().map(|o| o.inner.as_ref().clone()).collect();
    let c = scad_3d::ScadBlock3D::try_new(&impls)?;
    let o = scad_3d::ScadObject3D::Block(c);
    Some(wrap_3d(o))
}

/// Creates a 3D block [`ScadObject`] from a slice of [`ScadObject`]s.
///
/// # Arguments
///
/// + `objects` - A slice of [`ScadObject`]s to be grouped into a block
///
/// # Panics
///
/// Panics if the block cannot be created from the given objects
///
/// # Returns
///
/// A [`ScadObject`] representing the 3D block
pub fn block_3d(objects: &[ScadObject]) -> ScadObject {
    let impls: Vec<crate::common::ScadObjectImpl> =
        objects.iter().map(|o| o.inner.as_ref().clone()).collect();
    let c = scad_3d::ScadBlock3D::try_new(&impls)
        .expect("Objects in blocks needs to be ScadObject3D");
    let o = scad_3d::ScadObject3D::Block(c);
    wrap_3d(o)
}

/// Attempts to create a 3D block [`ScadObject`] from a slice of [`ScadObject`]s with a comment.
///
/// # Arguments
///
/// + `objects` - A slice of [`ScadObject`]s to be grouped into a block
/// + `comment` - A string slice to add a comment to the [`ScadObject`]
///
/// # Returns
///
/// An optional [`ScadObject`] representing the 3D block with an attached comment, or [`None`] if creation fails
pub fn try_block_3d_commented(objects: &[ScadObject], comment: &str) -> Option<ScadObject> {
    let mut obj = try_block_3d(objects)?;
    obj = obj.commented(comment);
    Some(obj)
}

/// Creates a 3D block [`ScadObject`] from a slice of [`ScadObject`]s with a comment.
///
/// # Arguments
///
/// + `objects` - A slice of [`ScadObject`]s to be grouped into a block
/// + `comment` - A string slice to add a comment to the [`ScadObject`]
///
/// # Panics
///
/// Panics if the block cannot be created from the given objects
///
/// # Returns
///
/// A [`ScadObject`] representing the 3D block with an attached comment
pub fn block_3d_commented(objects: &[ScadObject], comment: &str) -> ScadObject {
    let mut obj = block_3d(objects);
    obj = obj.commented(comment);
    obj
}

// mixed object generating functions
// ----------------------------------------

/// Creates a Mixed modifier [`ScadObject`] with a child object.
///
/// # Arguments
///
/// + `sentence` - A value that can be converted into a [`ScadModifierBodyMixed`]
/// + `child` - The child [`ScadObject`] to be modified
///
/// # Panics
///
/// Panics if the modifier cannot be created with the given child
///
/// # Returns
///
/// A [`ScadObject`] representing the Mixed modifier
pub fn modifier_mixed<T: Into<ScadModifierBodyMixed>>(
    sentence: T,
    child: ScadObject,
) -> ScadObject {
    let s: ScadModifierBodyMixed = sentence.into();
    let child_impl_rc: Rc<crate::common::ScadObjectImpl> = Rc::clone(&child.inner);
    let m = scad_mixed::ScadModifierMixed::new(s, child_impl_rc);
    let o = scad_mixed::ScadObjectMixed::Modifier(m);
    wrap_mixed(o)
}

/// Creates a Mixed modifier [`ScadObject`] with a child object and a comment.
///
/// # Arguments
///
/// + `sentence` - A value that can be converted into a [`ScadModifierBodyMixed`]
/// + `child` - The child [`ScadObject`] to be modified
/// + `comment` - A string slice to add a comment to the [`ScadObject`]
///
/// # Panics
///
/// Panics if the modifier cannot be created with the given child
///
/// # Returns
///
/// A [`ScadObject`] representing the Mixed modifier with an attached comment
pub fn modifier_mixed_commented<T: Into<ScadModifierBodyMixed>>(
    sentence: T,
    child: ScadObject,
    comment: &str,
) -> ScadObject {
    let mut obj = modifier_mixed(sentence, child);
    obj = obj.commented(comment);
    obj
}

/// Creates a Mixed block [`ScadObject`] from a slice of [`ScadObject`]s.
///
/// # Arguments
///
/// + `objects` - A slice of [`ScadObject`]s to be grouped into a block
///
/// # Panics
///
/// Panics if the block cannot be created from the given objects
///
/// # Returns
///
/// A [`ScadObject`] representing the Mixed block
pub fn block_mixed(objects: &[ScadObject]) -> ScadObject {
    let impls: Vec<crate::common::ScadObjectImpl> =
        objects.iter().map(|o| o.inner.as_ref().clone()).collect();
    let c = scad_mixed::ScadBlockMixed::new(&impls);
    let o = scad_mixed::ScadObjectMixed::Block(c);
    wrap_mixed(o)
}

/// Creates a Mixed block [`ScadObject`] from a slice of [`ScadObject`]s with a comment.
///
/// # Arguments
///
/// + `objects` - A slice of [`ScadObject`]s to be grouped into a block
/// + `comment` - A string slice to add a comment to the [`ScadObject`]
///
/// # Panics
///
/// Panics if the block cannot be created from the given objects
///
/// # Returns
///
/// A [`ScadObject`] representing the Mixed block with an attached comment
pub fn block_mixed_commented(objects: &[ScadObject], comment: &str) -> ScadObject {
    let mut obj = block_mixed(objects);
    obj = obj.commented(comment);
    obj
}
