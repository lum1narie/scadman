use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
    rc::Rc,
};

use ambassador::{delegatable_trait, Delegate};
use derive_more::derive::From;
use nalgebra as na;

use crate::{
    prelude::{Difference, Intersection},
    scad_2d::{ScadBlock2D, ScadModifier2D, ScadModifierBody2D, ScadObject2D},
    scad_3d::{ScadBlock3D, ScadModifier3D, ScadModifierBody3D, ScadObject3D},
    scad_display::{ambassador_impl_ScadDisplay, ScadDisplay},
    scad_mixed::ScadObjectMixed,
    scad_sentence::Union,
};

/// Unit of length to write in SCAD code.
pub type Unit = f64;
/// Container type for 2D things.
pub type Container2D<T> = na::Vector2<T>;
/// Container type for 3D things.
pub type Container3D<T> = na::Vector3<T>;
/// Data type for 2D points.
pub type Point2D = Container2D<Unit>;
/// Data type for 3D points.
pub type Point3D = Container3D<Unit>;
/// Data type for Affine transformations in 2D.
pub type AffineMatrix2D = na::Matrix2x3<Unit>;
/// Data type for Affine transformations in 3D.
pub type AffineMatrix3D = na::Matrix3x4<Unit>;

/// The number of space for indent
pub const INDENT: usize = 2;

/// Trait for builders that can be build [`ScadBuildable`]
pub trait ScadBuilder: Default {
    /// Type of the object that this object can build;
    type Target: ScadBuildable;
    /// Type of error that can be returned when building the [`Self::Target`].
    type Error: Debug;

    /// Build the [`Self::Target`] from the builder.
    ///
    /// # Returns
    ///
    /// + [`Ok(Self::Target)`] if the builder can build the [`Self::Target`]
    /// + [`Err(Self::Error)`] if the builder cannot build the [`Self::Target`]
    fn build_scad(&self) -> Result<Self::Target, Self::Error>;
}

/// Trait for scad objects that can be built from builder,
/// then can be become into a [`ScadObject2D`] or [`ScadObject3D`].
pub trait ScadBuildable: Sized {
    /// Type of the builder can be build this object.
    type Builder: ScadBuilder<Target = Self>;

    /// Create a new instance with a closure to configure its builder.
    ///
    /// # Arguments
    ///
    /// + `builder_config` - closure to configure the builder
    ///
    /// # Returns
    ///
    /// New instance of the [`Self::Enum`]
    fn build_with<T: FnOnce(&mut Self::Builder)>(builder_config: T) -> Self {
        let mut builder = Self::Builder::default();
        builder_config(&mut builder);
        builder.build_scad().expect("required fields are not set")
    }
}

/// Trait for objects representing a single sentence in SCAD.
pub(crate) trait ScadSentence: ScadDisplay + ScadBuildable {}

/// Trait for object that can be shown with comment.
#[delegatable_trait]
pub(crate) trait ScadCommentDisplay: ScadDisplay {
    /// Returns a string representation of the object with a comment.
    ///
    /// # Arguments
    ///
    /// + `comment` - The comment to be shown with the object.
    ///
    /// # Returns
    ///
    /// A string representation of the object with the comment.
    fn repr_scad_with_comment(&self, comment: &str) -> String {
        format!("/* {} */\n{}", comment, self.repr_scad())
    }
}

/// Trait for SCAD Objects
pub trait ScadObjectTrait: Clone {
    /// Returns a string representation of the object.
    /// Return value must include trailing '\n'
    fn to_code(&self) -> String;
    /// Returns the dimension type of the object.
    fn get_type(&self) -> ScadObjectDimensionType;
}

/// Struct representing a Scad Object
#[derive(Clone, Debug)]
pub struct ScadObject {
    /// The body of the SCAD Object
    pub body: ScadObjectBody,
    /// An optional comment for the Scad Object
    pub comment: Option<String>,
}

impl ScadObject {
    /// Creates a new [`ScadObject`] with a comment.
    pub fn new(body: ScadObjectBody, comment: &str) -> Self {
        Self {
            body,
            comment: Some(comment.to_string()),
        }
    }

    /// Sets the comment of the [`ScadObject`].
    pub fn set_comment(&mut self, comment: &str) {
        self.comment = Some(comment.to_string());
    }

    /// Sets the comment of the [`ScadObject`].
    pub fn commented(self, comment: &str) -> Self {
        Self {
            body: self.body,
            comment: Some(comment.to_string()),
        }
    }
}

impl ScadObjectTrait for ScadObject {
    fn to_code(&self) -> String {
        match &self.comment {
            Some(c) => self.body.repr_scad_with_comment(c),
            None => self.body.repr_scad(),
        }
    }

    fn get_type(&self) -> ScadObjectDimensionType {
        match &self.body {
            ScadObjectBody::Object2D(_) => ScadObjectDimensionType::Object2D,
            ScadObjectBody::Object3D(_) => ScadObjectDimensionType::Object3D,
            ScadObjectBody::ObjectMixed(_) => ScadObjectDimensionType::ObjectMixed,
        }
    }
}

