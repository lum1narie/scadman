use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
    rc::Rc,
};

use ambassador::delegatable_trait;
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
    type Target: ScadBuildable;
    /// Type of error that can be returned when building.
    type Error: Debug;

    /// Build the target object.
    fn build_scad(&self) -> Result<Self::Target, Self::Error>;
}

/// Trait for objects that can be built via a builder.
pub trait ScadBuildable: Sized {
    type Builder: ScadBuilder<Target = Self>;

    fn build_with<T: FnOnce(&mut Self::Builder)>(builder_config: T) -> Self {
        let mut builder = Self::Builder::default();
        builder_config(&mut builder);
        builder.build_scad().expect("required fields are not set")
    }
}

/// Trait for types that can be directly converted into a `ScadObjectGeneric`<D>.
pub trait IntoScad<D: DimensionType> {
    fn scad(self) -> ScadObjectGeneric<D>;
}

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
/// TODO: doc
pub trait ScadObjectTrait {
    /// Returns a string representation of the object (including trailing newline).
    fn to_code(&self) -> String;
    /// Returns the runtime dimension marker.
    fn get_type(&self) -> DimensionMarker;
}

/// Generic `ScadObject` parameterized by a dimension marker D.
///
/// NOTE: for a smoother, faster migration we provide both:
/// - `ScadObject`<D> generic (new preferred API)
/// - a type alias `ScadObject` (old untyped name) which maps to `ScadObject`<DMixed>
///
/// /// TODO: doc
#[derive(Clone, Debug)]
pub struct ScadObjectGeneric<D: DimensionType> {
    /// internal implementation uses a small enum wrapper (object-safe) instead
    /// of trying to make the original `ScadObjectTrait` dyn object-safe.
    pub(crate) inner: Rc<ScadObjectImpl>,
    pub(crate) phantom: std::marker::PhantomData<D>,
    /// Optional comment attached by users.
    pub comment: Option<String>,
}

impl<D: DimensionType> ScadObjectGeneric<D> {
    /// TODO: doc
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

