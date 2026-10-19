//! 🪆️ Artifact children and their records: `ArtifactChild`, child references and retained parts, local text owners,
//! and the DSL records of children, owner references and links that composition, codecs and the engine share.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
#[path = "📥️lifted/♻️retirement/🦀️.rs"]
pub mod lifted_retirement;
#[path = "🧬️retained-clone/🦀️.rs"]
pub mod artifact_child_retained_clone;
use semio_framework_os_spr::command::ArtifactMessageSource;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use std::marker::PhantomData;
use std::sync::Arc;
use semio_framework_os_store_envelope::owner::OwnerRef;
use semio_framework_os_store_link_schema::ArtifactLink;
use semio_framework_os_pack::pack_rt;
use semio_framework_dsl_record::RecordSpecProducer;
use semio_framework_pack_json::{ArtifactCanonicalJsonNode, ArtifactCanonicalJsonText, ArtifactCanonicalJsonTree};

#[path="🛬️receive/🦀️.rs"]
mod receiving;
pub use receiving::ArtifactChildNativeReceiver;

// 🧩️ Composable-vs-referenceable artifact primitives (ticket `26/08/12/UNIFIED-COMPOSABLE-
// ARTIFACT-SYSTEM` `📓️design-full-plan.md` §"1. Kernel primitives"). Two structurally distinct
// relationships between artifacts:
// - a **CHILD** (`ArtifactChild<S>` on the parent's snapshot / `OwnerRef` on the child's own
//   envelope): the parent creates and deletes it, it has exactly one owner, no pin, no
//   independent lifecycle, and nests inline in UI — but it is still its OWN `ArtifactEnvelope`
//   with its own `ArtifactVcs` history and store, never an inline subtree (this crate's own doc
//   comment / the design doc's "Child = own envelope" decision). Consequently a parent's diff
//   NEVER embeds a child diff.
// - a **LINK** (`ArtifactLink`/`LinkPin`): an independent lifecycle of its own, a PIN (so it can
//   be frozen to a specific `Head`/`Checkpoint`/content-addressed `Snapshot`), and renders as a
//   chip, never nested inline.
//
// `ArtifactRefs` lets a snapshot type declare its own children/links (defaulted to none, so a
// leaf artifact needs zero boilerplate); `LinkResolver`/`ChildStoreFactory` are the host-side
// seams that turn those declarations into real bytes/stores. `CompositionCoordinator` (its own
// `🔖️CompositionCoordinator` region, after `🔖️Space` below) orchestrates atomic multi-store
// dispatch across a parent and its children on top of these types.

/// 🧸️ Ownership handle a parent SNAPSHOT embeds for one owned child slot (e.g. a plugin's
/// generated field `mesh_child: ArtifactChild<MeshSnapshot>`). `S` is a compile-time-only phantom
/// naming which snapshot type the child is expected to materialize as — it never appears on the
/// wire and never constrains this type's own capabilities. `Clone`/`Debug`/`PartialEq` are
/// hand-implemented below (not `#[derive(..)]`'d): deriving them on a `PhantomData<S>`-carrying
/// struct adds an unwanted `S: Trait` bound to every impl even though `S` never appears in any
/// stored/compared data, so those three derives would silently fail to work "for any `S`" (the
/// task's own requirement) the moment a caller picked an `S` that itself didn't implement
/// `Clone`/`Debug`/`PartialEq`. `Serialize`/`Deserialize` avoid the same trap via `#[serde(bound =
/// "")]` instead of a hand impl — the standard serde idiom for a `PhantomData<S>` field, since
/// serde's derive macro honors an explicit empty bound list rather than inferring one from `S`.
/// `local_owner` is a serialization-skipped, immutable materialization retained by this exact
/// handle; it is intentionally absent from equality, debug, DSL, pack, and JSON identity. No pin,
/// no independent lifecycle: see the region doc's CHILD-vs-LINK split.
pub struct ArtifactChildLocalText(Arc<str>);

/// 🚫️ Why an owned child's local materialization cannot be read at a typed boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactChildMaterializationError {
    Absent,
    WrongType,
}

impl std::fmt::Display for ArtifactChildMaterializationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Absent => formatter.write_str("artifact child is not materialized"),
            Self::WrongType => formatter.write_str("artifact child materialization has a different exact type"),
        }
    }
}

impl std::error::Error for ArtifactChildMaterializationError {}

pub struct ArtifactChild<S> {
    pub child_id: String,
    pub target: semio_framework_artifact_reference::ArtifactRef,
    pub local_owner: Option<Arc<dyn std::any::Any + Send + Sync>>,
    pub _snapshot: PhantomData<S>,
}

