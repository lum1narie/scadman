//! legacy API which was available in v0.1.1 (deprecated)
#![deprecated(since = "0.2.0")]

use std::rc::Rc;

use crate::{
    common::{DMixed, DimensionType, ScadObjectGeneric, ScadObjectImpl, D2, D3},
    prelude::ScadModifierBodyMixed,
    scad_2d::{self, ScadModifierBody2D},
    scad_3d::{self, ScadModifierBody3D},
    scad_mixed,
};

// Helper adapters to produce runtime ScadObjectImpl from existing concrete enums.
// These adapter types implement the small object-safe traits defined in common.rs
// by delegating to the existing repr_scad / to_code paths.

// 2D generating functions
// ----------------------------------------

/// Creates a 2D primitive [`ScadObject`] from the given input.
///
/// # Arguments
///
/// TODO:
///
/// # Returns
///
/// A [`ScadObject`] representing the 2D primitive
pub fn primitive_2d(sentence: ScadObjectGeneric<D2>) -> ScadObjectGeneric<D2> {
    sentence
}

/// Creates a 2D primitive [`ScadObject`] with a comment.
///
/// # Arguments
///
/// + `sentence` - TODO:
/// + `comment` - A string slice to add a comment to the [`ScadObject`]
///
/// # Returns
///
/// A [`ScadObject`] representing the 2D primitive with an attached comment
pub fn primitive_2d_commented(
    sentence: ScadObjectGeneric<D2>,
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
/// TODO:
///
/// # Returns
///
/// A [`ScadObject`] representing the 3D primitive
pub fn primitive_3d(sentence: ScadObjectGeneric<D3>) -> ScadObjectGeneric<D3> {
    sentence
}

/// Creates a 3D primitive [`ScadObject`] with a comment.
///
/// # Arguments
///
/// + `sentence` - TODO:
/// + `comment` - A string slice to add a comment to the [`ScadObject`]
///
/// # Returns
///
/// A [`ScadObject`] representing the 3D primitive with an attached comment
pub fn primitive_3d_commented(
    sentence: ScadObjectGeneric<D3>,
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
