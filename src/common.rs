use std::{
    any::Any,
    fmt::Debug,
    ops::{Add, Mul, Sub},
    rc::Rc,
};

use nalgebra as na;

use crate::scad_display::ScadDisplay;

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

/// The number of spaces for indent in generated SCAD.

pub const INDENT: usize = 2;

/// Trait for builders that can build a Scad object.

pub trait ScadBuilder: Default {
    /// Type of the object that this builder constructs.
    type Target;

    /// Type of error that can be returned when building.
    type Error: Debug;

    /// Build the target object.
    fn build_scad(&self) -> Result<Self::Target, Self::Error>;
}

/// Trait for objects that can be built via a builder.

pub trait ScadBuildable: Sized {
    type Builder: ScadBuilder;

    fn build_with<T: FnOnce(&mut Self::Builder)>(
        builder_config: T,
    ) -> <Self::Builder as ScadBuilder>::Target {
        let mut builder = Self::Builder::default();

        builder_config(&mut builder);

        builder.build_scad().expect("required fields are not set")
    }
}

/// Trait for types that can be directly converted into a `ScadObjectGeneric`<D>.
/// Replaced by standard `Into` trait.

/// Trait for Scad sentence types (primitives/modifiers).

pub(crate) trait ScadSentence: ScadDisplay + ScadBuildable {}

/// Trait for object that can be shown with comment.

/// Marker types to represent object dimensions at the type level.
///
/// These are zero-sized types used as generic parameters for `ScadObject`<D>.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum DimensionMarker {
    Object2D,
    Object3D,
    ObjectMixed,
}

/// Trait used as a marker bound for dimension type parameter D.
pub trait DimensionType {
    const MARKER: DimensionMarker;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct D2;
impl DimensionType for D2 {
    const MARKER: DimensionMarker = DimensionMarker::Object2D;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct D3;
impl DimensionType for D3 {
    const MARKER: DimensionMarker = DimensionMarker::Object3D;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DMixed;
impl DimensionType for DMixed {
    const MARKER: DimensionMarker = DimensionMarker::ObjectMixed;
}

/// Trait for SCAD Objects. Kept for compatibility with modules that
/// used `ScadObjectTrait` before the refactor.
///
/// This trait provides a common interface for SCAD objects, allowing them to
/// return a string representation of their SCAD code and their runtime
/// dimension marker. It is primarily used to bridge older code with the
/// newer `ScadObjectGeneric` API.
pub trait ScadObjectTrait {
    /// Returns a string representation of the object (including trailing newline).
    ///
    /// # Returns
    /// A `String` containing the OpenSCAD code for the object.
    fn to_code(&self) -> String;
    /// Returns the runtime dimension marker.
    ///
    /// # Returns
    /// A `DimensionMarker` indicating whether the object is 2D, 3D, or Mixed.
    fn get_type(&self) -> DimensionMarker;
}

/// Generic `ScadObject` parameterized by a dimension marker D.
///
/// This struct represents a generic OpenSCAD object, allowing for type-level
/// distinction between 2D, 3D, and mixed-dimension objects using the `D`
/// type parameter, which must implement `DimensionType`.
///
/// For backwards compatibility, a type alias `ScadObject` is provided, which
/// maps to `ScadObjectGeneric<DMixed>`, allowing existing code to continue
/// compiling while encouraging new code to use the more type-safe generic
/// versions (`ScadObjectGeneric<D2>` or `ScadObjectGeneric<D3>`)
#[derive(Clone, Debug)]
pub struct ScadObjectGeneric<D: DimensionType> {
    /// Internal implementation uses a small enum wrapper (object-safe) instead
    /// of trying to make the original `ScadObjectTrait` dyn object-safe.
    /// This allows for shared ownership and polymorphic behavior at runtime.
    pub(crate) inner: Rc<ScadObjectImpl>,
    /// A `std::marker::PhantomData<D>` to associate the dimension
    /// marker `D` with this struct without actually storing a value of type `D`.
    pub(crate) phantom: std::marker::PhantomData<D>,
    /// An `Option<String>` to store an optional comment that will
    /// be prepended to the generated SCAD code when `to_code()` is called.
    pub comment: Option<String>,
}

impl<D: DimensionType> ScadObjectGeneric<D> {
    /// Creates a new `ScadObjectGeneric<D>` from an `Rc` reference to a
    /// `ScadObjectImpl`.
    ///
    /// This is an internal constructor used to wrap the concrete, runtime
    /// implementation of a SCAD object with the type-level dimension marker `D`.
    ///
    /// A `debug_assert!` is used to verify that the `DimensionMarker` of the
    /// provided `inner` implementation matches the generic type parameter `D`.
    /// For `DMixed` (untyped) generics, any `DimensionMarker` from the inner
    /// implementation is accepted, allowing for flexible runtime typing.
    ///
    /// # Arguments
    /// * `inner` - An `Rc<ScadObjectImpl>` representing the concrete SCAD object.
    ///
    /// # Returns
    /// A new `ScadObjectGeneric<D>` instance.
    pub fn from_impl(inner: Rc<ScadObjectImpl>) -> Self {
        // runtime assert that implementation marker matches generic marker.
        // For the untyped/mixed generic (DMixed) accept any inner marker: the
        // runtime object can carry Object2D/Object3D/ObjectMixed and still be
        // represented as an untyped ScadObject. For typed generics (D2/D3)
        // keep the debug_assert to catch construction bugs in debug builds.
        if D::MARKER != DimensionMarker::ObjectMixed {
            debug_assert!(
                inner.get_type() == D::MARKER,
                "dimension marker mismatch at construction"
            );
        }

        Self {
            inner,
            phantom: std::marker::PhantomData,
            comment: None,
        }
    }