/// 🪆️ The same child headers and optional typed local root after an exhaustive ownership transfer.
pub struct ArtifactChildRetainedParts<T> {
    pub child_id:String,
    pub target:semio_framework_artifact_reference::ArtifactRef,
    pub local_owner:Option<Arc<T>>,
}

impl<S> ArtifactChild<S> {
    /// 📤️ Moves all original fields or returns the same untouched child when its local type differs.
    pub fn into_typed_retained_parts<T:std::any::Any+Send+Sync>(self)->Result<ArtifactChildRetainedParts<T>,Self>{
        if self.local_owner.as_ref().is_some_and(|source|!source.is::<T>()){return Err(self)}
        let Self{child_id,target,local_owner,_snapshot:_}=self;
        let local_owner=local_owner.map(|source|Arc::downcast::<T>(source).unwrap_or_else(|_|unreachable!("original local type was borrowed before transfer")));
        Ok(ArtifactChildRetainedParts{child_id,target,local_owner})
    }
    /// 🏗️ Constructs a handle, threading the phantom marker for the caller.
    pub fn new(child_id: String, target: semio_framework_artifact_reference::ArtifactRef) -> Self {
        Self { child_id, target, local_owner: None, _snapshot: PhantomData }
    }

    /// 🪆 Attaches an immutable local-only materialization to this owned child handle.
    pub fn with_local_owner<T: std::any::Any + Send + Sync>(mut self, owner: Arc<T>) -> Self {
        self.local_owner = Some(owner);
        self
    }

    /// 🪆 Replaces this handle's immutable local-only materialization owner.
    pub fn set_local_owner<T: std::any::Any + Send + Sync>(&mut self, owner: Arc<T>) {
        self.local_owner = Some(owner);
    }

    /// 🧵 Retains a type-checked local-only materialization without cloning its payload.
    pub fn local_owner<T: std::any::Any + Send + Sync>(&self) -> Option<Arc<T>> {
        Arc::downcast::<T>(self.local_owner.clone()?).ok()
    }

    /// 📤️ Requires the exact local materialization at content-reading boundaries. Exporters must
    /// propagate this failure instead of serializing the handle or substituting default content.
    pub fn require_local_owner<T: std::any::Any + Send + Sync>(&self) -> Result<Arc<T>, ArtifactChildMaterializationError> {
        let owner = self.local_owner.clone().ok_or(ArtifactChildMaterializationError::Absent)?;
        Arc::downcast::<T>(owner).map_err(|_| ArtifactChildMaterializationError::WrongType)
    }

    /// 📤️ Transfers the exact typed local owner; a type mismatch retains the untouched owner.
    pub fn take_local_owner<T: std::any::Any + Send + Sync>(&mut self) -> Result<Option<Arc<T>>, &'static str> {
        let Some(owner) = self.local_owner.take() else {
            return Ok(None);
        };
        match Arc::downcast::<T>(owner) {
            Ok(owner) => Ok(Some(owner)),
            Err(owner) => {
                self.local_owner = Some(owner);
                Err("artifact child local owner has a different exact type")
            }
        }
    }

    /// 🪆 Attaches immutable local-only text to this owned child handle.
    pub fn with_local_text(mut self, text: Arc<str>) -> Self {
        self.local_owner = Some(Arc::new(ArtifactChildLocalText(text)));
        self
    }

    /// 🪆 Replaces this handle's immutable local-only text owner.
    pub fn set_local_text(&mut self, text: Arc<str>) {
        self.local_owner = Some(Arc::new(ArtifactChildLocalText(text)));
    }

    /// 🔎 Borrows this handle's local-only text when the child has been materialized.
    pub fn local_text(&self) -> Option<&str> {
        self.local_owner.as_deref()?.downcast_ref::<ArtifactChildLocalText>().map(|owner| owner.0.as_ref())
    }

    /// 🧵 Retains this handle's local-only text owner without copying its bytes.
    pub fn local_text_owner(&self) -> Option<Arc<str>> {
        self.local_owner.as_deref()?.downcast_ref::<ArtifactChildLocalText>().map(|owner| owner.0.clone())
    }

    /// 🪪️ Drops the compile-time-only `S` phantom, producing the type-erased projection
    /// `ArtifactRefs::child_refs` returns.
    pub async fn to_child_ref(&self, slot: &str) -> ChildRef {
        ChildRef { slot: slot.to_string(), child_id: self.child_id.clone(), target: self.target.clone() }
    }
}

