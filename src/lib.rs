//! A library for generating OpenSCAD code.

pub(crate) mod internal;
mod macros;

#[allow(clippy::redundant_pub_crate)]
mod common;
use std::rc::Rc;

pub use common::*;
use scad_2d::{
    ScadBlock2D, ScadModifier2D, ScadModifierBody2D, ScadObject2D, ScadPrimitive2D,
    ScadPrimitiveBody2D,
};
use scad_3d::{
    ScadBlock3D, ScadModifier3D, ScadModifierBody3D, ScadObject3D, ScadPrimitive3D,
    ScadPrimitiveBody3D,
};
use scad_mixed::{ScadBlockMixed, ScadModifierBodyMixed, ScadModifierMixed, ScadObjectMixed};

pub mod scad_display;
pub mod value_type;

pub mod scad_2d;
pub mod scad_3d;
pub mod scad_mixed;

pub mod scad_sentence;

/// import `prelude::*` so you can be ready to code!
pub mod prelude {
    // TODO:
    pub use crate::{
        block_2d, block_2d_commented, block_3d, block_3d_commented, block_mixed,
        block_mixed_commented, modifier_2d, modifier_2d_commented, modifier_3d,
        modifier_3d_commented, modifier_mixed, modifier_mixed_commented, primitive_2d,
        primitive_2d_commented, primitive_3d, primitive_3d_commented,
        scad_2d::{
            ScadBlock2D, ScadModifier2D, ScadModifierBody2D, ScadObject2D, ScadPrimitive2D,
            ScadPrimitiveBody2D,
        },
        scad_3d::{
            ScadBlock3D, ScadModifier3D, ScadModifierBody3D, ScadObject3D, ScadPrimitive3D,
            ScadPrimitiveBody3D,
        },
        scad_mixed::{ScadBlockMixed, ScadModifierBodyMixed, ScadModifierMixed, ScadObjectMixed},
        scad_sentence::{
            Circle, Color, Cube, Cylinder, Difference, Hull, Import2D, Import3D, Intersection,
            LinearExtrude, Minkowski, Mirror2D, Mirror3D, MultMatrix2D, MultMatrix3D, Offset,
            Polygon, Polyhedron, Resize2D, Resize3D, Rotate2D, Rotate3D, RotateExtrude, Scale2D,
            Scale3D, Sphere, Square, Surface, Text, Translate2D, Translate3D, Union,
        },
        try_block_2d, try_block_2d_commented, try_block_3d, try_block_3d_commented,
        try_modifier_2d, try_modifier_2d_commented, try_modifier_3d, try_modifier_3d_commented,
        value_type::{RGB, RGBA},
        AffineMatrix2D, AffineMatrix3D, Container2D, Container3D, Point2D, Point3D,
        ScadBuildable as _, ScadBuilder as _, ScadObject, ScadObjectBody, ScadObjectTrait as _,
        Unit,
    };
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
    let o: ScadObject2D<ScadObject> = p.into();
    o.into()
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
    let o: ScadObject2D<ScadObject> = p.into();
    ScadObject {
        body: ScadObjectBody::Object2D(o),
        comment: Some(comment.to_string()),
    }
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
    let m: ScadModifier2D<ScadObject> = ScadModifier2D::try_new(s, Rc::new(child))?;
    let o: ScadObject2D<ScadObject> = m.into();
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
pub fn modifier_2d<T: Into<ScadModifierBody2D>>(sentence: T, child: ScadObject) -> ScadObject {
    let s: ScadModifierBody2D = sentence.into();
    let Some(m): Option<ScadModifier2D<ScadObject>> =
        ScadModifier2D::try_new(s.clone(), Rc::new(child))
    else {
        panic!("Modifier {:?} requires: {:?}", s, s.get_children_type())
    };
    let o: ScadObject2D<ScadObject> = m.into();
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
pub fn try_modifier_2d_commented<T: Into<ScadModifierBody2D>>(
    sentence: T,
    child: ScadObject,
    comment: &str,
) -> Option<ScadObject> {
    let s: ScadModifierBody2D = sentence.into();
    let m: ScadModifier2D<ScadObject> = ScadModifier2D::try_new(s, Rc::new(child))?;
    let o: ScadObject2D<ScadObject> = m.into();
    Some(ScadObject {
        body: ScadObjectBody::Object2D(o),
        comment: Some(comment.to_string()),
    })
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
    let s: ScadModifierBody2D = sentence.into();
    let Some(m): Option<ScadModifier2D<ScadObject>> =
        ScadModifier2D::try_new(s.clone(), Rc::new(child))
    else {
        panic!("Modifier {:?} requires: {:?}", s, s.get_children_type())
    };
    let o: ScadObject2D<ScadObject> = m.into();
    ScadObject {
        body: ScadObjectBody::Object2D(o),
        comment: Some(comment.to_string()),
    }
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
    let c: ScadBlock2D<ScadObject> = ScadBlock2D::try_new(objects)?;
    let o: ScadObject2D<ScadObject> = c.into();
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
pub fn block_2d(objects: &[ScadObject]) -> ScadObject {
    let c: ScadBlock2D<ScadObject> =
        ScadBlock2D::try_new(objects).expect("Objects in blocks needs to be ScadObject2D");
    let o: ScadObject2D<ScadObject> = c.into();
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
pub fn try_block_2d_commented(objects: &[ScadObject], comment: &str) -> Option<ScadObject> {
    let c: ScadBlock2D<ScadObject> = ScadBlock2D::try_new(objects)?;
    let o: ScadObject2D<ScadObject> = c.into();
    Some(ScadObject {
        body: ScadObjectBody::Object2D(o),
        comment: Some(comment.to_string()),
    })
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
    let c: ScadBlock2D<ScadObject> =
        ScadBlock2D::try_new(objects).expect("Objects in blocks needs to be ScadObject2D");
    let o: ScadObject2D<ScadObject> = c.into();
    ScadObject {
        body: ScadObjectBody::Object2D(o),
        comment: Some(comment.to_string()),
    }
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
    let o: ScadObject3D<ScadObject> = p.into();
    o.into()
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
    let s: ScadPrimitiveBody3D = sentence.into();
    let p: ScadPrimitive3D = s.into();
    let o: ScadObject3D<ScadObject> = p.into();
    ScadObject {
        body: ScadObjectBody::Object3D(o),
        comment: Some(comment.to_string()),
    }
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
    let m: ScadModifier3D<ScadObject> = ScadModifier3D::try_new(s, Rc::new(child))?;
    let o: ScadObject3D<ScadObject> = m.into();
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
pub fn modifier_3d<T: Into<ScadModifierBody3D>>(sentence: T, child: ScadObject) -> ScadObject {
    let s: ScadModifierBody3D = sentence.into();
    let Some(m): Option<ScadModifier3D<ScadObject>> =
        ScadModifier3D::try_new(s.clone(), Rc::new(child))
    else {
        panic!("Modifier {:?} requires: {:?}", s, s.get_children_type())
    };
    let o: ScadObject3D<ScadObject> = m.into();
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
pub fn try_modifier_3d_commented<T: Into<ScadModifierBody3D>>(
    sentence: T,
    child: ScadObject,
    comment: &str,
) -> Option<ScadObject> {
    let s: ScadModifierBody3D = sentence.into();
    let m: ScadModifier3D<ScadObject> = ScadModifier3D::try_new(s, Rc::new(child))?;
    let o: ScadObject3D<ScadObject> = m.into();
    Some(ScadObject {
        body: ScadObjectBody::Object3D(o),
        comment: Some(comment.to_string()),
    })
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
    let s: ScadModifierBody3D = sentence.into();
    let Some(m): Option<ScadModifier3D<ScadObject>> =
        ScadModifier3D::try_new(s.clone(), Rc::new(child))
    else {
        panic!("Modifier {:?} requires: {:?}", s, s.get_children_type())
    };
    let o: ScadObject3D<ScadObject> = m.into();
    ScadObject {
        body: ScadObjectBody::Object3D(o),
        comment: Some(comment.to_string()),
    }
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
    let c: ScadBlock3D<ScadObject> = ScadBlock3D::try_new(objects)?;
    let o: ScadObject3D<ScadObject> = c.into();
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
pub fn block_3d(objects: &[ScadObject]) -> ScadObject {
    let c: ScadBlock3D<ScadObject> =
        ScadBlock3D::try_new(objects).expect("Objects in blocks needs to be ScadObject3D");
    let o: ScadObject3D<ScadObject> = c.into();
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
pub fn try_block_3d_commented(objects: &[ScadObject], comment: &str) -> Option<ScadObject> {
    let c: ScadBlock3D<ScadObject> = ScadBlock3D::try_new(objects)?;
    let o: ScadObject3D<ScadObject> = c.into();
    Some(ScadObject {
        body: ScadObjectBody::Object3D(o),
        comment: Some(comment.to_string()),
    })
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
    let c: ScadBlock3D<ScadObject> =
        ScadBlock3D::try_new(objects).expect("Objects in blocks needs to be ScadObject3D");
    let o: ScadObject3D<ScadObject> = c.into();
    ScadObject {
        body: ScadObjectBody::Object3D(o),
        comment: Some(comment.to_string()),
    }
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
    let m: ScadModifierMixed<ScadObject> = ScadModifierMixed::new(s, Rc::new(child));
    let o: ScadObjectMixed<ScadObject> = m.into();
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
    child: ScadObject,
    comment: &str,
) -> ScadObject {
    let s: ScadModifierBodyMixed = sentence.into();
    let m: ScadModifierMixed<ScadObject> = ScadModifierMixed::new(s, Rc::new(child));
    let o: ScadObjectMixed<ScadObject> = m.into();
    ScadObject {
        body: ScadObjectBody::ObjectMixed(o),
        comment: Some(comment.to_string()),
    }
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
    let c: ScadBlockMixed<ScadObject> = ScadBlockMixed::new(objects);
    let o: ScadObjectMixed<ScadObject> = c.into();
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
pub fn block_mixed_commented(objects: &[ScadObject], comment: &str) -> ScadObject {
    let c: ScadBlockMixed<ScadObject> = ScadBlockMixed::new(objects);
    let o: ScadObjectMixed<ScadObject> = c.into();
    ScadObject {
        body: ScadObjectBody::ObjectMixed(o),
        comment: Some(comment.to_string()),
    }
}