    /// Attaches a comment to the SCAD object.
    ///
    /// This method allows users to add a descriptive comment that will be
    /// prepended to the generated OpenSCAD code for this object. The comment
    /// is stored internally and rendered when `to_code()` is called.
    ///
    /// To ensure comments are preserved across object manipulations (e.g.,
    /// during union or difference operations), the internal `ScadObjectImpl`
    /// is wrapped with a `ScadObjectImplWithComment` adapter. This adapter
    /// overrides the `to_code()` behavior to include the comment.
    /// Double-wrapping is prevented by checking if the inner implementation
    /// is already a `ScadObjectImplWithComment`.
    ///
    /// # Arguments
    /// * `comment` - A string slice containing the comment to attach.
    ///
    /// # Returns
    /// The `ScadObjectGeneric<D>` instance with the comment attached.
    pub fn commented(mut self, comment: &str) -> Self {
        // keep backwards-compatible comment field
        self.comment = Some(comment.to_string());

        // Avoid double-wrapping: if inner already is our adapter, do not wrap again.
        let already_wrapped = match &*self.inner {
            ScadObjectImpl::Object2D(rc) => rc
                .as_any()
                .downcast_ref::<ScadObjectImplWithComment>()
                .is_some(),
            ScadObjectImpl::Object3D(rc) => rc
                .as_any()
                .downcast_ref::<ScadObjectImplWithComment>()
                .is_some(),
            ScadObjectImpl::ObjectMixed(rc) => rc
                .as_any()
                .downcast_ref::<ScadObjectImplWithComment>()
                .is_some(),
        };

        if !already_wrapped {
            let old_inner = Rc::clone(&self.inner);
            // Preserve the original variant (2D/3D/Mixed) so downstream consumers
            // that inspect the discriminant (e.g. block_2d) still see the same
            // runtime type.
            let new_impl = match old_inner.get_type() {
                DimensionMarker::Object2D => ScadObjectImpl::Object2D(Rc::new(
                    ScadObjectImplWithComment::new(old_inner, comment.to_string()),
                )),
                DimensionMarker::Object3D => ScadObjectImpl::Object3D(Rc::new(
                    ScadObjectImplWithComment::new(old_inner, comment.to_string()),
                )),
                DimensionMarker::ObjectMixed => ScadObjectImpl::ObjectMixed(Rc::new(
                    ScadObjectImplWithComment::new(old_inner, comment.to_string()),
                )),
            };
            self.inner = Rc::new(new_impl);
        }

        self
    }

    /// Generates the OpenSCAD code representation of this object.
    ///
    /// If a comment has been attached using the `commented()` method, it will
    /// be prepended to the generated code. Otherwise, it directly calls the
    /// `to_code()` method of the internal `ScadObjectImpl`.
    ///
    /// # Returns
    /// A `String` containing the OpenSCAD code for the object, potentially
    /// including a comment.
    pub fn to_code(&self) -> String {
        match &self.comment {
            Some(c) => format!("/* {} */\n{}", c, self.inner.to_code()),
            None => self.inner.to_code(),
        }
    }

    /// Converts this `ScadObjectGeneric<D>` into a `ScadObjectWrapperToDeprecated`.
    ///
    /// This method is primarily for interoperability with older parts of the
    /// codebase that still expect the `ScadObjectWrapperToDeprecated` type.
    /// It creates a `Weak` reference to the internal `ScadObjectImpl` to
    /// avoid ownership issues during the transition to the typed API.
    ///
    /// # Returns
    /// A `ScadObjectWrapperToDeprecated` instance.
    pub fn into_wrapper(self) -> ScadObjectWrapperToDeprecated {
        ScadObjectWrapperToDeprecated {
            inner: Rc::downgrade(&self.inner),
            comment: self.comment,
        }
    }
}

// End of common.rs — ensure file ends with a newline to satisfy the parser.

/// A thin runtime wrapper for interoperability with existing code that expected
/// a single `ScadObject` value. This keeps a weak reference to the real inner
/// Rc to avoid ownership changes when bridging typed -> untyped worlds.
///
/// This struct is part of the compatibility layer, allowing older code that
/// relied on a less-typed `ScadObject` to still function with the refactored
/// internal representation. It holds a `Weak` reference to the actual
/// `ScadObjectImpl` to prevent circular dependencies and manage lifetimes
/// gracefully.
#[derive(Clone, Debug)]
pub struct ScadObjectWrapperToDeprecated {
    pub(crate) inner: std::rc::Weak<ScadObjectImpl>,
    pub comment: Option<String>,
}

impl ScadObjectWrapperToDeprecated {
    /// Generates the OpenSCAD code for the wrapped object.
    ///
    /// This method attempts to upgrade the weak reference to the inner
    /// `ScadObjectImpl`. If successful, it generates the SCAD code,
    /// prepending any attached comment. If the weak reference cannot be
    /// upgraded (meaning the underlying `ScadObjectImpl` has been dropped),
    /// it returns an empty string.
    ///
    /// # Returns
    /// A `String` containing the OpenSCAD code, or an empty string if the
    /// wrapped object is no longer valid.
    pub fn to_code(&self) -> String {
        match self.inner.upgrade() {
            Some(rc) => match &self.comment {
                Some(c) => format!("/* {} */\n{}", c, rc.to_code()),
                None => rc.to_code(),
            },
            _ => String::new(),
        }
    }