impl<S> Clone for ArtifactChild<S> {
    fn clone(&self) -> Self {
        Self { child_id: self.child_id.clone(), target: self.target.clone(), local_owner: self.local_owner.clone(), _snapshot: PhantomData }
    }
}

impl<S> semio_framework_schema_composition::ChildFieldRefs for ArtifactChild<S> {
    const MANY: bool = false;
    fn visit_child_field<'a, V: semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self, slot: &'static str, visitor: &mut V) -> Result<(), V::Error> {
        visitor.step()?;
        visitor.child(
            slot,
            semio_framework_schema_composition::ChildRefFields {
                child_id: &self.child_id,
                artifact_id: &self.target.artifact_id,
                artifact_kind: &self.target.dialect.artifact_kind,
                standard: &self.target.dialect.standard,
                subset: &self.target.dialect.subset,
            },
        )
    }
}

impl<S> std::fmt::Debug for ArtifactChild<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArtifactChild").field("child_id", &self.child_id).field("target", &self.target).finish()
    }
}

impl<S> PartialEq for ArtifactChild<S> {
    fn eq(&self, other: &Self) -> bool {
        self.child_id == other.child_id && self.target == other.target
    }
}

/// 🌉️ Hand-written, not derived: `ArtifactChild<S>`'s `local_owner`/`_snapshot` fields are
/// serialization-skipped (see the struct's own doc comment), a shape `#[derive(ToValue,
/// FromValue)]` does not support — mirrors the pre-existing `#[serde(skip)]` hand-treatment,
/// round-tripping only `child_id`/`target`.
impl<S> ToValue for ArtifactChild<S> {
    fn to_value(&self) -> DslValue {
        DslValue::object([("childId".to_string(), ToValue::to_value(&self.child_id)), ("target".to_string(), ToValue::to_value(&self.target))])
    }
    fn to_value_controlled(&self, control: &mut protocol::value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
        control.scoped_depth(64, |control| {
            control.scoped_stage(|control| {
                control.begin_stage(2)?;
                let mut fields = protocol::value::DecodedValue::new(control.allocate_vec::<(String, DslValue)>(2)?, |fields: Vec<(String, DslValue)>| {
                    for (_, value) in fields {
                        <DslValue as FromValue>::retire_decoded(value);
                    }
                });
                let key = control.copy_text("childId")?;
                let child_id = ToValue::to_value_controlled(&self.child_id, control).map_err(|error| error.under("childId"))?;
                fields.get_mut().push((key, child_id));
                control.step()?;
                let key = control.copy_text("target")?;
                let target = ToValue::to_value_controlled(&self.target, control).map_err(|error| error.under("target"))?;
                fields.get_mut().push((key, target));
                control.step()?;
                Ok(DslValue::Object(fields.take()))
            })
        })
    }
}

/// 🌲️ Canonical wire of the persisted child identity: `childId` then `target`; the local owner is not part of the wire.
impl<S: Sync + 'static> ArtifactCanonicalJsonTree for ArtifactChild<S> {
    fn canonical_tree_node(&self) -> Result<ArtifactCanonicalJsonNode<'_>, ValueError> { Ok(ArtifactCanonicalJsonNode::Object(2)) }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn ArtifactCanonicalJsonTree, ValueError> {
        match ordinal { 0 => Ok(&self.child_id), 1 => Ok(&self.target), _ => Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical artifact child ordinal is absent")) }
    }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<ArtifactCanonicalJsonText<'_>, ValueError> {
        match ordinal { 0 => Ok("childId".into()), 1 => Ok("target".into()), _ => Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical artifact child key is absent")) }
    }
}

impl<S> FromValue for ArtifactChild<S> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(fields) = value else {
            return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for ArtifactChild, found {value:?}")));
        };
        let mut child_id = None;
        let mut target = None;
        for (key, entry) in fields {
            match key.as_str() {
                "childId" => child_id = Some(String::from_value(entry).map_err(|e| e.under("childId"))?),
                "target" => target = Some(semio_framework_artifact_reference::ArtifactRef::from_value(entry).map_err(|e| e.under("target"))?),
                _ => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("ArtifactChild has unknown field {key}"))),
            }
        }
        Ok(ArtifactChild {
            child_id: child_id.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "ArtifactChild missing childId"))?,
            target: target.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "ArtifactChild missing target"))?,
            local_owner: None,
            _snapshot: PhantomData,
        })
    }
    fn from_value_controlled(value: &DslValue, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        let DslValue::Object(fields) = value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for ArtifactChild")) };
        control.scoped_depth(64, |control| {
            control.scoped_stage(|control| {
                control.begin_stage(fields.len())?;
                let mut child_id = None;
                let mut target = None;
                for (key, entry) in fields {
                    match key.as_str() {
                        "childId" => child_id = Some(String::from_value_controlled(entry, control).map_err(|error| error.under("childId"))?),
                        "target" => target = Some(semio_framework_artifact_reference::ArtifactRef::from_value_controlled(entry, control).map_err(|error| error.under("target"))?),
                        _ => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "ArtifactChild has an unknown field")),
                    }
                    control.step()?;
                }
                control.checkpoint()?;
                Ok(Self {
                    child_id: child_id.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "ArtifactChild missing childId"))?,
                    target: target.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "ArtifactChild missing target"))?,
                    local_owner: None,
                    _snapshot: PhantomData,
                })
            })
        })
    }
}