impl From<ScadObjectBody> for ScadObject {
    fn from(value: ScadObjectBody) -> Self {
        Self {
            body: value,
            comment: None,
        }
    }
}

/// Enum representing the body of a Scad Object.
#[derive(Clone, Debug, From, Delegate)]
#[delegate(ScadDisplay)]
#[delegate(ScadCommentDisplay)]
pub enum ScadObjectBody {
    /// 2D Scad Object
    Object2D(ScadObject2D<ScadObject>),
    /// 3D Scad Object
    Object3D(ScadObject3D<ScadObject>),
    /// Mixed Scad Object
    ObjectMixed(ScadObjectMixed<ScadObject>),
}

macro_rules! __impl_from_for_scadobject {
    ( $type:ty ) => {
        impl From<$type> for ScadObject {
            fn from(value: $type) -> Self {
                Self {
                    body: value.into(),
                    comment: None,
                }
            }
        }
    };
}

__impl_from_for_scadobject!(ScadObject2D<ScadObject>);
__impl_from_for_scadobject!(ScadObject3D<ScadObject>);
__impl_from_for_scadobject!(ScadObjectMixed<ScadObject>);

/// Enum representing the dimension type of a Scad Object.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ScadObjectDimensionType {
    /// 2D Scad Object
    Object2D,
    /// 3D Scad Object
    Object3D,
    /// Mixed Scad Object
    ObjectMixed,
}

impl Add for ScadObject {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let self_type = self.get_type();
        let rhs_type = rhs.get_type();

        assert!(
            self_type == rhs_type,
            "`{self_type:?} + {rhs_type:?}` is not allowed"
        );
        assert!(
            self_type != ScadObjectDimensionType::ObjectMixed,
            "`+` of Mixed object is not allowed"
        );

        let mut children = Vec::new();

        // Extract children from self if it's a Union
        match &self.body {
            ScadObjectBody::Object2D(ScadObject2D::Modifier(m))
                if matches!(m.body, ScadModifierBody2D::Union(_)) =>
            {
                match &m.child.body {
                    ScadObjectBody::Object2D(ScadObject2D::Block(b)) => {
                        children.extend(b.objects.clone());
                    }
                    _ => children.push((*m.child).clone()),
                }
            }
            ScadObjectBody::Object3D(ScadObject3D::Modifier(m))
                if matches!(m.body, ScadModifierBody3D::Union(_)) =>
            {
                match &m.child.body {
                    ScadObjectBody::Object3D(ScadObject3D::Block(b)) => {
                        children.extend(b.objects.clone());
                    }
                    _ => children.push((*m.child).clone()),
                }
            }
            _ => children.push(self),
        }

        // Extract children from rhs if it's a Union
        match &rhs.body {
            ScadObjectBody::Object2D(ScadObject2D::Modifier(m))
                if matches!(m.body, ScadModifierBody2D::Union(_)) =>
            {
                match &m.child.body {
                    ScadObjectBody::Object2D(ScadObject2D::Block(b)) => {
                        children.extend(b.objects.clone());
                    }
                    _ => children.push((*m.child).clone()),
                }
            }
            ScadObjectBody::Object3D(ScadObject3D::Modifier(m))
                if matches!(m.body, ScadModifierBody3D::Union(_)) =>
            {
                match &m.child.body {
                    ScadObjectBody::Object3D(ScadObject3D::Block(b)) => {
                        children.extend(b.objects.clone());
                    }
                    _ => children.push((*m.child).clone()),
                }
            }
            _ => children.push(rhs),
        }

        // Create the new Union object
        match self_type {
            ScadObjectDimensionType::Object2D => {
                let block = ScadBlock2D::try_new(&children).expect("Children must be 2D");
                let block_obj: Self = Into::<ScadObject2D<Self>>::into(block).into();
                let union_modifier =
                    ScadModifier2D::try_new(Union::new().into(), Rc::new(block_obj))
                        .expect("Failed to create Union modifier");
                Into::<ScadObject2D<Self>>::into(union_modifier).into()
            }
            ScadObjectDimensionType::Object3D => {
                let block = ScadBlock3D::try_new(&children).expect("Children must be 3D");
                let block_obj: Self = Into::<ScadObject3D<Self>>::into(block).into();
                let union_modifier =
                    ScadModifier3D::try_new(Union::new().into(), Rc::new(block_obj))
                        .expect("Failed to create Union modifier");
                Into::<ScadObject3D<Self>>::into(union_modifier).into()
            }
            ScadObjectDimensionType::ObjectMixed => unreachable!(), // Already asserted
        }
    }
}

impl Sub for ScadObject {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let self_type = self.get_type();
        let rhs_type = rhs.get_type();

        assert!(
            self_type == rhs_type,
            "`{self_type:?} - {rhs_type:?}` is not allowed"
        );
        assert!(
            self_type != ScadObjectDimensionType::ObjectMixed,
            "`-` of Mixed object is not allowed"
        );

        let mut children = Vec::new();