    /// Returns the runtime dimension marker of the wrapped object.
    ///
    /// This method attempts to upgrade the weak reference to the inner
    /// `ScadObjectImpl`. If successful, it returns the `DimensionMarker`
    /// of the wrapped object. If the weak reference cannot be upgraded,
    /// it defaults to `DimensionMarker::ObjectMixed`.
    ///
    /// # Returns
    /// A `DimensionMarker` indicating the object's dimension, or
    /// `DimensionMarker::ObjectMixed` if the wrapped object is no longer valid.
    pub fn get_type(&self) -> DimensionMarker {
        match self.inner.upgrade() {
            Some(rc) => rc.get_type(),
            _ => DimensionMarker::ObjectMixed,
        }
    }
}

/// Runtime concrete implementation enum for object content.
///
/// We purposely implement a single concrete enum (`ScadObjectImpl`) that is
/// object-safe and carries the representation behavior. This is much easier to
/// use behind Rc/Weak than attempting to make the original trait dyn-safe.
pub enum ScadObjectImpl {
    /// 2D concrete variant
    Object2D(Rc<dyn ScadObjectRepr2D>),
    /// 3D concrete variant
    Object3D(Rc<dyn ScadObjectRepr3D>),
    /// Mixed concrete variant (single sentence/object)
    ObjectMixed(Rc<dyn ScadObjectReprMixed>),
}

impl ScadObjectImpl {
    /// Generates the OpenSCAD code representation for the encapsulated object.
    ///
    /// This method acts as a dispatcher, forwarding the `to_code` call to the
    /// specific implementation (2D, 3D, or Mixed) held within the enum.
    ///
    /// # Returns
    /// A `String` containing the OpenSCAD code for the object.
    pub fn to_code(&self) -> String {
        match self {
            Self::Object2D(v) => v.as_ref().to_code(),
            Self::Object3D(v) => v.as_ref().to_code(),
            Self::ObjectMixed(v) => v.as_ref().to_code(),
        }
    }

    /// Returns the `DimensionMarker` that identifies the type of the
    /// encapsulated SCAD object.
    ///
    /// This method provides a runtime way to determine if the object is
    /// a 2D, 3D, or Mixed dimension object.
    ///
    /// # Returns
    /// A `DimensionMarker` value (`Object2D`, `Object3D`, or `ObjectMixed`).
    pub const fn get_type(&self) -> DimensionMarker {
        match self {
            Self::Object2D(_) => DimensionMarker::Object2D,
            Self::Object3D(_) => DimensionMarker::Object3D,
            Self::ObjectMixed(_) => DimensionMarker::ObjectMixed,
        }
    }
}

impl Clone for ScadObjectImpl {
    fn clone(&self) -> Self {
        match self {
            Self::Object2D(rc) => Self::Object2D(Rc::clone(rc)),
            Self::Object3D(rc) => Self::Object3D(Rc::clone(rc)),
            Self::ObjectMixed(rc) => Self::ObjectMixed(Rc::clone(rc)),
        }
    }
}

impl Debug for ScadObjectImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Object2D(_) => f.debug_tuple("ScadObjectImpl::Object2D").finish(),
            Self::Object3D(_) => f.debug_tuple("ScadObjectImpl::Object3D").finish(),
            Self::ObjectMixed(_) => f.debug_tuple("ScadObjectImpl::ObjectMixed").finish(),
        }
    }
}

// Small object-safe helper traits implemented by the concrete sentence enums
// to allow `ScadObjectImpl` to call into existing repr code.

/// Object-safe trait for 2D SCAD object representations.
///
/// This trait provides an object-safe way to handle different concrete
/// 2D SCAD objects (primitives, modifiers, blocks) polymorphically
/// under an `Rc<dyn ScadObjectRepr2D>`.
pub trait ScadObjectRepr2D: Any {
    /// Generates the OpenSCAD code for the 2D object.
    ///
    /// # Returns
    /// A `String` containing the OpenSCAD code for the 2D object.
    fn to_code(&self) -> String;
    /// Allow downcasting to concrete types implementing this trait.
    /// Implemented by concrete enum types (see impls below).
    fn as_any(&self) -> &dyn Any;
}
/// Object-safe trait for 3D SCAD object representations.
///
/// This trait provides an object-safe way to handle different concrete
/// 3D SCAD objects (primitives, modifiers, blocks) polymorphically
/// under an `Rc<dyn ScadObjectRepr3D>`.
pub trait ScadObjectRepr3D: Any {
    /// Generates the OpenSCAD code for the 3D object.
    ///
    /// # Returns
    /// A `String` containing the OpenSCAD code for the 3D object.
    fn to_code(&self) -> String;
    /// Allow downcasting to concrete types implementing this trait.
    /// Implemented by concrete enum types (see impls below).
    fn as_any(&self) -> &dyn Any;
}
/// Object-safe trait for mixed-dimension SCAD object representations.
///
/// This trait provides an object-safe way to handle different concrete
/// mixed-dimension SCAD objects (primitives, modifiers, blocks) polymorphically
/// under an `Rc<dyn ScadObjectReprMixed>`.
pub trait ScadObjectReprMixed: Any {
    /// Generates the OpenSCAD code for the mixed-dimension object.
    ///
    /// # Returns
    /// A `String` containing the OpenSCAD code for the mixed-dimension object.
    fn to_code(&self) -> String;
    /// Allow downcasting to concrete types implementing this trait.
    /// Implemented by concrete enum types (see impls below).
    fn as_any(&self) -> &dyn Any;
}