/// 🪪️ Type-erased projection of one `ArtifactChild<S>` field, dropping the compile-time-only
/// `S` phantom so `ArtifactRefs::child_refs` can return a single homogeneous `Vec` across however
/// many differently-`S`-typed child slots a snapshot declares.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ChildRef {
    pub slot: String,
    pub child_id: String,
    pub target: semio_framework_artifact_reference::ArtifactRef,
}

/// 🧬️ Hand-crafted `crate::os_dsl::DslField` records for `ArtifactChild`/`OwnerRef`/`ArtifactLink`/
/// `LinkPin` (P6 idiom, the same one `BackboneMessage`'s `OpText`/`OpBinary` impls near
/// `🔖️Backbone` below use) — NOT new `Shape` variants: the design doc's deviation D1 deliberately
/// keeps `Shape` closed (it is exhaustively matched in ~20 files) and represents every new value
/// type here as an ordinary `Shape::Record`. Targets use the canonical literal
/// `semio_framework_artifact_reference::ArtifactRef` record so each owned reference component remains independent.
// 🚫️async: E4 fn-pointer slot — value stored in `Shape::Record(fn() -> RecordSpec)`
impl<S> semio_framework_dsl_record::BorrowedDslRecord for ArtifactChild<S>{
 const RECORD:semio_framework_dsl_record::BorrowedRecordSpec=semio_framework_dsl_record::BorrowedRecordSpec{keyword:None,layout:semio_framework_dsl_record::RecordLayout::Inline,fields:&[
 semio_framework_dsl_record::BorrowedFieldSpec::new(0,"child_id",semio_framework_dsl_record::BorrowedShape::Text),
 semio_framework_dsl_record::BorrowedFieldSpec::new(1,"target",<semio_framework_artifact_reference::ArtifactRef as semio_framework_dsl_record::BorrowedDslField>::SHAPE)]};
}

impl<S> semio_framework_dsl_record::BorrowedDslField for ArtifactChild<S>{const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<Self>);}

pub fn artifact_child_spec() -> semio_framework_dsl_record::RecordSpec {
    semio_framework_dsl_record::RecordSpec::new(
        None,
        semio_framework_dsl_record::RecordLayout::Inline,
        vec![semio_framework_dsl_record::FieldSpec::new(0, "child_id", semio_framework_dsl_record::Shape::Text), semio_framework_dsl_record::FieldSpec::new(1, "target", <semio_framework_artifact_reference::ArtifactRef as semio_framework_dsl_record::DslField>::shape())],
    )
}

pub fn artifact_child_spec_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::RecordSpec, ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(2)?;
        let mut fields = control.allocate_vec::<semio_framework_dsl_record::FieldSpec>(2)?;
        fields.push(semio_framework_dsl_record::producer::field(0, "child_id", semio_framework_dsl_record::Shape::Text, control)?);
        control.step()?;
        let target = <semio_framework_artifact_reference::ArtifactRef as semio_framework_dsl_record::DslField>::shape_controlled(control)?;
        fields.push(semio_framework_dsl_record::producer::field(1, "target", target, control)?);
        control.step()?;
        semio_framework_dsl_record::producer::record(None, semio_framework_dsl_record::RecordLayout::Inline, fields, control)
    })
}

pub fn artifact_child_spec_producer() -> RecordSpecProducer {
    RecordSpecProducer { ordinary: artifact_child_spec, decoding: |control| artifact_child_spec_controlled(control), encoding: |control| artifact_child_spec_controlled(control) }
}

