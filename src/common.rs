use std::fmt::Debug;

use ambassador::{delegatable_trait, Delegate};
use derive_more::derive::From;
use nalgebra as na;

use crate::{
    scad_2d::ScadObject2D,
    scad_3d::ScadObject3D,
    scad_display::{ambassador_impl_ScadDisplay, ScadDisplay},
    scad_mixed::ScadObjectMixed,
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

pub(crate) trait ScadSentence: ScadDisplay + ScadBuildable {}

#[delegatable_trait]
pub(crate) trait ScadCommentDisplay: ScadDisplay {
    fn repr_scad_with_comment(&self, comment: &str) -> String {
        format!("/* {} */\n{}", comment, self.repr_scad())
    }
}

pub trait ScadObjectTrait {
    fn to_code(&self) -> String;
    fn get_type(&self) -> ScadObjectDimensionType;
}

#[derive(Clone, Debug)]
pub struct ScadObject {
    pub body: ScadObjectBody,
    pub comment: Option<String>,
}

impl ScadObject {
    pub fn new(body: ScadObjectBody, comment: &str) -> Self {
        Self {
            body,
            comment: Some(comment.to_string()),
        }
    }

    pub fn set_comment(&mut self, comment: &str) {
        self.comment = Some(comment.to_string());
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

#[derive(Clone, Debug, From, Delegate)]
#[delegate(ScadDisplay)]
#[delegate(ScadCommentDisplay)]
pub enum ScadObjectBody {
    Object2D(ScadObject2D),
    Object3D(ScadObject3D),
    ObjectMixed(ScadObjectMixed),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ScadObjectDimensionType {
    Object2D,
    Object3D,
    ObjectMixed,
}