/// Adapter that wraps an existing `ScadObjectImpl` and emits a leading comment
/// when `to_code()` is called. This preserves comment information when the
/// runtime inner implementations are cloned and embedded into blocks/modify
/// constructs.
///
/// Implement all 2D/3D/Mixed repr traits so the adapter can be used in place
/// of the original concrete variant without changing the variant discriminant.
pub struct ScadObjectImplWithComment {
    pub child: Rc<ScadObjectImpl>,
    pub comment: String,
}

impl ScadObjectImplWithComment {
    /// Creates a new `ScadObjectImplWithComment` adapter.
    ///
    /// This constructor takes an `Rc` reference to the child `ScadObjectImpl`
    /// (the object to which the comment will be attached) and the comment
    /// string itself.
    ///
    /// # Arguments
    /// * `child` - An `Rc<ScadObjectImpl>` representing the object being commented.
    /// * `comment` - A `String` containing the comment text.
    ///
    /// # Returns
    /// A new `ScadObjectImplWithComment` instance.
    pub const fn new(child: Rc<ScadObjectImpl>, comment: String) -> Self {
        Self { child, comment }
    }

    /// Generates the OpenSCAD code with the comment prepended.
    ///
    /// This private helper formats the output by placing the stored comment
    /// before the SCAD code generated by the child object.
    ///
    /// # Returns
    /// A `String` containing the formatted OpenSCAD code with the comment.
    fn prefixed_code(&self) -> String {
        format!("/* {} */\n{}", self.comment, self.child.to_code())
    }
}