// 🚫️async: E1 pure — record conversion with no suspension point, consumed by the sync
// `DslField::to_value` impl below. See R9.
pub fn artifact_child_to_record<S>(child: &ArtifactChild<S>) -> semio_framework_dsl_record::RecordValue {
    let mut record = semio_framework_dsl_record::RecordValue::default();
    record.fields.insert(0, semio_framework_dsl_record::FieldValue::Text(child.child_id.clone()));
    record.fields.insert(1, <semio_framework_artifact_reference::ArtifactRef as semio_framework_dsl_record::DslField>::to_value(&child.target));
    record
}

// 🚫️async: E1 pure — same sync consumer as `artifact_child_to_record`. See R9.
/// 🧱️ Constructs the persisted child identity and literal target without a local owner.
pub fn artifact_child_from_record<S>(record: &semio_framework_dsl_record::RecordValue) -> Result<ArtifactChild<S>, String> {
    if record.fields.len() != 2 || record.fields.keys().any(|id| *id > 1) {
        return Err("artifact child requires its two literal fields".into());
    }
    let child_id = match record.get(0) {
        Some(semio_framework_dsl_record::FieldValue::Text(s)) => s.clone(),
        other => return Err(format!("expected child_id, found {other:?}")),
    };
    let target = <semio_framework_artifact_reference::ArtifactRef as semio_framework_dsl_record::DslField>::from_value(record.get(1).ok_or("missing artifact child target")?)?;
    Ok(ArtifactChild { child_id, target, local_owner: None, _snapshot: PhantomData })
}

impl<S> semio_framework_dsl_record::DslField for ArtifactChild<S> {
    fn projection_view(&self,path:&[usize])->Result<semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>,ValueError>{
        use semio_framework_dsl_record::{DslField,native_encoding::{FieldProjectionView as V,projection_path_error}};
        match path{[]=>Ok(V::Record(&[0,1])),[0,rest @ ..]=>DslField::projection_view(&self.child_id,rest),[1,rest @ ..]=>DslField::projection_view(&self.target,rest),_=>Err(projection_path_error())}
    }
    fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{
        use semio_framework_dsl_record::{DslField,native_encoding::projection_path_error};
        match path{[0,rest @ ..]=>DslField::projection_key(&self.child_id,rest,index),[1,rest @ ..]=>DslField::projection_key(&self.target,rest,index),_=>Err(projection_path_error())}
    }
    /// 🚫️async: E4 — see `DslField::shape`'s tag on the trait.
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Record(artifact_child_spec_producer())
    }
    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, ValueError> {
        control.checkpoint()?;
        Ok(semio_framework_dsl_record::Shape::Record(artifact_child_spec_producer()))
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Record(artifact_child_to_record(self))
    }
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_dsl_record::FieldValue, ValueError> {
        <Self as semio_framework_dsl_record::DslField>::to_record_controlled(self, control).map(semio_framework_dsl_record::FieldValue::Record)
    }
    fn to_record_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_dsl_record::RecordValue, ValueError> {
        control.scoped_depth(64, |control| {
            control.scoped_stage(|control| {
                control.begin_stage(2)?;
                let mut record = semio_framework_dsl_record::native_encoding::EncodedRecord::new(2, control)?;
                record.insert(0, semio_framework_dsl_record::FieldValue::Text(control.copy_text(&self.child_id)?))?;
                control.step()?;
                record.insert(1, <semio_framework_artifact_reference::ArtifactRef as semio_framework_dsl_record::DslField>::to_value_controlled(&self.target, control)?)?;
                control.step()?;
                Ok(record.take())
            })
        })
    }

    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Record(record) => artifact_child_from_record(record),
            other => Err(format!("expected Record, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &semio_framework_dsl_record::FieldValue, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        let semio_framework_dsl_record::FieldValue::Record(record) = value else {
            return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected artifact child Record"));
        };
        Self::from_record_controlled(record, control)
    }
    fn from_record_controlled(record: &semio_framework_dsl_record::RecordValue, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        control.scoped_stage(|control| {
            control.begin_stage(2)?;
            if record.fields.len() != 2 || record.fields.keys().any(|id| *id > 1) {
                return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "artifact child requires its two literal fields"));
            }
            let Some(semio_framework_dsl_record::FieldValue::Text(child_id)) = record.get(0) else {
                return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected artifact child identity Text"));
            };
            let target = record.get(1).ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing artifact child target"))?;
            let child_id = control.copy_text(child_id)?;
            control.step()?;
            let target = <semio_framework_artifact_reference::ArtifactRef as semio_framework_dsl_record::DslField>::from_value_controlled(target, control)?;
            control.step()?;
            Ok(Self { child_id, target, local_owner: None, _snapshot: PhantomData })
        })
    }
}
