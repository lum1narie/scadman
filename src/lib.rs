//! `scadman` is a Rust library designed for programmatically generating OpenSCAD code.
//!
//! It offers a type-safe and structured approach to defining 2D and 3D geometric
//! objects, applying transformations, and performing operations, ultimately outputting
//! valid OpenSCAD code.
//!
//! ## Key Features:
//! - **Generic Type System**: Leverages `ScadObjectGeneric<D>` to provide compile-time
//!   dimensional safety (2D, 3D, Mixed) while maintaining runtime flexibility.
//! - **Comprehensive API**: Supports a wide range of OpenSCAD primitives, modifiers,
//!   and block operations.
//! - **Builder Pattern**: Simplifies configuration of complex SCAD sentences.
//! - **Operator Overloading**: Enables intuitive use of `+`, `-`, `*` for boolean
//!   operations with runtime dimension checks.
//!
//! For detailed usage, examples, and an in-depth understanding of the type system,
//! please refer to the [README.md](https://github.com/lum1narie/scadman/blob/main/README.md).

#[macro_use]
pub(crate) mod internal;
mod macros;

#[allow(clippy::redundant_pub_crate)]
pub mod common;
use scad_2d::ScadModifierBody2D;
use scad_3d::ScadModifierBody3D;
use scad_mixed::ScadModifierBodyMixed;

pub mod scad_display;
pub mod value_type;

pub mod scad_sentence;

pub mod scad_2d;
pub mod scad_3d;
pub mod scad_mixed;

use std::rc::Rc;

use crate::{
    common::{DMixed, DimensionType, ScadObjectGeneric, ScadObjectImpl, D2, D3},
    prelude::{ScadPrimitiveBody2D, ScadPrimitiveBody3D},
    scad_2d::ScadPrimitive2D,
    scad_3d::ScadPrimitive3D,
};

/// import `prelude::*` so you can be ready to code!
///
/// This prelude intentionally exports a curated subset of the public API that
/// is stable after the runtime-refactor. It avoids exporting removed internal
/// types (like `ScadObjectBody`) while keeping the commonly used helpers.
pub mod prelude {
    pub use crate::{
        // factory helpers
        block_2d,
        block_2d_commented,
        block_3d,
        block_3d_commented,
        block_mixed,
        block_mixed_commented,
        common::{
            AffineMatrix2D, AffineMatrix3D, Container2D, Container3D, DimensionMarker, Point2D,
            Point3D, ScadBuildable as _, ScadBuilder as _, ScadObject, ScadObject2D, ScadObject3D,
            ScadObjectGeneric, ScadObjectUntyped, Unit,
        },
        modifier_2d,
        modifier_2d_commented,
        modifier_3d,
        modifier_3d_commented,
        modifier_mixed,
        modifier_mixed_commented,
        primitive_2d,
        primitive_2d_commented,
        primitive_3d,
        primitive_3d_commented,
        scad_2d::{
            ScadBlock2D, ScadModifier2D, ScadModifierBody2D, ScadPrimitive2D, ScadPrimitiveBody2D,
        },
        scad_3d::{
            ScadBlock3D, ScadModifier3D, ScadModifierBody3D, ScadPrimitive3D, ScadPrimitiveBody3D,
        },
        scad_mixed::{ScadBlockMixed, ScadModifierBodyMixed, ScadModifierMixed},
        scad_sentence::{
            Circle, Color, Cube, Cylinder, Difference, Hull, Import2D, Import3D, Intersection,
            LinearExtrude, Minkowski, Mirror2D, Mirror3D, MultMatrix2D, MultMatrix3D, Offset,
            Polygon, Polyhedron, Projection, Resize2D, Resize3D, Rotate2D, Rotate3D, RotateExtrude,
            Scale2D, Scale3D, Sphere, Square, Surface, Text, Translate2D, Translate3D, Union,
        },
        try_block_2d,
        try_block_2d_commented,
        try_block_3d,
        try_block_3d_commented,
        try_modifier_2d,
        try_modifier_2d_commented,
        try_modifier_3d,
        try_modifier_3d_commented,
        value_type::{RGB, RGBA},
    };
}