impl ScadObjectRepr2D for ScadObjectImplWithComment {
    fn to_code(&self) -> String {
        self.prefixed_code()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ScadObjectRepr3D for ScadObjectImplWithComment {
    fn to_code(&self) -> String {
        self.prefixed_code()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ScadObjectReprMixed for ScadObjectImplWithComment {
    fn to_code(&self) -> String {
        self.prefixed_code()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Implement the small object-safe repr traits for the concrete enum types
// defined in their modules. This avoids creating ad-hoc "Simple*" wrapper
// structs elsewhere and lets the real sentence/objects produce canonical code.

impl ScadObjectRepr2D for crate::scad_2d::ScadObject2D {
    fn to_code(&self) -> String {
        self.repr_scad()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl ScadObjectRepr3D for crate::scad_3d::ScadObject3D {
    fn to_code(&self) -> String {
        self.repr_scad()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl ScadObjectReprMixed for crate::scad_mixed::ScadObjectMixed {
    fn to_code(&self) -> String {
        self.repr_scad()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Backwards-compatible alias: plain `ScadObject` refers to the mixed/runtime
/// variant. Existing call sites that used `ScadObject` (non-generic) will now
/// keep compiling but the new, preferred API is `ScadObjectGeneric<D2>` / `ScadObjectGeneric<D3>`.
pub type ScadObjectUntyped = ScadObjectGeneric<DMixed>;
/// Provide a convenient short name `ScadObject` that maps to the untyped/mixed
/// variant for backwards compatibility.
pub type ScadObject = ScadObjectUntyped;
pub type ScadObject2D = ScadObjectGeneric<D2>;
pub type ScadObject3D = ScadObjectGeneric<D3>;

/// Backwards-compatible alias for the earlier runtime enum type.
// Backwards-compat alias removed: use `DimensionMarker` directly.

///
// Backwards-compat alias removed: use `DimensionMarker` directly.

// Implement conversion helpers and basic operators for untyped compatibility.
// Note: heavy use of boxed trait objects simplifies the transition but can be
// optimized later.

// =============================================================================
// Operators
// =============================================================================

fn flatten_union_parts_2d(rc: &Rc<ScadObjectImpl>, parts: &mut Vec<ScadObjectImpl>) {
    if let ScadObjectImpl::Object2D(inner_enum_rc) = &**rc {
        if let Some(concrete) = inner_enum_rc
            .as_any()
            .downcast_ref::<crate::scad_2d::ScadObject2D>()
        {
            match concrete {
                crate::scad_2d::ScadObject2D::Modifier(m) => {
                    if let crate::scad_2d::ScadModifierBody2D::Union(_) = m.body {
                        if let ScadObjectImpl::Object2D(child_enum_rc) = &*m.child {
                            if let Some(child_concrete) = child_enum_rc
                                .as_any()
                                .downcast_ref::<crate::scad_2d::ScadObject2D>(
                            ) {
                                if let crate::scad_2d::ScadObject2D::Block(b) = child_concrete {
                                    for obj in &b.objects {
                                        parts.push(obj.clone());
                                    }
                                    return;
                                }
                            }
                        }
                    }
                }
                crate::scad_2d::ScadObject2D::Block(b) => {
                    for obj in &b.objects {
                        parts.push(obj.clone());
                    }
                    return;
                }
                _ => {}
            }
        }
    }
    parts.push((**rc).clone());
}

fn create_union_object_2d(parts: Vec<ScadObjectImpl>) -> ScadObject {
    let block = crate::scad_2d::ScadBlock2D { objects: parts };
    let obj_enum = crate::scad_2d::ScadObject2D::Block(block);
    let rc_impl_child = Rc::new(ScadObjectImpl::Object2D(Rc::new(obj_enum)));
    let body = crate::scad_sentence::Union::new();
    let m = crate::scad_2d::ScadModifier2D::try_new(body.into(), Rc::clone(&rc_impl_child))
        .expect("Union modifier requires: Object2D");
    let o = crate::scad_2d::ScadObject2D::Modifier(m);
    let rc_impl = Rc::new(ScadObjectImpl::Object2D(Rc::new(o)));
    ScadObject::from_impl(rc_impl)
}

fn flatten_union_parts_3d(rc: &Rc<ScadObjectImpl>, parts: &mut Vec<ScadObjectImpl>) {
    if let ScadObjectImpl::Object3D(inner_enum_rc) = &**rc {
        if let Some(concrete) = inner_enum_rc
            .as_any()
            .downcast_ref::<crate::scad_3d::ScadObject3D>()
        {
            match concrete {
                crate::scad_3d::ScadObject3D::Modifier(m) => {
                    if let crate::scad_3d::ScadModifierBody3D::Union(_) = m.body {
                        if let ScadObjectImpl::Object3D(child_enum_rc) = &*m.child {
                            if let Some(child_concrete) = child_enum_rc
                                .as_any()
                                .downcast_ref::<crate::scad_3d::ScadObject3D>(
                            ) {
                                if let crate::scad_3d::ScadObject3D::Block(b) = child_concrete {
                                    for obj in &b.objects {
                                        parts.push(obj.clone());
                                    }
                                    return;
                                }
                            }
                        }
                    }
                }
                crate::scad_3d::ScadObject3D::Block(b) => {
                    for obj in &b.objects {
                        parts.push(obj.clone());
                    }
                    return;
                }
                _ => {}
            }
        }
    }
    parts.push((**rc).clone());
}

fn create_union_object_3d(parts: Vec<ScadObjectImpl>) -> ScadObject {
    let block = crate::scad_3d::ScadBlock3D { objects: parts };
    let obj_enum = crate::scad_3d::ScadObject3D::Block(block);
    let rc_impl_child = Rc::new(ScadObjectImpl::Object3D(Rc::new(obj_enum)));
    let body = crate::scad_sentence::Union::new();
    let m = crate::scad_3d::ScadModifier3D::try_new(body.into(), Rc::clone(&rc_impl_child))
        .expect("Union modifier requires: Object3D");
    let o = crate::scad_3d::ScadObject3D::Modifier(m);
    let rc_impl = Rc::new(ScadObjectImpl::Object3D(Rc::new(o)));
    ScadObject::from_impl(rc_impl)
}

fn flatten_difference_parts_2d(rc: &Rc<ScadObjectImpl>, parts: &mut Vec<ScadObjectImpl>) {
    if let ScadObjectImpl::Object2D(inner_enum_rc) = &**rc {
        if let Some(concrete) = inner_enum_rc
            .as_any()
            .downcast_ref::<crate::scad_2d::ScadObject2D>()
        {
            match concrete {
                crate::scad_2d::ScadObject2D::Modifier(m) => {
                    if let crate::scad_2d::ScadModifierBody2D::Difference(_) = m.body {
                        if let ScadObjectImpl::Object2D(child_enum_rc) = &*m.child {
                            if let Some(child_concrete) = child_enum_rc
                                .as_any()
                                .downcast_ref::<crate::scad_2d::ScadObject2D>(
                            ) {
                                if let crate::scad_2d::ScadObject2D::Block(b) = child_concrete {
                                    for obj in &b.objects {
                                        parts.push(obj.clone());
                                    }
                                    return;
                                }
                            }
                        }
                    }
                }
                crate::scad_2d::ScadObject2D::Block(b) => {
                    for obj in &b.objects {
                        parts.push(obj.clone());
                    }
                    return;
                }
                _ => {}
            }
        }
    }
    parts.push((**rc).clone());
}

fn create_difference_object_2d(parts: Vec<ScadObjectImpl>) -> ScadObject {
    let block = crate::scad_2d::ScadBlock2D { objects: parts };
    let obj_enum = crate::scad_2d::ScadObject2D::Block(block);
    let rc_child = Rc::new(ScadObjectImpl::Object2D(Rc::new(obj_enum)));
    let body = crate::scad_sentence::Difference::new();
    let m = crate::scad_2d::ScadModifier2D::try_new(body.into(), Rc::clone(&rc_child))
        .expect("Difference modifier requires: Object2D");
    let o = crate::scad_2d::ScadObject2D::Modifier(m);
    let rc_impl = Rc::new(ScadObjectImpl::Object2D(Rc::new(o)));
    ScadObject::from_impl(rc_impl)
}

fn flatten_difference_parts_3d(rc: &Rc<ScadObjectImpl>, parts: &mut Vec<ScadObjectImpl>) {
    if let ScadObjectImpl::Object3D(inner_enum_rc) = &**rc {
        if let Some(concrete) = inner_enum_rc
            .as_any()
            .downcast_ref::<crate::scad_3d::ScadObject3D>()
        {
            match concrete {
                crate::scad_3d::ScadObject3D::Modifier(m) => {
                    if let crate::scad_3d::ScadModifierBody3D::Difference(_) = m.body {
                        if let ScadObjectImpl::Object3D(child_enum_rc) = &*m.child {
                            if let Some(child_concrete) = child_enum_rc
                                .as_any()
                                .downcast_ref::<crate::scad_3d::ScadObject3D>(
                            ) {
                                if let crate::scad_3d::ScadObject3D::Block(b) = child_concrete {
                                    for obj in &b.objects {
                                        parts.push(obj.clone());
                                    }
                                    return;
                                }
                            }
                        }
                    }
                }
                crate::scad_3d::ScadObject3D::Block(b) => {
                    for obj in &b.objects {
                        parts.push(obj.clone());
                    }
                    return;
                }
                _ => {}
            }
        }
    }
    parts.push((**rc).clone());
}

fn create_difference_object_3d(parts: Vec<ScadObjectImpl>) -> ScadObject {
    let block = crate::scad_3d::ScadBlock3D { objects: parts };
    let obj_enum = crate::scad_3d::ScadObject3D::Block(block);
    let rc_impl_child = Rc::new(ScadObjectImpl::Object3D(Rc::new(obj_enum)));
    let body = crate::scad_sentence::Difference::new();
    let m = crate::scad_3d::ScadModifier3D::try_new(body.into(), Rc::clone(&rc_impl_child))
        .expect("Difference modifier requires: Object3D");
    let o = crate::scad_3d::ScadObject3D::Modifier(m);
    let rc_impl = Rc::new(ScadObjectImpl::Object3D(Rc::new(o)));
    ScadObject::from_impl(rc_impl)
}

fn flatten_intersection_parts_2d(rc: &Rc<ScadObjectImpl>, parts: &mut Vec<ScadObjectImpl>) {
    if let ScadObjectImpl::Object2D(inner_enum_rc) = &**rc {
        if let Some(concrete) = inner_enum_rc
            .as_any()
            .downcast_ref::<crate::scad_2d::ScadObject2D>()
        {
            match concrete {
                crate::scad_2d::ScadObject2D::Modifier(m) => {
                    if let crate::scad_2d::ScadModifierBody2D::Intersection(_) = m.body {
                        if let ScadObjectImpl::Object2D(child_enum_rc) = &*m.child {
                            if let Some(child_concrete) = child_enum_rc
                                .as_any()
                                .downcast_ref::<crate::scad_2d::ScadObject2D>(
                            ) {
                                if let crate::scad_2d::ScadObject2D::Block(b) = child_concrete {
                                    for obj in &b.objects {
                                        parts.push(obj.clone());
                                    }
                                    return;
                                }
                            }
                        }
                    }
                }
                crate::scad_2d::ScadObject2D::Block(b) => {
                    for obj in &b.objects {
                        parts.push(obj.clone());
                    }
                    return;
                }
                _ => {}
            }
        }
    }
    parts.push((**rc).clone());
}

fn create_intersection_object_2d(parts: Vec<ScadObjectImpl>) -> ScadObject {
    let block = crate::scad_2d::ScadBlock2D { objects: parts };
    let obj_enum = crate::scad_2d::ScadObject2D::Block(block);
    let rc_child = Rc::new(ScadObjectImpl::Object2D(Rc::new(obj_enum)));
    let body = crate::scad_sentence::Intersection::new();
    let m = crate::scad_2d::ScadModifier2D::try_new(body.into(), Rc::clone(&rc_child))
        .expect("Intersection modifier requires: Object2D");
    let o = crate::scad_2d::ScadObject2D::Modifier(m);
    let rc_impl = Rc::new(ScadObjectImpl::Object2D(Rc::new(o)));
    ScadObject::from_impl(rc_impl)
}

fn flatten_intersection_parts_3d(rc: &Rc<ScadObjectImpl>, parts: &mut Vec<ScadObjectImpl>) {
    if let ScadObjectImpl::Object3D(inner_enum_rc) = &**rc {
        if let Some(concrete) = inner_enum_rc
            .as_any()
            .downcast_ref::<crate::scad_3d::ScadObject3D>()
        {
            match concrete {
                crate::scad_3d::ScadObject3D::Modifier(m) => {
                    if let crate::scad_3d::ScadModifierBody3D::Intersection(_) = m.body {
                        if let ScadObjectImpl::Object3D(child_enum_rc) = &*m.child {
                            if let Some(child_concrete) = child_enum_rc
                                .as_any()
                                .downcast_ref::<crate::scad_3d::ScadObject3D>(
                            ) {
                                if let crate::scad_3d::ScadObject3D::Block(b) = child_concrete {
                                    for obj in &b.objects {
                                        parts.push(obj.clone());
                                    }
                                    return;
                                }
                            }
                        }
                    }
                }
                crate::scad_3d::ScadObject3D::Block(b) => {
                    for obj in &b.objects {
                        parts.push(obj.clone());
                    }
                    return;
                }
                _ => {}
            }
        }
    }
    parts.push((**rc).clone());
}

fn create_intersection_object_3d(parts: Vec<ScadObjectImpl>) -> ScadObject {
    let block = crate::scad_3d::ScadBlock3D { objects: parts };
    let obj_enum = crate::scad_3d::ScadObject3D::Block(block);
    let rc_impl_child = Rc::new(ScadObjectImpl::Object3D(Rc::new(obj_enum)));
    let body = crate::scad_sentence::Intersection::new();
    let m = crate::scad_3d::ScadModifier3D::try_new(body.into(), Rc::clone(&rc_impl_child))
        .expect("Intersection modifier requires: Object3D");
    let o = crate::scad_3d::ScadObject3D::Modifier(m);
    let rc_impl = Rc::new(ScadObjectImpl::Object3D(Rc::new(o)));
    ScadObject::from_impl(rc_impl)
}

// Add implementations
impl Add for ScadObject2D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut parts: Vec<ScadObjectImpl> = Vec::new();
        flatten_union_parts_2d(&self.inner, &mut parts);
        flatten_union_parts_2d(&rhs.inner, &mut parts);
        create_union_object_2d(parts).into()
    }
}
impl Add for ScadObject3D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut parts: Vec<ScadObjectImpl> = Vec::new();
        flatten_union_parts_3d(&self.inner, &mut parts);
        flatten_union_parts_3d(&rhs.inner, &mut parts);
        create_union_object_3d(parts).into()
    }
}
impl Add<ScadObject> for ScadObject2D {
    type Output = ScadObject;
    fn add(self, rhs: ScadObject) -> Self::Output {
        ScadObject::from(self) + rhs
    }
}
impl Add<ScadObject2D> for ScadObject {
    type Output = Self;
    fn add(self, rhs: ScadObject2D) -> Self::Output {
        self + ScadObject::from(rhs)
    }
}
impl Add<ScadObject> for ScadObject3D {
    type Output = ScadObject;
    fn add(self, rhs: ScadObject) -> Self::Output {
        ScadObject::from(self) + rhs
    }
}
impl Add<ScadObject3D> for ScadObject {
    type Output = Self;
    fn add(self, rhs: ScadObject3D) -> Self::Output {
        self + ScadObject::from(rhs)
    }
}
impl Add for ScadObject {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let left_rc = self.inner;
        let right_rc = rhs.inner;
        let left_type = left_rc.get_type();
        let right_type = right_rc.get_type();

        if left_type == DimensionMarker::ObjectMixed || right_type == DimensionMarker::ObjectMixed {
            let mut parts: Vec<ScadObjectImpl> = Vec::new();
            parts.push((*left_rc).clone());
            parts.push((*right_rc).clone());
            let block = crate::scad_mixed::ScadBlockMixed::new(&parts);
            let child = crate::scad_mixed::ScadObjectMixed::Block(block);
            let rc_child = Rc::new(ScadObjectImpl::ObjectMixed(Rc::new(child)));
            let body = crate::scad_sentence::Union::new();
            let modifier = crate::scad_mixed::ScadModifierMixed::new(body.into(), rc_child);
            let o = crate::scad_mixed::ScadObjectMixed::Modifier(modifier);
            let rc_impl = Rc::new(ScadObjectImpl::ObjectMixed(Rc::new(o)));
            return Self::from_impl(rc_impl);
        }

        assert!(
            left_type == right_type,
            "`{}` is not allowed",
            match (left_type, right_type) {
                (DimensionMarker::Object2D, DimensionMarker::Object3D) => "Object2D + Object3D",
                (DimensionMarker::Object3D, DimensionMarker::Object2D) => "Object3D + Object2D",
                _ => "Mismatched dimensions",
            }
        );

        match left_type {
            DimensionMarker::Object2D => {
                let mut parts: Vec<ScadObjectImpl> = Vec::new();
                flatten_union_parts_2d(&left_rc, &mut parts);
                flatten_union_parts_2d(&right_rc, &mut parts);
                create_union_object_2d(parts)
            }
            DimensionMarker::Object3D => {
                let mut parts: Vec<ScadObjectImpl> = Vec::new();
                flatten_union_parts_3d(&left_rc, &mut parts);
                flatten_union_parts_3d(&right_rc, &mut parts);
                create_union_object_3d(parts)
            }
            DimensionMarker::ObjectMixed => unreachable!(),
        }
    }
}

// Sub implementations
impl Sub for ScadObject2D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut parts: Vec<ScadObjectImpl> = Vec::new();
        flatten_difference_parts_2d(&self.inner, &mut parts);
        parts.push((*rhs.inner).clone());
        create_difference_object_2d(parts).into()
    }
}
impl Sub for ScadObject3D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut parts: Vec<ScadObjectImpl> = Vec::new();
        flatten_difference_parts_3d(&self.inner, &mut parts);
        parts.push((*rhs.inner).clone());
        create_difference_object_3d(parts).into()
    }
}
impl Sub<ScadObject> for ScadObject2D {
    type Output = ScadObject;
    fn sub(self, rhs: ScadObject) -> Self::Output {
        ScadObject::from(self) - rhs
    }
}
impl Sub<ScadObject2D> for ScadObject {
    type Output = Self;
    fn sub(self, rhs: ScadObject2D) -> Self::Output {
        self - ScadObject::from(rhs)
    }
}
impl Sub<ScadObject> for ScadObject3D {
    type Output = ScadObject;
    fn sub(self, rhs: ScadObject) -> Self::Output {
        ScadObject::from(self) - rhs
    }
}
impl Sub<ScadObject3D> for ScadObject {
    type Output = Self;
    fn sub(self, rhs: ScadObject3D) -> Self::Output {
        self - ScadObject::from(rhs)
    }
}
impl Sub for ScadObject {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let left_rc = self.inner;
        let right_rc = rhs.inner;
        let left_type = left_rc.get_type();
        let right_type = right_rc.get_type();

        if left_type == DimensionMarker::ObjectMixed || right_type == DimensionMarker::ObjectMixed {
            let mut parts: Vec<ScadObjectImpl> = Vec::new();
            parts.push((*left_rc).clone());
            parts.push((*right_rc).clone());
            let block = crate::scad_mixed::ScadBlockMixed::new(&parts);
            let child = crate::scad_mixed::ScadObjectMixed::Block(block);
            let rc_child = Rc::new(ScadObjectImpl::ObjectMixed(Rc::new(child)));
            let body = crate::scad_sentence::Difference::new();
            let modifier = crate::scad_mixed::ScadModifierMixed::new(body.into(), rc_child);
            let o = crate::scad_mixed::ScadObjectMixed::Modifier(modifier);
            let rc_impl = Rc::new(ScadObjectImpl::ObjectMixed(Rc::new(o)));
            return Self::from_impl(rc_impl);
        }

        assert!(
            left_type == right_type,
            "`{}` is not allowed",
            match (left_type, right_type) {
                (DimensionMarker::Object2D, DimensionMarker::Object3D) => "Object2D - Object3D",
                (DimensionMarker::Object3D, DimensionMarker::Object2D) => "Object3D - Object2D",
                _ => "Mismatched dimensions",
            }
        );

        match left_type {
            DimensionMarker::Object2D => {
                let mut parts: Vec<ScadObjectImpl> = Vec::new();
                flatten_difference_parts_2d(&left_rc, &mut parts);
                parts.push((*right_rc).clone());
                create_difference_object_2d(parts)
            }
            DimensionMarker::Object3D => {
                let mut parts: Vec<ScadObjectImpl> = Vec::new();
                flatten_difference_parts_3d(&left_rc, &mut parts);
                parts.push((*right_rc).clone());
                create_difference_object_3d(parts)
            }
            DimensionMarker::ObjectMixed => unreachable!(),
        }
    }
}

// Mul implementations
impl Mul for ScadObject2D {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut parts: Vec<ScadObjectImpl> = Vec::new();
        flatten_intersection_parts_2d(&self.inner, &mut parts);
        flatten_intersection_parts_2d(&rhs.inner, &mut parts);
        create_intersection_object_2d(parts).into()
    }
}
impl Mul for ScadObject3D {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut parts: Vec<ScadObjectImpl> = Vec::new();
        flatten_intersection_parts_3d(&self.inner, &mut parts);
        flatten_intersection_parts_3d(&rhs.inner, &mut parts);
        create_intersection_object_3d(parts).into()
    }
}
impl Mul<ScadObject> for ScadObject2D {
    type Output = ScadObject;
    fn mul(self, rhs: ScadObject) -> Self::Output {
        ScadObject::from(self) * rhs
    }
}
impl Mul<ScadObject2D> for ScadObject {
    type Output = Self;
    fn mul(self, rhs: ScadObject2D) -> Self::Output {
        self * ScadObject::from(rhs)
    }
}
impl Mul<ScadObject> for ScadObject3D {
    type Output = ScadObject;
    fn mul(self, rhs: ScadObject) -> Self::Output {
        ScadObject::from(self) * rhs
    }
}
impl Mul<ScadObject3D> for ScadObject {
    type Output = Self;
    fn mul(self, rhs: ScadObject3D) -> Self::Output {
        self * ScadObject::from(rhs)
    }
}
impl Mul for ScadObject {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let left_rc = self.inner;
        let right_rc = rhs.inner;
        let left_type = left_rc.get_type();
        let right_type = right_rc.get_type();

