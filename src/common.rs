use std::fmt::Debug;

use ambassador::delegatable_trait;
use nalgebra as na;

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

const INDENT: usize = 2;

/// Trait for objects that can be written to SCAD code.
#[delegatable_trait]
pub trait ScadObjectTrait: Debug + Clone {
    /// Returns the body of the SCAD code.
    ///
    /// Body means the part that is not include
    /// neither any other objects nor semicolon.
    ///
    /// Ex.
    /// + circle may has a body such as `"circle(r = 1)"`
    /// + hull has a body of `"hull();"`
    ///
    /// # Returns
    ///
    /// SCAD code body as a [`String`].
    fn get_body(&self) -> String;

    /// Returns the SCAD code of the object.
    ///
    /// Ex.
    /// + circle may returns such as `"circle(r = 1);"`
    ///
    /// # Returns
    ///
    /// SCAD code of the object as a [`String`].
    fn to_code(&self) -> String {
        let body = self.get_body();
        body + ";"
    }
}

/// Trait for scad objects that modify other objects.
pub trait ScadModifier: ScadObjectTrait {
    /// Type of the objects to be modified.
    /// This should be [`ScadObject2D`] or [`ScadObject3D`].
    type Children: ScadObjectTrait;

    /// Set the children of the modifier then return new object.
    ///
    /// # Arguments
    ///
    /// + `children`: Objects to be modified.
    ///
    /// # Returns
    ///
    /// The new object.
    fn apply_to(self, children: &[Self::Children]) -> Self;

    /// Returns the SCAD code of the children of the object.
    ///
    /// Children means the objects in inner level of the object.
    ///
    /// Ex.
    /// + hull may has a body such as `Some(vec!["circle(r = 1)", "square(size = 2)"])`
    ///
    /// # Returns
    ///
    /// + SCAD object that are children
    fn get_children(&self) -> &Vec<Self::Children>;

    /// Returns the SCAD code of the object with children.
    ///
    /// Ex.
    /// + hull may returns such as `"hull(){\n  circle(r = 1);\n  square(size = 2);\n}"`
    ///
    /// # Returns
    ///
    /// SCAD code of the object as a [`String`].
    fn to_code_with_children(&self) -> String {
        let body = self.get_body();
        let unindented_lines_itr = self.get_children().iter().flat_map(|sobj| {
            sobj.to_code()
                .split('\n')
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        });
        let children = unindented_lines_itr
            .map(|s| format!("{}{}", " ".repeat(INDENT), s))
            .collect::<Vec<_>>()
            .join("\n");
        format!("{body} {{\n{children}\n}}")
    }
}

/// Trait for scad objects that can be built from builder,
/// then can be become into a [`ScadObject2D`] or [`ScadObject3D`].
pub trait ScadBuildable: ScadObjectTrait + Into<Self::Enum> {
    /// Type of the builder can be build this object.
    type Builder: ScadBuilder<Target = Self>;
    /// Type of the enum that this object can be converted.
    /// This should be [`ScadObject2D`] or [`ScadObject3D`].
    type Enum;

    /// Create a new instance with a closure to configure its builder.
    ///
    /// # Arguments
    ///
    /// + `builder_config` - closure to configure the builder
    ///
    /// # Returns
    ///
    /// New instance of the [`Self::Enum`]
    fn build_with<T: FnOnce(&mut Self::Builder)>(builder_config: T) -> Self::Enum {
        let mut builder = Self::Builder::default();
        builder_config(&mut builder);
        builder
            .build_scad()
            .expect("required fields are not set")
            .into()
    }
}

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
