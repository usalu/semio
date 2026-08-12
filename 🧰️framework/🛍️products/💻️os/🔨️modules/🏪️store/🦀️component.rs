//! 🗃️ Local-first, non-blocking, client-side, in-memory document store — hot-swappable
//! backbones (`temp://`/`file://`/`folder://`/`remote://`) layered on `vcs`'s version-graph
//! algebra. `ArtifactStore`/`Backbone`/`BlobStore`/`Space`/the serialization seam
//! (`ArtifactDsl`/`ArtifactPack`/`pack_rt`/`ArtifactCodec`) all live here — apps depend on
//! `store`, never on `vcs`/`pack`/`dsl_core` directly (moved from `vcs/rs/lib.rs` by ticket
//! `26/07/28/EXTRACT-STORE-INTO-ITS-OWN-TECHNOLOGY`).
//!
//! Mutation of document state is sealed: public writes go through [`ArtifactStore::dispatch`]
//! (`Apply`/`IngestRemote`/`PruneDrafts`/…) or [`ArtifactStore::reset`]. Envelope fields stay
//! `pub` for serde/plugins; treat them as read-mostly and prefer [`ArtifactEnvelopeView`].

// The `crate::os_dsl::DslArtifact`/`crate::os_dsl::DslOps` derive macros emit `::crate::os_store::ArtifactDsl`/`::crate::os_store::OpText`
// paths (see `dsl/derive/rs/lib.rs`), which only resolve for crates that depend on `store` as an
// external crate — every real consumer, INCLUDING this crate's own `.ops` header grammar
// (`OpsHeaderLine` in `🔖️TextFormat` below, derived on the engine directly) as well as its in-crate
// test fixtures (a crate is never its own dependency otherwise). `// extern crate self removed after merge` is
// the same fix `vcs`/`dsl` use for their own in-crate derive usage: it makes `::store` resolve to
// this crate even when the derive is exercised in-crate.
// extern crate self removed after merge

use crate::os_dsl::{from_dsl_value, to_dsl_value, DslOps, DslRecord, DslValue};
use crate::os_spr::{Edit, OpBinary, OpText, Mutation, MutationDiff, MutationMeta, ReconcileReport};
use crate::os_spr::{ActorId, ArtifactId, HybridLogicalTimestamp, MutationId, SchemaId, UndoPolicy};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

// 🗃️ `store`'s facade over `vcs`'s version-graph algebra — apps that depend on `store` reach
// `Author`/`Change`/`Checkpoint`/`Alternative`/`VcsError`/etc through this crate, never through
// `vcs` directly (see the crate doc comment above).
pub use crate::os_vcs::{
    apply_collection_mutation, apply_mutation, collection_diff_from_mutation, content_addressed_checkpoint_id, content_addressed_entity_id, create_document_vcs_id, edit_scoped_id, inverse_collection_mutation, mint_alternative_id, mint_change_id, mint_edit_id, mint_mutation_id, Alternative, Author, Change, Checkpoint, CollectionDiff, CollectionMutation,
    ArtifactVcs, Identified, ItemPatch, Patchable, VcsError,
};

//#region 🔖️Schemas
/// @emoji 🔗️ Identifies the channel a document synchronizes through, when one is attached.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactBackboneRef {
    pub uri: String,
}

/// @emoji 🔗️ Builds a backbone reference from a channel URI.
pub fn document_backbone_ref(uri: &str) -> ArtifactBackboneRef {
    ArtifactBackboneRef { uri: uri.to_string() }
}

/// @emoji 🎯️ Undo/redo/checkout position — the store-facing twin of `crate::os_spr::HistoryCursor`.
/// Carries the FULL applied-edit list (not just the tail edit id): an edit undone mid-history
/// precedes later-applied edits in file order, and the redo stack can contain edits in any order
/// relative to `applied_edit_ids` — a single marker id cannot represent that. `checkpoint_id`
/// mirrors `ArtifactStore::current_checkpoint_id`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactCursor {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applied_edit_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub redo_edit_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactEnvelope<P, Mutation> {
    pub schema: String,
    pub id: String,
    pub vcs: ArtifactVcs<P, Mutation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backbone: Option<ArtifactBackboneRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_alternative_id: Option<String>,
    /// @emoji 🎯️ Undo/redo/checkout position, present only once a store has synced it (see
    /// `ArtifactStore::sync_cursor`) — absent for a freshly-constructed envelope or one loaded
    /// from a source that predates this field, in which case position stays runtime-only exactly
    /// as before.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<ArtifactCursor>,
    /// @emoji 🗣️ The dialect this envelope's `vcs.initial_snapshot` (and every replayed edit) is
    /// currently in — absent for envelopes minted before dialect-tracking existed, or for document
    /// kinds that never adopted more than one dialect. See `26/08/10` D4 evolution slice; nothing
    /// in `ArtifactStore::dispatch` reads or writes this yet (that wiring is later scope) — it is
    /// purely a persisted fact a future migration-aware caller can act on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dialect: Option<crate::os_io::ArtifactDialect>,
    /// @emoji 🧬️ Set once, the first time this envelope's snapshot was produced by migrating a
    /// prior document's dialect (see `migrate_document` below) rather than being authored directly
    /// in `dialect`. Absent for every envelope that was never migrated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migrated_from: Option<MigrationProvenance>,
    /// @emoji 🏠️ Present exactly when this envelope is a CHILD in a composition — the ownership
    /// stamp naming which parent/slot/`child_id` created it (see `🔖️Composition` below). Placed on
    /// the CHILD's own envelope (not only on the parent's `ArtifactChild` handle) so ownership is
    /// queryable directly from the child side — e.g. "is this document embeddable standalone, or
    /// does deleting it require going through its owner". Absent for every independent artifact
    /// (the overwhelming majority) and for every envelope minted before this ticket.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<OwnerRef>,
}

/// @emoji 🧬️ Provenance stamp for an envelope produced by `migrate_document`: which prior document,
/// which dialect it was in, and (if the migration ran at a specific checkpoint rather than the
/// live tip) which checkpoint. `migrated_at` follows this crate's existing `Checkpoint.timestamp`/
/// `Edit.started_at` convention (a caller-supplied string stamped via `now_iso()`, see below) — no
/// new clock abstraction introduced for this one field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MigrationProvenance {
    pub document_id: String,
    pub dialect: crate::os_io::ArtifactDialect,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_id: Option<String>,
    pub migrated_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ArtifactCommand<Mutation> {
    Apply {
        mutations: Vec<Mutation>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    Undo,
    Redo,
    UndoWithPolicy {
        policy: UndoPolicy,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        semantic_command: Option<Box<ArtifactCommand<Mutation>>>,
    },
    CommitCheckpoint {
        #[serde(skip_serializing_if = "Option::is_none")]
        message: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        authors: Vec<Author>,
    },
    CreateAlternative {
        name: String,
    },
    SwitchAlternative {
        alternative_id: String,
    },
    CheckoutCheckpoint {
        #[serde(rename = "checkpointId")]
        checkpoint_id: String,
    },
    AmendLast {
        mutations: Vec<Mutation>,
        /// @emoji 🪢️ Matches the last uncommitted edit's `coalesce_key` to absorb into it instead of creating a new edit.
        coalesce_key: Option<String>,
    },
    /// @emoji 🕸️ Feeds a remote MutationEnvelope through the causal DAG into the edit timeline.
    IngestRemote {
        #[serde(with = "operation_envelope_serde")]
        envelope: crate::os_spr::MutationEnvelope,
    },
    /// @emoji 🧹 Clears volatile draft-lane history that must never enter a Change/Checkpoint.
    PruneDrafts,
}
//#endregion 🔖️Schemas

//#region 🔖️Composition
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

/// @emoji 🧸️ Ownership handle a parent SNAPSHOT embeds for one owned child slot (e.g. a plugin's
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
/// No pin, no independent lifecycle: see the region doc's CHILD-vs-LINK split.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", bound = "")]
pub struct ArtifactChild<S> {
    pub child_id: String,
    pub target: crate::os_io::ArtifactRef,
    #[serde(skip)]
    _snapshot: PhantomData<S>,
}

impl<S> ArtifactChild<S> {
    /// 🏗️ Constructs a handle, threading the phantom marker for the caller.
    pub fn new(child_id: String, target: crate::os_io::ArtifactRef) -> Self {
        Self { child_id, target, _snapshot: PhantomData }
    }

    /// 🪪️ Drops the compile-time-only `S` phantom, producing the type-erased projection
    /// `ArtifactRefs::child_refs` returns.
    pub fn to_child_ref(&self, slot: &str) -> ChildRef {
        ChildRef { slot: slot.to_string(), child_id: self.child_id.clone(), target: self.target.clone() }
    }
}

impl<S> Clone for ArtifactChild<S> {
    fn clone(&self) -> Self {
        Self { child_id: self.child_id.clone(), target: self.target.clone(), _snapshot: PhantomData }
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

/// @emoji 🪪️ Type-erased projection of one `ArtifactChild<S>` field, dropping the compile-time-only
/// `S` phantom so `ArtifactRefs::child_refs` can return a single homogeneous `Vec` across however
/// many differently-`S`-typed child slots a snapshot declares.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildRef {
    pub slot: String,
    pub child_id: String,
    pub target: crate::os_io::ArtifactRef,
}

/// @emoji 🏠️ The ownership stamp placed on the CHILD's own `ArtifactEnvelope.owner` (not only on
/// the parent's `ArtifactChild` handle), so ownership is queryable directly from the child side —
/// e.g. "is this document embeddable standalone, or does deleting it require going through its
/// owner". `child_id` matches the owning `ArtifactChild<S>.child_id`/`ChildRef.child_id` exactly.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnerRef {
    pub parent: crate::os_io::ArtifactRef,
    pub slot: String,
    pub child_id: String,
}

/// @emoji 🔗️ An independent-lifecycle reference to another artifact: a PIN (so it can be frozen to
/// a specific point in the target's history) plus a `role` (the named slot it fills on the
/// referencing artifact, e.g. `"cover-image"`). Renders as a chip, never nests inline — the
/// structural opposite of `ArtifactChild`; see the region doc's CHILD-vs-LINK split.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactLink {
    pub target: crate::os_io::ArtifactRef,
    pub pin: LinkPin,
    pub role: String,
}

/// @emoji 📌️ What an `ArtifactLink` is frozen to: nothing (`Head`, always the target's live tip),
/// a specific `Checkpoint`, or a content-addressed `Snapshot` blob (survives even the target
/// document's own history being pruned/GC'd, since the bytes are escrowed independently).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum LinkPin {
    Head,
    Checkpoint { id: String },
    Snapshot { blob: BlobRef },
}

/// @emoji 🌳️ Lets a snapshot type declare its own composed children / referenced links. Both
/// methods default to empty, so a leaf artifact (the overwhelming majority) needs zero
/// boilerplate — only a technology that actually embeds `ArtifactChild<S>`/`ArtifactLink` fields
/// overrides them (typically derive-generated in a later wave, per the design doc's
/// `ChildSlotSpec`/`LinkSlotSpec` schema facets).
pub trait ArtifactRefs {
    fn child_refs(&self) -> Vec<ChildRef> {
        Vec::new()
    }
    fn links(&self) -> Vec<ArtifactLink> {
        Vec::new()
    }
}

/// @emoji 🔎️ What resolving one `ArtifactLink` found: the target materialized as real bytes, that
/// it is simply absent, or (for a `LinkPin::Snapshot`) only the escrowed blob reference is known
/// without the resolver having fetched its bytes yet.
#[derive(Clone, Debug, PartialEq)]
pub enum LinkState {
    Resolved { pack_bytes: Vec<u8>, dialect: crate::os_io::ArtifactDialect },
    Missing,
    PinnedOnly { blob: BlobRef },
}

/// @emoji 🕵️ Host-side, LAZY link resolution — never called during `CompositionCoordinator`
/// dispatch (links have no bearing on group-atomicity: unlike a child, a link's target is not part
/// of this artifact's own document and dispatch never touches it). A UI/renderer calls this only
/// when it actually needs to show/open the linked content.
pub trait LinkResolver {
    fn resolve(&self, link: &ArtifactLink) -> LinkState;
}

/// @emoji 🏭️ Type-erased per-artifact-kind child store constructor, the composition sibling of
/// `ArtifactCodec`/`register_document_codec` above. `create` mints a brand-new child store from a
/// freshly-baked initial pack (composition genesis, see `CompositionCoordinator::dispatch_group`);
/// `open` reconstructs one from a previously-persisted full envelope pack (loading an
/// already-existing child). Both return `Result` (unlike the task brief's bare `Box<dyn
/// SpaceMember>` shorthand) because either can genuinely fail on malformed bytes — every other
/// fallible construction seam in this file (`ArtifactCodec`'s `compile_dsl`/`print_mirror`,
/// `ChildStoreFactory`'s neighbor) returns `Result` for the same reason.
pub trait ChildStoreFactory: Send + Sync {
    fn create(&self, id: &str, dialect: &crate::os_io::ArtifactDialect, initial_pack: &[u8]) -> Result<Box<dyn SpaceMember>, VcsError>;
    fn open(&self, envelope_pack: &[u8]) -> Result<Box<dyn SpaceMember>, VcsError>;
}

static CHILD_STORE_FACTORY_REGISTRY: std::sync::OnceLock<std::sync::RwLock<HashMap<String, Arc<dyn ChildStoreFactory>>>> = std::sync::OnceLock::new();

fn child_store_factory_registry() -> &'static std::sync::RwLock<HashMap<String, Arc<dyn ChildStoreFactory>>> {
    CHILD_STORE_FACTORY_REGISTRY.get_or_init(|| std::sync::RwLock::new(HashMap::new()))
}

/// @emoji 📝️ Registers (or overwrites) the `ChildStoreFactory` for `kind` — idempotent, mirrors
/// `register_document_codec`'s call-once-at-init-time contract exactly.
pub fn register_child_store_factory(kind: crate::os_io::ArtifactKindId, factory: Arc<dyn ChildStoreFactory>) {
    let mut registry = child_store_factory_registry().write().unwrap_or_else(|poisoned| poisoned.into_inner());
    registry.insert(kind.as_str().to_string(), factory);
}

/// @emoji 🔎️ Looks up the `ChildStoreFactory` registered for `kind`, if any.
pub fn child_store_factory(kind: &crate::os_io::ArtifactKindId) -> Option<Arc<dyn ChildStoreFactory>> {
    let registry = child_store_factory_registry().read().unwrap_or_else(|poisoned| poisoned.into_inner());
    registry.get(kind.as_str()).cloned()
}

//#region 🔖️CompositionDsl
/// 🧬️ Hand-crafted `crate::os_dsl::DslField` records for `ArtifactChild`/`OwnerRef`/`ArtifactLink`/
/// `LinkPin` (P6 idiom, the same one `BackboneMessage`'s `OpText`/`OpBinary` impls near
/// `🔖️Backbone` below use) — NOT new `Shape` variants: the design doc's deviation D1 deliberately
/// keeps `Shape` closed (it is exhaustively matched in ~20 files) and represents every new value
/// type here as an ordinary `Shape::Record`. `crate::os_io::ArtifactRef` fields encode as their
/// `to_uri()` wire string (`Shape::Text`) rather than requiring `ArtifactRef: DslField` itself —
/// same "own the codec at this edge, don't couple types across crates" reasoning `CompositionPin`
/// (in `🌿️vcs`) already applies to the identical field shape.
fn artifact_child_spec() -> crate::os_dsl::RecordSpec {
    crate::os_dsl::RecordSpec::new(None, crate::os_dsl::RecordLayout::Inline, vec![crate::os_dsl::FieldSpec::new(0, "child_id", crate::os_dsl::Shape::Text), crate::os_dsl::FieldSpec::new(1, "target", crate::os_dsl::Shape::Text)])
}

fn artifact_child_to_record<S>(child: &ArtifactChild<S>) -> crate::os_dsl::RecordValue {
    let mut record = crate::os_dsl::RecordValue::default();
    record.fields.insert(0, crate::os_dsl::FieldValue::Text(child.child_id.clone()));
    record.fields.insert(1, crate::os_dsl::FieldValue::Text(child.target.to_uri()));
    record
}

fn artifact_child_from_record<S>(record: &crate::os_dsl::RecordValue) -> Result<ArtifactChild<S>, String> {
    let child_id = match record.get(0) {
        Some(crate::os_dsl::FieldValue::Text(s)) => s.clone(),
        other => return Err(format!("expected child_id, found {other:?}")),
    };
    let target = match record.get(1) {
        Some(crate::os_dsl::FieldValue::Text(s)) => crate::os_io::ArtifactRef::parse_uri(s)?,
        other => return Err(format!("expected target, found {other:?}")),
    };
    Ok(ArtifactChild::new(child_id, target))
}

impl<S> crate::os_dsl::DslField for ArtifactChild<S> {
    fn shape() -> crate::os_dsl::Shape {
        crate::os_dsl::Shape::Record(artifact_child_spec)
    }
    fn to_value(&self) -> crate::os_dsl::FieldValue {
        crate::os_dsl::FieldValue::Record(artifact_child_to_record(self))
    }
    fn from_value(value: &crate::os_dsl::FieldValue) -> Result<Self, String> {
        match value {
            crate::os_dsl::FieldValue::Record(record) => artifact_child_from_record(record),
            other => Err(format!("expected Record, found {other:?}")),
        }
    }
}

fn owner_ref_spec() -> crate::os_dsl::RecordSpec {
    crate::os_dsl::RecordSpec::new(
        None,
        crate::os_dsl::RecordLayout::Inline,
        vec![crate::os_dsl::FieldSpec::new(0, "parent", crate::os_dsl::Shape::Text), crate::os_dsl::FieldSpec::new(1, "slot", crate::os_dsl::Shape::Text), crate::os_dsl::FieldSpec::new(2, "child_id", crate::os_dsl::Shape::Text)],
    )
}

fn owner_ref_to_record(owner: &OwnerRef) -> crate::os_dsl::RecordValue {
    let mut record = crate::os_dsl::RecordValue::default();
    record.fields.insert(0, crate::os_dsl::FieldValue::Text(owner.parent.to_uri()));
    record.fields.insert(1, crate::os_dsl::FieldValue::Text(owner.slot.clone()));
    record.fields.insert(2, crate::os_dsl::FieldValue::Text(owner.child_id.clone()));
    record
}

fn owner_ref_from_record(record: &crate::os_dsl::RecordValue) -> Result<OwnerRef, String> {
    let parent = match record.get(0) {
        Some(crate::os_dsl::FieldValue::Text(s)) => crate::os_io::ArtifactRef::parse_uri(s)?,
        other => return Err(format!("expected parent, found {other:?}")),
    };
    let slot = match record.get(1) {
        Some(crate::os_dsl::FieldValue::Text(s)) => s.clone(),
        other => return Err(format!("expected slot, found {other:?}")),
    };
    let child_id = match record.get(2) {
        Some(crate::os_dsl::FieldValue::Text(s)) => s.clone(),
        other => return Err(format!("expected child_id, found {other:?}")),
    };
    Ok(OwnerRef { parent, slot, child_id })
}

impl crate::os_dsl::DslField for OwnerRef {
    fn shape() -> crate::os_dsl::Shape {
        crate::os_dsl::Shape::Record(owner_ref_spec)
    }
    fn to_value(&self) -> crate::os_dsl::FieldValue {
        crate::os_dsl::FieldValue::Record(owner_ref_to_record(self))
    }
    fn from_value(value: &crate::os_dsl::FieldValue) -> Result<Self, String> {
        match value {
            crate::os_dsl::FieldValue::Record(record) => owner_ref_from_record(record),
            other => Err(format!("expected Record, found {other:?}")),
        }
    }
}

fn link_pin_spec() -> crate::os_dsl::RecordSpec {
    crate::os_dsl::RecordSpec::new(
        None,
        crate::os_dsl::RecordLayout::Inline,
        vec![
            crate::os_dsl::FieldSpec::new(0, "kind", crate::os_dsl::Shape::Enum(vec![("head".to_string(), 0), ("checkpoint".to_string(), 1), ("snapshot".to_string(), 2)])),
            crate::os_dsl::FieldSpec::new(1, "checkpoint_id", crate::os_dsl::Shape::Text).optional(),
            crate::os_dsl::FieldSpec::new(2, "blob_hash", crate::os_dsl::Shape::Text).optional(),
            crate::os_dsl::FieldSpec::new(3, "blob_size", crate::os_dsl::Shape::UInt).optional(),
            crate::os_dsl::FieldSpec::new(4, "blob_media_type", crate::os_dsl::Shape::Text).optional(),
        ],
    )
}

fn link_pin_to_record(pin: &LinkPin) -> crate::os_dsl::RecordValue {
    let mut record = crate::os_dsl::RecordValue::default();
    match pin {
        LinkPin::Head => {
            record.fields.insert(0, crate::os_dsl::FieldValue::Enum(0));
        }
        LinkPin::Checkpoint { id } => {
            record.fields.insert(0, crate::os_dsl::FieldValue::Enum(1));
            record.fields.insert(1, crate::os_dsl::FieldValue::Text(id.clone()));
        }
        LinkPin::Snapshot { blob } => {
            record.fields.insert(0, crate::os_dsl::FieldValue::Enum(2));
            record.fields.insert(2, crate::os_dsl::FieldValue::Text(blob.hash.clone()));
            record.fields.insert(3, crate::os_dsl::FieldValue::UInt(blob.size));
            record.fields.insert(4, crate::os_dsl::FieldValue::Text(blob.media_type.clone()));
        }
    }
    record
}

fn link_pin_from_record(record: &crate::os_dsl::RecordValue) -> Result<LinkPin, String> {
    let ordinal = match record.get(0) {
        Some(crate::os_dsl::FieldValue::Enum(n)) => *n,
        other => return Err(format!("expected kind, found {other:?}")),
    };
    match ordinal {
        0 => Ok(LinkPin::Head),
        1 => {
            let id = match record.get(1) {
                Some(crate::os_dsl::FieldValue::Text(s)) => s.clone(),
                other => return Err(format!("expected checkpoint_id, found {other:?}")),
            };
            Ok(LinkPin::Checkpoint { id })
        }
        2 => {
            let hash = match record.get(2) {
                Some(crate::os_dsl::FieldValue::Text(s)) => s.clone(),
                other => return Err(format!("expected blob_hash, found {other:?}")),
            };
            let size = match record.get(3) {
                Some(crate::os_dsl::FieldValue::UInt(n)) => *n,
                other => return Err(format!("expected blob_size, found {other:?}")),
            };
            let media_type = match record.get(4) {
                Some(crate::os_dsl::FieldValue::Text(s)) => s.clone(),
                other => return Err(format!("expected blob_media_type, found {other:?}")),
            };
            Ok(LinkPin::Snapshot { blob: BlobRef { hash, size, media_type } })
        }
        other => Err(format!("unknown link pin kind ordinal {other}")),
    }
}

impl crate::os_dsl::DslField for LinkPin {
    fn shape() -> crate::os_dsl::Shape {
        crate::os_dsl::Shape::Record(link_pin_spec)
    }
    fn to_value(&self) -> crate::os_dsl::FieldValue {
        crate::os_dsl::FieldValue::Record(link_pin_to_record(self))
    }
    fn from_value(value: &crate::os_dsl::FieldValue) -> Result<Self, String> {
        match value {
            crate::os_dsl::FieldValue::Record(record) => link_pin_from_record(record),
            other => Err(format!("expected Record, found {other:?}")),
        }
    }
}

fn artifact_link_spec() -> crate::os_dsl::RecordSpec {
    crate::os_dsl::RecordSpec::new(
        None,
        crate::os_dsl::RecordLayout::Inline,
        vec![crate::os_dsl::FieldSpec::new(0, "target", crate::os_dsl::Shape::Text), crate::os_dsl::FieldSpec::new(1, "pin", crate::os_dsl::Shape::Record(link_pin_spec)), crate::os_dsl::FieldSpec::new(2, "role", crate::os_dsl::Shape::Text)],
    )
}

fn artifact_link_to_record(link: &ArtifactLink) -> crate::os_dsl::RecordValue {
    let mut record = crate::os_dsl::RecordValue::default();
    record.fields.insert(0, crate::os_dsl::FieldValue::Text(link.target.to_uri()));
    record.fields.insert(1, crate::os_dsl::FieldValue::Record(link_pin_to_record(&link.pin)));
    record.fields.insert(2, crate::os_dsl::FieldValue::Text(link.role.clone()));
    record
}

fn artifact_link_from_record(record: &crate::os_dsl::RecordValue) -> Result<ArtifactLink, String> {
    let target = match record.get(0) {
        Some(crate::os_dsl::FieldValue::Text(s)) => crate::os_io::ArtifactRef::parse_uri(s)?,
        other => return Err(format!("expected target, found {other:?}")),
    };
    let pin = match record.get(1) {
        Some(crate::os_dsl::FieldValue::Record(record)) => link_pin_from_record(record)?,
        other => return Err(format!("expected pin, found {other:?}")),
    };
    let role = match record.get(2) {
        Some(crate::os_dsl::FieldValue::Text(s)) => s.clone(),
        other => return Err(format!("expected role, found {other:?}")),
    };
    Ok(ArtifactLink { target, pin, role })
}

impl crate::os_dsl::DslField for ArtifactLink {
    fn shape() -> crate::os_dsl::Shape {
        crate::os_dsl::Shape::Record(artifact_link_spec)
    }
    fn to_value(&self) -> crate::os_dsl::FieldValue {
        crate::os_dsl::FieldValue::Record(artifact_link_to_record(self))
    }
    fn from_value(value: &crate::os_dsl::FieldValue) -> Result<Self, String> {
        match value {
            crate::os_dsl::FieldValue::Record(record) => artifact_link_from_record(record),
            other => Err(format!("expected Record, found {other:?}")),
        }
    }
}
//#endregion 🔖️CompositionDsl
//#endregion 🔖️Composition

//#region 🔖️Authority
/// @emoji 🧾 Receipt from the sole store write gate (`dispatch` / `reset`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CommandReceipt {
    pub edit_ids: Vec<String>,
    pub generation: u64,
}

/// @emoji 👁️ Read-only view over a document envelope — mutation is sealed through `dispatch`/`reset`.
#[derive(Clone, Copy, Debug)]
pub struct ArtifactEnvelopeView<'a, P, Mutation> {
    envelope: &'a ArtifactEnvelope<P, Mutation>,
}

impl<'a, P, Mutation> ArtifactEnvelopeView<'a, P, Mutation> {
    pub fn schema(&self) -> &str { &self.envelope.schema }
    pub fn id(&self) -> &str { &self.envelope.id }
    pub fn vcs(&self) -> &ArtifactVcs<P, Mutation> { &self.envelope.vcs }
    pub fn backbone(&self) -> Option<&ArtifactBackboneRef> { self.envelope.backbone.as_ref() }
    pub fn active_alternative_id(&self) -> Option<&str> { self.envelope.active_alternative_id.as_deref() }
    pub fn cursor(&self) -> Option<&ArtifactCursor> { self.envelope.cursor.as_ref() }
    pub fn inner(&self) -> &'a ArtifactEnvelope<P, Mutation> { self.envelope }
}

/// @emoji 📝 Draft-lane store alias — same algebra as ArtifactStore; PruneDrafts never enters a Change.
pub type DraftStore<P, Mutation> = ArtifactStore<P, Mutation>;
//#endregion 🔖️Authority

//#region 🔖️Text
//#region 🔖️Text
/// @emoji 📍️ 1-based line/column position inside DSL or op-log source text. Lives in `dsl_core`
/// (the token-native DSL engine's foundation crate, which sits below `vcs`); re-exported here so
/// every existing `crate::os_store::TextSpan`/`crate::os_store::TextError` import across the workspace keeps compiling.
pub use crate::os_dsl::{TextError, TextSpan};

/// @emoji 📜️ Handcrafted textual representation of a document snapshot, implemented once per
/// technology next to its `Snapshot` type. LAW: `P::parse_dsl(&snapshot.print_dsl())` recovers
/// an equal snapshot — canonical `print_dsl` output is always a `parse_dsl` fixpoint; hand-written
/// text may normalize (whitespace, ordering) before reaching that fixpoint.
pub trait ArtifactDsl: Sized {
    /// @emoji 🏷️ Legacy single-segment suffix used by fixture paths and codecs.
    const EXTENSION: &'static str;
    fn parse_dsl(text: &str) -> Result<Self, TextError>;
    fn print_dsl(&self) -> String;
    /// @emoji 🪪️ Dotted `plugin.artifact` identity for `.semio` preambles and on-disk names.
    fn envelope_id() -> &'static str {
        Self::EXTENSION
    }
}

pub use crate::os_semio as semio_format;

// 🎞️ CW3 kernel cut-over: `OpText` moved (method order flipped, behavior unchanged) to
// `protocol_command`, re-exported via the `🚧️TEMPORARY protocol shim` near the top of this file.

//#endregion 🔖️Text

//#region 🔖️Pack
//#region 🔖️Pack
/// @emoji 📦️ Binary counterpart of `🔖️Text` above — see the wave-1 design at
/// `.🦑️repo/🎫️tickets/26/07/27/PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS/` for the full container-format
/// contract. `pack`'s own `EncodeOptions`/`DecodeOptions`/`VerificationLevel` are re-exported under
/// a `Pack`-prefixed name (not a plain re-export — `dsl_derive`'s emitted `ArtifactPack` impl and
/// every downstream caller spell them `crate::os_store::PackEncodeOptions`/`crate::os_store::PackDecodeOptions`/
/// `crate::os_store::PackVerificationLevel`, so there is exactly one spelling repo-wide).
pub use crate::os_pack::{DecodeOptions as PackDecodeOptions, EncodeOptions as PackEncodeOptions, PackError, VerificationLevel as PackVerificationLevel};

/// @emoji 🧵️ Thin runtime bridge to `crate::os_pack::{encode_document, decode_document}`, resolved as
/// `::crate::os_store::pack_rt::...` by `dsl_derive`'s generated `ArtifactPack` impl (app crates depend on
/// `vcs`, never on `pack` directly — same seam `::crate::os_dsl::RecordSpec`/`RecordValue` already use). Also
/// hosts the schema-less `DslValue` bridge behind `impl ArtifactPack for DslValue` below.
pub mod pack_rt {
    use super::{PackDecodeOptions, PackEncodeOptions, PackError};
    use crate::os_dsl::{DslValue, FieldSpec, FieldValue, RecordLayout, RecordSpec, RecordValue, Shape};
    use std::collections::HashMap;

    /// @emoji 🚪️ Forwards to `crate::os_pack::encode_document`.
    pub fn encode_document(spec: &RecordSpec, record: &RecordValue, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        crate::os_pack::encode_document(spec, record, options)
    }

    /// @emoji 🚪️ Forwards to `crate::os_pack::decode_document`.
    pub fn decode_document(bytes: &[u8], spec: &RecordSpec, options: &PackDecodeOptions) -> Result<(RecordValue, crate::os_pack::DecodeReport), PackError> {
        crate::os_pack::decode_document(bytes, spec, options)
    }

    /// @emoji 🎯️ P6: container-less record body helpers for handcrafted OpBinary impls.
    pub fn encode_record_body(spec: &RecordSpec, record: &RecordValue, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        crate::os_pack::encode_record_body(spec, record, options)
    }
    pub fn decode_record_body(bytes: &[u8], spec: &RecordSpec, options: &PackDecodeOptions) -> Result<(RecordValue, crate::os_pack::DecodeReport), PackError> {
        crate::os_pack::decode_record_body(bytes, spec, options)
    }
    pub fn write_varint_u64(out: &mut Vec<u8>, value: u64) {
        crate::os_pack::write_varint_u64(out, value)
    }
    pub use crate::os_pack::ByteReader;
    /// @emoji 🎯️ Format byte every encoded operation starts with (handcrafted OpBinary convention).
    pub const OP_BINARY_FORMAT: u8 = 1;

    /// @emoji 🌱️ Field id the JSON bridge's synthetic single-field record wraps a whole
    /// `serde_json::Value` payload in — mirrors `crate::os_dsl::DslField for serde_json::Value`'s
    /// `Shape::Value` escape hatch (`dsl/rs/lib.rs`), lifted one level from "one field" to "one
    /// whole document" so schema-less apps (puzzle plugins, semio_compose_rs kit) get a pack encoding too.
    const VALUE_BRIDGE_FIELD_ID: u16 = 1;

    fn value_bridge_spec() -> RecordSpec {
        RecordSpec::new(None, RecordLayout::Lines, vec![FieldSpec::new(VALUE_BRIDGE_FIELD_ID, "value", Shape::Value)])
    }

    /// @emoji 🌱️ Encodes an arbitrary `DslValue` as a complete pack file.
    pub fn encode_pack_value(value: &DslValue) -> Vec<u8> {
        let mut fields = HashMap::new();
        fields.insert(VALUE_BRIDGE_FIELD_ID, FieldValue::Value(value.clone()));
        let record = RecordValue { fields };
        encode_document(&value_bridge_spec(), &record, &PackEncodeOptions::default()).expect("value bridge encode is infallible for a well-formed DslValue")
    }

    /// @emoji 🌱️ Inverse of `encode_pack_value`.
    pub fn decode_pack_value(bytes: &[u8]) -> Result<DslValue, PackError> {
        let (record, _report) = decode_document(bytes, &value_bridge_spec(), &PackDecodeOptions::default())?;
        match record.get(VALUE_BRIDGE_FIELD_ID) {
            Some(FieldValue::Value(dsl_value)) => Ok(dsl_value.clone()),
            _ => Ok(DslValue::Null),
        }
    }

    /// @emoji 🪶️ Container-less twin of `encode_pack_value` for per-message wire payloads.
    pub fn encode_wire_value(value: &DslValue) -> Vec<u8> {
        let mut fields = HashMap::new();
        fields.insert(VALUE_BRIDGE_FIELD_ID, FieldValue::Value(value.clone()));
        let record = RecordValue { fields };
        crate::os_pack::encode_record_body(&value_bridge_spec(), &record, &PackEncodeOptions::default()).expect("wire value encode is infallible for a well-formed DslValue")
    }

    /// @emoji 🪶️ Inverse of `encode_wire_value`.
    pub fn decode_wire_value(bytes: &[u8]) -> Result<DslValue, PackError> {
        let (record, _report) = crate::os_pack::decode_record_body(bytes, &value_bridge_spec(), &PackDecodeOptions::default())?;
        match record.get(VALUE_BRIDGE_FIELD_ID) {
            Some(FieldValue::Value(dsl_value)) => Ok(dsl_value.clone()),
            _ => Ok(DslValue::Null),
        }
    }

    /// @emoji 🧩️ Compose-only bridge — external technology; converts through `DslValue` without JSON on the wire.
    pub fn encode_json_value(value: &serde_json::Value) -> Vec<u8> {
        encode_pack_value(&json_value_to_dsl(value))
    }

    /// @emoji 🧩️ Compose-only inverse of `encode_json_value`.
    pub fn decode_json_value(bytes: &[u8]) -> Result<serde_json::Value, PackError> {
        decode_pack_value(bytes).map(dsl_value_to_json)
    }

    /// @emoji 📦️ Prefix for base64-wrapped pack bytes in scene `*Json` string slots (TS `PACK_B64_PREFIX`).
    pub const PACK_B64_PREFIX: &str = "pk:";

    /// @emoji 📦️ Lossless pack snapshot as a `pk:`-prefixed base64 string.
    pub fn pack_value_to_base64(bytes: &[u8]) -> String {
        use base64::Engine;
        format!("{}{}", PACK_B64_PREFIX, base64::engine::general_purpose::STANDARD.encode(bytes))
    }

    /// @emoji 📥️ Inverse of [`pack_value_to_base64`].
    pub fn pack_value_from_base64(encoded: &str) -> Result<Vec<u8>, PackError> {
        let payload = encoded.strip_prefix(PACK_B64_PREFIX).ok_or(PackError::Malformed { what: "pack base64", offset: 0, detail: "missing pk: prefix".into() })?;
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.decode(payload).map_err(|error| PackError::Malformed { what: "pack base64", offset: 0, detail: error.to_string() })
    }

    /// @emoji 🎬️ Decodes a component-scene `*Json` field when it carries [`pack_value_to_base64`] bytes.
    pub fn decode_scene_pack_field(encoded: &str) -> Result<DslValue, PackError> {
        if encoded.starts_with(PACK_B64_PREFIX) {
            decode_pack_value(&pack_value_from_base64(encoded)?)
        } else {
            Ok(json_value_to_dsl(&serde_json::from_str(encoded).map_err(|error| PackError::Malformed { what: "scene field", offset: 0, detail: error.to_string() })?))
        }
    }

    /// @emoji 🎬️ Expands a scene `*Json` slot to JSON text for engines that still ingest stringified payloads.
    pub fn scene_field_json_text(field: &str) -> Result<String, PackError> {
        if field.starts_with(PACK_B64_PREFIX) {
            let dsl = decode_pack_value(&pack_value_from_base64(field)?)?;
            Ok(serde_json::to_string(&dsl_value_to_json(dsl)).unwrap_or_else(|_| "null".into()))
        } else {
            Ok(field.to_string())
        }
    }

    /// @emoji 🧩️ Compose wire decode helper — renormalizes a `serde_json::Value` tree after pack decode.
    pub fn renormalize_json_wire_value(value: serde_json::Value) -> serde_json::Value {
        dsl_value_to_json(renormalize_whole_number_floats(json_value_to_dsl(&value)))
    }

    fn json_value_to_dsl(value: &serde_json::Value) -> DslValue {
        match value {
            serde_json::Value::Null => DslValue::Null,
            serde_json::Value::Bool(b) => DslValue::Bool(*b),
            serde_json::Value::Number(n) => DslValue::Number(n.as_f64().unwrap_or(0.0)),
            serde_json::Value::String(s) => DslValue::String(s.clone()),
            serde_json::Value::Array(items) => DslValue::Array(items.iter().map(json_value_to_dsl).collect()),
            serde_json::Value::Object(map) => DslValue::Object(map.iter().map(|(k, v)| (k.clone(), json_value_to_dsl(v))).collect()),
        }
    }

    pub fn dsl_value_to_json(value: DslValue) -> serde_json::Value {
        match value {
            DslValue::Null => serde_json::Value::Null,
            DslValue::Bool(b) => serde_json::Value::Bool(b),
            DslValue::Number(n) => serde_json::Number::from_f64(n).map(serde_json::Value::Number).unwrap_or(serde_json::Value::Null),
            DslValue::String(s) => serde_json::Value::String(s),
            DslValue::Array(items) => serde_json::Value::Array(items.into_iter().map(dsl_value_to_json).collect()),
            DslValue::Object(entries) => serde_json::Value::Object(entries.into_iter().map(|(k, v)| (k, dsl_value_to_json(v))).collect::<serde_json::Map<_, _>>()),
        }
    }

    /// @emoji ⚖️ Semantic JSON value equality — normalizes numeric representation (`3` vs `3.0`).
    pub fn json_values_equal(a: &serde_json::Value, b: &serde_json::Value) -> bool {
        json_value_to_dsl(a) == json_value_to_dsl(b)
    }

    /// @emoji 🔧️ Rewrites fractionless floats in a `DslValue` tree to whole-number floats for integer fields.
    pub fn renormalize_whole_number_floats(value: DslValue) -> DslValue {
        match value {
            DslValue::Number(n) => {
                if n.fract() == 0.0 && n.is_finite() && n.abs() < (1u64 << 53) as f64 {
                    DslValue::Number((n as i64) as f64)
                } else {
                    DslValue::Number(n)
                }
            }
            DslValue::Array(items) => DslValue::Array(items.into_iter().map(renormalize_whole_number_floats).collect()),
            DslValue::Object(entries) => DslValue::Object(entries.into_iter().map(|(k, v)| (k, renormalize_whole_number_floats(v))).collect()),
            other => other,
        }
    }
}

/// @emoji 📦️ Binary counterpart to `ArtifactDsl` — same shape, opposite face. LAW: `P::decode_pack(
/// &p.encode_pack())` recovers an equal `p`, AND (structurally, not just by test) `decode_pack(
/// encode_pack(p)) == parse_dsl(print_dsl(p))` — dsl and pack are two encodings of the identical
/// `(RecordSpec, RecordValue)` pair keyed by the same stable `u16` field ids `dsl_derive` assigns,
/// never two independent sources of truth. The `_with` methods are required (the seam
/// `dsl_derive`'s generated impl calls through `::crate::os_store::pack_rt`); the plain names are provided
/// defaults over `Pack{Encode,Decode}Options::default()`.
pub trait ArtifactPack: Sized {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError>;
    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError>;

    /// @emoji 📦️ `encode_pack_with` at default options — infallible in practice (mirrors
    /// `ArtifactDsl::print_dsl`'s infallible signature); panics only on a `PackLimits` overflow.
    fn encode_pack(&self) -> Vec<u8> {
        self.encode_pack_with(&PackEncodeOptions::default()).expect("default-options pack encode is infallible")
    }

    /// @emoji 📦️ `decode_pack_with` at default (Standard) verification.
    fn decode_pack(bytes: &[u8]) -> Result<Self, PackError> {
        Self::decode_pack_with(bytes, &PackDecodeOptions::default())
    }

    /// @emoji 🧬️ This document kind's structural field spec, for `ArtifactCodec::pack_schema_hash`
    /// (W5.7's semio_hub schema-hash validation — see that field's doc). Default `None` for hand-written
    /// `ArtifactPack` impls with no `RecordSpec` (schema-erased or synthetic fixture types, e.g.
    /// `serde_json::Value` above): those document kinds simply opt out (a zero hash reads as
    /// "schema-agnostic" everywhere this is consumed). `#[derive(crate::os_dsl::DslArtifact)]` overrides this
    /// with the real generated `__dsl_spec()`, giving every derive-based app kind (the overwhelming
    /// majority) a genuine structural fingerprint with zero manual per-app wiring.
    fn record_spec() -> Option<crate::os_dsl::RecordSpec> {
        None
    }
}

/// @emoji 📦️ Binary counterpart to `ArtifactTextFiles`. `pack` (whole `.spk` container bytes) and
/// `spr` (whole `.spr` op-log bytes, carrying real `inverse`/binary op payloads/cursor — see
/// `print_document_spr`) are AUTHORITATIVE; `ops` stays the op-log TEXT as a human-readable mirror
/// only (format-invariant across text/pack/spr, but forwards-only — see `print_ops_log`'s doc).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ArtifactPackFiles {
    pub pack: Vec<u8>,
    pub spr: Vec<u8>,
    pub ops: String,
}



/// @emoji 🔌️ Wire codec for the authoritative half of `ArtifactPackFiles` (`pack` + `spr`; `ops` is
/// a derived text mirror, not carried — `parse_document_pack` never reads it) — one length-prefixed
/// `pack` blob followed by the remaining bytes as `spr`. Used wherever a single binary blob must
/// stand in for a whole document (media document wire, WIT `list<u8>` document hops).
pub fn encode_document_pack_bytes(pack: &[u8], spr: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    crate::os_pack::write_varint_u64(&mut out, pack.len() as u64);
    out.extend_from_slice(pack);
    out.extend_from_slice(spr);
    out
}

/// @emoji 🔌️ Inverse of `encode_document_pack_bytes`.
pub fn decode_document_pack_bytes(bytes: &[u8]) -> Result<(Vec<u8>, Vec<u8>), VcsError> {
    let mut pos = 0usize;
    let pack_len = crate::os_pack::read_varint_u64(bytes, &mut pos).map_err(|error| VcsError::Deserialize(error.to_string()))? as usize;
    let pack_end = pos.checked_add(pack_len).ok_or_else(|| VcsError::Deserialize("document pack bytes overflow".to_string()))?;
    if pack_end > bytes.len() {
        return Err(VcsError::Deserialize("document pack bytes truncated".to_string()));
    }
    Ok((bytes[pos..pack_end].to_vec(), bytes[pack_end..].to_vec()))
}

/// @emoji 🧩️ Compose-only pack bridge (external technology).
impl ArtifactPack for serde_json::Value {
    fn encode_pack_with(&self, _options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        Ok(pack_rt::encode_json_value(self))
    }
    fn decode_pack_with(bytes: &[u8], _options: &PackDecodeOptions) -> Result<Self, PackError> {
        pack_rt::decode_json_value(bytes)
    }
}

/// @emoji 🌱️ Pack counterpart of the schema-less `DslValue` escape hatch: delegates to `pack_rt`'s value bridge.
impl ArtifactPack for DslValue {
    fn encode_pack_with(&self, _options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        Ok(pack_rt::encode_pack_value(self))
    }
    fn decode_pack_with(bytes: &[u8], _options: &PackDecodeOptions) -> Result<Self, PackError> {
        pack_rt::decode_pack_value(bytes)
    }
}

/// @emoji 🔀️ The closest `PackError` variant to "a text-format failure surfaced through a pack-facing
/// API" (e.g. `dsl_derive`'s generated `decode_pack_with`, whose `__dsl_from_record` step returns
/// `TextError`). A free function, not `impl From<TextError> for PackError`: both types are
/// re-exports of foreign crates (`dsl_core`/`pack_core`) through `vcs`, so a blanket `From` impl
/// here would violate the orphan rule — neither type is actually local to this crate.
pub fn text_error_to_pack_error(error: TextError) -> PackError {
    PackError::Schema(error.to_string())
}
//#endregion 🔖️Pack

//#region 🔖️OpRt
// P6: dsl::op_rt deleted. Handcrafted OpBinary impls use pack_rt record-body helpers; see OP_BINARY_FORMAT.
//#endregion 🔖️OpRt

//#region 🔖️CodecRegistry
//#region 🔖️CodecRegistry
/// @emoji 🗂️ Type-erased document codec — the bridge a schema-string-keyed caller (chiefly
/// `framework/sync`'s `FolderEndpoint`) uses to print/parse pack+ops without naming the concrete
/// `P`/`Mutation` types at that layer. Built once per document kind via `ArtifactCodec::of`
/// (wrapped one line per app by `register_document_codec_for_app` in `framework/plugin/rs/lib.rs`,
/// wave 2) and looked up by `schema` string through `register_document_codec`/`document_codec`.
#[derive(Clone)]
pub struct ArtifactCodec {
    pub schema: String,
    pub extension: &'static str,
    /// @emoji 🧬️ W5.7: a structural fingerprint of this document kind's field shape —
    /// `crate::os_pack::schema_hash(&spec)` over `P::record_spec()`, or `[0u8; 32]` when `P` has no
    /// `RecordSpec` (hand-written `ArtifactPack` impls, see that trait method's doc). Hub actors
    /// send this in `ClientFrame::Hello`; the semio_hub pins the first non-zero hash it sees per
    /// `(space, document)` scope and rejects a later mismatching one before `Welcome` — a zero
    /// hash always skips validation (schema-agnostic client). Durable pinning belongs in the db
    /// catalog once it grows a column for it; this in-memory pin is this wave's scope.
    pub pack_schema_hash: [u8; 32],
    /// @emoji 📤️ `(dsl text, ops text) -> (pack files, dsl mirror text)` — the hand-authored/
    /// imported fallback path: compiles text straight to binary pack+spr (no JSON envelope
    /// currency anywhere in between). Returns the re-printed canonical dsl mirror alongside the
    /// pack files so a caller can write all four files (`.pack`/`.spr`/`.dsl`/`.ops`) in one shot.
    pub compile_dsl: fn(&str, &str) -> Result<(ArtifactPackFiles, String), VcsError>,
    /// @emoji 📥️ `(pack bytes, spr bytes) -> (dsl text, ops text)` — the sanctioned human/agent
    /// LOGGING mirror, produced from the authoritative binary for schema-agnostic callers
    /// (`store_sync`'s `FolderEndpoint::Pack` write path) that never touch a concrete `P`/`Mutation`.
    pub print_mirror: fn(&[u8], &[u8]) -> Result<ArtifactTextFiles, VcsError>,
    /// @emoji 🧩️ One `MutationEnvelope` -> one printed `.ops` edit block (header line + indented
    /// op line), for `FolderTextStorage::append_ops`'s hot-path logging append — decodes the
    /// envelope's opaque `OpBinary` payload back into a concrete `Mutation` just long enough to
    /// print it, for schema-agnostic callers that otherwise never see a concrete op type.
    pub edit_text_from_envelope: fn(&crate::os_spr::MutationEnvelope) -> Result<String, VcsError>,
    /// Host-authoritative Emit apply: (pack, spr, encode_ops_vec) -> (pack, spr, ops text).
    pub apply_ops_binary: fn(&[u8], &[u8], &[u8]) -> Result<(Vec<u8>, Vec<u8>, String), VcsError>,
}

impl ArtifactCodec {
    /// @emoji 🏗️ Monomorphizes three non-capturing bridge functions for `(P, Mutation)` — each a
    /// genuine zero-sized `fn` item, coercible to a bare `fn` pointer — and pairs them with `schema`/
    /// `P::EXTENSION`. One call site per document kind (`register_document_codec_for_app`).
    pub fn of<P, Mutation>(schema: impl Into<String>) -> Self
    where
        P: Clone + PartialEq + Serialize + DeserializeOwned + ArtifactDsl + ArtifactPack + Send + 'static,
        Mutation: self::Mutation<P> + PartialEq + Serialize + DeserializeOwned + OpText + OpBinary + Send + 'static,
    {
        fn compile_dsl_impl<P, Mutation>(dsl: &str, ops: &str) -> Result<(ArtifactPackFiles, String), VcsError>
        where
            P: Clone + ArtifactDsl + ArtifactPack,
            Mutation: OpText + OpBinary + self::Mutation<P>,
        {
            let parsed: ParsedDocumentText<P, Mutation> = parse_document_text(dsl, ops).map_err(|error| VcsError::Deserialize(error.to_string()))?;
            let pack_files = print_document_pack(&parsed.envelope)?;
            let dsl_mirror = parsed.envelope.vcs.initial_snapshot.print_dsl();
            Ok((pack_files, dsl_mirror))
        }

        fn print_mirror_impl<P, Mutation>(pack: &[u8], spr: &[u8]) -> Result<ArtifactTextFiles, VcsError>
        where
            P: Clone + ArtifactDsl + ArtifactPack,
            Mutation: OpText + OpBinary + self::Mutation<P>,
        {
            let parsed: ParsedDocumentText<P, Mutation> = parse_document_pack(pack, spr).map_err(|error| VcsError::Deserialize(error.to_string()))?;
            print_document_text(&parsed.envelope)
        }

        fn apply_ops_binary_impl<P, Mutation>(pack: &[u8], spr: &[u8], ops_vec: &[u8]) -> Result<(Vec<u8>, Vec<u8>, String), VcsError>
        where
            P: Clone + Serialize + DeserializeOwned + ArtifactDsl + ArtifactPack,
            Mutation: OpText + OpBinary + self::Mutation<P>,
        {
            if ops_vec.is_empty() {
                if pack.is_empty() && spr.is_empty() {
                    return Ok((Vec::new(), Vec::new(), String::new()));
                }
                let parsed = parse_document_pack::<P, Mutation>(pack, spr).map_err(|error| VcsError::Deserialize(error.to_string()))?;
                let files = print_document_pack(&parsed.envelope)?;
                return Ok((files.pack, files.spr, files.ops));
            }
            let op_blobs = crate::os_spr::decode_ops_vec(ops_vec).map_err(|error| VcsError::Deserialize(error.to_string()))?;
            let mutations: Vec<Mutation> = op_blobs
                .iter()
                .map(|bytes| Mutation::decode_op(bytes).map_err(|error| VcsError::Deserialize(error.to_string())))
                .collect::<Result<_, _>>()?;
            let mut store = if pack.is_empty() && spr.is_empty() {
                return Err(VcsError::Deserialize("apply_ops_binary: lane has no pack+spr baseline".into()));
            } else {
                let parsed = parse_document_pack::<P, Mutation>(pack, spr).map_err(|error| VcsError::Deserialize(error.to_string()))?;
                let (applied, redo) = match &parsed.envelope.cursor {
                    Some(cursor) => (cursor.applied_edit_ids.clone(), cursor.redo_edit_ids.clone()),
                    None => (parsed.envelope.vcs.edits.iter().map(|edit| edit.id.clone()).collect(), Vec::new()),
                };
                let envelope = parsed.envelope;
                let mut store = ArtifactStore::new(envelope.clone());
                store.reset(envelope, applied, redo)?;
                store
            };
            store.dispatch(ArtifactCommand::Apply { mutations, description: None })?;
            let files = print_document_pack(store.envelope())?;
            Ok((files.pack, files.spr, files.ops))
        }

        fn edit_text_from_envelope_impl<P, Mutation>(envelope: &crate::os_spr::MutationEnvelope) -> Result<String, VcsError>
        where
            Mutation: OpText + OpBinary,
        {
            let edit = edit_from_operation_envelope::<Mutation>(envelope)?;
            print_edit_lines(&edit)
        }

        Self {
            schema: schema.into(),
            extension: P::envelope_id().into(),
            pack_schema_hash: P::record_spec().map(|spec| crate::os_pack::schema_hash(&spec)).unwrap_or([0u8; 32]),
            compile_dsl: compile_dsl_impl::<P, Mutation>,
            print_mirror: print_mirror_impl::<P, Mutation>,
            edit_text_from_envelope: edit_text_from_envelope_impl::<P, Mutation>,
            apply_ops_binary: apply_ops_binary_impl::<P, Mutation>,
        }
    }
}

static DOCUMENT_CODEC_REGISTRY: std::sync::OnceLock<std::sync::RwLock<HashMap<String, ArtifactCodec>>> = std::sync::OnceLock::new();

fn document_codec_registry() -> &'static std::sync::RwLock<HashMap<String, ArtifactCodec>> {
    DOCUMENT_CODEC_REGISTRY.get_or_init(|| std::sync::RwLock::new(HashMap::new()))
}

/// @emoji 📝️ Registers (or overwrites) the codec for `codec.schema` — idempotent, safe to call
/// repeatedly (every app's registration fn calls this once per document kind at program-init time).
pub fn register_document_codec(codec: ArtifactCodec) {
    let mut registry = document_codec_registry().write().unwrap_or_else(|poisoned| poisoned.into_inner());
    registry.insert(codec.schema.clone(), codec);
}

/// @emoji 🔎️ Looks up the codec registered for `schema`, if any.
pub fn document_codec(schema: &str) -> Option<ArtifactCodec> {
    let registry = document_codec_registry().read().unwrap_or_else(|poisoned| poisoned.into_inner());
    registry.get(schema).cloned()
}

/// @emoji 📜️ Reads the document schema id from an encoded `.spr` history log.
pub fn lane_schema_from_spr(spr: &[u8]) -> Option<String> {
    if spr.is_empty() {
        return None;
    }
    crate::os_spr::decode_history(spr, &crate::os_spr::DecodeOptions::default())
        .ok()
        .map(|log| log.schema)
        .filter(|schema| !schema.is_empty())
}
//#endregion 🔖️CodecRegistry

//#region 🔖️DialectMigration
/// @emoji 🧬️ One registered lossy/lossless pack-bytes transform between two dialects of the SAME
/// artifact kind (`from.artifact_kind == to.artifact_kind` is a convention this registry doesn't
/// itself enforce — callers key by the exact `(from, to)` pair they registered). `migrate_pack` is
/// a bare `fn` pointer (same non-capturing-bridge-function shape `ArtifactCodec::of` already uses
/// for `compile_dsl`/`print_mirror`/etc above) so a migration can be a plain top-level fn with zero
/// closure-capture ceremony. Additive-only: nothing in `ArtifactCodec`/`document_codec`/
/// `ArtifactStore::dispatch` reads this registry yet — see `26/08/10` D4 evolution slice scope note.
#[derive(Clone)]
pub struct DialectMigration {
    pub from: crate::os_io::ArtifactDialect,
    pub to: crate::os_io::ArtifactDialect,
    pub lossless: bool,
    pub migrate_pack: fn(&[u8]) -> Result<Vec<u8>, String>,
}

static DIALECT_MIGRATION_REGISTRY: std::sync::OnceLock<std::sync::RwLock<HashMap<(crate::os_io::ArtifactDialect, crate::os_io::ArtifactDialect), DialectMigration>>> = std::sync::OnceLock::new();

fn dialect_migration_registry() -> &'static std::sync::RwLock<HashMap<(crate::os_io::ArtifactDialect, crate::os_io::ArtifactDialect), DialectMigration>> {
    DIALECT_MIGRATION_REGISTRY.get_or_init(|| std::sync::RwLock::new(HashMap::new()))
}

/// @emoji 📝️ Registers (or overwrites) the migration for `(migration.from, migration.to)` —
/// idempotent, mirrors `register_document_codec`'s call-once-at-init-time contract.
pub fn register_dialect_migration(migration: DialectMigration) {
    let mut registry = dialect_migration_registry().write().unwrap_or_else(|poisoned| poisoned.into_inner());
    registry.insert((migration.from.clone(), migration.to.clone()), migration);
}

/// @emoji 🔁️ Looks up the exact `(from, to)` migration and runs its `migrate_pack` over
/// `pack_bytes`, or a clear `Err` naming both dialect coordinates when none is registered.
pub fn migrate_document(from: &crate::os_io::ArtifactDialect, to: &crate::os_io::ArtifactDialect, pack_bytes: &[u8]) -> Result<Vec<u8>, String> {
    let registry = dialect_migration_registry().read().unwrap_or_else(|poisoned| poisoned.into_inner());
    let migration = registry.get(&(from.clone(), to.clone())).ok_or_else(|| format!("no dialect migration registered for {} -> {}", from.to_coordinate(), to.to_coordinate()))?;
    (migration.migrate_pack)(pack_bytes)
}
//#endregion 🔖️DialectMigration

//#region 🔖️MergeHelpers
/// @emoji 🌳️ Walks `checkpoint_id`'s ancestor chain via `parent_id` back to the root, nearest-first
/// (`checkpoint_id` itself is the first entry). Cycle-guarded (a malformed/adversarial parent chain
/// stops instead of looping forever) — every well-formed chain built by `reconcile_alternative`/
/// `CommitCheckpoint` is already acyclic, this is defense in depth, not a documented invariant break.
fn checkpoint_ancestors<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, checkpoint_id: &str) -> Vec<String> {
    let mut chain = Vec::new();
    let mut seen = HashSet::new();
    let mut current = Some(checkpoint_id.to_string());
    while let Some(id) = current {
        if !seen.insert(id.clone()) {
            break;
        }
        let parent = envelope.vcs.checkpoints.iter().find(|checkpoint| checkpoint.id == id).and_then(|checkpoint| checkpoint.parent_id.clone());
        chain.push(id);
        current = parent;
    }
    chain
}

/// @emoji 🌳️ The merge-base of checkpoints `a` and `b`: the nearest checkpoint common to both
/// ancestor chains (via `parent_id`), or `None` if their histories share no common ancestor.
/// Supports branch-merge tooling that needs to know "everything since the fork point" on either
/// side. `b`'s chain is walked nearest-to-farthest so the FIRST hit in `a`'s ancestor set is the
/// nearest (not merely *a*) common ancestor.
pub fn merge_base<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, a: &str, b: &str) -> Option<String> {
    let ancestors_a: HashSet<String> = checkpoint_ancestors(envelope, a).into_iter().collect();
    checkpoint_ancestors(envelope, b).into_iter().find(|id| ancestors_a.contains(id))
}

pub fn reconcile_alternative<P, Mutation>(envelope: &mut ArtifactEnvelope<P, Mutation>, alternative_name: &str, checkpoint_message: Option<String>, authors: Vec<Author>) -> Result<String, VcsError>
where
    P: Clone + Serialize + DeserializeOwned,
    Mutation: Clone + Serialize + DeserializeOwned,
{
    if envelope.vcs.checkpoints.is_empty() {
        return Err(VcsError::NoCheckpoint);
    }
    let checkpoint_id = envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone()).ok_or(VcsError::NoCheckpoint)?;
    let alternative_id = mint_alternative_id(alternative_name, &[checkpoint_id.clone()]);
    envelope.vcs.alternatives.push(Alternative { id: alternative_id.clone(), name: alternative_name.to_string(), checkpoint_ids: vec![checkpoint_id] });
    if let Some(message) = checkpoint_message {
        let change = Change { id: mint_change_id(&[], Some(&message)), edit_ids: Vec::new(), description: Some(message), saved_at: now_iso() };
        let parent = envelope.vcs.checkpoints.last();
        let parent_id = parent.map(|checkpoint| checkpoint.id.clone());
        let mut change_ids = parent.map(|checkpoint| checkpoint.change_ids.clone()).unwrap_or_default();
        change_ids.push(change.id.clone());
        envelope.vcs.changes.push(change);
        let timestamp = now_iso();
        let checkpoint_message = Some("reconciled".to_string());
        // 🎯️ `&[]`: reconcile-alternative checkpoints carry no composition pins yet — the
        // `CompositionCoordinator` that populates real `CompositionPin`s on commit is a later wave.
        let id = content_addressed_checkpoint_id(parent_id.as_deref(), &change_ids, &envelope.vcs.changes, checkpoint_message.as_deref(), &authors, &timestamp, &[]);
        envelope.vcs.checkpoints.push(Checkpoint { id, change_ids, parent_id, authors, message: checkpoint_message, timestamp, composition_pins: Vec::new() });
    }
    Ok(alternative_id)
}
//#endregion 🔖️MergeHelpers

//#region 🔖️Config
pub type ConfigEnvelope<C, ConfigMutation> = ArtifactEnvelope<C, ConfigMutation>;
pub type ConfigStore<C, ConfigMutation> = ArtifactStore<C, ConfigMutation>;

pub fn create_config_envelope<C, ConfigMutation>(schema: &str, id: &str, initial_snapshot: C, backbone: Option<ArtifactBackboneRef>) -> ConfigEnvelope<C, ConfigMutation>
where
    C: Clone,
{
    create_document_envelope(schema, id, initial_snapshot, backbone)
}

/// @emoji 🧮️ Config snapshots use the same DSL law as documents — `ConfigRecord` marks config types.
pub trait ConfigRecord: ArtifactDsl {}

/// @emoji 🎯️ Marks `$ty` as whole-record (no field-level diff — an operation replaces the entire
/// config) with the trivial `ConfigRecord` + `MutationDiff<Self>` pair every hand-rolled
/// `impl crate::os_store::ConfigRecord for XConfig {}` + `impl crate::os_spr::MutationDiff<XConfig> for XConfig {
/// fn apply(...) -> XConfig { self.clone() } fn absorb(...) { *self = other; } }` duo repeated
/// (~33 crates) — `impl_whole_record_config!(XConfig);` replaces both. The orphan rule still
/// requires the macro invoked from `$ty`'s own crate (relies on the caller already having
/// `protocol` in scope by name, exactly as every hand-rolled impl this replaces already did).
#[macro_export]
macro_rules! impl_whole_record_config {
    ($ty:ty) => {
        impl $crate::ConfigRecord for $ty {}
        impl ::protocol::MutationDiff<$ty> for $ty {
            fn apply(&self, _base: &$ty) -> $ty {
                self.clone()
            }
            fn absorb(&mut self, other: Self) {
                *self = other;
            }
        }
    };
}

// config_spec_* removed — ConfigSpec is UI (framework-core); avoids kernel↔core cycle
//#endregion 🔖️Config

//#region 🔖️Materialize
pub fn create_document_envelope<P, Mutation>(schema: &str, id: &str, initial_snapshot: P, backbone: Option<ArtifactBackboneRef>) -> ArtifactEnvelope<P, Mutation>
where
    P: Clone,
{
    ArtifactEnvelope { schema: schema.into(), id: id.into(), vcs: ArtifactVcs { initial_snapshot, edits: Vec::new(), changes: Vec::new(), checkpoints: Vec::new(), alternatives: Vec::new() }, backbone, active_alternative_id: None, cursor: None, dialect: None, migrated_from: None, owner: None }
}

pub fn edit_ids_for_changes<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, change_ids: &[String]) -> Vec<String>
where
    Mutation: Clone,
    P: Clone,
{
    let mut edit_ids = Vec::new();
    for change_id in change_ids {
        if let Some(change) = envelope.vcs.changes.iter().find(|entry| entry.id == *change_id) {
            edit_ids.extend(change.edit_ids.iter().cloned());
        }
    }
    edit_ids
}

pub fn materialize_document_snapshot<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, applied_edit_ids: &[String]) -> Result<P, VcsError>
where
    P: Clone,
    Mutation: self::Mutation<P>,
{
    materialize_document_snapshot_with_conflicts(envelope, applied_edit_ids).map(|(snapshot, _conflicts)| snapshot)
}

/// @emoji 🤝️ Adapts `crate::os_spr::command::Mutation::reconcile`'s new instance-based signature (`&self`,
/// was a per-TYPE associated fn taking no instance at all) to the once-per-materialization call this
/// crate's replay/store paths always performed: runs the LAST applied operation's `reconcile` hook
/// against `snapshot`, or passes `snapshot` through unchanged (matching the trait's own no-op
/// default) if no operation has ever been applied yet. Every real `Mutation` impl in this crate
/// (`SpaceHistoryMutation`/`DemoMutation`/`TimestampedMutation`) inherits the default no-op
/// `reconcile`, which ignores `self` entirely and only inspects `snapshot` — so which specific
/// operation instance triggers the call is immaterial for every one of them; a technology that
/// overrides `reconcile` to do real cross-document/graph validation (see
/// `framework/product/os/core`'s `OsMutation`) is documented as inspecting the resulting
/// `snapshot`, not `self`, for the same reason. Maps `crate::os_spr::ReconcileReport` to this crate's
/// own `SpaceConflict` at this edge — `protocol_command` deliberately doesn't know about space
/// types (see its `Mutation::reconcile` doc comment).
fn reconcile_with_last<P, Op: Mutation<P>>(last_operation: Option<&Op>, snapshot: P) -> (P, Vec<SpaceConflict>) {
    match last_operation {
        Some(operation) => {
            let (snapshot, reports) = operation.reconcile(snapshot);
            (snapshot, reports.into_iter().map(SpaceConflict::from).collect())
        }
        None => (snapshot, Vec::new()),
    }
}

/// @emoji 🤝️ Same replay as {@link materialize_document_snapshot}, additionally surfacing whatever
/// {@link Mutation::reconcile} reports for the resulting snapshot. Kept as a twin function (rather
/// than changing `materialize_document_snapshot`'s signature) so every existing caller across the
/// workspace is unaffected; call sites that care about conflicts (e.g. `ArtifactStore`) opt into
/// this one instead.
pub fn materialize_document_snapshot_with_conflicts<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, applied_edit_ids: &[String]) -> Result<(P, Vec<SpaceConflict>), VcsError>
where
    P: Clone,
    Mutation: self::Mutation<P>,
{
    let mut snapshot = envelope.vcs.initial_snapshot.clone();
    let mut last_operation: Option<&Mutation> = None;
    for edit_id in applied_edit_ids {
        let edit = envelope.vcs.edits.iter().find(|entry| entry.id == *edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
        for operation in &edit.forwards {
            snapshot = apply_mutation(&snapshot, operation);
            last_operation = Some(operation);
        }
    }
    Ok(reconcile_with_last(last_operation, snapshot))
}

/// 🕰️ Single timestamp source for `Edit.started_at`/`Checkpoint.timestamp` — re-exported so
/// callers outside this crate (e.g. the framework session command log) stamp entries in the
/// exact same format.
pub fn now_iso() -> String {
    format!("{}", now_ms())
}

fn now_ms() -> u64 {
    #[cfg(any(not(target_arch = "wasm32"), target_env = "p2"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now().duration_since(UNIX_EPOCH).map(|duration| duration.as_millis() as u64).unwrap_or(0)
    }
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    {
        js_sys::Date::now() as u64
    }
}

fn uncommitted_edit_ids<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, applied_edit_ids: &[String]) -> Vec<String>
where
    Mutation: Clone,
    P: Clone,
{
    let committed: HashSet<String> = envelope.vcs.changes.iter().flat_map(|change| change.edit_ids.iter().cloned()).collect();
    applied_edit_ids.iter().filter(|id| !committed.contains(*id)).cloned().collect()
}

//#endregion 🔖️Materialize

//#region 🔖️TextFormat
//#region 🔖️TextFormat
/// @emoji 📄️ The two files a textual VCS document is made of: the DSL text (initial snapshot) and
/// the append-only op log (every edit ever created, forwards-only — see {@link parse_document_text}).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ArtifactTextFiles {
    pub dsl: String,
    pub ops: String,
}

/// @emoji 🧩️ The result of loading a document from text: the reconstructed envelope plus the live
/// snapshot folded from every edit, so a caller never has to replay again after loading.
#[derive(Clone, Debug, PartialEq)]
pub struct ParsedDocumentText<P, Mutation> {
    pub envelope: ArtifactEnvelope<P, Mutation>,
    pub snapshot: P,
}

//#region 🔖️OpsHeaderGrammar
/// @emoji 🖋️ One `by=[...]` list entry on a `checkpoint` header line: id then name, both positional
/// (bare-preferred, quoted only when needed — e.g. a name containing a space). `Author::avatar` is
/// never part of the textual `.ops` format (this mirrors the pre-derive printer, which never carried
/// it either — see {@link Author}).
#[derive(Clone, Debug, PartialEq, DslRecord)]
struct OpsAuthor {
    #[dsl(positional)]
    id: String,
    #[dsl(positional)]
    name: String,
}

impl From<&Author> for OpsAuthor {
    fn from(author: &Author) -> Self {
        Self { id: author.id.clone(), name: author.name.clone() }
    }
}

impl From<OpsAuthor> for Author {
    fn from(author: OpsAuthor) -> Self {
        Self { id: author.id, name: author.name, avatar: None }
    }
}

/// @emoji 🧾️ One `.ops` header/structural line — `doc`/`edit`/`change`/`checkpoint`/`alternative`/
/// `active` — re-derived directly on the `dsl_schema` grammar engine (`#[derive(DslOps)]` generates
/// `OpText::parse_op`/`print_op` from this declaration; see {@link print_edit_lines}/
/// {@link print_document_text}/{@link parse_document_text}, its only callers). Sigil-free lowercase
/// keywords (bare `doc`, never `@doc` — `@` is reserved for connection points everywhere else in the
/// unified DSL syntax); `id` is always the first positional field on every line; every other field is
/// a plain `key=value` attribute that is simply OMITTED when absent (no more `-` placeholder
/// sentinel); `edits`/`changes`/`checkpoints`/`by` are real DSL lists (`by=[ u1 "Ueli Saluz" ]`), not
/// comma-joined, percent-escaped strings.
#[derive(Clone, Debug, PartialEq, DslOps)]
enum OpsHeaderLine {
    Doc {
        #[dsl(positional)]
        id: String,
        schema: String,
    },
    Edit {
        #[dsl(positional)]
        id: String,
        started: String,
        actor: Option<String>,
        finished: Option<String>,
        key: Option<String>,
        description: Option<String>,
    },
    Change {
        #[dsl(positional)]
        id: String,
        saved: String,
        edits: Vec<String>,
        description: Option<String>,
    },
    Checkpoint {
        #[dsl(positional)]
        id: String,
        at: String,
        changes: Vec<String>,
        parent: Option<String>,
        by: Vec<OpsAuthor>,
        message: Option<String>,
    },
    Alternative {
        #[dsl(positional)]
        id: String,
        name: String,
        checkpoints: Vec<String>,
    },
    Active {
        #[dsl(positional)]
        id: String,
    },
    /// @emoji 🎯️ Undo/redo/checkout position — the FULL applied/redo edit-id lists, not a tail
    /// marker (see `ArtifactCursor`'s doc for why). Mirrors `crate::os_spr::HistoryCursor`'s grammar.
    Cursor { applied: Vec<String>, redo: Vec<String>, checkpoint: Option<String> },
}

//#region 🔖️OpCodec
/// 🎞️ Handcrafted OpText (P6).
impl OpText for OpsHeaderLine {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = crate::os_dsl::parse(
                    line,
                    &spec_fn(),
                    &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline },
                )?;
                return <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(crate::os_dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl OpBinary for OpsHeaderLine {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(crate::os_spr::ProtocolError::Malformed {
            what: "op variant",
            offset: 0,
            detail: format!("keyword {keyword:?} is not a declared variant"),
        })?;
        let spec = (variants[ordinal].1)();
        let body = crate::os_pack::encode_record_body(&spec, &record, &PackEncodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        crate::os_pack::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = crate::os_pack::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(crate::os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(crate::os_spr::ProtocolError::Malformed {
            what: "op variant",
            offset: 1,
            detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()),
        })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
        <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| crate::os_spr::ProtocolError::Malformed {
            what: "op record",
            offset: reader.position() as u64,
            detail: error.to_string(),
        })
    }
}
//#endregion 🔖️OpCodec

//#endregion 🔖️OpsHeaderGrammar

/// @emoji 📤️ Prints one edit as an `edit ...` header line followed by one two-space-indented
/// `print_op` line per forward operation — the hot-path append unit for the op log. Backwards
/// operations and per-operation metadata are never serialized; they are recomputed during
/// {@link parse_document_text}'s load replay.
pub fn print_edit_lines<Mutation: OpText>(edit: &Edit<Mutation>) -> Result<String, VcsError> {
    let header = OpsHeaderLine::Edit { id: edit.id.clone(), started: edit.started_at.clone(), actor: edit.actor.clone(), finished: edit.finished_at.clone(), key: edit.coalesce_key.clone(), description: edit.description.clone() };
    let mut out = header.print_op();
    out.push('\n');
    for operation in &edit.forwards {
        let printed = operation.print_op();
        if printed.contains('\n') {
            return Err(VcsError::Serialize("op-text print_op must not contain a newline".into()));
        }
        out.push_str("  ");
        out.push_str(&printed);
        out.push('\n');
    }
    Ok(out)
}

/// @emoji 📤️ Builds just the op-log half of a textual/pack document — `doc` header, every edit ever
/// created as an `edit` block, then `change`/`checkpoint`/`alternative`/`active` records. Shared by
/// `print_document_text` and `print_document_pack`: the op-log grammar never touches
/// `initial_snapshot`, so it is provably format-invariant and both printers thin out to this plus
/// their own initial-snapshot encoding.
fn print_ops_log<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>) -> Result<String, VcsError>
where
    Mutation: OpText,
{
    let mut ops = String::new();
    ops.push_str(&OpsHeaderLine::Doc { id: envelope.id.clone(), schema: envelope.schema.clone() }.print_op());
    ops.push('\n');
    for edit in &envelope.vcs.edits {
        ops.push_str(&print_edit_lines(edit)?);
    }
    for change in &envelope.vcs.changes {
        let header = OpsHeaderLine::Change { id: change.id.clone(), saved: change.saved_at.clone(), edits: change.edit_ids.clone(), description: change.description.clone() };
        ops.push_str(&header.print_op());
        ops.push('\n');
    }
    for checkpoint in &envelope.vcs.checkpoints {
        let header = OpsHeaderLine::Checkpoint {
            id: checkpoint.id.clone(),
            at: checkpoint.timestamp.clone(),
            changes: checkpoint.change_ids.clone(),
            parent: checkpoint.parent_id.clone(),
            by: checkpoint.authors.iter().map(OpsAuthor::from).collect(),
            message: checkpoint.message.clone(),
        };
        ops.push_str(&header.print_op());
        ops.push('\n');
    }
    for alternative in &envelope.vcs.alternatives {
        let header = OpsHeaderLine::Alternative { id: alternative.id.clone(), name: alternative.name.clone(), checkpoints: alternative.checkpoint_ids.clone() };
        ops.push_str(&header.print_op());
        ops.push('\n');
    }
    if let Some(active_id) = &envelope.active_alternative_id {
        ops.push_str(&OpsHeaderLine::Active { id: active_id.clone() }.print_op());
        ops.push('\n');
    }
    if let Some(cursor) = &envelope.cursor {
        let header = OpsHeaderLine::Cursor { applied: cursor.applied_edit_ids.clone(), redo: cursor.redo_edit_ids.clone(), checkpoint: cursor.checkpoint_id.clone() };
        ops.push_str(&header.print_op());
        ops.push('\n');
    }
    Ok(ops)
}

/// @emoji 📤️ Prints the full textual VCS document: the DSL text (initial snapshot) and the complete
/// op log (`doc` header, every edit ever created as an `edit` block, then `change`/`checkpoint`/
/// `alternative`/`active` records). Replaces the JSON envelope as the canonical persisted form.
pub fn print_document_text<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>) -> Result<ArtifactTextFiles, VcsError>
where
    P: ArtifactDsl,
    Mutation: OpText,
{
    let dsl = envelope.vcs.initial_snapshot.print_dsl();
    let ops = print_ops_log(envelope)?;
    Ok(ArtifactTextFiles { dsl, ops })
}

/// @emoji 🎞️ `crate::os_spr::UndoPolicy` ordinal, matching `HistoryOpMeta.undo_policy`'s wire shape —
/// distinct from `undo_policy_ordinal` above, which maps THIS crate's `ArtifactCommand`-facing
/// `UndoPolicy` (currently `semio_framework::UndoPolicy`; the two enums have identical
/// variants and will merge in the kernel-unification wave, see `protocol_core`'s own doc note).
fn protocol_undo_policy_ordinal(policy: UndoPolicy) -> u8 {
    match policy {
        UndoPolicy::ExactBaseOnly => 0,
        UndoPolicy::TransformAgainstConcurrent => 1,
        UndoPolicy::SemanticUndo => 2,
        UndoPolicy::CompensatingAction => 3,
    }
}

fn protocol_undo_policy_from_ordinal(ordinal: u8) -> UndoPolicy {
    match ordinal {
        1 => UndoPolicy::TransformAgainstConcurrent,
        2 => UndoPolicy::SemanticUndo,
        3 => UndoPolicy::CompensatingAction,
        _ => UndoPolicy::ExactBaseOnly,
    }
}

fn history_op_meta_from_operation_meta(meta: &MutationMeta) -> crate::os_spr::HistoryOpMeta {
    crate::os_spr::HistoryOpMeta {
        op_id: meta.mutation_id.as_ref().map(|id| id.0.clone()),
        dependencies: meta.dependencies.iter().map(|id| id.0.clone()).collect(),
        base_version: meta.base_version,
        author_id: meta.author_id.as_ref().map(|id| id.0.clone()),
        hlt: Some((meta.timestamp.actor, meta.timestamp.physical_ms as i64, meta.timestamp.logical)),
        undo_policy: protocol_undo_policy_ordinal(meta.undo_policy),
        payload_hash: meta.payload_hash.as_ref().map(|hash| hash.0),
        group_id: meta.group_id.clone(),
    }
}

fn mutation_meta_from_history_op_meta(meta: crate::os_spr::HistoryOpMeta) -> MutationMeta {
    let (actor, physical_ms, logical) = meta.hlt.unwrap_or((0, 0, 0));
    MutationMeta {
        mutation_id: meta.op_id.map(MutationId),
        dependencies: meta.dependencies.into_iter().map(MutationId).collect(),
        base_version: meta.base_version,
        author_id: meta.author_id.map(ActorId),
        timestamp: HybridLogicalTimestamp { actor, physical_ms: physical_ms as u64, logical },
        undo_policy: protocol_undo_policy_from_ordinal(meta.undo_policy),
        payload_hash: meta.payload_hash.map(crate::os_spr::PayloadHash),
        semantic_kind: None,
        label: None,
        group_id: meta.group_id,
    }
}

/// @emoji 🎯️ Builds the binary op-log twin of `print_ops_log` — a `crate::os_spr::HistoryLog` carrying
/// REAL `inverse`/binary op payloads/explicit meta/cursor, encoded via `crate::os_spr::encode_history`
/// with `write_backwards_section: true`. Unlike the `.ops` text mirror (forwards-only, see
/// `print_ops_log`'s doc), this is the AUTHORITATIVE persisted form: `parse_document_spr` recovers
/// inverse/meta byte-for-byte instead of recomputing them via replay.
fn history_op_payloads<Mutation: OpBinary>(mutations: &[Mutation]) -> Result<Vec<crate::os_spr::OpPayload>, VcsError> {
    mutations.iter().map(|op| Ok(crate::os_spr::OpPayload { text: None, binary: Some(op.encode_op().map_err(|error| VcsError::Serialize(error.to_string()))?) })).collect()
}

fn history_edit_from_edit<Mutation: OpBinary>(edit: &Edit<Mutation>) -> Result<crate::os_spr::HistoryEdit, VcsError> {
    Ok(crate::os_spr::HistoryEdit {
        id: edit.id.clone(),
        actor: edit.actor.clone(),
        started_at: edit.started_at.clone(),
        finished_at: edit.finished_at.clone(),
        coalesce_key: edit.coalesce_key.clone(),
        description: edit.description.clone(),
        ops: history_op_payloads(&edit.forwards)?,
        inverse: history_op_payloads(&edit.inverse)?,
        // 🎯️ An empty `mutation_meta` (e.g. a hand-authored/externally-injected edit with no
        // explicit meta, distinct from a real dispatch which always populates one entry per
        // forward op) is treated as ABSENT, not as `Some(vec![])` — `encode_edit` requires
        // `metas.len() == ops.len()` when meta is present at all, and an empty-but-`Some` vec
        // would spuriously fail that check for a non-empty `ops`.
        meta: if edit.mutation_meta.is_empty() { None } else { Some(edit.mutation_meta.iter().map(history_op_meta_from_operation_meta).collect()) },
    })
}

/// @emoji 🎯️ Encodes a bare, edit-free `.spr` op log for `schema` — the counterpart to a `.pack`
/// file carrying only an initial snapshot with no history yet (e.g. a single dropped `.pack`
/// file with no accompanying `.spr` sidecar). `doc_id` may be empty when the caller mints a fresh
/// id downstream (as `parse_document_spr` never cross-checks it against the pack). LAW:
/// `parse_document_spr(pack, &empty_document_spr(id, schema))` recovers exactly `P::decode_pack(pack)`
/// as both the initial and live snapshot, with zero edits.
pub fn empty_document_spr(doc_id: &str, schema: &str) -> Vec<u8> {
    let log = crate::os_spr::HistoryLog { doc_id: doc_id.to_string(), schema: schema.to_string(), ..crate::os_spr::HistoryLog::default() };
    crate::os_spr::encode_history(&log, &crate::os_spr::EncodeOptions::default()).expect("encoding an edit-free HistoryLog is infallible")
}

/// @emoji ➕️ Appends `edits` to an already-encoded `.spr` byte log — decode, extend, re-encode.
/// **Also refreshes `log.cursor.applied_edit_ids`** with the newly-appended edits' own ids: the
/// live snapshot a later `parse_document_spr` call folds is exactly `cursor.applied_edit_ids`
/// (see that function's doc); skipping this step would make appended edits durable but invisible
/// to the next reader. Only touches the cursor if one is already present (an edit-free/cursor-free
/// log has no undo/redo position to preserve). O(history) per call — a caller appending many
/// batches back-to-back pays the whole decode/encode cost each time; a streaming variant
/// (`SprWriter::resume` + a seeded `DictBuilder`/`edit_ordinals`) is a follow-up optimization, not
/// required for correctness (this function's asymptotics match the JSON-envelope full rewrite it
/// replaces).
pub fn append_history_edits_to_spr(spr: &[u8], edits: &[crate::os_spr::HistoryEdit]) -> Result<Vec<u8>, VcsError> {
    let mut log = crate::os_spr::decode_history(spr, &crate::os_spr::DecodeOptions::default()).map_err(|error| VcsError::Deserialize(error.to_string()))?;
    if let Some(cursor) = &mut log.cursor {
        cursor.applied_edit_ids.extend(edits.iter().map(|edit| edit.id.clone()));
    }
    log.edits.extend(edits.iter().cloned());
    let options = crate::os_spr::EncodeOptions { write_backwards_section: true, ..crate::os_spr::EncodeOptions::default() };
    crate::os_spr::encode_history(&log, &options).map_err(|error| VcsError::Serialize(error.to_string()))
}

pub fn print_document_spr<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>) -> Result<Vec<u8>, VcsError>
where
    Mutation: OpBinary,
{
    let mut edits = Vec::with_capacity(envelope.vcs.edits.len());
    for edit in &envelope.vcs.edits {
        edits.push(history_edit_from_edit::<Mutation>(edit)?);
    }
    let log = crate::os_spr::HistoryLog {
        doc_id: envelope.id.clone(),
        schema: envelope.schema.clone(),
        edits,
        changes: envelope.vcs.changes.iter().map(|change| crate::os_spr::HistoryChange { id: change.id.clone(), saved_at: change.saved_at.clone(), edit_ids: change.edit_ids.clone(), description: change.description.clone() }).collect(),
        checkpoints: envelope
            .vcs
            .checkpoints
            .iter()
            .map(|checkpoint| crate::os_spr::HistoryCheckpoint {
                id: checkpoint.id.clone(),
                timestamp: checkpoint.timestamp.clone(),
                change_ids: checkpoint.change_ids.clone(),
                parent_id: checkpoint.parent_id.clone(),
                authors: checkpoint.authors.iter().map(|author| crate::os_spr::HistoryAuthor { id: author.id.clone(), name: author.name.clone() }).collect(),
                message: checkpoint.message.clone(),
            })
            .collect(),
        alternatives: envelope.vcs.alternatives.iter().map(|alternative| crate::os_spr::HistoryAlternative { id: alternative.id.clone(), name: alternative.name.clone(), checkpoint_ids: alternative.checkpoint_ids.clone() }).collect(),
        active_alternative_id: envelope.active_alternative_id.clone(),
        cursor: envelope.cursor.as_ref().map(|cursor| crate::os_spr::HistoryCursor { applied_edit_ids: cursor.applied_edit_ids.clone(), redo_edit_ids: cursor.redo_edit_ids.clone(), checkpoint_id: cursor.checkpoint_id.clone() }),
    };
    let options = crate::os_spr::EncodeOptions { write_backwards_section: true, ..crate::os_spr::EncodeOptions::default() };
    crate::os_spr::encode_history(&log, &options).map_err(|error| VcsError::Serialize(error.to_string()))
}

/// @emoji 🎯️ Inverse of [`print_document_spr`]: rebuilds an envelope's `edits`/`changes`/
/// `checkpoints`/`alternatives`/`cursor` from a decoded `HistoryLog`, recovering `inverse` and
/// `mutation_meta` from the persisted data (never replay-recomputed, unlike the text path) — the
/// initial snapshot comes from `pack` via `ArtifactPack::decode_pack`, matching
/// `parse_document_pack`'s contract.
pub fn parse_document_spr<P, Mutation>(pack: &[u8], spr: &[u8]) -> Result<ParsedDocumentText<P, Mutation>, TextError>
where
    P: Clone + ArtifactPack,
    Mutation: OpText + OpBinary + self::Mutation<P>,
{
    let initial_snapshot = P::decode_pack(pack).map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))?;
    let log = crate::os_spr::decode_history(spr, &crate::os_spr::DecodeOptions::default()).map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))?;

    let decode_op = |payload: &crate::os_spr::OpPayload| -> Result<Mutation, TextError> {
        match (&payload.binary, &payload.text) {
            (Some(bytes), _) => Mutation::decode_op(bytes).map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1))),
            (None, Some(text)) => Mutation::parse_op(text),
            (None, None) => Err(TextError::new("op payload carries neither binary nor text".to_string(), TextSpan::at(1, 1))),
        }
    };

    let mut snapshot = initial_snapshot.clone();
    let mut edits: Vec<Edit<Mutation>> = Vec::with_capacity(log.edits.len());
    for (index, history_edit) in log.edits.into_iter().enumerate() {
        let forwards = history_edit.ops.iter().map(decode_op).collect::<Result<Vec<_>, _>>()?;
        let (inverse, mutation_meta) = if !history_edit.inverse.is_empty() || history_edit.meta.is_some() {
            let inverse = history_edit.inverse.iter().map(decode_op).collect::<Result<Vec<_>, _>>()?;
            let mutation_meta = history_edit.meta.map(|metas| metas.into_iter().map(mutation_meta_from_history_op_meta).collect()).unwrap_or_default();
            (inverse, mutation_meta)
        } else {
            let mut inverse = Vec::with_capacity(forwards.len());
            let mut mutation_meta = Vec::with_capacity(forwards.len());
            for operation in &forwards {
                let mut back = operation.inverse(&snapshot);
                back.reverse();
                inverse.extend(back);
                mutation_meta.push(MutationMeta {
                    mutation_id: Some(operation.mutation_id().unwrap_or_else(|| MutationId(mint_mutation_id(&operation.encode_op().unwrap_or_default())))),
                    dependencies: operation.dependencies(),
                    base_version: operation.base_version().map(|version| version.0).unwrap_or(0),
                    author_id: Some(operation.author_id().unwrap_or_else(|| ActorId("local".into()))),
                    timestamp: operation.timestamp().unwrap_or_else(|| HybridLogicalTimestamp::new(0, now_ms())),
                    undo_policy: operation.undo_policy(),
                    payload_hash: None,
                    semantic_kind: None,
                    label: None,
                    group_id: None,
                });
            }
            (inverse, mutation_meta)
        };
        for operation in &forwards {
            snapshot = apply_mutation(&snapshot, operation);
        }
        edits.push(Edit {
            id: history_edit.id,
            actor: history_edit.actor,
            forwards,
            inverse,
            mutation_meta,
            description: history_edit.description,
            coalesce_key: history_edit.coalesce_key,
            sequence_number: index as i32 + 1,
            started_at: history_edit.started_at,
            finished_at: history_edit.finished_at,
        });
    }

    let cursor = log.cursor.map(|cursor| ArtifactCursor { applied_edit_ids: cursor.applied_edit_ids, redo_edit_ids: cursor.redo_edit_ids, checkpoint_id: cursor.checkpoint_id });
    let envelope = ArtifactEnvelope {
        schema: log.schema,
        id: log.doc_id,
        vcs: ArtifactVcs {
            initial_snapshot,
            edits,
            changes: log.changes.into_iter().map(|change| Change { id: change.id, edit_ids: change.edit_ids, description: change.description, saved_at: change.saved_at }).collect(),
            checkpoints: log
                .checkpoints
                .into_iter()
                .map(|checkpoint| Checkpoint {
                    id: checkpoint.id,
                    change_ids: checkpoint.change_ids,
                    parent_id: checkpoint.parent_id,
                    authors: checkpoint.authors.into_iter().map(|author| Author { id: author.id, name: author.name, avatar: None }).collect(),
                    message: checkpoint.message,
                    timestamp: checkpoint.timestamp,
                    // 🎯️ `crate::os_spr::HistoryCheckpoint` (the `.spr` durable form) does not carry
                    // composition pins yet — extending that codec is out of this wave's scope (see
                    // `UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/📓️wave1-reports/b1-spr-vcs-report.md`
                    // sharedFileRequests). `composition_pins` is therefore in-memory-only until a
                    // later wave threads it through `history_op_meta`-style encode/decode.
                    composition_pins: Vec::new(),
                })
                .collect(),
            alternatives: log.alternatives.into_iter().map(|alternative| Alternative { id: alternative.id, name: alternative.name, checkpoint_ids: alternative.checkpoint_ids }).collect(),
        },
        backbone: None,
        active_alternative_id: log.active_alternative_id,
        cursor: cursor.clone(),
        dialect: None,
        migrated_from: None,
        // 🎯️ `crate::os_spr::HistoryLog` (the `.spr` durable form) does not carry the owner stamp
        // yet either — same in-memory-only deferral as `composition_pins` above (see
        // `UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/📓️wave1-reports/b2-store-composition-report.md`
        // sharedFileRequests) until a later wave threads `OwnerRef` through the history codec.
        owner: None,
    };

    let snapshot = if let Some(cursor) = &cursor {
        let mut folded = envelope.vcs.initial_snapshot.clone();
        let mut last_operation = None;
        for edit_id in &cursor.applied_edit_ids {
            if let Some(edit) = envelope.vcs.edits.iter().find(|edit| &edit.id == edit_id) {
                for operation in &edit.forwards {
                    folded = apply_mutation(&folded, operation);
                    last_operation = Some(operation);
                }
            }
        }
        let (reconciled, _conflicts) = reconcile_with_last(last_operation, folded);
        reconciled
    } else {
        let last_operation = envelope.vcs.edits.last().and_then(|edit| edit.forwards.last());
        let (reconciled, _conflicts) = reconcile_with_last(last_operation, snapshot);
        reconciled
    };
    Ok(ParsedDocumentText { envelope, snapshot })
}

/// @emoji 📤️ Pack counterpart of `print_document_text`: identical op-log TEXT body (`print_ops_log`)
/// for the human-readable mirror, but the initial snapshot is encoded to pack bytes
/// (`ArtifactPack::encode_pack`) instead of printed to DSL text — plus the AUTHORITATIVE `.spr`
/// binary op log (`print_document_spr`), which carries real inverse/binary payloads/cursor.
pub fn print_document_pack<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>) -> Result<ArtifactPackFiles, VcsError>
where
    P: ArtifactPack,
    Mutation: OpText + OpBinary,
{
    let pack = envelope.vcs.initial_snapshot.encode_pack();
    let spr = print_document_spr(envelope)?;
    let ops = print_ops_log(envelope)?;
    Ok(ArtifactPackFiles { pack, spr, ops })
}

/// @emoji 📥️ Replays `ops` against an already-obtained `initial_snapshot` — the parse-independent
/// tail shared by `parse_document_text` (which obtains the snapshot via `P::parse_dsl`) and
/// `parse_document_pack` (via `P::decode_pack`). When the log carries a `cursor` line, the
/// returned live snapshot reflects exactly `cursor.applied_edit_ids`, restoring the exact
/// undo/redo position across a save/load cycle. Absent a cursor (logs predating this field, or a
/// caller that never persisted one), every `edit` is treated as applied, in file order — the
/// original JSON `load_document`-compatible behavior.
fn replay_ops<P, Mutation>(initial_snapshot: P, ops: &str) -> Result<ParsedDocumentText<P, Mutation>, TextError>
where
    P: Clone,
    Mutation: OpText + self::Mutation<P>,
{
    let mut schema = String::new();
    let mut id = String::new();
    let mut edits: Vec<Edit<Mutation>> = Vec::new();
    let mut changes: Vec<Change> = Vec::new();
    let mut checkpoints: Vec<Checkpoint> = Vec::new();
    let mut alternatives: Vec<Alternative> = Vec::new();
    let mut active_alternative_id: Option<String> = None;
    let mut cursor: Option<ArtifactCursor> = None;
    let mut snapshot = initial_snapshot.clone();

    /// @emoji 🕰️ An `edit` header line's fields, held until its trailing indented op-lines are all
    /// read (its final `Edit` can only be built once `forwards` — and therefore `inverse`/
    /// `mutation_meta`, both computed by replaying against `snapshot` — are known).
    struct PendingEdit {
        line_no: u32,
        id: String,
        actor: Option<String>,
        started_at: String,
        finished_at: Option<String>,
        coalesce_key: Option<String>,
        description: Option<String>,
    }
    let mut pending_edit: Option<PendingEdit> = None;
    let mut pending_forwards: Vec<Mutation> = Vec::new();

    let flush_pending_edit = |pending_edit: &mut Option<PendingEdit>, pending_forwards: &mut Vec<Mutation>, edits: &mut Vec<Edit<Mutation>>, snapshot: &mut P| -> Result<(), TextError> {
        let Some(header) = pending_edit.take() else {
            return Ok(());
        };
        let forwards = std::mem::take(pending_forwards);
        let mut inverse = Vec::with_capacity(forwards.len());
        let mut mutation_meta = Vec::with_capacity(forwards.len());
        for operation in &forwards {
            operation.validate(snapshot).map_err(|message| TextError::new(message, TextSpan::at(header.line_no, 1)))?;
            let mut back = operation.inverse(snapshot);
            back.reverse();
            inverse.extend(back);
            mutation_meta.push(MutationMeta {
                mutation_id: Some(operation.mutation_id().unwrap_or_else(|| MutationId(mint_mutation_id(&serde_json::to_vec(operation).unwrap_or_default())))),
                dependencies: operation.dependencies(),
                base_version: operation.base_version().map(|version| version.0).unwrap_or(0),
                author_id: Some(operation.author_id().unwrap_or_else(|| ActorId("local".into()))),
                timestamp: operation.timestamp().unwrap_or_else(|| HybridLogicalTimestamp::new(0, now_ms())),
                undo_policy: operation.undo_policy(),
                payload_hash: None,
                semantic_kind: None,
                label: None,
                group_id: None,
            });
            *snapshot = apply_mutation(snapshot, operation);
        }
        edits.push(Edit {
            id: header.id,
            actor: header.actor,
            forwards,
            inverse,
            mutation_meta,
            description: header.description,
            coalesce_key: header.coalesce_key,
            sequence_number: edits.len() as i32 + 1,
            started_at: header.started_at,
            finished_at: header.finished_at,
        });
        Ok(())
    };

    for (index, raw_line) in ops.lines().enumerate() {
        let line_no = index as u32 + 1;
        let trimmed = raw_line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if raw_line.starts_with("  ") && pending_edit.is_some() {
            let operation = Mutation::parse_op(trimmed).map_err(|error| TextError::new(error.message, TextSpan::at(line_no, error.span.column)))?;
            pending_forwards.push(operation);
            continue;
        }
        flush_pending_edit(&mut pending_edit, &mut pending_forwards, &mut edits, &mut snapshot)?;
        let line = OpsHeaderLine::parse_op(trimmed).map_err(|error| TextError::new(error.message, TextSpan::at(line_no, error.span.column)))?;
        match line {
            OpsHeaderLine::Doc { id: doc_id, schema: doc_schema } => {
                schema = doc_schema;
                id = doc_id;
            }
            OpsHeaderLine::Edit { id: edit_id, started, actor, finished, key, description } => {
                pending_edit = Some(PendingEdit { line_no, id: edit_id, actor, started_at: started, finished_at: finished, coalesce_key: key, description });
                pending_forwards = Vec::new();
            }
            OpsHeaderLine::Change { id: change_id, saved, edits: edit_ids, description } => {
                changes.push(Change { id: change_id, edit_ids, description, saved_at: saved });
            }
            OpsHeaderLine::Checkpoint { id: checkpoint_id, at, changes: change_ids, parent, by, message } => {
                checkpoints.push(Checkpoint { id: checkpoint_id, change_ids, parent_id: parent, authors: by.into_iter().map(Author::from).collect(), message, timestamp: at, composition_pins: Vec::new() });
            }
            OpsHeaderLine::Alternative { id: alternative_id, name, checkpoints: checkpoint_ids } => {
                alternatives.push(Alternative { id: alternative_id, name, checkpoint_ids });
            }
            OpsHeaderLine::Active { id: active_id } => {
                active_alternative_id = Some(active_id);
            }
            OpsHeaderLine::Cursor { applied, redo, checkpoint } => {
                cursor = Some(ArtifactCursor { applied_edit_ids: applied, redo_edit_ids: redo, checkpoint_id: checkpoint });
            }
        }
    }
    flush_pending_edit(&mut pending_edit, &mut pending_forwards, &mut edits, &mut snapshot)?;

    // 🎯️ `.ops` text format does not carry the owner stamp yet either — same in-memory-only
    // deferral as `composition_pins` a few lines above (see
    // `UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/📓️wave1-reports/b2-store-composition-report.md`
    // sharedFileRequests) until a later wave threads `OwnerRef` through the text grammar.
    let envelope = ArtifactEnvelope { schema, id, vcs: ArtifactVcs { initial_snapshot, edits, changes, checkpoints, alternatives }, backbone: None, active_alternative_id, cursor: cursor.clone(), dialect: None, migrated_from: None, owner: None };
    // 🎯️ W4: every edit is still folded above in file order (needed for correct inverse/meta —
    // an edit's inverse depends on the snapshot state at the time it was made, which requires
    // walking the FULL sequence regardless of undo/redo position). Only the RETURNED live
    // snapshot differs: when a cursor is present, it reflects only `cursor.applied_edit_ids`
    // (the store's actual undo/redo position); absent a cursor, every edit is still treated as
    // applied, preserving the pre-W4 behavior for logs that predate this field.
    let snapshot = if let Some(cursor) = &cursor {
        let mut folded = envelope.vcs.initial_snapshot.clone();
        let mut last_operation = None;
        for edit_id in &cursor.applied_edit_ids {
            if let Some(edit) = envelope.vcs.edits.iter().find(|edit| &edit.id == edit_id) {
                for operation in &edit.forwards {
                    folded = apply_mutation(&folded, operation);
                    last_operation = Some(operation);
                }
            }
        }
        let (reconciled, _conflicts) = reconcile_with_last(last_operation, folded);
        reconciled
    } else {
        let last_operation = envelope.vcs.edits.last().and_then(|edit| edit.forwards.last());
        let (reconciled, _conflicts) = reconcile_with_last(last_operation, snapshot);
        reconciled
    };
    Ok(ParsedDocumentText { envelope, snapshot })
}

/// @emoji 📥️ Parses the textual VCS document back into an envelope plus its live (fully-replayed)
/// snapshot — obtains the initial snapshot via `P::parse_dsl` then shares `replay_ops`.
pub fn parse_document_text<P, Mutation>(dsl: &str, ops: &str) -> Result<ParsedDocumentText<P, Mutation>, TextError>
where
    P: Clone + ArtifactDsl,
    Mutation: OpText + self::Mutation<P>,
{
    let initial_snapshot = P::parse_dsl(dsl)?;
    replay_ops(initial_snapshot, ops)
}

/// @emoji 📥️ spr-first pack counterpart of `parse_document_text`: pack+spr are the AUTHORITATIVE
/// pair (see `ArtifactPackFiles`'s doc) — this is a thin forward onto `parse_document_spr`, which
/// recovers real `inverse`/`mutation_meta`/`cursor` instead of recomputing them via replay.
pub fn parse_document_pack<P, Mutation>(pack: &[u8], spr: &[u8]) -> Result<ParsedDocumentText<P, Mutation>, TextError>
where
    P: Clone + ArtifactPack,
    Mutation: OpText + OpBinary + self::Mutation<P>,
{
    parse_document_spr(pack, spr)
}
//#endregion 🔖️TextFormat

//#region 🔖️CommandFormat
mod operation_envelope_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(envelope: &crate::os_spr::MutationEnvelope, serializer: S) -> Result<S::Ok, S::Error> {
        let mut bytes = Vec::new();
        crate::os_spr::encode_envelope(envelope, &mut bytes);
        bytes.serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<crate::os_spr::MutationEnvelope, D::Error> {
        let bytes = Vec::<u8>::deserialize(deserializer)?;
        let mut pos = 0;
        crate::os_spr::decode_envelope(&bytes, &mut pos).map_err(serde::de::Error::custom)
    }
}

/// @emoji 🕹️ One structural `ArtifactCommand` line — `apply`/`undo`/`redo`/`commit-checkpoint`/
/// `create-alternative`/`switch-alternative`/`checkout`/`amend` — the command-level twin of
/// `OpsHeaderLine`, re-derived on the same `dsl_schema` grammar engine. `Apply`/`Amend` carry no
/// operations here (those follow as 2-space-indented `Op::print_op` lines, exactly like
/// `print_edit_lines`); `Undo`'s `policy` is `None` for the plain `undo` command and `Some(token)`
/// for `UndoWithPolicy` (token = kebab of the `UndoPolicy` variant name), optionally followed by an
/// indented nested command block for `semantic-undo`/`compensating-action`.
#[derive(Clone, Debug, PartialEq, DslOps)]
enum CommandHeaderLine {
    Apply {
        description: Option<String>,
    },
    Undo {
        policy: Option<String>,
    },
    Redo,
    CommitCheckpoint {
        message: Option<String>,
        by: Vec<OpsAuthor>,
    },
    CreateAlternative {
        name: String,
    },
    SwitchAlternative {
        #[dsl(positional)]
        id: String,
    },
    Checkout {
        #[dsl(positional)]
        id: String,
    },
    Amend {
        key: Option<String>,
    },
    PruneDrafts,
}

//#region 🔖️OpCodec
/// 🎞️ Handcrafted OpText (P6).
impl OpText for CommandHeaderLine {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = crate::os_dsl::parse(
                    line,
                    &spec_fn(),
                    &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline },
                )?;
                return <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(crate::os_dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl OpBinary for CommandHeaderLine {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(crate::os_spr::ProtocolError::Malformed {
            what: "op variant",
            offset: 0,
            detail: format!("keyword {keyword:?} is not a declared variant"),
        })?;
        let spec = (variants[ordinal].1)();
        let body = crate::os_pack::encode_record_body(&spec, &record, &PackEncodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        crate::os_pack::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = crate::os_pack::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(crate::os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(crate::os_spr::ProtocolError::Malformed {
            what: "op variant",
            offset: 1,
            detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()),
        })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
        <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| crate::os_spr::ProtocolError::Malformed {
            what: "op record",
            offset: reader.position() as u64,
            detail: error.to_string(),
        })
    }
}
//#endregion 🔖️OpCodec


fn undo_policy_to_token(policy: UndoPolicy) -> &'static str {
    match policy {
        UndoPolicy::ExactBaseOnly => "exact-base-only",
        UndoPolicy::TransformAgainstConcurrent => "transform-against-concurrent",
        UndoPolicy::SemanticUndo => "semantic-undo",
        UndoPolicy::CompensatingAction => "compensating-action",
    }
}

fn parse_undo_policy_token(token: &str) -> Result<UndoPolicy, TextError> {
    match token {
        "exact-base-only" => Ok(UndoPolicy::ExactBaseOnly),
        "transform-against-concurrent" => Ok(UndoPolicy::TransformAgainstConcurrent),
        "semantic-undo" => Ok(UndoPolicy::SemanticUndo),
        "compensating-action" => Ok(UndoPolicy::CompensatingAction),
        other => Err(crate::os_dsl::__rt::field_error(format!("unknown undo policy token {other:?}"))),
    }
}

fn undo_policy_ordinal(policy: UndoPolicy) -> u8 {
    match policy {
        UndoPolicy::ExactBaseOnly => 0,
        UndoPolicy::TransformAgainstConcurrent => 1,
        UndoPolicy::SemanticUndo => 2,
        UndoPolicy::CompensatingAction => 3,
    }
}

fn undo_policy_from_ordinal(ordinal: u8) -> Result<UndoPolicy, crate::os_spr::ProtocolError> {
    match ordinal {
        0 => Ok(UndoPolicy::ExactBaseOnly),
        1 => Ok(UndoPolicy::TransformAgainstConcurrent),
        2 => Ok(UndoPolicy::SemanticUndo),
        3 => Ok(UndoPolicy::CompensatingAction),
        other => Err(crate::os_spr::ProtocolError::Malformed { what: "undo policy ordinal", offset: 0, detail: format!("unknown undo policy ordinal {other}") }),
    }
}

/// @emoji 📤️ Prints every 2-space-indented `Op::print_op` line for one `apply`/`amend` body,
/// erroring exactly like `print_edit_lines` if any op prints a line containing a newline.
fn print_indented_ops<Op: OpText>(out: &mut String, mutations: &[Op]) -> Result<(), VcsError> {
    for mutation in mutations {
        let printed = mutation.print_op();
        if printed.contains('\n') {
            return Err(VcsError::Serialize("op-text print_op must not contain a newline".into()));
        }
        out.push_str("  ");
        out.push_str(&printed);
        out.push('\n');
    }
    Ok(())
}

/// @emoji 📥️ Parses every already-2-space-indented body line of an `apply`/`amend` command as one
/// operation each — the command-level twin of `replay_ops`'s indented-op-line branch.
fn parse_indented_ops<Op: OpText>(body_lines: &[&str]) -> Result<Vec<Op>, TextError> {
    let mut mutations = Vec::with_capacity(body_lines.len());
    for raw in body_lines {
        if !raw.starts_with("  ") {
            return Err(crate::os_dsl::__rt::field_error(format!("expected a 2-space-indented op line, got: {raw:?}")));
        }
        mutations.push(Op::parse_op(raw.trim())?);
    }
    Ok(mutations)
}

/// @emoji 📥️ Strips exactly one 2-space indent level from every line, joining them back into a
/// standalone command text — used to recurse `parse_command` into a `semantic-undo`/
/// `compensating-action` nested command block.
fn dedent_command_lines(lines: &[&str]) -> Result<String, TextError> {
    let mut out = String::new();
    for raw in lines {
        if !raw.starts_with("  ") {
            return Err(crate::os_dsl::__rt::field_error(format!("expected a 2-space-indented nested command line, got: {raw:?}")));
        }
        out.push_str(&raw[2..]);
        out.push('\n');
    }
    Ok(out)
}

/// @emoji 📤️ Prints a `ArtifactCommand` as its one-line-per-structural-field header, plus any
/// 2-space-indented operation lines (`Apply`/`AmendLast`) or a further-indented nested command
/// block (`UndoWithPolicy`'s `semantic_command`) — the maximum-token-efficient textual twin of
/// `encode_command`. `Author::avatar` is never printed, mirroring `OpsAuthor`'s `by=[...]` law.
pub fn print_command<Op: OpText>(command: &ArtifactCommand<Op>) -> Result<String, VcsError> {
    let mut out = String::new();
    match command {
        ArtifactCommand::Apply { mutations, description } => {
            out.push_str(&CommandHeaderLine::Apply { description: description.clone() }.print_op());
            out.push('\n');
            print_indented_ops(&mut out, mutations)?;
        }
        ArtifactCommand::Undo => {
            out.push_str(&CommandHeaderLine::Undo { policy: None }.print_op());
            out.push('\n');
        }
        ArtifactCommand::Redo => {
            out.push_str(&CommandHeaderLine::Redo.print_op());
            out.push('\n');
        }
        ArtifactCommand::UndoWithPolicy { policy, semantic_command } => {
            out.push_str(&CommandHeaderLine::Undo { policy: Some(undo_policy_to_token(*policy).to_string()) }.print_op());
            out.push('\n');
            if let Some(nested) = semantic_command {
                let nested_text = print_command(nested)?;
                for line in nested_text.lines() {
                    out.push_str("  ");
                    out.push_str(line);
                    out.push('\n');
                }
            }
        }
        ArtifactCommand::CommitCheckpoint { message, authors } => {
            let header = CommandHeaderLine::CommitCheckpoint { message: message.clone(), by: authors.iter().map(OpsAuthor::from).collect() };
            out.push_str(&header.print_op());
            out.push('\n');
        }
        ArtifactCommand::CreateAlternative { name } => {
            out.push_str(&CommandHeaderLine::CreateAlternative { name: name.clone() }.print_op());
            out.push('\n');
        }
        ArtifactCommand::SwitchAlternative { alternative_id } => {
            out.push_str(&CommandHeaderLine::SwitchAlternative { id: alternative_id.clone() }.print_op());
            out.push('\n');
        }
        ArtifactCommand::CheckoutCheckpoint { checkpoint_id } => {
            out.push_str(&CommandHeaderLine::Checkout { id: checkpoint_id.clone() }.print_op());
            out.push('\n');
        }
        ArtifactCommand::AmendLast { mutations, coalesce_key } => {
            out.push_str(&CommandHeaderLine::Amend { key: coalesce_key.clone() }.print_op());
            out.push('\n');
            print_indented_ops(&mut out, mutations)?;
        }
        ArtifactCommand::IngestRemote { .. } => {
            return Err(VcsError::Serialize("IngestRemote has no text command form".into()));
        }
        ArtifactCommand::PruneDrafts => {
            out.push_str(&CommandHeaderLine::PruneDrafts.print_op());
            out.push('\n');
        }
    }
    Ok(out)
}

/// @emoji 📥️ Parses a `print_command`-produced (or hand-authored) command text back into a
/// `ArtifactCommand`. LAW: `parse_command(&print_command(c)?) == Ok(c)` for every `c`.
pub fn parse_command<Op: OpText>(text: &str) -> Result<ArtifactCommand<Op>, TextError> {
    let all_lines: Vec<&str> = text.lines().collect();
    let mut header: Option<(u32, &str)> = None;
    let mut body_start = all_lines.len();
    for (index, raw) in all_lines.iter().enumerate() {
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        header = Some((index as u32 + 1, trimmed));
        body_start = index + 1;
        break;
    }
    let (header_line_no, header_text) = header.ok_or_else(|| crate::os_dsl::__rt::field_error("empty command text"))?;
    let header_line = CommandHeaderLine::parse_op(header_text).map_err(|error| TextError::new(error.message, TextSpan::at(header_line_no, error.span.column)))?;
    let body_lines: Vec<&str> = all_lines[body_start..].iter().filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#')).copied().collect();

    match header_line {
        CommandHeaderLine::Apply { description } => {
            let mutations = parse_indented_ops(&body_lines)?;
            if mutations.is_empty() {
                return Err(crate::os_dsl::__rt::field_error("apply requires at least one operation line"));
            }
            Ok(ArtifactCommand::Apply { mutations, description })
        }
        CommandHeaderLine::Undo { policy: None } => Ok(ArtifactCommand::Undo),
        CommandHeaderLine::Undo { policy: Some(token) } => {
            let policy = parse_undo_policy_token(&token)?;
            let semantic_command = if body_lines.is_empty() {
                None
            } else {
                let dedented = dedent_command_lines(&body_lines)?;
                Some(Box::new(parse_command::<Op>(&dedented)?))
            };
            Ok(ArtifactCommand::UndoWithPolicy { policy, semantic_command })
        }
        CommandHeaderLine::Redo => Ok(ArtifactCommand::Redo),
        CommandHeaderLine::CommitCheckpoint { message, by } => Ok(ArtifactCommand::CommitCheckpoint { message, authors: by.into_iter().map(Author::from).collect() }),
        CommandHeaderLine::CreateAlternative { name } => Ok(ArtifactCommand::CreateAlternative { name }),
        CommandHeaderLine::SwitchAlternative { id } => Ok(ArtifactCommand::SwitchAlternative { alternative_id: id }),
        CommandHeaderLine::Checkout { id } => Ok(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: id }),
        CommandHeaderLine::Amend { key } => {
            let mutations = parse_indented_ops(&body_lines)?;
            if mutations.is_empty() {
                return Err(crate::os_dsl::__rt::field_error("amend requires at least one operation line"));
            }
            Ok(ArtifactCommand::AmendLast { mutations, coalesce_key: key })
        }
        CommandHeaderLine::PruneDrafts => Ok(ArtifactCommand::PruneDrafts),
    }
}

/// @emoji 🎯️ Format byte every encoded command starts with — matches `crate::os_dsl::op_rt::OP_BINARY_FORMAT`
/// (B-R6 "one wire convention": `format u8 | ordinal varint | record body`).
pub const COMMAND_BINARY_FORMAT: u8 = 1;

fn write_command_str(out: &mut Vec<u8>, s: &str) {
    crate::os_pack::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}

fn read_command_str(reader: &mut crate::os_pack::ByteReader<'_>) -> Result<String, crate::os_spr::ProtocolError> {
    let len = reader.read_varint_u64()?;
    let bytes = reader.read_bytes(len as usize)?;
    std::str::from_utf8(bytes).map(str::to_string).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "command string", offset: 0, detail: error.to_string() })
}

fn write_command_ops<Op: OpBinary>(out: &mut Vec<u8>, mutations: &[Op]) -> Result<(), crate::os_spr::ProtocolError> {
    crate::os_pack::write_varint_u64(out, mutations.len() as u64);
    for mutation in mutations {
        let bytes = mutation.encode_op()?;
        crate::os_pack::write_varint_u64(out, bytes.len() as u64);
        out.extend_from_slice(&bytes);
    }
    Ok(())
}

fn read_command_ops<Op: OpBinary>(reader: &mut crate::os_pack::ByteReader<'_>) -> Result<Vec<Op>, crate::os_spr::ProtocolError> {
    let count = reader.read_varint_u64()?;
    let mut mutations = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let len = reader.read_varint_u64()?;
        let bytes = reader.read_bytes(len as usize)?;
        mutations.push(Op::decode_op(bytes)?);
    }
    Ok(mutations)
}

/// @emoji 🎯️ B-R6 "one documented generic impl": `ArtifactCommand<Op>` cannot go through
/// `#[derive(crate::os_dsl::DslOps)]` like every concrete per-technology `Mutation` enum does — it is generic
/// over a FOREIGN `Op: OpBinary` from whichever technology is dispatching, and the derive
/// only lowers a CONCRETE type's own fields to a `RecordSpec`; there is no way to describe "some
/// other crate's already-`OpBinary` type" as a `crate::os_dsl::DslField` shape. This hand-written impl is the
/// one place `ArtifactCommand`'s binary layout is still spelled out by hand — everywhere else in the
/// workspace, encoding is derive-generated. Byte layout matches the convention exactly: `format u8
/// (=1, see `COMMAND_BINARY_FORMAT`) | variant ordinal varint (`ArtifactCommand` declaration order,
/// numerically identical to the old hand-rolled tag byte for these 9 variants since LEB128 varints
/// under 128 are single bytes) | body`. The binary twin of `print_command`/`parse_command`. LAW:
/// `ArtifactCommand::decode_op(&command.encode_op()?) == Ok(command)`.
impl<Op: OpBinary> OpBinary for ArtifactCommand<Op> {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        let mut out = vec![COMMAND_BINARY_FORMAT];
        match self {
            ArtifactCommand::Apply { mutations, description } => {
                crate::os_pack::write_varint_u64(&mut out, 0);
                out.push(if description.is_some() { 0b01 } else { 0 });
                if let Some(text) = description {
                    write_command_str(&mut out, text);
                }
                write_command_ops(&mut out, mutations)?;
            }
            ArtifactCommand::Undo => crate::os_pack::write_varint_u64(&mut out, 1),
            ArtifactCommand::Redo => crate::os_pack::write_varint_u64(&mut out, 2),
            ArtifactCommand::UndoWithPolicy { policy, semantic_command } => {
                crate::os_pack::write_varint_u64(&mut out, 3);
                out.push(undo_policy_ordinal(*policy));
                out.push(if semantic_command.is_some() { 0b01 } else { 0 });
                if let Some(nested) = semantic_command {
                    let nested_bytes = nested.encode_op()?;
                    crate::os_pack::write_varint_u64(&mut out, nested_bytes.len() as u64);
                    out.extend_from_slice(&nested_bytes);
                }
            }
            ArtifactCommand::CommitCheckpoint { message, authors } => {
                crate::os_pack::write_varint_u64(&mut out, 4);
                out.push(if message.is_some() { 0b01 } else { 0 });
                if let Some(text) = message {
                    write_command_str(&mut out, text);
                }
                crate::os_pack::write_varint_u64(&mut out, authors.len() as u64);
                for author in authors {
                    write_command_str(&mut out, &author.id);
                    write_command_str(&mut out, &author.name);
                }
            }
            ArtifactCommand::CreateAlternative { name } => {
                crate::os_pack::write_varint_u64(&mut out, 5);
                write_command_str(&mut out, name);
            }
            ArtifactCommand::SwitchAlternative { alternative_id } => {
                crate::os_pack::write_varint_u64(&mut out, 6);
                write_command_str(&mut out, alternative_id);
            }
            ArtifactCommand::CheckoutCheckpoint { checkpoint_id } => {
                crate::os_pack::write_varint_u64(&mut out, 7);
                write_command_str(&mut out, checkpoint_id);
            }
            ArtifactCommand::AmendLast { mutations, coalesce_key } => {
                crate::os_pack::write_varint_u64(&mut out, 8);
                out.push(if coalesce_key.is_some() { 0b01 } else { 0 });
                if let Some(key) = coalesce_key {
                    write_command_str(&mut out, key);
                }
                write_command_ops(&mut out, mutations)?;
            }
            ArtifactCommand::IngestRemote { envelope } => {
                crate::os_pack::write_varint_u64(&mut out, 9);
                let mut bytes = Vec::new();
                crate::os_spr::encode_envelope(envelope, &mut bytes);
                crate::os_pack::write_varint_u64(&mut out, bytes.len() as u64);
                out.extend_from_slice(&bytes);
            }
            ArtifactCommand::PruneDrafts => crate::os_pack::write_varint_u64(&mut out, 10),
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        let mut reader = crate::os_pack::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != COMMAND_BINARY_FORMAT {
            return Err(crate::os_spr::ProtocolError::Malformed { what: "command format", offset: 0, detail: format!("unsupported command format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        match ordinal {
            0 => {
                let presence = reader.read_u8()?;
                let description = if presence & 0b01 != 0 { Some(read_command_str(&mut reader)?) } else { None };
                let mutations = read_command_ops(&mut reader)?;
                Ok(ArtifactCommand::Apply { mutations, description })
            }
            1 => Ok(ArtifactCommand::Undo),
            2 => Ok(ArtifactCommand::Redo),
            3 => {
                let policy = undo_policy_from_ordinal(reader.read_u8()?)?;
                let presence = reader.read_u8()?;
                let semantic_command = if presence & 0b01 != 0 {
                    let len = reader.read_varint_u64()?;
                    let nested_bytes = reader.read_bytes(len as usize)?;
                    Some(Box::new(ArtifactCommand::<Op>::decode_op(nested_bytes)?))
                } else {
                    None
                };
                Ok(ArtifactCommand::UndoWithPolicy { policy, semantic_command })
            }
            4 => {
                let presence = reader.read_u8()?;
                let message = if presence & 0b01 != 0 { Some(read_command_str(&mut reader)?) } else { None };
                let author_count = reader.read_varint_u64()?;
                let mut authors = Vec::with_capacity(author_count as usize);
                for _ in 0..author_count {
                    let id = read_command_str(&mut reader)?;
                    let name = read_command_str(&mut reader)?;
                    authors.push(Author { id, name, avatar: None });
                }
                Ok(ArtifactCommand::CommitCheckpoint { message, authors })
            }
            5 => Ok(ArtifactCommand::CreateAlternative { name: read_command_str(&mut reader)? }),
            6 => Ok(ArtifactCommand::SwitchAlternative { alternative_id: read_command_str(&mut reader)? }),
            7 => Ok(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: read_command_str(&mut reader)? }),
            8 => {
                let presence = reader.read_u8()?;
                let coalesce_key = if presence & 0b01 != 0 { Some(read_command_str(&mut reader)?) } else { None };
                let mutations = read_command_ops(&mut reader)?;
                Ok(ArtifactCommand::AmendLast { mutations, coalesce_key })
            }
            9 => {
                let len = reader.read_varint_u64()?;
                let bytes = reader.read_bytes(len as usize)?;
                let mut pos = 0;
                let envelope = crate::os_spr::decode_envelope(bytes, &mut pos).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "ingest envelope", offset: 0, detail: error.to_string() })?;
                Ok(ArtifactCommand::IngestRemote { envelope })
            }
            10 => Ok(ArtifactCommand::PruneDrafts),
            other => Err(crate::os_spr::ProtocolError::Malformed { what: "command variant", offset: 1, detail: format!("unknown command ordinal {other}") }),
        }
    }
}


//#endregion 🔖️CommandFormat

//#region 🔖️History
//#region 🔖️History
/// @emoji 📜️ One row of a checkpoint history/ancestor graph.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryColumn {
    pub checkpoint_id: String,
    pub timestamp: String,
    pub labels: Vec<String>,
    pub authors: Vec<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_checkpoint_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub lane: usize,
    pub alternative_ids: Vec<String>,
}

fn checkpoint_alternatives<'a, P, Mutation>(envelope: &'a ArtifactEnvelope<P, Mutation>, checkpoint_id: &str) -> Vec<&'a Alternative> {
    envelope.vcs.alternatives.iter().filter(|alternative| alternative.checkpoint_ids.iter().any(|id| id == checkpoint_id)).collect()
}

fn is_checkpoint_main_only<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, checkpoint_id: &str) -> bool {
    checkpoint_alternatives(envelope, checkpoint_id).is_empty()
}

fn has_main_only_descendant<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, children_of: &HashMap<String, Vec<String>>, checkpoint_id: &str, seen: &mut HashSet<String>) -> bool {
    if !seen.insert(checkpoint_id.to_string()) {
        return false;
    }
    for child_id in children_of.get(checkpoint_id).into_iter().flatten() {
        if is_checkpoint_main_only(envelope, child_id) || has_main_only_descendant(envelope, children_of, child_id, seen) {
            return true;
        }
    }
    false
}

/// @emoji 🛤️ Assigns each checkpoint a swimlane: alternatives get lanes `1..n` in array order, lane
/// `0` is the main trunk. A checkpoint sits on lane 0 if it belongs to no alternative or has any
/// main-only descendant (cycle-guarded DFS); otherwise it takes its single alternative's lane, or
/// the minimum lane among several. Mirrors premigration `assignHistoryCheckpointLanes`.
fn assign_history_checkpoint_lanes<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>) -> HashMap<String, usize> {
    let mut lane_by_alternative: HashMap<String, usize> = HashMap::new();
    for (index, alternative) in envelope.vcs.alternatives.iter().enumerate() {
        lane_by_alternative.insert(alternative.id.clone(), index + 1);
    }
    let mut children_of: HashMap<String, Vec<String>> = HashMap::new();
    for checkpoint in &envelope.vcs.checkpoints {
        if let Some(parent_id) = &checkpoint.parent_id {
            children_of.entry(parent_id.clone()).or_default().push(checkpoint.id.clone());
        }
    }
    let mut lane_by_checkpoint_id: HashMap<String, usize> = HashMap::new();
    for checkpoint in &envelope.vcs.checkpoints {
        if checkpoint.parent_id.is_none() {
            lane_by_checkpoint_id.insert(checkpoint.id.clone(), 0);
            continue;
        }
        let mut seen = HashSet::new();
        if is_checkpoint_main_only(envelope, &checkpoint.id) || has_main_only_descendant(envelope, &children_of, &checkpoint.id, &mut seen) {
            lane_by_checkpoint_id.insert(checkpoint.id.clone(), 0);
            continue;
        }
        let alternatives = checkpoint_alternatives(envelope, &checkpoint.id);
        let lanes: Vec<usize> = alternatives.iter().map(|alternative| *lane_by_alternative.get(&alternative.id).unwrap_or(&0)).collect();
        let lane = if lanes.len() == 1 { lanes[0] } else { lanes.into_iter().min().unwrap_or(0) };
        lane_by_checkpoint_id.insert(checkpoint.id.clone(), lane);
    }
    lane_by_checkpoint_id
}

/// @emoji 📜️ Builds the ancestor-graph rows for a checkpoint history view: newest checkpoint first,
/// each carrying its swimlane, labels (alternative names, `"main"` fallback on the newest unlabeled
/// row), and authors. Mirrors premigration `buildHistoryColumns`.
pub fn build_history_columns<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>) -> Vec<HistoryColumn> {
    let lane_by_checkpoint_id = assign_history_checkpoint_lanes(envelope);
    envelope
        .vcs
        .checkpoints
        .iter()
        .rev()
        .enumerate()
        .map(|(index, checkpoint)| {
            let alternatives = checkpoint_alternatives(envelope, &checkpoint.id);
            let alternative_ids: Vec<String> = alternatives.iter().map(|alternative| alternative.id.clone()).collect();
            let mut labels: Vec<String> = alternatives.iter().map(|alternative| alternative.name.clone()).collect();
            if labels.is_empty() && index == 0 {
                labels.push("main".into());
            }
            HistoryColumn {
                checkpoint_id: checkpoint.id.clone(),
                timestamp: checkpoint.timestamp.clone(),
                labels,
                authors: checkpoint.authors.clone(),
                parent_checkpoint_id: checkpoint.parent_id.clone(),
                description: checkpoint.message.clone(),
                lane: *lane_by_checkpoint_id.get(&checkpoint.id).unwrap_or(&0),
                alternative_ids,
            }
        })
        .collect()
}
//#endregion 🔖️History

//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
pub struct ArtifactStore<P, Mutation>
where
    P: Clone + Serialize + DeserializeOwned,
    Mutation: Clone + Serialize + DeserializeOwned + self::Mutation<P>,
{
    envelope: ArtifactEnvelope<P, Mutation>,
    backbone: Option<Box<dyn Backbone>>,
    dag: crate::os_spr::MutationDag,
    applied_edit_ids: Vec<String>,
    redo_edit_ids: Vec<String>,
    edit_sequence: i32,
    generation: u64,
    /// @emoji 🧭️ The checkpoint new commits parent onto; advances on commit/checkout/switch. Not
    /// part of the wire envelope — callers that reconstruct the store per call (e.g. a WASM plugin)
    /// must save/restore it themselves via {@link current_checkpoint_id}/{@link set_current_checkpoint_id}.
    current_checkpoint_id: Option<String>,
    /// @emoji 🖋️ Identity of the local actor driving this store. Set from each local `Apply`/
    /// `AmendLast`'s operation author; compared against `Edit.actor` so undo never touches foreign
    /// edits. Not part of the wire envelope — callers that reconstruct the store per call must
    /// save/restore it via {@link local_actor_id}/{@link set_local_actor_id}.
    local_actor_id: Option<String>,
    /// @emoji 🤝️ Conflicts reported by the last {@link Mutation::reconcile} pass, refreshed after
    /// remote ingestion (see {@link ingest_envelope}). Empty for every document kind that keeps the
    /// default no-operation `reconcile`. Not part of the wire envelope — it is derived, not source of truth.
    conflicts: Vec<SpaceConflict>,
    /// @emoji ⚡️ The live, incrementally-maintained RAW fold of `initial_snapshot` over every
    /// `forwards` operation in `applied_edit_ids` order — i.e. exactly what a full
    /// {@link materialize_document_snapshot} replay computes BEFORE its single final
    /// {@link Mutation::reconcile} call. Kept in lock-step by every mutating command below instead of
    /// replaying on every read, so `snapshot()`/`Apply`/`AmendLast` are O(new work) instead of
    /// O(total history). Cold-path commands (checkout/switch/set_state, which reassign
    /// `applied_edit_ids` wholesale rather than appending) fall back to a full raw-fold recompute —
    /// see `fold_current`. Differential ground truth: `test_support::assert_live_equals_replay`.
    current: P,
    /// @emoji 🪢️ `(edit_id, snapshot right before that edit's forwards were first applied)` for
    /// whichever edit is CURRENTLY the tail of `applied_edit_ids` — refreshed by `Apply`/`AmendLast`
    /// (fresh-edit branch)/`Redo`, left untouched by further amends to the same edit (so it always
    /// points at the state before the edit as a whole, not before its latest increment). Powers an
    /// O(1) `Undo` of exactly this edit; any other undo (not the cached tail, or `None`) falls back
    /// to `fold_current` — always correct, just not always O(1).
    tail_undo_cache: Option<(String, P)>,
}

/// @emoji 🖋️ Derives an edit's authoring actor from its per-operation metadata (the author of its
/// first operation), so a local edit records who produced it for later `UndoPolicy` classification.
fn edit_actor_from_meta(mutation_meta: &[MutationMeta]) -> Option<String> {
    mutation_meta.first().and_then(|meta| meta.author_id.clone()).map(|actor_id| actor_id.0)
}

impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: Clone + Serialize + DeserializeOwned + ArtifactPack,
    Mutation: Clone + Serialize + DeserializeOwned + self::Mutation<P> + OpBinary + OpText,
{
    /// @emoji 🚫️ A store is always constructed with no backbone attached — the envelope's
    /// `backbone` field is a descriptor of the last attachment, never an instruction to
    /// reconnect. Callers attach explicitly via {@link attach_backbone}/{@link attach_backbone_uri}.
    ///
    /// @emoji 🎯️ When `envelope.cursor` is present (a `.pack`+`.spr` load, see
    /// `parse_document_spr`), `applied_edit_ids`/`redo_edit_ids`/`current_checkpoint_id`/`current`
    /// are seeded from it — restoring the exact undo/redo position across a save/load cycle.
    /// `local_actor_id` is seeded from the tail applied edit's actor so `UndoPolicy::ExactBaseOnly`'s
    /// foreign-edit check keeps working immediately after reload (a real `VcsArtifactApp` overrides
    /// it anyway via `set_local_actor_id` on every dispatch). Absent a cursor, every edit is
    /// treated as applied and `local_actor_id` stays `None` (pre-W4 behavior).
    pub fn new(envelope: ArtifactEnvelope<P, Mutation>) -> Self {
        let (applied_edit_ids, redo_edit_ids, current_checkpoint_id, current, local_actor_id) = match &envelope.cursor {
            Some(cursor) => {
                let mut folded = envelope.vcs.initial_snapshot.clone();
                let mut last_actor: Option<String> = None;
                for edit_id in &cursor.applied_edit_ids {
                    if let Some(edit) = envelope.vcs.edits.iter().find(|edit| &edit.id == edit_id) {
                        for operation in &edit.forwards {
                            folded = apply_mutation(&folded, operation);
                        }
                        last_actor = edit.actor.clone();
                    }
                }
                let checkpoint_id = cursor.checkpoint_id.clone().or_else(|| envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone()));
                (cursor.applied_edit_ids.clone(), cursor.redo_edit_ids.clone(), checkpoint_id, folded, last_actor)
            }
            None => {
                let checkpoint_id = envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone());
                (Vec::new(), Vec::new(), checkpoint_id, envelope.vcs.initial_snapshot.clone(), None)
            }
        };
        Self { envelope, backbone: None, dag: crate::os_spr::MutationDag::new(), applied_edit_ids, redo_edit_ids, edit_sequence: 0, generation: 0, current_checkpoint_id, local_actor_id, conflicts: Vec::new(), current, tail_undo_cache: None }
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn envelope(&self) -> &ArtifactEnvelope<P, Mutation> {
        &self.envelope
    }

    /// @emoji 👁️ Read-only envelope view — prefer this over mutating through public fields.
    pub fn envelope_view(&self) -> ArtifactEnvelopeView<'_, P, Mutation> {
        ArtifactEnvelopeView { envelope: &self.envelope }
    }

    pub fn applied_edit_ids(&self) -> &[String] {
        &self.applied_edit_ids
    }

    /// @emoji ↪️ Pending redo stack (edit ids undone since the last fresh `Apply`).
    pub fn redo_edit_ids(&self) -> &[String] {
        &self.redo_edit_ids
    }

    /// @emoji 🧭️ The checkpoint new commits currently parent onto (defaults to the latest checkpoint
    /// on construction/`set_state`; advances on commit/checkout/switch).
    pub fn current_checkpoint_id(&self) -> Option<&str> {
        self.current_checkpoint_id.as_deref()
    }

    /// @emoji 🧭️ Restores the checkout position after reconstructing the store from a serialized
    /// envelope (`set_state` resets it to the latest checkpoint, which is wrong once a caller has
    /// checked out an older one).
    pub fn set_current_checkpoint_id(&mut self, checkpoint_id: Option<String>) {
        self.current_checkpoint_id = checkpoint_id;
    }

    /// @emoji 🖋️ The local actor id used to distinguish this store's own edits from ingested ones.
    /// Not part of the wire envelope — a caller reconstructing the store per call must save/restore
    /// it via {@link set_local_actor_id} for `UndoPolicy` to keep classifying foreign edits.
    pub fn local_actor_id(&self) -> Option<&str> {
        self.local_actor_id.as_deref()
    }

    /// @emoji 🖋️ Sets the local actor id (see {@link local_actor_id}). Called automatically from each
    /// local `Apply`/`AmendLast`; callers that reconstruct the store per dispatch restore it here.
    pub fn set_local_actor_id(&mut self, actor_id: Option<String>) {
        self.local_actor_id = actor_id;
    }

    /// @emoji 🔧️ The most recently created/amended edit's `(forwards, inverse, per-operation meta)`.
    /// Used right after `dispatch(Apply{..})`/`AmendLast` to build a `KernelMutation`/`InvocationResult`
    /// with a true inverse from the just-recorded `Edit.inverse`.
    pub fn edit_mutations(&self) -> Option<(&[Mutation], &[Mutation], &[MutationMeta])> {
        self.envelope.vcs.edits.last().map(|edit| (edit.forwards.as_slice(), edit.inverse.as_slice(), edit.mutation_meta.as_slice()))
    }

    /// @emoji 📜️ Ancestor-graph rows for this store's checkpoint history. See {@link build_history_columns}.
    pub fn history_columns(&self) -> Vec<HistoryColumn> {
        build_history_columns(&self.envelope)
    }

    /// @emoji ♻️ Sole public reload API — replaces the former public `set_state`/`set_envelope` escape hatches.
    pub fn reset(&mut self, envelope: ArtifactEnvelope<P, Mutation>, applied_edit_ids: Vec<String>, redo_edit_ids: Vec<String>) -> Result<CommandReceipt, VcsError> {
        self.set_state(envelope, applied_edit_ids, redo_edit_ids);
        Ok(CommandReceipt { edit_ids: self.applied_edit_ids.clone(), generation: self.generation() })
    }

    pub(crate) fn set_envelope(&mut self, envelope: ArtifactEnvelope<P, Mutation>, applied_edit_ids: Vec<String>) {
        self.set_state(envelope, applied_edit_ids, Vec::new());
    }

    /// @emoji 💾️ Restores full store state including the redo stack, so `Redo` survives
    /// round-tripping through a serialized envelope (e.g. one `dispatch` call per request).
    pub(crate) fn set_state(&mut self, envelope: ArtifactEnvelope<P, Mutation>, applied_edit_ids: Vec<String>, redo_edit_ids: Vec<String>) {
        self.backbone = None;
        self.edit_sequence = envelope.vcs.edits.iter().map(|edit| edit.sequence_number).max().unwrap_or(0);
        self.current_checkpoint_id = envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone());
        self.envelope = envelope;
        // 🌱️ These ids are adopted directly, not through `dag.insert`, so the dag never learns they're
        // satisfied — seed it or a later remote envelope whose `deps` reference one would sit `Pending`
        // forever (see `MutationDag::seed_applied`). Covers every `set_state` caller: `set_envelope`
        // (store reconstruction from a persisted/cloned document), checkpoint checkout, etc.
        for edit_id in &applied_edit_ids {
            self.dag.seed_applied(MutationId(edit_id.clone()));
        }
        self.applied_edit_ids = applied_edit_ids;
        self.redo_edit_ids = redo_edit_ids;
        self.conflicts = Vec::new();
        self.tail_undo_cache = None;
        self.current = self.fold_current().expect("set_state: fold_current should not fail for a consistent envelope");
        self.bump();
    }

    /// @emoji 🧭️ Restores applied edits + checkout position for `checkpoint_id`, clearing redo.
    /// Shared by `createAlternative`/`switchAlternative`/`checkoutCheckpoint`. Mirrors premigration
    /// `checkoutCheckpointInternal`. Cold path: reassigns `applied_edit_ids` wholesale (not a tail
    /// append), so `current` is recomputed by a full raw-fold rather than an incremental update.
    fn checkout_checkpoint_internal(&mut self, checkpoint_id: String) {
        let applied = self.envelope.vcs.checkpoints.iter().find(|checkpoint| checkpoint.id == checkpoint_id).map(|checkpoint| edit_ids_for_changes(&self.envelope, &checkpoint.change_ids)).unwrap_or_default();
        self.applied_edit_ids = applied;
        self.redo_edit_ids.clear();
        self.current_checkpoint_id = Some(checkpoint_id);
        self.tail_undo_cache = None;
        self.current = self.fold_current().expect("checkout: fold_current should not fail for a consistent envelope");
    }

    /// @emoji ⚡️ The live snapshot: `Mutation::reconcile` applied to the incrementally-maintained
    /// `current` fold. Always `Ok` in practice (kept as `Result` for API stability); O(1) instead of a
    /// full replay. See the `current` field doc for the maintenance invariant.
    pub fn snapshot(&self) -> Result<P, VcsError> {
        Ok(reconcile_with_last(self.last_applied_operation(), self.current.clone()).0)
    }

    /// @emoji 🤝️ `current` reconciled, plus whatever conflicts {@link Mutation::reconcile} reports.
    /// O(1) instead of a full replay — see {@link snapshot}.
    pub fn snapshot_with_conflicts(&self) -> Result<(P, Vec<SpaceConflict>), VcsError> {
        Ok(reconcile_with_last(self.last_applied_operation(), self.current.clone()))
    }

    /// @emoji 🎞️ The last-applied edit's last forward operation — the instance `reconcile_with_last`
    /// runs `Mutation::reconcile` against (see that fn's doc comment for why any single instance is
    /// equivalent to the old per-TYPE associated-fn call for every technology in this repo today).
    fn last_applied_operation(&self) -> Option<&Mutation> {
        self.applied_edit_ids.last().and_then(|edit_id| self.envelope.vcs.edits.iter().find(|edit| edit.id == *edit_id)).and_then(|edit| edit.forwards.last())
    }

    /// @emoji 🔂️ Full raw fold of `initial_snapshot` over every `forwards` op in `applied_edit_ids`
    /// order, WITHOUT the final `Mutation::reconcile` pass — the from-scratch computation `current`
    /// is an incrementally-maintained cache of. Used to recompute `current` on the cold paths that
    /// reassign `applied_edit_ids` wholesale instead of appending/popping its tail.
    fn fold_current(&self) -> Result<P, VcsError> {
        let mut snapshot = self.envelope.vcs.initial_snapshot.clone();
        for edit_id in &self.applied_edit_ids {
            let edit = self.envelope.vcs.edits.iter().find(|entry| entry.id == *edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
            for operation in &edit.forwards {
                snapshot = apply_mutation(&snapshot, operation);
            }
        }
        Ok(snapshot)
    }

    /// @emoji 🤝️ Conflicts from the last reconciliation pass (see {@link conflicts} field doc).
    pub fn conflicts(&self) -> &[SpaceConflict] {
        &self.conflicts
    }

    pub fn dispatch(&mut self, command: ArtifactCommand<Mutation>) -> Result<CommandReceipt, VcsError> {
        self.pump()?;
        let skip_flush = matches!(command, ArtifactCommand::IngestRemote { .. } | ArtifactCommand::PruneDrafts);
        let is_apply = matches!(command, ArtifactCommand::Apply { .. });
        let before = self.applied_edit_ids.len();
        self.dispatch_inner(command)?;
        if !skip_flush {
            self.flush_outbound(is_apply)?;
        }
        // Undo/redo shrink `applied_edit_ids`; only Apply/AmendLast append past `before`.
        let edit_ids = if self.applied_edit_ids.len() >= before {
            self.applied_edit_ids[before..].to_vec()
        } else {
            Vec::new()
        };
        Ok(CommandReceipt {
            edit_ids,
            generation: self.generation(),
        })
    }

    fn dispatch_inner(&mut self, command: ArtifactCommand<Mutation>) -> Result<(), VcsError> {
        match command {
            ArtifactCommand::Undo => self.dispatch(ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::ExactBaseOnly, semantic_command: None }).map(|_| ()),
            ArtifactCommand::UndoWithPolicy { policy, semantic_command } => match policy {
                UndoPolicy::ExactBaseOnly => {
                    let last = self.applied_edit_ids.last().cloned().ok_or(VcsError::NothingToUndo)?;
                    if !self.edit_is_local(&last) {
                        return Err(VcsError::ForeignEdit(last));
                    }
                    self.applied_edit_ids.pop();
                    self.redo_edit_ids.push(last.clone());
                    // ⚡️ O(1) fast path when undoing exactly the cached tail edit; any other shape
                    // (cache miss, or a prior mid-history undo already invalidated it) falls back to a
                    // full raw-fold recompute — always correct, see `fold_current`.
                    match self.tail_undo_cache.take() {
                        Some((cached_id, cached_pre)) if cached_id == last => {
                            self.current = cached_pre;
                        }
                        _ => {
                            self.current = self.fold_current()?;
                        }
                    }
                    self.bump();
                    Ok(())
                }
                UndoPolicy::TransformAgainstConcurrent => {
                    let position = self.applied_edit_ids.iter().rposition(|id| self.edit_is_local(id)).ok_or(VcsError::NothingToUndo)?;
                    let removed = self.applied_edit_ids.remove(position);
                    self.redo_edit_ids.push(removed);
                    // 🔂️ Removing a MID-history edit has no cheap incremental inverse; cold-path replay.
                    self.tail_undo_cache = None;
                    self.current = self.fold_current()?;
                    self.bump();
                    Ok(())
                }
                UndoPolicy::SemanticUndo | UndoPolicy::CompensatingAction => {
                    let command = semantic_command.ok_or_else(|| VcsError::Backbone("semantic undo requires compensating command".into()))?;
                    self.dispatch_inner(*command)
                }
            },
            ArtifactCommand::Redo => {
                let next = self.redo_edit_ids.pop().ok_or(VcsError::NothingToRedo)?;
                self.applied_edit_ids.push(next.clone());
                // ⚡️ Fold the redone edit's forwards onto `current` in their own natural order — cheap
                // and correct regardless of the edit's internal op grouping (unlike undo, this never
                // needs `Edit.inverse`). Re-seeds `tail_undo_cache` so a following Undo is O(1) again.
                if let Some(edit) = self.envelope.vcs.edits.iter().find(|entry| entry.id == next) {
                    let pre = self.current.clone();
                    let mut folded = pre.clone();
                    for operation in &edit.forwards {
                        folded = apply_mutation(&folded, operation);
                    }
                    self.current = folded;
                    self.tail_undo_cache = Some((next, pre));
                }
                self.bump();
                Ok(())
            }
            ArtifactCommand::CommitCheckpoint { message, authors } => {
                let pending = uncommitted_edit_ids(&self.envelope, &self.applied_edit_ids);
                if pending.is_empty() {
                    return Ok(());
                }
                let change = Change { id: mint_change_id(&pending, message.as_deref()), edit_ids: pending, description: message.clone(), saved_at: now_iso() };
                let parent = self.current_checkpoint_id.as_ref().and_then(|id| self.envelope.vcs.checkpoints.iter().find(|cp| cp.id == *id));
                let mut change_ids = parent.map(|cp| cp.change_ids.clone()).unwrap_or_default();
                let parent_id = parent.map(|cp| cp.id.clone());
                change_ids.push(change.id.clone());
                // 🎞️ CW3: the new change is pushed BEFORE computing the checkpoint id (was after),
                // so `content_addressed_checkpoint_id` can hash its actual content, not a placeholder.
                self.envelope.vcs.changes.push(change);
                let timestamp = now_iso();
                // 🎯️ `&[]`: `ArtifactStore<P, Mutation>` has no notion of owned children yet — the
                // `CompositionCoordinator` that dispatches across parent + child stores and
                // populates real `CompositionPin`s here is a later wave (see design doc §1).
                let id = content_addressed_checkpoint_id(parent_id.as_deref(), &change_ids, &self.envelope.vcs.changes, message.as_deref(), &authors, &timestamp, &[]);
                let checkpoint = Checkpoint { id, change_ids, parent_id, authors, message, timestamp, composition_pins: Vec::new() };
                let checkpoint_id = checkpoint.id.clone();
                self.envelope.vcs.checkpoints.push(checkpoint);
                if let Some(alternative_id) = self.envelope.active_alternative_id.clone() {
                    if let Some(alternative) = self.envelope.vcs.alternatives.iter_mut().find(|alt| alt.id == alternative_id) {
                        alternative.checkpoint_ids.push(checkpoint_id.clone());
                    }
                }
                self.current_checkpoint_id = Some(checkpoint_id);
                self.bump();
                Ok(())
            }
            ArtifactCommand::CreateAlternative { name } => {
                if self.envelope.vcs.checkpoints.is_empty() {
                    self.dispatch(ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() })?;
                }
                let checkpoint_id = self.current_checkpoint_id.clone().or_else(|| self.envelope.vcs.checkpoints.last().map(|cp| cp.id.clone())).ok_or(VcsError::NoCheckpoint)?;
                let alt_id = mint_alternative_id(&name, &[checkpoint_id.clone()]);
                self.envelope.vcs.alternatives.push(Alternative { id: alt_id.clone(), name, checkpoint_ids: vec![checkpoint_id.clone()] });
                self.envelope.active_alternative_id = Some(alt_id);
                self.checkout_checkpoint_internal(checkpoint_id);
                self.bump();
                Ok(())
            }
            ArtifactCommand::SwitchAlternative { alternative_id } => {
                let alternative = self.envelope.vcs.alternatives.iter().find(|alt| alt.id == alternative_id).ok_or_else(|| VcsError::UnknownAlternative(alternative_id.clone()))?.clone();
                let checkpoint_id = alternative.checkpoint_ids.last().ok_or(VcsError::NoCheckpoint)?.clone();
                if !self.envelope.vcs.checkpoints.iter().any(|cp| cp.id == checkpoint_id) {
                    return Err(VcsError::NoCheckpoint);
                }
                self.checkout_checkpoint_internal(checkpoint_id);
                self.envelope.active_alternative_id = Some(alternative_id);
                self.bump();
                Ok(())
            }
            ArtifactCommand::CheckoutCheckpoint { checkpoint_id } => {
                if !self.envelope.vcs.checkpoints.iter().any(|cp| cp.id == checkpoint_id) {
                    return Err(VcsError::UnknownChange(checkpoint_id.clone()));
                }
                self.checkout_checkpoint_internal(checkpoint_id.clone());
                self.envelope.active_alternative_id = self.envelope.vcs.alternatives.iter().find(|alt| alt.checkpoint_ids.last() == Some(&checkpoint_id)).map(|alt| alt.id.clone());
                self.bump();
                Ok(())
            }
            ArtifactCommand::Apply { mutations, description } => {
                if mutations.is_empty() {
                    return Err(VcsError::EmptyApply);
                }
                let started_at = now_iso();
                // ⚡️ `current` is always up to date (maintained by every mutating command below), so
                // this is an O(1) clone instead of a full replay — see the `current` field doc.
                let pre_snapshot = self.current.clone();
                let (forwards, inverse, mutation_meta, post) = Self::replay_mutations(&pre_snapshot, mutations);
                let actor = edit_actor_from_meta(&mutation_meta);
                self.local_actor_id = actor.clone();
                self.edit_sequence += 1;
                let forwards_fingerprint = serde_json::to_vec(&forwards).unwrap_or_default();
                let edit = Edit { id: mint_edit_id(actor.as_deref(), self.edit_sequence, &forwards_fingerprint), actor, forwards, inverse, mutation_meta, description, coalesce_key: None, sequence_number: self.edit_sequence, started_at, finished_at: Some(now_iso()) };
                self.tail_undo_cache = Some((edit.id.clone(), pre_snapshot));
                self.applied_edit_ids.push(edit.id.clone());
                self.envelope.vcs.edits.push(edit);
                self.current = post;
                self.redo_edit_ids.clear();
                self.bump();
                Ok(())
            }
            ArtifactCommand::AmendLast { mutations, coalesce_key } => {
                if mutations.is_empty() {
                    return Err(VcsError::EmptyApply);
                }
                let amend_target = self.applied_edit_ids.last().cloned().filter(|last_id| {
                    coalesce_key.is_some()
                        && uncommitted_edit_ids(&self.envelope, &self.applied_edit_ids).contains(last_id)
                        && self.envelope.vcs.edits.iter().find(|edit| edit.id == *last_id).map(|edit| edit.coalesce_key == coalesce_key).unwrap_or(false)
                });
                if let Some(edit_id) = amend_target {
                    // ⚡️ `current` already reflects this edit's existing forwards (it was folded in
                    // when the edit was created or last amended), so it's always the correct base for
                    // the NEW operations — O(1) instead of the old cache-validity dance.
                    let pre_snapshot = self.current.clone();
                    let (new_forwards, new_inverse, new_mutation_meta, post) = Self::replay_mutations(&pre_snapshot, mutations);
                    if let Some(edit) = self.envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id) {
                        edit.forwards.extend(new_forwards);
                        edit.inverse.extend(new_inverse);
                        edit.mutation_meta.extend(new_mutation_meta);
                        edit.finished_at = Some(now_iso());
                    }
                    self.current = post;
                    self.redo_edit_ids.clear();
                    self.bump();
                    Ok(())
                } else {
                    let started_at = now_iso();
                    let pre_snapshot = self.current.clone();
                    let (forwards, inverse, mutation_meta, post) = Self::replay_mutations(&pre_snapshot, mutations);
                    let actor = edit_actor_from_meta(&mutation_meta);
                    self.local_actor_id = actor.clone();
                    self.edit_sequence += 1;
                    let forwards_fingerprint = serde_json::to_vec(&forwards).unwrap_or_default();
                    let edit_id = mint_edit_id(actor.as_deref(), self.edit_sequence, &forwards_fingerprint);
                    let edit = Edit { id: edit_id.clone(), actor, forwards, inverse, mutation_meta, description: None, coalesce_key, sequence_number: self.edit_sequence, started_at, finished_at: Some(now_iso()) };
                    self.tail_undo_cache = Some((edit_id, pre_snapshot));
                    self.applied_edit_ids.push(edit.id.clone());
                    self.envelope.vcs.edits.push(edit);
                    self.current = post;
                    self.redo_edit_ids.clear();
                    self.bump();
                    Ok(())
                }
            }
            ArtifactCommand::IngestRemote { envelope } => {
                let _receipt = self.ingest_remote(envelope)?;
                Ok(())
            }
            ArtifactCommand::PruneDrafts => {
                // Reserved for draft-lane stores ({@link DraftStore}): real prune lands with draft ops.
                Ok(())
            }
        }
    }

    /// @emoji 🔂️ Replays `operations` over `pre_snapshot`, returning forwards, reversed-inverse,
    /// per-operation metadata, and the resulting snapshot. Shared by `Apply` and `AmendLast`. This
    /// IS the artifact engine — `crate::os_engine::ArtifactEngine` never existed as a live trait
    /// (see `.claude/plans/the-mutations-are-extremely-compiled-pumpkin.md`), so `Mutation::diff`/
    /// `inverse` are called directly here on purpose, not as a placeholder for a future indirection.
    fn replay_mutations(pre_snapshot: &P, mutations: Vec<Mutation>) -> (Vec<Mutation>, Vec<Mutation>, Vec<MutationMeta>, P) {
        let mut snapshot = pre_snapshot.clone();
        let mut forwards = Vec::with_capacity(mutations.len());
        let mut inverse = Vec::new();
        let mut mutation_meta = Vec::with_capacity(mutations.len());
        for mutation in mutations {
            let mut back = mutation.inverse(&snapshot);
            back.reverse();
            inverse.extend(back);
            mutation_meta.push(MutationMeta {
                mutation_id: Some(mutation.mutation_id().unwrap_or_else(|| MutationId(mint_mutation_id(&mutation.encode_op().unwrap_or_default())))),
                dependencies: mutation.dependencies(),
                base_version: mutation.base_version().map(|version| version.0).unwrap_or(0),
                author_id: Some(mutation.author_id().unwrap_or_else(|| ActorId("local".into()))),
                timestamp: mutation.timestamp().unwrap_or_else(|| HybridLogicalTimestamp::new(0, now_ms())),
                undo_policy: mutation.undo_policy(),
                // 🎞️ CW3: direct blake3 (same primitive `crate::os_pack::ContentHash` uses) replaces the
                // old `framework_hash::hash_bytes` String hash — `crate::os_spr::PayloadHash` is
                // now `[u8; 32]`, not a hex string. NOT `crate::os_pack::content_hash`, which reads a pack
                // FILE's footer rather than hashing arbitrary bytes. 🎯️ B2: hashes the real
                // `OpBinary` encoding, not a JSON serialization — two ops that encode identically
                // via `encode_op()` but differ in JSON shape (or vice versa) must hash identically.
                payload_hash: Some(crate::os_spr::PayloadHash(*blake3::hash(&mutation.encode_op().unwrap_or_default()).as_bytes())),
                semantic_kind: None,
                label: None,
                group_id: None,
            });
            snapshot = apply_mutation(&snapshot, &mutation);
            forwards.push(mutation);
        }
        (forwards, inverse, mutation_meta, snapshot)
    }

    /// @emoji 🕹️ Parses `command_text` via [`parse_command`] and dispatches it — the op-line
    /// textual entry point (op-efficient one-line-per-structural-field commands, indented op
    /// lines for `Apply`/`AmendLast`).
    pub fn dispatch_text(&mut self, command_text: &str) -> Result<CommandReceipt, VcsError>
    where
        Mutation: OpText,
    {
        let command = parse_command(command_text).map_err(|error| VcsError::Deserialize(error.to_string()))?;
        self.dispatch(command)
    }

    /// @emoji 🕹️ Decodes `command_bytes` via [`decode_command`] and dispatches it — the binary
    /// entry point used for both communication (backbone/semio_hub) and storage (`.spr`).
    pub fn dispatch_binary(&mut self, command_bytes: &[u8]) -> Result<CommandReceipt, VcsError>
    where
        Mutation: OpBinary,
    {
        let command = <ArtifactCommand<Mutation> as OpBinary>::decode_op(command_bytes).map_err(|error| VcsError::Deserialize(error.to_string()))?;
        self.dispatch(command)
    }

    /// @emoji 📸️ The whole-document snapshot as real `pack`+`spr` bytes — what `flush_outbound`
    /// sends over `BackboneMessage::Snapshot` and what any other caller needing a full-fidelity
    /// binary snapshot (never JSON) should call.
    pub fn snapshot_pack(&self) -> Result<ArtifactPackFiles, VcsError> {
        print_document_pack(&self.envelope)
    }

    pub fn snapshot_json(&self) -> Result<String, VcsError> {
        let snapshot = self.snapshot()?;
        serde_json::to_string(&snapshot).map_err(|e| VcsError::Serialize(e.to_string()))
    }

    /// @emoji 📦️ Serializes the full document envelope (snapshot + VCS history) as JSON.
    pub fn envelope_json(&self) -> Result<String, VcsError> {
        serde_json::to_string(&self.envelope).map_err(|e| VcsError::Serialize(e.to_string()))
    }

    /// @emoji 🔗️ Attaches a backbone channel, reconciling any already-persisted state before
    /// seeding it with this store's current snapshot.
    pub fn attach_backbone(&mut self, backbone: Box<dyn Backbone>) -> Result<(), VcsError> {
        self.envelope.backbone = Some(backbone.descriptor());
        self.backbone = Some(backbone);
        self.pump()?;
        self.flush_outbound(false)?;
        self.bump();
        Ok(())
    }

    /// @emoji 🔗️ Resolves a backbone URI and attaches it. Only available inside the wasm sandbox,
    /// where every scheme forwards to the host over the injected {@link BackboneChannelPort} (a pure
    /// queue). On native targets, callers attach an explicit `Box<dyn Backbone>` via
    /// {@link attach_backbone} — the `framework/sync` actor layer owns all IO-performing endpoints.
    #[cfg(target_arch = "wasm32")]
    pub fn attach_backbone_uri(&mut self, uri: &str) -> Result<(), VcsError> {
        self.attach_backbone(resolve_backbone(uri)?)
    }

    /// @emoji ✂️ Detaches the backbone; the WIP graph stays in memory, simply unsynchronized.
    pub fn detach_backbone(&mut self) -> Option<Box<dyn Backbone>> {
        self.envelope.backbone = None;
        self.bump();
        self.backbone.take()
    }

    pub fn backbone_ref(&self) -> Option<&ArtifactBackboneRef> {
        self.envelope.backbone.as_ref()
    }

    /// @emoji 📡️ Drains inbound backbone messages into the edit timeline. Safe to call anytime;
    /// `dispatch` already calls this before every command.
    pub fn tick(&mut self) -> Result<bool, VcsError> {
        self.pump()
    }

    /// @emoji 🕸️ Feeds a remote {@link MutationEnvelope} through the causal DAG, applying it (and any
    /// now-unblocked dependents) into the edit timeline. Closes the sync gap between
    /// `framework/sync`'s `MutationDag` and the vcs edit history.
    /// @emoji 🕸️ Sole public remote write gate — parallel to `dispatch` for causal envelopes.
    pub(crate) fn ingest_remote(&mut self, envelope: crate::os_spr::MutationEnvelope) -> Result<(), VcsError> {
        self.dag.insert(envelope).map_err(|error| VcsError::Backbone(error.to_string()))?;
        for envelope in self.dag.drain_applied_envelopes() {
            self.ingest_envelope(envelope)?;
        }
        Ok(())
    }

    fn ingest_envelope(&mut self, envelope: crate::os_spr::MutationEnvelope) -> Result<(), VcsError> {
        let mut edit: Edit<Mutation> = edit_from_operation_envelope(&envelope)?;
        edit.actor = Some(envelope.actor.0.clone());
        if self.envelope.vcs.edits.iter().any(|existing| existing.id == edit.id) {
            return Ok(());
        }
        self.edit_sequence += 1;
        edit.sequence_number = self.edit_sequence;
        edit.started_at = now_iso();
        let edit_id = edit.id.clone();
        // ⚡️ Fold just the new edit's forwards onto the existing `current` (which already reflects
        // every prior applied edit) — algebraically identical to a full raw-fold replay, in O(new ops).
        for operation in &edit.forwards {
            self.current = apply_mutation(&self.current, operation);
        }
        self.envelope.vcs.edits.push(edit);
        self.applied_edit_ids.push(edit_id);
        self.tail_undo_cache = None;
        // 🤝️ Tail reconciliation hook: remote ingestion is the one path where this store's snapshot
        // can diverge from what a local `Apply` alone would produce, so refresh conflicts here.
        let (_, conflicts) = reconcile_with_last(self.last_applied_operation(), self.current.clone());
        self.conflicts = conflicts;
        self.bump();
        Ok(())
    }

    fn merge_remote_snapshot(&mut self, pack: &[u8], spr: &[u8]) -> Result<(), VcsError> {
        let remote: ArtifactEnvelope<P, Mutation> = parse_document_pack(pack, spr).map_err(|error| VcsError::Deserialize(error.to_string()))?.envelope;
        if self.envelope.vcs.edits.is_empty() {
            let applied: Vec<String> = remote.vcs.edits.iter().map(|edit| edit.id.clone()).collect();
            self.edit_sequence = remote.vcs.edits.iter().map(|edit| edit.sequence_number).max().unwrap_or(0);
            let backbone_ref = self.envelope.backbone.clone();
            self.envelope = remote;
            self.envelope.backbone = backbone_ref;
            // 🌱️ A snapshot adopts these edits directly (not through `dag.insert`), so the dag never
            // learns they're satisfied — seed it here or a later envelope whose `deps` point back at
            // one of these ids would sit `Pending` forever (see `MutationDag::seed_applied`). Seed each
            // edit's own id AND its per-op WIRE ids (`crate::os_spr::mutation_ids_for_edit` — the same
            // ids `ingest_envelope` would key a remote copy of these ops under, see the double-
            // delivery note below) so a `BackboneMessage::Mutations` for one of these ops that
            // arrives later is recognized as `AlreadyApplied` instead of re-materializing it.
            for edit in &self.envelope.vcs.edits {
                self.dag.seed_applied(MutationId(edit.id.clone()));
                for mutation_id in crate::os_spr::mutation_ids_for_edit(edit) {
                    self.dag.seed_applied(mutation_id);
                }
            }
            self.applied_edit_ids = applied;
            self.redo_edit_ids.clear();
            self.tail_undo_cache = None;
            // 🔂️ Wholesale replacement, not a tail append — cold-path full raw-fold recompute.
            self.current = self.fold_current()?;
            self.bump();
            return Ok(());
        }
        // 🪪️ An edit's top-level `id` is NOT a stable cross-store identity: `ingest_envelope` (the
        // `BackboneMessage::Mutations` path) reconstructs a remote op under its WIRE id
        // (`envelope.mutation_id`, from `crate::os_spr::mutation_ids_for_edit`/`mutation_meta`), which
        // differs from the id the op's own edit carries on the store that authored it. Without also
        // indexing by each known edit's derived op ids, a snapshot re-broadcasting an edit this store
        // already ingested via Operations (under that different wire id) reads as "new" and gets
        // merged a second time — confirmed double-delivery: harmless for idempotent patch-style ops,
        // but a visible duplicate for insert-style ops (see raster's `addLayer` convergence test).
        let mut known_ids: HashSet<String> = HashSet::new();
        for edit in &self.envelope.vcs.edits {
            known_ids.insert(edit.id.clone());
            known_ids.extend(crate::os_spr::mutation_ids_for_edit(edit).into_iter().map(|id| id.0));
        }
        let mut newly_merged_ids: Vec<String> = Vec::new();
        for edit in remote.vcs.edits {
            let operation_ids = crate::os_spr::mutation_ids_for_edit(&edit);
            let already_known = known_ids.contains(&edit.id) || (!operation_ids.is_empty() && operation_ids.iter().all(|id| known_ids.contains(&id.0)));
            if already_known {
                continue;
            }
            self.edit_sequence = self.edit_sequence.max(edit.sequence_number);
            self.applied_edit_ids.push(edit.id.clone());
            newly_merged_ids.push(edit.id.clone());
            known_ids.insert(edit.id.clone());
            known_ids.extend(operation_ids.iter().map(|id| id.0.clone()));
            // ⚡️ Each newly-merged edit is appended at the tail, so folding its forwards onto `current`
            // in iteration order is exactly a prefix-extension of the existing raw fold.
            for operation in &edit.forwards {
                self.current = apply_mutation(&self.current, operation);
            }
            for mutation_id in operation_ids {
                self.dag.seed_applied(mutation_id);
            }
            self.envelope.vcs.edits.push(edit);
        }
        for edit_id in &newly_merged_ids {
            self.dag.seed_applied(MutationId(edit_id.clone()));
        }
        merge_by_id(&mut self.envelope.vcs.changes, remote.vcs.changes, |change| &change.id);
        merge_by_id(&mut self.envelope.vcs.checkpoints, remote.vcs.checkpoints, |checkpoint| &checkpoint.id);
        merge_by_id(&mut self.envelope.vcs.alternatives, remote.vcs.alternatives, |alternative| &alternative.id);
        self.tail_undo_cache = None;
        self.bump();
        Ok(())
    }

    /// @emoji 📥️ Pumps every queued inbound message from the attached backbone into the timeline.
    fn pump(&mut self) -> Result<bool, VcsError> {
        let Some(mut backbone) = self.backbone.take() else {
            return Ok(false);
        };
        let received = backbone.receive();
        self.backbone = Some(backbone);
        let messages = received?;
        if messages.is_empty() {
            return Ok(false);
        }
        let mut acked_op_ids: Vec<String> = Vec::new();
        for message in messages {
            match message {
                BackboneMessage::Snapshot { pack, spr } => self.merge_remote_snapshot(&pack, &spr)?,
                BackboneMessage::Mutations { envelopes } => {
                    let envelopes = crate::os_spr::decode_envelopes(&envelopes).map_err(|error| VcsError::Deserialize(error.to_string()))?;
                    let op_ids: Vec<String> = envelopes.iter().map(|envelope| envelope.mutation_id.0.clone()).collect();
                    for envelope in envelopes {
                        self.ingest_remote(envelope)?;
                    }
                    acked_op_ids.extend(op_ids);
                }
                // A store never consumes acks (they flow store→actor); drain and ignore any that echo back.
                BackboneMessage::Ack { .. } => {}
            }
        }
        if !acked_op_ids.is_empty() {
            if let Some(mut backbone) = self.backbone.take() {
                let result = backbone.send(BackboneMessage::Ack { op_ids: acked_op_ids });
                self.backbone = Some(backbone);
                result?;
            }
        }
        Ok(true)
    }

    /// @emoji 📤️ Sends the just-applied change outward: one {@link crate::os_spr::MutationEnvelope} per
    /// forward op for `Apply` (`crate::os_spr::mutation_envelope_from_edit`'s per-op fan-out — W5/W6),
    /// or a full snapshot for every structural command (undo/redo/checkpoint/alternative/amend).
    fn flush_outbound(&mut self, is_apply: bool) -> Result<(), VcsError> {
        let Some(mut backbone) = self.backbone.take() else {
            return Ok(());
        };
        let result = if is_apply {
            match self.envelope.vcs.edits.last() {
                Some(edit) => {
                    let document_id = ArtifactId(self.envelope.id.clone());
                    let schema = SchemaId(self.envelope.schema.clone());
                    match crate::os_spr::mutation_envelope_from_edit::<P, Mutation>(edit, &document_id, &schema) {
                        Ok(op_envelopes) => {
                            // Registers these locally-authored ops as already-applied in our own
                            // DAG, so a later remote envelope depending on one doesn't stall as
                            // pending. `seed_applied` (out-of-band knowledge, mark-only) — NOT
                            // `insert` (which stores the envelope for later `drain_applied_
                            // envelopes()` too), or the next real remote `ingest_remote` call on
                            // this same store would drain and re-materialize this already-local
                            // edit as a SECOND, duplicate edit under its wire mutation_id (which
                            // differs from the edit's own local id, so `ingest_envelope`'s by-id
                            // dedup check never catches it).
                            for op_envelope in &op_envelopes {
                                self.dag.seed_applied(op_envelope.mutation_id.clone());
                            }
                            backbone.send(BackboneMessage::Mutations { envelopes: crate::os_spr::encode_envelopes(&op_envelopes) })
                        }
                        Err(error) => Err(VcsError::Serialize(error.to_string())),
                    }
                }
                None => Ok(()),
            }
        } else {
            self.snapshot_pack().and_then(|files| backbone.send(BackboneMessage::Snapshot { pack: files.pack, spr: files.spr }))
        };
        self.backbone = Some(backbone);
        result
    }

    /// @emoji 🖋️ Whether `edit_id` was authored by the local actor. Unauthored (legacy) edits count
    /// as local; every other actor is foreign and must not be undone by this store.
    fn edit_is_local(&self, edit_id: &str) -> bool {
        self.envelope.vcs.edits.iter().find(|edit| edit.id == edit_id).map(|edit| edit.actor.is_none() || edit.actor.as_deref() == self.local_actor_id.as_deref()).unwrap_or(false)
    }

    /// @emoji 🎯️ Mirrors `applied_edit_ids`/`redo_edit_ids`/`current_checkpoint_id` into
    /// `envelope.cursor` — the single choke point that keeps the persisted cursor in sync with
    /// live undo/redo state. Called from every `bump()`, so every mutating command re-syncs it.
    fn sync_cursor(&mut self) {
        self.envelope.cursor = Some(ArtifactCursor { applied_edit_ids: self.applied_edit_ids.clone(), redo_edit_ids: self.redo_edit_ids.clone(), checkpoint_id: self.current_checkpoint_id.clone() });
    }

    fn bump(&mut self) {
        self.generation += 1;
        self.sync_cursor();
    }
}

fn merge_by_id<T: Clone>(local: &mut Vec<T>, remote: Vec<T>, id_of: impl Fn(&T) -> &String) {
    let mut existing: HashSet<String> = local.iter().map(|item| id_of(item).clone()).collect();
    for item in remote {
        if existing.insert(id_of(&item).clone()) {
            local.push(item);
        }
    }
}

// 🎯️ W6 kernel unification: this crate's own `mutation_envelope_from_edit` (whole-edit-per-
// envelope, JSON payload) is DELETED — `flush_outbound` now calls `crate::os_spr::
// mutation_envelope_from_edit` directly (one `crate::os_spr::MutationEnvelope` per forward op,
// `OpBinary`-encoded payloads — W5's frozen-contract signature). `hash_bytes`'s import above this
// region stays needed elsewhere in this file (`replay_mutations`'s `payload_hash`, unaffected).

/// @emoji 📦️ Recovers a single-op `Edit` from one causal wire envelope. `crate::os_spr::causal::
/// MutationEnvelope` carries exactly one op per envelope (W5's binary reshape) — the receiving-
/// side half of the per-op fan-out `crate::os_spr::mutation_envelope_from_edit` performs when sending
/// (see `flush_outbound`). `sequence_number`/`started_at` are placeholders `ingest_envelope`
/// overwrites (mirroring the local-edit convention: `self.edit_sequence += 1` then `now_iso()`).
/// `undo_policy` defaults to `ExactBaseOnly` — not a lossy conversion: `crate::os_spr::causal::
/// MutationEnvelope` carries no undo_policy at all (only the local `Edit`/`MutationMeta` shape
/// does), and a remote edit is always foreign, so this field is never consulted for it anyway
/// (`edit_is_local` gates undo eligibility on authorship, not `undo_policy`).
pub fn edit_from_operation_envelope<Mutation: OpBinary>(envelope: &crate::os_spr::MutationEnvelope) -> Result<Edit<Mutation>, VcsError> {
    let forward = Mutation::decode_op(&envelope.diff.payload).map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let inverse = if envelope.inverse.payload.is_empty() { Vec::new() } else { vec![Mutation::decode_op(&envelope.inverse.payload).map_err(|error| VcsError::Deserialize(error.to_string()))?] };
    Ok(Edit {
        id: envelope.mutation_id.0.clone(),
        actor: Some(envelope.actor.0.clone()),
        forwards: vec![forward],
        inverse,
        mutation_meta: vec![MutationMeta {
            mutation_id: Some(envelope.mutation_id.clone()),
            dependencies: envelope.dependencies.clone(),
            base_version: 0,
            author_id: Some(envelope.actor.clone()),
            timestamp: envelope.timestamp,
            undo_policy: UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: None,
            label: None,
            group_id: None,
        }],
        description: None,
        coalesce_key: None,
        sequence_number: 0,
        started_at: String::new(),
        finished_at: None,
    })
}
//#endregion 🔖️ArtifactStore

//#region 🔖️Backbone
//#region 🔖️Backbone
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceConflict {
    pub kind: String,
    pub uri: String,
    pub message: String,
}

/// @emoji 🎞️ Maps `crate::os_spr::command::Mutation::reconcile`'s new `Vec<ReconcileReport>` result onto
/// this crate's own conflict type — see `reconcile_with_last`'s doc comment for why the mapping
/// happens at this edge rather than `protocol_command` knowing about `SpaceConflict` directly.
/// `kind: report.id` verbatim (NOT prefixed with severity) — a technology's own `reconcile` override
/// (e.g. `framework/product/os/core`'s `OsMutation`) round-trips its own `SpaceConflict.kind`
/// through `ReconcileReport.id` on the way in (see that crate's `reconcile` wrapper), and callers
/// pattern-match `SpaceConflict.kind` against exact strings (e.g. `"workflow/edge-orphaned"`) —
/// mangling it here would silently break every such exact-match call site. `severity` has no
/// `SpaceConflict` field to land in, so it is dropped (a real, structural information loss inherent
/// to `ReconcileReport`'s frozen shape, not fixable at this edge). `ReconcileReport` also has no
/// URI-shaped field (it targets a schema-opaque `id`, not a space member resource), so `uri` is
/// left empty for any report that didn't originate from a `SpaceConflict` round-trip.
impl From<ReconcileReport> for SpaceConflict {
    fn from(report: ReconcileReport) -> Self {
        SpaceConflict { kind: report.id, uri: String::new(), message: report.message }
    }
}

/// @emoji 📨️ Wire message exchanged over an attached backbone channel. B-R6 "kill hand-rolled binary
/// codecs": `#[derive(crate::os_dsl::DslOps)]` generates `OpBinary::encode_op`/`decode_op` (`format u8 (=1) |
/// variant ordinal varint | record body`, `crate::os_dsl::op_rt`) — this is the one real binary encoding for
/// every caller, including the wasm-sandbox `BackboneChannelPort` seam (see that trait's doc) — the
/// WIT `backbone-send`/`backbone-poll` host functions carry these exact bytes as `list<u8>`.
/// `Operations.envelopes` carries `crate::os_spr::encode_envelopes`/`decode_envelopes` bytes rather than a
/// real `Vec<crate::os_spr::MutationEnvelope>` field: `MutationEnvelope` lives in `protocol_causal`,
/// which sits BELOW `dsl` in the dependency graph (`dsl` → `protocol` → `protocol_causal`), so it
/// cannot implement `crate::os_dsl::DslField` without a dependency cycle — the derive can only lower fields
/// shaped from types it can see. `#[dsl(base64)]` marks each `Vec<u8>` field `Shape::Bytes64`
/// (otherwise a bare `Vec<u8>` lowers to a `List<UInt>`, one DSL list element per byte).
#[derive(Clone, Debug, PartialEq, DslOps)]
pub enum BackboneMessage {
    Snapshot {
        #[dsl(base64)]
        pack: Vec<u8>,
        #[dsl(base64)]
        spr: Vec<u8>,
    },
    Mutations {
        #[dsl(base64)]
        envelopes: Vec<u8>,
    },
    /// @emoji ✅️ Acknowledges inbound operations the store has ingested (store→actor). Lets a future actor
    /// implement at-least-once redelivery with id-based dedupe — safe across store crashes/reloads.
    Ack {
        op_ids: Vec<String>,
    },
}

//#region 🔖️OpCodec
/// 🎞️ Handcrafted OpText (P6).
impl OpText for BackboneMessage {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = crate::os_dsl::parse(
                    line,
                    &spec_fn(),
                    &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline },
                )?;
                return <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(crate::os_dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl OpBinary for BackboneMessage {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(crate::os_spr::ProtocolError::Malformed {
            what: "op variant",
            offset: 0,
            detail: format!("keyword {keyword:?} is not a declared variant"),
        })?;
        let spec = (variants[ordinal].1)();
        let body = crate::os_pack::encode_record_body(&spec, &record, &PackEncodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        crate::os_pack::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = crate::os_pack::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(crate::os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(crate::os_spr::ProtocolError::Malformed {
            what: "op variant",
            offset: 1,
            detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()),
        })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
        <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| crate::os_spr::ProtocolError::Malformed {
            what: "op record",
            offset: reader.position() as u64,
            detail: error.to_string(),
        })
    }
}
//#endregion 🔖️OpCodec


/// @emoji 🧵️ Non-blocking, IO-free in-memory queue contract between a `ArtifactStore` and its
/// sync actor. `send`/`receive` MUST return immediately: implementations only enqueue/dequeue
/// `BackboneMessage`s — never HTTP, never filesystem, never a blocking wait. All IO (persistence,
/// semio_hub sync, file watching, presence) lives behind this queue in `framework/sync`'s actor layer,
/// which owns the other end; the store's `pump()`/`flush_outbound()` run synchronously on the
/// caller's thread and must never be blocked by transport work.
///
/// URI schemes are resolved by the host actor (`framework/sync`): `temp://` (in-memory),
/// `file://` (single JSON blob), `folder://` (sqlite `.semio/document.db`), `remote://` (OS semio_hub).
pub trait Backbone: Send + Sync {
    fn descriptor(&self) -> ArtifactBackboneRef;
    fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError>;
    fn receive(&mut self) -> Result<Vec<BackboneMessage>, VcsError>;
}

pub trait BackbonePort: Send + Sync {
    fn read(&self, uri: &str) -> Result<String, VcsError>;
    fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError>;
}

static HOST_BACKBONE_PORT: Mutex<Option<Arc<dyn BackbonePort>>> = Mutex::new(None);

/// @emoji 🔌️ Injects the browser or dev-server backbone port for wasm file/folder IO.
pub fn set_host_backbone_port(port: Arc<dyn BackbonePort>) {
    if let Ok(mut guard) = HOST_BACKBONE_PORT.lock() {
        *guard = Some(port);
    }
}

fn host_backbone_port() -> Option<Arc<dyn BackbonePort>> {
    HOST_BACKBONE_PORT.lock().ok().and_then(|guard| guard.clone())
}

#[derive(Default)]
pub struct MemoryBackbonePort {
    files: Mutex<HashMap<String, String>>,
}

impl MemoryBackbonePort {
    pub fn new() -> Self {
        Self::default()
    }
}

impl BackbonePort for MemoryBackbonePort {
    fn read(&self, uri: &str) -> Result<String, VcsError> {
        self.files.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?.get(uri).cloned().ok_or_else(|| VcsError::Backbone(format!("missing backbone file {uri}")))
    }

    fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError> {
        self.files.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?.insert(uri.to_string(), payload.to_string());
        Ok(())
    }
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
fn local_storage_backbone_key(uri: &str) -> String {
    format!("semio:vcs:{uri}")
}

/// @emoji 💾️ Browser `localStorage` backbone port with in-memory fallback for native tests.
pub struct LocalStorageBackbonePort {
    fallback: MemoryBackbonePort,
}

impl LocalStorageBackbonePort {
    pub fn new() -> Self {
        Self { fallback: MemoryBackbonePort::new() }
    }
}

impl Default for LocalStorageBackbonePort {
    fn default() -> Self {
        Self::new()
    }
}

impl BackbonePort for LocalStorageBackbonePort {
    fn read(&self, uri: &str) -> Result<String, VcsError> {
        if let Some(port) = host_backbone_port() {
            if let Ok(value) = port.read(uri) {
                return Ok(value);
            }
        }
        #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(value)) = storage.get_item(&local_storage_backbone_key(uri)) {
                        return Ok(value);
                    }
                }
            }
        }
        self.fallback.read(uri)
    }

    fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError> {
        self.fallback.write(uri, payload)?;
        if let Some(port) = host_backbone_port() {
            let _ = port.write(uri, payload);
        }
        #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item(&local_storage_backbone_key(uri), payload);
                }
            }
        }
        Ok(())
    }
}

/// @emoji 🕸️ Injectable duplex transport across the wasm sandbox boundary (program ↔ host process).
/// `message`/the `poll` result are `BackboneMessage::encode_op`/`decode_op` (`crate::os_spr::OpBinary`) bytes.
pub trait BackboneChannelPort: Send + Sync {
    fn send(&self, uri: &str, message: &[u8]) -> Result<(), VcsError>;
    fn poll(&self, uri: &str) -> Result<Vec<Vec<u8>>, VcsError>;
}

static HOST_BACKBONE_CHANNEL: Mutex<Option<Arc<dyn BackboneChannelPort>>> = Mutex::new(None);

/// @emoji 🔌️ Injects the plugin host's duplex backbone channel for wasm-sandboxed document stores.
pub fn set_host_backbone_channel(channel: Arc<dyn BackboneChannelPort>) {
    if let Ok(mut guard) = HOST_BACKBONE_CHANNEL.lock() {
        *guard = Some(channel);
    }
}

fn host_backbone_channel() -> Option<Arc<dyn BackboneChannelPort>> {
    HOST_BACKBONE_CHANNEL.lock().ok().and_then(|guard| guard.clone())
}

/// @emoji 🧵️ Backbone that forwards messages across the wasm sandbox boundary to the host process,
/// which resolves the real `file://`/`folder://`/`remote://` backbone on its own (native) side.
pub struct PortBackbone {
    uri: String,
}

impl PortBackbone {
    pub fn new(uri: &str) -> Self {
        Self { uri: uri.to_string() }
    }
}

impl Backbone for PortBackbone {
    fn descriptor(&self) -> ArtifactBackboneRef {
        document_backbone_ref(&self.uri)
    }

    fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError> {
        let channel = host_backbone_channel().ok_or_else(|| VcsError::Backbone("backbone channel requires host port".into()))?;
        let bytes = message.encode_op().map_err(|error| VcsError::Serialize(error.to_string()))?;
        channel.send(&self.uri, &bytes)
    }

    fn receive(&mut self) -> Result<Vec<BackboneMessage>, VcsError> {
        let channel = host_backbone_channel().ok_or_else(|| VcsError::Backbone("backbone channel requires host port".into()))?;
        channel.poll(&self.uri)?.into_iter().map(|bytes| BackboneMessage::decode_op(&bytes).map_err(|e| VcsError::Deserialize(e.to_string()))).collect()
    }
}

/// @emoji 🔗️ Two crossed in-memory channel ends: whatever `a` sends, `b` receives, and vice versa.
pub struct MemoryBackbone {
    uri: String,
    inbox: Arc<Mutex<VecDeque<BackboneMessage>>>,
    outbox: Arc<Mutex<VecDeque<BackboneMessage>>>,
}

impl MemoryBackbone {
    pub fn pair(uri_a: &str, uri_b: &str) -> (Self, Self) {
        let a_to_b = Arc::new(Mutex::new(VecDeque::new()));
        let b_to_a = Arc::new(Mutex::new(VecDeque::new()));
        (Self { uri: uri_a.to_string(), inbox: b_to_a.clone(), outbox: a_to_b.clone() }, Self { uri: uri_b.to_string(), inbox: a_to_b, outbox: b_to_a })
    }
}

impl Backbone for MemoryBackbone {
    fn descriptor(&self) -> ArtifactBackboneRef {
        document_backbone_ref(&self.uri)
    }

    fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError> {
        self.outbox.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?.push_back(message);
        Ok(())
    }

    fn receive(&mut self) -> Result<Vec<BackboneMessage>, VcsError> {
        let mut inbox = self.inbox.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?;
        Ok(inbox.drain(..).collect())
    }
}

/// @emoji 🔗️ The store-side end of a pair of crossed in-memory queues. Implements the non-blocking
/// {@link Backbone} contract; the matching {@link ChannelBackboneRemote} is held by an external sync
/// actor (built in `framework/sync`, a later workstream) that pushes inbound messages and drains the
/// store's outbound ones. This crate only provides the queue plumbing — never the actor itself.
pub struct ChannelBackbone {
    uri: String,
    inbound: Arc<Mutex<VecDeque<BackboneMessage>>>,
    outbound: Arc<Mutex<VecDeque<BackboneMessage>>>,
}

/// @emoji 🎛️ The actor-side end paired with a {@link ChannelBackbone}: `push` delivers a message to
/// the store's inbound queue, `drain` collects everything the store has sent outbound. Not a
/// `Backbone` — this is the handle an IO-owning actor endpoint holds across the store boundary.
pub struct ChannelBackboneRemote {
    uri: String,
    inbound: Arc<Mutex<VecDeque<BackboneMessage>>>,
    outbound: Arc<Mutex<VecDeque<BackboneMessage>>>,
}

impl ChannelBackbone {
    /// @emoji 🔗️ Creates a crossed pair sharing a URI: the store attaches the `ChannelBackbone`; the
    /// actor keeps the `ChannelBackboneRemote`.
    pub fn pair(uri: &str) -> (ChannelBackbone, ChannelBackboneRemote) {
        let inbound = Arc::new(Mutex::new(VecDeque::new()));
        let outbound = Arc::new(Mutex::new(VecDeque::new()));
        (ChannelBackbone { uri: uri.to_string(), inbound: inbound.clone(), outbound: outbound.clone() }, ChannelBackboneRemote { uri: uri.to_string(), inbound, outbound })
    }
}

impl Backbone for ChannelBackbone {
    fn descriptor(&self) -> ArtifactBackboneRef {
        document_backbone_ref(&self.uri)
    }

    fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError> {
        self.outbound.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?.push_back(message);
        Ok(())
    }

    fn receive(&mut self) -> Result<Vec<BackboneMessage>, VcsError> {
        let mut inbound = self.inbound.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?;
        Ok(inbound.drain(..).collect())
    }
}

impl ChannelBackboneRemote {
    pub fn descriptor(&self) -> ArtifactBackboneRef {
        document_backbone_ref(&self.uri)
    }

    /// @emoji 📥️ Delivers a message to the store's inbound queue (actor→store).
    pub fn push(&self, message: BackboneMessage) -> Result<(), VcsError> {
        self.inbound.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?.push_back(message);
        Ok(())
    }

    /// @emoji 📤️ Collects everything the store has sent outbound (store→actor), draining the queue.
    pub fn drain(&self) -> Result<Vec<BackboneMessage>, VcsError> {
        let mut outbound = self.outbound.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?;
        Ok(outbound.drain(..).collect())
    }
}

/// @emoji 🔌️ Resolves a backbone URI to a concrete channel implementation. Only available inside the
/// wasm sandbox, where every scheme forwards to the host process over the injected
/// {@link BackboneChannelPort} (a pure in-memory queue). Native IO-performing backbones moved out of
/// this crate entirely — the `framework/sync` actor layer owns them.
#[cfg(target_arch = "wasm32")]
pub fn resolve_backbone(uri: &str) -> Result<Box<dyn Backbone>, VcsError> {
    Ok(Box::new(PortBackbone::new(uri)))
}
//#endregion 🔖️Backbone

//#region 🔖️BlobStore
//#region 🔖️BlobStore
/// @emoji 📦️ A content-addressed blob's identity + metadata. Never carries the bytes themselves —
/// callers that just put/read a blob already hold those; this is what gets embedded in a document
/// (e.g. a `MergeStrategyKind::ContentAddressedBlob` field) to reference it durably.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlobRef {
    pub hash: String,
    pub size: u64,
    pub media_type: String,
}

/// @emoji 🗄️ Content-addressed blob persistence backing `MergeStrategyKind::ContentAddressedBlob` /
/// `ArtifactKind::ContentAddressedBlob` (`framework/core/rs` 🔖️MergeStrategy region). `put` is idempotent —
/// it dedupes by the Blake3 hash of the bytes ({@link framework_hash::hash_bytes}), so writing
/// the same content twice never rewrites storage. Implementors decide the backing medium (sqlite here,
/// a semio_hub HTTP route in a later ticket, an IndexedDB cache in the browser).
pub trait BlobStore: Send + Sync {
    fn put(&self, bytes: &[u8], media_type: &str) -> Result<BlobRef, VcsError>;
    fn get(&self, hash: &str) -> Result<Option<Vec<u8>>, VcsError>;
    fn has(&self, hash: &str) -> Result<bool, VcsError>;
    fn delete(&self, hash: &str) -> Result<(), VcsError>;
}
//#endregion 🔖️BlobStore

//#region 🔖️Space
//#region SpaceMember
/// @emoji 🧑️‍🤝️‍🧑️ Object-safe façade over a `ArtifactStore<P, Mutation>` so a space host can hold a
/// heterogeneous registry of documents (`HashMap<String, Box<dyn SpaceMember>>`) without knowing
/// each member's concrete `P`/`Mutation`. Blanket-implemented below by delegating to `dispatch` — never
/// reimplement the underlying VCS mechanics here.
/// `: Send` (UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM C1, cross-boundary fix authorized by the ticket
/// orchestrator — see `📓️wave1-reports/c1-plugin-composition-report.md`'s `## Send bound`
/// section): `VcsArtifactApp`'s new child-store map (`🔌️plugin/🦀️component.rs`) holds `Box<dyn
/// SpaceMember>` as a field of a type that must stay `Send` (`PluginApp: Send`, frozen contract),
/// matching the pattern every OTHER erased trait-object seam in this file already follows
/// (`Backbone`/`BackbonePort`/`BackboneChannelPort` are all `Send + Sync`) — `SpaceMember` was the
/// one outlier. Only ONE implementor exists repo-wide (the blanket impl below, confirmed via
/// `grep -rn "impl.*SpaceMember for"`), so this is a verified, not speculative, widening.
pub trait SpaceMember: Send {
    fn document_id(&self) -> &str;
    /// @emoji 🩸️ Whether this member has edits applied since its last checkpoint (mirrors the
    /// `CommitCheckpoint` dispatch's own "nothing to commit" check via `uncommitted_edit_ids`).
    fn is_dirty(&self) -> bool;
    fn commit_checkpoint(&mut self, message: String, authors: Vec<Author>) -> Result<String, VcsError>;
    fn current_checkpoint_id(&self) -> Option<String>;
    fn current_alternative_id(&self) -> Option<String>;
    fn checkout(&mut self, checkpoint_id: &str, alternative_id: &str) -> Result<(), VcsError>;
    fn create_alternative(&mut self, name: String) -> Result<String, VcsError>;
    // 🎞️ CW3: `crate::os_spr::HybridLogicalTimestamp` (not `semio_framework`'s local one) — these
    // read `MutationMeta.timestamp`, which is the moved struct's field, typed against protocol_core.
    fn last_local_edit_timestamp(&self) -> Option<HybridLogicalTimestamp>;
    fn last_undone_local_edit_timestamp(&self) -> Option<HybridLogicalTimestamp>;
    fn undo(&mut self) -> Result<(), VcsError>;
    fn redo(&mut self) -> Result<(), VcsError>;
    /// @emoji 🪄️ Downcast escape hatch: a space host UI (or a test) needs the concrete
    /// `ArtifactStore<P, Mutation>` back out of a `Box<dyn SpaceMember>` — e.g. to `Apply` a
    /// technology-specific `Mutation`, which can't appear in this object-safe trait. `Self: 'static` is
    /// implied by every real `P`/`Mutation` pair, so this never fails for a genuine member.
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    // 🎯️ B2 `CompositionCoordinator` seam (`UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`, `🔖️CompositionCoordinator`
    // region below `🔖️Space`): eight object-safe methods, in two groups. `validate_wire`/
    // `dispatch_wire`/`tail_group_id` are the three the task brief named explicitly. The other five
    // (`tail_edit_id`, `redo_tail`, `stamp_tail_group_id`, `set_owner`) are necessary, deliberate
    // additions this wave makes beyond that literal list — see `📓️wave1-reports/
    // b2-store-composition-report.md`'s "Design decisions" for why each is unavoidable given the
    // object-safety constraint: `GroupReceipt`/`GroupUndoReport` need real edit ids (not just group
    // membership), `dispatch_group`'s phase 2 needs a way to stamp a shared `group_id` onto a member
    // AFTER an ordinary `Apply` already hard-codes `group_id: None`, and genesis needs a way to set
    // `ArtifactEnvelope.owner` on a freshly-created child through the same type-erased interface.
    /// @emoji 🧪️ Decodes `ops` as a sequence of individually-`OpBinary`-encoded `Mutation`s (the
    /// SAME wire shape `ArtifactCommand::Apply.mutations` bundles — see `dispatch_wire`'s doc
    /// comment) and runs `crate::os_spr::Mutation::validate` on each against a snapshot threaded
    /// forward through the whole slice, so op `i` validates against the state ops `0..i` would
    /// produce, never a stale base. Never applies anything. `Result<_, String>` (not `VcsError`)
    /// because `Mutation::validate` itself returns `Result<(), String>` — a direct pass-through,
    /// not a new error taxonomy.
    fn validate_wire(&self, ops: &[Vec<u8>]) -> Result<(), String>;
    /// @emoji 📡️ Decodes `cmd_bytes` as one binary `ArtifactCommand<Mutation>` and dispatches it via
    /// `dispatch_binary`. `CompositionCoordinator` builds `cmd_bytes` from a member's
    /// `ChildDispatch.ops`/`parent_ops` by replicating `write_command_ops`'s byte layout directly
    /// (length-prefixed already-encoded op bytes) WITHOUT ever decoding an individual op — the
    /// reason this whole family takes/returns bytes instead of a typed `ArtifactCommand<Mutation>`
    /// is that `SpaceMember` itself must stay object-safe (no generic method can appear on a trait
    /// object), so the coordinator stays fully agnostic of every member's concrete `Mutation` type.
    fn dispatch_wire(&mut self, cmd_bytes: &[u8]) -> Result<CommandReceipt, VcsError>;
    /// @emoji 🏷️ The `MutationMeta.group_id` recorded on this member's TAIL applied edit's last
    /// operation, if any — lets `CompositionCoordinator::undo_group` recognize "does this member's
    /// most recent edit belong to composite gesture X" without downcasting to a concrete
    /// `ArtifactStore<P, Mutation>`.
    fn tail_group_id(&self) -> Option<String>;
    /// @emoji 🆔️ The id of this member's TAIL applied edit, if any — `tail_group_id`'s companion
    /// getter, so `GroupReceipt`/`GroupUndoReport` can report WHICH edit a group touched/undid, not
    /// only that group membership matched.
    fn tail_edit_id(&self) -> Option<String>;
    /// @emoji ↩️🏷️ `(tail_group_id, tail_edit_id)`'s REDO-direction mirror: the `(edit_id,
    /// group_id)` of whatever edit sits at the top of this member's redo stack (the one a following
    /// `redo()` would reapply), used by `CompositionCoordinator::redo_group` the way `tail_group_id`/
    /// `tail_edit_id` are used by `undo_group`.
    fn redo_tail(&self) -> Option<(String, Option<String>)>;
    /// @emoji 🖋️ Stamps `group_id` onto every `MutationMeta` entry of this member's TAIL applied
    /// edit — the mechanism `CompositionCoordinator::dispatch_group`'s phase 2 uses to give every
    /// member of one composite gesture the SAME `MutationMeta.group_id` after dispatching each
    /// member's own `Apply` independently (the ordinary `Apply` path has no way to accept an
    /// externally-supplied group id — see `ArtifactStore::replay_mutations`, which always stamps
    /// `group_id: None`). Errors with `VcsError::UnknownEdit` if this member has no applied edits at
    /// all — never true on the path `dispatch_group` actually calls it from, since it always calls
    /// this immediately after a successful `dispatch_wire`.
    fn stamp_tail_group_id(&mut self, group_id: &str) -> Result<(), VcsError>;
    /// @emoji 🏠️ Sets (or clears) this member's own envelope `owner` stamp — the mechanism
    /// `CompositionCoordinator::dispatch_group`'s phase 2 uses to record a freshly-`ChildGenesis`-
    /// created child's `OwnerRef` directly on the child's own envelope (see
    /// `ArtifactEnvelope.owner`'s doc comment for why ownership must be queryable from the child
    /// side, not only from the parent's `ArtifactChild` handle). Not part of the ordinary
    /// VCS/dispatch surface — no ordinary `Apply` mutation can reach envelope metadata — so it needs
    /// its own object-safe setter.
    fn set_owner(&mut self, owner: Option<OwnerRef>);
}

impl<P, Mutation> SpaceMember for ArtifactStore<P, Mutation>
where
    P: Clone + Serialize + DeserializeOwned + ArtifactPack + Send + 'static,
    Mutation: Clone + Serialize + DeserializeOwned + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    fn document_id(&self) -> &str {
        self.envelope().id.as_str()
    }

    fn is_dirty(&self) -> bool {
        !uncommitted_edit_ids(&self.envelope, self.applied_edit_ids()).is_empty()
    }

    fn commit_checkpoint(&mut self, message: String, authors: Vec<Author>) -> Result<String, VcsError> {
        self.dispatch(ArtifactCommand::CommitCheckpoint { message: Some(message), authors })?;
        // `self.current_checkpoint_id()` resolves to the inherent method (`Option<&str>`), not this
        // trait method — Rust prefers inherent methods over trait methods of the same name.
        self.current_checkpoint_id().map(|id| id.to_string()).ok_or(VcsError::NoCheckpoint)
    }

    fn current_checkpoint_id(&self) -> Option<String> {
        self.current_checkpoint_id().map(|id| id.to_string())
    }

    fn current_alternative_id(&self) -> Option<String> {
        self.envelope().active_alternative_id.clone()
    }

    fn checkout(&mut self, checkpoint_id: &str, alternative_id: &str) -> Result<(), VcsError> {
        if !alternative_id.is_empty() {
            let is_alternative_tip = self.envelope().vcs.alternatives.iter().find(|alternative| alternative.id == alternative_id).map(|alternative| alternative.checkpoint_ids.last().map(String::as_str) == Some(checkpoint_id)).unwrap_or(false);
            if is_alternative_tip {
                return self.dispatch(ArtifactCommand::SwitchAlternative { alternative_id: alternative_id.to_string() }).map(|_| ());
            }
        }
        self.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: checkpoint_id.to_string() }).map(|_| ())
    }

    fn create_alternative(&mut self, name: String) -> Result<String, VcsError> {
        self.dispatch(ArtifactCommand::CreateAlternative { name })?;
        self.envelope().active_alternative_id.clone().ok_or(VcsError::NoCheckpoint)
    }

    fn last_local_edit_timestamp(&self) -> Option<HybridLogicalTimestamp> {
        self.applied_edit_ids().iter().rev().find_map(|edit_id| {
            if !self.edit_is_local(edit_id) {
                return None;
            }
            self.envelope().vcs.edits.iter().find(|edit| edit.id == *edit_id).and_then(|edit| edit.mutation_meta.last()).map(|meta| meta.timestamp)
        })
    }

    fn last_undone_local_edit_timestamp(&self) -> Option<HybridLogicalTimestamp> {
        self.redo_edit_ids().iter().rev().find_map(|edit_id| {
            if !self.edit_is_local(edit_id) {
                return None;
            }
            self.envelope().vcs.edits.iter().find(|edit| edit.id == *edit_id).and_then(|edit| edit.mutation_meta.last()).map(|meta| meta.timestamp)
        })
    }

    fn undo(&mut self) -> Result<(), VcsError> {
        self.dispatch(ArtifactCommand::Undo).map(|_| ())
    }

    fn redo(&mut self) -> Result<(), VcsError> {
        self.dispatch(ArtifactCommand::Redo).map(|_| ())
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn validate_wire(&self, ops: &[Vec<u8>]) -> Result<(), String> {
        let mut running = self.snapshot().map_err(|error| error.to_string())?;
        for op in ops {
            let mutation = <Mutation as OpBinary>::decode_op(op).map_err(|error| error.to_string())?;
            mutation.validate(&running)?;
            running = apply_mutation(&running, &mutation);
        }
        Ok(())
    }

    fn dispatch_wire(&mut self, cmd_bytes: &[u8]) -> Result<CommandReceipt, VcsError> {
        self.dispatch_binary(cmd_bytes)
    }

    fn tail_group_id(&self) -> Option<String> {
        let edit_id = self.applied_edit_ids().last()?;
        self.envelope().vcs.edits.iter().find(|edit| edit.id == *edit_id)?.mutation_meta.last()?.group_id.clone()
    }

    fn tail_edit_id(&self) -> Option<String> {
        self.applied_edit_ids().last().cloned()
    }

    fn redo_tail(&self) -> Option<(String, Option<String>)> {
        let edit_id = self.redo_edit_ids().last()?.clone();
        let group_id = self.envelope().vcs.edits.iter().find(|edit| edit.id == edit_id).and_then(|edit| edit.mutation_meta.last()).and_then(|meta| meta.group_id.clone());
        Some((edit_id, group_id))
    }

    fn stamp_tail_group_id(&mut self, group_id: &str) -> Result<(), VcsError> {
        let edit_id = self.applied_edit_ids().last().cloned().ok_or(VcsError::NothingToUndo)?;
        let edit = self.envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
        for meta in edit.mutation_meta.iter_mut() {
            meta.group_id = Some(group_id.to_string());
        }
        Ok(())
    }

    fn set_owner(&mut self, owner: Option<OwnerRef>) {
        self.envelope.owner = owner;
    }
}
//#endregion SpaceMember

//#region SpaceHistoryDocument
/// @emoji 📌️ One member document's position at the moment a `SpaceCheckpoint` was recorded.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceMemberPin {
    pub document_id: String,
    pub checkpoint_id: String,
    /// @emoji 🌿️ Empty string when the member had no active alternative (its own trunk) at pin time.
    #[serde(default)]
    pub alternative_id: String,
}

/// @emoji 🗄️ A space-wide checkpoint: one pin per registered member, so checking it out (or an
/// alternative built on top of it) fans out deterministically to every member's own VCS.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceCheckpoint {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub message: String,
    pub authors: Vec<Author>,
    pub timestamp: HybridLogicalTimestamp,
    pub members: Vec<SpaceMemberPin>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceAlternative {
    pub id: String,
    pub name: String,
    pub checkpoint_ids: Vec<String>,
}

/// @emoji 🏷️ Schema id of the space-wide history meta-document — `"os.space.history"`, under the
/// `os.` schema lattice (a generic os-shell abstraction, not the `s.` product lattice), separate
/// from `space::S_SPACE_SCHEMA`/`space::S_COLLECTION_SCHEMA` (this crate sits below `space` in the
/// dependency graph, so it declares its own constant rather than depending on that crate's). The
/// `.spr` extension (`SpaceHistorySnapshot::EXTENSION`, `"space-history"`) is unchanged.
pub const S_SPACE_HISTORY_SCHEMA: &str = "os.space.history";

/// @emoji 🗄️ Snapshot of the `S_SPACE_HISTORY_SCHEMA` (`"os.space.history"`) meta-document: itself
/// an ordinary `ArtifactVcs` document kind (dogfooded — no bespoke transport), holding the
/// space-level checkpoint/alternative graph that `SpaceHost` composes on top of every registered
/// member's own history.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceHistorySnapshot {
    pub checkpoints: Vec<SpaceCheckpoint>,
    pub alternatives: Vec<SpaceAlternative>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_alternative_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum SpaceHistoryMutation {
    CommitSpaceCheckpoint {
        checkpoint: SpaceCheckpoint,
    },
    CreateSpaceAlternative {
        alternative: SpaceAlternative,
    },
    SwitchSpaceAlternative {
        alternative_id: String,
    },
    /// @emoji ↩️ Mechanical inverse of `CommitSpaceCheckpoint`; never dispatched directly by
    /// `SpaceHost` (space undo is derived and member-local, see `SpaceHost::undo`), only
    /// produced by `inverse` for VCS round-trip correctness.
    RemoveSpaceCheckpoint {
        checkpoint_id: String,
    },
    /// @emoji ↩️ Mechanical inverse of `CreateSpaceAlternative`; see `RemoveSpaceCheckpoint`.
    RemoveSpaceAlternative {
        alternative_id: String,
    },
    /// @emoji ↩️ Mechanical inverse of `SwitchSpaceAlternative`; see `RemoveSpaceCheckpoint`.
    SetActiveSpaceAlternative {
        alternative_id: Option<String>,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceHistoryDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_checkpoint: Option<SpaceCheckpoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_checkpoint_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_alternative: Option<SpaceAlternative>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_alternative_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_active_alternative_id: Option<Option<String>>,
}

impl MutationDiff<SpaceHistorySnapshot> for SpaceHistoryDiff {
    fn apply(&self, snapshot: &SpaceHistorySnapshot) -> SpaceHistorySnapshot {
        let mut next = snapshot.clone();
        if let Some(checkpoint) = &self.add_checkpoint {
            next.checkpoints.push(checkpoint.clone());
        }
        if let Some(checkpoint_id) = &self.remove_checkpoint_id {
            next.checkpoints.retain(|checkpoint| checkpoint.id != *checkpoint_id);
        }
        if let Some(alternative) = &self.add_alternative {
            next.alternatives.push(alternative.clone());
        }
        if let Some(alternative_id) = &self.remove_alternative_id {
            next.alternatives.retain(|alternative| alternative.id != *alternative_id);
        }
        if let Some(active) = &self.set_active_alternative_id {
            next.active_alternative_id = active.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.add_checkpoint.is_some() {
            self.add_checkpoint = other.add_checkpoint;
        }
        if other.remove_checkpoint_id.is_some() {
            self.remove_checkpoint_id = other.remove_checkpoint_id;
        }
        if other.add_alternative.is_some() {
            self.add_alternative = other.add_alternative;
        }
        if other.remove_alternative_id.is_some() {
            self.remove_alternative_id = other.remove_alternative_id;
        }
        if other.set_active_alternative_id.is_some() {
            self.set_active_alternative_id = other.set_active_alternative_id;
        }
    }
}

impl Mutation<SpaceHistorySnapshot> for SpaceHistoryMutation {
    type Diff = SpaceHistoryDiff;

    fn diff(&self, _snapshot: &SpaceHistorySnapshot) -> SpaceHistoryDiff {
        match self {
            SpaceHistoryMutation::CommitSpaceCheckpoint { checkpoint } => SpaceHistoryDiff { add_checkpoint: Some(checkpoint.clone()), ..Default::default() },
            SpaceHistoryMutation::CreateSpaceAlternative { alternative } => SpaceHistoryDiff { add_alternative: Some(alternative.clone()), set_active_alternative_id: Some(Some(alternative.id.clone())), ..Default::default() },
            SpaceHistoryMutation::SwitchSpaceAlternative { alternative_id } => SpaceHistoryDiff { set_active_alternative_id: Some(Some(alternative_id.clone())), ..Default::default() },
            SpaceHistoryMutation::RemoveSpaceCheckpoint { checkpoint_id } => SpaceHistoryDiff { remove_checkpoint_id: Some(checkpoint_id.clone()), ..Default::default() },
            SpaceHistoryMutation::RemoveSpaceAlternative { alternative_id } => SpaceHistoryDiff { remove_alternative_id: Some(alternative_id.clone()), ..Default::default() },
            SpaceHistoryMutation::SetActiveSpaceAlternative { alternative_id } => SpaceHistoryDiff { set_active_alternative_id: Some(alternative_id.clone()), ..Default::default() },
        }
    }

    fn inverse(&self, snapshot: &SpaceHistorySnapshot) -> Vec<Self> {
        match self {
            SpaceHistoryMutation::CommitSpaceCheckpoint { checkpoint } => {
                vec![SpaceHistoryMutation::RemoveSpaceCheckpoint { checkpoint_id: checkpoint.id.clone() }]
            }
            SpaceHistoryMutation::CreateSpaceAlternative { alternative } => {
                vec![SpaceHistoryMutation::SetActiveSpaceAlternative { alternative_id: snapshot.active_alternative_id.clone() }, SpaceHistoryMutation::RemoveSpaceAlternative { alternative_id: alternative.id.clone() }]
            }
            SpaceHistoryMutation::SwitchSpaceAlternative { .. } => vec![SpaceHistoryMutation::SetActiveSpaceAlternative { alternative_id: snapshot.active_alternative_id.clone() }],
            SpaceHistoryMutation::RemoveSpaceCheckpoint { checkpoint_id } => {
                snapshot.checkpoints.iter().find(|checkpoint| checkpoint.id == *checkpoint_id).map(|checkpoint| vec![SpaceHistoryMutation::CommitSpaceCheckpoint { checkpoint: checkpoint.clone() }]).unwrap_or_default()
            }
            SpaceHistoryMutation::RemoveSpaceAlternative { alternative_id } => {
                snapshot.alternatives.iter().find(|alternative| alternative.id == *alternative_id).map(|alternative| vec![SpaceHistoryMutation::CreateSpaceAlternative { alternative: alternative.clone() }]).unwrap_or_default()
            }
            SpaceHistoryMutation::SetActiveSpaceAlternative { .. } => vec![SpaceHistoryMutation::SetActiveSpaceAlternative { alternative_id: snapshot.active_alternative_id.clone() }],
        }
    }
}

// 🎯️ B2: `ArtifactStore`'s shared impl block now requires `P: ArtifactPack` + `Mutation: OpText
// + OpBinary` for every instantiation (the pack+spr binary snapshot pipeline, needed by
// `SpaceHost::attach_backbone`'s real backbone-attach path — this dogfooded meta-document DOES
// cross a real wire once a backbone is attached, see `studio_vcs_host_meta_document_is_backbone_
// attachable_and_detachable`). `SpaceCheckpoint`/`SpaceAlternative` embed foreign types
// (`crate::os_vcs::Author`, `crate::os_spr::HybridLogicalTimestamp`) that cannot derive `crate::os_dsl::DslRecord`
// (orphan rule; `dsl`'s own dependency graph would cycle back through `protocol`), so a full
// `#[derive(DslArtifact)]`/`#[derive(DslOps)]` grammar is out of reach here without a larger
// dedicated field-mirroring effort (tracked as a B9 follow-up, same as the `serde_json::Value`-
// projected apps' analogous DSL-quality gap — see `impl ArtifactPack for serde_json::Value`
// above). BINARY face: real pack `Shape::Value` TLV bytes via `pack_rt::encode_json_value`
// (compliant structured binary per this wave's scope ruling — NOT raw JSON text bytes, unlike
// the deleted `serde_json::to_vec` hatch this replaces). TEXT face: JSON text, the same
// documented, scoped exception the `Value`-projected apps already have.
// 🎯️ `renormalize_whole_number_floats` moved into `pack_rt` (this file's :209) — general
// property of `pack_rt::decode_json_value`'s output, not specific to this type; `semio_compose_rs`'s
// `ComposeWireMutation` needs the exact same fix and calls the same `pack_rt::` function.
use pack_rt::renormalize_whole_number_floats;

impl OpText for SpaceHistoryMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).expect("SpaceHistoryMutation serializes infallibly")
    }
    fn parse_op(line: &str) -> Result<Self, TextError> {
        serde_json::from_str(line).map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))
    }
}
impl OpBinary for SpaceHistoryMutation {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        let value = to_dsl_value(self).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "space history op", offset: 0, detail: error })?;
        Ok(pack_rt::encode_pack_value(&value))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        let value = pack_rt::decode_pack_value(bytes).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "space history op", offset: 0, detail: error.to_string() })?;
        from_dsl_value(renormalize_whole_number_floats(value)).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "space history op", offset: 0, detail: error })
    }
}
impl ArtifactDsl for SpaceHistorySnapshot {
    const EXTENSION: &'static str = "space-history";
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        serde_json::from_str(text).map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        serde_json::to_string(self).expect("SpaceHistorySnapshot serializes infallibly")
    }
}
impl ArtifactPack for SpaceHistorySnapshot {
    fn encode_pack_with(&self, _options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let value = to_dsl_value(self).map_err(PackError::Schema)?;
        Ok(pack_rt::encode_pack_value(&value))
    }
    fn decode_pack_with(bytes: &[u8], _options: &PackDecodeOptions) -> Result<Self, PackError> {
        let value = pack_rt::decode_pack_value(bytes)?;
        from_dsl_value(renormalize_whole_number_floats(value)).map_err(PackError::Schema)
    }
}
//#endregion SpaceHistoryDocument

//#region SpaceHost
/// @emoji 🏛️ Composes many `SpaceMember` documents under one space-wide checkpoint/alternative
/// timeline, itself stored in a dogfooded `S_SPACE_HISTORY_SCHEMA` (`"os.space.history"`)
/// meta-document. App-agnostic: this crate has no notion of what a member document *is*, only that
/// it satisfies `SpaceMember`.
pub struct SpaceHost {
    meta: ArtifactStore<SpaceHistorySnapshot, SpaceHistoryMutation>,
    members: HashMap<String, Box<dyn SpaceMember>>,
}





impl SpaceHost {
    pub fn new(meta_envelope: ArtifactEnvelope<SpaceHistorySnapshot, SpaceHistoryMutation>) -> Self {
        Self { meta: ArtifactStore::new(meta_envelope), members: HashMap::new() }
    }

    pub fn register_member(&mut self, member: Box<dyn SpaceMember>) {
        self.members.insert(member.document_id().to_string(), member);
    }

    /// @emoji 📚️ Batch counterpart to `register_member`: registers a space's manifest document, its
    /// collection documents, and any currently-open artifact documents together in one call, so the
    /// very next `commit_space_checkpoint` pins all of them atomically in the SAME space-wide
    /// checkpoint (see `🪐️space`'s `SpaceSnapshot`/`CollectionSnapshot`/document-artifact
    /// stores, W4's storage wiring — this crate stays app-agnostic and never names those types
    /// directly, only their common `SpaceMember` façade). Purely additive sugar over calling
    /// `register_member` three times in this order; no new mechanism.
    pub fn register_space_documents(&mut self, manifest: Box<dyn SpaceMember>, collections: Vec<Box<dyn SpaceMember>>, artifacts: Vec<Box<dyn SpaceMember>>) {
        self.register_member(manifest);
        for collection in collections {
            self.register_member(collection);
        }
        for artifact in artifacts {
            self.register_member(artifact);
        }
    }

    pub fn unregister_member(&mut self, document_id: &str) -> Option<Box<dyn SpaceMember>> {
        self.members.remove(document_id)
    }

    pub fn member(&self, document_id: &str) -> Option<&dyn SpaceMember> {
        self.members.get(document_id).map(|member| member.as_ref())
    }

    pub fn member_mut<'a>(&'a mut self, document_id: &str) -> Option<&'a mut (dyn SpaceMember + 'a)> {
        match self.members.get_mut(document_id) {
            Some(member) => Some(member.as_mut()),
            None => None,
        }
    }

    pub fn meta_snapshot(&self) -> Result<SpaceHistorySnapshot, VcsError> {
        self.meta.snapshot()
    }

    /// @emoji 🔗️ Attaches a backbone to the space-wide meta-document, same runtime-attach/detach
    /// contract as any other `ArtifactStore` — default is unattached, this is always an
    /// explicit call.
    pub fn attach_backbone(&mut self, backbone: Box<dyn Backbone>) -> Result<(), VcsError> {
        self.meta.attach_backbone(backbone)
    }

    /// @emoji ✂️ Detaches the meta-document's backbone; the space history stays in memory.
    pub fn detach_backbone(&mut self) -> Option<Box<dyn Backbone>> {
        self.meta.detach_backbone()
    }

    pub fn backbone_ref(&self) -> Option<&ArtifactBackboneRef> {
        self.meta.backbone_ref()
    }

    /// @emoji 📡️ Drains inbound backbone messages into the meta-document's edit timeline.
    pub fn tick(&mut self) -> Result<bool, VcsError> {
        self.meta.tick()
    }

    /// @emoji 💾️ Commits every dirty member (leaving clean members' existing checkpoints untouched),
    /// pins each member's resulting `(checkpoint, alternative)`, and records one `SpaceCheckpoint`
    /// on the meta-document — applied *and* committed there too, so the space history itself is
    /// durable the moment this returns.
    pub fn commit_space_checkpoint(&mut self, message: String, authors: Vec<Author>) -> Result<String, VcsError> {
        let mut document_ids: Vec<String> = self.members.keys().cloned().collect();
        document_ids.sort();
        let mut pins = Vec::with_capacity(document_ids.len());
        for document_id in &document_ids {
            let member = self.members.get_mut(document_id).expect("just collected from members");
            if member.is_dirty() {
                member.commit_checkpoint(message.clone(), authors.clone())?;
            }
            let checkpoint_id = member.current_checkpoint_id().ok_or(VcsError::NoCheckpoint)?;
            pins.push(SpaceMemberPin { document_id: document_id.clone(), checkpoint_id, alternative_id: member.current_alternative_id().unwrap_or_default() });
        }
        let pins_fingerprint = serde_json::to_vec(&pins).unwrap_or_default();
        let mut space_checkpoint_payload = message.as_bytes().to_vec();
        space_checkpoint_payload.push(0);
        space_checkpoint_payload.extend_from_slice(&pins_fingerprint);
        let checkpoint_id = content_addressed_entity_id("space-checkpoint", &space_checkpoint_payload);
        let parent_id = self.meta.snapshot()?.checkpoints.last().map(|checkpoint| checkpoint.id.clone());
        let checkpoint = SpaceCheckpoint { id: checkpoint_id.clone(), parent_id, message: message.clone(), authors, timestamp: HybridLogicalTimestamp::new(0, now_ms()), members: pins };
        // 🎯️ W6: the `Apply` below uses `dispatch_inner` (not `dispatch`), skipping its automatic
        // per-dispatch `flush_outbound` — the very next `CommitCheckpoint` dispatch flushes a full
        // snapshot that already includes this `Apply`'s edit, so a separate incremental flush here
        // would resend the same change twice. Before W5/W6's per-op wire envelopes this was
        // harmless (both flushes tagged the change with the same `edit.id`, so a receiver's
        // id-based dedup silently absorbed the duplicate); now that `Operations` messages carry
        // per-OP ids (distinct from the edit's own id — see `flush_outbound`), the two flushes are
        // no longer accidentally deduplicable, so avoiding the redundant one is the real fix.
        self.meta.dispatch_inner(ArtifactCommand::Apply { mutations: vec![SpaceHistoryMutation::CommitSpaceCheckpoint { checkpoint }], description: Some(message) })?;
        self.meta.dispatch(ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() })?;
        Ok(checkpoint_id)
    }

    /// @emoji 🌿️ Records a `SpaceAlternative` pinned at the current space checkpoint tip (or none,
    /// if nothing has been committed yet), so it can later be switched back into.
    pub fn create_space_alternative(&mut self, name: String) -> Result<String, VcsError> {
        let checkpoint_ids: Vec<String> = self.meta.snapshot()?.checkpoints.last().map(|checkpoint| checkpoint.id.clone()).into_iter().collect();
        let mut space_alternative_payload = name.as_bytes().to_vec();
        space_alternative_payload.push(0);
        space_alternative_payload.extend_from_slice(checkpoint_ids.join("\0").as_bytes());
        let alternative_id = content_addressed_entity_id("space-alternative", &space_alternative_payload);
        let alternative = SpaceAlternative { id: alternative_id.clone(), name, checkpoint_ids };
        self.meta.dispatch(ArtifactCommand::Apply { mutations: vec![SpaceHistoryMutation::CreateSpaceAlternative { alternative }], description: None })?;
        Ok(alternative_id)
    }

    /// @emoji 🔀️ Fans out to every member pinned by `checkpoint_id`'s `SpaceCheckpoint`, restoring
    /// each to its exact recorded `(checkpoint, alternative)`.
    pub fn checkout_space_checkpoint(&mut self, checkpoint_id: &str) -> Result<(), VcsError> {
        let snapshot = self.meta.snapshot()?;
        let checkpoint = snapshot.checkpoints.iter().find(|checkpoint| checkpoint.id == checkpoint_id).ok_or(VcsError::NoCheckpoint)?;
        for pin in &checkpoint.members {
            if let Some(member) = self.members.get_mut(&pin.document_id) {
                member.checkout(&pin.checkpoint_id, &pin.alternative_id)?;
            }
        }
        Ok(())
    }

    /// @emoji 🔀️ Switches the studio's active alternative and fans out to its tip checkpoint's pins.
    pub fn switch_space_alternative(&mut self, alternative_id: &str) -> Result<(), VcsError> {
        let snapshot = self.meta.snapshot()?;
        let alternative = snapshot.alternatives.iter().find(|alternative| alternative.id == alternative_id).ok_or_else(|| VcsError::UnknownAlternative(alternative_id.to_string()))?;
        let checkpoint_id = alternative.checkpoint_ids.last().cloned().ok_or(VcsError::NoCheckpoint)?;
        self.meta.dispatch(ArtifactCommand::Apply { mutations: vec![SpaceHistoryMutation::SwitchSpaceAlternative { alternative_id: alternative_id.to_string() }], description: None })?;
        self.checkout_space_checkpoint(&checkpoint_id)
    }

    /// @emoji ↩️ Derived, local-only undo: targets whichever registered member has the most recent
    /// `last_local_edit_timestamp` (by {@link HybridLogicalTimestamp::cmp_key}) and undoes just that
    /// member. Never dispatched against the meta-document — space-level undo has no `SpaceHistoryMutation`
    /// of its own, it is purely a cross-member ordering policy.
    pub fn undo(&mut self) -> Result<(), VcsError> {
        let target = self.members.iter().filter_map(|(document_id, member)| member.last_local_edit_timestamp().map(|timestamp| (timestamp.cmp_key(), document_id.clone()))).max_by_key(|(cmp_key, _)| *cmp_key).map(|(_, document_id)| document_id);
        let document_id = target.ok_or(VcsError::NothingToUndo)?;
        self.members.get_mut(&document_id).ok_or(VcsError::NothingToUndo)?.undo()
    }

    /// @emoji ↪️ Derived, local-only redo: mirrors `undo`, targeting the member with the most
    /// recent `last_undone_local_edit_timestamp`.
    pub fn redo(&mut self) -> Result<(), VcsError> {
        let target =
            self.members.iter().filter_map(|(document_id, member)| member.last_undone_local_edit_timestamp().map(|timestamp| (timestamp.cmp_key(), document_id.clone()))).max_by_key(|(cmp_key, _)| *cmp_key).map(|(_, document_id)| document_id);
        let document_id = target.ok_or(VcsError::NothingToRedo)?;
        self.members.get_mut(&document_id).ok_or(VcsError::NothingToRedo)?.redo()
    }
}
//#endregion SpaceHost
//#endregion 🔖️Space

//#region 🔖️CompositionCoordinator
// 🧩️ Atomic composite dispatch across a parent `SpaceMember` and N child `SpaceMember`s — the
// mechanism that makes ONE user gesture spanning a parent and several `🔖️Composition` children
// into ONE undo step, by orchestrating multi-store dispatch through the object-safe
// `SpaceMember::validate_wire`/`dispatch_wire`/`tail_group_id`/etc. seam (extended just above,
// `🔖️Space`'s `SpaceMember` region) rather than needing to know any member's concrete `P`/
// `Mutation`. Two-phase: `dispatch_group`'s phase 1 validates every op on every member against its
// CURRENT snapshot with zero side effects (any failure ⇒ nothing dispatched anywhere); phase 2
// applies in the fixed order child geneses → child edits → parent ops, stamping every resulting
// edit's `MutationMeta.group_id` with the same minted `invocation_id` — the shared stamp a later
// `undo_group`/`redo_group` call recognizes. `CompositionGraph` tracks the ownership forest/link
// DAG this all leans on for cycle/ownership validation.

/// @emoji 📮️ One child's share of a composite dispatch: which child, its ops (each individually
/// `crate::os_spr::OpBinary`-encoded — the SAME per-op wire shape `ArtifactCommand::Apply.mutations`
/// bundles, see `SpaceMember::dispatch_wire`'s doc comment), the schema those ops decode against,
/// and human-readable labels (one per op, forward-compat metadata for a future audit/diagnostic
/// surface — `dispatch_group` does not interpret them itself, see this ticket's
/// `📓️wave1-reports/b2-store-composition-report.md` for the scoping note).
#[derive(Clone, Debug, PartialEq)]
pub struct ChildDispatch {
    pub child: crate::os_io::ArtifactRef,
    pub ops: Vec<Vec<u8>>,
    pub op_schema: SchemaId,
    pub labels: Vec<String>,
}

/// @emoji 🌱️ One new child to create in this same composite gesture: which parent-relative `slot`,
/// what dialect it materializes as, and its baked initial pack bytes (fed to the registered
/// `ChildStoreFactory::create`). No `ops` — a freshly-created child has nothing to replay yet, only
/// an initial snapshot.
#[derive(Clone, Debug, PartialEq)]
pub struct ChildGenesis {
    pub slot: String,
    pub dialect: crate::os_io::ArtifactDialect,
    pub initial_pack: Vec<u8>,
}

/// @emoji 🧾️ Receipt from a successful `CompositionCoordinator::dispatch_group`: the minted shared
/// `MutationMeta.group_id` every touched member's tail edit now carries, one `(member, edit_id)`
/// pair per member that actually got a new edit (parent included when `parent_ops` was non-empty;
/// a child with empty `ops` contributes none), and every `ChildGenesis`-created member's live
/// `Box<dyn SpaceMember>` — genesis has no pre-existing caller-held reference to hand back through
/// (unlike `children`, which the caller already owns), so this is the only way a freshly-created
/// child ever reaches the caller for registration (e.g. into a `SpaceHost`). No `Clone`/`Debug`/
/// `PartialEq`: `Box<dyn SpaceMember>` supports none of them.
pub struct GroupReceipt {
    pub invocation_id: String,
    pub member_edits: Vec<(crate::os_io::ArtifactRef, String)>,
    pub created_children: Vec<(crate::os_io::ArtifactRef, Box<dyn SpaceMember>)>,
}

/// @emoji 🎛️ Cross-member metadata for one `dispatch_group` call. `description` becomes every
/// dispatched member's own `ArtifactCommand::Apply.description`. `actor`/`coalesce_key` are
/// accepted or forward-compat/audit purposes but NOT yet wired into dispatch — object-safe
/// `SpaceMember` has no `set_local_actor_id`/`AmendLast` seam today (only `ArtifactStore`'s own
/// inherent API does), so honoring them is deferred to whichever later wave extends that surface;
/// see `📓️wave1-reports/b2-store-composition-report.md`'s scoping note.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GroupMeta {
    pub actor: Option<String>,
    pub description: Option<String>,
    pub coalesce_key: Option<String>,
}

/// @emoji 🧾️ Best-effort group undo/redo report: `undone` is every member that WAS rolled
/// back/reapplied (with the edit id touched), `skipped` is every member whose tail didn't belong to
/// the requested group, or whose own `undo()`/`redo()` itself failed — recorded as a diagnostic,
/// never aborting the rest of the group. Generalizes the existing benign `NothingToUndo`/
/// `ForeignEdit` collapse (`🔌️plugin/🦀️component.rs`'s `UndoWithPolicy` dispatch) to the
/// multi-member case: one collaborator's foreign/failed child edit must never permanently freeze a
/// parent's undo stack.
#[derive(Debug, PartialEq)]
pub struct GroupUndoReport {
    pub undone: Vec<(crate::os_io::ArtifactRef, String)>,
    pub skipped: Vec<(crate::os_io::ArtifactRef, VcsError)>,
}

/// @emoji 🕸️ Ownership forest (`Owns`, each child has AT MOST one owner, no cycles) + reference DAG
/// (`Links`, acyclic) over artifact ids — the structure `CompositionCoordinator::dispatch_group`'s
/// phase 1 consults for ownership/cycle validation, incrementally maintained (`sync_member`) rather
/// than ever rebuilt from scratch on the hot path. A `CompositionCoordinator` owns one; a host that
/// needs the graph without dispatching anything (e.g. a UI computing "would this drag-and-drop
/// create a cycle") can also use a standalone `CompositionGraph` directly — every method here takes
/// `&self`/`&mut self`, none require a live `CompositionCoordinator`.
#[derive(Debug, Default)]
pub struct CompositionGraph {
    owns: HashMap<String, (String, String)>,
    links: HashMap<String, HashSet<String>>,
}

impl CompositionGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// 🔎️ The owning parent's artifact id, if `child_id` is currently tracked as owned.
    pub fn owner_of(&self, child_id: &str) -> Option<&str> {
        self.owns.get(child_id).map(|(parent_id, _slot)| parent_id.as_str())
    }

    /// 🔎️ The slot `child_id` currently occupies under its owner, if tracked.
    pub fn slot_of(&self, child_id: &str) -> Option<&str> {
        self.owns.get(child_id).map(|(_parent_id, slot)| slot.as_str())
    }

    /// ✅️ Whether owning `child_id` under `parent_id` would create a cycle — walks `parent_id`'s
    /// OWN ancestor chain looking for `child_id`; finding it means `child_id` would become both an
    /// ancestor and a (prospective) descendant of `parent_id`. Also true when `parent_id ==
    /// child_id` (an artifact cannot own itself).
    pub fn would_cycle_owns(&self, parent_id: &str, child_id: &str) -> bool {
        if parent_id == child_id {
            return true;
        }
        let mut current = parent_id.to_string();
        let mut guard = 0usize;
        while let Some((owner, _slot)) = self.owns.get(&current) {
            if owner == child_id {
                return true;
            }
            current = owner.clone();
            guard += 1;
            if guard > self.owns.len() + 1 {
                break;
            }
        }
        false
    }

    /// 🌱️ Records `child_id` as owned by `parent_id` in `slot`. Rejects (a) adopting a child
    /// already owned by a DIFFERENT parent (single-ownership invariant) and (b) any edge that would
    /// cycle (see `would_cycle_owns`) — re-owning by the SAME parent/slot is idempotent, not an error.
    pub fn insert_owns(&mut self, parent_id: &str, slot: &str, child_id: &str) -> Result<(), String> {
        if let Some((existing_owner, _existing_slot)) = self.owns.get(child_id) {
            if existing_owner != parent_id {
                return Err(format!("{child_id} is already owned by {existing_owner}, cannot also be owned by {parent_id}"));
            }
        }
        if self.would_cycle_owns(parent_id, child_id) {
            return Err(format!("owning {child_id} under {parent_id} would create a composition cycle"));
        }
        self.owns.insert(child_id.to_string(), (parent_id.to_string(), slot.to_string()));
        Ok(())
    }

    /// ✂️ Removes `child_id`'s ownership edge (e.g. on `extract`/`delete`), returning
    /// `(parent_id, slot)` if it was tracked.
    pub fn remove_owns(&mut self, child_id: &str) -> Option<(String, String)> {
        self.owns.remove(child_id)
    }

    /// ✅️ Whether linking `from -> to` would create a cycle — true when `to` can already reach
    /// `from` (including `from == to`).
    pub fn would_cycle_links(&self, from: &str, to: &str) -> bool {
        if from == to {
            return true;
        }
        let mut stack = vec![to.to_string()];
        let mut seen: HashSet<String> = HashSet::new();
        while let Some(node) = stack.pop() {
            if node == from {
                return true;
            }
            if !seen.insert(node.clone()) {
                continue;
            }
            if let Some(targets) = self.links.get(&node) {
                stack.extend(targets.iter().cloned());
            }
        }
        false
    }

    /// 🔗️ Records a `from -> to` link edge. Rejects a cycle (see `would_cycle_links`); adding the
    /// same edge twice is idempotent.
    pub fn insert_link(&mut self, from: &str, to: &str) -> Result<(), String> {
        if self.would_cycle_links(from, to) {
            return Err(format!("linking {from} -> {to} would create a cycle"));
        }
        self.links.entry(from.to_string()).or_default().insert(to.to_string());
        Ok(())
    }

    /// ✂️ Removes one `from -> to` link edge, if present.
    pub fn remove_link(&mut self, from: &str, to: &str) {
        if let Some(targets) = self.links.get_mut(from) {
            targets.remove(to);
        }
    }

    /// 🔎️ Every link target currently recorded FROM `from`.
    pub fn links_from(&self, from: &str) -> Vec<String> {
        self.links.get(from).map(|targets| targets.iter().cloned().collect()).unwrap_or_default()
    }

    /// 🔄️ Rebuilds `artifact_id`'s OWN outgoing edges (both `Owns`-as-parent and `Links`-as-source)
    /// from its live `ArtifactRefs` projection — the incremental-maintenance seam a host (e.g.
    /// `SpaceHost`) calls after every dispatch that might have changed `artifact_id`'s
    /// children/links, instead of ever recomputing the whole graph from scratch. Never touches
    /// edges where `artifact_id` is the TARGET (another artifact's own `sync_member` call owns those).
    pub fn sync_member<P: ArtifactRefs>(&mut self, artifact_id: &str, snapshot: &P) -> Result<(), String> {
        self.owns.retain(|_child_id, (parent_id, _slot)| parent_id != artifact_id);
        for child in snapshot.child_refs() {
            self.insert_owns(artifact_id, &child.slot, &child.child_id)?;
        }
        self.links.remove(artifact_id);
        for link in snapshot.links() {
            self.insert_link(artifact_id, &link.target.artifact_id)?;
        }
        Ok(())
    }
}

/// @emoji 🧵️ Builds the exact `ArtifactCommand::<Mutation>::Apply` binary layout (see that impl's
/// `OpBinary::encode_op` in `🔖️CommandFormat` above, ordinal-0 arm) directly from already-
/// `OpBinary`-encoded op bytes, WITHOUT ever decoding them — this is how `CompositionCoordinator`
/// stays fully generic over every technology's concrete `Mutation` type while still producing bytes
/// `SpaceMember::dispatch_wire`'s blanket impl (which DOES know the concrete type, on the receiving
/// member) can decode correctly via the ordinary `dispatch_binary` path. Must be kept byte-for-byte
/// in sync with `write_command_ops`/`ArtifactCommand::encode_op`'s ordinal-0 (`Apply`) arm.
fn build_apply_command_bytes(ops: &[Vec<u8>], description: Option<&str>) -> Vec<u8> {
    let mut out = vec![COMMAND_BINARY_FORMAT];
    crate::os_pack::write_varint_u64(&mut out, 0);
    out.push(if description.is_some() { 0b01 } else { 0 });
    if let Some(text) = description {
        write_command_str(&mut out, text);
    }
    crate::os_pack::write_varint_u64(&mut out, ops.len() as u64);
    for op in ops {
        crate::os_pack::write_varint_u64(&mut out, op.len() as u64);
        out.extend_from_slice(op);
    }
    out
}

/// @emoji 🧮️ Deterministic order-and-length-sensitive fingerprint of a raw op-bytes slice — the
/// `parent_edit_fingerprint` ingredient `mint_child_id`/`mint_invocation_id` hash into a new id, so
/// two replicas that receive the identical `parent_ops`/`ChildDispatch.ops` converge on identical
/// ids without ever needing to actually apply anything first.
fn concat_ops_fingerprint(ops: &[Vec<u8>]) -> Vec<u8> {
    let mut buffer = Vec::new();
    for op in ops {
        buffer.extend_from_slice(&(op.len() as u64).to_le_bytes());
        buffer.extend_from_slice(op);
    }
    buffer
}

/// @emoji 🆔️ Deterministic child envelope/handle id — design doc §1's "Child envelope ids are
/// minted deterministically... `parent_id ++ slot ++ parent_edit_fingerprint ++ ordinal`" formula,
/// using the SAME `content_addressed_entity_id` helper `🌿️vcs`'s other `mint_*_id` functions build
/// on. This one id serves BOTH roles: it is the new child's `ArtifactRef.artifact_id` AND its
/// `ArtifactChild<S>.child_id`/`OwnerRef.child_id` — a composition slot instance and the artifact
/// envelope it points at share one identity, so there is nothing to keep in sync between them.
pub fn mint_child_id(parent_id: &str, slot: &str, parent_edit_fingerprint: &[u8], ordinal: u32) -> String {
    let mut payload = parent_id.as_bytes().to_vec();
    payload.push(0);
    payload.extend_from_slice(slot.as_bytes());
    payload.push(0);
    payload.extend_from_slice(parent_edit_fingerprint);
    payload.push(0);
    payload.extend_from_slice(&ordinal.to_le_bytes());
    content_addressed_entity_id("child", &payload)
}

/// @emoji 🆔️ Deterministic group/invocation id: hashes the parent id, the parent ops' fingerprint,
/// and every dispatched child's `(child_id, ops fingerprint)` pair (sorted by child id first, so
/// caller-supplied `children` order never affects convergence) — two replicas performing the
/// identical composite gesture (same parent, same ops everywhere) converge on the identical
/// `GroupReceipt.invocation_id`/`MutationMeta.group_id` stamp without any coordination.
fn mint_invocation_id(parent_id: &str, parent_edit_fingerprint: &[u8], child_fingerprints: &[(String, Vec<u8>)]) -> String {
    let mut ordered: Vec<&(String, Vec<u8>)> = child_fingerprints.iter().collect();
    ordered.sort_by(|(a, _), (b, _)| a.cmp(b));
    let mut payload = parent_id.as_bytes().to_vec();
    payload.push(0);
    payload.extend_from_slice(parent_edit_fingerprint);
    for (child_id, fingerprint) in ordered {
        payload.push(0);
        payload.extend_from_slice(child_id.as_bytes());
        payload.push(0);
        payload.extend_from_slice(fingerprint);
    }
    content_addressed_entity_id("invocation", &payload)
}

/// @emoji 🧯️ Folds a post-validation dispatch failure with its compensation report: if every
/// already-applied member rolled back cleanly (`report.skipped` empty), the ORIGINAL error is
/// returned unchanged (compensation is a transparent implementation detail on the success path);
/// otherwise wraps both into `VcsError::CompensationFailed` so the caller sees the full picture —
/// what failed AND what could not be rolled back — rather than either fact silently.
fn fold_compensation_error(original: VcsError, report: GroupUndoReport) -> VcsError {
    if report.skipped.is_empty() {
        original
    } else {
        let skipped_desc: Vec<String> = report.skipped.iter().map(|(reference, error)| format!("{}: {error}", reference.to_uri())).collect();
        VcsError::CompensationFailed(format!("original error: {original}; members that failed to roll back: [{}]", skipped_desc.join(", ")))
    }
}

/// @emoji 🧩️ Atomic composite dispatch across a parent + N children — see this region's doc
/// comment for the two-phase protocol. Holds a `CompositionGraph` incrementally maintained across
/// calls (`graph`/`graph_mut` for a host to `sync_member` into, or to consult directly for UI-level
/// "would this cycle" checks without dispatching anything).
#[derive(Debug, Default)]
pub struct CompositionCoordinator {
    graph: CompositionGraph,
}

impl CompositionCoordinator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn graph(&self) -> &CompositionGraph {
        &self.graph
    }

    pub fn graph_mut(&mut self) -> &mut CompositionGraph {
        &mut self.graph
    }

    /// ✂️ Undoes `undo`/re-dispatches `undo` on the applied-so-far members in REVERSE application
    /// order (parent — if it was itself applied — first, then children in reverse dispatch order),
    /// collecting a `GroupUndoReport` rather than propagating the first failure — see
    /// `dispatch_group`'s doc comment for why every already-applied member must still get a
    /// best-effort rollback attempt even if an earlier one in this same pass failed.
    fn compensate(parent_ref: &crate::os_io::ArtifactRef, parent: &mut dyn SpaceMember, children: &mut [(&mut dyn SpaceMember, ChildDispatch)], applied_children: &[(usize, String)], parent_applied: Option<&str>) -> GroupUndoReport {
        let mut undone = Vec::new();
        let mut skipped = Vec::new();
        if let Some(edit_id) = parent_applied {
            match parent.undo() {
                Ok(()) => undone.push((parent_ref.clone(), edit_id.to_string())),
                Err(error) => skipped.push((parent_ref.clone(), error)),
            }
        }
        for (index, edit_id) in applied_children.iter().rev() {
            let (member, dispatch) = &mut children[*index];
            match member.undo() {
                Ok(()) => undone.push((dispatch.child.clone(), edit_id.clone())),
                Err(error) => skipped.push((dispatch.child.clone(), error)),
            }
        }
        GroupUndoReport { undone, skipped }
    }

    /// 🧩️ Dispatches one composite gesture spanning `parent` + `children` (+ any brand-new
    /// `genesis` children) as a single atomic unit.
    ///
    /// **Phase 1 — validate-all, zero side effects.** Every non-empty op slice (`parent_ops`, each
    /// `ChildDispatch.ops`) is checked via `SpaceMember::validate_wire` against that member's
    /// CURRENT snapshot; every `children` entry's claimed ownership is checked against `self.graph`
    /// (`VcsError::OwnershipViolation` if the graph does not currently track `parent_ref` as that
    /// child's owner); every `genesis` slot's deterministic id is minted and checked for a cycle
    /// (`VcsError::CompositionCycle`) and a registered `ChildStoreFactory`
    /// (`VcsError::ValidationFailed` if none). Any failure here returns immediately — NOTHING has
    /// been dispatched anywhere yet.
    ///
    /// **Phase 2 — apply in fixed order: child geneses → child edits → parent ops.** This order
    /// guarantees a parent's own `Apply` (which typically ADDS the `ArtifactChild` handle pointing
    /// at a just-created genesis child) never references a child that does not exist locally yet.
    /// Every member that receives an `Apply` gets its tail edit's `MutationMeta.group_id` stamped
    /// with the same minted `invocation_id` right after dispatching it.
    ///
    /// **Compensation.** A failure during phase 2 (a `dispatch_wire`/`stamp_tail_group_id` call
    /// that phase 1's validation did not catch — e.g. a `Mutation::validate` that is not fully
    /// exhaustive, or a genuinely unexpected `VcsError`) triggers `compensate`: `Undo` on every
    /// already-applied member in reverse order. This is sound under `&mut` on every member for the
    /// whole call (single-threaded per-app actor discipline) because each such member's group edit
    /// IS its tail — exact-base undo is mechanical, never a mid-history removal. If compensation
    /// itself fails to fully roll back, the returned error is `VcsError::CompensationFailed`
    /// (`fold_compensation_error`) carrying both the original failure and which members could not
    /// be rolled back, rather than silently leaving partial state unreported.
    pub fn dispatch_group(&mut self, parent_ref: &crate::os_io::ArtifactRef, parent: &mut dyn SpaceMember, children: &mut [(&mut dyn SpaceMember, ChildDispatch)], parent_ops: Vec<Vec<u8>>, genesis: Vec<ChildGenesis>, meta: GroupMeta) -> Result<GroupReceipt, VcsError> {
        //#region Phase1Validate
        if !parent_ops.is_empty() {
            parent.validate_wire(&parent_ops).map_err(VcsError::ValidationFailed)?;
        }
        for (member, dispatch) in children.iter() {
            match self.graph.owner_of(&dispatch.child.artifact_id) {
                Some(owner_id) if owner_id == parent_ref.artifact_id => {}
                _ => return Err(VcsError::OwnershipViolation(format!("{} is not a currently-tracked owned child of {}", dispatch.child.artifact_id, parent_ref.artifact_id))),
            }
            if !dispatch.ops.is_empty() {
                member.validate_wire(&dispatch.ops).map_err(VcsError::ValidationFailed)?;
            }
        }
        let parent_edit_fingerprint = concat_ops_fingerprint(&parent_ops);
        let mut minted_child_ids: Vec<String> = Vec::with_capacity(genesis.len());
        for (ordinal, spec) in genesis.iter().enumerate() {
            let child_id = mint_child_id(&parent_ref.artifact_id, &spec.slot, &parent_edit_fingerprint, ordinal as u32);
            if self.graph.would_cycle_owns(&parent_ref.artifact_id, &child_id) {
                return Err(VcsError::CompositionCycle(format!("creating child {child_id} in slot {} under {} would cycle", spec.slot, parent_ref.artifact_id)));
            }
            let kind = crate::os_io::ArtifactKindId::parse(&spec.dialect.artifact_kind).map_err(VcsError::ValidationFailed)?;
            if child_store_factory(&kind).is_none() {
                return Err(VcsError::ValidationFailed(format!("no ChildStoreFactory registered for kind {}", spec.dialect.artifact_kind)));
            }
            minted_child_ids.push(child_id);
        }
        //#endregion Phase1Validate

        //#region Phase2Apply
        let child_fingerprints: Vec<(String, Vec<u8>)> = children.iter().map(|(_, dispatch)| (dispatch.child.artifact_id.clone(), concat_ops_fingerprint(&dispatch.ops))).collect();
        let invocation_id = mint_invocation_id(&parent_ref.artifact_id, &parent_edit_fingerprint, &child_fingerprints);

        let mut created_children: Vec<(crate::os_io::ArtifactRef, Box<dyn SpaceMember>)> = Vec::with_capacity(genesis.len());
        for (ordinal, spec) in genesis.into_iter().enumerate() {
            let child_id = minted_child_ids[ordinal].clone();
            let kind = crate::os_io::ArtifactKindId::parse(&spec.dialect.artifact_kind).expect("validated in phase 1");
            let factory = child_store_factory(&kind).expect("validated in phase 1");
            // 🎯️ Nothing to compensate on a genesis failure: no `dispatch_wire` has run yet in this
            // call, and any earlier-succeeding genesis member in this same loop was never
            // registered/dispatched to anywhere — it simply gets dropped along with this `Err`.
            let mut member = factory.create(&child_id, &spec.dialect, &spec.initial_pack)?;
            let target = crate::os_io::ArtifactRef { artifact_id: child_id.clone(), dialect: spec.dialect.clone() };
            member.set_owner(Some(OwnerRef { parent: parent_ref.clone(), slot: spec.slot.clone(), child_id: child_id.clone() }));
            self.graph.insert_owns(&parent_ref.artifact_id, &spec.slot, &child_id).map_err(VcsError::OwnershipViolation)?;
            created_children.push((target, member));
        }

        let mut applied_children: Vec<(usize, String)> = Vec::new();
        for index in 0..children.len() {
            if children[index].1.ops.is_empty() {
                continue;
            }
            let command_bytes = build_apply_command_bytes(&children[index].1.ops, meta.description.as_deref());
            let receipt = match children[index].0.dispatch_wire(&command_bytes) {
                Ok(receipt) => receipt,
                Err(error) => {
                    let report = Self::compensate(parent_ref, parent, children, &applied_children, None);
                    return Err(fold_compensation_error(error, report));
                }
            };
            let edit_id = receipt.edit_ids.last().cloned().unwrap_or_default();
            applied_children.push((index, edit_id));
            if let Err(error) = children[index].0.stamp_tail_group_id(&invocation_id) {
                let report = Self::compensate(parent_ref, parent, children, &applied_children, None);
                return Err(fold_compensation_error(error, report));
            }
        }

        let mut parent_edit_id: Option<String> = None;
        if !parent_ops.is_empty() {
            let command_bytes = build_apply_command_bytes(&parent_ops, meta.description.as_deref());
            let receipt = match parent.dispatch_wire(&command_bytes) {
                Ok(receipt) => receipt,
                Err(error) => {
                    let report = Self::compensate(parent_ref, parent, children, &applied_children, None);
                    return Err(fold_compensation_error(error, report));
                }
            };
            let edit_id = receipt.edit_ids.last().cloned().unwrap_or_default();
            if let Err(error) = parent.stamp_tail_group_id(&invocation_id) {
                let report = Self::compensate(parent_ref, parent, children, &applied_children, Some(&edit_id));
                return Err(fold_compensation_error(error, report));
            }
            parent_edit_id = Some(edit_id);
        }
        //#endregion Phase2Apply

        let mut member_edits: Vec<(crate::os_io::ArtifactRef, String)> = applied_children.iter().map(|(index, edit_id)| (children[*index].1.child.clone(), edit_id.clone())).collect();
        if let Some(edit_id) = parent_edit_id {
            member_edits.push((parent_ref.clone(), edit_id));
        }
        Ok(GroupReceipt { invocation_id, member_edits, created_children })
    }

    /// ↩️ Best-effort group undo: for every `(reference, member)` pair (caller-ordered — put the
    /// parent first, matching `dispatch_group`'s own "undo parent-first then children" fixed
    /// order), undoes it if and only if `member.tail_group_id() == Some(group_id)`; a member whose
    /// tail belongs to a different (or no) group, or whose own `undo()` call errors, is SKIPPED and
    /// recorded in the returned report rather than aborting the rest — see `GroupUndoReport`'s doc
    /// comment for why abort-all would be actively harmful here.
    pub fn undo_group(members: &mut [(&crate::os_io::ArtifactRef, &mut dyn SpaceMember)], group_id: &str) -> GroupUndoReport {
        let mut undone = Vec::new();
        let mut skipped = Vec::new();
        for (reference, member) in members.iter_mut() {
            let tail_group = member.tail_group_id();
            if tail_group.as_deref() == Some(group_id) {
                let edit_id = member.tail_edit_id().unwrap_or_default();
                match member.undo() {
                    Ok(()) => undone.push(((*reference).clone(), edit_id)),
                    Err(error) => skipped.push(((*reference).clone(), error)),
                }
            } else {
                skipped.push(((*reference).clone(), VcsError::ForeignEdit(member.document_id().to_string())));
            }
        }
        GroupUndoReport { undone, skipped }
    }

    /// ↪️ `undo_group`'s redo-direction mirror: caller orders `members` children-first (matching
    /// `dispatch_group`'s apply order, so redo re-establishes the group in the same order it was
    /// originally applied), redoing each member whose `redo_tail()` group id matches.
    pub fn redo_group(members: &mut [(&crate::os_io::ArtifactRef, &mut dyn SpaceMember)], group_id: &str) -> GroupUndoReport {
        let mut undone = Vec::new();
        let mut skipped = Vec::new();
        for (reference, member) in members.iter_mut() {
            match member.redo_tail() {
                Some((edit_id, Some(tail_group))) if tail_group == group_id => match member.redo() {
                    Ok(()) => undone.push(((*reference).clone(), edit_id)),
                    Err(error) => skipped.push(((*reference).clone(), error)),
                },
                _ => skipped.push(((*reference).clone(), VcsError::NothingToRedo)),
            }
        }
        GroupUndoReport { undone, skipped }
    }
}
//#endregion 🔖️CompositionCoordinator

//#region 🔖️TestSupport
/// @emoji 🧪️ Round-trip assertions shared by every technology crate's `Mutation` test suite.
pub mod test_support {
    use super::*;

    /// @emoji 🔁️ Asserts that applying `operation` then applying its reversed `inverse(pre)` restores `pre`.
    pub fn assert_operation_round_trip<P, Mutation>(pre: &P, operation: Mutation)
    where
        P: Clone + PartialEq + std::fmt::Debug,
        Mutation: self::Mutation<P>,
    {
        let post = apply_mutation(pre, &operation);
        let mut inverse = operation.inverse(pre);
        inverse.reverse();
        let restored = inverse.iter().fold(post, |snapshot, back_operation| apply_mutation(&snapshot, back_operation));
        assert_eq!(&restored, pre, "operation inverse did not restore pre-state");
    }

    /// @emoji 🗄️ Asserts a full store round trip: Apply→Undo restores `initial`, Redo restores the
    /// post-apply snapshot, and replay-materialization agrees with the live store snapshot.
    pub fn assert_store_roundtrip<P, Mutation>(initial: P, operation: Mutation)
    where
        P: Clone + Serialize + DeserializeOwned + ArtifactPack + PartialEq + std::fmt::Debug,
        Mutation: Clone + Serialize + DeserializeOwned + self::Mutation<P> + OpBinary + OpText,
    {
        let envelope = create_document_envelope("test/v1", "test", initial.clone(), None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![operation], description: None }).expect("apply");
        let post = store.snapshot().expect("post snapshot");
        store.dispatch(ArtifactCommand::Undo).expect("undo");
        assert_eq!(store.snapshot().expect("undo snapshot"), initial, "undo did not restore initial snapshot");
        store.dispatch(ArtifactCommand::Redo).expect("redo");
        assert_eq!(store.snapshot().expect("redo snapshot"), post, "redo did not restore post snapshot");
        let replayed = materialize_document_snapshot(store.envelope(), store.applied_edit_ids()).expect("replay");
        assert_eq!(replayed, post, "materialization from replay diverged from store snapshot");
    }

    /// @emoji 📜️ Asserts a DSL round trip: `P::parse_dsl(&snapshot.print_dsl())` recovers an equal
    /// snapshot. The compile-time validation ground truth for every technology's `🔖️Dsl` region —
    /// call this from a `#[test]` over every `include_str!` fixture.
    pub fn assert_dsl_round_trip<P>(snapshot: &P)
    where
        P: ArtifactDsl + PartialEq + std::fmt::Debug,
    {
        let printed = snapshot.print_dsl();
        let parsed = P::parse_dsl(&printed).unwrap_or_else(|error| panic!("dsl parse failed: {error}"));
        assert_eq!(&parsed, snapshot, "dsl round trip diverged;\nprinted:\n{printed}");
    }

    /// @emoji 🧮️ Config artifact twin of [`assert_dsl_round_trip`] — same law for `ConfigRecord` snapshots.
    pub fn assert_config_round_trip<C>(snapshot: &C)
    where
        C: ConfigRecord + PartialEq + std::fmt::Debug,
    {
        assert_dsl_round_trip(snapshot);
    }

    /// @emoji 🧭️ Non-panicking twin of [`assert_dsl_round_trip`] for a repo-wide fixture-law SWEEP
    /// (W6: `.🦑️repo/🎫️tickets/.../DSL-FIXTURE-LAW-SWEEP`): checks BOTH laws directly against real
    /// shipped `📚️examples/**` fixture TEXT (not a hand-built in-memory value), which is exactly what
    /// a single per-app round-trip test built on its own simpler hardcoded example can miss — a
    /// printer/parser asymmetry only a real fixture's actual formatting (comment placement, field
    /// order, quoting) would trip.
    ///
    /// **Law 1 — parse→print→reparse fixpoint**: `text` parses to `first`; printing then reparsing
    /// `first` must recover an equal value (`second`) — the generic form of [`assert_dsl_round_trip`],
    /// but starting from arbitrary fixture text instead of an already-canonical in-memory value.
    ///
    /// **Law 2 — canonicalize idempotence**: `canonicalize(x) := print_dsl(parse_dsl(x))`. For every
    /// derive-generated `ArtifactDsl` impl this IS `crate::os_dsl::schema::canonicalize(x, spec, opts)`
    /// (`__rt::print_document_record`/`parse_document_record` route straight through
    /// `crate::os_dsl::schema::parse`/`print` in `JoinMode::Document`, the exact pair `canonicalize` composes)
    /// — and it is the correct generalization for hand-rolled (Route A idiom) `ArtifactDsl` impls
    /// that have no `RecordSpec` at all. `canonicalize(text) == printed_once`;
    /// `canonicalize(printed_once) == printed_twice`; idempotence is `printed_once == printed_twice`.
    ///
    /// Returns `Ok(())` on success, `Err(description)` on the first law violated — never panics, so
    /// a caller sweeping many fixture files can collect every failure before reporting.
    pub fn check_dsl_fixture_text_laws<P>(text: &str) -> Result<(), String>
    where
        P: ArtifactDsl + PartialEq,
    {
        let first = P::parse_dsl(text).map_err(|error| format!("parse failed: {error}"))?;
        let printed_once = first.print_dsl();
        let second = P::parse_dsl(&printed_once).map_err(|error| format!("reparse failed: {error}\nprinted:\n{printed_once}"))?;
        if first != second {
            return Err(format!("parse->print->reparse fixpoint diverged;\nprinted:\n{printed_once}"));
        }
        let printed_twice = second.print_dsl();
        if printed_once != printed_twice {
            return Err(format!("canonicalize is not idempotent;\nonce:\n{printed_once}\ntwice:\n{printed_twice}"));
        }
        Ok(())
    }

    /// @emoji 📦️ Asserts a pack round trip: `P::decode_pack(&snapshot.encode_pack())` recovers an
    /// equal snapshot — the pack sibling of `assert_dsl_round_trip`.
    pub fn assert_pack_round_trip<P>(snapshot: &P)
    where
        P: ArtifactPack + PartialEq + std::fmt::Debug,
    {
        let bytes = snapshot.encode_pack();
        let decoded = P::decode_pack(&bytes).unwrap_or_else(|error| panic!("pack decode failed: {error}"));
        assert_eq!(&decoded, snapshot, "pack round trip diverged");
    }

    /// @emoji ⚖️ Asserts dsl and pack are two encodings of the SAME value: `decode_pack(
    /// encode_pack(p)) == parse_dsl(print_dsl(p)) == p` — the compile-time validation ground truth
    /// for the whole pack rollout's central LAW (see `ArtifactPack`'s doc comment).
    pub fn assert_dsl_pack_equivalence<P>(snapshot: &P)
    where
        P: ArtifactDsl + ArtifactPack + Clone + PartialEq + std::fmt::Debug,
    {
        let via_pack = P::decode_pack(&snapshot.encode_pack()).unwrap_or_else(|error| panic!("pack decode failed: {error}"));
        let via_dsl = P::parse_dsl(&snapshot.print_dsl()).unwrap_or_else(|error| panic!("dsl parse failed: {error}"));
        assert_eq!(&via_pack, snapshot, "pack round trip diverged from source snapshot");
        assert_eq!(&via_dsl, snapshot, "dsl round trip diverged from source snapshot");
        assert_eq!(via_pack, via_dsl, "pack and dsl round trips diverged from each other");
    }

    /// @emoji ⚡️ Asserts an op-text round trip for a single operation: `print_op` contains no newline
    /// and `Op::parse_op` recovers an equal operation from it. The compile-time validation ground
    /// truth for every technology's `🔖️OpText` region — call this once per `Mutation` variant.
    pub fn assert_op_line_round_trip<Op>(operation: &Op)
    where
        Op: OpText + PartialEq + std::fmt::Debug,
    {
        let printed = operation.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got: {printed:?}");
        let parsed = Op::parse_op(&printed).unwrap_or_else(|error| panic!("op parse failed: {error}"));
        assert_eq!(&parsed, operation, "op-text round trip diverged; printed: {printed:?}");
    }

    /// @emoji ⚖️ Asserts op text and op binary are two encodings of the SAME operation:
    /// `decode_op(encode_op(op)) == parse_op(print_op(op)) == op`, and the binary encoding is
    /// deterministic. The compile-time validation ground truth for every technology's `OpBinary`
    /// impl — the op-level mirror of {@link assert_dsl_pack_equivalence}.
    pub fn assert_op_text_binary_equivalence<Op>(operation: &Op)
    where
        Op: OpText + OpBinary + PartialEq + std::fmt::Debug,
    {
        assert_op_line_round_trip(operation);
        let encoded = operation.encode_op().unwrap_or_else(|error| panic!("op encode failed: {error}"));
        let encoded_again = operation.encode_op().unwrap_or_else(|error| panic!("op re-encode failed: {error}"));
        assert_eq!(encoded, encoded_again, "op binary encoding is not deterministic");
        let decoded = Op::decode_op(&encoded).unwrap_or_else(|error| panic!("op decode failed: {error}"));
        assert_eq!(&decoded, operation, "op-binary round trip diverged from source operation");
    }

    /// @emoji ⚖️ Asserts command text and command binary are two encodings of the SAME command:
    /// `ArtifactCommand::decode_op(&c.encode_op()) == parse_command(print_command(c)) == c`, and the
    /// binary encoding is deterministic. The compile-time validation ground truth for
    /// `ArtifactCommand`'s text/binary pair — the command-level mirror of
    /// `assert_op_text_binary_equivalence`.
    pub fn assert_command_text_binary_equivalence<Op>(command: &ArtifactCommand<Op>)
    where
        Op: OpText + OpBinary + Clone + PartialEq + std::fmt::Debug,
    {
        let printed = print_command(command).unwrap_or_else(|error| panic!("command print failed: {error}"));
        let parsed: ArtifactCommand<Op> = parse_command(&printed).unwrap_or_else(|error| panic!("command parse failed: {error}"));
        assert_eq!(&parsed, command, "command text round trip diverged; printed:\n{printed}");
        let encoded = command.encode_op().unwrap_or_else(|error| panic!("command encode failed: {error}"));
        let encoded_again = command.encode_op().unwrap_or_else(|error| panic!("command re-encode failed: {error}"));
        assert_eq!(encoded, encoded_again, "command binary encoding is not deterministic");
        let decoded: ArtifactCommand<Op> = ArtifactCommand::<Op>::decode_op(&encoded).unwrap_or_else(|error| panic!("command decode failed: {error}"));
        assert_eq!(&decoded, command, "command binary round trip diverged from source command");
    }

    /// @emoji 📄️ Asserts that printing a store's envelope to text and parsing it back yields the same
    /// live snapshot the store already holds — the ground truth for {@link print_document_text}/
    /// {@link parse_document_text} on any technology once it implements `ArtifactDsl` + `OpText`.
    pub fn assert_document_text_round_trip<P, Mutation>(store: &ArtifactStore<P, Mutation>)
    where
        P: Clone + ArtifactDsl + ArtifactPack + PartialEq + std::fmt::Debug + Serialize + DeserializeOwned,
        Mutation: Clone + OpText + self::Mutation<P> + PartialEq + Serialize + DeserializeOwned + OpBinary,
    {
        let live = store.snapshot().expect("store snapshot");
        let files = print_document_text(store.envelope()).expect("print document text");
        let parsed: ParsedDocumentText<P, Mutation> = parse_document_text(&files.dsl, &files.ops).unwrap_or_else(|error| panic!("parse document text failed: {error}"));
        assert_eq!(parsed.snapshot, live, "document-text round trip diverged from store snapshot");
    }

    /// @emoji 🗄️ Asserts a full pack-based document round trip: mirrors
    /// `assert_document_text_round_trip` but via `print_document_pack`/`parse_document_pack`, and
    /// additionally asserts the pack path's parsed snapshot agrees with the text path's — the two
    /// storage formats must never diverge on the same store.
    pub fn assert_document_pack_round_trip<P, Mutation>(store: &ArtifactStore<P, Mutation>)
    where
        P: Clone + ArtifactDsl + ArtifactPack + PartialEq + std::fmt::Debug + Serialize + DeserializeOwned,
        Mutation: Clone + OpText + OpBinary + self::Mutation<P> + PartialEq + Serialize + DeserializeOwned,
    {
        let live = store.snapshot().expect("store snapshot");
        let pack_files = print_document_pack(store.envelope()).expect("print document pack");
        let parsed_pack: ParsedDocumentText<P, Mutation> = parse_document_pack(&pack_files.pack, &pack_files.spr).unwrap_or_else(|error| panic!("parse document pack failed: {error}"));
        assert_eq!(parsed_pack.snapshot, live, "document-pack round trip diverged from store snapshot");

        let text_files = print_document_text(store.envelope()).expect("print document text");
        let parsed_text: ParsedDocumentText<P, Mutation> = parse_document_text(&text_files.dsl, &text_files.ops).unwrap_or_else(|error| panic!("parse document text failed: {error}"));
        assert_eq!(parsed_pack.snapshot, parsed_text.snapshot, "document-pack path diverged from document-text path");
    }

    /// @emoji ✉️ Asserts that converting an `Edit<Mutation>` into `crate::os_spr::MutationEnvelope`s
    /// (`protocol_causal`'s canonical wire/causal representation, moved from `framework/core` in CW3,
    /// via `crate::os_spr::mutation_envelope_from_edit`) preserves every operation's essential facts —
    /// the causal-wire sibling of `assert_pack_round_trip`/`assert_dsl_round_trip` for the app
    /// fan-out's "pack laws" cluster.
    ///
    /// `MutationEnvelope` is a runtime struct that is never itself re-serialized back into an
    /// `Edit` (unlike `encode_pack`/`decode_pack`, there is no `envelope_to_edit` inverse — vcs's OWN
    /// `edit_from_operation_envelope` recovers a *whole edit* from vcs's own, differently-shaped,
    /// per-edit `semio_framework::MutationEnvelope`, not from this per-operation
    /// `protocol_causal` one), so a byte-level encode-then-decode law is not meaningful here.
    /// Instead this checks the two LAWS that actually matter for this bridge: (1) whatever
    /// `edit.mutation_meta` explicitly recorded for a slot (the ground-truth source
    /// `mutation_envelope_from_edit` prefers over its own `Mutation`-trait/structural fallbacks —
    /// see that function's own doc comment) survives unchanged onto the envelope's
    /// `mutation_id`/`dependencies`/`actor`/`timestamp`; and (2) `envelope.diff.payload`/
    /// `envelope.inverse.inverse_diff` decode back (via `Mutation`'s own `Deserialize` impl) into
    /// operations equal to `edit.forwards[i]`/`edit.inverse[i]` — the part a hand-rolled
    /// `Serialize`/`Deserialize` pair can silently break. Deliberately does NOT recompute the
    /// envelope's fallback chain (id/actor/deps when `mutation_meta` is absent) itself, since doing
    /// so would just re-run `mutation_envelope_from_edit`'s own logic against itself and always
    /// agree — see this function's `🧪️Tests` sibling for a deliberately lossy `Mutation` impl that
    /// trips law (2).
    pub fn assert_command_envelope_round_trip<P, Mutation>(edit: &Edit<Mutation>, document_id: &ArtifactId, schema: &SchemaId)
    where
        P: Clone + PartialEq + std::fmt::Debug,
        Mutation: self::Mutation<P> + PartialEq + std::fmt::Debug + OpBinary,
    {
        let envelopes = crate::os_spr::mutation_envelope_from_edit::<P, Mutation>(edit, document_id, schema).unwrap_or_else(|error| panic!("mutation_envelope_from_edit must succeed for a well-formed edit: {error}"));
        assert_eq!(envelopes.len(), edit.forwards.len(), "one envelope must be produced per forward operation");
        for (index, envelope) in envelopes.iter().enumerate() {
            assert_eq!(envelope.document_id, *document_id, "document id did not survive the envelope conversion");
            if let Some(meta) = edit.mutation_meta.get(index) {
                if let Some(mutation_id) = &meta.mutation_id {
                    assert_eq!(&envelope.mutation_id, mutation_id, "explicit operation id did not survive the envelope conversion");
                }
                assert_eq!(envelope.dependencies, meta.dependencies, "explicit dependencies did not survive the envelope conversion");
                if let Some(author_id) = &meta.author_id {
                    assert_eq!(&envelope.actor, author_id, "explicit author id did not survive the envelope conversion");
                }
                assert_eq!(envelope.timestamp, meta.timestamp, "explicit timestamp did not survive the envelope conversion");
            }
            let recovered_forward = Mutation::decode_op(&envelope.diff.payload).unwrap_or_else(|error| panic!("envelope diff payload must decode back into an equal operation: {error}"));
            assert_eq!(&recovered_forward, &edit.forwards[index], "envelope diff payload did not decode back into an equal forward operation");
            match edit.inverse.get(index) {
                Some(backward) => {
                    let recovered_backward = Mutation::decode_op(&envelope.inverse.payload).unwrap_or_else(|error| panic!("envelope inverse payload must decode back into an equal operation: {error}"));
                    assert_eq!(&recovered_backward, backward, "envelope inverse payload did not decode back into an equal backward operation");
                }
                None => assert!(envelope.inverse.payload.is_empty(), "inverse payload must be empty when the edit has no corresponding inverse op"),
            }
        }
    }

    /// @emoji 🩺️ Asserts the store's incrementally-maintained live snapshot agrees with a
    /// from-scratch full replay — the differential check for `ArtifactStore`'s stateful `current`
    /// field. Call after arbitrary command sequences (apply/amend/undo/redo/checkpoint/switch
    /// interleavings) in a tech's own tests to confirm the incremental fast paths never diverge from
    /// the replay ground truth.
    pub fn assert_live_equals_replay<P, Mutation>(store: &ArtifactStore<P, Mutation>)
    where
        P: Clone + ArtifactPack + PartialEq + std::fmt::Debug + Serialize + DeserializeOwned,
        Mutation: Clone + Serialize + DeserializeOwned + self::Mutation<P> + OpBinary + OpText,
    {
        let live = store.snapshot().expect("store snapshot");
        let replayed = materialize_document_snapshot(store.envelope(), store.applied_edit_ids()).expect("replay");
        assert_eq!(live, replayed, "store's live snapshot diverged from full-replay materialization");
    }

    /// 🎚️ Strongest achieved native IO fidelity for export/reimport laws (S8).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IoFidelityClass {
        Exact,
        Canonical,
        Semantic,
        Lossy,
    }

    /// 🗂️ Vendored real-world example input for subset integrated roundtrip harnesses.
    pub struct ExampleAsset<'a> {
        pub bytes: &'a [u8],
        pub text: Option<&'a str>,
        pub provenance: &'a str,
    }

    /// 🪆 Subset-facing surface the integrated harness needs. Implement on a thin adapter per subset.
    pub trait SubsetRoundtripSpec {
        type Snapshot: Clone + PartialEq + std::fmt::Debug + ArtifactDsl + ArtifactPack + Serialize + DeserializeOwned;
        type Mutation: Clone + PartialEq + std::fmt::Debug + Mutation<Self::Snapshot> + OpText + OpBinary;
        type Inference: Clone + PartialEq + std::fmt::Debug + Default;
        fn dialect() -> crate::os_io::ArtifactDialect;
        fn fidelity() -> IoFidelityClass;
        fn drops() -> &'static [&'static str];
        fn parse_native(asset: &ExampleAsset<'_>) -> Result<Self::Snapshot, String>;
        fn export_native(snapshot: &Self::Snapshot) -> Result<Vec<u8>, String>;
        fn reimport_native(bytes: &[u8]) -> Result<Self::Snapshot, String>;
        fn infer(snapshot: &Self::Snapshot) -> Self::Inference;
        fn sample_mutations(snapshot: &Self::Snapshot) -> Vec<Self::Mutation>;
        fn validate_payload(bytes: &[u8]) -> Result<(), Vec<String>>;
        fn validate_negative(bytes: &[u8]) -> Result<Vec<String>, String>;
        fn is_derived() -> bool {
            false
        }
    }

    fn skip_stage(error: &str) -> bool {
        error.starts_with("SKIP:")
    }

    fn skip_validation(codes: &[String]) -> bool {
        codes.len() == 1 && skip_stage(&codes[0])
    }

    /// 🧪 Assert import/export byte fidelity matches the declared class at the raw-byte layer.
    pub fn assert_import_export_fidelity_bytes(original: &[u8], exported: &[u8], class: IoFidelityClass) {
        match class {
            IoFidelityClass::Exact => assert_eq!(exported, original, "exact fidelity requires byte-identical export"),
            IoFidelityClass::Canonical | IoFidelityClass::Semantic | IoFidelityClass::Lossy => {}
        }
    }

    /// 🎯 Assert two inference runs are identical (determinism law S6).
    pub fn assert_inference_determinism<I: PartialEq + std::fmt::Debug>(a: &I, b: &I) {
        assert_eq!(a, b, "inference is not deterministic across two runs on the same snapshot");
    }

    /// 🔁 Drive S0–S10 subset roundtrip stages. Stages that need unavailable hooks are skipped only when
    /// the corresponding trait method returns an explicit Err starting with "SKIP:"; otherwise failures panic with stage id.
    pub fn assert_subset_roundtrip<S: SubsetRoundtripSpec>(example: &ExampleAsset<'_>, negative: Option<&ExampleAsset<'_>>) {
        assert!(!example.bytes.is_empty(), "S0: example bytes must be non-empty");
        assert!(!example.provenance.is_empty(), "S0: example provenance must be non-empty");

        let dialect = S::dialect();
        assert!(!dialect.artifact_kind.is_empty(), "S1: dialect artifact_kind must be non-empty");
        assert!(!dialect.standard.is_empty(), "S1: dialect standard must be non-empty");
        assert!(!dialect.subset.is_empty(), "S1: dialect subset must be non-empty");

        let snapshot = match S::parse_native(example) {
            Ok(snapshot) => snapshot,
            Err(error) if skip_stage(&error) => return,
            Err(error) => panic!("S2 failed: {error}"),
        };

        assert_dsl_round_trip(&snapshot);
        assert_pack_round_trip(&snapshot);
        assert_dsl_pack_equivalence(&snapshot);

        let mutations = S::sample_mutations(&snapshot);
        for mutation in &mutations {
            assert_operation_round_trip(&snapshot, mutation.clone());
            assert_op_line_round_trip(mutation);
            assert_op_text_binary_equivalence(mutation);
        }

        assert_inference_determinism(&S::infer(&snapshot), &S::infer(&snapshot));

        if let Some(first) = mutations.first() {
            assert_store_roundtrip(snapshot.clone(), first.clone());
        }

        match S::export_native(&snapshot) {
            Ok(exported) => {
                assert_import_export_fidelity_bytes(example.bytes, &exported, S::fidelity());
                match S::reimport_native(&exported) {
                    Ok(reimported) => match S::fidelity() {
                        IoFidelityClass::Exact => {}
                        IoFidelityClass::Canonical | IoFidelityClass::Semantic => {
                            assert_eq!(&reimported, &snapshot, "S8: canonical/semantic fidelity requires equal snapshot after reimport");
                        }
                        IoFidelityClass::Lossy if S::drops().is_empty() => {
                            assert_eq!(&reimported, &snapshot, "S8: lossy fidelity with empty drop set requires equal snapshot after reimport");
                        }
                        IoFidelityClass::Lossy => {}
                    },
                    Err(error) if skip_stage(&error) => {}
                    Err(error) => panic!("S8 reimport failed: {error}"),
                }
            }
            Err(error) if skip_stage(&error) => {}
            Err(error) => panic!("S8 export failed: {error}"),
        }

        match S::validate_payload(example.bytes) {
            Ok(()) => {}
            Err(codes) if skip_validation(&codes) => {}
            Err(codes) => panic!("S9 validate_payload failed: {codes:?}"),
        }

        if S::is_derived() {
            if let Some(negative) = negative {
                match S::validate_negative(negative.bytes) {
                    Ok(codes) => assert!(!codes.is_empty(), "S9: derived negative must yield non-empty diagnostic codes"),
                    Err(error) if skip_stage(&error) => {}
                    Err(error) => panic!("S9 validate_negative failed: {error}"),
                }
            }
        }
    }
}
//#endregion 🔖️TestSupport

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, crate::os_dsl::DslArtifact)]
    #[dsl(id = "demo.doc", extension = "demo")]
    struct DemoSnapshot {
        n: i32,
    }

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6).
impl ArtifactDsl for DemoSnapshot {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str { Self::__DSL_ENVELOPE_ID }
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = match semio_format::split_text_preamble(text) { Ok((_, rest)) => rest, Err(_) => text };
        let record = crate::os_dsl::parse(body, &Self::__dsl_spec(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = crate::os_dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), crate::os_dsl::JoinMode::Document);
        let envelope = semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), semio_format::Component::Dsl, 1).expect("valid envelope_id");
        semio_format::wrap_text(&envelope, &body)
    }
}
/// 📦️ Handcrafted ArtifactPack (P6).
impl ArtifactPack for DemoSnapshot {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let inner = pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), semio_format::Component::Pack, 1).map_err(|e| PackError::Schema(e.to_string()))?;
        Ok(semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = semio_format::unwrap_binary(bytes).map_err(|e| PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as ArtifactDsl>::envelope_id() {
            return Err(PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(text_error_to_pack_error)
    }
    fn record_spec() -> Option<crate::os_dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}
//#endregion 🔖️ArtifactCodec


    // `impl crate::os_store::ArtifactPack for DemoSnapshot` is now generated automatically by
    // `#[derive(crate::os_dsl::DslArtifact)]` above (see dsl/derive/rs/lib.rs's `🔖️DslArtifact` region) —
    // same seam as its `impl crate::os_store::ArtifactDsl for DemoSnapshot` sibling.

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    struct DemoDiff {
        n: Option<i32>,
    }



    impl MutationDiff<DemoSnapshot> for DemoDiff {
        fn apply(&self, snapshot: &DemoSnapshot) -> DemoSnapshot {
            DemoSnapshot { n: self.n.unwrap_or(snapshot.n) }
        }

        fn absorb(&mut self, other: Self) {
            if other.n.is_some() {
                self.n = other.n;
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, crate::os_dsl::DslOps)]
    #[serde(tag = "operation")]
    enum DemoMutation {
        #[dsl(key = "set-n")]
        SetN { n: i32 },
    }

//#region 🔖️OpCodec
/// 🎞️ Handcrafted OpText (P6).
impl OpText for DemoMutation {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = crate::os_dsl::parse(
                    line,
                    &spec_fn(),
                    &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline },
                )?;
                return <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(crate::os_dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl OpBinary for DemoMutation {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(crate::os_spr::ProtocolError::Malformed {
            what: "op variant",
            offset: 0,
            detail: format!("keyword {keyword:?} is not a declared variant"),
        })?;
        let spec = (variants[ordinal].1)();
        let body = crate::os_pack::encode_record_body(&spec, &record, &PackEncodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        crate::os_pack::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = crate::os_pack::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(crate::os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(crate::os_spr::ProtocolError::Malformed {
            what: "op variant",
            offset: 1,
            detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()),
        })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
        <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| crate::os_spr::ProtocolError::Malformed {
            what: "op record",
            offset: reader.position() as u64,
            detail: error.to_string(),
        })
    }
}
//#endregion 🔖️OpCodec


    impl Mutation<DemoSnapshot> for DemoMutation {
        type Diff = DemoDiff;

        fn diff(&self, _snapshot: &DemoSnapshot) -> DemoDiff {
            match self {
                DemoMutation::SetN { n } => DemoDiff { n: Some(*n) },
            }
        }

        fn inverse(&self, snapshot: &DemoSnapshot) -> Vec<Self> {
            vec![DemoMutation::SetN { n: snapshot.n }]
        }
    }

    /// @emoji 🛰️ Builds a foreign {@link MutationEnvelope} (as if authored by `actor` on another peer) by

    /// applying `operation` in a throwaway peer store and stamping the envelope's actor id.
    fn foreign_mutation_envelope(actor: &str, operation: DemoMutation) -> crate::os_spr::MutationEnvelope {
        let mut peer = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: 0 }, None));
        peer.dispatch(ArtifactCommand::Apply { mutations: vec![operation], description: None }).expect("peer apply");
        let edit = peer.envelope().vcs.edits.last().expect("peer edit").clone();
        let document_id = ArtifactId(peer.envelope().id.clone());
        let schema = SchemaId(peer.envelope().schema.clone());
        let mut envelopes = crate::os_spr::mutation_envelope_from_edit::<DemoSnapshot, DemoMutation>(&edit, &document_id, &schema).expect("operation envelope");
        let mut envelope = envelopes.pop().expect("exactly one op envelope for a single-op edit");
        envelope.actor = ActorId(actor.to_string());
        envelope
    }

    #[test]
    fn materialize_replays_forward_mutations() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply");
        assert_eq!(store.snapshot().expect("snapshot").n, 1);
        assert_eq!(store.envelope().vcs.edits.len(), 1);
    }

    #[test]
    fn undo_redo_round_trip() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply");
        store.dispatch(ArtifactCommand::Undo).expect("undo");
        assert_eq!(store.snapshot().expect("snapshot").n, 0);
        store.dispatch(ArtifactCommand::Redo).expect("redo");
        assert_eq!(store.snapshot().expect("snapshot").n, 1);
    }

    #[test]
    fn apply_computes_backwards_from_pre_state() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 5 }], description: None }).expect("apply");
        let edit = &store.envelope().vcs.edits[0];
        assert_eq!(edit.inverse, vec![DemoMutation::SetN { n: 0 }]);
    }

    #[test]
    fn commit_checkpoint_wraps_edits_into_change() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("init".into()), authors: vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }] }).expect("commit");
        assert_eq!(store.envelope().vcs.changes.len(), 1);
        assert_eq!(store.envelope().vcs.checkpoints.len(), 1);
        assert_eq!(store.envelope().vcs.checkpoints[0].message, Some("init".into()));
    }

    #[test]
    fn checkout_checkpoint_restores_applied_edits() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c1".into()), authors: Vec::new() }).expect("commit");
        let checkpoint_id = store.envelope().vcs.checkpoints[0].id.clone();
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 9 }], description: None }).expect("apply2");
        assert_eq!(store.snapshot().expect("snapshot").n, 9);
        store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id }).expect("checkout");
        assert_eq!(store.snapshot().expect("snapshot").n, 1);
    }

    #[test]
    fn alternatives_switch_restores_checkpoint_chain() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply");
        store.dispatch(ArtifactCommand::CreateAlternative { name: "branch-a".into() }).expect("create alternative");
        let alt_id = store.envelope().vcs.alternatives[0].id.clone();
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).expect("apply on branch");
        store.dispatch(ArtifactCommand::SwitchAlternative { alternative_id: alt_id }).expect("switch");
        assert_eq!(store.snapshot().expect("snapshot").n, 1);
    }

    #[test]
    fn checkout_old_checkpoint_then_commit_creates_a_fork() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c1".into()), authors: Vec::new() }).expect("commit c1");
        let c1 = store.envelope().vcs.checkpoints[0].id.clone();
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).expect("apply");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c2".into()), authors: Vec::new() }).expect("commit c2");
        store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: c1.clone() }).expect("checkout c1");
        assert_eq!(store.current_checkpoint_id(), Some(c1.as_str()));
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 9 }], description: None }).expect("apply");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("fork".into()), authors: Vec::new() }).expect("commit fork");
        let children: Vec<&Checkpoint> = store.envelope().vcs.checkpoints.iter().filter(|checkpoint| checkpoint.parent_id.as_deref() == Some(c1.as_str())).collect();
        assert_eq!(children.len(), 2, "checking out an old checkpoint before committing must fork, not extend the trunk");
    }

    #[test]
    fn create_alternative_appends_commits_to_its_own_checkpoint_chain() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("root".into()), authors: Vec::new() }).expect("commit root");
        store.dispatch(ArtifactCommand::CreateAlternative { name: "feature-a".into() }).expect("create alternative");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).expect("apply");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("branch commit".into()), authors: Vec::new() }).expect("commit on branch");
        assert_eq!(store.envelope().vcs.alternatives[0].checkpoint_ids.len(), 2);
        assert_eq!(store.envelope().vcs.checkpoints.len(), 2);
    }

    #[test]
    fn history_columns_orders_newest_first_and_labels_trunk_root() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c1".into()), authors: Vec::new() }).expect("commit c1");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).expect("apply");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c2".into()), authors: Vec::new() }).expect("commit c2");
        let columns = store.history_columns();
        assert_eq!(columns.len(), 2);
        assert_eq!(columns[0].description, Some("c2".into()), "newest checkpoint must be first");
        assert_eq!(columns[0].lane, 0);
        assert_eq!(columns[0].labels, vec!["main".to_string()], "newest unlabeled row falls back to main");
        assert!(columns[1].labels.is_empty(), "only the newest row gets the main fallback");
        let json = serde_json::to_string(&columns[0]).expect("serialize");
        assert!(json.contains("checkpointId"), "wire format must be camelCase: {json}");
    }

    #[test]
    fn history_columns_assigns_distinct_lanes_and_pulls_main_only_descendants_to_trunk() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("root".into()), authors: Vec::new() }).expect("commit root");
        let root = store.envelope().vcs.checkpoints[0].id.clone();

        store.dispatch(ArtifactCommand::CreateAlternative { name: "feature-a".into() }).expect("create feature-a");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).expect("apply");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("a1".into()), authors: Vec::new() }).expect("commit a1");

        store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: root.clone() }).expect("checkout root");
        store.dispatch(ArtifactCommand::CreateAlternative { name: "feature-b".into() }).expect("create feature-b");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 3 }], description: None }).expect("apply");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("b1".into()), authors: Vec::new() }).expect("commit b1");

        store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: root.clone() }).expect("checkout root again");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 4 }], description: None }).expect("apply");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("main resumed".into()), authors: Vec::new() }).expect("commit main resumed");

        let columns = store.history_columns();
        assert_eq!(columns.len(), 4, "root + a1 + b1 + main-resumed");
        let by_message: HashMap<String, &HistoryColumn> = columns.iter().filter_map(|column| column.description.clone().map(|description| (description, column))).collect();
        assert_eq!(by_message["root"].lane, 0, "root has no parent, lane 0");
        assert_eq!(by_message["main resumed"].lane, 0, "commit with no alternative stays on the trunk");
        let a_lane = by_message["a1"].lane;
        let b_lane = by_message["b1"].lane;
        assert_ne!(a_lane, 0, "a1 belongs to an alternative, not the trunk");
        assert_ne!(b_lane, 0, "b1 belongs to an alternative, not the trunk");
        assert_ne!(a_lane, b_lane, "distinct alternatives must get distinct swimlanes");

        let root_children: Vec<&HistoryColumn> = columns.iter().filter(|column| column.parent_checkpoint_id.as_deref() == Some(root.as_str())).collect();
        assert_eq!(root_children.len(), 3, "root forked three ways: a1, b1, main-resumed");
    }

    #[test]
    fn backbone_message_binary_round_trips_every_variant() {
        let snapshot = BackboneMessage::Snapshot { pack: vec![1, 2, 3], spr: Vec::new() };
        assert_eq!(BackboneMessage::decode_op(&snapshot.encode_op().unwrap()).unwrap(), snapshot);

        let envelope = sample_envelope_for_backbone_test();
        let operations = BackboneMessage::Mutations { envelopes: crate::os_spr::encode_envelopes(&[envelope.clone(), envelope]) };
        assert_eq!(BackboneMessage::decode_op(&operations.encode_op().unwrap()).unwrap(), operations);

        let ack = BackboneMessage::Ack { op_ids: vec!["op-1".to_string(), "op-2".to_string()] };
        assert_eq!(BackboneMessage::decode_op(&ack.encode_op().unwrap()).unwrap(), ack);

        let empty_ack = BackboneMessage::Ack { op_ids: Vec::new() };
        assert_eq!(BackboneMessage::decode_op(&empty_ack.encode_op().unwrap()).unwrap(), empty_ack);
    }

    fn sample_envelope_for_backbone_test() -> crate::os_spr::MutationEnvelope {
        crate::os_spr::MutationEnvelope {
            mutation_id: MutationId("op-1".to_string()),
            document_id: ArtifactId("doc-1".to_string()),
            actor: ActorId("actor-1".to_string()),
            dependencies: Vec::new(),
            diff: crate::os_spr::ArtifactDiff { schema: SchemaId("demo/v1".to_string()), payload: vec![1, 2, 3] },
            inverse: crate::os_spr::InverseMutation { schema: SchemaId("demo/v1".to_string()), payload: Vec::new() },
            timestamp: HybridLogicalTimestamp::new(0, 0),
        }
    }

    #[test]
    fn no_backbone_by_default() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        assert!(envelope.backbone.is_none(), "a fresh document has no attached backbone");
        let store = ArtifactStore::new(envelope);
        assert!(store.backbone_ref().is_none());
    }

    #[test]
    fn memory_backbone_pair_propagates_edits_bidirectionally() {
        let (backbone_a, backbone_b) = MemoryBackbone::pair("peer-a", "peer-b");
        let envelope_a: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let envelope_b: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store_a = ArtifactStore::new(envelope_a);
        let mut store_b = ArtifactStore::new(envelope_b);
        store_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        store_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

        store_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply on a");
        store_b.tick().expect("tick b");
        assert_eq!(store_b.snapshot().expect("snapshot b").n, 1, "b receives a's edit");

        store_b.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).expect("apply on b");
        store_a.tick().expect("tick a");
        assert_eq!(store_a.snapshot().expect("snapshot a").n, 2, "a receives b's edit");
    }

    #[test]
    fn detach_backbone_stops_synchronizing_but_keeps_the_wip_graph() {
        let (backbone_a, backbone_b) = MemoryBackbone::pair("peer-a", "peer-b");
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store_a = ArtifactStore::new(envelope.clone());
        let mut store_b = ArtifactStore::new(envelope);
        store_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        store_b.attach_backbone(Box::new(backbone_b)).expect("attach b");
        store_a.detach_backbone();
        assert!(store_a.backbone_ref().is_none());

        store_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 9 }], description: None }).expect("apply after detach still works on the in-memory graph");
        assert_eq!(store_a.snapshot().expect("snapshot a").n, 9);
        store_b.tick().expect("tick b");
        assert_eq!(store_b.snapshot().expect("snapshot b").n, 0, "detached edits never reach the peer");
    }

    #[test]
    fn deserialized_envelope_with_stale_backbone_ref_never_auto_attaches() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut stale_json: serde_json::Value = serde_json::to_value(&envelope).expect("serialize envelope");
        stale_json["backbone"] = serde_json::json!({ "uri": "folder:///nonexistent/path" });
        let stale_envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = serde_json::from_value(stale_json).expect("deserialize envelope with stale backbone ref");

        let mut store = ArtifactStore::new(stale_envelope.clone());
        assert!(store.tick().expect("tick with no live backbone is a no-operation") == false, "no backbone was ever attached, so there is nothing to pump");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply works purely against the in-memory graph");
        assert_eq!(store.snapshot().expect("snapshot").n, 1);

        store.reset(stale_envelope, Vec::new(), Vec::new()).expect("reset");
        assert!(store.tick().expect("tick after set_state with no live backbone is a no-operation") == false, "set_state must not resurrect IO from a stale backbone descriptor either");
    }

    /// @emoji 🧬️ `26/08/10` D4 evolution slice regression check (the plan's own explicitly flagged
    /// risk): an envelope encoded BEFORE `dialect`/`migrated_from` existed — modeled here by hand-
    /// building the OLD JSON object shape rather than serializing a struct literal, so this test
    /// still fails honestly if the fields ever lose their `#[serde(default)]` — must still decode,
    /// with both new fields defaulting to `None`. Mirrors
    /// `deserialized_envelope_with_stale_backbone_ref_never_auto_attaches`'s technique one field
    /// family over.
    #[test]
    fn old_envelope_without_dialect_fields_still_decodes() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut old_shape_json: serde_json::Value = serde_json::to_value(&envelope).expect("serialize envelope");
        let old_shape_object = old_shape_json.as_object_mut().expect("envelope serializes to a JSON object");
        // 🕰️ Simulate bytes persisted before this wave: no `dialect`/`migratedFrom` keys present at
        // all (not even `null`) — the exact shape every already-on-disk envelope has today.
        old_shape_object.remove("dialect");
        old_shape_object.remove("migratedFrom");
        assert!(!old_shape_object.contains_key("dialect") && !old_shape_object.contains_key("migratedFrom"), "test setup must actually produce the old, field-absent shape");

        let decoded: ArtifactEnvelope<DemoSnapshot, DemoMutation> = serde_json::from_value(old_shape_json).unwrap_or_else(|error| panic!("an old-shaped envelope (predating dialect/migratedFrom) must still decode: {error}"));
        assert_eq!(decoded.dialect, None, "dialect must default to None when the key was entirely absent");
        assert_eq!(decoded.migrated_from, None, "migratedFrom must default to None when the key was entirely absent");
        assert_eq!(decoded.vcs.initial_snapshot, envelope.vcs.initial_snapshot, "the rest of the envelope must be unaffected");

        // 🔁️ And the forward direction: a freshly-populated envelope round-trips both new fields.
        let mut populated = envelope.clone();
        populated.dialect = Some(crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.gif".into(), standard: "89a".into(), subset: "*".into() });
        populated.migrated_from = Some(MigrationProvenance {
            document_id: "demo-old".into(),
            dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.gif".into(), standard: "87a".into(), subset: "*".into() },
            checkpoint_id: Some("ck-abc123".into()),
            migrated_at: now_iso(),
        });
        let populated_json = serde_json::to_value(&populated).expect("serialize populated envelope");
        let redecoded: ArtifactEnvelope<DemoSnapshot, DemoMutation> = serde_json::from_value(populated_json).expect("deserialize populated envelope");
        assert_eq!(redecoded.dialect, populated.dialect);
        assert_eq!(redecoded.migrated_from, populated.migrated_from);
    }

    #[test]
    fn document_codec_of_round_trips_dsl_and_pack_and_edit_text() {
        let codec = ArtifactCodec::of::<DemoSnapshot, DemoMutation>("demo/v1");
        assert_eq!(codec.schema, "demo/v1");
        assert_eq!(codec.extension, "demo.doc");

        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 4 }, None);
        let text_files = print_document_text(&envelope).expect("print document text");

        let (pack_files, dsl_mirror) = (codec.compile_dsl)(&text_files.dsl, &text_files.ops).expect("codec compile_dsl");
        assert_eq!(dsl_mirror, DemoSnapshot { n: 4 }.print_dsl(), "dsl mirror matches the initial snapshot's print_dsl");

        let mirrored = (codec.print_mirror)(&pack_files.pack, &pack_files.spr).expect("codec print_mirror");
        assert_eq!(mirrored.dsl, dsl_mirror, "print_mirror's dsl text agrees with compile_dsl's own mirror, no JSON round trip");

        let document_id = ArtifactId("demo".to_string());
        let schema = SchemaId("demo/v1".to_string());
        let edit = Edit {
            id: "edit-1".into(),
            actor: Some("peer".into()),
            forwards: vec![DemoMutation::SetN { n: 9 }],
            inverse: vec![DemoMutation::SetN { n: 4 }],
            mutation_meta: Vec::new(),
            description: None,
            coalesce_key: None,
            sequence_number: 1,
            started_at: "0".into(),
            finished_at: None,
        };
        let mut op_envelopes = crate::os_spr::mutation_envelope_from_edit::<DemoSnapshot, DemoMutation>(&edit, &document_id, &schema).expect("op envelopes");
        let op_envelope = op_envelopes.pop().expect("exactly one op envelope for a single-op edit");
        let edit_text = (codec.edit_text_from_envelope)(&op_envelope).expect("codec edit_text_from_envelope");
        assert!(edit_text.contains("set-n"), "edit text contains the printed op line: {edit_text:?}");
        assert!(!edit_text.contains('\n') || edit_text.trim_end_matches('\n').lines().count() <= 2, "one header line + one op line: {edit_text:?}");

        register_document_codec(codec);
        assert!(document_codec("demo/v1").is_some(), "registered codec is discoverable by schema string");
        assert!(document_codec("no-such-schema").is_none());
    }

    /// 🔎️ Ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W1 Task
    /// 4: confirms `register_document_codec`'s ACTUAL collision behavior for a later wave's
    /// load-bearing "13 subsets, 13 distinct ids, same artifact_kind+standard" design — the
    /// registry is a plain `HashMap<schema, ArtifactCodec>` and `register_document_codec` is
    /// documented "idempotent, safe to call repeatedly", i.e. a SECOND registration under the SAME
    /// id silently overwrites the first (last-registered-wins) rather than panicking. Distinct ids
    /// (the semio artifact's actual design) never collide and are unaffected either way; this test
    /// only documents what happens if two subsets' ids ever accidentally coincide (a typo, not by
    /// design) — currently: silent data loss of the first codec, not a loud panic.
    #[test]
    fn register_document_codec_same_id_twice_overwrites_silently_not_panics() {
        let first = ArtifactCodec::of::<DemoSnapshot, DemoMutation>("test.duplicate-id-probe/v1");
        let second = ArtifactCodec { pack_schema_hash: [7u8; 32], ..first.clone() };
        assert_ne!(first.pack_schema_hash, second.pack_schema_hash, "fixture precondition: the two codecs must be distinguishable");

        register_document_codec(first);
        register_document_codec(second.clone()); // must not panic — see doc comment above
        let resolved = document_codec("test.duplicate-id-probe/v1").expect("still registered after the second call");
        assert_eq!(resolved.pack_schema_hash, second.pack_schema_hash, "second registration silently won — no panic, no error, no side channel signaling the collision");
    }

    #[test]
    fn attach_reconciles_a_pushed_snapshot() {
        let (channel, remote) = ChannelBackbone::pair("chan");
        let seeded: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut seed_store = ArtifactStore::new(seeded);
        seed_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 5 }], description: None }).expect("apply");
        let seed_files = seed_store.snapshot_pack().expect("seed snapshot");
        remote.push(BackboneMessage::Snapshot { pack: seed_files.pack, spr: seed_files.spr }).expect("push snapshot");

        let fresh: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(fresh);
        store.attach_backbone(Box::new(channel)).expect("attach reconciles the pushed snapshot");
        assert_eq!(store.snapshot().expect("snapshot").n, 5, "adopted the pushed snapshot's edit");
    }

    #[test]
    fn channel_backbone_round_trips_between_store_and_actor() {
        let (channel, remote) = ChannelBackbone::pair("chan");
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.attach_backbone(Box::new(channel)).expect("attach");
        let attach_flush = remote.drain().expect("drain attach");
        assert!(attach_flush.iter().any(|message| matches!(message, BackboneMessage::Snapshot { .. })), "attach flushes a snapshot to the actor end: {attach_flush:?}");

        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 4 }], description: None }).expect("apply");
        let outbound = remote.drain().expect("drain apply");
        assert!(outbound.iter().any(|message| matches!(message, BackboneMessage::Mutations { .. })), "a local apply is sent outbound as mutations: {outbound:?}");

        remote.push(BackboneMessage::Mutations { envelopes: crate::os_spr::encode_envelopes(&[foreign_mutation_envelope("peer", DemoMutation::SetN { n: 8 })]) }).expect("push inbound operations");
        store.tick().expect("tick");
        assert_eq!(store.snapshot().expect("snapshot").n, 8, "store ingests the actor's inbound operations");
    }

    #[test]
    fn pump_acks_ingested_operations() {
        let (channel, remote) = ChannelBackbone::pair("chan");
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.attach_backbone(Box::new(channel)).expect("attach");
        let _ = remote.drain().expect("drain attach snapshot");

        let inbound = foreign_mutation_envelope("peer", DemoMutation::SetN { n: 7 });
        let mutation_id = inbound.mutation_id.0.clone();
        remote.push(BackboneMessage::Mutations { envelopes: crate::os_spr::encode_envelopes(&[inbound]) }).expect("push inbound operations");
        store.tick().expect("tick");
        assert_eq!(store.snapshot().expect("snapshot").n, 7, "ingested the inbound operation");

        let outbound = remote.drain().expect("drain ack");
        assert!(outbound.iter().any(|message| matches!(message, BackboneMessage::Ack { op_ids } if op_ids == &vec![mutation_id.clone()])), "successful operations ingest emits an Ack for the ingested operation ids: {outbound:?}");
    }

    #[test]
    fn exact_base_only_undo_refuses_a_foreign_tail() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("local apply");
        store.dispatch(ArtifactCommand::IngestRemote { envelope: foreign_mutation_envelope("peer", DemoMutation::SetN { n: 2 }) }).expect("ingest foreign");
        assert_eq!(store.snapshot().expect("snapshot").n, 2, "foreign edit sits at the tail");

        let error = store.dispatch(ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::ExactBaseOnly, semantic_command: None }).expect_err("undo must refuse a foreign tail");
        assert!(matches!(error, VcsError::ForeignEdit(_)), "got {error:?}");
        assert_eq!(store.snapshot().expect("snapshot").n, 2, "the timeline is untouched after refusal");
    }

    #[test]
    fn transform_against_concurrent_undo_skips_over_a_foreign_tail() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("local apply");
        let local_edit_id = store.applied_edit_ids()[0].clone();
        let foreign = foreign_mutation_envelope("peer", DemoMutation::SetN { n: 2 });
        let foreign_id = foreign.mutation_id.0.clone();
        store.dispatch(ArtifactCommand::IngestRemote { envelope: foreign }).expect("ingest foreign");
        assert_eq!(store.applied_edit_ids().len(), 2, "local + foreign are both applied");

        store.dispatch(ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::TransformAgainstConcurrent, semantic_command: None }).expect("transform undo removes the local edit from mid-timeline");
        assert_eq!(store.applied_edit_ids(), std::slice::from_ref(&foreign_id), "only the local edit is removed; the concurrent foreign edit stays applied");
        assert_eq!(store.redo_edit_ids(), std::slice::from_ref(&local_edit_id), "the local edit is on the redo stack");
        assert_eq!(store.snapshot().expect("snapshot").n, 2, "snapshot re-materializes from the foreign edit alone");

        store.dispatch(ArtifactCommand::Redo).expect("redo brings the local edit back");
        assert_eq!(store.applied_edit_ids().len(), 2);
        assert_eq!(store.snapshot().expect("snapshot").n, 1, "redo re-applies the local edit at the tail");
    }

    #[test]
    fn compensating_undo_dispatches_semantic_command() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 5 }], description: None }).expect("apply");
        let undo_apply = ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 0 }], description: Some("compensate".into()) };
        store.dispatch(ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::CompensatingAction, semantic_command: Some(Box::new(undo_apply)) }).expect("compensating undo");
        assert_eq!(store.snapshot().expect("snapshot").n, 0);
    }

    #[test]
    fn edit_mutations_exposes_the_latest_edit() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        assert!(store.edit_mutations().is_none(), "no edits yet");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 5 }], description: None }).expect("apply");
        let (forwards, inverse, meta) = store.edit_mutations().expect("edit operations");
        assert_eq!(forwards, &[DemoMutation::SetN { n: 5 }]);
        assert_eq!(inverse, &[DemoMutation::SetN { n: 0 }], "inverse restores the pre-state");
        assert_eq!(meta.len(), 1);
    }

    #[test]
    fn amend_last_absorbs_into_matching_coalesce_key() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN { n: 1 }], coalesce_key: Some("drag".into()) }).expect("first amend");
        store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN { n: 2 }], coalesce_key: Some("drag".into()) }).expect("second amend");
        assert_eq!(store.envelope().vcs.edits.len(), 1, "coalesced into a single edit");
        assert_eq!(store.snapshot().expect("snapshot").n, 2);
        store.dispatch(ArtifactCommand::Undo).expect("undo");
        assert_eq!(store.snapshot().expect("snapshot after undo").n, 0, "undo restores pre-gesture state in one step");
    }

    #[test]
    fn amend_last_incremental_path_matches_full_replay_over_many_amends() {
        // 🪢️ Regression guard for the incremental `AmendLast` path (see `AmendCache`): many sequential
        // amends into the same coalesced edit — e.g. a long slider drag — must still produce exactly the
        // same edit (forwards/inverse/mutation_meta length, final snapshot, one-step undo) as the
        // previous full-replay-every-time implementation, just without re-replaying history each time.
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        for n in 1..=50 {
            store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN { n }], coalesce_key: Some("drag".into()) }).expect("amend");
        }
        assert_eq!(store.envelope().vcs.edits.len(), 1, "still a single coalesced edit");
        let edit = store.envelope().vcs.edits.last().expect("edit");
        assert_eq!(edit.forwards.len(), 50);
        assert_eq!(edit.inverse.len(), 50);
        assert_eq!(edit.mutation_meta.len(), 50);
        assert_eq!(store.snapshot().expect("snapshot").n, 50);
        store.dispatch(ArtifactCommand::Undo).expect("undo");
        assert_eq!(store.snapshot().expect("snapshot after undo").n, 0, "one undo reverts the whole 50-step coalesced gesture");
    }

    #[test]
    fn amend_last_incremental_cache_survives_undo_redo_round_trip() {
        // 🪢️ Undo/redo only move edit ids between `applied_edit_ids`/`redo_edit_ids` — they never mutate
        // an edit's own `forwards`, so a cached post-snapshot keyed by `(edit_id, forwards_len)` stays
        // valid across an undo immediately followed by a redo of the very same coalesced edit.
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN { n: 1 }], coalesce_key: Some("drag".into()) }).expect("first amend");
        store.dispatch(ArtifactCommand::Undo).expect("undo");
        store.dispatch(ArtifactCommand::Redo).expect("redo");
        store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN { n: 2 }], coalesce_key: Some("drag".into()) }).expect("amend after undo/redo");
        assert_eq!(store.envelope().vcs.edits.len(), 1, "still coalesced into the original edit");
        assert_eq!(store.snapshot().expect("snapshot").n, 2);
        store.dispatch(ArtifactCommand::Undo).expect("undo again");
        assert_eq!(store.snapshot().expect("snapshot after undo").n, 0);
    }

    #[test]
    fn amend_last_starts_new_edit_when_coalesce_key_differs() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN { n: 1 }], coalesce_key: Some("drag-a".into()) }).expect("first drag");
        store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN { n: 2 }], coalesce_key: Some("drag-b".into()) }).expect("second drag");
        assert_eq!(store.envelope().vcs.edits.len(), 2, "distinct gestures are separate edits");
    }

    #[test]
    fn amend_last_does_not_absorb_into_committed_edit() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN { n: 1 }], coalesce_key: Some("drag".into()) }).expect("amend");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }).expect("commit");
        store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN { n: 2 }], coalesce_key: Some("drag".into()) }).expect("amend after commit");
        assert_eq!(store.envelope().vcs.edits.len(), 2, "committed edits are never amended, even with a matching coalesce key");
    }

    #[test]
    fn assert_subset_harness_fidelity_and_inference_helpers() {
        test_support::assert_import_export_fidelity_bytes(b"fixture", b"fixture", test_support::IoFidelityClass::Exact);
        test_support::assert_import_export_fidelity_bytes(b"fixture", b"other", test_support::IoFidelityClass::Canonical);
        test_support::assert_inference_determinism(&7_i32, &7_i32);
    }

    #[test]
    #[should_panic(expected = "exact fidelity requires byte-identical export")]
    fn assert_import_export_fidelity_bytes_exact_rejects_divergence() {
        test_support::assert_import_export_fidelity_bytes(b"fixture", b"other", test_support::IoFidelityClass::Exact);
    }

    #[test]
    fn test_support_round_trip_helpers_pass_for_demo_operation() {
        test_support::assert_operation_round_trip(&DemoSnapshot { n: 4 }, DemoMutation::SetN { n: 9 });
        test_support::assert_store_roundtrip(DemoSnapshot { n: 4 }, DemoMutation::SetN { n: 9 });

        let edit = Edit::<DemoMutation> {
            id: "edit-command-envelope".into(),
            actor: Some("actor-fallback".into()),
            forwards: vec![DemoMutation::SetN { n: 9 }],
            inverse: vec![DemoMutation::SetN { n: 4 }],
            mutation_meta: vec![MutationMeta {
                mutation_id: Some(MutationId("op-a".into())),
                dependencies: vec![MutationId("op-0".into())],
                base_version: 0,
                author_id: Some(ActorId("actor-explicit".into())),
                timestamp: HybridLogicalTimestamp::new(1, 1000),
                undo_policy: UndoPolicy::ExactBaseOnly,
                payload_hash: None,
                semantic_kind: None,
                label: None,
                group_id: None,
            }],
            description: None,
            coalesce_key: None,
            sequence_number: 1,
            started_at: "2026-07-27T00:00:00Z".into(),
            finished_at: None,
        };
        test_support::assert_command_envelope_round_trip::<DemoSnapshot, DemoMutation>(&edit, &ArtifactId("doc-command-envelope".into()), &SchemaId("demo/v1".into()));
    }

    /// @emoji 🪤️ Proves `assert_command_envelope_round_trip` is not a trivially-true check: a hand-rolled
    /// `Mutation` whose `Deserialize` impl silently drops its own field (encodes `n` faithfully but
    /// always decodes to `n: 0`) must trip law (2) of the doc comment on
    /// `assert_command_envelope_round_trip` — the same "deliberately lossy impl" pattern
    /// `protocol_testkit`'s `op_text_round_trip_panics_on_a_lossy_impl` uses for `assert_op_text_round_trip`.
    #[test]
    #[should_panic(expected = "did not decode back into an equal forward operation")]
    fn command_envelope_round_trip_panics_on_a_lossy_operation() {
        #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
        struct LossyDiff;

        impl MutationDiff<DemoSnapshot> for LossyDiff {
            fn apply(&self, snapshot: &DemoSnapshot) -> DemoSnapshot {
                snapshot.clone()
            }
            fn absorb(&mut self, _other: Self) {}
        }

        #[derive(Clone, Debug, PartialEq)]
        struct LossyMutation {
            n: i32,
        }

        impl Serialize for LossyMutation {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_i32(self.n)
            }
        }

        impl<'de> Deserialize<'de> for LossyMutation {
            fn deserialize<D: serde::Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
                Ok(LossyMutation { n: 0 })
            }
        }

        impl Mutation<DemoSnapshot> for LossyMutation {
            type Diff = LossyDiff;
            fn diff(&self, _snapshot: &DemoSnapshot) -> LossyDiff {
                LossyDiff
            }
            fn inverse(&self, _snapshot: &DemoSnapshot) -> Vec<Self> {
                vec![self.clone()]
            }
        }

        impl OpBinary for LossyMutation {
            fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
                Ok(self.n.to_le_bytes().to_vec())
            }
            fn decode_op(_bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
                Ok(LossyMutation { n: 0 })
            }
        }

        let edit = Edit::<LossyMutation> {
            id: "edit-lossy".into(),
            actor: None,
            forwards: vec![LossyMutation { n: 7 }],
            inverse: vec![],
            mutation_meta: vec![],
            description: None,
            coalesce_key: None,
            sequence_number: 0,
            started_at: "2026-07-27T00:00:00Z".into(),
            finished_at: None,
        };
        test_support::assert_command_envelope_round_trip::<DemoSnapshot, LossyMutation>(&edit, &ArtifactId("doc-lossy".into()), &SchemaId("lossy/v1".into()));
    }

    // `DemoSnapshot`'s `crate::os_store::ArtifactDsl` impl and `DemoMutation`'s `crate::os_store::OpText` impl are now
    // generated by `#[derive(crate::os_dsl::DslArtifact)]`/`#[derive(crate::os_dsl::DslOps)]` on the type definitions
    // themselves (see `DemoSnapshot`/`DemoMutation` above) — the `dsl_schema` grammar replaces
    // this crate's own hand-rolled `"n <value>"`/`"set-n <value>"` printer/parser.

    #[test]
    fn demo_dsl_round_trips() {
        test_support::assert_dsl_round_trip(&DemoSnapshot { n: 42 });
    }

    #[test]
    fn demo_dsl_pack_equivalence() {
        test_support::assert_dsl_pack_equivalence(&DemoSnapshot { n: 42 });
    }

    #[test]
    fn demo_op_text_round_trips() {
        test_support::assert_op_line_round_trip(&DemoMutation::SetN { n: 7 });
    }

    #[test]
    fn demo_op_binary_round_trips_and_matches_text() {
        let operation = DemoMutation::SetN { n: 7 };
        let encoded = operation.encode_op().expect("op encode");
        let encoded_again = operation.encode_op().expect("op re-encode");
        assert_eq!(encoded, encoded_again, "op binary encoding must be deterministic");
        assert_eq!(encoded[0], pack_rt::OP_BINARY_FORMAT);
        let decoded = DemoMutation::decode_op(&encoded).expect("op decode");
        assert_eq!(decoded, operation);
        let via_text = DemoMutation::parse_op(&operation.print_op()).expect("op parse");
        assert_eq!(via_text, decoded, "binary and text round trips diverged");
    }

    #[test]
    fn demo_op_binary_rejects_unknown_format_and_ordinal() {
        let operation = DemoMutation::SetN { n: 7 };
        let mut wrong_format = operation.encode_op().expect("op encode");
        wrong_format[0] = 9;
        assert!(DemoMutation::decode_op(&wrong_format).is_err(), "format 9 must be rejected");
        let out_of_range = [pack_rt::OP_BINARY_FORMAT, 0x7E];
        assert!(DemoMutation::decode_op(&out_of_range).is_err(), "ordinal beyond declared variants must be rejected");
    }

    #[test]
    fn print_edit_lines_emits_one_indented_line_per_forward_op() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply");
        let edit = store.envelope().vcs.edits.last().expect("edit");
        let printed = print_edit_lines(edit).expect("print edit lines");
        assert!(printed.starts_with("edit "), "got {printed:?}");
        assert!(printed.contains("\n  set-n n=1\n"));
    }

    #[test]
    fn document_text_round_trips_after_apply_and_checkpoint() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 3 }], description: Some("bump".into()) }).expect("apply");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c1".into()), authors: vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }] }).expect("commit");
        test_support::assert_document_text_round_trip(&store);
        test_support::assert_document_pack_round_trip(&store);
    }

    #[test]
    fn parse_document_text_rejects_invalid_op_line_with_span() {
        let files = ArtifactTextFiles { dsl: "n=0\n".to_string(), ops: "doc demo schema=demo/v1\nedit e1 started=\"1\"\n  not-an-op\n".to_string() };
        let error = parse_document_text::<DemoSnapshot, DemoMutation>(&files.dsl, &files.ops).unwrap_err();
        assert_eq!(error.span.line, 3);
    }

    /// @emoji 🩺️ Stresses the stateful `current`/`tail_undo_cache` fast paths — multi-op edits, amend
    /// gestures, undo/redo, and a checkpoint (cold-path recompute) all interleaved — against the
    /// full-replay differential oracle, so any divergence between the incremental paths and a
    /// from-scratch replay fails loudly here rather than surfacing as a silent snapshot bug later.
    #[test]
    fn stateful_current_matches_full_replay_across_interleaved_commands() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);

        // Multi-operation edit: current must fold both ops, matching a from-scratch replay.
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }, DemoMutation::SetN { n: 2 }], description: None }).expect("apply multi-op edit");
        test_support::assert_live_equals_replay(&store);
        assert_eq!(store.snapshot().expect("snapshot").n, 2);

        // Amend gesture: the first `AmendLast` cannot merge into the preceding `Apply`-created edit
        // (`Apply` never sets a `coalesce_key`, so it can never match), so it starts a NEW edit; the
        // second `AmendLast` shares that edit's key and merges into it — two edits total, the second
        // one carrying two coalesced increments (3 then 4).
        store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN { n: 3 }], coalesce_key: Some("drag".into()) }).expect("amend 1");
        store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN { n: 4 }], coalesce_key: Some("drag".into()) }).expect("amend 2");
        test_support::assert_live_equals_replay(&store);
        assert_eq!(store.snapshot().expect("snapshot").n, 4);
        assert_eq!(store.envelope().vcs.edits.len(), 2, "the amend gesture started its own edit, not a third");

        // Undo the whole amended edit (O(1) tail-cache path) restores the `Apply`-edit's state, not
        // the initial snapshot — only the amend gesture's edit is undone here.
        store.dispatch(ArtifactCommand::Undo).expect("undo");
        test_support::assert_live_equals_replay(&store);
        assert_eq!(store.snapshot().expect("snapshot").n, 2);
        store.dispatch(ArtifactCommand::Redo).expect("redo");
        test_support::assert_live_equals_replay(&store);
        assert_eq!(store.snapshot().expect("snapshot").n, 4);

        // Checkpoint (cold path through `checkout_checkpoint_internal` is NOT exercised by commit
        // itself, but a following apply + a second, older undo still must agree with replay).
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c1".into()), authors: Vec::new() }).expect("commit");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 5 }], description: None }).expect("apply after checkpoint");
        test_support::assert_live_equals_replay(&store);
        store.dispatch(ArtifactCommand::Undo).expect("undo after checkpoint");
        test_support::assert_live_equals_replay(&store);
        assert_eq!(store.snapshot().expect("snapshot").n, 4);
    }

    //#region 🏛️SpaceTests
    /// @emoji ⏱️ Like `DemoMutation` but with an explicit, test-controlled `timestamp()` override, so
    /// undo-ordering-by-HLT tests don't depend on real wall-clock resolution.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, crate::os_dsl::DslOps)]
    #[serde(tag = "operation")]
    enum TimestampedMutation {
        #[dsl(key = "set-n")]
        SetN { n: i32, physical_ms: u64 },
    }

//#region 🔖️OpCodec
/// 🎞️ Handcrafted OpText (P6).
impl OpText for TimestampedMutation {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = crate::os_dsl::parse(
                    line,
                    &spec_fn(),
                    &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline },
                )?;
                return <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(crate::os_dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl OpBinary for TimestampedMutation {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(crate::os_spr::ProtocolError::Malformed {
            what: "op variant",
            offset: 0,
            detail: format!("keyword {keyword:?} is not a declared variant"),
        })?;
        let spec = (variants[ordinal].1)();
        let body = crate::os_pack::encode_record_body(&spec, &record, &PackEncodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        crate::os_pack::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = crate::os_pack::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(crate::os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(crate::os_spr::ProtocolError::Malformed {
            what: "op variant",
            offset: 1,
            detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()),
        })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
        <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| crate::os_spr::ProtocolError::Malformed {
            what: "op record",
            offset: reader.position() as u64,
            detail: error.to_string(),
        })
    }
}
//#endregion 🔖️OpCodec




    impl Mutation<DemoSnapshot> for TimestampedMutation {
        type Diff = DemoDiff;

        fn diff(&self, _snapshot: &DemoSnapshot) -> DemoDiff {
            match self {
                TimestampedMutation::SetN { n, .. } => DemoDiff { n: Some(*n) },
            }
        }

        fn inverse(&self, snapshot: &DemoSnapshot) -> Vec<Self> {
            vec![TimestampedMutation::SetN { n: snapshot.n, physical_ms: 0 }]
        }

        fn timestamp(&self) -> Option<HybridLogicalTimestamp> {
            match self {
                TimestampedMutation::SetN { physical_ms, .. } => Some(HybridLogicalTimestamp::new(0, *physical_ms)),
            }
        }
    }

    /// @emoji 🪄️ Downcasts a registered `dyn SpaceMember` back to its concrete demo store.
    fn demo_member<'a, Mutation: self::Mutation<DemoSnapshot> + 'static>(host: &'a mut SpaceHost, document_id: &str) -> &'a mut ArtifactStore<DemoSnapshot, Mutation> {
        host.member_mut(document_id).expect("member registered").as_any_mut().downcast_mut::<ArtifactStore<DemoSnapshot, Mutation>>().expect("concrete member type matches")
    }

    #[test]
    fn register_space_documents_registers_manifest_collections_and_artifacts_together() {
        // 🎯️ Every member below gets at least one uncommitted edit (dirty), mirroring
        // `space_checkpoint_commits_dirty_members_and_pins_their_checkpoints`'s `member_a` — a fresh
        // member with zero edits and zero checkpoints has no `current_checkpoint_id` yet, which
        // `commit_space_checkpoint` requires of every registered member (dirty ones are auto-committed,
        // already-clean ones just need a prior checkpoint).
        let mut manifest = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "space-manifest", DemoSnapshot { n: 0 }, None));
        manifest.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply manifest edit");
        let mut collection_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "collection-a", DemoSnapshot { n: 0 }, None));
        collection_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).expect("apply collection a edit");
        let mut collection_b = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "collection-b", DemoSnapshot { n: 0 }, None));
        collection_b.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 3 }], description: None }).expect("apply collection b edit");
        let mut artifact_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "artifact-a", DemoSnapshot { n: 0 }, None));
        artifact_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 7 }], description: None }).expect("apply artifact edit");

        let mut host = SpaceHost::new(create_document_envelope(&format!("{S_SPACE_HISTORY_SCHEMA}/v1"), "studio", SpaceHistorySnapshot::default(), None));
        host.register_space_documents(Box::new(manifest), vec![Box::new(collection_a), Box::new(collection_b)], vec![Box::new(artifact_a)]);

        assert!(host.member("space-manifest").is_some(), "manifest registered");
        assert!(host.member("collection-a").is_some(), "collection a registered");
        assert!(host.member("collection-b").is_some(), "collection b registered");
        assert!(host.member("artifact-a").is_some(), "artifact registered");

        let space_checkpoint_id = host.commit_space_checkpoint("initial space checkpoint".into(), Vec::new()).expect("commit space checkpoint");
        let snapshot = host.meta_snapshot().expect("meta snapshot");
        let checkpoint = snapshot.checkpoints.iter().find(|checkpoint| checkpoint.id == space_checkpoint_id).expect("checkpoint recorded");
        assert_eq!(checkpoint.members.len(), 4, "manifest + 2 collections + 1 artifact all pinned atomically in one space checkpoint");
        let pinned_ids: HashSet<&str> = checkpoint.members.iter().map(|pin| pin.document_id.as_str()).collect();
        assert_eq!(pinned_ids, HashSet::from(["space-manifest", "collection-a", "collection-b", "artifact-a"]));
    }

    #[test]
    fn space_checkpoint_commits_dirty_members_and_pins_their_checkpoints() {
        let mut member_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "member-a", DemoSnapshot { n: 0 }, None));
        member_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply a");

        let mut member_b = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "member-b", DemoSnapshot { n: 0 }, None));
        member_b.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 5 }], description: None }).expect("apply b");
        member_b.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("b-init".into()), authors: Vec::new() }).expect("commit b upfront, so it starts clean");
        let member_b_checkpoint = member_b.current_checkpoint_id().expect("b checkpoint").to_string();

        let mut host = SpaceHost::new(create_document_envelope(&format!("{S_SPACE_HISTORY_SCHEMA}/v1"), "studio", SpaceHistorySnapshot::default(), None));
        host.register_member(Box::new(member_a));
        host.register_member(Box::new(member_b));

        let space_checkpoint_id = host.commit_space_checkpoint("studio init".into(), vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }]).expect("commit space checkpoint");

        let snapshot = host.meta_snapshot().expect("meta snapshot");
        assert_eq!(snapshot.checkpoints.len(), 1);
        let checkpoint = &snapshot.checkpoints[0];
        assert_eq!(checkpoint.id, space_checkpoint_id);
        assert_eq!(checkpoint.members.len(), 2, "pins one entry per registered member");
        let pin_b = checkpoint.members.iter().find(|pin| pin.document_id == "member-b").expect("pin b");
        assert_eq!(pin_b.checkpoint_id, member_b_checkpoint, "clean member reuses its existing checkpoint");
        assert!(!host.member("member-a").expect("member a").is_dirty(), "dirty member-a is committed (and therefore clean) by the space checkpoint");
    }

    #[test]
    fn space_vcs_host_meta_document_is_backbone_attachable_and_detachable() {
        let (backbone_a, backbone_b) = MemoryBackbone::pair("studio-a", "studio-b");
        let meta_envelope: ArtifactEnvelope<SpaceHistorySnapshot, SpaceHistoryMutation> = create_document_envelope(&format!("{S_SPACE_HISTORY_SCHEMA}/v1"), "studio", SpaceHistorySnapshot::default(), None);
        let mut host_a = SpaceHost::new(meta_envelope.clone());
        let mut host_b = SpaceHost::new(meta_envelope);
        assert!(host_a.backbone_ref().is_none(), "default is unattached, like any other ArtifactStore");

        host_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        host_b.attach_backbone(Box::new(backbone_b)).expect("attach b");
        assert!(host_a.backbone_ref().is_some());

        let mut member = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "member-a", DemoSnapshot { n: 0 }, None));
        member.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply on member, so it's dirty and can be committed");
        host_a.register_member(Box::new(member));
        host_a.commit_space_checkpoint("studio init".into(), Vec::new()).expect("commit space checkpoint on a");

        host_b.tick().expect("tick b");
        assert_eq!(host_b.meta_snapshot().expect("meta snapshot b").checkpoints.len(), 1, "the space-wide checkpoint replicates through the meta-document's backbone");

        host_a.detach_backbone();
        assert!(host_a.backbone_ref().is_none());
        host_a.commit_space_checkpoint("studio offline".into(), Vec::new()).expect("meta history keeps working purely in memory once detached");
        host_b.tick().expect("tick b again");
        assert_eq!(host_b.meta_snapshot().expect("meta snapshot b unchanged").checkpoints.len(), 1, "detached space edits never reach the peer");
    }

    #[test]
    fn space_checkout_checkpoint_fans_out_and_restores_pinned_member_state() {
        let member_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "member-a", DemoSnapshot { n: 0 }, None));
        let mut host = SpaceHost::new(create_document_envelope(&format!("{S_SPACE_HISTORY_SCHEMA}/v1"), "studio", SpaceHistorySnapshot::default(), None));
        host.register_member(Box::new(member_a));

        demo_member::<DemoMutation>(&mut host, "member-a").dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply 1");
        let space_checkpoint_1 = host.commit_space_checkpoint("first".into(), Vec::new()).expect("commit 1");

        demo_member::<DemoMutation>(&mut host, "member-a").dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).expect("apply 2");
        host.commit_space_checkpoint("second".into(), Vec::new()).expect("commit 2");
        assert_eq!(demo_member::<DemoMutation>(&mut host, "member-a").snapshot().expect("snapshot").n, 2, "member reflects the second space checkpoint before checking out the first");

        host.checkout_space_checkpoint(&space_checkpoint_1).expect("checkout space checkpoint 1");
        assert_eq!(demo_member::<DemoMutation>(&mut host, "member-a").snapshot().expect("snapshot").n, 1, "checking out the first space checkpoint fans out and restores member-a's pinned state");
    }

    #[test]
    fn space_switch_alternative_fans_out_and_restores_pinned_member_state() {
        let member_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "member-a", DemoSnapshot { n: 0 }, None));
        let mut host = SpaceHost::new(create_document_envelope(&format!("{S_SPACE_HISTORY_SCHEMA}/v1"), "studio", SpaceHistorySnapshot::default(), None));
        host.register_member(Box::new(member_a));

        demo_member::<DemoMutation>(&mut host, "member-a").dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply 1");
        host.commit_space_checkpoint("root".into(), Vec::new()).expect("commit root");

        let alt_id = host.create_space_alternative("branch-a".into()).expect("create alternative");

        demo_member::<DemoMutation>(&mut host, "member-a").dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).expect("apply 2 (uncommitted at the studio level)");
        assert_eq!(demo_member::<DemoMutation>(&mut host, "member-a").snapshot().expect("snapshot").n, 2, "uncommitted edit is live before switching");

        host.switch_space_alternative(&alt_id).expect("switch alternative fans out to its pinned checkpoint");
        assert_eq!(demo_member::<DemoMutation>(&mut host, "member-a").snapshot().expect("snapshot").n, 1, "switching alternatives restores each member to its pinned checkpoint, discarding the uncommitted edit");
    }

    #[test]
    fn space_undo_and_redo_target_the_member_with_the_most_recent_local_edit_by_hlt() {
        let mut member_early = ArtifactStore::new(create_document_envelope::<DemoSnapshot, TimestampedMutation>("demo-ts/v1", "member-early", DemoSnapshot { n: 0 }, None));
        member_early.dispatch(ArtifactCommand::Apply { mutations: vec![TimestampedMutation::SetN { n: 1, physical_ms: 1_000 }], description: None }).expect("apply early");

        let mut member_late = ArtifactStore::new(create_document_envelope::<DemoSnapshot, TimestampedMutation>("demo-ts/v1", "member-late", DemoSnapshot { n: 0 }, None));
        member_late.dispatch(ArtifactCommand::Apply { mutations: vec![TimestampedMutation::SetN { n: 9, physical_ms: 2_000 }], description: None }).expect("apply late");

        let mut host = SpaceHost::new(create_document_envelope(&format!("{S_SPACE_HISTORY_SCHEMA}/v1"), "studio", SpaceHistorySnapshot::default(), None));
        host.register_member(Box::new(member_early));
        host.register_member(Box::new(member_late));

        host.undo().expect("space undo targets the member with the higher HLT");
        assert_eq!(demo_member::<TimestampedMutation>(&mut host, "member-early").snapshot().expect("early snapshot").n, 1, "earlier local edit (lower HLT) is untouched");
        assert_eq!(demo_member::<TimestampedMutation>(&mut host, "member-late").snapshot().expect("late snapshot").n, 0, "later local edit (higher HLT) is the one undone");

        host.redo().expect("studio redo targets the most recently undone edit");
        assert_eq!(demo_member::<TimestampedMutation>(&mut host, "member-late").snapshot().expect("late snapshot after redo").n, 9, "redo restores the member's most recently undone edit");
    }

    #[test]
    fn default_reconcile_hook_is_a_no_op_for_existing_document_kinds() {
        let snapshot = DemoSnapshot { n: 4 };
        let (reconciled, conflicts) = DemoMutation::SetN { n: 4 }.reconcile(snapshot.clone());
        assert_eq!(reconciled, snapshot, "default reconcile leaves the snapshot untouched");
        assert!(conflicts.is_empty(), "default reconcile reports no conflicts");

        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 3 }], description: None }).expect("apply");
        let replayed = materialize_document_snapshot(store.envelope(), store.applied_edit_ids()).expect("replay");
        assert_eq!(replayed.n, 3, "materialize_document_snapshot is unaffected by the no-operation default reconcile hook");
        let (with_conflicts, conflicts) = store.snapshot_with_conflicts().expect("snapshot with conflicts");
        assert_eq!(with_conflicts.n, 3);
        assert!(conflicts.is_empty());
        assert!(store.conflicts().is_empty(), "no remote ingestion happened, so the store's conflict buffer stays empty");
    }

    #[test]
    fn space_history_op_round_trips() {
        let checkpoint = SpaceCheckpoint {
            id: "sc-1".into(),
            parent_id: None,
            message: "root".into(),
            authors: Vec::new(),
            timestamp: HybridLogicalTimestamp::new(0, 1),
            members: vec![SpaceMemberPin { document_id: "member-a".into(), checkpoint_id: "cp-1".into(), alternative_id: String::new() }],
        };
        test_support::assert_operation_round_trip(&SpaceHistorySnapshot::default(), SpaceHistoryMutation::CommitSpaceCheckpoint { checkpoint: checkpoint.clone() });

        let with_checkpoint = SpaceHistorySnapshot { checkpoints: vec![checkpoint], alternatives: Vec::new(), active_alternative_id: None };
        let alternative = SpaceAlternative { id: "sa-1".into(), name: "branch".into(), checkpoint_ids: vec!["sc-1".into()] };
        test_support::assert_operation_round_trip(&with_checkpoint, SpaceHistoryMutation::CreateSpaceAlternative { alternative });

        let with_alternative_active = SpaceHistorySnapshot { active_alternative_id: Some("sa-1".into()), ..with_checkpoint };
        test_support::assert_operation_round_trip(&with_alternative_active, SpaceHistoryMutation::SwitchSpaceAlternative { alternative_id: "sa-other".into() });
    }

    //#endregion 🏛️StudioTests

    //#region 🔖️TextFormatHelpers
    #[test]
    fn ops_author_conversion_drops_avatar_matching_the_ops_text_format() {
        let author = Author { id: "a1".into(), name: "Alice".into(), avatar: Some("http://example/a1.png".into()) };
        let round_tripped: Author = OpsAuthor::from(&author).into();
        assert_eq!(round_tripped, Author { id: "a1".into(), name: "Alice".into(), avatar: None }, "OpsAuthor never carries avatar — it is not part of the .ops text format");
    }

    #[test]
    fn ops_header_line_checkpoint_round_trips_including_delimiter_and_quote_characters_in_authors() {
        let header = OpsHeaderLine::Checkpoint {
            id: "c1".to_string(),
            at: "18".to_string(),
            changes: vec!["ch1".to_string(), "ch2".to_string()],
            parent: None,
            by: vec![OpsAuthor { id: "a:1,x".to_string(), name: "Alice, A. \"the great\"".to_string() }, OpsAuthor { id: "b2".to_string(), name: "Bob".to_string() }],
            message: Some("first \"checkpoint\"".to_string()),
        };
        let printed = header.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line: {printed:?}");
        assert!(!printed.contains("parent="), "an absent optional field must be omitted, not printed as a '-' placeholder: {printed}");
        let parsed = OpsHeaderLine::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed for {printed:?}: {e}"));
        assert_eq!(parsed, header, "OpsHeaderLine::Checkpoint round trip diverged for {printed:?}");
    }

    #[test]
    fn ops_header_line_edit_round_trips_including_a_quoted_description() {
        let header = OpsHeaderLine::Edit { id: "e1".to_string(), started: "1".to_string(), actor: None, finished: None, key: None, description: Some("hello \"world\"".to_string()) };
        let printed = header.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line: {printed:?}");
        assert!(!printed.contains("actor="), "an absent optional field must be omitted: {printed}");
        let parsed = OpsHeaderLine::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed for {printed:?}: {e}"));
        assert_eq!(parsed, header, "OpsHeaderLine::Edit round trip diverged for {printed:?}");
    }

    #[test]
    fn ops_header_line_cursor_round_trips_the_full_applied_and_redo_lists() {
        let header = OpsHeaderLine::Cursor { applied: vec!["e1".to_string(), "e3".to_string()], redo: vec!["e2".to_string()], checkpoint: Some("ck-1".to_string()) };
        let printed = header.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line: {printed:?}");
        let parsed = OpsHeaderLine::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed for {printed:?}: {e}"));
        assert_eq!(parsed, header, "OpsHeaderLine::Cursor round trip diverged for {printed:?}");
    }

    #[test]
    fn ops_header_line_parse_op_rejects_a_line_with_no_known_keyword() {
        let error = OpsHeaderLine::parse_op("not a structural line").unwrap_err();
        assert!(error.message.contains("unknown operation line"), "got {error:?}");
    }

    #[test]
    fn parse_document_text_rejects_a_header_line_missing_its_required_positional_id() {
        let files = ArtifactTextFiles { dsl: "n=0\n".to_string(), ops: "active\n".to_string() };
        let error = parse_document_text::<DemoSnapshot, DemoMutation>(&files.dsl, &files.ops).unwrap_err();
        assert!(error.message.contains("expected Text"), "got {error:?}");
        assert_eq!(error.span.line, 1);
    }

    #[test]
    fn parse_document_text_rejects_an_unknown_header_line_keyword() {
        let files = ArtifactTextFiles { dsl: "n=0\n".to_string(), ops: "doc demo schema=demo/v1\nbogus id=x\n".to_string() };
        let error = parse_document_text::<DemoSnapshot, DemoMutation>(&files.dsl, &files.ops).unwrap_err();
        assert!(error.message.contains("unknown operation line"), "got {error:?}");
        assert_eq!(error.span.line, 2);
    }

    #[test]
    fn document_text_round_trips_with_an_active_alternative_and_a_quoted_description() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: Some("said \"hi\" and used a \\ backslash".into()) }).expect("apply");
        store.dispatch(ArtifactCommand::CreateAlternative { name: "branch \"a\"".into() }).expect("create alternative (auto-commits and activates it)");
        assert!(store.envelope().active_alternative_id.is_some(), "precondition: an alternative is active");
        let files = print_document_text(store.envelope()).expect("print document text");
        assert!(files.ops.lines().any(|line| line.starts_with("active ")), "an active alternative must print an `active` header line: {}", files.ops);
        test_support::assert_document_text_round_trip(&store);
        test_support::assert_document_pack_round_trip(&store);
    }

    #[test]
    fn document_text_round_trips_a_cursor_after_undo_then_apply_interleaving() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply e1");
        store.dispatch(ArtifactCommand::Undo).expect("undo e1");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).expect("apply e2");
        // e1 (undone, in redo) precedes e2 (applied) in file order — exactly the interleaving a
        // single tail-edit marker cannot represent (see HistoryCursor's doc).
        assert_eq!(store.applied_edit_ids().len(), 1, "only e2 is applied");
        let files = print_document_text(store.envelope()).expect("print document text");
        assert!(files.ops.lines().any(|line| line.starts_with("cursor ")), "a synced cursor must print a `cursor` header line: {}", files.ops);
        let parsed = parse_document_text::<DemoSnapshot, DemoMutation>(&files.dsl, &files.ops).unwrap_or_else(|error| panic!("parse document text failed: {error}"));
        assert_eq!(parsed.envelope.cursor, store.envelope().cursor.clone(), "cursor diverged across a print/parse round trip");
        assert_eq!(parsed.snapshot.n, 2, "restored snapshot must reflect only the applied edit (e2), not both");
    }

    /// @emoji 🔐️ The save→load→undo proof (contract's runtime-behavior requirement): a store's
    /// undo/redo position survives a full pack+spr save/load cycle, not just its snapshot value.
    #[test]
    fn save_load_undo_proof_pack_spr_round_trip_preserves_undo_redo_position() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply e1");
        let post_e1 = store.snapshot().expect("post-e1 snapshot");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).expect("apply e2");
        let post_e2 = store.snapshot().expect("post-e2 snapshot");
        store.dispatch(ArtifactCommand::Undo).expect("undo e2");
        assert_eq!(store.snapshot().expect("live snapshot"), post_e1, "precondition: live store is back at post-e1");
        test_support::assert_live_equals_replay(&store);

        // Save: print_document_pack persists pack (initial snapshot) + spr (real inverse/meta,
        // AND the cursor reflecting exactly "e1 applied, e2 in redo").
        let pack_files = print_document_pack(store.envelope()).expect("print document pack");
        assert!(!pack_files.spr.is_empty(), "spr bytes must be non-empty once an edit exists");

        // Load: a FRESH store built only from persisted bytes — no access to the original `store`.
        let parsed: ParsedDocumentText<DemoSnapshot, DemoMutation> = parse_document_pack(&pack_files.pack, &pack_files.spr).unwrap_or_else(|error| panic!("parse document pack failed: {error}"));
        assert_eq!(parsed.snapshot, post_e1, "loaded snapshot must equal post-e1, proving undo position survived the save");
        let mut reloaded = ArtifactStore::new(parsed.envelope);
        assert_eq!(reloaded.snapshot().expect("reloaded snapshot"), post_e1, "ArtifactStore::new must seed live state from the persisted cursor");
        assert_eq!(reloaded.applied_edit_ids(), store.applied_edit_ids(), "applied_edit_ids must survive the round trip");
        test_support::assert_live_equals_replay(&reloaded);

        // Redo restores e2 — proving the redo stack (not just applied_edit_ids) survived.
        reloaded.dispatch(ArtifactCommand::Redo).expect("redo e2 after reload");
        assert_eq!(reloaded.snapshot().expect("post-redo snapshot"), post_e2);
        test_support::assert_live_equals_replay(&reloaded);

        // Undo twice from here reaches the true initial state.
        reloaded.dispatch(ArtifactCommand::Undo).expect("undo e2 again");
        reloaded.dispatch(ArtifactCommand::Undo).expect("undo e1");
        assert_eq!(reloaded.snapshot().expect("final snapshot"), DemoSnapshot { n: 0 });
        test_support::assert_live_equals_replay(&reloaded);
    }

    //#endregion 🔖️TextFormatHelpers

    //#region 🔖️CommandErrorPaths
    #[test]
    fn apply_with_no_mutations_is_rejected() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        let error = store.dispatch(ArtifactCommand::Apply { mutations: Vec::new(), description: None }).unwrap_err();
        assert_eq!(error, VcsError::EmptyApply);
    }

    #[test]
    fn amend_last_with_no_mutations_is_rejected() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        let error = store.dispatch(ArtifactCommand::AmendLast { mutations: Vec::new(), coalesce_key: None }).unwrap_err();
        assert_eq!(error, VcsError::EmptyApply);
    }

    #[test]
    fn undo_with_nothing_applied_is_rejected() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        assert_eq!(store.dispatch(ArtifactCommand::Undo).unwrap_err(), VcsError::NothingToUndo);
    }

    #[test]
    fn redo_with_nothing_undone_is_rejected() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        assert_eq!(store.dispatch(ArtifactCommand::Redo).unwrap_err(), VcsError::NothingToRedo);
    }

    #[test]
    fn checkout_of_an_unknown_checkpoint_is_rejected() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        let error = store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: "nope".into() }).unwrap_err();
        assert_eq!(error, VcsError::UnknownChange("nope".into()));
    }

    #[test]
    fn switch_to_an_unknown_alternative_is_rejected() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        let error = store.dispatch(ArtifactCommand::SwitchAlternative { alternative_id: "nope".into() }).unwrap_err();
        assert_eq!(error, VcsError::UnknownAlternative("nope".into()));
    }

    #[test]
    fn switch_to_an_alternative_whose_pinned_checkpoint_is_missing_is_rejected() {
        let mut envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        envelope.vcs.alternatives.push(Alternative { id: "alt-dangling".into(), name: "dangling".into(), checkpoint_ids: vec!["checkpoint-that-was-never-recorded".into()] });
        let mut store = ArtifactStore::new(envelope);
        let error = store.dispatch(ArtifactCommand::SwitchAlternative { alternative_id: "alt-dangling".into() }).unwrap_err();
        assert_eq!(error, VcsError::NoCheckpoint, "the alternative's pinned checkpoint id must actually exist");
    }

    #[test]
    fn create_alternative_with_no_edits_and_no_checkpoints_is_rejected() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        let error = store.dispatch(ArtifactCommand::CreateAlternative { name: "x".into() }).unwrap_err();
        assert_eq!(error, VcsError::NoCheckpoint, "the auto-commit has nothing pending, so there is still no checkpoint to branch from");
    }

    #[test]
    fn compensating_undo_without_a_semantic_command_is_rejected() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply");
        let error = store.dispatch(ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::CompensatingAction, semantic_command: None }).unwrap_err();
        assert!(matches!(error, VcsError::Backbone(_)), "got {error:?}");
    }

    #[test]
    fn materialize_document_snapshot_rejects_an_unknown_edit_id() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let error = materialize_document_snapshot(&envelope, &["missing-edit".to_string()]).unwrap_err();
        assert_eq!(error, VcsError::UnknownEdit("missing-edit".into()));
    }

    #[test]
    fn dispatch_text_applies_a_command_block_and_snapshot_json_reflects_it() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        let command_text = print_command(&ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 7 }], description: None }).expect("print command");
        store.dispatch_text(&command_text).expect("dispatch text");
        assert_eq!(store.snapshot_json().expect("snapshot json"), serde_json::to_string(&DemoSnapshot { n: 7 }).unwrap());

        let error = store.dispatch_text("not a command").unwrap_err();
        assert!(matches!(error, VcsError::Deserialize(_)), "got {error:?}");
    }

    #[test]
    fn dispatch_binary_applies_an_encoded_command_and_rejects_wrong_format() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        let command_bytes = ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 7 }], description: None }.encode_op().expect("encode command");
        store.dispatch_binary(&command_bytes).expect("dispatch binary");
        assert_eq!(store.snapshot_json().expect("snapshot json"), serde_json::to_string(&DemoSnapshot { n: 7 }).unwrap());

        let mut wrong_format = command_bytes.clone();
        wrong_format[0] = 9;
        let error = store.dispatch_binary(&wrong_format).unwrap_err();
        assert!(matches!(error, VcsError::Deserialize(_)), "got {error:?}");
    }

    #[test]
    fn command_text_binary_equivalence_holds_for_every_document_command_variant() {
        let commands: Vec<ArtifactCommand<DemoMutation>> = vec![
            ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 7 }], description: Some("set n".to_string()) },
            ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 7 }], description: None },
            ArtifactCommand::Undo,
            ArtifactCommand::Redo,
            ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::ExactBaseOnly, semantic_command: None },
            ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::TransformAgainstConcurrent, semantic_command: None },
            ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::CompensatingAction, semantic_command: Some(Box::new(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 0 }], description: None })) },
            ArtifactCommand::CommitCheckpoint { message: Some("checkpoint".to_string()), authors: vec![Author { id: "u1".to_string(), name: "Ueli Saluz".to_string(), avatar: None }] },
            ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() },
            ArtifactCommand::CreateAlternative { name: "branch".to_string() },
            ArtifactCommand::SwitchAlternative { alternative_id: "alt-1".to_string() },
            ArtifactCommand::CheckoutCheckpoint { checkpoint_id: "ck-1".to_string() },
            ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN { n: 3 }], coalesce_key: Some("drag".to_string()) },
        ];
        for command in &commands {
            test_support::assert_command_text_binary_equivalence(command);
        }
    }

    //#endregion 🔖️CommandErrorPaths

    //#region 🔖️ReconcileAlternative
    #[test]
    fn reconcile_alternative_requires_an_existing_checkpoint() {
        let mut envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let error = reconcile_alternative(&mut envelope, "reconciled", None, Vec::new()).unwrap_err();
        assert_eq!(error, VcsError::NoCheckpoint);
    }

    #[test]
    fn reconcile_alternative_pins_the_latest_checkpoint_and_optionally_records_a_reconciliation_checkpoint() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c1".into()), authors: Vec::new() }).expect("commit");
        let base_checkpoint_id = store.envelope().vcs.checkpoints[0].id.clone();

        let mut without_message = store.envelope().clone();
        let alt_id = reconcile_alternative(&mut without_message, "no-record", None, Vec::new()).expect("reconcile without message");
        assert_eq!(without_message.vcs.alternatives.last().unwrap().checkpoint_ids, vec![base_checkpoint_id.clone()]);
        assert_eq!(without_message.vcs.checkpoints.len(), 1, "no checkpoint_message means no new checkpoint is recorded");
        assert!(!alt_id.is_empty());

        let mut with_message = store.envelope().clone();
        let authors = vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }];
        reconcile_alternative(&mut with_message, "recorded", Some("merged concurrent work".into()), authors.clone()).expect("reconcile with message");
        assert_eq!(with_message.vcs.checkpoints.len(), 2, "a checkpoint_message appends one reconciliation checkpoint");
        let recorded_checkpoint = with_message.vcs.checkpoints.last().unwrap();
        assert_eq!(recorded_checkpoint.parent_id, Some(base_checkpoint_id));
        assert_eq!(recorded_checkpoint.authors, authors);
        assert_eq!(recorded_checkpoint.message, Some("reconciled".into()), "the reconciliation checkpoint's own message is fixed, distinct from the change description");
        assert_eq!(with_message.vcs.changes.last().unwrap().description, Some("merged concurrent work".into()), "the passed checkpoint_message becomes the change's description");
    }

    #[test]
    fn commit_checkpoint_mints_distinct_content_addressed_ids_for_distinct_commits() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply 1");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("first".into()), authors: Vec::new() }).expect("commit 1");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).expect("apply 2");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("second".into()), authors: Vec::new() }).expect("commit 2");

        let ids: Vec<&str> = store.envelope().vcs.checkpoints.iter().map(|checkpoint| checkpoint.id.as_str()).collect();
        assert_eq!(ids.len(), 2);
        assert_ne!(ids[0], ids[1], "two distinct commits must mint two distinct checkpoint ids");
        assert!(ids.iter().all(|id| id.starts_with("ck-")));
    }

    #[test]
    fn merge_base_finds_the_nearest_common_ancestor_across_a_fork() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply root");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("root".into()), authors: Vec::new() }).expect("commit root");
        let root_id = store.envelope().vcs.checkpoints[0].id.clone();

        store.dispatch(ArtifactCommand::CreateAlternative { name: "feature-a".into() }).expect("create feature-a");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).expect("apply a");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("a1".into()), authors: Vec::new() }).expect("commit a1");
        let a1_id = store.envelope().vcs.checkpoints.last().unwrap().id.clone();

        store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: root_id.clone() }).expect("checkout root");
        store.dispatch(ArtifactCommand::CreateAlternative { name: "feature-b".into() }).expect("create feature-b");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 3 }], description: None }).expect("apply b");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("b1".into()), authors: Vec::new() }).expect("commit b1");
        let b1_id = store.envelope().vcs.checkpoints.last().unwrap().id.clone();

        assert_eq!(merge_base(store.envelope(), &a1_id, &b1_id), Some(root_id.clone()), "a1 and b1 forked at root");
        assert_eq!(merge_base(store.envelope(), &a1_id, &root_id), Some(root_id.clone()), "root is its own descendant's merge-base");
        assert_eq!(merge_base(store.envelope(), &root_id, &root_id), Some(root_id), "a checkpoint is its own merge-base");
    }

    #[test]
    fn merge_base_is_none_for_a_dangling_unknown_checkpoint_id() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("root".into()), authors: Vec::new() }).expect("commit");
        let root_id = store.envelope().vcs.checkpoints[0].id.clone();

        assert_eq!(merge_base(store.envelope(), &root_id, "unknown-checkpoint"), None, "an id absent from the checkpoint list shares no ancestry with anything");
    }

    //#endregion 🔖️ContentAddressedCheckpointAndMergeBase

    //#region 🔖️RemoteSnapshotMerge
    #[test]
    fn snapshot_merge_into_a_nonempty_store_adds_only_the_new_remote_edits_and_records() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("local apply");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("local".into()), authors: Vec::new() }).expect("local commit");

        let mut remote_store = ArtifactStore::new(store.envelope().clone());
        remote_store.reset(store.envelope().clone(), store.applied_edit_ids().to_vec(), Vec::new()).expect("reset remote");
        remote_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).expect("remote apply");
        remote_store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("remote".into()), authors: Vec::new() }).expect("remote commit");

        let (channel, remote_end) = ChannelBackbone::pair("chan");
        store.attach_backbone(Box::new(channel)).expect("attach");
        let _ = remote_end.drain().expect("drain attach snapshot");
        let remote_files = remote_store.snapshot_pack().expect("remote snapshot");
        remote_end.push(BackboneMessage::Snapshot { pack: remote_files.pack, spr: remote_files.spr }).expect("push snapshot");
        store.tick().expect("tick merges the pushed snapshot");

        assert_eq!(store.envelope().vcs.edits.len(), 2, "the shared original edit is deduped, only the new remote edit is added");
        assert_eq!(store.envelope().vcs.checkpoints.len(), 2, "the remote's new checkpoint is merged in by id");
        assert_eq!(store.snapshot().expect("snapshot").n, 2, "current folds in the newly merged edit's forwards");
    }

    //#endregion 🔖️RemoteSnapshotMerge

    //#region 🔖️SpaceMemberCheckoutRouting
    #[test]
    fn space_member_checkout_switches_at_the_alternative_tip_and_falls_back_to_checkout_when_stale() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply");
        store.dispatch(ArtifactCommand::CreateAlternative { name: "feature".into() }).expect("create alternative (auto-commits since no checkpoint existed yet)");
        let alt_id = store.envelope().vcs.alternatives[0].id.clone();
        let tip = store.envelope().vcs.alternatives[0].checkpoint_ids.last().expect("alt has a tip").clone();

        SpaceMember::checkout(&mut store, &tip, &alt_id).expect("checkout at the tip routes through SwitchAlternative");
        assert_eq!(store.envelope().active_alternative_id, Some(alt_id.clone()), "switching to the tip keeps it active");

        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).expect("apply on branch");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c2".into()), authors: Vec::new() }).expect("commit c2, advancing the alt's tip past `tip`");

        SpaceMember::checkout(&mut store, &tip, &alt_id).expect("checkout of the now-stale tip falls back to CheckoutCheckpoint");
        assert_eq!(store.snapshot().expect("snapshot").n, 1, "restored the old checkpoint's state");
        assert_eq!(store.envelope().active_alternative_id, None, "the checked-out checkpoint is no longer any alternative's tip, so nothing is active");
    }

    //#endregion 🔖️SpaceMemberCheckoutRouting

    //#region 🔖️BackbonePorts
    #[test]
    fn memory_backbone_port_round_trips_and_reports_a_missing_file() {
        let port = MemoryBackbonePort::new();
        let error = port.read("file://nowhere").unwrap_err();
        assert!(matches!(error, VcsError::Backbone(_)), "got {error:?}");
        port.write("file://a", "payload-1").expect("write");
        assert_eq!(port.read("file://a").expect("read"), "payload-1");
        port.write("file://a", "payload-2").expect("overwrite");
        assert_eq!(port.read("file://a").expect("read after overwrite"), "payload-2", "write is an upsert");
    }

    #[test]
    fn local_storage_backbone_port_falls_back_to_its_in_memory_store() {
        let port = LocalStorageBackbonePort::new();
        let error = port.read("local://missing").unwrap_err();
        assert!(matches!(error, VcsError::Backbone(_)), "got {error:?}");
        port.write("local://a", "value").expect("write falls back to the in-memory store");
        assert_eq!(port.read("local://a").expect("read falls back too"), "value");

        let defaulted = LocalStorageBackbonePort::default();
        assert!(defaulted.read("local://a").is_err(), "Default constructs its own independent fallback store");
    }

    //#endregion 🔖️BackbonePorts

    //#region 🔖️PackValueFixtures
    fn pack_value_fixture_corpus() -> Vec<(&'static str, DslValue)> {
        vec![
            ("null", DslValue::Null),
            ("bool_true", DslValue::Bool(true)),
            ("bool_false", DslValue::Bool(false)),
            ("int_zero", DslValue::Number(0.0)),
            ("int_negative_one", DslValue::Number(-1.0)),
            ("float_pi", DslValue::Number(3.14)),
            ("float_whole_number", DslValue::Number(2.0)),
            ("string_empty", DslValue::String(String::new())),
            ("string_escapes", DslValue::String("hello\nworld with \"quotes\"".into())),
            ("array_empty", DslValue::Array(vec![])),
            ("array_ints", DslValue::Array(vec![DslValue::Number(1.0), DslValue::Number(2.0), DslValue::Number(3.0)])),
            ("object_empty", DslValue::Object(vec![])),
            ("object_mixed", DslValue::object([("a".into(), DslValue::Number(1.0)), ("b".into(), DslValue::Array(vec![DslValue::Bool(true), DslValue::Null]))])),
            (
                "nested_deep",
                DslValue::object([(
                    "a".into(),
                    DslValue::object([("b".into(), DslValue::object([("c".into(), DslValue::Array(vec![DslValue::Number(1.0), DslValue::Number(2.0), DslValue::object([("d".into(), DslValue::String("leaf".into()))])]))]))]),
                )]),
            ),
        ]
    }

    fn dsl_value_numeric_insensitive_eq(a: &DslValue, b: &DslValue) -> bool {
        match (a, b) {
            (DslValue::Number(x), DslValue::Number(y)) => x == y,
            (DslValue::Array(x), DslValue::Array(y)) => x.len() == y.len() && x.iter().zip(y).all(|(a, b)| dsl_value_numeric_insensitive_eq(a, b)),
            (DslValue::Object(x), DslValue::Object(y)) => x.len() == y.len() && x.iter().all(|(k, v)| y.iter().find(|(ok, _)| ok == k).is_some_and(|(_, ov)| dsl_value_numeric_insensitive_eq(v, ov))),
            _ => a == b,
        }
    }

    /// @emoji 🧾️ Hex-dumps `pack_rt::encode_pack_value` over a representative `DslValue`
    /// corpus — ground truth for `HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS`'s TS
    /// `PackValueCodec` mirror (`framework/product/os/ts/index.ts`). Run with `--nocapture` to
    /// capture the printed `name -> hex` lines; also asserts `decode_pack_value(encode_pack_value(v))
    /// == v` for every entry so the corpus is never accidentally out of date with the real codec.
    #[test]
    fn pack_value_fixture_corpus_hex_dump() {
        for (name, value) in pack_value_fixture_corpus() {
            let bytes = pack_rt::encode_pack_value(&value);
            let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
            println!("[pack_value_fixture] {name} ({} bytes) -> {hex}", bytes.len());
            let decoded = pack_rt::decode_pack_value(&bytes).expect("decode_pack_value");
            assert!(dsl_value_numeric_insensitive_eq(&decoded, &value), "round-trip mismatch for fixture {name}: {decoded:?} != {value:?}");
        }
    }

    /// @emoji 🪶️ Hex-dumps `pack_rt::encode_wire_value` over the SAME fixture corpus — ground
    /// truth for the container-less wire codec mirror in TS.
    #[test]
    fn pack_wire_value_fixture_corpus_hex_dump() {
        for (name, value) in pack_value_fixture_corpus() {
            let bytes = pack_rt::encode_wire_value(&value);
            let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
            println!("[pack_wire_value_fixture] {name} ({} bytes) -> {hex}", bytes.len());
            let decoded = pack_rt::decode_wire_value(&bytes).expect("decode_wire_value");
            assert!(dsl_value_numeric_insensitive_eq(&decoded, &value), "round-trip mismatch for fixture {name}: {decoded:?} != {value:?}");
        }
    }
    //#endregion 🔖️PackValueFixtures

    //#region 🔖️CompositionTests
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, crate::os_dsl::DslOps)]
    #[serde(tag = "operation")]
    enum ValidatedMutation {
        #[dsl(key = "set-n-validated")]
        SetN { n: i32 },
    }

    impl OpText for ValidatedMutation {
        fn parse_op(line: &str) -> Result<Self, TextError> {
            let variants = <Self as crate::os_dsl::DslVariants>::variants();
            for (keyword, spec_fn) in &variants {
                let probe = format!("{} ", keyword);
                if line == keyword.as_str() || line.starts_with(&probe) {
                    let record = crate::os_dsl::parse(line, &spec_fn(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline })?;
                    return <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record);
                }
            }
            Err(crate::os_dsl::__rt::field_error(format!("unknown operation line '{line}'")))
        }
        fn print_op(&self) -> String {
            let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
            let variants = <Self as crate::os_dsl::DslVariants>::variants();
            let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
            crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)
        }
    }

    impl OpBinary for ValidatedMutation {
        fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
            const OP_BINARY_FORMAT: u8 = 1;
            let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
            let variants = <Self as crate::os_dsl::DslVariants>::variants();
            let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
            let spec = (variants[ordinal].1)();
            let body = crate::os_pack::encode_record_body(&spec, &record, &PackEncodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
            let mut out = Vec::with_capacity(body.len() + 3);
            out.push(OP_BINARY_FORMAT);
            crate::os_pack::write_varint_u64(&mut out, ordinal as u64);
            out.extend_from_slice(&body);
            Ok(out)
        }
        fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
            const OP_BINARY_FORMAT: u8 = 1;
            let mut reader = crate::os_pack::ByteReader::new(bytes);
            let format = reader.read_u8()?;
            if format != OP_BINARY_FORMAT {
                return Err(crate::os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
            }
            let ordinal = reader.read_varint_u64()?;
            let variants = <Self as crate::os_dsl::DslVariants>::variants();
            let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
            let spec = spec_fn();
            let body = &bytes[reader.position()..];
            let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
            <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })
        }
    }

    /// @emoji 🛂️ `Mutation::validate` override the atomicity/compensation tests below trigger
    /// deterministically (`n < 0` fails) — `DemoMutation` always accepts, so it cannot exercise
    /// `SpaceMember::validate_wire`'s failure path.
    impl Mutation<DemoSnapshot> for ValidatedMutation {
        type Diff = DemoDiff;
        fn diff(&self, _snapshot: &DemoSnapshot) -> DemoDiff {
            match self {
                ValidatedMutation::SetN { n } => DemoDiff { n: Some(*n) },
            }
        }
        fn inverse(&self, snapshot: &DemoSnapshot) -> Vec<Self> {
            vec![ValidatedMutation::SetN { n: snapshot.n }]
        }
        fn validate(&self, _snapshot: &DemoSnapshot) -> Result<(), String> {
            match self {
                ValidatedMutation::SetN { n } if *n < 0 => Err(format!("n must be >= 0, got {n}")),
                ValidatedMutation::SetN { .. } => Ok(()),
            }
        }
    }

    /// @emoji 🏭️ Minimal `ChildStoreFactory` fixture: `create` seeds a `DemoSnapshot` store
    /// (decoding `initial_pack` when non-empty, defaulting to `n: 0` otherwise); `open` is not
    /// exercised by these tests, so it stays a stub.
    struct DemoChildFactory;
    impl ChildStoreFactory for DemoChildFactory {
        fn create(&self, id: &str, _dialect: &crate::os_io::ArtifactDialect, initial_pack: &[u8]) -> Result<Box<dyn SpaceMember>, VcsError> {
            let initial = if initial_pack.is_empty() { DemoSnapshot { n: 0 } } else { DemoSnapshot::decode_pack(initial_pack).map_err(|error| VcsError::Deserialize(error.to_string()))? };
            let envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", id, initial, None);
            Ok(Box::new(ArtifactStore::new(envelope)))
        }
        fn open(&self, _envelope_pack: &[u8]) -> Result<Box<dyn SpaceMember>, VcsError> {
            Err(VcsError::Deserialize("DemoChildFactory::open is not exercised by these fixtures".into()))
        }
    }

    #[test]
    fn artifact_child_dsl_field_round_trips_via_pack_and_value() {
        let target = crate::os_io::ArtifactRef { artifact_id: "child-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.mesh".into(), standard: "87a".into(), subset: "mesh".into() } };
        let child: ArtifactChild<DemoSnapshot> = ArtifactChild::new("child-1".into(), target);

        let spec = artifact_child_spec();
        let record = artifact_child_to_record(&child);
        let bytes = crate::os_pack::encode_record_body(&spec, &record, &PackEncodeOptions::default()).expect("encode");
        let (decoded_record, _report) = crate::os_pack::decode_record_body(&bytes, &spec, &PackDecodeOptions::default()).expect("decode");
        let decoded: ArtifactChild<DemoSnapshot> = artifact_child_from_record(&decoded_record).expect("from_record");
        assert_eq!(decoded, child);

        let value = <ArtifactChild<DemoSnapshot> as crate::os_dsl::DslField>::to_value(&child);
        let via_field = <ArtifactChild<DemoSnapshot> as crate::os_dsl::DslField>::from_value(&value).expect("from_value");
        assert_eq!(via_field, child);

        assert_eq!(child.to_child_ref("mesh-slot"), ChildRef { slot: "mesh-slot".into(), child_id: "child-1".into(), target: child.target.clone() });
    }

    #[test]
    fn owner_ref_dsl_field_round_trips_via_pack() {
        let owner = OwnerRef {
            parent: crate::os_io::ArtifactRef { artifact_id: "parent-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.object".into(), standard: "1".into(), subset: "*".into() } },
            slot: "mesh-slot".into(),
            child_id: "child-1".into(),
        };
        let spec = owner_ref_spec();
        let record = owner_ref_to_record(&owner);
        let bytes = crate::os_pack::encode_record_body(&spec, &record, &PackEncodeOptions::default()).expect("encode");
        let (decoded_record, _report) = crate::os_pack::decode_record_body(&bytes, &spec, &PackDecodeOptions::default()).expect("decode");
        let decoded = owner_ref_from_record(&decoded_record).expect("from_record");
        assert_eq!(decoded, owner);
    }

    #[test]
    fn artifact_link_dsl_field_round_trips_every_link_pin_variant() {
        let target = crate::os_io::ArtifactRef { artifact_id: "linked-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.image".into(), standard: "1".into(), subset: "*".into() } };
        let pins = vec![LinkPin::Head, LinkPin::Checkpoint { id: "ck-abc123".into() }, LinkPin::Snapshot { blob: BlobRef { hash: "deadbeef".into(), size: 42, media_type: "image/png".into() } }];
        for pin in pins {
            let link = ArtifactLink { target: target.clone(), pin: pin.clone(), role: "cover-image".into() };
            let spec = artifact_link_spec();
            let record = artifact_link_to_record(&link);
            let bytes = crate::os_pack::encode_record_body(&spec, &record, &PackEncodeOptions::default()).expect("encode");
            let (decoded_record, _report) = crate::os_pack::decode_record_body(&bytes, &spec, &PackDecodeOptions::default()).expect("decode");
            let decoded = artifact_link_from_record(&decoded_record).expect("from_record");
            assert_eq!(decoded, link, "round trip diverged for pin variant {pin:?}");
        }
    }

    #[test]
    fn artifact_refs_defaults_to_empty_for_a_leaf_snapshot() {
        struct LeafSnapshot;
        impl ArtifactRefs for LeafSnapshot {}
        let snapshot = LeafSnapshot;
        assert!(snapshot.child_refs().is_empty());
        assert!(snapshot.links().is_empty());
    }

    #[test]
    fn link_resolver_reports_resolved_missing_and_pinned_only_states() {
        struct DemoResolver;
        impl LinkResolver for DemoResolver {
            fn resolve(&self, link: &ArtifactLink) -> LinkState {
                match link.role.as_str() {
                    "known" => LinkState::Resolved { pack_bytes: vec![1, 2, 3], dialect: link.target.dialect.clone() },
                    "pinned" => LinkState::PinnedOnly { blob: BlobRef { hash: "deadbeef".into(), size: 3, media_type: "application/octet-stream".into() } },
                    _ => LinkState::Missing,
                }
            }
        }
        let target = crate::os_io::ArtifactRef { artifact_id: "linked-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.image".into(), standard: "1".into(), subset: "*".into() } };
        let resolver = DemoResolver;
        assert!(matches!(resolver.resolve(&ArtifactLink { target: target.clone(), pin: LinkPin::Head, role: "known".into() }), LinkState::Resolved { .. }));
        assert!(matches!(resolver.resolve(&ArtifactLink { target: target.clone(), pin: LinkPin::Head, role: "pinned".into() }), LinkState::PinnedOnly { .. }));
        assert!(matches!(resolver.resolve(&ArtifactLink { target, pin: LinkPin::Head, role: "gone".into() }), LinkState::Missing));
    }

    #[test]
    fn composition_graph_owns_forest_rejects_second_owner_and_cycle() {
        let mut graph = CompositionGraph::new();
        graph.insert_owns("parent-a", "slot-1", "child-1").expect("first owner ok");
        graph.insert_owns("parent-a", "slot-1", "child-1").expect("idempotent re-own ok");

        let error = graph.insert_owns("parent-b", "slot-1", "child-1").expect_err("second owner must be rejected");
        assert!(error.contains("already owned"), "unexpected message: {error}");

        graph.insert_owns("child-1", "slot-x", "grandchild-1").expect("child owns grandchild ok");
        assert!(graph.would_cycle_owns("grandchild-1", "parent-a"), "parent-a is an ancestor of grandchild-1 via child-1");
        let cycle_error = graph.insert_owns("grandchild-1", "slot-y", "parent-a").expect_err("cycle must be rejected");
        assert!(cycle_error.contains("cycle"), "unexpected message: {cycle_error}");

        assert_eq!(graph.owner_of("child-1"), Some("parent-a"));
        assert_eq!(graph.slot_of("child-1"), Some("slot-1"));
        graph.remove_owns("child-1");
        assert_eq!(graph.owner_of("child-1"), None);
    }

    #[test]
    fn composition_graph_links_reject_cycle_but_allow_converging_dag_edges() {
        let mut graph = CompositionGraph::new();
        graph.insert_link("a", "b").expect("a->b ok");
        graph.insert_link("b", "c").expect("b->c ok");
        graph.insert_link("a", "c").expect("a->c ok, a converging (non-cyclic) edge");

        let error = graph.insert_link("c", "a").expect_err("closing the loop must be rejected");
        assert!(error.contains("cycle"), "unexpected message: {error}");

        assert_eq!(HashSet::<String>::from_iter(graph.links_from("a")), HashSet::from(["b".to_string(), "c".to_string()]));
        graph.remove_link("a", "b");
        assert_eq!(graph.links_from("a"), vec!["c".to_string()]);
    }

    #[test]
    fn mint_child_id_converges_across_two_replicas_and_varies_by_ordinal_and_slot() {
        let parent_id = "parent-1";
        let slot = "mesh-slot";
        let ops: Vec<Vec<u8>> = vec![DemoMutation::SetN { n: 7 }.encode_op().expect("encode")];
        let fingerprint_replica_1 = concat_ops_fingerprint(&ops);
        let fingerprint_replica_2 = concat_ops_fingerprint(&ops);
        assert_eq!(fingerprint_replica_1, fingerprint_replica_2, "identical ops must fingerprint identically");

        let id_replica_1 = mint_child_id(parent_id, slot, &fingerprint_replica_1, 0);
        let id_replica_2 = mint_child_id(parent_id, slot, &fingerprint_replica_2, 0);
        assert_eq!(id_replica_1, id_replica_2, "two replicas performing the identical genesis must converge on the identical child id");

        let id_different_ordinal = mint_child_id(parent_id, slot, &fingerprint_replica_1, 1);
        assert_ne!(id_replica_1, id_different_ordinal, "a different ordinal must mint a different id");

        let id_different_slot = mint_child_id(parent_id, "other-slot", &fingerprint_replica_1, 0);
        assert_ne!(id_replica_1, id_different_slot, "a different slot must mint a different id");
    }

    /// @emoji 🧪️ TASK 2's "validate-all atomicity" law: one bad op anywhere ⇒ nothing applied on
    /// ANY member, parent included.
    #[test]
    fn dispatch_group_validate_all_atomicity_one_bad_member_applies_nothing() {
        let parent_ref = crate::os_io::ArtifactRef { artifact_id: "parent-atomic-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
        let child_ref = crate::os_io::ArtifactRef { artifact_id: "child-atomic-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

        let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, ValidatedMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, ValidatedMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: 0 }, None));

        let mut coordinator = CompositionCoordinator::new();
        coordinator.graph_mut().insert_owns(&parent_ref.artifact_id, "slot-1", &child_ref.artifact_id).expect("seed ownership");

        let good_op = ValidatedMutation::SetN { n: 5 }.encode_op().expect("encode good op");
        let bad_op = ValidatedMutation::SetN { n: -1 }.encode_op().expect("encode bad op");
        let parent_ops = vec![good_op];
        let child_dispatch = ChildDispatch { child: child_ref.clone(), ops: vec![bad_op], op_schema: SchemaId("demo/v1".into()), labels: vec!["bad".into()] };
        let mut children: [(&mut dyn SpaceMember, ChildDispatch); 1] = [(&mut child_store as &mut dyn SpaceMember, child_dispatch)];

        let result = coordinator.dispatch_group(&parent_ref, &mut parent_store as &mut dyn SpaceMember, &mut children, parent_ops, Vec::new(), GroupMeta::default());
        match result {
            Ok(_) => panic!("expected the group dispatch to fail phase-1 validation, but it succeeded"),
            Err(VcsError::ValidationFailed(_)) => {}
            Err(other) => panic!("expected ValidationFailed, got a different VcsError: {other}"),
        }
        assert!(parent_store.envelope().vcs.edits.is_empty(), "parent must have zero edits after a failed group dispatch");
        assert!(child_store.envelope().vcs.edits.is_empty(), "child must have zero edits after a failed group dispatch");
    }

    /// @emoji 🧪️ TASK 2's ownership-check law: `dispatch_group` refuses to touch a `ChildDispatch`
    /// whose claimed parent the coordinator's own `CompositionGraph` does not currently track —
    /// zero side effects, same as any other phase-1 failure.
    #[test]
    fn dispatch_group_rejects_a_child_the_graph_does_not_track_as_owned() {
        let parent_ref = crate::os_io::ArtifactRef { artifact_id: "parent-unowned-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
        let child_ref = crate::os_io::ArtifactRef { artifact_id: "child-unowned-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

        let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, ValidatedMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, ValidatedMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        let mut coordinator = CompositionCoordinator::new();
        // Deliberately NOT seeding `coordinator.graph_mut().insert_owns(..)` — the graph has no
        // record that `parent_ref` owns `child_ref`.

        let op = ValidatedMutation::SetN { n: 1 }.encode_op().expect("encode");
        let child_dispatch = ChildDispatch { child: child_ref.clone(), ops: vec![op], op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
        let mut children: [(&mut dyn SpaceMember, ChildDispatch); 1] = [(&mut child_store as &mut dyn SpaceMember, child_dispatch)];

        let result = coordinator.dispatch_group(&parent_ref, &mut parent_store as &mut dyn SpaceMember, &mut children, Vec::new(), Vec::new(), GroupMeta::default());
        match result {
            Ok(_) => panic!("expected an OwnershipViolation, but the dispatch succeeded"),
            Err(VcsError::OwnershipViolation(_)) => {}
            Err(other) => panic!("expected OwnershipViolation, got a different VcsError: {other}"),
        }
        assert!(child_store.envelope().vcs.edits.is_empty());
    }

    /// @emoji 🧪️ Directly exercises `CompositionCoordinator::compensate` (private, visible to this
    /// nested test module) — the reverse-order rollback `dispatch_group`'s phase 2 falls back to on
    /// a late failure. Proves the order (parent first, then children in reverse dispatch order) and
    /// that a clean rollback restores every member's pre-group snapshot.
    #[test]
    fn compensate_undoes_applied_members_in_reverse_order() {
        let parent_ref = crate::os_io::ArtifactRef { artifact_id: "parent-comp-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
        let child_a_ref = crate::os_io::ArtifactRef { artifact_id: "child-comp-a".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };
        let child_b_ref = crate::os_io::ArtifactRef { artifact_id: "child-comp-b".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

        let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        parent_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply parent");
        let parent_edit_id = parent_store.envelope().vcs.edits.last().expect("parent edit").id.clone();

        let mut child_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &child_a_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        child_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).expect("apply a");
        let child_a_edit_id = child_a.envelope().vcs.edits.last().expect("a edit").id.clone();

        let mut child_b = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &child_b_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        child_b.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 3 }], description: None }).expect("apply b");
        let child_b_edit_id = child_b.envelope().vcs.edits.last().expect("b edit").id.clone();

        let dispatch_a = ChildDispatch { child: child_a_ref.clone(), ops: Vec::new(), op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
        let dispatch_b = ChildDispatch { child: child_b_ref.clone(), ops: Vec::new(), op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
        let mut children: [(&mut dyn SpaceMember, ChildDispatch); 2] = [(&mut child_a as &mut dyn SpaceMember, dispatch_a), (&mut child_b as &mut dyn SpaceMember, dispatch_b)];
        let applied_children = vec![(0usize, child_a_edit_id), (1usize, child_b_edit_id)];

        let report = CompositionCoordinator::compensate(&parent_ref, &mut parent_store as &mut dyn SpaceMember, &mut children, &applied_children, Some(&parent_edit_id));

        assert!(report.skipped.is_empty(), "every member should have undone cleanly");
        assert_eq!(report.undone.len(), 3);
        assert_eq!(report.undone[0].0.artifact_id, parent_ref.artifact_id, "parent undone first");
        assert_eq!(report.undone[1].0.artifact_id, child_b_ref.artifact_id, "then the LAST-dispatched child");
        assert_eq!(report.undone[2].0.artifact_id, child_a_ref.artifact_id, "then the first-dispatched child");

        assert_eq!(parent_store.snapshot().expect("parent snapshot").n, 0, "parent's edit was undone");
        assert_eq!(child_a.snapshot().expect("a snapshot").n, 0, "child a's edit was undone");
        assert_eq!(child_b.snapshot().expect("b snapshot").n, 0, "child b's edit was undone");
    }

    /// @emoji 🧪️ TASK 2's "if compensation itself fails" law: a member whose own rollback errors is
    /// recorded in `GroupUndoReport.skipped` (never panics, never aborts compensating the rest),
    /// and `fold_compensation_error` upgrades the original failure into `VcsError::CompensationFailed`
    /// carrying both facts.
    #[test]
    fn compensate_reports_skipped_when_a_members_own_undo_fails_and_folds_to_compensation_failed() {
        let parent_ref = crate::os_io::ArtifactRef { artifact_id: "parent-comp-fail-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
        let child_ref = crate::os_io::ArtifactRef { artifact_id: "child-comp-fail-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

        // Parent has NOTHING applied, so `parent.undo()` deterministically fails with
        // `NothingToUndo` — simulating a member whose own rollback errors mid-compensation.
        let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        child_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 4 }], description: None }).expect("apply child");
        let child_edit_id = child_store.envelope().vcs.edits.last().expect("child edit").id.clone();

        let dispatch = ChildDispatch { child: child_ref.clone(), ops: Vec::new(), op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
        let mut children: [(&mut dyn SpaceMember, ChildDispatch); 1] = [(&mut child_store as &mut dyn SpaceMember, dispatch)];
        let applied_children = vec![(0usize, child_edit_id)];

        let report = CompositionCoordinator::compensate(&parent_ref, &mut parent_store as &mut dyn SpaceMember, &mut children, &applied_children, Some("bogus-parent-edit-id"));

        assert_eq!(report.skipped.len(), 1, "the parent's own failed undo must be recorded, not panic");
        assert_eq!(report.skipped[0].0.artifact_id, parent_ref.artifact_id);
        assert!(matches!(&report.skipped[0].1, VcsError::NothingToUndo));
        assert_eq!(report.undone.len(), 1, "the child must still be undone despite the parent's rollback failure");
        assert_eq!(child_store.snapshot().expect("child snapshot").n, 0);

        let folded = fold_compensation_error(VcsError::ValidationFailed("late failure".into()), report);
        assert!(matches!(folded, VcsError::CompensationFailed(_)), "a non-empty skipped list must fold into CompensationFailed, got {folded}");
    }

    /// @emoji 🧪️ TASK 2's "deterministic child id minting across two simulated replicas" law,
    /// end-to-end through `dispatch_group` itself (not just the bare `mint_child_id` helper): two
    /// independent coordinators/parents dispatching the IDENTICAL genesis converge on the identical
    /// minted child id and the identical `invocation_id`.
    #[test]
    fn dispatch_group_mints_genesis_child_ids_deterministically_across_replicas() {
        register_child_store_factory(crate::os_io::ArtifactKindId::parse("s.stdio.demochild").expect("valid kind"), Arc::new(DemoChildFactory));

        let parent_ref = crate::os_io::ArtifactRef { artifact_id: "parent-genesis-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
        let genesis_dialect = crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() };
        let genesis = vec![ChildGenesis { slot: "mesh-slot".into(), dialect: genesis_dialect, initial_pack: Vec::new() }];
        let parent_ops: Vec<Vec<u8>> = vec![DemoMutation::SetN { n: 1 }.encode_op().expect("encode")];

        let mut parent_1 = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        let mut coordinator_1 = CompositionCoordinator::new();
        let mut children_1: [(&mut dyn SpaceMember, ChildDispatch); 0] = [];
        let receipt_1 = coordinator_1.dispatch_group(&parent_ref, &mut parent_1 as &mut dyn SpaceMember, &mut children_1, parent_ops.clone(), genesis.clone(), GroupMeta::default()).expect("replica 1 dispatch");

        let mut parent_2 = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        let mut coordinator_2 = CompositionCoordinator::new();
        let mut children_2: [(&mut dyn SpaceMember, ChildDispatch); 0] = [];
        let receipt_2 = coordinator_2.dispatch_group(&parent_ref, &mut parent_2 as &mut dyn SpaceMember, &mut children_2, parent_ops, genesis, GroupMeta::default()).expect("replica 2 dispatch");

        assert_eq!(receipt_1.created_children.len(), 1);
        assert_eq!(receipt_2.created_children.len(), 1);
        assert_eq!(receipt_1.created_children[0].0.artifact_id, receipt_2.created_children[0].0.artifact_id, "two replicas performing the identical genesis must mint the identical child id");
        assert_eq!(receipt_1.invocation_id, receipt_2.invocation_id, "two replicas performing the identical composite gesture must converge on the identical invocation id");

        assert_eq!(parent_1.snapshot().expect("parent snapshot").n, 1);
        assert_eq!(coordinator_1.graph().owner_of(&receipt_1.created_children[0].0.artifact_id), Some(parent_ref.artifact_id.as_str()));
        assert_eq!(receipt_1.member_edits.len(), 1, "only the parent got a real edit here — no existing children were dispatched");
        assert_eq!(receipt_1.member_edits[0].0.artifact_id, parent_ref.artifact_id);
    }

    /// @emoji 🧪️ TASK 2's group-undo law: a member whose tail belongs to a DIFFERENT (foreign)
    /// group is skipped, never aborting the rest of the group's undo.
    #[test]
    fn undo_group_skips_a_foreign_tail_member_but_still_undoes_the_rest() {
        let group_id = "group-xyz";
        let parent_ref = crate::os_io::ArtifactRef { artifact_id: "parent-undo-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
        let child_ref = crate::os_io::ArtifactRef { artifact_id: "child-undo-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };
        let foreign_ref = crate::os_io::ArtifactRef { artifact_id: "foreign-undo-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

        let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        parent_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply parent");
        parent_store.stamp_tail_group_id(group_id).expect("stamp parent");

        let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        child_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).expect("apply child");
        child_store.stamp_tail_group_id(group_id).expect("stamp child");

        let mut foreign_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &foreign_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        foreign_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 3 }], description: None }).expect("apply foreign");
        foreign_store.stamp_tail_group_id("some-other-group").expect("stamp foreign");

        let mut members: [(&crate::os_io::ArtifactRef, &mut dyn SpaceMember); 3] =
            [(&parent_ref, &mut parent_store as &mut dyn SpaceMember), (&child_ref, &mut child_store as &mut dyn SpaceMember), (&foreign_ref, &mut foreign_store as &mut dyn SpaceMember)];

        let report = CompositionCoordinator::undo_group(&mut members, group_id);

        assert_eq!(report.undone.len(), 2, "parent + child both belong to the group and must be undone");
        assert_eq!(report.skipped.len(), 1, "the foreign member must be skipped, not abort the group");
        assert_eq!(report.skipped[0].0.artifact_id, foreign_ref.artifact_id);
        assert!(matches!(&report.skipped[0].1, VcsError::ForeignEdit(_)));

        assert_eq!(parent_store.snapshot().expect("parent snapshot").n, 0, "parent's group edit was undone");
        assert_eq!(child_store.snapshot().expect("child snapshot").n, 0, "child's group edit was undone");
        assert_eq!(foreign_store.snapshot().expect("foreign snapshot").n, 3, "the foreign member's own edit must be left untouched");
    }

    /// @emoji 🧪️ `redo_group`'s mirror of the same law: a foreign-group member's redo stack is left
    /// untouched while a matching member is reapplied.
    #[test]
    fn redo_group_skips_a_foreign_tail_member_but_still_redoes_the_rest() {
        let group_id = "group-redo-1";
        let parent_ref = crate::os_io::ArtifactRef { artifact_id: "parent-redo-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
        let foreign_ref = crate::os_io::ArtifactRef { artifact_id: "foreign-redo-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

        let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        parent_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply parent");
        parent_store.stamp_tail_group_id(group_id).expect("stamp parent");
        parent_store.dispatch(ArtifactCommand::Undo).expect("undo parent, seeding its redo stack");

        let mut foreign_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &foreign_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        foreign_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 9 }], description: None }).expect("apply foreign");
        foreign_store.stamp_tail_group_id("some-other-group").expect("stamp foreign");
        foreign_store.dispatch(ArtifactCommand::Undo).expect("undo foreign, seeding its redo stack");

        let mut members: [(&crate::os_io::ArtifactRef, &mut dyn SpaceMember); 2] = [(&parent_ref, &mut parent_store as &mut dyn SpaceMember), (&foreign_ref, &mut foreign_store as &mut dyn SpaceMember)];
        let report = CompositionCoordinator::redo_group(&mut members, group_id);

        assert_eq!(report.undone.len(), 1, "only the matching-group member is redone");
        assert_eq!(report.undone[0].0.artifact_id, parent_ref.artifact_id);
        assert_eq!(report.skipped.len(), 1);
        assert_eq!(report.skipped[0].0.artifact_id, foreign_ref.artifact_id);

        assert_eq!(parent_store.snapshot().expect("parent snapshot").n, 1, "parent's edit was reapplied");
        assert_eq!(foreign_store.snapshot().expect("foreign snapshot").n, 0, "the foreign member's redo stack was left untouched");
    }
    //#endregion 🔖️CompositionTests
}
//#endregion 🧪️Tests
//#endregion 🧪️Tests