        if left_type == DimensionMarker::ObjectMixed || right_type == DimensionMarker::ObjectMixed {
            let mut parts: Vec<ScadObjectImpl> = Vec::new();
            parts.push((*left_rc).clone());
            parts.push((*right_rc).clone());
            let block = crate::scad_mixed::ScadBlockMixed::new(&parts);
            let child = crate::scad_mixed::ScadObjectMixed::Block(block);
            let rc_child = Rc::new(ScadObjectImpl::ObjectMixed(Rc::new(child)));
            let body = crate::scad_sentence::Intersection::new();
            let modifier = crate::scad_mixed::ScadModifierMixed::new(body.into(), rc_child);
            let o = crate::scad_mixed::ScadObjectMixed::Modifier(modifier);
            let rc_impl = Rc::new(ScadObjectImpl::ObjectMixed(Rc::new(o)));
            return Self::from_impl(rc_impl);
        }

        assert!(
            left_type == right_type,
            "`{}` is not allowed",
            match (left_type, right_type) {
                (DimensionMarker::Object2D, DimensionMarker::Object3D) => "Object2D * Object3D",
                (DimensionMarker::Object3D, DimensionMarker::Object2D) => "Object3D * Object2D",
                _ => "Mismatched dimensions",
            }
        );

        match left_type {
            DimensionMarker::Object2D => {
                let mut parts: Vec<ScadObjectImpl> = Vec::new();
                flatten_intersection_parts_2d(&left_rc, &mut parts);
                flatten_intersection_parts_2d(&right_rc, &mut parts);
                create_intersection_object_2d(parts)
            }
            DimensionMarker::Object3D => {
                let mut parts: Vec<ScadObjectImpl> = Vec::new();
                flatten_intersection_parts_3d(&left_rc, &mut parts);
                flatten_intersection_parts_3d(&right_rc, &mut parts);
                create_intersection_object_3d(parts)
            }
            DimensionMarker::ObjectMixed => unreachable!(),
        }
    }
}