    /// TODO: doc
    pub fn commented(mut self, comment: &str) -> Self {
        // keep backwards-compatible comment field
        self.comment = Some(comment.to_string());

        use std::rc::Rc;

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

    /// TODO: doc
    pub fn to_code(&self) -> String {
        match &self.comment {
            Some(c) => format!("/* {} */\n{}", c, self.inner.to_code()),
            None => self.inner.to_code(),
        }
    }

    /// TODO: doc
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
#[derive(Clone, Debug)]
pub struct ScadObjectWrapperToDeprecated {
    pub(crate) inner: std::rc::Weak<ScadObjectImpl>,
    pub comment: Option<String>,
}

/// TODO: doc
impl ScadObjectWrapperToDeprecated {
    pub fn to_code(&self) -> String {
        match self.inner.upgrade() {
            Some(rc) => match &self.comment {
                Some(c) => format!("/* {} */\n{}", c, rc.to_code()),
                None => rc.to_code(),
            },
            _ => String::new(),
        }
    }

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
    pub fn to_code(&self) -> String {
        match self {
            Self::Object2D(v) => v.as_ref().to_code(),
            Self::Object3D(v) => v.as_ref().to_code(),
            Self::ObjectMixed(v) => v.as_ref().to_code(),
        }
    }
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

/// Small object-safe helper traits implemented by the concrete sentence enums
/// to allow `ScadObjectImpl` to call into existing repr code.
use std::any::Any;

pub trait ScadObjectRepr2D: Any {
    fn to_code(&self) -> String;
    /// Allow downcasting to concrete types implementing this trait.
    /// Implemented by concrete enum types (see impls below).
    fn as_any(&self) -> &dyn Any;
}
pub trait ScadObjectRepr3D: Any {
    fn to_code(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}
pub trait ScadObjectReprMixed: Any {
    fn to_code(&self) -> String;
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
    pub const fn new(child: Rc<ScadObjectImpl>, comment: String) -> Self {
        Self { child, comment }
    }

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
/// Backwards-compatible alias for the earlier runtime enum type.
// Backwards-compat alias removed: use `DimensionMarker` directly.

///
// Backwards-compat alias removed: use `DimensionMarker` directly.

/// Thin, typed wrappers over the generic `ScadObjectGeneric`<D>.
/// These provide a typed, ergonomic API surface while reusing the existing
/// runtime implementation under the hood.
///
/// Note: kept intentionally lightweight to remain non-breaking.
///
/// Public-facing wrapper types presented to library users. These are thin
/// wrappers around `ScadObjectGeneric`<D*> that expose a small, ergonomic API
/// (`to_code` / `into_untyped`). The concrete enums in `scad_2d/scad_3d/scad_mixed`
/// remain in their modules and are referenced via fully-qualified paths.
#[derive(Clone, Debug)]
pub struct ScadObject2D(pub(crate) ScadObjectGeneric<D2>);

impl ScadObject2D {
    /// Generate SCAD code for the object.
    pub fn to_code(&self) -> String {
        self.0.to_code()
    }

    /// Convert into the backwards-compatible untyped `ScadObject`.
    pub fn into_untyped(self) -> ScadObject {
        ScadObjectGeneric::from_impl(self.0.inner)
    }
}

impl From<ScadObject2D> for ScadObject {
    fn from(v: ScadObject2D) -> Self {
        v.into_untyped()
    }
}

impl ScadDisplay for ScadObject2D {
    fn repr_scad(&self) -> String {
        // Use the typed wrapper's to_code to produce the canonical SCAD
        // representation (including any comment).
        self.to_code()
    }
}

#[derive(Clone, Debug)]
pub struct ScadObject3D(pub(crate) ScadObjectGeneric<D3>);

impl ScadObject3D {
    /// Generate SCAD code for the object.
    pub fn to_code(&self) -> String {
        self.0.to_code()
    }

    /// Convert into the backwards-compatible untyped `ScadObject`.
    pub fn into_untyped(self) -> ScadObject {
        ScadObjectGeneric::from_impl(self.0.inner)
    }
}

impl From<ScadObject3D> for ScadObject {
    fn from(v: ScadObject3D) -> Self {
        v.into_untyped()
    }
}

impl ScadDisplay for ScadObject3D {
    fn repr_scad(&self) -> String {
        self.to_code()
    }
}

// Allow converting an untyped runtime ScadObject (ScadObjectGeneric<DMixed>) into
// a typed ScadObject3D by runtime-checking the inner variant. This enables ergonomic
// calls like Translate3D::build_with(...).apply_to(untyped_scad_object).
impl From<ScadObjectGeneric<DMixed>> for ScadObject3D {
    fn from(v: ScadObjectGeneric<DMixed>) -> Self {
        assert!(
            v.inner.get_type() == DimensionMarker::Object3D,
            "Modifier requires: Object3D"
        );
        Self(ScadObjectGeneric::<D3>::from_impl(v.inner))
    }
}

impl From<ScadObjectGeneric<DMixed>> for ScadObject2D {
    fn from(v: ScadObjectGeneric<DMixed>) -> Self {
        assert!(
            v.inner.get_type() == DimensionMarker::Object2D,
            "Modifier requires: Object2D"
        );
        Self(ScadObjectGeneric::<D2>::from_impl(v.inner))
    }
}

// Added From implementations for ScadObjectGeneric<D>
impl From<ScadObjectGeneric<DMixed>> for ScadObjectGeneric<D2> {
    fn from(v: ScadObjectGeneric<DMixed>) -> Self {
        assert!(
            v.inner.get_type() == DimensionMarker::Object2D,
            "Dimension mismatch: expected Object2D, got {:?}",
            v.inner.get_type()
        );
        Self::from_impl(v.inner)
    }
}

impl From<ScadObjectGeneric<DMixed>> for ScadObjectGeneric<D3> {
    fn from(v: ScadObjectGeneric<DMixed>) -> Self {
        assert!(
            v.inner.get_type() == DimensionMarker::Object3D,
            "Dimension mismatch: expected Object3D, got {:?}",
            v.inner.get_type()
        );
        Self::from_impl(v.inner)
    }
}

#[derive(Clone, Debug)]
pub struct ScadObjectMixed(pub(crate) ScadObjectGeneric<DMixed>);

impl ScadObjectMixed {
    /// Generate SCAD code for the object.
    pub fn to_code(&self) -> String {
        self.0.to_code()
    }

    /// Convert into the backwards-compatible untyped `ScadObject`.
    pub fn into_untyped(self) -> ScadObject {
        ScadObjectGeneric::from_impl(self.0.inner)
    }
}

impl From<ScadObjectMixed> for ScadObject {
    fn from(v: ScadObjectMixed) -> Self {
        v.into_untyped()
    }
}

// Added From implementations for concrete ScadObject2D and ScadObject3D to ScadObjectGeneric<D>
impl From<ScadObject2D> for ScadObjectGeneric<D2> {
    fn from(v: ScadObject2D) -> Self {
        v.0
    }
}

impl From<ScadObject3D> for ScadObjectGeneric<D3> {
    fn from(v: ScadObject3D) -> Self {
        v.0
    }
}

// Added From implementations for typed ScadObjectGeneric to untyped ScadObjectGeneric<DMixed>
impl From<ScadObjectGeneric<D2>> for ScadObjectGeneric<DMixed> {
    fn from(v: ScadObjectGeneric<D2>) -> Self {
        Self::from_impl(v.inner)
    }
}

impl From<ScadObjectGeneric<D3>> for ScadObjectGeneric<DMixed> {
    fn from(v: ScadObjectGeneric<D3>) -> Self {
        Self::from_impl(v.inner)
    }
}

impl ScadDisplay for ScadObjectMixed {
    fn repr_scad(&self) -> String {
        self.to_code()
    }
}

/// Implement conversion helpers and basic operators for untyped compatibility.
/// Note: heavy use of boxed trait objects simplifies the transition but can be
/// optimized later.

// Operators for the generic untyped ScadObject (ScadObjectGeneric<DMixed>)
// Provide Add/Sub/Mul so existing code using ScadObject values can use + - *
impl Add for ScadObjectGeneric<DMixed> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let left_rc = self.inner;
        let right_rc = rhs.inner;

        // If runtime dimension mismatch -> panic with message tests expect.
        assert!(
            left_rc.get_type() == right_rc.get_type(),
            "`{}` is not allowed",
            match (left_rc.get_type(), right_rc.get_type()) {
                (DimensionMarker::Object2D, DimensionMarker::Object3D) => {
                    "Object2D + Object3D"
                }
                (DimensionMarker::Object3D, DimensionMarker::Object2D) => {
                    "Object3D + Object2D"
                }
                _ => "Mismatched dimensions",
            }
        );

        match (&*left_rc, &*right_rc) {
            // Same-dimension 2D: flatten left-side unions/blocks when possible.
            (ScadObjectImpl::Object2D(_), ScadObjectImpl::Object2D(_)) => {
                let mut parts: Vec<ScadObjectImpl> = Vec::new();

                let mut extend_from = |rc: &Rc<ScadObjectImpl>| {
                    if let ScadObjectImpl::Object2D(inner_enum_rc) = &**rc {
                        if let Some(concrete) = inner_enum_rc
                            .as_any()
                            .downcast_ref::<crate::scad_2d::ScadObject2D>()
                        {
                            match concrete {
                                crate::scad_2d::ScadObject2D::Modifier(m) => {
                                    if let crate::scad_2d::ScadModifierBody2D::Union(_) = m.body {
                                        if let ScadObjectImpl::Object2D(child_enum_rc) = &*m.child {
                                            if let Some(child_concrete) =
                                                child_enum_rc
                                                    .as_any()
                                                    .downcast_ref::<crate::scad_2d::ScadObject2D>()
                                            {
                                                if let crate::scad_2d::ScadObject2D::Block(b) =
                                                    child_concrete
                                                {
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
                };

                extend_from(&left_rc); // flatten left
                extend_from(&right_rc); // append right as whole (do not flatten rhs)

                let block = crate::scad_2d::ScadBlock2D { objects: parts };
                let obj_enum = crate::scad_2d::ScadObject2D::Block(block);
                let rc_impl_child = Rc::new(ScadObjectImpl::Object2D(Rc::new(obj_enum)));
                let body = crate::scad_sentence::Union::new();
                let m =
                    crate::scad_2d::ScadModifier2D::try_new(body.into(), Rc::clone(&rc_impl_child))
                        .expect("Union modifier requires: Object2D");
                let o = crate::scad_2d::ScadObject2D::Modifier(m);
                let rc_impl = Rc::new(ScadObjectImpl::Object2D(Rc::new(o)));
                Self::from_impl(rc_impl)
            }

            // Same-dimension 3D: flatten left-side unions/blocks when possible.
            (ScadObjectImpl::Object3D(_), ScadObjectImpl::Object3D(_)) => {
                let mut parts: Vec<ScadObjectImpl> = Vec::new();

                let mut extend_from = |rc: &Rc<ScadObjectImpl>| {
                    if let ScadObjectImpl::Object3D(inner_enum_rc) = &**rc {
                        if let Some(concrete) = inner_enum_rc
                            .as_any()
                            .downcast_ref::<crate::scad_3d::ScadObject3D>()
                        {
                            match concrete {
                                crate::scad_3d::ScadObject3D::Modifier(m) => {
                                    if let crate::scad_3d::ScadModifierBody3D::Union(_) = m.body {
                                        if let ScadObjectImpl::Object3D(child_enum_rc) = &*m.child {
                                            if let Some(child_concrete) =
                                                child_enum_rc
                                                    .as_any()
                                                    .downcast_ref::<crate::scad_3d::ScadObject3D>()
                                            {
                                                if let crate::scad_3d::ScadObject3D::Block(b) =
                                                    child_concrete
                                                {
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
                };

                extend_from(&left_rc);
                extend_from(&right_rc);

                let block = crate::scad_3d::ScadBlock3D { objects: parts };
                let obj_enum = crate::scad_3d::ScadObject3D::Block(block);
                let rc_impl_child = Rc::new(ScadObjectImpl::Object3D(Rc::new(obj_enum)));
                let body = crate::scad_sentence::Union::new();
                let m =
                    crate::scad_3d::ScadModifier3D::try_new(body.into(), Rc::clone(&rc_impl_child))
                        .expect("Union modifier requires: Object3D");
                let o = crate::scad_3d::ScadObject3D::Modifier(m);
                let rc_impl = Rc::new(ScadObjectImpl::Object3D(Rc::new(o)));
                Self::from_impl(rc_impl)
            }

            // Mixed or cross-dimension handled as mixed union (shouldn't happen due to earlier panic)
            _ => {
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
                Self::from_impl(rc_impl)
            }
        }
    }
}

impl Sub for ScadObjectGeneric<DMixed> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let left_rc = self.inner;
        let right_rc = rhs.inner;

        // dimension mismatch -> panic with expected message
        assert!(
            left_rc.get_type() == right_rc.get_type(),
            "`{}` is not allowed",
            match (left_rc.get_type(), right_rc.get_type()) {
                (DimensionMarker::Object2D, DimensionMarker::Object3D) => {
                    "Object2D - Object3D"
                }
                (DimensionMarker::Object3D, DimensionMarker::Object2D) => {
                    "Object3D - Object2D"
                }
                _ => "Mismatched dimensions",
            }
        );

        match (&*left_rc, &*right_rc) {
            (ScadObjectImpl::Object2D(_), ScadObjectImpl::Object2D(_)) => {
                // For difference, flatten left-side differences or blocks into parts,
                // but do not flatten the rhs (preserve nesting on the right).
                let mut parts: Vec<ScadObjectImpl> = Vec::new();

                let mut extend_from = |rc: &Rc<ScadObjectImpl>| {
                    if let ScadObjectImpl::Object2D(inner_enum_rc) = &**rc {
                        if let Some(concrete) = inner_enum_rc
                            .as_any()
                            .downcast_ref::<crate::scad_2d::ScadObject2D>()
                        {
                            match concrete {
                                crate::scad_2d::ScadObject2D::Modifier(m) => {
                                    if let crate::scad_2d::ScadModifierBody2D::Difference(_) =
                                        m.body
                                    {
                                        if let ScadObjectImpl::Object2D(child_enum_rc) = &*m.child {
                                            if let Some(child_concrete) =
                                                child_enum_rc
                                                    .as_any()
                                                    .downcast_ref::<crate::scad_2d::ScadObject2D>()
                                            {
                                                if let crate::scad_2d::ScadObject2D::Block(b) =
                                                    child_concrete
                                                {
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
                };

                extend_from(&left_rc);
                parts.push((*right_rc).clone());

                let block = crate::scad_2d::ScadBlock2D { objects: parts };
                let obj_enum = crate::scad_2d::ScadObject2D::Block(block);
                let rc_child = Rc::new(ScadObjectImpl::Object2D(Rc::new(obj_enum)));
                let body = crate::scad_sentence::Difference::new();
                let m = crate::scad_2d::ScadModifier2D::try_new(body.into(), Rc::clone(&rc_child))
                    .expect("Difference modifier requires: Object2D");
                let o = crate::scad_2d::ScadObject2D::Modifier(m);
                let rc_impl = Rc::new(ScadObjectImpl::Object2D(Rc::new(o)));
                Self::from_impl(rc_impl)
            }

            (ScadObjectImpl::Object3D(_), ScadObjectImpl::Object3D(_)) => {
                let mut parts: Vec<ScadObjectImpl> = Vec::new();

                let mut extend_from = |rc: &Rc<ScadObjectImpl>| {
                    if let ScadObjectImpl::Object3D(inner_enum_rc) = &**rc {
                        if let Some(concrete) = inner_enum_rc
                            .as_any()
                            .downcast_ref::<crate::scad_3d::ScadObject3D>()
                        {
                            match concrete {
                                crate::scad_3d::ScadObject3D::Modifier(m) => {
                                    if let crate::scad_3d::ScadModifierBody3D::Difference(_) =
                                        m.body
                                    {
                                        if let ScadObjectImpl::Object3D(child_enum_rc) = &*m.child {
                                            if let Some(child_concrete) =
                                                child_enum_rc
                                                    .as_any()
                                                    .downcast_ref::<crate::scad_3d::ScadObject3D>()
                                            {
                                                if let crate::scad_3d::ScadObject3D::Block(b) =
                                                    child_concrete
                                                {
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
                };

                extend_from(&left_rc);
                parts.push((*right_rc).clone());

                let block = crate::scad_3d::ScadBlock3D { objects: parts };
                let obj_enum = crate::scad_3d::ScadObject3D::Block(block);
                let rc_child = Rc::new(ScadObjectImpl::Object3D(Rc::new(obj_enum)));
                let body = crate::scad_sentence::Difference::new();
                let m = crate::scad_3d::ScadModifier3D::try_new(body.into(), Rc::clone(&rc_child))
                    .expect("Difference modifier requires: Object3D");
                let o = crate::scad_3d::ScadObject3D::Modifier(m);
                let rc_impl = Rc::new(ScadObjectImpl::Object3D(Rc::new(o)));
                Self::from_impl(rc_impl)
            }

            _ => {
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
                Self::from_impl(rc_impl)
            }
        }
    }
}

impl Mul for ScadObjectGeneric<DMixed> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let left_rc = self.inner;
        let right_rc = rhs.inner;

        // dimension mismatch -> panic with expected message
        assert!(
            left_rc.get_type() == right_rc.get_type(),
            "`{}` is not allowed",
            match (left_rc.get_type(), right_rc.get_type()) {
                (DimensionMarker::Object2D, DimensionMarker::Object3D) => {
                    "Object2D * Object3D"
                }
                (DimensionMarker::Object3D, DimensionMarker::Object2D) => {
                    "Object3D * Object2D"
                }
                _ => "Mismatched dimensions",
            }
        );

        match (&*left_rc, &*right_rc) {
            (ScadObjectImpl::Object2D(_), ScadObjectImpl::Object2D(_)) => {
                let mut parts: Vec<ScadObjectImpl> = Vec::new();

                let mut extend_from = |rc: &Rc<ScadObjectImpl>| {
                    if let ScadObjectImpl::Object2D(inner_enum_rc) = &**rc {
                        if let Some(concrete) = inner_enum_rc
                            .as_any()
                            .downcast_ref::<crate::scad_2d::ScadObject2D>()
                        {
                            match concrete {
                                crate::scad_2d::ScadObject2D::Modifier(m) => {
                                    if let crate::scad_2d::ScadModifierBody2D::Intersection(_) =
                                        m.body
                                    {
                                        if let ScadObjectImpl::Object2D(child_enum_rc) = &*m.child {
                                            if let Some(child_concrete) =
                                                child_enum_rc
                                                    .as_any()
                                                    .downcast_ref::<crate::scad_2d::ScadObject2D>()
                                            {
                                                if let crate::scad_2d::ScadObject2D::Block(b) =
                                                    child_concrete
                                                {
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
                };

                extend_from(&left_rc);
                extend_from(&right_rc);

                let block = crate::scad_2d::ScadBlock2D { objects: parts };
                let obj_enum = crate::scad_2d::ScadObject2D::Block(block);
                let rc_child = Rc::new(ScadObjectImpl::Object2D(Rc::new(obj_enum)));
                let body = crate::scad_sentence::Intersection::new();
                let m = crate::scad_2d::ScadModifier2D::try_new(body.into(), Rc::clone(&rc_child))
                    .expect("Intersection modifier requires: Object2D");
                let o = crate::scad_2d::ScadObject2D::Modifier(m);
                let rc_impl = Rc::new(ScadObjectImpl::Object2D(Rc::new(o)));
                Self::from_impl(rc_impl)
            }

            (ScadObjectImpl::Object3D(_), ScadObjectImpl::Object3D(_)) => {
                let mut parts: Vec<ScadObjectImpl> = Vec::new();

                let mut extend_from = |rc: &Rc<ScadObjectImpl>| {
                    if let ScadObjectImpl::Object3D(inner_enum_rc) = &**rc {
                        if let Some(concrete) = inner_enum_rc
                            .as_any()
                            .downcast_ref::<crate::scad_3d::ScadObject3D>()
                        {
                            match concrete {
                                crate::scad_3d::ScadObject3D::Modifier(m) => {
                                    if let crate::scad_3d::ScadModifierBody3D::Intersection(_) =
                                        m.body
                                    {
                                        if let ScadObjectImpl::Object3D(child_enum_rc) = &*m.child {
                                            if let Some(child_concrete) =
                                                child_enum_rc
                                                    .as_any()
                                                    .downcast_ref::<crate::scad_3d::ScadObject3D>()
                                            {
                                                if let crate::scad_3d::ScadObject3D::Block(b) =
                                                    child_concrete
                                                {
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
                };

                extend_from(&left_rc);
                extend_from(&right_rc);

                let block = crate::scad_3d::ScadBlock3D { objects: parts };
                let obj_enum = crate::scad_3d::ScadObject3D::Block(block);
                let rc_child = Rc::new(ScadObjectImpl::Object3D(Rc::new(obj_enum)));
                let body = crate::scad_sentence::Intersection::new();
                let m = crate::scad_3d::ScadModifier3D::try_new(body.into(), Rc::clone(&rc_child))
                    .expect("Intersection modifier requires: Object3D");
                let o = crate::scad_3d::ScadObject3D::Modifier(m);
                let rc_impl = Rc::new(ScadObjectImpl::Object3D(Rc::new(o)));
                Self::from_impl(rc_impl)
            }

            _ => {
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
                Self::from_impl(rc_impl)
            }
        }
    }
}