        // Extract children from self if it's a Difference
        match &self.body {
            ScadObjectBody::Object2D(ScadObject2D::Modifier(m))
                if matches!(m.body, ScadModifierBody2D::Difference(_)) =>
            {
                match &m.child.body {
                    ScadObjectBody::Object2D(ScadObject2D::Block(b)) => {
                        children.extend(b.objects.clone());
                    }
                    _ => children.push((*m.child).clone()),
                }
            }
            ScadObjectBody::Object3D(ScadObject3D::Modifier(m))
                if matches!(m.body, ScadModifierBody3D::Difference(_)) =>
            {
                match &m.child.body {
                    ScadObjectBody::Object3D(ScadObject3D::Block(b)) => {
                        children.extend(b.objects.clone());
                    }
                    _ => children.push((*m.child).clone()),
                }
            }
            _ => children.push(self),
        }

        // Add rhs as is (don't expand if it's a Difference)
        children.push(rhs);

        // Create the new Difference object
        match self_type {
            ScadObjectDimensionType::Object2D => {
                let block = ScadBlock2D::try_new(&children).expect("Children must be 2D");
                let block_obj: Self = Into::<ScadObject2D<Self>>::into(block).into();
                let diff_modifier =
                    ScadModifier2D::try_new(Difference::new().into(), Rc::new(block_obj))
                        .expect("Failed to create Difference modifier");
                Into::<ScadObject2D<Self>>::into(diff_modifier).into()
            }
            ScadObjectDimensionType::Object3D => {
                let block = ScadBlock3D::try_new(&children).expect("Children must be 3D");
                let block_obj: Self = Into::<ScadObject3D<Self>>::into(block).into();
                let diff_modifier =
                    ScadModifier3D::try_new(Difference::new().into(), Rc::new(block_obj))
                        .expect("Failed to create Difference modifier");
                Into::<ScadObject3D<Self>>::into(diff_modifier).into()
            }
            ScadObjectDimensionType::ObjectMixed => unreachable!(), // Already asserted
        }
    }
}

impl Mul for ScadObject {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let self_type = self.get_type();
        let rhs_type = rhs.get_type();

        assert!(
            self_type == rhs_type,
            "`{self_type:?} * {rhs_type:?}` is not allowed"
        );
        assert!(
            self_type != ScadObjectDimensionType::ObjectMixed,
            "`*` of Mixed object is not allowed"
        );

        let mut children = Vec::new();

        // Extract children from self if it's an Intersection
        match &self.body {
            ScadObjectBody::Object2D(ScadObject2D::Modifier(m))
                if matches!(m.body, ScadModifierBody2D::Intersection(_)) =>
            {
                match &m.child.body {
                    ScadObjectBody::Object2D(ScadObject2D::Block(b)) => {
                        children.extend(b.objects.clone());
                    }
                    _ => children.push((*m.child).clone()),
                }
            }
            ScadObjectBody::Object3D(ScadObject3D::Modifier(m))
                if matches!(m.body, ScadModifierBody3D::Intersection(_)) =>
            {
                match &m.child.body {
                    ScadObjectBody::Object3D(ScadObject3D::Block(b)) => {
                        children.extend(b.objects.clone());
                    }
                    _ => children.push((*m.child).clone()),
                }
            }
            _ => children.push(self),
        }

        // Extract children from rhs if it's an Intersection
        match &rhs.body {
            ScadObjectBody::Object2D(ScadObject2D::Modifier(m))
                if matches!(m.body, ScadModifierBody2D::Intersection(_)) =>
            {
                match &m.child.body {
                    ScadObjectBody::Object2D(ScadObject2D::Block(b)) => {
                        children.extend(b.objects.clone());
                    }
                    _ => children.push((*m.child).clone()),
                }
            }
            ScadObjectBody::Object3D(ScadObject3D::Modifier(m))
                if matches!(m.body, ScadModifierBody3D::Intersection(_)) =>
            {
                match &m.child.body {
                    ScadObjectBody::Object3D(ScadObject3D::Block(b)) => {
                        children.extend(b.objects.clone());
                    }
                    _ => children.push((*m.child).clone()),
                }
            }
            _ => children.push(rhs),
        }

        // Create the new Intersection object
        match self_type {
            ScadObjectDimensionType::Object2D => {
                let block = ScadBlock2D::try_new(&children).expect("Children must be 2D");
                let block_obj: Self = Into::<ScadObject2D<Self>>::into(block).into();
                let inter_modifier =
                    ScadModifier2D::try_new(Intersection::new().into(), Rc::new(block_obj))
                        .expect("Failed to create Intersection modifier");
                Into::<ScadObject2D<Self>>::into(inter_modifier).into()
            }
            ScadObjectDimensionType::Object3D => {
                let block = ScadBlock3D::try_new(&children).expect("Children must be 3D");
                let block_obj: Self = Into::<ScadObject3D<Self>>::into(block).into();
                let inter_modifier =
                    ScadModifier3D::try_new(Intersection::new().into(), Rc::new(block_obj))
                        .expect("Failed to create Intersection modifier");
                Into::<ScadObject3D<Self>>::into(inter_modifier).into()
            }
            ScadObjectDimensionType::ObjectMixed => unreachable!(), // Already asserted
        }
    }
}