// From 2D to untyped
impl From<ScadObjectGeneric<D2>> for ScadObjectGeneric<DMixed> {
    fn from(val: ScadObjectGeneric<D2>) -> Self {
        let mut obj = Self::from_impl(val.inner);
        obj.comment = val.comment;
        obj
    }
}

// From 3D to untyped
impl From<ScadObjectGeneric<D3>> for ScadObjectGeneric<DMixed> {
    fn from(val: ScadObjectGeneric<D3>) -> Self {
        let mut obj = Self::from_impl(val.inner);
        obj.comment = val.comment;
        obj
    }
}

// From untyped to 2D (requires runtime check)
impl From<ScadObjectGeneric<DMixed>> for ScadObjectGeneric<D2> {
    fn from(val: ScadObjectGeneric<DMixed>) -> Self {
        assert!(
            val.inner.get_type() == D2::MARKER,
            "dimension mismatch: expected 2D object, found {:?}",
            val.inner.get_type()
        );
        let mut obj = Self::from_impl(val.inner);
        obj.comment = val.comment;
        obj
    }
}

// From untyped to 3D (requires runtime check)
impl From<ScadObjectGeneric<DMixed>> for ScadObjectGeneric<D3> {
    fn from(val: ScadObjectGeneric<DMixed>) -> Self {
        assert!(
            val.inner.get_type() == D3::MARKER,
            "dimension mismatch: expected 3D object, found {:?}",
            val.inner.get_type()
        );
        let mut obj = Self::from_impl(val.inner);
        obj.comment = val.comment;
        obj
    }
}