// Helper adapters to produce runtime ScadObjectImpl from existing concrete enums.
// These adapter types implement the small object-safe traits defined in common.rs
// by delegating to the existing repr_scad / to_code paths.

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
pub fn primitive_2d<T: Into<ScadPrimitiveBody2D>>(sentence: T) -> ScadObjectGeneric<D2> {
    let s: ScadPrimitiveBody2D = sentence.into();
    let p: ScadPrimitive2D = s.into();
    p.into()
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
) -> ScadObjectGeneric<D2> {
    primitive_2d(sentence).commented(comment)
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
pub fn try_modifier_2d<T: Into<ScadModifierBody2D>, D: DimensionType>(
    sentence: T,
    child: ScadObjectGeneric<D>,
) -> Option<ScadObjectGeneric<D2>> {
    let s: ScadModifierBody2D = sentence.into();
    let child_impl_rc: Rc<ScadObjectImpl> = Rc::clone(&child.inner);
    let m = scad_2d::ScadModifier2D::try_new(s, child_impl_rc)?;
    let o = scad_2d::ScadObject2D::Modifier(m);
    Some(o.into())
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
pub fn modifier_2d<T: Into<ScadModifierBody2D>, D: DimensionType>(
    sentence: T,
    child: ScadObjectGeneric<D>,
) -> ScadObjectGeneric<D2> {
    let s: ScadModifierBody2D = sentence.into();
    let child_impl_rc: Rc<ScadObjectImpl> = Rc::clone(&child.inner);
    let m = scad_2d::ScadModifier2D::try_new(s.clone(), child_impl_rc).unwrap_or_else(|| {
        panic!(
            "Modifier {:?} cannot be applied to the given object. Expected children type: {:?}",
            s,
            s.get_children_type()
        )
    });
    let o = scad_2d::ScadObject2D::Modifier(m);
    o.into()
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
pub fn try_modifier_2d_commented<T: Into<ScadModifierBody2D>, D: DimensionType>(
    sentence: T,
    child: ScadObjectGeneric<D>,
    comment: &str,
) -> Option<ScadObjectGeneric<D2>> {
    try_modifier_2d(sentence, child).map(|obj| obj.commented(comment))
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
pub fn modifier_2d_commented<T: Into<ScadModifierBody2D>, D: DimensionType>(
    sentence: T,
    child: ScadObjectGeneric<D>,
    comment: &str,
) -> ScadObjectGeneric<D2> {
    modifier_2d(sentence, child).commented(comment)
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
pub fn try_block_2d(objects: &[ScadObjectGeneric<D2>]) -> Option<ScadObjectGeneric<D2>> {
    let impls: Vec<ScadObjectImpl> = objects.iter().map(|o| o.inner.as_ref().clone()).collect();
    let c = scad_2d::ScadBlock2D::try_new(&impls)?;
    let o = scad_2d::ScadObject2D::Block(c);
    Some(o.into())
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
pub fn block_2d(objects: &[ScadObjectGeneric<D2>]) -> ScadObjectGeneric<D2> {
    let impls: Vec<ScadObjectImpl> = objects.iter().map(|o| o.inner.as_ref().clone()).collect();
    let c =
        scad_2d::ScadBlock2D::try_new(&impls).expect("Objects in blocks needs to be ScadObject2D");
    let o = scad_2d::ScadObject2D::Block(c);
    o.into()
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
pub fn try_block_2d_commented(
    objects: &[ScadObjectGeneric<D2>],
    comment: &str,
) -> Option<ScadObjectGeneric<D2>> {
    try_block_2d(objects).map(|obj| obj.commented(comment))
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
pub fn block_2d_commented(
    objects: &[ScadObjectGeneric<D2>],
    comment: &str,
) -> ScadObjectGeneric<D2> {
    block_2d(objects).commented(comment)
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
pub fn primitive_3d<T: Into<ScadPrimitiveBody3D>>(sentence: T) -> ScadObjectGeneric<D3> {
    let s: ScadPrimitiveBody3D = sentence.into();
    let p: ScadPrimitive3D = s.into();
    p.into()
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
) -> ScadObjectGeneric<D3> {
    primitive_3d(sentence).commented(comment)
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
pub fn try_modifier_3d<T: Into<ScadModifierBody3D>, D: DimensionType>(
    sentence: T,
    child: ScadObjectGeneric<D>,
) -> Option<ScadObjectGeneric<D3>> {
    let s: ScadModifierBody3D = sentence.into();
    let child_impl_rc: Rc<ScadObjectImpl> = Rc::clone(&child.inner);
    let m = scad_3d::ScadModifier3D::try_new(s, child_impl_rc)?;
    let o = scad_3d::ScadObject3D::Modifier(m);
    Some(o.into())
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
pub fn modifier_3d<T: Into<ScadModifierBody3D>, D: DimensionType>(
    sentence: T,
    child: ScadObjectGeneric<D>,
) -> ScadObjectGeneric<D3> {
    let s: ScadModifierBody3D = sentence.into();
    let child_impl_rc: Rc<ScadObjectImpl> = Rc::clone(&child.inner);
    let m = scad_3d::ScadModifier3D::try_new(s.clone(), child_impl_rc).unwrap_or_else(|| {
        panic!(
            "Modifier {:?} cannot be applied to the given object. Expected children type: {:?}",
            s,
            s.get_children_type()
        )
    });
    let o = scad_3d::ScadObject3D::Modifier(m);
    o.into()
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
pub fn try_modifier_3d_commented<T: Into<ScadModifierBody3D>, D: DimensionType>(
    sentence: T,
    child: ScadObjectGeneric<D>,
    comment: &str,
) -> Option<ScadObjectGeneric<D3>> {
    try_modifier_3d(sentence, child).map(|obj| obj.commented(comment))
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
pub fn modifier_3d_commented<T: Into<ScadModifierBody3D>, D: DimensionType>(
    sentence: T,
    child: ScadObjectGeneric<D>,
    comment: &str,
) -> ScadObjectGeneric<D3> {
    modifier_3d(sentence, child).commented(comment)
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
pub fn try_block_3d(objects: &[ScadObjectGeneric<D3>]) -> Option<ScadObjectGeneric<D3>> {
    let impls: Vec<ScadObjectImpl> = objects.iter().map(|o| o.inner.as_ref().clone()).collect();
    let c = scad_3d::ScadBlock3D::try_new(&impls)?;
    let o = scad_3d::ScadObject3D::Block(c);
    Some(o.into())
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
pub fn block_3d(objects: &[ScadObjectGeneric<D3>]) -> ScadObjectGeneric<D3> {
    let impls: Vec<ScadObjectImpl> = objects.iter().map(|o| o.inner.as_ref().clone()).collect();
    let c =
        scad_3d::ScadBlock3D::try_new(&impls).expect("Objects in blocks needs to be ScadObject3D");
    let o = scad_3d::ScadObject3D::Block(c);
    o.into()
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
pub fn try_block_3d_commented(
    objects: &[ScadObjectGeneric<D3>],
    comment: &str,
) -> Option<ScadObjectGeneric<D3>> {
    try_block_3d(objects).map(|obj| obj.commented(comment))
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
pub fn block_3d_commented(
    objects: &[ScadObjectGeneric<D3>],
    comment: &str,
) -> ScadObjectGeneric<D3> {
    block_3d(objects).commented(comment)
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
    child: ScadObjectGeneric<DMixed>,
) -> ScadObjectGeneric<DMixed> {
    let s: ScadModifierBodyMixed = sentence.into();
    let child_impl_rc: Rc<ScadObjectImpl> = Rc::clone(&child.inner);
    let m = scad_mixed::ScadModifierMixed::new(s, child_impl_rc);
    let o = scad_mixed::ScadObjectMixed::Modifier(m);
    o.into()
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
    child: ScadObjectGeneric<DMixed>,
    comment: &str,
) -> ScadObjectGeneric<DMixed> {
    modifier_mixed(sentence, child).commented(comment)
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
pub fn block_mixed(objects: &[ScadObjectGeneric<DMixed>]) -> ScadObjectGeneric<DMixed> {
    let impls: Vec<ScadObjectImpl> = objects.iter().map(|o| o.inner.as_ref().clone()).collect();
    let c = scad_mixed::ScadBlockMixed::new(&impls);
    let o = scad_mixed::ScadObjectMixed::Block(c);
    o.into()
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
pub fn block_mixed_commented(
    objects: &[ScadObjectGeneric<DMixed>],
    comment: &str,
) -> ScadObjectGeneric<DMixed> {
    block_mixed(objects).commented(comment)
}
