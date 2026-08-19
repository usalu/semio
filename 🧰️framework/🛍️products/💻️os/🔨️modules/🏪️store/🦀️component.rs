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

// 🚫️async: R7 — `async fn` in a public trait warns because auto trait bounds (e.g. `Send`) cannot
// be named on the method. Answered structurally per R3: every former `dyn` seam in this crate is a
// concrete enum, so `Send` is derived at each spawn site from the concrete type, never from a bound
// on the trait method's returned future. This crate is guest-reachable, so its futures are
// deliberately `?Send` — do not "fix" this warning by adding `-> impl Future<..> + Send` (contradicts
// R3) or by making a trait method sync (contradicts O1/R1).
#![allow(async_fn_in_trait)]

// The `crate::os_dsl::DslArtifact`/`crate::os_dsl::DslOps` derive macros emit `::crate::os_store::ArtifactDsl`/`::crate::os_store::OpText`
// paths (see `dsl/derive/rs/lib.rs`), which only resolve for crates that depend on `store` as an
// external crate — every real consumer, INCLUDING this crate's own `.ops` header grammar
// (`OpsHeaderLine` in `🔖️TextFormat` below, derived on the engine directly) as well as its in-crate
// test fixtures (a crate is never its own dependency otherwise). `// extern crate self removed after merge` is
// the same fix `vcs`/`dsl` use for their own in-crate derive usage: it makes `::store` resolve to
// this crate even when the derive is exercised in-crate.
// extern crate self removed after merge

use crate::os_dsl::{DslOps, DslRecord, DslValue, from_dsl_value, to_dsl_value};
use crate::os_spr::{ActorId, ArtifactId, HybridLogicalTimestamp, MutationId, SchemaId, UndoPolicy};
use crate::os_spr::{Edit, Mutation, MutationDiff, MutationMeta, OpBinary, OpText};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard};

// 🗃️ `store`'s facade over `vcs`'s version-graph algebra — apps that depend on `store` reach
// `Author`/`Change`/`Checkpoint`/`Alternative`/`VcsError`/etc through this crate, never through
// `vcs` directly (see the crate doc comment above).
pub use crate::os_vcs::{
    Alternative, ArtifactVcs, Author, Change, Checkpoint, CollectionDiff, CollectionMutation, Identified, ItemPatch, Patchable, VcsError, apply_collection_mutation, apply_mutation, collection_diff_from_mutation, content_addressed_checkpoint_id,
    content_addressed_entity_id, edit_scoped_id, inverse_collection_mutation, mint_alternative_id, mint_change_id, mint_edit_id, mint_mutation_id,
};

//#region 🔖️ArtifactAssembly
/// 🧷️ One process-wide guard for a plugin's all-registry publication phase.
pub struct ArtifactAssemblyTransaction {
    _guard: MutexGuard<'static, ()>,
}

/// 🚫️ The all-registry publication barrier is unavailable after a writer panic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArtifactAssemblyTransactionError {
    Unavailable,
}

impl std::fmt::Display for ArtifactAssemblyTransactionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("artifact assembly transaction unavailable")
    }
}

impl std::error::Error for ArtifactAssemblyTransactionError {}

fn artifact_assembly_lock() -> &'static Mutex<()> {
    static LOCK: std::sync::OnceLock<Mutex<()>> = std::sync::OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

/// 🧷️ Begins the only cross-registry transaction accepted by artifact registration APIs.
#[must_use]
pub async fn begin_artifact_assembly() -> Result<ArtifactAssemblyTransaction, ArtifactAssemblyTransactionError> {
    Ok(ArtifactAssemblyTransaction { _guard: artifact_assembly_lock().lock().map_err(|_| ArtifactAssemblyTransactionError::Unavailable)? })
}
//#endregion 🔖️ArtifactAssembly

//#region 🔖️Schemas
/// @emoji 🔗️ Identifies the channel a document synchronizes through, when one is attached.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactBackboneRef {
    pub uri: String,
}

/// @emoji 🔗️ Builds a backbone reference from a channel URI.
pub async fn document_backbone_ref(uri: &str) -> ArtifactBackboneRef {
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

//#region 🔖️HistoryLane
/// @emoji 🛤️ Which undo/redo cursor a recorded edit belongs to. `Document` is the default — the
/// ordinary main history `Undo`/`Redo` travels. Any other lane (starting with `Interaction`) is a
/// persisted, replayable SIDE history: mutations recorded under it stay in `ArtifactVcs.edits` and
/// `ArtifactEnvelope.lanes` forever (never dropped, never un-persisted), but default `Undo`/`Redo`
/// skip past them to the nearest `Document`-lane entry instead of reverting them — see
/// `ArtifactStore::dispatch`'s `Undo`/`Redo` arms, and `ArtifactCommand::UndoInLane`/`RedoInLane`
/// for the explicit lane-scoped API that CAN walk a non-`Document` lane. Deliberately an
/// extensible general store mechanism — e.g. a future framework-owned selection/interaction state
/// that must survive reload but must never be what a document editor's undo reverts — not a
/// hover/selection special case baked into this crate. See ticket
/// `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM/📋️master.md` decision 1.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HistoryLane {
    #[default]
    Document,
    Interaction,
}

impl HistoryLane {
    async fn is_document(&self) -> bool {
        matches!(self, HistoryLane::Document)
    }
}
//#endregion 🔖️HistoryLane

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
    /// @emoji 🛤️ Sparse `Edit.id → HistoryLane` ledger: only entries recorded under a NON-`Document`
    /// lane are ever inserted (an ordinary document edit never gets a map entry at all), so an id
    /// absent from this map is `HistoryLane::Document` by construction — see
    /// `ArtifactStore::edit_lane`. Additive — absent (empty map) for every envelope minted before
    /// this field existed, all of which therefore decode as if every edit were `Document` lane,
    /// matching prior undo/redo behavior exactly. Lives on the envelope (not on `Edit<Mutation>`
    /// itself, which this crate's per-technology `Mutation` types don't own) so it survives a plain
    /// JSON round trip (`ArtifactStore::envelope_json`) alongside `cursor`/`owner`/`dialect`.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub lanes: std::collections::BTreeMap<String, HistoryLane>,
    pub edit_messages: Vec<crate::os_spr::EditMessages>,
    pub conflicts: Vec<crate::os_spr::Conflict>,
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
    /// @emoji 🛤️ `Apply`'s lane-tagged twin — records the resulting edit under `lane` (via
    /// `ArtifactEnvelope.lanes`) instead of the implicit `HistoryLane::Document` a plain `Apply`
    /// gets. New variant, not a field added to `Apply` itself, so every existing `Apply {
    /// mutations, description }` construction across the workspace keeps compiling untouched.
    ApplyInLane {
        mutations: Vec<Mutation>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(default)]
        lane: HistoryLane,
    },
    /// @emoji 🛤️ `AmendLast`'s lane-tagged twin — see `ApplyInLane`'s doc for why this is an
    /// additive new variant rather than a field on `AmendLast`.
    AmendLastInLane {
        mutations: Vec<Mutation>,
        coalesce_key: Option<String>,
        #[serde(default)]
        lane: HistoryLane,
    },
    /// @emoji 🛤️ Explicit lane-scoped undo: mirrors plain `Undo`'s `ExactBaseOnly` semantics (must
    /// be local, must be the nearest-to-tail match) but searches `applied_edit_ids` for the nearest
    /// entry whose `HistoryLane` is exactly `lane`, instead of `HistoryLane::Document`. Lets a
    /// caller walk a non-`Document` lane on purpose — the completing half of "default `Undo`/`Redo`
    /// skip non-`Document` lanes" (see `ArtifactStore::dispatch`'s `Undo` arm).
    UndoInLane {
        lane: HistoryLane,
    },
    /// @emoji 🛤️ `UndoInLane`'s redo-direction sibling — mirrors plain `Redo`, but pops the
    /// nearest-to-top `redo_edit_ids` entry whose `HistoryLane` is exactly `lane`.
    RedoInLane {
        lane: HistoryLane,
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
    /// @emoji ⚖️ Sets this store's local `crate::os_spr::MergePolicy` — authority-local state, never
    /// carried on a `crate::os_spr::MutationEnvelope`/`BackboneMessage`, never part of shared
    /// history (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6). Ordinal
    /// 15 — frozen, appended after `PruneDrafts`.
    SetMergePolicy {
        policy: crate::os_spr::MergePolicy,
    },
    /// @emoji ⚔️ Resolves an `Open` `crate::os_spr::Conflict` by id — see
    /// `ArtifactStore::resolve_conflict` for what `Accept`/`Discard` do for each conflict kind.
    /// Ordinal 16 — frozen.
    ResolveConflict {
        conflict_id: String,
        resolution: crate::os_spr::ConflictResolution,
    },
}

impl<Mutation> ArtifactCommand<Mutation> {
    /// 🔮️ Names the projection-invalidating transition this command performs, if it has one.
    pub async fn projection_cause(&self) -> Option<ArtifactProjectionCause> {
        match self {
            Self::Apply { .. } | Self::ApplyInLane { .. } | Self::AmendLast { .. } | Self::AmendLastInLane { .. } => Some(ArtifactProjectionCause::Apply),
            Self::IngestRemote { .. } => Some(ArtifactProjectionCause::RemoteIngest),
            Self::Undo | Self::UndoWithPolicy { .. } | Self::UndoInLane { .. } => Some(ArtifactProjectionCause::Undo),
            Self::Redo | Self::RedoInLane { .. } => Some(ArtifactProjectionCause::Redo),
            Self::CheckoutCheckpoint { .. } | Self::CreateAlternative { .. } | Self::SwitchAlternative { .. } => Some(ArtifactProjectionCause::Checkout),
            Self::CommitCheckpoint { .. } => Some(ArtifactProjectionCause::Checkpoint),
            Self::PruneDrafts => Some(ArtifactProjectionCause::PruneDrafts),
            Self::SetMergePolicy { .. } => Some(ArtifactProjectionCause::PolicyChange),
            Self::ResolveConflict { .. } => Some(ArtifactProjectionCause::RemoteIngest),
        }
    }
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
    pub async fn new(child_id: String, target: crate::os_io::ArtifactRef) -> Self {
        Self { child_id, target, _snapshot: PhantomData }
    }

    /// 🪪️ Drops the compile-time-only `S` phantom, producing the type-erased projection
    /// `ArtifactRefs::child_refs` returns.
    pub async fn to_child_ref(&self, slot: &str) -> ChildRef {
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
    async fn child_refs(&self) -> Vec<ChildRef> {
        Vec::new()
    }
    async fn links(&self) -> Vec<ArtifactLink> {
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
    async fn resolve(&self, link: &ArtifactLink) -> LinkState;
}

/// @emoji 🗂️ The read-only directory a `MemberLinkResolver` resolves against — whatever holds the
/// live members (a `SpaceHost`, or a test fixture). Returns owned PACK bytes rather than a
/// `&dyn SpaceMember` so the resolver can be OWNED and stored (a borrowed member reference could
/// never live in a long-lived host field), and `Option<Result<..>>` distinguishes "no such member"
/// (→ `LinkState::Missing`) from "member exists but reading it failed" (→ a real error, never
/// silently reported as absent).
pub trait MemberDirectory {
    async fn head_pack(&self, artifact_id: &str) -> Option<Result<Vec<u8>, VcsError>>;
    async fn checkpoint_pack(&self, artifact_id: &str, checkpoint_id: &str) -> Option<Result<Vec<u8>, VcsError>>;
}

impl MemberDirectory for SpaceHost {
    async fn head_pack(&self, artifact_id: &str) -> Option<Result<Vec<u8>, VcsError>> {
        match self.member(artifact_id).await { Some(member) => Some(member.document_pack_bytes().await), None => None }
    }

    async fn checkpoint_pack(&self, artifact_id: &str, checkpoint_id: &str) -> Option<Result<Vec<u8>, VcsError>> {
        match self.member(artifact_id).await { Some(member) => Some(member.pack_at_checkpoint(checkpoint_id).await), None => None }
    }
}

/// @emoji 🕵️ The one production `LinkResolver`: resolves each `LinkPin` against a `MemberDirectory`
/// (plus an optional `BlobStore` for escrowed snapshot pins). This is what makes "referenced
/// artifacts with their own version history" real — `Head` reads the target's live tip, while
/// `Checkpoint` reads the target's content AS OF that checkpoint, so a pinned reference keeps
/// showing what it was pinned to no matter how far the target has since moved.
///
/// A `Snapshot` pin with no blob store, or whose bytes are not in the store, degrades to
/// `PinnedOnly` — the blob reference is still known and displayable (a chip can render "pinned to
/// <hash>" without the content), which is precisely the state `LinkState::PinnedOnly` exists for.
/// A read error is never laundered into `Missing`: absence and failure are different answers, and
/// only absence is benign.
/// @emoji 🕳️ The default `B` for a `MemberLinkResolver` that carries no blob escrow — uninhabited,
/// same stand-in shape as `BackboneChannelPorts`/`NoMembers`, so `MemberLinkResolver::new` never
/// needs its caller to name a concrete `BlobStore` implementor it isn't using.
pub enum NoBlobStore {}

impl BlobStore for NoBlobStore {
    async fn put(&self, _bytes: &[u8], _media_type: &str) -> Result<BlobRef, VcsError> {
        match *self {}
    }

    async fn get(&self, _hash: &str) -> Result<Option<Vec<u8>>, VcsError> {
        match *self {}
    }

    async fn has(&self, _hash: &str) -> Result<bool, VcsError> {
        match *self {}
    }

    async fn delete(&self, _hash: &str) -> Result<(), VcsError> {
        match *self {}
    }
}

pub struct MemberLinkResolver<D, B = NoBlobStore> {
    directory: D,
    blobs: Option<Arc<B>>,
}

impl<D: MemberDirectory> MemberLinkResolver<D, NoBlobStore> {
    /// 🏗️ Resolver without blob escrow — `LinkPin::Snapshot` always degrades to `PinnedOnly`.
    pub async fn new(directory: D) -> Self {
        Self { directory, blobs: None }
    }
}

impl<D: MemberDirectory, B: BlobStore> MemberLinkResolver<D, B> {
    /// 🏗️ Resolver that can also materialize content-addressed `LinkPin::Snapshot` blobs.
    pub async fn with_blobs(directory: D, blobs: Arc<B>) -> Self {
        Self { directory, blobs: Some(blobs) }
    }
}

impl<D: MemberDirectory, B: BlobStore> LinkResolver for MemberLinkResolver<D, B> {
    async fn resolve(&self, link: &ArtifactLink) -> LinkState {
        let dialect = link.target.dialect.clone();
        match &link.pin {
            LinkPin::Head => match self.directory.head_pack(&link.target.artifact_id).await {
                Some(Ok(pack_bytes)) => LinkState::Resolved { pack_bytes, dialect },
                Some(Err(_)) | None => LinkState::Missing,
            },
            LinkPin::Checkpoint { id } => match self.directory.checkpoint_pack(&link.target.artifact_id, id).await {
                Some(Ok(pack_bytes)) => LinkState::Resolved { pack_bytes, dialect },
                Some(Err(_)) | None => LinkState::Missing,
            },
            LinkPin::Snapshot { blob } => {
                let resolved = match &self.blobs {
                    Some(blobs) => blobs.get(&blob.hash).await.ok().flatten(),
                    None => None,
                };
                match resolved {
                    Some(pack_bytes) => LinkState::Resolved { pack_bytes, dialect },
                    None => LinkState::PinnedOnly { blob: blob.clone() },
                }
            }
        }
    }
}

/// @emoji 🏭️ Generic genesis-construction helper every `space_members!`-generated `MemberFactory::
/// create` arm calls — the composition sibling of `ArtifactCodec`/`register_document_codec` above.
/// Bakes a brand-new child store from a freshly-baked initial pack (composition genesis, see
/// `CompositionCoordinator::dispatch_group`): `initial_pack` decoded as `P` (empty rejected — a
/// genesis caller always has an initial snapshot, and silently substituting `P::default()` would
/// mint a child whose content nobody chose), stamped with `dialect`. Replaces the old
/// `ChildStoreFactory`/`TypedChildStoreFactory` object-erased pair (O1 — a global `Arc<dyn
/// ChildStoreFactory>` registry keyed by runtime kind string is exactly the dyn-dispatched seam the
/// program drops): the registry's kind-keying now lives in the generated enum's own `match kind`.
pub async fn create_member_store<P, Mutation>(schema: &str, id: &str, dialect: &crate::os_io::ArtifactDialect, initial_pack: &[u8]) -> Result<ArtifactStore<P, Mutation>, VcsError>
where
    P: Clone + Serialize + DeserializeOwned + ArtifactPack + Send + 'static,
    Mutation: Clone + Serialize + DeserializeOwned + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    if initial_pack.is_empty() {
        return Err(VcsError::Deserialize(format!("child genesis for {id} carries an empty initial pack")));
    }
    let initial = P::decode_pack(initial_pack).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let mut envelope = create_document_envelope::<P, Mutation>(schema, id, initial, None).await;
    envelope.dialect = Some(dialect.clone());
    ArtifactStore::new(envelope).await
}

/// @emoji 🏭️ Generic reload helper every `space_members!`-generated `MemberFactory::open` arm
/// calls — reconstructs a previously-persisted child from its full envelope pack via the same
/// `parse_document_pack` → `reset(envelope, applied, redo)` path `apply_ops_binary` uses, so a
/// reloaded child restores its exact undo/redo cursor position, not merely its content. The
/// invariant is `owner.is_some() ⇒ dialect.is_some()` — every envelope that is somebody's child
/// knows which dialect it materializes as, which is what lets `ArtifactView.children` type a child
/// without consulting the parent.
pub async fn open_member_store<P, Mutation>(envelope_pack: &[u8]) -> Result<ArtifactStore<P, Mutation>, VcsError>
where
    P: Clone + Serialize + DeserializeOwned + ArtifactPack + Send + 'static,
    Mutation: Clone + Serialize + DeserializeOwned + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    let (pack, spr) = decode_document_pack_bytes(envelope_pack).await?;
    let parsed = parse_document_pack::<P, Mutation>(&pack, &spr).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let envelope = parsed.envelope;
    if envelope.owner.is_some() && envelope.dialect.is_none() {
        return Err(VcsError::Deserialize(format!("owned child {} carries no dialect", envelope.id)));
    }
    let (applied, redo) = match &envelope.cursor {
        Some(cursor) => (cursor.applied_edit_ids.clone(), cursor.redo_edit_ids.clone()),
        None => (envelope.vcs.edits.iter().map(|edit| edit.id.clone()).collect(), Vec::new()),
    };
    let mut store = ArtifactStore::new(envelope.clone()).await?;
    store.reset(envelope, applied, redo).await?;
    Ok(store)
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
// 🚫️async: E4 fn-pointer slot — value stored in `Shape::Record(fn() -> RecordSpec)`
fn artifact_child_spec() -> crate::os_dsl::RecordSpec {
    crate::os_dsl::RecordSpec::new(None, crate::os_dsl::RecordLayout::Inline, vec![crate::os_dsl::FieldSpec::new(0, "child_id", crate::os_dsl::Shape::Text), crate::os_dsl::FieldSpec::new(1, "target", crate::os_dsl::Shape::Text)])
}

async fn artifact_child_to_record<S>(child: &ArtifactChild<S>) -> crate::os_dsl::RecordValue {
    let mut record = crate::os_dsl::RecordValue::default();
    record.fields.insert(0, crate::os_dsl::FieldValue::Text(child.child_id.clone()));
    record.fields.insert(1, crate::os_dsl::FieldValue::Text(child.target.to_uri().await));
    record
}

async fn artifact_child_from_record<S>(record: &crate::os_dsl::RecordValue) -> Result<ArtifactChild<S>, String> {
    let child_id = match record.get(0) {
        Some(crate::os_dsl::FieldValue::Text(s)) => s.clone(),
        other => return Err(format!("expected child_id, found {other:?}")),
    };
    let target = match record.get(1) {
        Some(crate::os_dsl::FieldValue::Text(s)) => crate::os_io::ArtifactRef::parse_uri(s).await?,
        other => return Err(format!("expected target, found {other:?}")),
    };
    Ok(ArtifactChild::new(child_id, target).await)
}

impl<S> crate::os_dsl::DslField for ArtifactChild<S> {
    // 🚫️async: E4 — see `DslField::shape`'s tag on the trait.
    fn shape() -> crate::os_dsl::Shape {
        crate::os_dsl::Shape::Record(artifact_child_spec)
    }
    async fn to_value(&self) -> crate::os_dsl::FieldValue {
        crate::os_dsl::FieldValue::Record(artifact_child_to_record(self).await)
    }
    async fn from_value(value: &crate::os_dsl::FieldValue) -> Result<Self, String> {
        match value {
            crate::os_dsl::FieldValue::Record(record) => artifact_child_from_record(record).await,
            other => Err(format!("expected Record, found {other:?}")),
        }
    }
}

// 🚫️async: E4 fn-pointer slot — value stored in `Shape::Record(fn() -> RecordSpec)`
fn owner_ref_spec() -> crate::os_dsl::RecordSpec {
    crate::os_dsl::RecordSpec::new(
        None,
        crate::os_dsl::RecordLayout::Inline,
        vec![crate::os_dsl::FieldSpec::new(0, "parent", crate::os_dsl::Shape::Text), crate::os_dsl::FieldSpec::new(1, "slot", crate::os_dsl::Shape::Text), crate::os_dsl::FieldSpec::new(2, "child_id", crate::os_dsl::Shape::Text)],
    )
}

async fn owner_ref_to_record(owner: &OwnerRef) -> crate::os_dsl::RecordValue {
    let mut record = crate::os_dsl::RecordValue::default();
    record.fields.insert(0, crate::os_dsl::FieldValue::Text(owner.parent.to_uri().await));
    record.fields.insert(1, crate::os_dsl::FieldValue::Text(owner.slot.clone()));
    record.fields.insert(2, crate::os_dsl::FieldValue::Text(owner.child_id.clone()));
    record
}

async fn owner_ref_from_record(record: &crate::os_dsl::RecordValue) -> Result<OwnerRef, String> {
    let parent = match record.get(0) {
        Some(crate::os_dsl::FieldValue::Text(s)) => crate::os_io::ArtifactRef::parse_uri(s).await?,
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
    // 🚫️async: E4 — see `DslField::shape`'s tag on the trait.
    fn shape() -> crate::os_dsl::Shape {
        crate::os_dsl::Shape::Record(owner_ref_spec)
    }
    async fn to_value(&self) -> crate::os_dsl::FieldValue {
        crate::os_dsl::FieldValue::Record(owner_ref_to_record(self).await)
    }
    async fn from_value(value: &crate::os_dsl::FieldValue) -> Result<Self, String> {
        match value {
            crate::os_dsl::FieldValue::Record(record) => owner_ref_from_record(record).await,
            other => Err(format!("expected Record, found {other:?}")),
        }
    }
}

// 🚫️async: E4 fn-pointer slot — value stored in `Shape::Record(fn() -> RecordSpec)`
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

async fn link_pin_to_record(pin: &LinkPin) -> crate::os_dsl::RecordValue {
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

async fn link_pin_from_record(record: &crate::os_dsl::RecordValue) -> Result<LinkPin, String> {
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
    // 🚫️async: E4 — see `DslField::shape`'s tag on the trait.
    fn shape() -> crate::os_dsl::Shape {
        crate::os_dsl::Shape::Record(link_pin_spec)
    }
    async fn to_value(&self) -> crate::os_dsl::FieldValue {
        crate::os_dsl::FieldValue::Record(link_pin_to_record(self).await)
    }
    async fn from_value(value: &crate::os_dsl::FieldValue) -> Result<Self, String> {
        match value {
            crate::os_dsl::FieldValue::Record(record) => link_pin_from_record(record).await,
            other => Err(format!("expected Record, found {other:?}")),
        }
    }
}

// 🚫️async: E4 fn-pointer slot — value stored in `Shape::Record(fn() -> RecordSpec)`
fn artifact_link_spec() -> crate::os_dsl::RecordSpec {
    crate::os_dsl::RecordSpec::new(
        None,
        crate::os_dsl::RecordLayout::Inline,
        vec![crate::os_dsl::FieldSpec::new(0, "target", crate::os_dsl::Shape::Text), crate::os_dsl::FieldSpec::new(1, "pin", crate::os_dsl::Shape::Record(link_pin_spec)), crate::os_dsl::FieldSpec::new(2, "role", crate::os_dsl::Shape::Text)],
    )
}

async fn artifact_link_to_record(link: &ArtifactLink) -> crate::os_dsl::RecordValue {
    let mut record = crate::os_dsl::RecordValue::default();
    record.fields.insert(0, crate::os_dsl::FieldValue::Text(link.target.to_uri().await));
    record.fields.insert(1, crate::os_dsl::FieldValue::Record(link_pin_to_record(&link.pin).await));
    record.fields.insert(2, crate::os_dsl::FieldValue::Text(link.role.clone()));
    record
}

async fn artifact_link_from_record(record: &crate::os_dsl::RecordValue) -> Result<ArtifactLink, String> {
    let target = match record.get(0) {
        Some(crate::os_dsl::FieldValue::Text(s)) => crate::os_io::ArtifactRef::parse_uri(s).await?,
        other => return Err(format!("expected target, found {other:?}")),
    };
    let pin = match record.get(1) {
        Some(crate::os_dsl::FieldValue::Record(record)) => link_pin_from_record(record).await?,
        other => return Err(format!("expected pin, found {other:?}")),
    };
    let role = match record.get(2) {
        Some(crate::os_dsl::FieldValue::Text(s)) => s.clone(),
        other => return Err(format!("expected role, found {other:?}")),
    };
    Ok(ArtifactLink { target, pin, role })
}

impl crate::os_dsl::DslField for ArtifactLink {
    // 🚫️async: E4 — see `DslField::shape`'s tag on the trait.
    fn shape() -> crate::os_dsl::Shape {
        crate::os_dsl::Shape::Record(artifact_link_spec)
    }
    async fn to_value(&self) -> crate::os_dsl::FieldValue {
        crate::os_dsl::FieldValue::Record(artifact_link_to_record(self).await)
    }
    async fn from_value(value: &crate::os_dsl::FieldValue) -> Result<Self, String> {
        match value {
            crate::os_dsl::FieldValue::Record(record) => artifact_link_from_record(record).await,
            other => Err(format!("expected Record, found {other:?}")),
        }
    }
}
//#endregion 🔖️CompositionDsl
//#endregion 🔖️Composition

//#region 🔖️Authority
/// @emoji 🧾 Receipt from the sole store write gate (`dispatch` / `reset`). `messages`/`worst` carry
/// whatever `crate::os_spr::MutationMessage`s the command's own replay produced (empty/`None` for a
/// structural command with nothing to report) — `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-
/// FIRST-CLASS-CONFLICTS` §C6. `Eq` dropped (was `#[derive(.., PartialEq, Eq)]`): `EditMessages`
/// only derives `PartialEq`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CommandReceipt {
    pub edit_ids: Vec<String>,
    pub generation: u64,
    pub messages: Vec<crate::os_spr::EditMessages>,
    pub worst: Option<crate::os_dsl::Severity>,
}

//#region 🔮️Projection
//#region 🧭️Stamps
/// 🧬️ History identity for a projection input, independent of the process-local generation.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArtifactRevision {
    pub artifact_id: String,
    pub schema: String,
    pub applied_edit_ids: Vec<String>,
    pub redo_edit_ids: Vec<String>,
    pub checkpoint_id: Option<String>,
}

/// 🎯️ Exact state identity a projection or inference result must match before it is accepted.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArtifactProjectionStamp {
    pub revision: ArtifactRevision,
    pub generation: u64,
}

/// 🔄️ Store transition that produced a projection input.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ArtifactProjectionCause {
    Apply,
    RemoteIngest,
    Replay,
    Undo,
    Redo,
    Reset,
    Checkout,
    PolicyChange,
    ExternalResourceChange,
    Checkpoint,
    PruneDrafts,
}

/// 🗄️ Explicit prior-result handling; projections never infer cache authority from state alone.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ArtifactProjectionCacheMode {
    Rebuild,
    ReusePrevious,
    ValidatePrevious,
}
//#endregion 🧭️Stamps

//#region 📡️Events
/// 📬️ Typed projection/inference input carrying the exact artifact version it observed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactProjectionEvent<State, Previous, Policy> {
    pub stamp: ArtifactProjectionStamp,
    pub cause: ArtifactProjectionCause,
    pub state: State,
    pub previous: Option<Previous>,
    pub cache_mode: ArtifactProjectionCacheMode,
    pub policy: Policy,
}

/// 🧩️ Projection output plus an optional semantic-diff candidate. This layer validates freshness;
/// strict `ArtifactDiff` application stays owned by the semantic-mutation boundary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactProjectionResult<Output, Diff = ()> {
    pub stamp: ArtifactProjectionStamp,
    pub output: Output,
    pub proposed_diff: Option<Diff>,
}

impl<State, Previous, Policy> ArtifactProjectionEvent<State, Previous, Policy> {
    /// 🧪️ Couples an asynchronously computed result to this event's immutable stamp.
    pub async fn result<Output, Diff>(&self, output: Output, proposed_diff: Option<Diff>) -> ArtifactProjectionResult<Output, Diff> {
        ArtifactProjectionResult { stamp: self.stamp.clone(), output, proposed_diff }
    }
}

/// ✅️ A current projection result, intentionally separated from semantic-diff application.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AcceptedArtifactProjection<Output, Diff = ()> {
    pub output: Output,
    pub proposed_diff: Option<Diff>,
}

/// 🚫️ A result computed against an older state and therefore forbidden from changing current state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StaleArtifactProjection {
    pub computed_for: ArtifactProjectionStamp,
    pub current: ArtifactProjectionStamp,
}

/// 📣️ One state or dependency transition that invalidates derived artifact work.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactProjectionInvalidation {
    pub cause: ArtifactProjectionCause,
    pub stamp: ArtifactProjectionStamp,
}
//#endregion 📡️Events
//#endregion 🔮️Projection

/// @emoji 👁️ Read-only view over a document envelope — mutation is sealed through `dispatch`/`reset`.
#[derive(Clone, Copy, Debug)]
pub struct ArtifactEnvelopeView<'a, P, Mutation> {
    envelope: &'a ArtifactEnvelope<P, Mutation>,
}

impl<'a, P, Mutation> ArtifactEnvelopeView<'a, P, Mutation> {
    pub async fn schema(&self) -> &str {
        &self.envelope.schema
    }
    pub async fn id(&self) -> &str {
        &self.envelope.id
    }
    pub async fn vcs(&self) -> &ArtifactVcs<P, Mutation> {
        &self.envelope.vcs
    }
    pub async fn backbone(&self) -> Option<&ArtifactBackboneRef> {
        self.envelope.backbone.as_ref()
    }
    pub async fn active_alternative_id(&self) -> Option<&str> {
        self.envelope.active_alternative_id.as_deref()
    }
    pub async fn cursor(&self) -> Option<&ArtifactCursor> {
        self.envelope.cursor.as_ref()
    }
    pub async fn edit_messages(&self) -> &[crate::os_spr::EditMessages] {
        &self.envelope.edit_messages
    }
    pub async fn conflicts(&self) -> &[crate::os_spr::Conflict] {
        &self.envelope.conflicts
    }
    pub async fn inner(&self) -> &'a ArtifactEnvelope<P, Mutation> {
        self.envelope
    }
}

/// @emoji 📝 Draft-lane store alias — same algebra as ArtifactStore; PruneDrafts never enters a Change.
pub type DraftStore<P, Mutation> = ArtifactStore<P, Mutation>;

//#region 🔖️EphemeralLanes
/// @emoji 👥️ The PRESENCE lane's store: ephemeral SHARED state — a last-writer-wins roster, NOT an
/// event log.
///
/// 🎯️ Why not `ArtifactStore` (which `ConfigStore`/`DraftStore` both alias): presence has no
/// history, no undo, no checkpoints and no merge. Each peer is the sole author of its own presence
/// value, and the wire already carries whole encoded snapshots per peer
/// (`ClientFrame::Presence { peer }` / `ServerFrame::Presence { peers }`), so a later frame from a
/// peer simply SUPERSEDES its earlier one. Modelling that as an op log would mint an unbounded
/// history of cursor positions nobody can ever undo.
///
/// `generation` bumps on every local change so the host can tell "something to broadcast" from
/// "nothing changed" without diffing snapshots — the signal a heartbeat coalescer throttles on.
#[derive(Clone, Debug)]
pub struct PresenceStore<P, Mutation> {
    local: P,
    peers: HashMap<String, (P, i64)>,
    generation: u64,
    _mutation: PhantomData<fn() -> Mutation>,
}

// 🚫️async: E1 impl of external `Default` — signature fixed outside this repo.
impl<P: Clone + Default, Mutation: self::Mutation<P>> Default for PresenceStore<P, Mutation> {
    fn default() -> Self {
        Self::new(P::default())
    }
}

impl<P: Clone, Mutation: self::Mutation<P>> PresenceStore<P, Mutation> {
    /// 🏗️ A roster holding only this actor's own initial presence.
    // 🚫️async: E1 pure constructor, consumed by `Default::default()` — see R9.
    pub fn new(local: P) -> Self {
        Self { local, peers: HashMap::new(), generation: 0, _mutation: PhantomData }
    }

    /// 👤️ This actor's own presence — what gets broadcast.
    pub async fn local(&self) -> &P {
        &self.local
    }

    /// ✍️ Applies local presence operations atomically, bumping `generation` after the full batch.
    pub async fn apply(&mut self, mutations: &[Mutation]) -> crate::os_spr::MutationApplyResult<()> {
        let mut candidate = self.local.clone();
        for mutation in mutations {
            candidate = mutation.diff(&candidate).await.diff().await.apply(&candidate).await?;
        }
        if !mutations.is_empty() {
            self.local = candidate;
            self.generation = self.generation.wrapping_add(1);
        }
        Ok(())
    }

    /// 📥️ Adopts a remote peer's whole presence snapshot, superseding whatever it last sent.
    /// `received_at_ms` is the host's receive clock, used only to expire silent peers.
    pub async fn adopt_peer(&mut self, actor: impl Into<String>, presence: P, received_at_ms: i64) {
        self.peers.insert(actor.into(), (presence, received_at_ms));
    }

    /// 🚪️ Drops a peer that left.
    pub async fn remove_peer(&mut self, actor: &str) -> bool {
        self.peers.remove(actor).is_some()
    }

    /// ⏳️ Drops every peer whose last update is older than `oldest_allowed_ms` — a disconnected
    /// collaborator's cursor must not linger forever.
    pub async fn expire_peers(&mut self, oldest_allowed_ms: i64) {
        self.peers.retain(|_, (_, received_at_ms)| *received_at_ms >= oldest_allowed_ms);
    }

    /// 👥️ Every peer's current presence, sorted by actor so readers get a stable order.
    pub async fn peers(&self) -> Vec<(&str, &P)> {
        let mut peers: Vec<(&str, &P)> = self.peers.iter().map(|(actor, (presence, _))| (actor.as_str(), presence)).collect();
        peers.sort_by_key(|(actor, _)| *actor);
        peers
    }

    /// 🔢️ Bumps on every local change; never on adopting a remote peer (that needs no rebroadcast).
    pub async fn generation(&self) -> u64 {
        self.generation
    }
}

/// @emoji 🫧️ The TRANSIENT lane's store: ephemeral LOCAL-ONLY UI state.
///
/// 🎯️ Presence minus the roster. Nothing here is ever shared, persisted, packed, checkpointed or
/// undone — it is exactly the state that used to hide in plugin `thread_local!`s and untyped shell
/// fields, given a typed home so it can be reached only through `Emit`/`Lanes` like every other
/// lane. If a value must survive a reload it belongs in `config`; if a peer must see it, in
/// `presence`; if it is document content, in the artifact (or its draft).
#[derive(Clone, Debug)]
pub struct TransientStore<P, Mutation> {
    current: P,
    generation: u64,
    _mutation: PhantomData<fn() -> Mutation>,
}

// 🚫️async: E1 impl of external `Default` — signature fixed outside this repo.
impl<P: Clone + Default, Mutation: self::Mutation<P>> Default for TransientStore<P, Mutation> {
    fn default() -> Self {
        Self::new(P::default())
    }
}

impl<P: Clone, Mutation: self::Mutation<P>> TransientStore<P, Mutation> {
    // 🚫️async: E1 pure constructor, consumed by `Default::default()` — see R9.
    pub fn new(current: P) -> Self {
        Self { current, generation: 0, _mutation: PhantomData }
    }

    pub async fn current(&self) -> &P {
        &self.current
    }

    /// ✍️ Applies transient operations atomically, without history.
    pub async fn apply(&mut self, mutations: &[Mutation]) -> crate::os_spr::MutationApplyResult<()> {
        let mut candidate = self.current.clone();
        for mutation in mutations {
            candidate = mutation.diff(&candidate).await.diff().await.apply(&candidate).await?;
        }
        if !mutations.is_empty() {
            self.current = candidate;
            self.generation = self.generation.wrapping_add(1);
        }
        Ok(())
    }

    /// 🔄️ Discards everything — what a host does when a view closes.
    pub async fn reset(&mut self, current: P) {
        self.current = current;
        self.generation = self.generation.wrapping_add(1);
    }

    /// 🔢️ Bumps on every change, so a renderer can skip untouched frames.
    pub async fn generation(&self) -> u64 {
        self.generation
    }
}

//#region 🔖️InteractionStore
/// @emoji 🕹️ Ephemeral LOCAL-ONLY lane for the framework interaction mechanism's hover half — the
/// `PresenceStore`/`TransientStore` sibling for `InteractionState.hover`.
///
/// 🎯️ Only hover lives here. Selection + `active_mode`/`active_granularity` are
/// **persisted-local** (`InteractionState`'s other three fields) and already flow through the
/// ordinary `ConfigStore` (`ArtifactStore<Config, ConfigMutation>`) via `ArtifactCommand::
/// ApplyInLane { lane: HistoryLane::Interaction, .. }` — no new store type needed for that half,
/// it is the ENTIRE point of the `HistoryLane` mechanism above. Hover must never take that path
/// (never persisted, never in `Edit`/`Change` history), so it gets this dedicated ephemeral
/// sibling instead, shaped exactly like `TransientStore` (single local value, no peer roster —
/// peers' hover arrives over the wire as `PresencePeer.interaction`, not through this store).
///
/// `S`/`Mutation` are supplied by the caller (mirrors `PresenceStore<A::Presence,
/// A::PresenceMutation>` in `VcsArtifactApp`): this crate has no dependency edge to the
/// `semio-framework` crate that owns `InteractionState`/`DomainHover` (that crate depends on
/// THIS one, for its `spr`/`dsl`/`pack` types — the reverse edge would cycle), so `S` stays
/// generic here and gets instantiated with the app's concrete hover-shaped type one layer up,
/// where that dependency edge actually exists.
#[derive(Clone, Debug)]
pub struct InteractionStore<S, Mutation> {
    hover: S,
    generation: u64,
    _mutation: PhantomData<fn() -> Mutation>,
}

// 🚫️async: E1 impl of external `Default` — signature fixed outside this repo.
impl<S: Clone + Default, Mutation: self::Mutation<S>> Default for InteractionStore<S, Mutation> {
    fn default() -> Self {
        Self::new(S::default())
    }
}

impl<S: Clone, Mutation: self::Mutation<S>> InteractionStore<S, Mutation> {
    /// 🏗️ Starts with this actor's own initial hover state (typically empty).
    // 🚫️async: E1 pure constructor, consumed by `Default::default()` — see R9.
    pub fn new(hover: S) -> Self {
        Self { hover, generation: 0, _mutation: PhantomData }
    }

    /// 👁️ This actor's own hover — what the presence heartbeat mirrors onto `PresenceDomain.hovered`.
    pub async fn hover(&self) -> &S {
        &self.hover
    }

    /// ✍️ Applies hover updates atomically. The caller's `Mutation` impl is expected to route
    /// through the framework's `next_hover` pure fn before landing here.
    pub async fn apply(&mut self, mutations: &[Mutation]) -> crate::os_spr::MutationApplyResult<()> {
        let mut candidate = self.hover.clone();
        for mutation in mutations {
            candidate = mutation.diff(&candidate).await.diff().await.apply(&candidate).await?;
        }
        if !mutations.is_empty() {
            self.hover = candidate;
            self.generation = self.generation.wrapping_add(1);
        }
        Ok(())
    }

    /// 🔄️ Discards hover — a host clears this when a view/window closes (nothing left to hover).
    pub async fn reset(&mut self, hover: S) {
        self.hover = hover;
        self.generation = self.generation.wrapping_add(1);
    }

    /// 🔢️ Bumps on every local hover change; drives the same broadcast-coalescing signal as
    /// `PresenceStore::generation`.
    pub async fn generation(&self) -> u64 {
        self.generation
    }
}
//#endregion 🔖️InteractionStore
//#endregion 🔖️EphemeralLanes
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
    async fn parse_dsl(text: &str) -> Result<Self, TextError>;
    async fn print_dsl(&self) -> String;
    /// @emoji 🪪️ Dotted `plugin.artifact` identity for `.semio` preambles and on-disk names.
    async fn envelope_id() -> &'static str {
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
    pub async fn encode_document(spec: &RecordSpec, record: &RecordValue, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        crate::os_pack::encode_document(spec, record, options).await
    }

    /// @emoji 🚪️ Forwards to `crate::os_pack::decode_document`.
    pub async fn decode_document(bytes: &[u8], spec: &RecordSpec, options: &PackDecodeOptions) -> Result<(RecordValue, crate::os_pack::DecodeReport), PackError> {
        crate::os_pack::decode_document(bytes, spec, options).await
    }

    /// @emoji 🎯️ P6: container-less record body helpers for handcrafted OpBinary impls.
    pub async fn encode_record_body(spec: &RecordSpec, record: &RecordValue, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        crate::os_pack::encode_record_body(spec, record, options).await
    }
    pub async fn decode_record_body(bytes: &[u8], spec: &RecordSpec, options: &PackDecodeOptions) -> Result<(RecordValue, crate::os_pack::DecodeReport), PackError> {
        crate::os_pack::decode_record_body(bytes, spec, options).await
    }
    pub async fn write_varint_u64(out: &mut Vec<u8>, value: u64) {
        crate::os_pack::write_varint_u64(out, value).await
    }
    pub use crate::os_pack::ByteReader;
    /// @emoji 🎯️ Format byte every encoded operation starts with (handcrafted OpBinary convention).
    pub const OP_BINARY_FORMAT: u8 = 1;

    /// @emoji 🌱️ Field id the JSON bridge's synthetic single-field record wraps a whole
    /// `serde_json::Value` payload in — mirrors `crate::os_dsl::DslField for serde_json::Value`'s
    /// `Shape::Value` escape hatch (`dsl/rs/lib.rs`), lifted one level from "one field" to "one
    /// whole document" so schema-less apps (puzzle plugins, semio_compose_rs kit) get a pack encoding too.
    const VALUE_BRIDGE_FIELD_ID: u16 = 1;

    async fn value_bridge_spec() -> RecordSpec {
        RecordSpec::new(None, RecordLayout::Lines, vec![FieldSpec::new(VALUE_BRIDGE_FIELD_ID, "value", Shape::Value)])
    }

    /// @emoji 🌱️ Encodes an arbitrary `DslValue` as a complete pack file.
    pub async fn encode_pack_value(value: &DslValue) -> Vec<u8> {
        let mut fields = HashMap::new();
        fields.insert(VALUE_BRIDGE_FIELD_ID, FieldValue::Value(value.clone()));
        let record = RecordValue { fields };
        encode_document(&value_bridge_spec().await, &record, &PackEncodeOptions::default()).await.expect("value bridge encode is infallible for a well-formed DslValue")
    }

    /// @emoji 🌱️ Inverse of `encode_pack_value`.
    pub async fn decode_pack_value(bytes: &[u8]) -> Result<DslValue, PackError> {
        let (record, _report) = decode_document(bytes, &value_bridge_spec().await, &PackDecodeOptions::default()).await?;
        match record.get(VALUE_BRIDGE_FIELD_ID) {
            Some(FieldValue::Value(dsl_value)) => Ok(dsl_value.clone()),
            _ => Ok(DslValue::Null),
        }
    }

    /// @emoji 🪶️ Container-less twin of `encode_pack_value` for per-message wire payloads.
    pub async fn encode_wire_value(value: &DslValue) -> Vec<u8> {
        let mut fields = HashMap::new();
        fields.insert(VALUE_BRIDGE_FIELD_ID, FieldValue::Value(value.clone()));
        let record = RecordValue { fields };
        crate::os_pack::encode_record_body(&value_bridge_spec().await, &record, &PackEncodeOptions::default()).await.expect("wire value encode is infallible for a well-formed DslValue")
    }

    /// @emoji 🪶️ Inverse of `encode_wire_value`.
    pub async fn decode_wire_value(bytes: &[u8]) -> Result<DslValue, PackError> {
        let (record, _report) = crate::os_pack::decode_record_body(bytes, &value_bridge_spec().await, &PackDecodeOptions::default()).await?;
        match record.get(VALUE_BRIDGE_FIELD_ID) {
            Some(FieldValue::Value(dsl_value)) => Ok(dsl_value.clone()),
            _ => Ok(DslValue::Null),
        }
    }

    /// @emoji 🧩️ Compose-only bridge — external technology; converts through `DslValue` without JSON on the wire.
    pub async fn encode_json_value(value: &serde_json::Value) -> Vec<u8> {
        encode_pack_value(&json_value_to_dsl(value)).await
    }

    /// @emoji 🧩️ Compose-only inverse of `encode_json_value`.
    pub async fn decode_json_value(bytes: &[u8]) -> Result<serde_json::Value, PackError> {
        decode_pack_value(bytes).await.map(dsl_value_to_json)
    }

    /// @emoji 📦️ Prefix for base64-wrapped pack bytes in scene `*Json` string slots (TS `PACK_B64_PREFIX`).
    pub const PACK_B64_PREFIX: &str = "pk:";

    /// @emoji 📦️ Lossless pack snapshot as a `pk:`-prefixed base64 string.
    pub async fn pack_value_to_base64(bytes: &[u8]) -> String {
        use base64::Engine;
        format!("{}{}", PACK_B64_PREFIX, base64::engine::general_purpose::STANDARD.encode(bytes))
    }

    /// @emoji 📥️ Inverse of [`pack_value_to_base64`].
    pub async fn pack_value_from_base64(encoded: &str) -> Result<Vec<u8>, PackError> {
        let payload = encoded.strip_prefix(PACK_B64_PREFIX).ok_or(PackError::Malformed { what: "pack base64", offset: 0, detail: "missing pk: prefix".into() })?;
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.decode(payload).map_err(|error| PackError::Malformed { what: "pack base64", offset: 0, detail: error.to_string() })
    }

    /// @emoji 🎬️ Decodes a component-scene `*Json` field when it carries [`pack_value_to_base64`] bytes.
    pub async fn decode_scene_pack_field(encoded: &str) -> Result<DslValue, PackError> {
        if encoded.starts_with(PACK_B64_PREFIX) {
            decode_pack_value(&pack_value_from_base64(encoded).await?).await
        } else {
            Ok(json_value_to_dsl(&serde_json::from_str(encoded).map_err(|error| PackError::Malformed { what: "scene field", offset: 0, detail: error.to_string() })?))
        }
    }

    /// @emoji 🎬️ Expands a scene `*Json` slot to JSON text for engines that still ingest stringified payloads.
    pub async fn scene_field_json_text(field: &str) -> Result<String, PackError> {
        if field.starts_with(PACK_B64_PREFIX) {
            let dsl = decode_pack_value(&pack_value_from_base64(field).await?).await?;
            Ok(serde_json::to_string(&dsl_value_to_json(dsl)).unwrap_or_else(|_| "null".into()))
        } else {
            Ok(field.to_string())
        }
    }

    /// @emoji 🧩️ Compose wire decode helper — renormalizes a `serde_json::Value` tree after pack decode.
    // 🚫️async: E1 pure transform pipeline consumed by json_values_equal's `==` (external PartialEq) — see R9
    pub fn renormalize_json_wire_value(value: serde_json::Value) -> serde_json::Value {
        dsl_value_to_json(renormalize_whole_number_floats(json_value_to_dsl(&value)))
    }

    // 🚫️async: E1 pure recursive transform, consumed by json_values_equal's `==` (external PartialEq) — see R9
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

    // 🚫️async: E1 pure recursive transform, consumed by json_values_equal's `==` (external PartialEq) — see R9
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
    // 🚫️async: E1 pure predicate; a fleet PartialEq::eq impl calls this synchronously — see R9
    pub fn json_values_equal(a: &serde_json::Value, b: &serde_json::Value) -> bool {
        json_value_to_dsl(a) == json_value_to_dsl(b)
    }

    /// @emoji 🔧️ Rewrites fractionless floats in a `DslValue` tree to whole-number floats for integer fields.
    // 🚫️async: E1 pure recursive transform, consumed by json_values_equal's `==` (external PartialEq) — see R9
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
    async fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError>;
    async fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError>;

    /// @emoji 📦️ `encode_pack_with` at default options — infallible in practice (mirrors
    /// `ArtifactDsl::print_dsl`'s infallible signature); panics only on a `PackLimits` overflow.
    async fn encode_pack(&self) -> Vec<u8> {
        self.encode_pack_with(&PackEncodeOptions::default()).await.expect("default-options pack encode is infallible")
    }

    /// @emoji 📦️ `decode_pack_with` at default (Standard) verification.
    async fn decode_pack(bytes: &[u8]) -> Result<Self, PackError> {
        Self::decode_pack_with(bytes, &PackDecodeOptions::default()).await
    }

    /// @emoji 🧬️ This document kind's structural field spec, for `ArtifactCodec::pack_schema_hash`
    /// (W5.7's semio_hub schema-hash validation — see that field's doc). Default `None` for hand-written
    /// `ArtifactPack` impls with no `RecordSpec` (schema-erased or synthetic fixture types, e.g.
    /// `serde_json::Value` above): those document kinds simply opt out (a zero hash reads as
    /// "schema-agnostic" everywhere this is consumed). `#[derive(crate::os_dsl::DslArtifact)]` overrides this
    /// with the real generated `__dsl_spec()`, giving every derive-based app kind (the overwhelming
    /// majority) a genuine structural fingerprint with zero manual per-app wiring.
    async fn record_spec() -> Option<crate::os_dsl::RecordSpec> {
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
pub async fn encode_document_pack_bytes(pack: &[u8], spr: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    crate::os_pack::write_varint_u64(&mut out, pack.len() as u64).await;
    out.extend_from_slice(pack);
    out.extend_from_slice(spr);
    out
}

/// @emoji 🔌️ Inverse of `encode_document_pack_bytes`.
pub async fn decode_document_pack_bytes(bytes: &[u8]) -> Result<(Vec<u8>, Vec<u8>), VcsError> {
    let mut pos = 0usize;
    let pack_len = crate::os_pack::read_varint_u64(bytes, &mut pos).await.map_err(|error| VcsError::Deserialize(error.to_string()))? as usize;
    let pack_end = pos.checked_add(pack_len).ok_or_else(|| VcsError::Deserialize("document pack bytes overflow".to_string()))?;
    if pack_end > bytes.len() {
        return Err(VcsError::Deserialize("document pack bytes truncated".to_string()));
    }
    Ok((bytes[pos..pack_end].to_vec(), bytes[pack_end..].to_vec()))
}

/// @emoji 🧩️ Compose-only pack bridge (external technology).
impl ArtifactPack for serde_json::Value {
    async fn encode_pack_with(&self, _options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        Ok(pack_rt::encode_json_value(self).await)
    }
    async fn decode_pack_with(bytes: &[u8], _options: &PackDecodeOptions) -> Result<Self, PackError> {
        pack_rt::decode_json_value(bytes).await
    }
}

/// @emoji 🌱️ Pack counterpart of the schema-less `DslValue` escape hatch: delegates to `pack_rt`'s value bridge.
impl ArtifactPack for DslValue {
    async fn encode_pack_with(&self, _options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        Ok(pack_rt::encode_pack_value(self).await)
    }
    async fn decode_pack_with(bytes: &[u8], _options: &PackDecodeOptions) -> Result<Self, PackError> {
        pack_rt::decode_pack_value(bytes).await
    }
}

/// @emoji 🔀️ The closest `PackError` variant to "a text-format failure surfaced through a pack-facing
/// API" (e.g. `dsl_derive`'s generated `decode_pack_with`, whose `__dsl_from_record` step returns
/// `TextError`). A free function, not `impl From<TextError> for PackError`: both types are
/// re-exports of foreign crates (`dsl_core`/`pack_core`) through `vcs`, so a blanket `From` impl
/// here would violate the orphan rule — neither type is actually local to this crate.
pub async fn text_error_to_pack_error(error: TextError) -> PackError {
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
    // 🚫️async: E4 fn-pointer erasure-table thunk (R1(ii): ComposeFuture/IoFuture-shaped) — an
    // `async fn` item's pointer type is unnameable, so the table stores a plain `fn` that itself
    // returns a boxed future.
    pub compile_dsl: for<'a> fn(&'a str, &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(ArtifactPackFiles, String), VcsError>> + 'a>>,
    /// @emoji 📥️ `(pack bytes, spr bytes) -> (dsl text, ops text)` — the sanctioned human/agent
    /// LOGGING mirror, produced from the authoritative binary for schema-agnostic callers
    /// (`store_sync`'s `FolderEndpoint::Pack` write path) that never touch a concrete `P`/`Mutation`.
    // 🚫️async: E4 fn-pointer erasure-table thunk (R1(ii)) — see `compile_dsl`'s tag above.
    pub print_mirror: for<'a> fn(&'a [u8], &'a [u8]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ArtifactTextFiles, VcsError>> + 'a>>,
    /// @emoji 🧩️ One `MutationEnvelope` -> one printed `.ops` edit block (header line + indented
    /// op line), for `FolderTextStorage::append_ops`'s hot-path logging append — decodes the
    /// envelope's opaque `OpBinary` payload back into a concrete `Mutation` just long enough to
    /// print it, for schema-agnostic callers that otherwise never see a concrete op type.
    // 🚫️async: E4 fn-pointer erasure-table thunk (R1(ii)) — see `compile_dsl`'s tag above.
    pub edit_text_from_envelope: for<'a> fn(&'a crate::os_spr::MutationEnvelope) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, VcsError>> + 'a>>,
    /// Host-authoritative Emit apply: (pack, spr, encode_ops_vec) -> (pack, spr, ops text).
    // 🚫️async: E4 fn-pointer erasure-table thunk (R1(ii)) — see `compile_dsl`'s tag above.
    pub apply_ops_binary: for<'a> fn(&'a [u8], &'a [u8], &'a [u8]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(Vec<u8>, Vec<u8>, String), VcsError>> + 'a>>,
}

impl ArtifactCodec {
    /// @emoji 🏗️ Monomorphizes three non-capturing bridge functions for `(P, Mutation)` — each a
    /// genuine zero-sized `fn` item, coercible to a bare `fn` pointer — and pairs them with `schema`/
    /// `P::EXTENSION`. One call site per document kind (`register_document_codec_for_app`).
    pub async fn of<P, Mutation>(schema: impl Into<String>) -> Self
    where
        P: Clone + PartialEq + Serialize + DeserializeOwned + ArtifactDsl + ArtifactPack + Send + 'static,
        Mutation: self::Mutation<P> + PartialEq + Serialize + DeserializeOwned + OpText + OpBinary + Send + 'static,
    {
        // 🚫️async: E4 fn-pointer erasure-table thunk — VALUE goes into `ArtifactCodec::compile_dsl`
        // (`fn` slot, unnameable if `async fn`); wraps its real async body in `Box::pin` per R1(ii).
        fn compile_dsl_impl<'a, P, Mutation>(dsl: &'a str, ops: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(ArtifactPackFiles, String), VcsError>> + 'a>>
        where
            P: Clone + ArtifactDsl + ArtifactPack,
            Mutation: OpText + OpBinary + self::Mutation<P>,
        {
            Box::pin(async move {
                let parsed: ParsedDocumentText<P, Mutation> = parse_document_text(dsl, ops).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
                let pack_files = print_document_pack(&parsed.envelope).await?;
                let dsl_mirror = parsed.envelope.vcs.initial_snapshot.print_dsl();
                Ok((pack_files, dsl_mirror.await))
            })
        }

        // 🚫️async: E4 fn-pointer erasure-table thunk — see `compile_dsl_impl`'s tag above.
        fn print_mirror_impl<'a, P, Mutation>(pack: &'a [u8], spr: &'a [u8]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ArtifactTextFiles, VcsError>> + 'a>>
        where
            P: Clone + ArtifactDsl + ArtifactPack,
            Mutation: OpText + OpBinary + self::Mutation<P>,
        {
            Box::pin(async move {
                let parsed: ParsedDocumentText<P, Mutation> = parse_document_pack(pack, spr).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
                print_document_text(&parsed.envelope).await
            })
        }

        // 🚫️async: E4 fn-pointer erasure-table thunk — see `compile_dsl_impl`'s tag above.
        fn apply_ops_binary_impl<'a, P, Mutation>(pack: &'a [u8], spr: &'a [u8], ops_vec: &'a [u8]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(Vec<u8>, Vec<u8>, String), VcsError>> + 'a>>
        where
            P: Clone + Serialize + DeserializeOwned + ArtifactDsl + ArtifactPack,
            Mutation: OpText + OpBinary + self::Mutation<P>,
        {
            Box::pin(async move {
                if ops_vec.is_empty() {
                    if pack.is_empty() && spr.is_empty() {
                        return Ok((Vec::new(), Vec::new(), String::new()));
                    }
                    let parsed = parse_document_pack::<P, Mutation>(pack, spr).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
                    let files = print_document_pack(&parsed.envelope).await?;
                    return Ok((files.pack, files.spr, files.ops));
                }
                let op_blobs = crate::os_spr::decode_ops_vec(ops_vec).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
                let mut mutations: Vec<Mutation> = Vec::with_capacity(op_blobs.len());
                for bytes in &op_blobs {
                    mutations.push(Mutation::decode_op(bytes).await.map_err(|error| VcsError::Deserialize(error.to_string()))?);
                }
                let mut store = if pack.is_empty() && spr.is_empty() {
                    return Err(VcsError::Deserialize("apply_ops_binary: lane has no pack+spr baseline".into()));
                } else {
                    let parsed = parse_document_pack::<P, Mutation>(pack, spr).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
                    let (applied, redo) = match &parsed.envelope.cursor {
                        Some(cursor) => (cursor.applied_edit_ids.clone(), cursor.redo_edit_ids.clone()),
                        None => (parsed.envelope.vcs.edits.iter().map(|edit| edit.id.clone()).collect(), Vec::new()),
                    };
                    let envelope = parsed.envelope;
                    let mut store = ArtifactStore::new(envelope.clone()).await?;
                    store.reset(envelope, applied, redo).await?;
                    store
                };
                store.dispatch(ArtifactCommand::Apply { mutations, description: None }).await?;
                let files = print_document_pack(store.envelope().await).await?;
                Ok((files.pack, files.spr, files.ops))
            })
        }

        // 🚫️async: E4 fn-pointer erasure-table thunk — see `compile_dsl_impl`'s tag above.
        fn edit_text_from_envelope_impl<'a, P, Mutation>(envelope: &'a crate::os_spr::MutationEnvelope) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, VcsError>> + 'a>>
        where
            Mutation: OpText + OpBinary,
        {
            Box::pin(async move {
                let edit = edit_from_operation_envelope::<Mutation>(envelope).await?;
                print_edit_lines(&edit).await
            })
        }

        Self {
            schema: schema.into(),
            extension: P::envelope_id().await.into(),
            // 🌀️ `schema_hash` is async; `Option::map`'s closure is sync (R10 shape 1), so it's
            // written as an explicit match instead.
            pack_schema_hash: match P::record_spec().await {
                Some(spec) => crate::os_pack::schema_hash(&spec).await,
                None => [0u8; 32],
            },
            compile_dsl: compile_dsl_impl::<P, Mutation>,
            print_mirror: print_mirror_impl::<P, Mutation>,
            edit_text_from_envelope: edit_text_from_envelope_impl::<P, Mutation>,
            apply_ops_binary: apply_ops_binary_impl::<P, Mutation>,
        }
    }
}

static DOCUMENT_CODEC_REGISTRY: std::sync::OnceLock<std::sync::RwLock<std::collections::BTreeMap<String, ArtifactCodec>>> = std::sync::OnceLock::new();

async fn document_codec_registry() -> &'static std::sync::RwLock<std::collections::BTreeMap<String, ArtifactCodec>> {
    DOCUMENT_CODEC_REGISTRY.get_or_init(|| std::sync::RwLock::new(std::collections::BTreeMap::new()))
}

/// ⚠️ A document schema already has a codec owner. The established codec is never replaced.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentCodecRegistryConflict {
    pub schema: String,
}

/// ⚠️ Document-codec registration cannot replace an owner or use an unavailable registry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DocumentCodecRegistryError {
    Conflict(DocumentCodecRegistryConflict),
    Unavailable,
}

impl std::fmt::Display for DocumentCodecRegistryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Conflict(error) => write!(formatter, "document codec registration conflicts for schema {}", error.schema),
            Self::Unavailable => formatter.write_str("document codec registry unavailable"),
        }
    }
}

impl std::error::Error for DocumentCodecRegistryError {}

fn same_document_codec(left: &ArtifactCodec, right: &ArtifactCodec) -> bool {
    left.schema == right.schema
        && left.extension == right.extension
        && left.pack_schema_hash == right.pack_schema_hash
        && std::ptr::fn_addr_eq(left.compile_dsl, right.compile_dsl)
        && std::ptr::fn_addr_eq(left.print_mirror, right.print_mirror)
        && std::ptr::fn_addr_eq(left.edit_text_from_envelope, right.edit_text_from_envelope)
        && std::ptr::fn_addr_eq(left.apply_ops_binary, right.apply_ops_binary)
}

async fn validate_document_codecs(registry: &std::collections::BTreeMap<String, ArtifactCodec>, codecs: &[ArtifactCodec]) -> Result<(), DocumentCodecRegistryError> {
    let mut proposed: std::collections::BTreeMap<&str, &ArtifactCodec> = std::collections::BTreeMap::new();
    for codec in codecs {
        match proposed.get(codec.schema.as_str()) {
            Some(existing) if same_document_codec(existing, codec) => {}
            Some(_) => return Err(DocumentCodecRegistryError::Conflict(DocumentCodecRegistryConflict { schema: codec.schema.clone() })),
            None => {
                proposed.insert(codec.schema.as_str(), codec);
            }
        }
    }
    for (schema, codec) in proposed {
        match registry.get(schema) {
            Some(existing) if same_document_codec(existing, codec) => {}
            Some(_) => return Err(DocumentCodecRegistryError::Conflict(DocumentCodecRegistryConflict { schema: schema.to_string() })),
            None => {}
        }
    }
    Ok(())
}

/// 🔬️ Verifies document codecs against all established schemas without mutating the registry.
#[must_use]
pub async fn preflight_document_codecs(codecs: &[ArtifactCodec]) -> Result<(), DocumentCodecRegistryError> {
    let assembly = begin_artifact_assembly().await.map_err(|_| DocumentCodecRegistryError::Unavailable)?;
    preflight_document_codecs_in_assembly(&assembly, codecs).await
}

/// 🔬️ Verifies document codecs while one artifact assembly owns the shared publication barrier.
#[must_use]
pub async fn preflight_document_codecs_in_assembly(_assembly: &ArtifactAssemblyTransaction, codecs: &[ArtifactCodec]) -> Result<(), DocumentCodecRegistryError> {
    let registry = document_codec_registry().await.read().map_err(|_| DocumentCodecRegistryError::Unavailable)?;
    validate_document_codecs(&registry, codecs).await
}

/// 📝️ Registers one schema codec exactly once. Collisions fail deterministically before any
/// replacement can occur, so registration order never changes decoding behavior.
#[must_use]
pub async fn register_document_codec(codec: ArtifactCodec) -> Result<(), DocumentCodecRegistryError> {
    register_document_codecs(vec![codec]).await
}

/// 📝️ Registers document codecs only when every descriptor and executable is conflict-free.
#[must_use]
pub async fn register_document_codecs(codecs: Vec<ArtifactCodec>) -> Result<(), DocumentCodecRegistryError> {
    let assembly = begin_artifact_assembly().await.map_err(|_| DocumentCodecRegistryError::Unavailable)?;
    register_document_codecs_in_assembly(&assembly, codecs).await
}

/// 📝️ Publishes preflighted document codecs while one artifact assembly owns the shared barrier.
#[must_use]
pub async fn register_document_codecs_in_assembly(_assembly: &ArtifactAssemblyTransaction, codecs: Vec<ArtifactCodec>) -> Result<(), DocumentCodecRegistryError> {
    let mut registry = document_codec_registry().await.write().map_err(|_| DocumentCodecRegistryError::Unavailable)?;
    validate_document_codecs(&registry, &codecs).await?;
    for codec in codecs {
        if !registry.contains_key(&codec.schema) {
            registry.insert(codec.schema.clone(), codec);
        }
    }
    Ok(())
}

/// 🔎️ Looks up the codec registered for `schema`, if any.
#[must_use]
pub async fn document_codec(schema: &str) -> Result<Option<ArtifactCodec>, DocumentCodecRegistryError> {
    let registry = document_codec_registry().await.read().map_err(|_| DocumentCodecRegistryError::Unavailable)?;
    Ok(registry.get(schema).cloned())
}

/// @emoji 📜️ Reads the document schema id from an encoded `.spr` history log.
pub async fn lane_schema_from_spr(spr: &[u8]) -> Option<String> {
    if spr.is_empty() {
        return None;
    }
    crate::os_spr::decode_history(spr, &crate::os_spr::DecodeOptions::default()).await.ok().map(|log| log.schema).filter(|schema| !schema.is_empty())
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

static DIALECT_MIGRATION_REGISTRY: std::sync::OnceLock<std::sync::RwLock<std::collections::BTreeMap<(crate::os_io::ArtifactDialect, crate::os_io::ArtifactDialect), DialectMigration>>> = std::sync::OnceLock::new();

async fn dialect_migration_registry() -> &'static std::sync::RwLock<std::collections::BTreeMap<(crate::os_io::ArtifactDialect, crate::os_io::ArtifactDialect), DialectMigration>> {
    DIALECT_MIGRATION_REGISTRY.get_or_init(|| std::sync::RwLock::new(std::collections::BTreeMap::new()))
}

/// ⚠️ Dialect-migration registration cannot overwrite an owner or cross artifact kinds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DialectMigrationRegistryError {
    CrossArtifactKind { from: crate::os_io::ArtifactDialect, to: crate::os_io::ArtifactDialect },
    Conflict { from: crate::os_io::ArtifactDialect, to: crate::os_io::ArtifactDialect },
    Unavailable,
}

impl std::fmt::Display for DialectMigrationRegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CrossArtifactKind { from, to } => write!(f, "dialect migration crosses artifact kinds from {from:?} to {to:?}"),
            Self::Conflict { from, to } => write!(f, "dialect migration conflict from {from:?} to {to:?}"),
            Self::Unavailable => write!(f, "dialect migration registry unavailable"),
        }
    }
}
impl std::error::Error for DialectMigrationRegistryError {}

/// 🚫️ A migration was unavailable or the registered executable rejected its input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DialectMigrationError {
    Missing { from: crate::os_io::ArtifactDialect, to: crate::os_io::ArtifactDialect },
    Unavailable,
    Rejected(String),
}

async fn same_dialect_migration(left: &DialectMigration, right: &DialectMigration) -> bool {
    left.from == right.from && left.to == right.to && left.lossless == right.lossless && std::ptr::fn_addr_eq(left.migrate_pack, right.migrate_pack)
}

async fn validate_dialect_migrations(registry: &std::collections::BTreeMap<(crate::os_io::ArtifactDialect, crate::os_io::ArtifactDialect), DialectMigration>, migrations: &[DialectMigration]) -> Result<(), DialectMigrationRegistryError> {
    let mut proposed: std::collections::BTreeMap<(crate::os_io::ArtifactDialect, crate::os_io::ArtifactDialect), &DialectMigration> = std::collections::BTreeMap::new();
    for migration in migrations {
        if migration.from.artifact_kind != migration.to.artifact_kind {
            return Err(DialectMigrationRegistryError::CrossArtifactKind { from: migration.from.clone(), to: migration.to.clone() });
        }
        let key = (migration.from.clone(), migration.to.clone());
        match proposed.get(&key) {
            Some(existing) if same_dialect_migration(existing, migration).await => {}
            Some(_) => return Err(DialectMigrationRegistryError::Conflict { from: key.0, to: key.1 }),
            None => {
                proposed.insert(key, migration);
            }
        }
    }
    for (key, migration) in proposed {
        match registry.get(&key) {
            Some(existing) if same_dialect_migration(existing, migration).await => {}
            Some(_) => return Err(DialectMigrationRegistryError::Conflict { from: key.0, to: key.1 }),
            None => {}
        }
    }
    Ok(())
}

/// 🔬️ Verifies a migration set against established dialect pairs without mutation.
#[must_use]
pub async fn preflight_dialect_migrations(migrations: &[DialectMigration]) -> Result<(), DialectMigrationRegistryError> {
    let assembly = begin_artifact_assembly().await.map_err(|_| DialectMigrationRegistryError::Unavailable)?;
    preflight_dialect_migrations_in_assembly(&assembly, migrations).await
}

/// 🔬️ Verifies migrations while one artifact assembly owns the shared publication barrier.
#[must_use]
pub async fn preflight_dialect_migrations_in_assembly(_assembly: &ArtifactAssemblyTransaction, migrations: &[DialectMigration]) -> Result<(), DialectMigrationRegistryError> {
    let registry = dialect_migration_registry().await.read().map_err(|_| DialectMigrationRegistryError::Unavailable)?;
    validate_dialect_migrations(&registry, migrations).await
}

/// 📝️ Registers a migration only when its full descriptor and executable identity match.
#[must_use]
pub async fn register_dialect_migration(migration: DialectMigration) -> Result<(), DialectMigrationRegistryError> {
    register_dialect_migrations(vec![migration]).await
}

/// 📝️ Registers migrations only when every candidate pair is conflict-free.
#[must_use]
pub async fn register_dialect_migrations(migrations: Vec<DialectMigration>) -> Result<(), DialectMigrationRegistryError> {
    let assembly = begin_artifact_assembly().await.map_err(|_| DialectMigrationRegistryError::Unavailable)?;
    register_dialect_migrations_in_assembly(&assembly, migrations).await
}

/// 📝️ Publishes preflighted migrations while one artifact assembly owns the shared barrier.
#[must_use]
pub async fn register_dialect_migrations_in_assembly(_assembly: &ArtifactAssemblyTransaction, migrations: Vec<DialectMigration>) -> Result<(), DialectMigrationRegistryError> {
    let mut registry = dialect_migration_registry().await.write().map_err(|_| DialectMigrationRegistryError::Unavailable)?;
    validate_dialect_migrations(&registry, &migrations).await?;
    for migration in migrations {
        let key = (migration.from.clone(), migration.to.clone());
        if !registry.contains_key(&key) {
            registry.insert(key, migration);
        }
    }
    Ok(())
}

/// @emoji 🔁️ Looks up the exact `(from, to)` migration and runs its `migrate_pack` over
/// `pack_bytes`, or a clear `Err` naming both dialect coordinates when none is registered.
#[must_use]
pub async fn migrate_document(from: &crate::os_io::ArtifactDialect, to: &crate::os_io::ArtifactDialect, pack_bytes: &[u8]) -> Result<Vec<u8>, DialectMigrationError> {
    let registry = dialect_migration_registry().await.read().map_err(|_| DialectMigrationError::Unavailable)?;
    let migration = registry.get(&(from.clone(), to.clone())).ok_or_else(|| DialectMigrationError::Missing { from: from.clone(), to: to.clone() })?;
    (migration.migrate_pack)(pack_bytes).map_err(DialectMigrationError::Rejected)
}

/// 🧷️ All writable store registries held before an artifact assembly can publish anything.
pub struct ArtifactAssemblyStoreRegistryGuards {
    document_codecs: std::sync::RwLockWriteGuard<'static, std::collections::BTreeMap<String, ArtifactCodec>>,
    dialect_migrations: std::sync::RwLockWriteGuard<'static, std::collections::BTreeMap<(crate::os_io::ArtifactDialect, crate::os_io::ArtifactDialect), DialectMigration>>,
}

/// 🚫️ A staged store registry assembly cannot be preflighted or committed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArtifactAssemblyStoreRegistryError {
    DocumentCodec(DocumentCodecRegistryError),
    DialectMigration(DialectMigrationRegistryError),
}

impl std::fmt::Display for ArtifactAssemblyStoreRegistryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DocumentCodec(error) => error.fmt(formatter),
            Self::DialectMigration(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for ArtifactAssemblyStoreRegistryError {}

/// 🧷️ Acquires every store registry write lock before an assembly validates or mutates state.
#[must_use]
pub async fn acquire_artifact_assembly_store_registry_guards(_assembly: &ArtifactAssemblyTransaction) -> Result<ArtifactAssemblyStoreRegistryGuards, ArtifactAssemblyStoreRegistryError> {
    let document_codecs = document_codec_registry().await.write().map_err(|_| ArtifactAssemblyStoreRegistryError::DocumentCodec(DocumentCodecRegistryError::Unavailable))?;
    let dialect_migrations = dialect_migration_registry().await.write().map_err(|_| ArtifactAssemblyStoreRegistryError::DialectMigration(DialectMigrationRegistryError::Unavailable))?;
    Ok(ArtifactAssemblyStoreRegistryGuards { document_codecs, dialect_migrations })
}

/// 🔬️ Verifies all staged store rows while every affected write lock is already held.
#[must_use]
pub async fn preflight_artifact_assembly_store_registry_guards(guards: &ArtifactAssemblyStoreRegistryGuards, document_codecs: &[ArtifactCodec], dialect_migrations: &[DialectMigration]) -> Result<(), ArtifactAssemblyStoreRegistryError> {
    validate_document_codecs(&guards.document_codecs, document_codecs).await.map_err(ArtifactAssemblyStoreRegistryError::DocumentCodec)?;
    validate_dialect_migrations(&guards.dialect_migrations, dialect_migrations).await.map_err(ArtifactAssemblyStoreRegistryError::DialectMigration)
}

/// 📌️ Publishes rows proven safe by `preflight_artifact_assembly_store_registry_guards`.
pub async fn commit_artifact_assembly_store_registry_guards(guards: &mut ArtifactAssemblyStoreRegistryGuards, document_codecs: Vec<ArtifactCodec>, dialect_migrations: Vec<DialectMigration>) {
    for codec in document_codecs {
        guards.document_codecs.entry(codec.schema.clone()).or_insert(codec);
    }
    for migration in dialect_migrations {
        let key = (migration.from.clone(), migration.to.clone());
        guards.dialect_migrations.entry(key).or_insert(migration);
    }
}
//#endregion 🔖️DialectMigration

//#region 🔖️MergeHelpers
/// @emoji 🌳️ Walks `checkpoint_id`'s ancestor chain via `parent_id` back to the root, nearest-first
/// (`checkpoint_id` itself is the first entry). Cycle-guarded (a malformed/adversarial parent chain
/// stops instead of looping forever) — every well-formed chain built by `reconcile_alternative`/
/// `CommitCheckpoint` is already acyclic, this is defense in depth, not a documented invariant break.
async fn checkpoint_ancestors<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, checkpoint_id: &str) -> Vec<String> {
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
pub async fn merge_base<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, a: &str, b: &str) -> Option<String> {
    let ancestors_a: HashSet<String> = checkpoint_ancestors(envelope, a).await.into_iter().collect();
    checkpoint_ancestors(envelope, b).await.into_iter().find(|id| ancestors_a.contains(id))
}

pub async fn reconcile_alternative<P, Mutation>(envelope: &mut ArtifactEnvelope<P, Mutation>, alternative_name: &str, checkpoint_message: Option<String>, authors: Vec<Author>) -> Result<String, VcsError>
where
    P: Clone + Serialize + DeserializeOwned,
    Mutation: Clone + Serialize + DeserializeOwned,
{
    if envelope.vcs.checkpoints.is_empty() {
        return Err(VcsError::NoCheckpoint);
    }
    let checkpoint_id = envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone()).ok_or(VcsError::NoCheckpoint)?;
    // 🌀️ `alternative_id` is both cloned and returned below — a future can only be awaited once
    // (R10 shape 2), so it is resolved to a plain `String` here.
    let alternative_id = mint_alternative_id(alternative_name, &[checkpoint_id.clone()]).await;
    envelope.vcs.alternatives.push(Alternative { id: alternative_id.clone(), name: alternative_name.to_string(), checkpoint_ids: vec![checkpoint_id] });
    if let Some(message) = checkpoint_message {
        let change = Change { id: mint_change_id(&[], Some(&message)).await, edit_ids: Vec::new(), description: Some(message), saved_at: now_iso().await };
        let parent = envelope.vcs.checkpoints.last();
        let parent_id = parent.map(|checkpoint| checkpoint.id.clone());
        let mut change_ids = parent.map(|checkpoint| checkpoint.change_ids.clone()).unwrap_or_default();
        change_ids.push(change.id.clone());
        envelope.vcs.changes.push(change);
        // 🌀️ Same reasoning as `alternative_id` above — resolved once, before it is borrowed AND moved.
        let timestamp = now_iso().await;
        let checkpoint_message = Some("reconciled".to_string());
        // 🎯️ `&[]`: reconcile-alternative checkpoints carry no composition pins yet — the
        // `CompositionCoordinator` that populates real `CompositionPin`s on commit is a later wave.
        let id = content_addressed_checkpoint_id(parent_id.as_deref(), &change_ids, &envelope.vcs.changes, checkpoint_message.as_deref(), &authors, &timestamp, &[]).await;
        envelope.vcs.checkpoints.push(Checkpoint { id, change_ids, parent_id, authors, message: checkpoint_message, timestamp, composition_pins: Vec::new() });
    }
    Ok(alternative_id)
}
//#endregion 🔖️MergeHelpers

//#region 🔖️Config
pub type ConfigEnvelope<C, ConfigMutation> = ArtifactEnvelope<C, ConfigMutation>;
pub type ConfigStore<C, ConfigMutation> = ArtifactStore<C, ConfigMutation>;

pub async fn create_config_envelope<C, ConfigMutation>(schema: &str, id: &str, initial_snapshot: C, backbone: Option<ArtifactBackboneRef>) -> ConfigEnvelope<C, ConfigMutation>
where
    C: Clone,
{
    create_document_envelope(schema, id, initial_snapshot, backbone).await
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
            async fn apply(&self, _base: &$ty) -> ::protocol::MutationApplyResult<$ty> {
                Ok(self.clone())
            }
            async fn absorb(&mut self, other: Self) {
                *self = other;
            }
        }
    };
}

// config_spec_* removed — ConfigSpec is UI (framework-core); avoids kernel↔core cycle
//#endregion 🔖️Config

//#region 🔖️Materialize
pub async fn create_document_envelope<P, Mutation>(schema: &str, id: &str, initial_snapshot: P, backbone: Option<ArtifactBackboneRef>) -> ArtifactEnvelope<P, Mutation>
where
    P: Clone,
{
    ArtifactEnvelope {
        schema: schema.into(),
        id: id.into(),
        vcs: ArtifactVcs { initial_snapshot, edits: Vec::new(), changes: Vec::new(), checkpoints: Vec::new(), alternatives: Vec::new() },
        backbone,
        active_alternative_id: None,
        cursor: Some(ArtifactCursor { applied_edit_ids: Vec::new(), redo_edit_ids: Vec::new(), checkpoint_id: None }),
        dialect: None,
        migrated_from: None,
        owner: None,
        lanes: std::collections::BTreeMap::new(),
        edit_messages: Vec::new(),
        conflicts: Vec::new(),
    }
}

pub async fn edit_ids_for_changes<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, change_ids: &[String]) -> Vec<String>
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

/// @emoji 🔂️ Full raw fold of `initial_snapshot` over every `applied_edit_ids` edit's `forwards`.
/// `crate::os_spr::command::Mutation::reconcile` and its diagnostic-bag-returning twin
/// `materialize_document_snapshot_with_conflicts` are GONE
/// (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6/C10) — concurrent-merge
/// arbitration is now `ArtifactStore::ingest_remote`/`resolve_conflict`'s job against
/// `📡️spr/⚔️conflict`'s first-class `Conflict`/`MergeReport`, not a post-materialization hook here.
pub async fn materialize_document_snapshot<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, applied_edit_ids: &[String]) -> Result<P, VcsError>
where
    P: Clone,
    Mutation: self::Mutation<P>,
{
    let mut snapshot = envelope.vcs.initial_snapshot.clone();
    for edit_id in applied_edit_ids {
        let edit = envelope.vcs.edits.iter().find(|entry| entry.id == *edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
        for operation in &edit.forwards {
            snapshot = apply_mutation(&snapshot, operation).await?.0;
        }
    }
    Ok(snapshot)
}

/// 🕰️ Single timestamp source for `Edit.started_at`/`Checkpoint.timestamp` — re-exported so
/// callers outside this crate (e.g. the framework session command log) stamp entries in the
/// exact same format.
pub async fn now_iso() -> String {
    format!("{}", now_ms().await)
}

async fn now_ms() -> u64 {
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

/// 🩺️ Defense in depth, NOT the fix for the id-domain discontinuity itself (see {@link
/// stamp_primary_operation_identity}, which is): every id this returns is about to become a
/// `Change.edit_ids` entry, so if one isn't backed by a real `envelope.vcs.edits` entry, a much
/// louder and more specific `debug_assert!` here beats discovering it two calls later as
/// `validate_durable_history`'s cryptic "invalid edit reference". Deliberately does NOT filter the
/// id out in release builds — silently shrinking the checkpoint would drop a real edit out of
/// history without a trace, exactly the failure mode `checkpoint_after_ingesting_a_remote_edit_
/// stays_valid_once_the_sender_s_own_checkpoint_snapshot_arrives`'s doc comment warns against.
async fn uncommitted_edit_ids<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, applied_edit_ids: &[String]) -> Vec<String>
where
    Mutation: Clone,
    P: Clone,
{
    let committed: HashSet<String> = envelope.vcs.changes.iter().flat_map(|change| change.edit_ids.iter().cloned()).collect();
    applied_edit_ids
        .iter()
        .filter(|id| !committed.contains(*id))
        .cloned()
        .inspect(|id| debug_assert!(envelope.vcs.edits.iter().any(|edit| edit.id == *id), "uncommitted_edit_ids: {id} is applied but has no backing envelope.vcs.edits entry — the checkpoint about to reference it will fail validate_durable_history"))
        .collect()
}

/// 🪪️ For a SINGLE-op edit only, overrides its sole `mutation_meta[0].mutation_id` with the edit's
/// own real `id` — preserving a SINGLE global edit-id domain across the wire for the dominant case
/// (one atomic user action, one forward op). `replay_mutations` always pre-fills
/// `mutation_meta[i].mutation_id` (`mutation.mutation_id().unwrap_or_else(mint_mutation_id` of the
/// raw op bytes), so by the time an `Edit` exists it is never absent — a plain `is_none()` guard
/// would never fire. `mutation_id()` has exactly one implementor repo-wide (the
/// `crate::os_spr::Mutation<P>` trait's own `None` default — confirmed zero overrides), so op 0
/// always carries `mint_mutation_id`'s CONTENT hash of the raw op bytes: an identity that shares
/// nothing with `edit.id` (`mint_edit_id`'s actor+sequence+full-fingerprint hash) by construction of
/// the two different formulas — not merely "sometimes absent". For a single-op edit, op 0 IS the
/// edit's entire wire identity (`crate::os_spr::mutation_ids_for_edit`'s only entry for it), so
/// overriding it to the edit's real id means `edit_from_operation_envelope` reconstructs an EXACT
/// id match on every receiving store, instead of a content-hash id that never existed locally and
/// cannot resolve a `Change`/`Checkpoint` minted (by the sender, or a third store relaying its
/// snapshot) against the edit's real id. MUST NOT extend to a genuine multi-op edit (`forwards.len()
/// > 1`): `ingest_remote` reconstructs one local single-forward `Edit` per WIRE op, so giving op 0
/// the parent edit's bare id would make that one-forward phantom edit collide, id-for-id, with the
/// real N-forward edit once its full snapshot later arrives — `merge_remote_snapshot`'s
/// `same_edit_operation_identities_and_payloads` check (rightly) rejects the shape mismatch as
/// `"remote history conflicts with established edit"` (caught by
/// `operations_then_snapshot_partitions_a_multi_forward_ledger_by_wire_edit` while developing this
/// fix). Multi-op ops keep their content-hash ids, unchanged — they were never at risk of the
/// single-op collision this closes, since no other store ever knew them under the edit's bare id.
/// Called from every fresh-edit constructor (`apply_command`, `amend_command`'s fresh branch); an
/// `AmendLast` extension always appends beyond index 0 of an already multi-op edit, so it never
/// qualifies. See
/// `checkpoint_after_ingesting_a_remote_edit_stays_valid_once_the_sender_s_own_checkpoint_snapshot_arrives`
/// for the regression this fixes.
async fn stamp_primary_operation_identity<Mutation>(edit: &mut Edit<Mutation>) {
    if edit.forwards.len() != 1 {
        return;
    }
    if let Some(first) = edit.mutation_meta.first_mut() {
        first.mutation_id = Some(MutationId(edit.id.clone()));
    }
}

/// 🕊️ `CommitCheckpoint`'s "nothing uncommitted" rejection message — a `pub const`, not an inline
/// literal, so a caller (e.g. `VcsArtifactApp::dispatch_action` in `🔌️plugin/🦀️component.rs`) can
/// recognize this SPECIFIC, benign outcome by value rather than duplicating the string. Genuinely
/// benign, not just here to silence an error: after a `CommitCheckpoint` dispatch's own `pump()`
/// absorbs a peer's checkpoint that already covers every edit this store has applied (a real,
/// expected outcome of full convergence — see
/// `checkpoint_after_ingesting_a_remote_edit_stays_valid_once_the_sender_s_own_checkpoint_snapshot_arrives`'s
/// sibling scenario), there is truly nothing left to commit — the same "requested but nothing to
/// do" shape as `VcsError::NothingToUndo`/`NothingToRedo`, just without a dedicated variant.
pub const EMPTY_CHECKPOINT_MESSAGE: &str = "cannot create an empty checkpoint";

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
        sequence: i32,
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
    /// @emoji 🔙️ One edit's complete inverse sequence, encoded with the operation's own text grammar.
    Inverse { edit: String, ops: Vec<String> },
    /// @emoji 🪪️ One authoritative metadata record for a forward operation.
    Metadata { edit: String, index: u32, data: String },
    /// @emoji 📨️ One durable diagnostic ledger entry, explicitly owned by an edit.
    Message { edit: String, data: String },
    /// @emoji ⚔️ One first-class conflict, including its content-addressed identity and lifecycle.
    Conflict { data: String },
}

//#region 🔖️OpCodec
/// 🎞️ Handcrafted OpText (P6).
impl OpText for OpsHeaderLine {
    async fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = crate::os_dsl::parse(line, &spec_fn(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline }).await?;
                return <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record).await;
            }
        }
        Err(crate::os_dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self).await;
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline).await
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl OpBinary for OpsHeaderLine {
    async fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self).await;
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1)();
        let body = crate::os_pack::encode_record_body(&spec, &record, &PackEncodeOptions::default()).await.map_err(crate::os_spr::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        crate::os_pack::write_varint_u64(&mut out, ordinal as u64).await;
        out.extend_from_slice(&body);
        Ok(out)
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = crate::os_pack::ByteReader::new(bytes).await;
        let format = reader.read_u8().await?;
        if format != OP_BINARY_FORMAT {
            return Err(crate::os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64().await?;
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = spec_fn();
        let body = &bytes[reader.position().await..];
        let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &PackDecodeOptions::default()).await.map_err(crate::os_spr::ProtocolError::from)?;
        let record_offset = reader.position().await as u64;
        <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record).await.map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "op record", offset: record_offset, detail: error.to_string() })
    }
}
//#endregion 🔖️OpCodec

//#endregion 🔖️OpsHeaderGrammar

/// @emoji 📤️ Prints one edit as an `edit ...` header line followed by one two-space-indented
/// `print_op` line per forward operation — the hot-path append unit for the op log. Its matching
/// inverse and authoritative metadata records are emitted by `print_ops_log` immediately after it.
pub async fn print_edit_lines<Mutation: OpText>(edit: &Edit<Mutation>) -> Result<String, VcsError> {
    let header = OpsHeaderLine::Edit {
        id: edit.id.clone(),
        sequence: edit.sequence_number,
        started: edit.started_at.clone(),
        actor: edit.actor.clone(),
        finished: edit.finished_at.clone(),
        key: edit.coalesce_key.clone(),
        description: edit.description.clone(),
    };
    let mut out = header.print_op().await;
    out.push('\n');
    for operation in &edit.forwards {
        let printed = operation.print_op().await;
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
/// their own initial-snapshot encoding. Every replay-critical value is explicit: forward and
/// inverse operations, operation metadata, message ledger, conflicts, and cursor.
async fn print_ops_log<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>) -> Result<String, VcsError>
where
    Mutation: OpText,
{
    let mut ops = String::new();
    ops.push_str(&OpsHeaderLine::Doc { id: envelope.id.clone(), schema: envelope.schema.clone() }.print_op().await);
    ops.push('\n');
    for edit in &envelope.vcs.edits {
        ops.push_str(&print_edit_lines(edit).await?);
        if edit.mutation_meta.len() != edit.forwards.len() {
            return Err(VcsError::ValidationFailed(format!("edit {} has {} metadata entries for {} forward operations", edit.id, edit.mutation_meta.len(), edit.forwards.len())));
        }
        let mut inverse: Vec<String> = Vec::with_capacity(edit.inverse.len());
        for operation in &edit.inverse {
            let text = operation.print_op().await;
            if text.contains('\n') {
                return Err(VcsError::Serialize("op-text print_op must not contain a newline".into()));
            }
            inverse.push(text);
        }
        ops.push_str(&OpsHeaderLine::Inverse { edit: edit.id.clone(), ops: inverse }.print_op().await);
        ops.push('\n');
        for (index, meta) in edit.mutation_meta.iter().enumerate() {
            let data = serde_json::to_string(meta).map_err(|error| VcsError::Serialize(error.to_string()))?;
            ops.push_str(&OpsHeaderLine::Metadata { edit: edit.id.clone(), index: index as u32, data }.print_op().await);
            ops.push('\n');
        }
    }
    for change in &envelope.vcs.changes {
        let header = OpsHeaderLine::Change { id: change.id.clone(), saved: change.saved_at.clone(), edits: change.edit_ids.clone(), description: change.description.clone() };
        ops.push_str(&header.print_op().await);
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
        ops.push_str(&header.print_op().await);
        ops.push('\n');
    }
    for alternative in &envelope.vcs.alternatives {
        let header = OpsHeaderLine::Alternative { id: alternative.id.clone(), name: alternative.name.clone(), checkpoints: alternative.checkpoint_ids.clone() };
        ops.push_str(&header.print_op().await);
        ops.push('\n');
    }
    if let Some(active_id) = &envelope.active_alternative_id {
        ops.push_str(&OpsHeaderLine::Active { id: active_id.clone() }.print_op().await);
        ops.push('\n');
    }
    let cursor = envelope.cursor.as_ref().ok_or_else(|| VcsError::ValidationFailed("text persistence requires an explicit cursor".to_string()))?;
    let header = OpsHeaderLine::Cursor { applied: cursor.applied_edit_ids.clone(), redo: cursor.redo_edit_ids.clone(), checkpoint: cursor.checkpoint_id.clone() };
    ops.push_str(&header.print_op().await);
    ops.push('\n');
    for entry in &envelope.edit_messages {
        if !envelope.vcs.edits.iter().any(|edit| edit.id == entry.edit_id) {
            return Err(VcsError::ValidationFailed(format!("message ledger references unknown edit {}", entry.edit_id)));
        }
        for message in &entry.messages {
            let data = serde_json::to_string(message).map_err(|error| VcsError::Serialize(error.to_string()))?;
            ops.push_str(&OpsHeaderLine::Message { edit: entry.edit_id.clone(), data }.print_op().await);
            ops.push('\n');
        }
    }
    for conflict in &envelope.conflicts {
        let data = serde_json::to_string(conflict).map_err(|error| VcsError::Serialize(error.to_string()))?;
        ops.push_str(&OpsHeaderLine::Conflict { data }.print_op().await);
        ops.push('\n');
    }
    Ok(ops)
}

/// @emoji 📤️ Prints the full textual VCS document: the DSL text (initial snapshot) and the complete
/// op log (`doc` header, every edit, inverse/meta/message/conflict records, and explicit cursor).
/// Replaces the JSON envelope as the canonical persisted form.
pub async fn print_document_text<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>) -> Result<ArtifactTextFiles, VcsError>
where
    P: ArtifactDsl,
    Mutation: OpText,
{
    let dsl = envelope.vcs.initial_snapshot.print_dsl();
    let ops = print_ops_log(envelope).await?;
    Ok(ArtifactTextFiles { dsl: dsl.await, ops })
}

/// @emoji 🎞️ `crate::os_spr::UndoPolicy` ordinal, matching `HistoryOpMeta.undo_policy`'s wire shape —
/// distinct from `undo_policy_ordinal` above, which maps THIS crate's `ArtifactCommand`-facing
/// `UndoPolicy` (currently `semio_framework::UndoPolicy`; the two enums have identical
/// variants and will merge in the kernel-unification wave, see `protocol_core`'s own doc note).
// 🚫️async: E1 pure ordinal mapping, consumed by `history_op_meta_from_operation_meta` which is
// itself consumed only through `Iterator::map` fn-item arguments — see R9.
fn protocol_undo_policy_ordinal(policy: UndoPolicy) -> u8 {
    match policy {
        UndoPolicy::ExactBaseOnly => 0,
        UndoPolicy::TransformAgainstConcurrent => 1,
        UndoPolicy::SemanticUndo => 2,
        UndoPolicy::CompensatingAction => 3,
    }
}

// 🚫️async: E1 pure ordinal mapping, consumed by `mutation_meta_from_history_op_meta` — see R9.
fn protocol_undo_policy_from_ordinal(ordinal: u8) -> UndoPolicy {
    match ordinal {
        1 => UndoPolicy::TransformAgainstConcurrent,
        2 => UndoPolicy::SemanticUndo,
        3 => UndoPolicy::CompensatingAction,
        _ => UndoPolicy::ExactBaseOnly,
    }
}

// 🚫️async: E1 pure field-copy, consumed only through `Iterator::map` fn-item arguments — see R9.
fn history_message_from_mutation_message(message: &crate::os_spr::MutationMessage) -> crate::os_spr::history::HistoryMessage {
    crate::os_spr::history::HistoryMessage { level: message.level.as_u8(), code: message.code.0.clone(), message: message.message.clone(), target: message.target.clone(), op_index: message.op_index }
}

// 🚫️async: E1 pure field-copy, consumed only through `Iterator::map` fn-item arguments — see R9.
fn mutation_message_from_history_message(message: crate::os_spr::history::HistoryMessage) -> Result<crate::os_spr::MutationMessage, String> {
    let level = crate::os_dsl::Severity::from_u8(message.level).ok_or_else(|| format!("unknown mutation message severity {}", message.level))?;
    Ok(crate::os_spr::MutationMessage { level, code: crate::os_dsl::FaultCode(message.code), message: message.message, target: message.target, op_index: message.op_index })
}

// 🚫️async: E1 pure field-copy, consumed only through `Iterator::map` fn-item arguments — see R9.
fn history_op_meta_from_operation_meta(meta: &MutationMeta, messages: impl IntoIterator<Item = crate::os_spr::history::HistoryMessage>) -> crate::os_spr::HistoryOpMeta {
    crate::os_spr::HistoryOpMeta {
        op_id: meta.mutation_id.as_ref().map(|id| id.0.clone()),
        dependencies: meta.dependencies.iter().map(|id| id.0.clone()).collect(),
        base_version: meta.base_version,
        author_id: meta.author_id.as_ref().map(|id| id.0.clone()),
        hlt: Some((meta.timestamp.actor, meta.timestamp.physical_ms as i64, meta.timestamp.logical)),
        undo_policy: protocol_undo_policy_ordinal(meta.undo_policy),
        payload_hash: meta.payload_hash.as_ref().map(|hash| hash.0),
        group_id: meta.group_id.clone(),
        origin: meta.origin.clone(),
        messages: messages.into_iter().collect(),
    }
}

// 🚫️async: E1 pure field-copy, consumed by `mutation_meta_from_history_op_meta`'s own caller
// (§) without `.await` — see R9.
fn mutation_meta_from_history_op_meta(meta: crate::os_spr::HistoryOpMeta) -> Result<(MutationMeta, Vec<crate::os_spr::MutationMessage>), String> {
    let (actor, physical_ms, logical) = meta.hlt.ok_or_else(|| "history operation metadata has no hybrid-clock timestamp".to_string())?;
    let physical_ms = u64::try_from(physical_ms).map_err(|_| "history operation metadata has a negative hybrid-clock physical time".to_string())?;
    if meta.op_id.as_ref().is_none_or(|id| id.trim().is_empty()) {
        return Err("history operation metadata has no stable operation identity".to_string());
    }
    let messages = meta.messages.into_iter().map(mutation_message_from_history_message).collect::<Result<Vec<_>, _>>()?;
    Ok((
        MutationMeta {
            mutation_id: meta.op_id.map(MutationId),
            dependencies: meta.dependencies.into_iter().map(MutationId).collect(),
            base_version: meta.base_version,
            author_id: meta.author_id.map(ActorId),
            timestamp: HybridLogicalTimestamp { actor, physical_ms, logical },
            undo_policy: protocol_undo_policy_from_ordinal(meta.undo_policy),
            payload_hash: meta.payload_hash.map(crate::os_spr::PayloadHash),
            semantic_kind: None,
            label: None,
            group_id: meta.group_id,
            origin: meta.origin,
        },
        messages,
    ))
}

async fn expected_mutation_message_level(code: &str) -> Option<crate::os_dsl::Severity> {
    match code {
        "mutation.target-missing" => Some(crate::os_dsl::Severity::Error),
        "mutation.no-op" | "mutation.partial" | "mutation.clamped" => Some(crate::os_dsl::Severity::Warning),
        "mutation.duplicate-id" | "mutation.invariant" => Some(crate::os_dsl::Severity::Fatal),
        "mutation.cascade" => Some(crate::os_dsl::Severity::Info),
        _ => None,
    }
}

async fn validate_persisted_message(message: &crate::os_spr::MutationMessage, operation_count: Option<usize>) -> Result<(), VcsError> {
    let Some(expected_level) = expected_mutation_message_level(&message.code.0).await else {
        return Err(VcsError::ValidationFailed(format!("history carries unknown mutation message code {}", message.code.0)));
    };
    if message.level != expected_level || message.message.trim().is_empty() || message.target.iter().any(|target| target.trim().is_empty()) {
        return Err(VcsError::ValidationFailed(format!("history carries malformed mutation message {}", message.code.0)));
    }
    if let Some(operation_count) = operation_count {
        if message.op_index.map_or(true, |index| index as usize >= operation_count) {
            return Err(VcsError::ValidationFailed(format!("history carries an invalid operation index for mutation message {}", message.code.0)));
        }
    }
    Ok(())
}

async fn stable_mutation_ids_for_edit<Mutation>(edit: &Edit<Mutation>) -> Result<Vec<MutationId>, VcsError> {
    if edit.mutation_meta.len() != edit.forwards.len() {
        return Err(VcsError::ValidationFailed(format!("edit {} does not carry one operation identity per forward mutation", edit.id)));
    }
    edit.mutation_meta.iter().map(|meta| meta.mutation_id.clone().filter(|id| !id.0.trim().is_empty()).ok_or_else(|| VcsError::ValidationFailed(format!("edit {} has an operation without a stable identity", edit.id)))).collect()
}

async fn conflict_messages_for_edits<Mutation>(edits: &[Edit<Mutation>], replayed: &[crate::os_spr::EditMessages]) -> Result<Vec<crate::os_spr::MutationMessage>, VcsError> {
    let replayed_by_edit: HashMap<&str, &crate::os_spr::EditMessages> = replayed.iter().map(|entry| (entry.edit_id.as_str(), entry)).collect();
    if replayed_by_edit.len() != replayed.len() {
        return Err(VcsError::ValidationFailed("conflict messages repeat an edit owner".to_string()));
    }
    let mut messages = Vec::new();
    let mut offset = 0u32;
    for edit in edits {
        if let Some(entry) = replayed_by_edit.get(edit.id.as_str()) {
            for message in &entry.messages {
                validate_persisted_message(message, Some(edit.forwards.len())).await?;
                let mut message = message.clone();
                let index = message.op_index.ok_or_else(|| VcsError::ValidationFailed(format!("conflict message {} has no operation index", message.code.0)))?;
                message.op_index = Some(offset.checked_add(index).ok_or_else(|| VcsError::ValidationFailed("conflict operation index overflow".to_string()))?);
                messages.push(message);
            }
        }
        offset = offset
            .checked_add(u32::try_from(edit.forwards.len()).map_err(|_| VcsError::ValidationFailed(format!("edit {} has too many operations", edit.id)))?)
            .ok_or_else(|| VcsError::ValidationFailed("conflict operation index overflow".to_string()))?;
    }
    Ok(messages)
}

async fn canonical_conflict_actors(actors: impl IntoIterator<Item = ActorId>) -> Vec<ActorId> {
    let mut actors: Vec<ActorId> = actors.into_iter().collect();
    actors.sort_by(|left, right| left.0.cmp(&right.0));
    actors.dedup_by(|left, right| left.0 == right.0);
    actors
}

async fn validate_persisted_conflicts<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>) -> Result<(), VcsError> {
    let mut conflict_ids = HashSet::new();
    let known_edits: HashMap<&str, &Edit<Mutation>> = envelope.vcs.edits.iter().map(|edit| (edit.id.as_str(), edit)).collect();
    for conflict in &envelope.conflicts {
        if conflict.id.0.trim().is_empty() || !conflict_ids.insert(conflict.id.0.as_str()) || conflict.messages.is_empty() {
            return Err(VcsError::ValidationFailed(format!("history repeats or omits conflict identity {}", conflict.id.0)));
        }
        if conflict.actors.is_empty() || conflict.actors.iter().any(|actor| actor.0.trim().is_empty()) || conflict.actors != canonical_conflict_actors(conflict.actors.clone()).await {
            return Err(VcsError::ValidationFailed(format!("conflict {} has malformed actor identities", conflict.id.0)));
        }
        // 🎯️ A conflict's messages index its kind-specific flattened operation sequence: wire
        // envelopes in `Quarantined` order, or every forward operation of `Degraded` edits in
        // `edit_ids` order. This makes `op_index` globally unambiguous for a conflict rather
        // than accidentally treating every message as if it belonged to its first edit.
        let (mutation_ids, operation_count) = match &conflict.kind {
            crate::os_spr::ConflictKind::Quarantined { envelopes } => {
                if envelopes.is_empty() {
                    return Err(VcsError::ValidationFailed(format!("quarantined conflict {} has no mutation envelopes", conflict.id.0)));
                }
                let mut mutation_ids = Vec::with_capacity(envelopes.len());
                let expected_actors = canonical_conflict_actors(envelopes.iter().map(|remote| remote.actor.clone())).await;
                if conflict.actors != expected_actors {
                    return Err(VcsError::ValidationFailed(format!("quarantined conflict {} has actors inconsistent with its envelopes", conflict.id.0)));
                }
                for remote in envelopes {
                    let mut dependencies = HashSet::new();
                    if remote.document_id.0 != envelope.id
                        || remote.diff.schema.0 != envelope.schema
                        || remote.inverse.schema != remote.diff.schema
                        || remote.mutation_id.0.trim().is_empty()
                        || remote.actor.0.trim().is_empty()
                        || remote.dependencies.iter().any(|dependency| dependency.0.trim().is_empty() || dependency == &remote.mutation_id || !dependencies.insert(dependency.0.as_str()))
                    {
                        return Err(VcsError::ValidationFailed(format!("quarantined conflict {} has invalid mutation envelopes", conflict.id.0)));
                    }
                    if conflict.timestamp < remote.timestamp {
                        return Err(VcsError::ValidationFailed(format!("quarantined conflict {} predates its mutation envelope", conflict.id.0)));
                    }
                    mutation_ids.push(remote.mutation_id.clone());
                }
                let mut ids = HashSet::new();
                if mutation_ids.iter().any(|mutation_id| !ids.insert(mutation_id.0.as_str())) {
                    return Err(VcsError::ValidationFailed(format!("quarantined conflict {} repeats a mutation identity", conflict.id.0)));
                }
                (mutation_ids, envelopes.len())
            }
            crate::os_spr::ConflictKind::Degraded { edit_ids } => {
                if edit_ids.is_empty() {
                    return Err(VcsError::ValidationFailed(format!("degraded conflict {} has no edit references", conflict.id.0)));
                }
                let mut ids = HashSet::new();
                let mut mutation_ids = Vec::new();
                let mut expected_actors = Vec::new();
                for edit_id in edit_ids {
                    if !ids.insert(edit_id.as_str()) {
                        return Err(VcsError::ValidationFailed(format!("degraded conflict {} repeats edit reference {edit_id}", conflict.id.0)));
                    }
                    let edit = known_edits.get(edit_id.as_str()).ok_or_else(|| VcsError::ValidationFailed(format!("degraded conflict {} references unknown edit {edit_id}", conflict.id.0)))?;
                    let timestamp = edit.mutation_meta.first().map(|meta| meta.timestamp).ok_or_else(|| VcsError::ValidationFailed(format!("degraded conflict {} references edit {edit_id} without operation metadata", conflict.id.0)))?;
                    if conflict.timestamp < timestamp {
                        return Err(VcsError::ValidationFailed(format!("degraded conflict {} predates edit {edit_id}", conflict.id.0)));
                    }
                    expected_actors.push(ActorId(edit.actor.clone().ok_or_else(|| VcsError::ValidationFailed(format!("degraded conflict {} references actorless edit {edit_id}", conflict.id.0)))?));
                    mutation_ids.extend(stable_mutation_ids_for_edit(edit).await.map_err(|_| VcsError::ValidationFailed(format!("degraded conflict {} references edit {edit_id} without stable operation identities", conflict.id.0)))?);
                }
                if conflict.actors != canonical_conflict_actors(expected_actors).await {
                    return Err(VcsError::ValidationFailed(format!("degraded conflict {} has actors inconsistent with its edits", conflict.id.0)));
                }
                let operation_count = mutation_ids.len();
                (mutation_ids, operation_count)
            }
        };
        for message in &conflict.messages {
            validate_persisted_message(message, Some(operation_count)).await?;
        }
        if conflict.id != crate::os_spr::ConflictId::new(&conflict.kind, &ArtifactId(envelope.id.clone()), &mutation_ids, &conflict.timestamp).await {
            return Err(VcsError::ValidationFailed(format!("conflict {} does not match its content-addressed identity", conflict.id.0)));
        }
    }
    Ok(())
}

/// @emoji 🧱️ Validates the complete durable history graph at every persistence boundary.
/// Deliberately free of store construction bounds: codecs must reject malformed history before a
/// caller needs to construct an `ArtifactStore`.
async fn validate_durable_history<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>) -> Result<(), VcsError> {
    let mut edit_ids = HashSet::new();
    let mut edit_sequences = HashSet::new();
    for edit in &envelope.vcs.edits {
        if !edit_ids.insert(edit.id.as_str()) {
            return Err(VcsError::ValidationFailed(format!("history repeats authoritative edit {}", edit.id)));
        }
        if edit.sequence_number < 0 || !edit_sequences.insert(edit.sequence_number) {
            return Err(VcsError::ValidationFailed(format!("history has an invalid edit sequence {} for {}", edit.sequence_number, edit.id)));
        }
    }
    let mut message_edit_ids = HashSet::new();
    for entry in &envelope.edit_messages {
        if !message_edit_ids.insert(entry.edit_id.as_str()) || !edit_ids.contains(entry.edit_id.as_str()) {
            return Err(VcsError::ValidationFailed(format!("history has an invalid message ledger for edit {}", entry.edit_id)));
        }
        let operation_count = envelope.vcs.edits.iter().find(|edit| edit.id == entry.edit_id).map(|edit| edit.forwards.len()).ok_or_else(|| VcsError::ValidationFailed(format!("history has an invalid message ledger for edit {}", entry.edit_id)))?;
        let mut any_malformed = false;
        for message in &entry.messages {
            if validate_persisted_message(message, Some(operation_count)).await.is_err() {
                any_malformed = true;
                break;
            }
        }
        if entry.messages.is_empty() || any_malformed {
            return Err(VcsError::ValidationFailed(format!("history has malformed messages for edit {}", entry.edit_id)));
        }
    }
    validate_persisted_conflicts(envelope).await?;
    let mut change_ids = HashSet::new();
    for change in &envelope.vcs.changes {
        if !change_ids.insert(change.id.as_str()) {
            return Err(VcsError::ValidationFailed(format!("history repeats authoritative change {}", change.id)));
        }
        let mut referenced_edits = HashSet::new();
        for edit_id in &change.edit_ids {
            if !referenced_edits.insert(edit_id.as_str()) || !edit_ids.contains(edit_id.as_str()) {
                return Err(VcsError::ValidationFailed(format!("change {} has an invalid edit reference {edit_id}", change.id)));
            }
        }
    }
    let mut checkpoint_ids = HashSet::new();
    for checkpoint in &envelope.vcs.checkpoints {
        if !checkpoint_ids.insert(checkpoint.id.as_str()) {
            return Err(VcsError::ValidationFailed(format!("history repeats authoritative checkpoint {}", checkpoint.id)));
        }
    }
    for checkpoint in &envelope.vcs.checkpoints {
        let mut referenced_changes = HashSet::new();
        for change_id in &checkpoint.change_ids {
            if !referenced_changes.insert(change_id.as_str()) || !change_ids.contains(change_id.as_str()) {
                return Err(VcsError::ValidationFailed(format!("checkpoint {} has an invalid change reference {change_id}", checkpoint.id)));
            }
        }
        if let Some(parent_id) = &checkpoint.parent_id {
            if !checkpoint_ids.contains(parent_id.as_str()) || parent_id == &checkpoint.id {
                return Err(VcsError::ValidationFailed(format!("checkpoint {} has an invalid parent reference {parent_id}", checkpoint.id)));
            }
        }
        validate_composition_pins(&checkpoint.composition_pins).await?;
        if !checkpoint.composition_pins.is_empty() {
            let expected = checkpoint_identity(checkpoint, &envelope.vcs.changes).await;
            if checkpoint.id != expected {
                return Err(VcsError::ValidationFailed(format!("checkpoint {} does not match its content-addressed composition identity {expected}", checkpoint.id)));
            }
        }
    }
    let mut alternative_ids = HashSet::new();
    for alternative in &envelope.vcs.alternatives {
        if !alternative_ids.insert(alternative.id.as_str()) {
            return Err(VcsError::ValidationFailed(format!("history repeats authoritative alternative {}", alternative.id)));
        }
        let mut referenced_checkpoints = HashSet::new();
        for checkpoint_id in &alternative.checkpoint_ids {
            if !referenced_checkpoints.insert(checkpoint_id.as_str()) || !checkpoint_ids.contains(checkpoint_id.as_str()) {
                return Err(VcsError::ValidationFailed(format!("alternative {} has an invalid checkpoint reference {checkpoint_id}", alternative.id)));
            }
        }
    }
    if let Some(active_alternative_id) = &envelope.active_alternative_id {
        if !alternative_ids.contains(active_alternative_id.as_str()) {
            return Err(VcsError::ValidationFailed(format!("history names an unknown active alternative {active_alternative_id}")));
        }
    }
    if let Some(cursor) = &envelope.cursor {
        validate_history_lanes(envelope, &cursor.applied_edit_ids, &cursor.redo_edit_ids).await?;
        if let Some(checkpoint_id) = &cursor.checkpoint_id {
            if !checkpoint_ids.contains(checkpoint_id.as_str()) {
                return Err(VcsError::ValidationFailed(format!("history cursor names an unknown checkpoint {checkpoint_id}")));
            }
        }
    }
    Ok(())
}

async fn validate_history_lanes<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, applied_edit_ids: &[String], redo_edit_ids: &[String]) -> Result<(), VcsError> {
    let known = envelope.vcs.edits.iter().map(|edit| edit.id.as_str()).collect::<HashSet<_>>();
    let mut applied = HashSet::new();
    for edit_id in applied_edit_ids {
        if !applied.insert(edit_id.as_str()) {
            return Err(VcsError::ValidationFailed(format!("history repeats applied edit {edit_id}")));
        }
        if !known.contains(edit_id.as_str()) {
            return Err(VcsError::UnknownEdit(edit_id.clone()));
        }
    }
    let mut redo = HashSet::new();
    for edit_id in redo_edit_ids {
        if !redo.insert(edit_id.as_str()) {
            return Err(VcsError::ValidationFailed(format!("history repeats redo edit {edit_id}")));
        }
        if applied.contains(edit_id.as_str()) {
            return Err(VcsError::ValidationFailed("history places an edit in both applied and redo lanes".to_string()));
        }
        if !known.contains(edit_id.as_str()) {
            return Err(VcsError::UnknownEdit(edit_id.clone()));
        }
    }
    Ok(())
}

/// @emoji 🎯️ Builds the binary op-log twin of `print_ops_log` — a `crate::os_spr::HistoryLog` carrying
/// REAL `inverse`/binary op payloads/explicit meta/cursor, encoded via `crate::os_spr::encode_history`
/// with `write_backwards_section: true`. Unlike the `.ops` text mirror (forwards-only, see
/// `print_ops_log`'s doc), this is the AUTHORITATIVE persisted form: `parse_document_spr` recovers
/// inverse/meta byte-for-byte instead of recomputing them via replay.
async fn history_op_payloads<Mutation: OpBinary>(mutations: &[Mutation]) -> Result<Vec<crate::os_spr::OpPayload>, VcsError> {
    let mut payloads = Vec::with_capacity(mutations.len());
    for op in mutations {
        let binary = op.encode_op().await.map_err(|error| VcsError::Serialize(error.to_string()))?;
        payloads.push(crate::os_spr::OpPayload { text: None, binary: Some(binary) });
    }
    Ok(payloads)
}

async fn history_edit_from_edit<Mutation: OpBinary>(edit: &Edit<Mutation>, messages: &[crate::os_spr::MutationMessage]) -> Result<crate::os_spr::HistoryEdit, VcsError> {
    if messages.iter().any(|message| message.op_index.map_or(true, |index| index as usize >= edit.forwards.len())) {
        return Err(VcsError::ValidationFailed(format!("edit {} carries a message without a valid operation index", edit.id)));
    }
    Ok(crate::os_spr::HistoryEdit {
        id: edit.id.clone(),
        actor: edit.actor.clone(),
        started_at: edit.started_at.clone(),
        finished_at: edit.finished_at.clone(),
        coalesce_key: edit.coalesce_key.clone(),
        description: edit.description.clone(),
        ops: history_op_payloads(&edit.forwards).await?,
        inverse: history_op_payloads(&edit.inverse).await?,
        // 🎯️ An empty `mutation_meta` (e.g. a hand-authored/externally-injected edit with no
        // explicit meta, distinct from a real dispatch which always populates one entry per
        // forward op) is treated as ABSENT, not as `Some(vec![])` — `encode_edit` requires
        // `metas.len() == ops.len()` when meta is present at all, and an empty-but-`Some` vec
        // would spuriously fail that check for a non-empty `ops`.
        meta: if edit.mutation_meta.is_empty() {
            if messages.is_empty() {
                None
            } else {
                return Err(VcsError::ValidationFailed(format!("edit {} carries messages without operation metadata", edit.id)));
            }
        } else {
            Some(edit.mutation_meta.iter().enumerate().map(|(index, meta)| history_op_meta_from_operation_meta(meta, messages.iter().filter(move |message| message.op_index == Some(index as u32)).map(history_message_from_mutation_message))).collect())
        },
    })
}

async fn history_conflict_from_conflict(conflict: &crate::os_spr::Conflict) -> crate::os_spr::history::HistoryConflict {
    let (kind, edit_ids, envelopes) = match &conflict.kind {
        crate::os_spr::ConflictKind::Quarantined { envelopes, .. } => {
            // 🌀️ `encode_envelope` is async (📡️replication, out of this packet's scope); `map`'s
            // closure is sync (R10 shape 1), so it's hoisted into an explicit loop.
            let mut encoded = Vec::with_capacity(envelopes.len());
            for envelope in envelopes {
                let mut bytes = Vec::new();
                crate::os_spr::encode_envelope(envelope, &mut bytes).await;
                encoded.push(bytes);
            }
            (0, Vec::new(), encoded)
        }
        crate::os_spr::ConflictKind::Degraded { edit_ids } => (1, edit_ids.clone(), Vec::new()),
    };
    crate::os_spr::history::HistoryConflict {
        id: conflict.id.0.clone(),
        kind,
        status: match conflict.status {
            crate::os_spr::ConflictStatus::Open => 0,
            crate::os_spr::ConflictStatus::Accepted => 1,
            crate::os_spr::ConflictStatus::Discarded => 2,
        },
        actors: conflict.actors.iter().map(|actor| actor.0.clone()).collect(),
        hlt: (conflict.timestamp.actor, conflict.timestamp.physical_ms, conflict.timestamp.logical),
        edit_ids,
        envelopes,
        messages: conflict.messages.iter().map(history_message_from_mutation_message).collect(),
    }
}

async fn conflict_from_history_conflict(conflict: crate::os_spr::history::HistoryConflict) -> Result<crate::os_spr::Conflict, String> {
    let kind = match conflict.kind {
        0 => {
            let mut envelopes = Vec::with_capacity(conflict.envelopes.len());
            for bytes in conflict.envelopes {
                let mut position = 0;
                let envelope = crate::os_spr::decode_envelope(&bytes, &mut position).await.map_err(|error| error.to_string())?;
                if position != bytes.len() {
                    return Err("quarantined conflict envelope has trailing bytes".into());
                }
                envelopes.push(envelope);
            }
            crate::os_spr::ConflictKind::Quarantined { envelopes }
        }
        1 => crate::os_spr::ConflictKind::Degraded { edit_ids: conflict.edit_ids },
        value => return Err(format!("unknown conflict kind {value}")),
    };
    let status = match conflict.status {
        0 => crate::os_spr::ConflictStatus::Open,
        1 => crate::os_spr::ConflictStatus::Accepted,
        2 => crate::os_spr::ConflictStatus::Discarded,
        value => return Err(format!("unknown conflict status {value}")),
    };
    Ok(crate::os_spr::Conflict {
        id: crate::os_spr::ConflictId(conflict.id),
        kind,
        status,
        messages: conflict.messages.into_iter().map(mutation_message_from_history_message).collect::<Result<Vec<_>, _>>()?,
        actors: conflict.actors.into_iter().map(ActorId).collect(),
        timestamp: HybridLogicalTimestamp { actor: conflict.hlt.0, physical_ms: conflict.hlt.1, logical: conflict.hlt.2 },
    })
}

/// @emoji 🎯️ Encodes a bare, edit-free `.spr` op log for `schema` — the counterpart to a `.pack`
/// file carrying only an initial snapshot with no history yet (e.g. a single dropped `.pack`
/// file with no accompanying `.spr` sidecar). `doc_id` may be empty when the caller mints a fresh
/// id downstream (as `parse_document_spr` never cross-checks it against the pack). LAW:
/// `parse_document_spr(pack, &empty_document_spr(id, schema))` recovers exactly `P::decode_pack(pack)`
/// as both the initial and live snapshot, with zero edits.
pub async fn empty_document_spr(doc_id: &str, schema: &str) -> Vec<u8> {
    let log = crate::os_spr::HistoryLog { doc_id: doc_id.to_string(), schema: schema.to_string(), ..crate::os_spr::HistoryLog::default() };
    crate::os_spr::encode_history(&log, &crate::os_spr::EncodeOptions::default()).await.expect("encoding an edit-free HistoryLog is infallible")
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
pub async fn append_history_edits_to_spr(spr: &[u8], edits: &[crate::os_spr::HistoryEdit]) -> Result<Vec<u8>, VcsError> {
    let mut log = crate::os_spr::decode_history(spr, &crate::os_spr::DecodeOptions::default()).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
    if let Some(cursor) = &mut log.cursor {
        cursor.applied_edit_ids.extend(edits.iter().map(|edit| edit.id.clone()));
    }
    log.edits.extend(edits.iter().cloned());
    let options = crate::os_spr::EncodeOptions { write_backwards_section: true, ..crate::os_spr::EncodeOptions::default() };
    crate::os_spr::encode_history(&log, &options).await.map_err(|error| VcsError::Serialize(error.to_string()))
}

/// @emoji 🧩️ Projects an envelope's composition facts (`owner`, `dialect`, and every checkpoint's
/// `composition_pins`) into the durable `HistoryComposition` overlay, or `None` when the document
/// has none — which is the overwhelming majority, so an ordinary leaf document's `.spr` gains not a
/// single byte from this record existing. `ArtifactRef`s cross the boundary as their `to_uri()`
/// wire string, the same "own the codec at this edge" convention `CompositionPin`/`ArtifactChild`
/// already use rather than coupling `ArtifactRef` into the protocol crate.
async fn history_composition_from_envelope<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>) -> Option<crate::os_spr::HistoryComposition> {
    // 🌀️ `ArtifactRef::to_uri` is async (🚪️io, out of this packet's scope) so it cannot run inside
    // `Option`/`Iterator::map`'s sync closures (R10 shape 1) — hoisted into explicit loops instead.
    let owner = match envelope.owner.as_ref() {
        Some(owner) => Some((owner.parent.to_uri().await, owner.slot.clone(), owner.child_id.clone())),
        None => None,
    };
    let dialect = envelope.dialect.as_ref().map(|dialect| (dialect.artifact_kind.clone(), dialect.standard.clone(), dialect.subset.clone()));
    let mut checkpoint_pins: Vec<(String, Vec<(String, String)>)> = Vec::new();
    for checkpoint in envelope.vcs.checkpoints.iter().filter(|checkpoint| !checkpoint.composition_pins.is_empty()) {
        let mut pins = Vec::with_capacity(checkpoint.composition_pins.len());
        for pin in &checkpoint.composition_pins {
            pins.push((pin.child_ref.to_uri().await, pin.checkpoint_id.clone()));
        }
        checkpoint_pins.push((checkpoint.id.clone(), pins));
    }
    if owner.is_none() && dialect.is_none() && checkpoint_pins.is_empty() {
        return None;
    }
    Some(crate::os_spr::HistoryComposition { owner, dialect, checkpoint_pins })
}

/// @emoji 🧩️ Inverse of `history_composition_from_envelope`: malformed ownership or pins reject
/// the authoritative history rather than being silently discarded.
async fn apply_history_composition<P, Mutation>(envelope: &mut ArtifactEnvelope<P, Mutation>, composition: &crate::os_spr::HistoryComposition) -> Result<(), VcsError> {
    envelope.owner = match &composition.owner {
        Some((parent, slot, child_id)) => Some(OwnerRef { parent: crate::os_io::ArtifactRef::parse_uri(parent).await.map_err(VcsError::Deserialize)?, slot: slot.clone(), child_id: child_id.clone() }),
        None => None,
    };
    envelope.dialect = composition.dialect.as_ref().map(|(artifact_kind, standard, subset)| crate::os_io::ArtifactDialect { artifact_kind: artifact_kind.clone(), standard: standard.clone(), subset: subset.clone() });
    for (checkpoint_id, pins) in &composition.checkpoint_pins {
        let checkpoint = envelope.vcs.checkpoints.iter_mut().find(|checkpoint| checkpoint.id == *checkpoint_id).ok_or_else(|| VcsError::UnknownChange(checkpoint_id.clone()))?;
        let mut resolved_pins = Vec::with_capacity(pins.len());
        for (child_uri, pin_checkpoint_id) in pins {
            let child_ref = crate::os_io::ArtifactRef::parse_uri(child_uri).await.map_err(VcsError::Deserialize)?;
            resolved_pins.push(crate::os_vcs::CompositionPin { child_ref, checkpoint_id: pin_checkpoint_id.clone() });
        }
        checkpoint.composition_pins = resolved_pins;
        validate_composition_pins(&checkpoint.composition_pins).await?;
    }
    Ok(())
}

pub async fn print_document_spr<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>) -> Result<Vec<u8>, VcsError>
where
    Mutation: OpBinary,
{
    validate_persisted_conflicts(envelope).await?;
    let mut message_ledger = std::collections::BTreeMap::new();
    for entry in &envelope.edit_messages {
        if message_ledger.insert(entry.edit_id.as_str(), entry.messages.as_slice()).is_some() {
            return Err(VcsError::ValidationFailed(format!("duplicate message ledger for edit {}", entry.edit_id)));
        }
        if !envelope.vcs.edits.iter().any(|edit| edit.id == entry.edit_id) {
            return Err(VcsError::ValidationFailed(format!("message ledger references unknown edit {}", entry.edit_id)));
        }
        let operation_count = envelope.vcs.edits.iter().find(|edit| edit.id == entry.edit_id).map(|edit| edit.forwards.len()).ok_or_else(|| VcsError::ValidationFailed(format!("message ledger references unknown edit {}", entry.edit_id)))?;
        if entry.messages.is_empty() {
            return Err(VcsError::ValidationFailed(format!("message ledger for edit {} is empty", entry.edit_id)));
        }
        for message in entry.messages.iter() {
            validate_persisted_message(message, Some(operation_count)).await?;
        }
    }
    let mut edits = Vec::with_capacity(envelope.vcs.edits.len());
    for edit in &envelope.vcs.edits {
        edits.push(history_edit_from_edit::<Mutation>(edit, message_ledger.get(edit.id.as_str()).copied().unwrap_or(&[])).await?);
    }
    // 🌀️ `history_conflict_from_conflict` is async (calls the 📡️replication `encode_envelope`);
    // `Iterator::map`'s closure is sync (R10 shape 1), so it's hoisted into an explicit loop.
    let mut conflicts = Vec::with_capacity(envelope.conflicts.len());
    for conflict in &envelope.conflicts {
        conflicts.push(history_conflict_from_conflict(conflict).await);
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
        composition: history_composition_from_envelope(envelope).await,
        conflicts,
    };
    let options = crate::os_spr::EncodeOptions { write_backwards_section: true, ..crate::os_spr::EncodeOptions::default() };
    crate::os_spr::encode_history(&log, &options).await.map_err(|error| VcsError::Serialize(error.to_string()))
}

/// @emoji 🎯️ Inverse of [`print_document_spr`]: rebuilds an envelope's `edits`/`changes`/
/// `checkpoints`/`alternatives`/`cursor` from a decoded `HistoryLog`, recovering `inverse` and
/// `mutation_meta` from the persisted data (never replay-recomputed, unlike the text path) — the
/// initial snapshot comes from `pack` via `ArtifactPack::decode_pack`, matching
/// `parse_document_pack`'s contract.
pub async fn parse_document_spr<P, Mutation>(pack: &[u8], spr: &[u8]) -> Result<ParsedDocumentText<P, Mutation>, TextError>
where
    P: Clone + ArtifactPack,
    Mutation: OpText + OpBinary + self::Mutation<P>,
{
    let initial_snapshot = P::decode_pack(pack).await.map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))?;
    let mut log = crate::os_spr::decode_history(spr, &crate::os_spr::DecodeOptions::default()).await.map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))?;

    async fn decode_op<P, Mutation: OpText + OpBinary + self::Mutation<P>>(payload: &crate::os_spr::OpPayload) -> Result<Mutation, TextError> {
        match (&payload.binary, &payload.text) {
            (Some(bytes), _) => Mutation::decode_op(bytes).await.map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1))),
            (None, Some(text)) => Mutation::parse_op(text).await,
            (None, None) => Err(TextError::new("op payload carries neither binary nor text".to_string(), TextSpan::at(1, 1))),
        }
    }

    async fn decode_ops<P, Mutation: OpText + OpBinary + self::Mutation<P>>(payloads: &[crate::os_spr::OpPayload]) -> Result<Vec<Mutation>, TextError> {
        let mut out = Vec::with_capacity(payloads.len());
        for payload in payloads {
            out.push(decode_op::<P, Mutation>(payload).await?);
        }
        Ok(out)
    }

    let mut snapshot = initial_snapshot.clone();
    let mut edits: Vec<Edit<Mutation>> = Vec::with_capacity(log.edits.len());
    let mut edit_messages = Vec::new();
    for (index, history_edit) in log.edits.into_iter().enumerate() {
        let edit_id = history_edit.id.clone();
        let forwards = decode_ops::<P, Mutation>(&history_edit.ops).await?;
        let inverse = decode_ops::<P, Mutation>(&history_edit.inverse).await?;
        let metas = history_edit.meta.ok_or_else(|| TextError::new(format!("history edit {edit_id} has no authoritative operation metadata"), TextSpan::at(1, 1)))?;
        if metas.len() != forwards.len() {
            return Err(TextError::new(format!("history edit {edit_id} has {} metadata entries for {} forward operations", metas.len(), forwards.len()), TextSpan::at(1, 1)));
        }
        let mut durable_messages = Vec::new();
        let mutation_meta = metas
            .into_iter()
            .map(|meta| {
                let (meta, messages) = mutation_meta_from_history_op_meta(meta)?;
                durable_messages.extend(messages);
                Ok(meta)
            })
            .collect::<Result<Vec<_>, String>>()
            .map_err(|error| TextError::new(error, TextSpan::at(1, 1)))?;
        if !durable_messages.is_empty() {
            edit_messages.push(crate::os_spr::EditMessages { edit_id: edit_id.clone(), messages: durable_messages });
        }
        for operation in &forwards {
            snapshot = apply_mutation(&snapshot, operation).await.map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))?.0;
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

    let cursor = log
        .cursor
        .map(|cursor| ArtifactCursor { applied_edit_ids: cursor.applied_edit_ids, redo_edit_ids: cursor.redo_edit_ids, checkpoint_id: cursor.checkpoint_id })
        .ok_or_else(|| TextError::new("history has no explicit cursor".to_string(), TextSpan::at(1, 1)))?;
    // 🌀️ `conflict_from_history_conflict` is async (calls the 📡️replication `decode_envelope`);
    // `Iterator::map`'s closure is sync (R10 shape 1), so it's hoisted into an explicit loop.
    let mut conflicts = Vec::with_capacity(log.conflicts.len());
    for conflict in std::mem::take(&mut log.conflicts) {
        conflicts.push(conflict_from_history_conflict(conflict).await.map_err(|error| TextError::new(error, TextSpan::at(1, 1)))?);
    }
    let mut envelope = ArtifactEnvelope {
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
                    // 🧩️ Filled below by `apply_history_composition` from the `REC_COMPOSITION`
                    // overlay, which is decoded per-document rather than per-checkpoint.
                    composition_pins: Vec::new(),
                })
                .collect(),
            alternatives: log.alternatives.into_iter().map(|alternative| Alternative { id: alternative.id, name: alternative.name, checkpoint_ids: alternative.checkpoint_ids }).collect(),
        },
        backbone: None,
        active_alternative_id: log.active_alternative_id,
        cursor: Some(cursor.clone()),
        // 🧩️ Both stamped below by `apply_history_composition` from the `REC_COMPOSITION` overlay.
        dialect: None,
        migrated_from: None,
        owner: None,
        // 🎯️ `crate::os_spr::HistoryLog`/`HistoryEdit` don't carry a lane overlay yet (like
        // `composition_pins` above, that codec extension is out of this wave's scope) — a
        // `.pack`+`.spr` reload therefore loses non-`Document` lane tags today; only the plain
        // `ArtifactStore::envelope_json` path round-trips them. Follow-up for whichever wave wires
        // real persisted-local interaction state through this reload path.
        lanes: std::collections::BTreeMap::new(),
        edit_messages,
        conflicts,
    };
    if let Some(composition) = &log.composition {
        apply_history_composition(&mut envelope, composition).await.map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))?;
    }
    validate_durable_history(&envelope).await.map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))?;

    let mut snapshot = envelope.vcs.initial_snapshot.clone();
    for edit_id in &cursor.applied_edit_ids {
        let edit = envelope.vcs.edits.iter().find(|edit| &edit.id == edit_id).ok_or_else(|| TextError::new(format!("history cursor references unknown edit {edit_id}"), TextSpan::at(1, 1)))?;
        for operation in &edit.forwards {
            snapshot = apply_mutation(&snapshot, operation).await.map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))?.0;
        }
    }
    Ok(ParsedDocumentText { envelope, snapshot })
}

/// @emoji 📤️ Pack counterpart of `print_document_text`: identical op-log TEXT body (`print_ops_log`)
/// for the human-readable mirror, but the initial snapshot is encoded to pack bytes
/// (`ArtifactPack::encode_pack`) instead of printed to DSL text — plus the AUTHORITATIVE `.spr`
/// binary op log (`print_document_spr`), which carries real inverse/binary payloads/cursor.
pub async fn print_document_pack<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>) -> Result<ArtifactPackFiles, VcsError>
where
    P: ArtifactPack,
    Mutation: OpText + OpBinary,
{
    let pack = envelope.vcs.initial_snapshot.encode_pack();
    let spr = print_document_spr(envelope).await?;
    let ops = print_ops_log(envelope).await?;
    Ok(ArtifactPackFiles { pack: pack.await, spr, ops })
}

/// @emoji 📥️ Parses the explicit `.ops` records against an already-obtained initial snapshot.
/// Text persistence is intentionally strict: it requires inverse operations, one authoritative
/// metadata record per forward operation, durable messages/conflicts, and exactly one cursor.
async fn replay_ops<P, Mutation>(initial_snapshot: P, ops: &str) -> Result<ParsedDocumentText<P, Mutation>, TextError>
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
    let mut inverse_by_edit: HashMap<String, Vec<Mutation>> = HashMap::new();
    let mut metadata_by_edit: HashMap<String, HashMap<u32, MutationMeta>> = HashMap::new();
    let mut messages_by_edit: HashMap<String, Vec<crate::os_spr::MutationMessage>> = HashMap::new();
    let mut message_order = Vec::new();
    let mut conflicts = Vec::new();
    let mut saw_doc = false;

    struct PendingEdit {
        id: String,
        sequence_number: i32,
        actor: Option<String>,
        started_at: String,
        finished_at: Option<String>,
        coalesce_key: Option<String>,
        description: Option<String>,
    }
    let mut pending_edit: Option<PendingEdit> = None;
    let mut pending_forwards: Vec<Mutation> = Vec::new();

    let flush_pending_edit = |pending_edit: &mut Option<PendingEdit>, pending_forwards: &mut Vec<Mutation>, edits: &mut Vec<Edit<Mutation>>| -> Result<(), TextError> {
        let Some(header) = pending_edit.take() else {
            return Ok(());
        };
        let forwards = std::mem::take(pending_forwards);
        edits.push(Edit {
            id: header.id,
            actor: header.actor,
            forwards,
            inverse: Vec::new(),
            mutation_meta: Vec::new(),
            description: header.description,
            coalesce_key: header.coalesce_key,
            sequence_number: header.sequence_number,
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
            let operation = Mutation::parse_op(trimmed).await.map_err(|error| TextError::new(error.message, TextSpan::at(line_no, error.span.column)))?;
            pending_forwards.push(operation);
            continue;
        }
        flush_pending_edit(&mut pending_edit, &mut pending_forwards, &mut edits)?;
        let line = OpsHeaderLine::parse_op(trimmed).await.map_err(|error| TextError::new(error.message, TextSpan::at(line_no, error.span.column)))?;
        match line {
            OpsHeaderLine::Doc { id: doc_id, schema: doc_schema } => {
                if saw_doc {
                    return Err(TextError::new("ops text repeats its document header".to_string(), TextSpan::at(line_no, 1)));
                }
                saw_doc = true;
                schema = doc_schema;
                id = doc_id;
            }
            OpsHeaderLine::Edit { id: edit_id, sequence, started, actor, finished, key, description } => {
                if edits.iter().any(|edit| edit.id == edit_id) || pending_edit.as_ref().is_some_and(|edit| edit.id == edit_id) {
                    return Err(TextError::new(format!("ops text repeats edit {edit_id}"), TextSpan::at(line_no, 1)));
                }
                pending_edit = Some(PendingEdit { id: edit_id, sequence_number: sequence, actor, started_at: started, finished_at: finished, coalesce_key: key, description });
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
                if cursor.is_some() {
                    return Err(TextError::new("ops text repeats its cursor".to_string(), TextSpan::at(line_no, 1)));
                }
                cursor = Some(ArtifactCursor { applied_edit_ids: applied, redo_edit_ids: redo, checkpoint_id: checkpoint });
            }
            OpsHeaderLine::Inverse { edit, ops } => {
                let mut inverse = Vec::with_capacity(ops.len());
                for operation in &ops {
                    inverse.push(Mutation::parse_op(operation).await.map_err(|error| TextError::new(error.message, TextSpan::at(line_no, error.span.column)))?);
                }
                if inverse_by_edit.insert(edit.clone(), inverse).is_some() {
                    return Err(TextError::new(format!("ops text repeats inverse record for edit {edit}"), TextSpan::at(line_no, 1)));
                }
            }
            OpsHeaderLine::Metadata { edit, index, data } => {
                let metadata = serde_json::from_str(&data).map_err(|error| TextError::new(format!("invalid metadata record for edit {edit}: {error}"), TextSpan::at(line_no, 1)))?;
                if metadata_by_edit.entry(edit.clone()).or_default().insert(index, metadata).is_some() {
                    return Err(TextError::new(format!("ops text repeats metadata index {index} for edit {edit}"), TextSpan::at(line_no, 1)));
                }
            }
            OpsHeaderLine::Message { edit, data } => {
                let message = serde_json::from_str(&data).map_err(|error| TextError::new(format!("invalid message record for edit {edit}: {error}"), TextSpan::at(line_no, 1)))?;
                if !messages_by_edit.contains_key(&edit) {
                    message_order.push(edit.clone());
                }
                messages_by_edit.entry(edit).or_default().push(message);
            }
            OpsHeaderLine::Conflict { data } => {
                let conflict = serde_json::from_str(&data).map_err(|error| TextError::new(format!("invalid conflict record: {error}"), TextSpan::at(line_no, 1)))?;
                conflicts.push(conflict);
            }
        }
    }
    flush_pending_edit(&mut pending_edit, &mut pending_forwards, &mut edits)?;
    if !saw_doc {
        return Err(TextError::new("ops text has no document header".to_string(), TextSpan::at(1, 1)));
    }
    let cursor = cursor.ok_or_else(|| TextError::new("ops text has no explicit cursor".to_string(), TextSpan::at(1, 1)))?;
    let known_edits: HashSet<String> = edits.iter().map(|edit| edit.id.clone()).collect();
    for edit in &mut edits {
        edit.inverse = inverse_by_edit.remove(&edit.id).ok_or_else(|| TextError::new(format!("ops text has no inverse record for edit {}", edit.id), TextSpan::at(1, 1)))?;
        let metadata = metadata_by_edit.remove(&edit.id).ok_or_else(|| TextError::new(format!("ops text has no metadata records for edit {}", edit.id), TextSpan::at(1, 1)))?;
        if metadata.len() != edit.forwards.len() || (0..edit.forwards.len() as u32).any(|index| !metadata.contains_key(&index)) {
            return Err(TextError::new(format!("ops text metadata does not exactly cover edit {}", edit.id), TextSpan::at(1, 1)));
        }
        edit.mutation_meta =
            (0..edit.forwards.len() as u32).map(|index| metadata.get(&index).cloned().ok_or_else(|| TextError::new(format!("ops text metadata is missing index {index} for edit {}", edit.id), TextSpan::at(1, 1)))).collect::<Result<Vec<_>, _>>()?;
    }
    if let Some(edit) = inverse_by_edit.keys().next() {
        return Err(TextError::new(format!("ops text has an inverse for unknown edit {edit}"), TextSpan::at(1, 1)));
    }
    if let Some(edit) = metadata_by_edit.keys().next() {
        return Err(TextError::new(format!("ops text has metadata for unknown edit {edit}"), TextSpan::at(1, 1)));
    }
    let mut applied = HashSet::new();
    for edit_id in &cursor.applied_edit_ids {
        if !known_edits.contains(edit_id) || !applied.insert(edit_id.as_str()) {
            return Err(TextError::new(format!("ops cursor has an unknown or duplicate applied edit {edit_id}"), TextSpan::at(1, 1)));
        }
    }
    let mut redo = HashSet::new();
    for edit_id in &cursor.redo_edit_ids {
        if !known_edits.contains(edit_id) || !redo.insert(edit_id.as_str()) || applied.contains(edit_id.as_str()) {
            return Err(TextError::new(format!("ops cursor has an unknown, duplicate, or overlapping redo edit {edit_id}"), TextSpan::at(1, 1)));
        }
    }
    let mut edit_messages = Vec::new();
    for edit_id in message_order {
        let messages = messages_by_edit.remove(&edit_id).ok_or_else(|| TextError::new(format!("ops text lost message ownership for edit {edit_id}"), TextSpan::at(1, 1)))?;
        let edit = edits.iter().find(|edit| edit.id == edit_id).ok_or_else(|| TextError::new(format!("ops text has messages for unknown edit {edit_id}"), TextSpan::at(1, 1)))?;
        for message in &messages {
            validate_persisted_message(message, Some(edit.forwards.len())).await.map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))?;
        }
        edit_messages.push(crate::os_spr::EditMessages { edit_id, messages });
    }
    let envelope = ArtifactEnvelope {
        schema,
        id,
        vcs: ArtifactVcs { initial_snapshot, edits, changes, checkpoints, alternatives },
        backbone: None,
        active_alternative_id,
        cursor: Some(cursor.clone()),
        dialect: None,
        migrated_from: None,
        owner: None,
        lanes: std::collections::BTreeMap::new(),
        edit_messages,
        conflicts,
    };
    validate_durable_history(&envelope).await.map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))?;
    let mut snapshot = envelope.vcs.initial_snapshot.clone();
    for edit_id in &cursor.applied_edit_ids {
        let edit = envelope.vcs.edits.iter().find(|edit| &edit.id == edit_id).ok_or_else(|| TextError::new(format!("ops cursor references unknown edit {edit_id}"), TextSpan::at(1, 1)))?;
        for operation in &edit.forwards {
            snapshot = apply_mutation(&snapshot, operation).await.map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))?.0;
        }
    }
    Ok(ParsedDocumentText { envelope, snapshot })
}

/// @emoji 📥️ Parses the textual VCS document back into an envelope plus its live (fully-replayed)
/// snapshot — obtains the initial snapshot via `P::parse_dsl` then shares `replay_ops`.
pub async fn parse_document_text<P, Mutation>(dsl: &str, ops: &str) -> Result<ParsedDocumentText<P, Mutation>, TextError>
where
    P: Clone + ArtifactDsl,
    Mutation: OpText + self::Mutation<P>,
{
    let initial_snapshot = P::parse_dsl(dsl).await?;
    replay_ops(initial_snapshot, ops).await
}

/// @emoji 📥️ spr-first pack counterpart of `parse_document_text`: pack+spr are the AUTHORITATIVE
/// pair (see `ArtifactPackFiles`'s doc) — this is a thin forward onto `parse_document_spr`, which
/// recovers real `inverse`/`mutation_meta`/`cursor` instead of recomputing them via replay.
pub async fn parse_document_pack<P, Mutation>(pack: &[u8], spr: &[u8]) -> Result<ParsedDocumentText<P, Mutation>, TextError>
where
    P: Clone + ArtifactPack,
    Mutation: OpText + OpBinary + self::Mutation<P>,
{
    parse_document_spr(pack, spr).await
}
//#endregion 🔖️TextFormat

//#region 🔖️CommandFormat
mod operation_envelope_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    // 🚫️async: E1 serde `#[serde(with = ...)]` module fn — signature fixed by serde derive, must
    // stay sync. Blocked pending `crate::os_spr::encode_envelope`/`decode_envelope` (📡️replication,
    // not this packet's path) going sync per R9 — see lease-request in the packet report.
    pub fn serialize<S: Serializer>(envelope: &crate::os_spr::MutationEnvelope, serializer: S) -> Result<S::Ok, S::Error> {
        let mut bytes = Vec::new();
        crate::os_spr::encode_envelope(envelope, &mut bytes);
        bytes.serialize(serializer)
    }

    // 🚫️async: E1 — see `serialize`'s tag above.
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
    /// @emoji 🛤️ Text twin of `ArtifactCommand::ApplyInLane` — `lane` is a kebab token
    /// (`history_lane_to_token`/`parse_history_lane_token`), matching `Undo.policy`'s convention.
    ApplyInLane {
        description: Option<String>,
        lane: String,
    },
    AmendInLane {
        key: Option<String>,
        lane: String,
    },
    UndoInLane {
        lane: String,
    },
    RedoInLane {
        lane: String,
    },
    /// @emoji ⚖️ Text twin of `ArtifactCommand::SetMergePolicy` — `policy` is a kebab token
    /// (`merge_policy_to_token`/`parse_merge_policy_token`), matching `Undo.policy`'s convention.
    SetMergePolicy {
        policy: String,
    },
    /// @emoji ⚔️ Text twin of `ArtifactCommand::ResolveConflict` — `resolution` is a kebab token
    /// (`conflict_resolution_to_token`/`parse_conflict_resolution_token`).
    ResolveConflict {
        #[dsl(positional)]
        conflict_id: String,
        resolution: String,
    },
}

//#region 🔖️OpCodec
/// 🎞️ Handcrafted OpText (P6).
impl OpText for CommandHeaderLine {
    async fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = crate::os_dsl::parse(line, &spec_fn(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline }).await?;
                return <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record).await;
            }
        }
        Err(crate::os_dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self).await;
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline).await
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl OpBinary for CommandHeaderLine {
    async fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self).await;
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1)();
        let body = crate::os_pack::encode_record_body(&spec, &record, &PackEncodeOptions::default()).await.map_err(crate::os_spr::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        crate::os_pack::write_varint_u64(&mut out, ordinal as u64).await;
        out.extend_from_slice(&body);
        Ok(out)
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = crate::os_pack::ByteReader::new(bytes).await;
        let format = reader.read_u8().await?;
        if format != OP_BINARY_FORMAT {
            return Err(crate::os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64().await?;
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = spec_fn();
        let body = &bytes[reader.position().await..];
        let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &PackDecodeOptions::default()).await.map_err(crate::os_spr::ProtocolError::from)?;
        let record_offset = reader.position().await as u64;
        <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record).await.map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "op record", offset: record_offset, detail: error.to_string() })
    }
}
//#endregion 🔖️OpCodec

async fn undo_policy_to_token(policy: UndoPolicy) -> &'static str {
    match policy {
        UndoPolicy::ExactBaseOnly => "exact-base-only",
        UndoPolicy::TransformAgainstConcurrent => "transform-against-concurrent",
        UndoPolicy::SemanticUndo => "semantic-undo",
        UndoPolicy::CompensatingAction => "compensating-action",
    }
}

async fn parse_undo_policy_token(token: &str) -> Result<UndoPolicy, TextError> {
    match token {
        "exact-base-only" => Ok(UndoPolicy::ExactBaseOnly),
        "transform-against-concurrent" => Ok(UndoPolicy::TransformAgainstConcurrent),
        "semantic-undo" => Ok(UndoPolicy::SemanticUndo),
        "compensating-action" => Ok(UndoPolicy::CompensatingAction),
        other => Err(crate::os_dsl::__rt::field_error(format!("unknown undo policy token {other:?}"))),
    }
}

async fn undo_policy_ordinal(policy: UndoPolicy) -> u8 {
    match policy {
        UndoPolicy::ExactBaseOnly => 0,
        UndoPolicy::TransformAgainstConcurrent => 1,
        UndoPolicy::SemanticUndo => 2,
        UndoPolicy::CompensatingAction => 3,
    }
}

async fn undo_policy_from_ordinal(ordinal: u8) -> Result<UndoPolicy, crate::os_spr::ProtocolError> {
    match ordinal {
        0 => Ok(UndoPolicy::ExactBaseOnly),
        1 => Ok(UndoPolicy::TransformAgainstConcurrent),
        2 => Ok(UndoPolicy::SemanticUndo),
        3 => Ok(UndoPolicy::CompensatingAction),
        other => Err(crate::os_spr::ProtocolError::Malformed { what: "undo policy ordinal", offset: 0, detail: format!("unknown undo policy ordinal {other}") }),
    }
}

/// @emoji 🛤️ `HistoryLane`'s text token — the `ApplyInLane`/`AmendInLane`/`UndoInLane`/
/// `RedoInLane` command lines' `lane=...` value, mirroring `undo_policy_to_token`'s convention.
async fn history_lane_to_token(lane: HistoryLane) -> &'static str {
    match lane {
        HistoryLane::Document => "document",
        HistoryLane::Interaction => "interaction",
    }
}

async fn parse_history_lane_token(token: &str) -> Result<HistoryLane, TextError> {
    match token {
        "document" => Ok(HistoryLane::Document),
        "interaction" => Ok(HistoryLane::Interaction),
        other => Err(crate::os_dsl::__rt::field_error(format!("unknown history lane token {other:?}"))),
    }
}

/// @emoji 🛤️ `HistoryLane`'s binary ordinal, mirroring `undo_policy_ordinal`'s convention.
async fn history_lane_ordinal(lane: HistoryLane) -> u8 {
    match lane {
        HistoryLane::Document => 0,
        HistoryLane::Interaction => 1,
    }
}

async fn history_lane_from_ordinal(ordinal: u8) -> Result<HistoryLane, crate::os_spr::ProtocolError> {
    match ordinal {
        0 => Ok(HistoryLane::Document),
        1 => Ok(HistoryLane::Interaction),
        other => Err(crate::os_spr::ProtocolError::Malformed { what: "history lane ordinal", offset: 0, detail: format!("unknown history lane ordinal {other}") }),
    }
}

/// @emoji ⚖️ `crate::os_spr::MergePolicy`'s text token — the `SetMergePolicy` command line's
/// `policy=...` value, mirroring `undo_policy_to_token`'s convention.
async fn merge_policy_to_token(policy: crate::os_spr::MergePolicy) -> &'static str {
    match policy {
        crate::os_spr::MergePolicy::LaissezFaire => "laissez-faire",
        crate::os_spr::MergePolicy::Normal => "normal",
        crate::os_spr::MergePolicy::Vigilant => "vigilant",
    }
}

async fn parse_merge_policy_token(token: &str) -> Result<crate::os_spr::MergePolicy, TextError> {
    match token {
        "laissez-faire" => Ok(crate::os_spr::MergePolicy::LaissezFaire),
        "normal" => Ok(crate::os_spr::MergePolicy::Normal),
        "vigilant" => Ok(crate::os_spr::MergePolicy::Vigilant),
        other => Err(crate::os_dsl::__rt::field_error(format!("unknown merge policy token {other:?}"))),
    }
}

/// @emoji ⚔️ `crate::os_spr::ConflictResolution`'s text token — the `ResolveConflict` command line's
/// `resolution=...` value, mirroring `undo_policy_to_token`'s convention.
async fn conflict_resolution_to_token(resolution: crate::os_spr::ConflictResolution) -> &'static str {
    match resolution {
        crate::os_spr::ConflictResolution::Accept => "accept",
        crate::os_spr::ConflictResolution::Discard => "discard",
    }
}

async fn parse_conflict_resolution_token(token: &str) -> Result<crate::os_spr::ConflictResolution, TextError> {
    match token {
        "accept" => Ok(crate::os_spr::ConflictResolution::Accept),
        "discard" => Ok(crate::os_spr::ConflictResolution::Discard),
        other => Err(crate::os_dsl::__rt::field_error(format!("unknown conflict resolution token {other:?}"))),
    }
}

/// @emoji ⚔️ `crate::os_spr::ConflictResolution`'s binary ordinal, mirroring
/// `undo_policy_ordinal`'s convention.
async fn conflict_resolution_ordinal(resolution: crate::os_spr::ConflictResolution) -> u8 {
    match resolution {
        crate::os_spr::ConflictResolution::Accept => 0,
        crate::os_spr::ConflictResolution::Discard => 1,
    }
}

async fn conflict_resolution_from_ordinal(ordinal: u8) -> Result<crate::os_spr::ConflictResolution, crate::os_spr::ProtocolError> {
    match ordinal {
        0 => Ok(crate::os_spr::ConflictResolution::Accept),
        1 => Ok(crate::os_spr::ConflictResolution::Discard),
        other => Err(crate::os_spr::ProtocolError::Malformed { what: "conflict resolution ordinal", offset: 0, detail: format!("unknown conflict resolution ordinal {other}") }),
    }
}

/// @emoji 📤️ Prints every 2-space-indented `Op::print_op` line for one `apply`/`amend` body,
/// erroring exactly like `print_edit_lines` if any op prints a line containing a newline.
async fn print_indented_ops<Op: OpText>(out: &mut String, mutations: &[Op]) -> Result<(), VcsError> {
    for mutation in mutations {
        let printed = mutation.print_op().await;
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
async fn parse_indented_ops<Op: OpText>(body_lines: &[&str]) -> Result<Vec<Op>, TextError> {
    let mut mutations = Vec::with_capacity(body_lines.len());
    for raw in body_lines {
        if !raw.starts_with("  ") {
            return Err(crate::os_dsl::__rt::field_error(format!("expected a 2-space-indented op line, got: {raw:?}")));
        }
        mutations.push(Op::parse_op(raw.trim()).await?);
    }
    Ok(mutations)
}

/// @emoji 📥️ Strips exactly one 2-space indent level from every line, joining them back into a
/// standalone command text — used to recurse `parse_command` into a `semantic-undo`/
/// `compensating-action` nested command block.
async fn dedent_command_lines(lines: &[&str]) -> Result<String, TextError> {
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
pub async fn print_command<Op: OpText>(command: &ArtifactCommand<Op>) -> Result<String, VcsError> {
    let mut out = String::new();
    match command {
        ArtifactCommand::Apply { mutations, description } => {
            out.push_str(&CommandHeaderLine::Apply { description: description.clone() }.print_op().await);
            out.push('\n');
            print_indented_ops(&mut out, mutations).await?;
        }
        ArtifactCommand::Undo => {
            out.push_str(&CommandHeaderLine::Undo { policy: None }.print_op().await);
            out.push('\n');
        }
        ArtifactCommand::Redo => {
            out.push_str(&CommandHeaderLine::Redo.print_op().await);
            out.push('\n');
        }
        ArtifactCommand::UndoWithPolicy { policy, semantic_command } => {
            out.push_str(&CommandHeaderLine::Undo { policy: Some(undo_policy_to_token(*policy).await.to_string()) }.print_op().await);
            out.push('\n');
            if let Some(nested) = semantic_command {
                let nested_text = print_command(nested).await?;
                for line in nested_text.lines() {
                    out.push_str("  ");
                    out.push_str(line);
                    out.push('\n');
                }
            }
        }
        ArtifactCommand::CommitCheckpoint { message, authors } => {
            let header = CommandHeaderLine::CommitCheckpoint { message: message.clone(), by: authors.iter().map(OpsAuthor::from).collect() };
            out.push_str(&header.print_op().await);
            out.push('\n');
        }
        ArtifactCommand::CreateAlternative { name } => {
            out.push_str(&CommandHeaderLine::CreateAlternative { name: name.clone() }.print_op().await);
            out.push('\n');
        }
        ArtifactCommand::SwitchAlternative { alternative_id } => {
            out.push_str(&CommandHeaderLine::SwitchAlternative { id: alternative_id.clone() }.print_op().await);
            out.push('\n');
        }
        ArtifactCommand::CheckoutCheckpoint { checkpoint_id } => {
            out.push_str(&CommandHeaderLine::Checkout { id: checkpoint_id.clone() }.print_op().await);
            out.push('\n');
        }
        ArtifactCommand::AmendLast { mutations, coalesce_key } => {
            out.push_str(&CommandHeaderLine::Amend { key: coalesce_key.clone() }.print_op().await);
            out.push('\n');
            print_indented_ops(&mut out, mutations).await?;
        }
        ArtifactCommand::ApplyInLane { mutations, description, lane } => {
            out.push_str(&CommandHeaderLine::ApplyInLane { description: description.clone(), lane: history_lane_to_token(*lane).await.to_string() }.print_op().await);
            out.push('\n');
            print_indented_ops(&mut out, mutations).await?;
        }
        ArtifactCommand::AmendLastInLane { mutations, coalesce_key, lane } => {
            out.push_str(&CommandHeaderLine::AmendInLane { key: coalesce_key.clone(), lane: history_lane_to_token(*lane).await.to_string() }.print_op().await);
            out.push('\n');
            print_indented_ops(&mut out, mutations).await?;
        }
        ArtifactCommand::UndoInLane { lane } => {
            out.push_str(&CommandHeaderLine::UndoInLane { lane: history_lane_to_token(*lane).await.to_string() }.print_op().await);
            out.push('\n');
        }
        ArtifactCommand::RedoInLane { lane } => {
            out.push_str(&CommandHeaderLine::RedoInLane { lane: history_lane_to_token(*lane).await.to_string() }.print_op().await);
            out.push('\n');
        }
        ArtifactCommand::IngestRemote { .. } => {
            return Err(VcsError::Serialize("IngestRemote has no text command form".into()));
        }
        ArtifactCommand::PruneDrafts => {
            out.push_str(&CommandHeaderLine::PruneDrafts.print_op().await);
            out.push('\n');
        }
        ArtifactCommand::SetMergePolicy { policy } => {
            out.push_str(&CommandHeaderLine::SetMergePolicy { policy: merge_policy_to_token(*policy).await.to_string() }.print_op().await);
            out.push('\n');
        }
        ArtifactCommand::ResolveConflict { conflict_id, resolution } => {
            out.push_str(&CommandHeaderLine::ResolveConflict { conflict_id: conflict_id.clone(), resolution: conflict_resolution_to_token(*resolution).await.to_string() }.print_op().await);
            out.push('\n');
        }
    }
    Ok(out)
}

/// @emoji 📥️ Parses a `print_command`-produced (or hand-authored) command text back into a
/// `ArtifactCommand`. LAW: `parse_command(&print_command(c)?) == Ok(c)` for every `c`.
pub async fn parse_command<Op: OpText>(text: &str) -> Result<ArtifactCommand<Op>, TextError> {
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
    let header_line = CommandHeaderLine::parse_op(header_text).await.map_err(|error| TextError::new(error.message, TextSpan::at(header_line_no, error.span.column)))?;
    let body_lines: Vec<&str> = all_lines[body_start..].iter().filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#')).copied().collect();

    match header_line {
        CommandHeaderLine::Apply { description } => {
            let mutations = parse_indented_ops(&body_lines).await?;
            if mutations.is_empty() {
                return Err(crate::os_dsl::__rt::field_error("apply requires at least one operation line"));
            }
            Ok(ArtifactCommand::Apply { mutations, description })
        }
        CommandHeaderLine::Undo { policy: None } => Ok(ArtifactCommand::Undo),
        CommandHeaderLine::Undo { policy: Some(token) } => {
            let policy = parse_undo_policy_token(&token).await?;
            let semantic_command = if body_lines.is_empty() {
                None
            } else {
                let dedented = dedent_command_lines(&body_lines).await?;
                Some(Box::new(parse_command::<Op>(&dedented).await?))
            };
            Ok(ArtifactCommand::UndoWithPolicy { policy, semantic_command })
        }
        CommandHeaderLine::Redo => Ok(ArtifactCommand::Redo),
        CommandHeaderLine::CommitCheckpoint { message, by } => Ok(ArtifactCommand::CommitCheckpoint { message, authors: by.into_iter().map(Author::from).collect() }),
        CommandHeaderLine::CreateAlternative { name } => Ok(ArtifactCommand::CreateAlternative { name }),
        CommandHeaderLine::SwitchAlternative { id } => Ok(ArtifactCommand::SwitchAlternative { alternative_id: id }),
        CommandHeaderLine::Checkout { id } => Ok(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: id }),
        CommandHeaderLine::Amend { key } => {
            let mutations = parse_indented_ops(&body_lines).await?;
            if mutations.is_empty() {
                return Err(crate::os_dsl::__rt::field_error("amend requires at least one operation line"));
            }
            Ok(ArtifactCommand::AmendLast { mutations, coalesce_key: key })
        }
        CommandHeaderLine::PruneDrafts => Ok(ArtifactCommand::PruneDrafts),
        CommandHeaderLine::ApplyInLane { description, lane } => {
            let mutations = parse_indented_ops(&body_lines).await?;
            if mutations.is_empty() {
                return Err(crate::os_dsl::__rt::field_error("apply-in-lane requires at least one operation line"));
            }
            Ok(ArtifactCommand::ApplyInLane { mutations, description, lane: parse_history_lane_token(&lane).await? })
        }
        CommandHeaderLine::AmendInLane { key, lane } => {
            let mutations = parse_indented_ops(&body_lines).await?;
            if mutations.is_empty() {
                return Err(crate::os_dsl::__rt::field_error("amend-in-lane requires at least one operation line"));
            }
            Ok(ArtifactCommand::AmendLastInLane { mutations, coalesce_key: key, lane: parse_history_lane_token(&lane).await? })
        }
        CommandHeaderLine::UndoInLane { lane } => Ok(ArtifactCommand::UndoInLane { lane: parse_history_lane_token(&lane).await? }),
        CommandHeaderLine::RedoInLane { lane } => Ok(ArtifactCommand::RedoInLane { lane: parse_history_lane_token(&lane).await? }),
        CommandHeaderLine::SetMergePolicy { policy } => Ok(ArtifactCommand::SetMergePolicy { policy: parse_merge_policy_token(&policy).await? }),
        CommandHeaderLine::ResolveConflict { conflict_id, resolution } => Ok(ArtifactCommand::ResolveConflict { conflict_id, resolution: parse_conflict_resolution_token(&resolution).await? }),
    }
}

/// @emoji 🎯️ Format byte every encoded command starts with — matches `crate::os_dsl::op_rt::OP_BINARY_FORMAT`
/// (B-R6 "one wire convention": `format u8 | ordinal varint | record body`).
pub const COMMAND_BINARY_FORMAT: u8 = 1;

async fn write_command_str(out: &mut Vec<u8>, s: &str) {
    crate::os_pack::write_varint_u64(out, s.len() as u64).await;
    out.extend_from_slice(s.as_bytes());
}

async fn read_command_str(reader: &mut crate::os_pack::ByteReader<'_>) -> Result<String, crate::os_spr::ProtocolError> {
    let len = reader.read_varint_u64().await?;
    let bytes = reader.read_bytes(len as usize).await?;
    std::str::from_utf8(bytes).map(str::to_string).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "command string", offset: 0, detail: error.to_string() })
}

async fn write_command_ops<Op: OpBinary>(out: &mut Vec<u8>, mutations: &[Op]) -> Result<(), crate::os_spr::ProtocolError> {
    crate::os_pack::write_varint_u64(out, mutations.len() as u64).await;
    for mutation in mutations {
        let bytes = mutation.encode_op().await?;
        crate::os_pack::write_varint_u64(out, bytes.len() as u64).await;
        out.extend_from_slice(&bytes);
    }
    Ok(())
}

async fn read_command_ops<Op: OpBinary>(reader: &mut crate::os_pack::ByteReader<'_>) -> Result<Vec<Op>, crate::os_spr::ProtocolError> {
    let count = reader.read_varint_u64().await?;
    let mut mutations = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let len = reader.read_varint_u64().await?;
        let bytes = reader.read_bytes(len as usize).await?;
        mutations.push(Op::decode_op(bytes).await?);
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
    async fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        let mut out = vec![COMMAND_BINARY_FORMAT];
        match self {
            ArtifactCommand::Apply { mutations, description } => {
                crate::os_pack::write_varint_u64(&mut out, 0).await;
                out.push(if description.is_some() { 0b01 } else { 0 });
                if let Some(text) = description {
                    write_command_str(&mut out, text).await;
                }
                write_command_ops(&mut out, mutations).await?;
            }
            ArtifactCommand::Undo => crate::os_pack::write_varint_u64(&mut out, 1).await,
            ArtifactCommand::Redo => crate::os_pack::write_varint_u64(&mut out, 2).await,
            ArtifactCommand::UndoWithPolicy { policy, semantic_command } => {
                crate::os_pack::write_varint_u64(&mut out, 3).await;
                out.push(undo_policy_ordinal(*policy).await);
                out.push(if semantic_command.is_some() { 0b01 } else { 0 });
                if let Some(nested) = semantic_command {
                    let nested_bytes = nested.encode_op().await?;
                    crate::os_pack::write_varint_u64(&mut out, nested_bytes.len() as u64).await;
                    out.extend_from_slice(&nested_bytes);
                }
            }
            ArtifactCommand::CommitCheckpoint { message, authors } => {
                crate::os_pack::write_varint_u64(&mut out, 4).await;
                out.push(if message.is_some() { 0b01 } else { 0 });
                if let Some(text) = message {
                    write_command_str(&mut out, text).await;
                }
                crate::os_pack::write_varint_u64(&mut out, authors.len() as u64).await;
                for author in authors {
                    write_command_str(&mut out, &author.id).await;
                    write_command_str(&mut out, &author.name).await;
                }
            }
            ArtifactCommand::CreateAlternative { name } => {
                crate::os_pack::write_varint_u64(&mut out, 5).await;
                write_command_str(&mut out, name).await;
            }
            ArtifactCommand::SwitchAlternative { alternative_id } => {
                crate::os_pack::write_varint_u64(&mut out, 6).await;
                write_command_str(&mut out, alternative_id).await;
            }
            ArtifactCommand::CheckoutCheckpoint { checkpoint_id } => {
                crate::os_pack::write_varint_u64(&mut out, 7).await;
                write_command_str(&mut out, checkpoint_id).await;
            }
            ArtifactCommand::AmendLast { mutations, coalesce_key } => {
                crate::os_pack::write_varint_u64(&mut out, 8).await;
                out.push(if coalesce_key.is_some() { 0b01 } else { 0 });
                if let Some(key) = coalesce_key {
                    write_command_str(&mut out, key).await;
                }
                write_command_ops(&mut out, mutations).await?;
            }
            ArtifactCommand::IngestRemote { envelope } => {
                crate::os_pack::write_varint_u64(&mut out, 9).await;
                let mut bytes = Vec::new();
                crate::os_spr::encode_envelope(envelope, &mut bytes).await;
                crate::os_pack::write_varint_u64(&mut out, bytes.len() as u64).await;
                out.extend_from_slice(&bytes);
            }
            ArtifactCommand::PruneDrafts => crate::os_pack::write_varint_u64(&mut out, 10).await,
            ArtifactCommand::ApplyInLane { mutations, description, lane } => {
                crate::os_pack::write_varint_u64(&mut out, 11).await;
                out.push(if description.is_some() { 0b01 } else { 0 });
                if let Some(text) = description {
                    write_command_str(&mut out, text).await;
                }
                out.push(history_lane_ordinal(*lane).await);
                write_command_ops(&mut out, mutations).await?;
            }
            ArtifactCommand::AmendLastInLane { mutations, coalesce_key, lane } => {
                crate::os_pack::write_varint_u64(&mut out, 12).await;
                out.push(if coalesce_key.is_some() { 0b01 } else { 0 });
                if let Some(key) = coalesce_key {
                    write_command_str(&mut out, key).await;
                }
                out.push(history_lane_ordinal(*lane).await);
                write_command_ops(&mut out, mutations).await?;
            }
            ArtifactCommand::UndoInLane { lane } => {
                crate::os_pack::write_varint_u64(&mut out, 13).await;
                out.push(history_lane_ordinal(*lane).await);
            }
            ArtifactCommand::RedoInLane { lane } => {
                crate::os_pack::write_varint_u64(&mut out, 14).await;
                out.push(history_lane_ordinal(*lane).await);
            }
            ArtifactCommand::SetMergePolicy { policy } => {
                crate::os_pack::write_varint_u64(&mut out, 15).await;
                out.push(policy.as_u8().await);
            }
            ArtifactCommand::ResolveConflict { conflict_id, resolution } => {
                crate::os_pack::write_varint_u64(&mut out, 16).await;
                write_command_str(&mut out, conflict_id).await;
                out.push(conflict_resolution_ordinal(*resolution).await);
            }
        }
        Ok(out)
    }

    async fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        let mut reader = crate::os_pack::ByteReader::new(bytes).await;
        let format = reader.read_u8().await?;
        if format != COMMAND_BINARY_FORMAT {
            return Err(crate::os_spr::ProtocolError::Malformed { what: "command format", offset: 0, detail: format!("unsupported command format {format}") });
        }
        let ordinal = reader.read_varint_u64().await?;
        match ordinal {
            0 => {
                let presence = reader.read_u8().await?;
                let description = if presence & 0b01 != 0 { Some(read_command_str(&mut reader).await?) } else { None };
                let mutations = read_command_ops(&mut reader).await?;
                Ok(ArtifactCommand::Apply { mutations, description })
            }
            1 => Ok(ArtifactCommand::Undo),
            2 => Ok(ArtifactCommand::Redo),
            3 => {
                let policy = undo_policy_from_ordinal(reader.read_u8().await?).await?;
                let presence = reader.read_u8().await?;
                let semantic_command = if presence & 0b01 != 0 {
                    let len = reader.read_varint_u64().await?;
                    let nested_bytes = reader.read_bytes(len as usize).await?;
                    Some(Box::new(ArtifactCommand::<Op>::decode_op(nested_bytes).await?))
                } else {
                    None
                };
                Ok(ArtifactCommand::UndoWithPolicy { policy, semantic_command })
            }
            4 => {
                let presence = reader.read_u8().await?;
                let message = if presence & 0b01 != 0 { Some(read_command_str(&mut reader).await?) } else { None };
                let author_count = reader.read_varint_u64().await?;
                let mut authors = Vec::with_capacity(author_count as usize);
                for _ in 0..author_count {
                    let id = read_command_str(&mut reader).await?;
                    let name = read_command_str(&mut reader).await?;
                    authors.push(Author { id, name, avatar: None });
                }
                Ok(ArtifactCommand::CommitCheckpoint { message, authors })
            }
            5 => Ok(ArtifactCommand::CreateAlternative { name: read_command_str(&mut reader).await? }),
            6 => Ok(ArtifactCommand::SwitchAlternative { alternative_id: read_command_str(&mut reader).await? }),
            7 => Ok(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: read_command_str(&mut reader).await? }),
            8 => {
                let presence = reader.read_u8().await?;
                let coalesce_key = if presence & 0b01 != 0 { Some(read_command_str(&mut reader).await?) } else { None };
                let mutations = read_command_ops(&mut reader).await?;
                Ok(ArtifactCommand::AmendLast { mutations, coalesce_key })
            }
            9 => {
                let len = reader.read_varint_u64().await?;
                let bytes = reader.read_bytes(len as usize).await?;
                let mut pos = 0;
                let envelope = crate::os_spr::decode_envelope(bytes, &mut pos).await.map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "ingest envelope", offset: 0, detail: error.to_string() })?;
                Ok(ArtifactCommand::IngestRemote { envelope })
            }
            10 => Ok(ArtifactCommand::PruneDrafts),
            11 => {
                let presence = reader.read_u8().await?;
                let description = if presence & 0b01 != 0 { Some(read_command_str(&mut reader).await?) } else { None };
                let lane = history_lane_from_ordinal(reader.read_u8().await?).await?;
                let mutations = read_command_ops(&mut reader).await?;
                Ok(ArtifactCommand::ApplyInLane { mutations, description, lane })
            }
            12 => {
                let presence = reader.read_u8().await?;
                let coalesce_key = if presence & 0b01 != 0 { Some(read_command_str(&mut reader).await?) } else { None };
                let lane = history_lane_from_ordinal(reader.read_u8().await?).await?;
                let mutations = read_command_ops(&mut reader).await?;
                Ok(ArtifactCommand::AmendLastInLane { mutations, coalesce_key, lane })
            }
            13 => Ok(ArtifactCommand::UndoInLane { lane: history_lane_from_ordinal(reader.read_u8().await?).await? }),
            14 => Ok(ArtifactCommand::RedoInLane { lane: history_lane_from_ordinal(reader.read_u8().await?).await? }),
            15 => {
                let policy = crate::os_spr::MergePolicy::from_u8(reader.read_u8().await?).await.ok_or_else(|| crate::os_spr::ProtocolError::Malformed { what: "merge policy ordinal", offset: 1, detail: "unknown merge policy ordinal".to_string() })?;
                Ok(ArtifactCommand::SetMergePolicy { policy })
            }
            16 => {
                let conflict_id = read_command_str(&mut reader).await?;
                let resolution = conflict_resolution_from_ordinal(reader.read_u8().await?).await?;
                Ok(ArtifactCommand::ResolveConflict { conflict_id, resolution })
            }
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

async fn checkpoint_alternatives<'a, P, Mutation>(envelope: &'a ArtifactEnvelope<P, Mutation>, checkpoint_id: &str) -> Vec<&'a Alternative> {
    envelope.vcs.alternatives.iter().filter(|alternative| alternative.checkpoint_ids.iter().any(|id| id == checkpoint_id)).collect()
}

async fn is_checkpoint_main_only<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, checkpoint_id: &str) -> bool {
    checkpoint_alternatives(envelope, checkpoint_id).await.is_empty()
}

async fn has_main_only_descendant<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, children_of: &HashMap<String, Vec<String>>, checkpoint_id: &str, seen: &mut HashSet<String>) -> bool {
    if !seen.insert(checkpoint_id.to_string()) {
        return false;
    }
    for child_id in children_of.get(checkpoint_id).into_iter().flatten() {
        // 🌀️ Self-recursive async fn — `Box::pin` at the recursive call site only (R10 shape 3):
        // the fn's own opaque future type would otherwise need to contain itself (E0733).
        if is_checkpoint_main_only(envelope, child_id).await || Box::pin(has_main_only_descendant(envelope, children_of, child_id, seen)).await {
            return true;
        }
    }
    false
}

/// @emoji 🛤️ Assigns each checkpoint a swimlane: alternatives get lanes `1..n` in array order, lane
/// `0` is the main trunk. A checkpoint sits on lane 0 if it belongs to no alternative or has any
/// main-only descendant (cycle-guarded DFS); otherwise it takes its single alternative's lane, or
/// the minimum lane among several. Mirrors premigration `assignHistoryCheckpointLanes`.
async fn assign_history_checkpoint_lanes<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>) -> HashMap<String, usize> {
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
        if is_checkpoint_main_only(envelope, &checkpoint.id).await || has_main_only_descendant(envelope, &children_of, &checkpoint.id, &mut seen).await {
            lane_by_checkpoint_id.insert(checkpoint.id.clone(), 0);
            continue;
        }
        let alternatives = checkpoint_alternatives(envelope, &checkpoint.id);
        let lanes: Vec<usize> = alternatives.await.iter().map(|alternative| *lane_by_alternative.get(&alternative.id).unwrap_or(&0)).collect();
        let lane = if lanes.len() == 1 { lanes[0] } else { lanes.into_iter().min().unwrap_or(0) };
        lane_by_checkpoint_id.insert(checkpoint.id.clone(), lane);
    }
    lane_by_checkpoint_id
}

/// @emoji 📜️ Builds the ancestor-graph rows for a checkpoint history view: newest checkpoint first,
/// each carrying its swimlane, labels (alternative names, `"main"` fallback on the newest unlabeled
/// row), and authors. Mirrors premigration `buildHistoryColumns`.
pub async fn build_history_columns<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>) -> Vec<HistoryColumn> {
    let lane_by_checkpoint_id = assign_history_checkpoint_lanes(envelope).await;
    let mut columns = Vec::with_capacity(envelope.vcs.checkpoints.len());
    for (index, checkpoint) in envelope.vcs.checkpoints.iter().rev().enumerate() {
        let alternatives = checkpoint_alternatives(envelope, &checkpoint.id).await;
        let alternative_ids: Vec<String> = alternatives.iter().map(|alternative| alternative.id.clone()).collect();
        let mut labels: Vec<String> = alternatives.iter().map(|alternative| alternative.name.clone()).collect();
        if labels.is_empty() && index == 0 {
            labels.push("main".into());
        }
        columns.push(HistoryColumn {
            checkpoint_id: checkpoint.id.clone(),
            timestamp: checkpoint.timestamp.clone(),
            labels,
            authors: checkpoint.authors.clone(),
            parent_checkpoint_id: checkpoint.parent_id.clone(),
            description: checkpoint.message.clone(),
            lane: *lane_by_checkpoint_id.get(&checkpoint.id).unwrap_or(&0),
            alternative_ids,
        });
    }
    columns
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
    backbone: Option<Backbones>,
    dag: crate::os_spr::MutationDag,
    applied_edit_ids: Vec<String>,
    redo_edit_ids: Vec<String>,
    edit_sequence: i32,
    generation: u64,
    last_projection_cause: Option<ArtifactProjectionCause>,
    /// @emoji 🧭️ The checkpoint new commits parent onto; advances on commit/checkout/switch. Not
    /// part of the wire envelope — callers that reconstruct the store per call (e.g. a WASM plugin)
    /// must save/restore it themselves via {@link current_checkpoint_id}/{@link set_current_checkpoint_id}.
    current_checkpoint_id: Option<String>,
    /// @emoji 🖋️ Identity of the local actor driving this store. Set from each local `Apply`/
    /// `AmendLast`'s operation author; compared against `Edit.actor` so undo never touches foreign
    /// edits. Not part of the wire envelope — callers that reconstruct the store per call must
    /// save/restore it via {@link local_actor_id}/{@link set_local_actor_id}.
    local_actor_id: Option<String>,
    /// @emoji ⚖️ This store's local `crate::os_spr::MergePolicy` — authority-local state, defaults
    /// to `Normal`, never carried on the wire envelope (never part of an artifact's shared history).
    merge_policy: crate::os_spr::MergePolicy,
    /// @emoji 📒️ Indexed view of the envelope-owned durable mutation-message ledger. Rebuilt from
    /// `ArtifactEnvelope::edit_messages` whenever a complete state is loaded.
    edit_messages: HashMap<String, Vec<crate::os_spr::MutationMessage>>,
    /// @emoji ⏰️ Monotone hybrid logical clock: `tick`s on every local apply, `merge`s in a remote
    /// tick on every ingest — replaces the old per-call `HybridLogicalTimestamp::new(0, now_ms())`
    /// construction in `replay_mutations`. Not part of the wire envelope.
    clock: HybridLogicalTimestamp,
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
    /// @emoji 📨️ Transient handoff from whichever `dispatch_inner` arm actually produced messages
    /// this call (`apply_command`/`amend_command`/`ingest_remote`/`resolve_conflict`) to `dispatch`,
    /// which drains it into the returned `CommandReceipt`. Reset at the top of every `dispatch`
    /// call; every other arm leaves it at its `Default`. Not part of the wire envelope.
    pending_report: PendingCommandReport,
}

/// @emoji 📨️ See `ArtifactStore::pending_report`'s doc.
#[derive(Default)]
struct PendingCommandReport {
    /// 🎯️ Overrides `dispatch`'s tail-diff `edit_ids` guess — needed once a merge can insert
    /// mid-history (`ingest_remote`/`resolve_conflict`), where "everything past `before`" no longer
    /// names only the newly-landed edits.
    edit_ids: Option<Vec<String>>,
    messages: Vec<crate::os_spr::EditMessages>,
    worst: Option<crate::os_dsl::Severity>,
}

/// @emoji 🖋️ Derives an edit's authoring actor from its per-operation metadata (the author of its
/// first operation), so a local edit records who produced it for later `UndoPolicy` classification.
async fn edit_actor_from_meta(mutation_meta: &[MutationMeta]) -> Option<String> {
    mutation_meta.first().and_then(|meta| meta.author_id.clone()).map(|actor_id| actor_id.0)
}

async fn validate_composition_pins(pins: &[crate::os_vcs::CompositionPin]) -> Result<(), VcsError> {
    let mut children = HashSet::new();
    for pin in pins {
        let child_uri = pin.child_ref.to_uri().await;
        let reparsed = crate::os_io::ArtifactRef::parse_uri(&child_uri).await.map_err(VcsError::ValidationFailed)?;
        if reparsed != pin.child_ref || pin.child_ref.artifact_id.trim().is_empty() || pin.checkpoint_id.trim().is_empty() {
            return Err(VcsError::ValidationFailed(format!("invalid composition pin {child_uri}")));
        }
        if !children.insert(child_uri.clone()) {
            return Err(VcsError::ValidationFailed(format!("checkpoint repeats composition child {child_uri}")));
        }
    }
    Ok(())
}

async fn checkpoint_identity(checkpoint: &Checkpoint, changes: &[Change]) -> String {
    content_addressed_checkpoint_id(checkpoint.parent_id.as_deref(), &checkpoint.change_ids, changes, checkpoint.message.as_deref(), &checkpoint.authors, &checkpoint.timestamp, &checkpoint.composition_pins).await
}

impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: Clone + Serialize + DeserializeOwned + ArtifactPack,
    Mutation: Clone + Serialize + DeserializeOwned + self::Mutation<P> + OpBinary + OpText,
{
    async fn seed_runtime_state(envelope: &ArtifactEnvelope<P, Mutation>) -> (crate::os_spr::MutationDag, i32, HybridLogicalTimestamp) {
        let mut dag = crate::os_spr::MutationDag::new();
        let mut edit_sequence = 0;
        // 🌀️ `clock` is mutated in place across the loop below (`merge` takes `&mut self`) — it
        // must be resolved to a plain value ONCE, not re-`.await`ed per iteration (R10 shape 2).
        let mut clock = HybridLogicalTimestamp::new(0, now_ms().await).await;
        for edit in &envelope.vcs.edits {
            edit_sequence = edit_sequence.max(edit.sequence_number);
            dag.seed_applied(MutationId(edit.id.clone()));
            for mutation_id in crate::os_spr::mutation_ids_for_edit(edit).await {
                dag.seed_applied(mutation_id);
            }
            for meta in &edit.mutation_meta {
                clock.merge(&meta.timestamp).await;
            }
        }
        (dag, edit_sequence, clock)
    }

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
    /// treated as applied in authoritative history order.
    pub async fn new(envelope: ArtifactEnvelope<P, Mutation>) -> Result<Self, VcsError> {
        validate_durable_history(&envelope).await?;
        let (applied_edit_ids, redo_edit_ids, current_checkpoint_id) = match &envelope.cursor {
            Some(cursor) => (cursor.applied_edit_ids.clone(), cursor.redo_edit_ids.clone(), cursor.checkpoint_id.clone().or_else(|| envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone()))),
            None => (envelope.vcs.edits.iter().map(|edit| edit.id.clone()).collect(), Vec::new(), envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone())),
        };
        validate_history_lanes(&envelope, &applied_edit_ids, &redo_edit_ids).await?;
        let current = Self::fold_history(&envelope, &applied_edit_ids).await?;
        let local_actor_id = applied_edit_ids.last().and_then(|edit_id| envelope.vcs.edits.iter().find(|edit| edit.id == *edit_id)).and_then(|edit| edit.actor.clone());
        let edit_messages = envelope.edit_messages.iter().map(|entry| (entry.edit_id.clone(), entry.messages.clone())).collect();
        let (dag, edit_sequence, clock) = Self::seed_runtime_state(&envelope).await;
        Ok(Self {
            envelope,
            backbone: None,
            dag,
            applied_edit_ids,
            redo_edit_ids,
            edit_sequence,
            generation: 0,
            last_projection_cause: None,
            current_checkpoint_id,
            local_actor_id,
            merge_policy: crate::os_spr::MergePolicy::default(),
            edit_messages,
            clock,
            current,
            tail_undo_cache: None,
            pending_report: PendingCommandReport::default(),
        })
    }

    pub async fn generation(&self) -> u64 {
        self.generation
    }

    /// 🧬️ Returns the current history identity used to invalidate derived work.
    pub async fn artifact_revision(&self) -> ArtifactRevision {
        ArtifactRevision { artifact_id: self.envelope.id.clone(), schema: self.envelope.schema.clone(), applied_edit_ids: self.applied_edit_ids.clone(), redo_edit_ids: self.redo_edit_ids.clone(), checkpoint_id: self.current_checkpoint_id.clone() }
    }

    /// 🎯️ Returns the exact revision plus local generation a projection result must match.
    pub async fn projection_stamp(&self) -> ArtifactProjectionStamp {
        ArtifactProjectionStamp { revision: self.artifact_revision().await, generation: self.generation }
    }

    /// 📬️ Captures one immutable projection or inference input from the current reconciled snapshot.
    pub async fn projection_event<Previous, Policy>(&self, cause: ArtifactProjectionCause, previous: Option<Previous>, cache_mode: ArtifactProjectionCacheMode, policy: Policy) -> Result<ArtifactProjectionEvent<P, Previous, Policy>, VcsError> {
        Ok(ArtifactProjectionEvent { stamp: self.projection_stamp().await, cause, state: self.snapshot().await?, previous, cache_mode, policy })
    }

    /// 📣️ Returns the last successful transition through the shared projection invalidation seam.
    pub async fn last_projection_invalidation(&self) -> Option<ArtifactProjectionInvalidation> {
        // 🌀️ `projection_stamp` is async; `Option::map`'s closure is sync (R10 shape 1).
        match self.last_projection_cause {
            Some(cause) => Some(ArtifactProjectionInvalidation { cause, stamp: self.projection_stamp().await }),
            None => None,
        }
    }

    /// 🔄️ Invalidates projections after a verified replay that did not otherwise change history.
    pub async fn invalidate_after_replay(&mut self) -> ArtifactProjectionInvalidation {
        self.invalidate_projections(ArtifactProjectionCause::Replay).await
    }

    /// 🛂️ Invalidates projections after a policy change outside semantic event history.
    pub async fn invalidate_after_policy_change(&mut self) -> ArtifactProjectionInvalidation {
        self.invalidate_projections(ArtifactProjectionCause::PolicyChange).await
    }

    /// 🔗️ Invalidates projections after an external resource changed beneath the artifact.
    pub async fn invalidate_after_external_resource_change(&mut self) -> ArtifactProjectionInvalidation {
        self.invalidate_projections(ArtifactProjectionCause::ExternalResourceChange).await
    }

    async fn invalidate_projections(&mut self, cause: ArtifactProjectionCause) -> ArtifactProjectionInvalidation {
        self.bump();
        self.last_projection_cause = Some(cause);
        ArtifactProjectionInvalidation { cause, stamp: self.projection_stamp().await }
    }

    /// 🛂️ Accepts only a result computed for the exact current revision and generation. Accepted
    /// semantic diffs are returned to their owning strict-apply boundary, never applied here.
    pub async fn accept_projection_result<Output, Diff>(&self, result: ArtifactProjectionResult<Output, Diff>) -> Result<AcceptedArtifactProjection<Output, Diff>, StaleArtifactProjection> {
        // 🌀️ A future is consumed by one `.await` (R10 shape 2) — resolved once, reused below.
        let current = self.projection_stamp().await;
        if result.stamp != current {
            return Err(StaleArtifactProjection { computed_for: result.stamp, current });
        }
        Ok(AcceptedArtifactProjection { output: result.output, proposed_diff: result.proposed_diff })
    }

    pub async fn envelope(&self) -> &ArtifactEnvelope<P, Mutation> {
        &self.envelope
    }

    /// @emoji 👁️ Read-only envelope view — prefer this over mutating through public fields.
    pub async fn envelope_view(&self) -> ArtifactEnvelopeView<'_, P, Mutation> {
        ArtifactEnvelopeView { envelope: &self.envelope }
    }

    pub async fn applied_edit_ids(&self) -> &[String] {
        &self.applied_edit_ids
    }

    /// @emoji ↪️ Pending redo stack (edit ids undone since the last fresh `Apply`).
    pub async fn redo_edit_ids(&self) -> &[String] {
        &self.redo_edit_ids
    }

    /// @emoji 🧭️ The checkpoint new commits currently parent onto (defaults to the latest checkpoint
    /// on construction/`set_state`; advances on commit/checkout/switch).
    pub async fn current_checkpoint_id(&self) -> Option<&str> {
        self.current_checkpoint_id.as_deref()
    }

    /// @emoji 🧭️ Restores the checkout position after reconstructing the store from a serialized
    /// envelope (`set_state` resets it to the latest checkpoint, which is wrong once a caller has
    /// checked out an older one).
    #[must_use]
    pub async fn set_current_checkpoint_id(&mut self, checkpoint_id: Option<String>) -> Result<ArtifactProjectionInvalidation, VcsError> {
        if let Some(checkpoint_id) = &checkpoint_id {
            if !self.envelope.vcs.checkpoints.iter().any(|checkpoint| checkpoint.id == *checkpoint_id) {
                return Err(VcsError::UnknownChange(checkpoint_id.clone()));
            }
        }
        self.current_checkpoint_id = checkpoint_id;
        Ok(self.invalidate_projections(ArtifactProjectionCause::Checkout).await)
    }

    /// @emoji 🖋️ The local actor id used to distinguish this store's own edits from ingested ones.
    /// Not part of the wire envelope — a caller reconstructing the store per call must save/restore
    /// it via {@link set_local_actor_id} for `UndoPolicy` to keep classifying foreign edits.
    pub async fn local_actor_id(&self) -> Option<&str> {
        self.local_actor_id.as_deref()
    }

    /// @emoji 🖋️ Sets the local actor id (see {@link local_actor_id}). Called automatically from each
    /// local `Apply`/`AmendLast`; callers that reconstruct the store per dispatch restore it here.
    pub async fn set_local_actor_id(&mut self, actor_id: Option<String>) {
        self.local_actor_id = actor_id;
    }

    /// @emoji 🔧️ The most recently created/amended edit's `(forwards, inverse, per-operation meta)`.
    /// Used right after `dispatch(Apply{..})`/`AmendLast` to build a `KernelMutation`/`InvocationResult`
    /// with a true inverse from the just-recorded `Edit.inverse`.
    pub async fn edit_mutations(&self) -> Option<(&[Mutation], &[Mutation], &[MutationMeta])> {
        self.envelope.vcs.edits.last().map(|edit| (edit.forwards.as_slice(), edit.inverse.as_slice(), edit.mutation_meta.as_slice()))
    }

    /// @emoji 📜️ Ancestor-graph rows for this store's checkpoint history. See {@link build_history_columns}.
    pub async fn history_columns(&self) -> Vec<HistoryColumn> {
        build_history_columns(&self.envelope).await
    }

    /// @emoji ♻️ Sole public reload API — replaces the former public `set_state`/`set_envelope` escape hatches.
    pub async fn reset(&mut self, envelope: ArtifactEnvelope<P, Mutation>, applied_edit_ids: Vec<String>, redo_edit_ids: Vec<String>) -> Result<CommandReceipt, VcsError> {
        self.set_state(envelope, applied_edit_ids, redo_edit_ids).await?;
        self.last_projection_cause = Some(ArtifactProjectionCause::Reset);
        Ok(CommandReceipt { edit_ids: self.applied_edit_ids.clone(), generation: self.generation().await, messages: Vec::new(), worst: None })
    }

    /// @emoji 💾️ Restores full store state including the redo stack, so `Redo` survives
    /// round-tripping through a serialized envelope (e.g. one `dispatch` call per request).
    pub(crate) async fn set_state(&mut self, envelope: ArtifactEnvelope<P, Mutation>, applied_edit_ids: Vec<String>, redo_edit_ids: Vec<String>) -> Result<(), VcsError> {
        validate_durable_history(&envelope).await?;
        validate_history_lanes(&envelope, &applied_edit_ids, &redo_edit_ids).await?;
        let current = Self::fold_history(&envelope, &applied_edit_ids).await?;
        let (dag, edit_sequence, clock) = Self::seed_runtime_state(&envelope).await;
        self.backbone = None;
        self.edit_sequence = edit_sequence;
        self.current_checkpoint_id = envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone());
        self.envelope = envelope;
        self.dag = dag;
        self.clock = clock;
        self.applied_edit_ids = applied_edit_ids;
        self.redo_edit_ids = redo_edit_ids;
        self.edit_messages = self.envelope.edit_messages.iter().map(|entry| (entry.edit_id.clone(), entry.messages.clone())).collect();
        self.tail_undo_cache = None;
        self.current = current;
        self.bump();
        Ok(())
    }

    /// @emoji 🧭️ Restores applied edits + checkout position for `checkpoint_id`, clearing redo.
    /// Shared by `createAlternative`/`switchAlternative`/`checkoutCheckpoint`. Mirrors premigration
    /// `checkoutCheckpointInternal`. Cold path: reassigns `applied_edit_ids` wholesale (not a tail
    /// append), so `current` is recomputed by a full raw-fold rather than an incremental update.
    async fn checkout_checkpoint_internal(&mut self, checkpoint_id: String) -> Result<(), VcsError> {
        let checkpoint = self.envelope.vcs.checkpoints.iter().find(|checkpoint| checkpoint.id == checkpoint_id).ok_or_else(|| VcsError::UnknownChange(checkpoint_id.clone()))?;
        // 🌀️ `applied` is both borrowed and moved below — a future can only be awaited once
        // (R10 shape 2), so it is resolved to a plain `Vec<String>` here.
        let applied = edit_ids_for_changes(&self.envelope, &checkpoint.change_ids).await;
        let current = Self::fold_history(&self.envelope, &applied).await?;
        self.applied_edit_ids = applied;
        self.redo_edit_ids.clear();
        self.current_checkpoint_id = Some(checkpoint_id);
        self.tail_undo_cache = None;
        self.current = current;
        Ok(())
    }

    /// @emoji 📌️ Records the composed children a checkpoint was taken against. Not reachable through
    /// an ordinary `Apply` (no mutation can touch checkpoint metadata), so — like `set_owner` — it
    /// needs its own setter. Called by `VcsArtifactApp`'s checkpoint cascade right after the
    /// checkpoint is created, since a pin can only name a child checkpoint that already exists.
    /// Re-derives `content_addressed_checkpoint_id` so the checkpoint's identity actually covers its
    /// pins rather than merely storing them beside it.
    #[must_use]
    pub async fn set_checkpoint_composition_pins(&mut self, checkpoint_id: &str, pins: Vec<crate::os_vcs::CompositionPin>) -> Result<ArtifactProjectionInvalidation, VcsError> {
        validate_composition_pins(&pins).await?;
        let target_index = self.envelope.vcs.checkpoints.iter().position(|checkpoint| checkpoint.id == checkpoint_id).ok_or_else(|| VcsError::UnknownChange(checkpoint_id.to_string()))?;
        if self.envelope.vcs.checkpoints.iter().any(|checkpoint| checkpoint.parent_id.as_deref() == Some(checkpoint_id)) {
            return Err(VcsError::ValidationFailed(format!("cannot repin checkpoint {checkpoint_id} after a descendant exists")));
        }
        let mut candidate = self.envelope.clone();
        let previous_id = candidate.vcs.checkpoints[target_index].id.clone();
        candidate.vcs.checkpoints[target_index].composition_pins = pins;
        let next_id = checkpoint_identity(&candidate.vcs.checkpoints[target_index], &candidate.vcs.changes).await;
        if next_id != previous_id && candidate.vcs.checkpoints.iter().enumerate().any(|(index, checkpoint)| index != target_index && checkpoint.id == next_id) {
            return Err(VcsError::ValidationFailed(format!("rederived checkpoint identity {next_id} collides with established history")));
        }
        candidate.vcs.checkpoints[target_index].id = next_id.clone();
        for alternative in &mut candidate.vcs.alternatives {
            for id in &mut alternative.checkpoint_ids {
                if *id == previous_id {
                    *id = next_id.clone();
                }
            }
        }
        if candidate.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.as_ref()) == Some(&previous_id) {
            if let Some(cursor) = &mut candidate.cursor {
                cursor.checkpoint_id = Some(next_id.clone());
            }
        }
        validate_durable_history(&candidate).await?;
        self.envelope = candidate;
        if self.current_checkpoint_id.as_deref() == Some(checkpoint_id) {
            self.current_checkpoint_id = Some(next_id);
        }
        Ok(self.invalidate_projections(ArtifactProjectionCause::Checkpoint).await)
    }

    /// @emoji ⚡️ The live snapshot: the incrementally-maintained `current` fold, as-is. Always `Ok`
    /// in practice (kept as `Result` for API stability); O(1) instead of a full replay. See the
    /// `current` field doc for the maintenance invariant.
    pub async fn snapshot(&self) -> Result<P, VcsError> {
        Ok(self.current.clone())
    }

    /// @emoji 🔂️ Full raw fold of `initial_snapshot` over every `forwards` op in `applied_edit_ids`
    /// order, WITHOUT the final `Mutation::reconcile` pass — the from-scratch computation `current`
    /// is an incrementally-maintained cache of. Used to recompute `current` on the cold paths that
    /// reassign `applied_edit_ids` wholesale instead of appending/popping its tail.
    async fn fold_current(&self) -> Result<P, VcsError> {
        fold_history(&self.envelope, &self.applied_edit_ids).await
    }

    async fn fold_history(envelope: &ArtifactEnvelope<P, Mutation>, applied_edit_ids: &[String]) -> Result<P, VcsError> {
        fold_history(envelope, applied_edit_ids).await
    }

    /// 🔁️ Core replay for steps 5–9 of `ingest_remote`/`resolve_conflict`/`merge_remote_snapshot`
    /// (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6): folds
    /// `order[k..]` from `base`, recomputing each op's `diff` outcome against the SHIFTED state so
    /// its inverse can be rebased and its messages collected — `forwards`/`mutation_meta` for edits
    /// already present in `edits` are read back unchanged, never rewritten. Pure: the caller decides
    /// accept/reject and commits (or discards) the result. Returns the replayed state, each
    /// suffix edit's rebased inverse (keyed by edit id), and each suffix edit's messages in order.
    async fn replay_suffix(base: &P, order: &[String], k: usize, edits: &HashMap<String, Edit<Mutation>>) -> Result<(P, HashMap<String, Vec<Mutation>>, Vec<crate::os_spr::EditMessages>), VcsError> {
        let mut state = base.clone();
        let mut rebased_inverse = HashMap::new();
        let mut replayed = Vec::new();
        for edit_id in &order[k..] {
            // 🎯️ `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` J1 robustness
            // pass (HIGH-1): an id named in `order` but absent from `edits` means `applied_edit_ids`/
            // `vcs.edits` fell out of sync (crash/recovery, a partially-applied ingest) — event
            // sourcing must not silently drop the edit and compute a wrong snapshot, so this is a
            // loud, typed, structural `VcsError`, not a bare `continue`.
            let edit = edits.get(edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
            let mut edit_messages = Vec::new();
            let mut inverse = Vec::new();
            for (op_index, op) in edit.forwards.iter().enumerate() {
                let outcome = op.diff(&state).await.stamp_op_index(op_index as u32);
                let (diff, op_messages) = outcome.await.into_parts().await;
                edit_messages.extend(op_messages);
                // 🌀️ `.await` resolves to an owned value — `.reverse()` on the unawaited future's
                // result was mutating a throwaway temporary, never `back` itself.
                let mut back = op.inverse(&state).await;
                back.reverse();
                inverse.extend(back);
                state = diff.apply(&state).await?;
            }
            rebased_inverse.insert(edit_id.clone(), inverse);
            replayed.push(crate::os_spr::EditMessages { edit_id: edit_id.clone(), messages: edit_messages });
        }
        Ok((state, rebased_inverse, replayed))
    }

    /// 🔀️ Like {@link replay_suffix}, but decides accept-vs-quarantine PER EDIT instead of
    /// atomically over the whole suffix (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-
    /// CLASS-CONFLICTS` §C6 step 7-8, H1 determinism fix). An edit already committed by an
    /// earlier `ingest_remote` call, once a later-arriving earlier-HLC envelope forces
    /// `order[k..]` to include it again, gets exactly the accept/quarantine verdict it would have
    /// gotten had every envelope in `order[k..]` arrived in true HLC order in the first place —
    /// so the merge converges to the same `applied_edit_ids`/state/conflicts regardless of which
    /// of two separate `ingest_remote` calls happened first. A quarantined edit's forward ops and
    /// `MutationMeta` are never touched (they stay exactly as recorded); only its diff/inverse/
    /// messages were freshly recomputed above, and it is simply left out of `committed_ids` — the
    /// same "diffs/inverses/messages get recomputed, forwards ops never rewritten" contract an
    /// `Undo` already relies on (id drops out of `applied_edit_ids`, the `Edit` record stays put).
    #[allow(clippy::type_complexity)]
    async fn replay_suffix_partitioned(base: &P, order: &[String], k: usize, edits: &HashMap<String, Edit<Mutation>>, policy: crate::os_spr::MergePolicy) -> Result<(P, Vec<String>, Vec<String>, HashMap<String, Vec<Mutation>>, Vec<crate::os_spr::EditMessages>), VcsError> {
        let mut state = base.clone();
        let mut committed_ids = Vec::new();
        let mut quarantined_ids = Vec::new();
        let mut rebased_inverse = HashMap::new();
        let mut replayed = Vec::new();
        for edit_id in &order[k..] {
            // 🎯️ HIGH-1 (same reasoning as `replay_suffix` above): a ghost id here means the store's
            // own bookkeeping desynced, which is corruption, never a mutation-level outcome — surface
            // it as a typed `VcsError` so a caller finds out instead of silently getting a wrong
            // snapshot back.
            let edit = edits.get(edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
            let mut edit_messages = Vec::new();
            let mut inverse = Vec::new();
            let mut candidate_state = state.clone();
            for (op_index, op) in edit.forwards.iter().enumerate() {
                let outcome = op.diff(&candidate_state).await.stamp_op_index(op_index as u32);
                let (diff, op_messages) = outcome.await.into_parts().await;
                edit_messages.extend(op_messages);
                let mut back = op.inverse(&candidate_state).await;
                back.reverse();
                inverse.extend(back);
                candidate_state = diff.apply(&candidate_state).await?;
            }
            let edit_worst = crate::os_spr::worst_level(&edit_messages).await;
            replayed.push(crate::os_spr::EditMessages { edit_id: edit_id.clone(), messages: edit_messages });
            let rejects = match edit_worst {
                Some(level) => policy.rejects(level).await,
                None => false,
            };
            if rejects {
                quarantined_ids.push(edit_id.clone());
                continue;
            }
            state = candidate_state;
            rebased_inverse.insert(edit_id.clone(), inverse);
            committed_ids.push(edit_id.clone());
        }
        Ok((state, committed_ids, quarantined_ids, rebased_inverse, replayed))
    }

    /// 🎯️ HIGH-2 (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` J1
    /// robustness pass): strict counterpart to a `filter_map` over `edits` — a `ConflictId` is
    /// content-addressed from a mutation-id set, so silently skipping an id missing from `edits`
    /// (the old `filter_map` behaviour) can leave `quarantined_ids`/`degraded_ids` non-empty while
    /// the resolved edit list is empty, minting a `ConflictId` from zero mutation ids — a content
    /// address that addresses no content, so two unrelated conflicts can collide on it. `edits` is
    /// always `edits_by_id` here, and every id passed in already came out of
    /// `replay_suffix_partitioned`'s own `committed_ids`/`quarantined_ids` — which, since HIGH-1,
    /// only ever contains ids it already found in `edits` — so this should never actually fail in
    /// production; it exists so a future refactor that breaks that invariant fails loudly instead of
    /// quietly minting a garbage conflict id.
    async fn edits_for_ids(ids: &[String], edits: &HashMap<String, Edit<Mutation>>) -> Result<Vec<Edit<Mutation>>, VcsError> {
        ids.iter().map(|id| edits.get(id).cloned().ok_or_else(|| VcsError::UnknownEdit(id.clone()))).collect()
    }

    /// 🎯️ MEDIUM-3 (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` J1
    /// robustness pass): `envelope.conflicts` used to be appended on every rejection/degradation and
    /// truncated only incidentally (`resolve_conflict`'s `Quarantined`+`Accept` arm discarding the
    /// re-ingest's own transient conflicts). A peer that keeps sending a batch this replica keeps
    /// quarantining — `ingest_remote`'s dag never advances on quarantine, so the SAME envelope is
    /// eligible for redelivery forever — grows this list, and the persisted `.spr`, without bound.
    /// Two named caps, two different failure modes because `Open`/resolved conflicts carry different
    /// weight: a resolved (`Accepted`/`Discarded`) conflict is a closed historical fact a UI no
    /// longer needs enumerated, so it is PRUNABLE — `prune_resolved_conflicts` evicts the oldest
    /// (lowest push-index, i.e. first non-`Open` entry found scanning from the front) resolved
    /// conflicts once the total exceeds [`Self::RESOLVED_CONFLICT_CAP`]. An `Open` conflict is a
    /// pending decision a human/authority has not yet acted on — silently dropping one would erase
    /// unresolved history, so it is never evicted; instead [`Self::ensure_open_conflict_capacity`]
    /// is checked BEFORE any state mutation in every call site that might mint one, and refuses with
    /// a loud, typed `VcsError` (atomic — nothing applied) once open conflicts would exceed
    /// [`Self::OPEN_CONFLICT_CAP`], forcing existing ones to be resolved before backlog can grow
    /// further. `edit_messages` (the durable per-edit message ledger) was reviewed for the same
    /// pattern and deliberately gets NO cap: unlike `conflicts`, it only grows once per edit that
    /// ever reaches `committed_ids` (`record_edit_messages`/`replace_edit_messages` are never called
    /// for a quarantined id — its entry is explicitly cleared instead), so its size tracks genuine
    /// applied-history growth, not redelivery-of-rejected-content amplification; capping it would
    /// mean silently losing a still-applied edit's own diagnostic record, which is strictly worse.
    const OPEN_CONFLICT_CAP: usize = 256;
    const RESOLVED_CONFLICT_CAP: usize = 512;

    /// 🎯️ MEDIUM-3: refuses to mint `additional_open` more `Open` conflicts once doing so would push
    /// this envelope past [`Self::OPEN_CONFLICT_CAP`]. Callers check this BEFORE mutating any store
    /// field for the call, so a refusal is atomic — see the const's own doc for why open conflicts
    /// are capped-but-never-silently-dropped rather than pruned like resolved ones.
    async fn ensure_open_conflict_capacity(&self, additional_open: usize) -> Result<(), VcsError> {
        if additional_open == 0 {
            return Ok(());
        }
        let open_now = self.envelope.conflicts.iter().filter(|conflict| conflict.status == crate::os_spr::ConflictStatus::Open).count();
        if open_now + additional_open > Self::OPEN_CONFLICT_CAP {
            return Err(VcsError::ValidationFailed(format!("open conflict backlog is at capacity ({open_now} of {} open, {additional_open} more pending) — resolve existing open conflicts before this artifact can record another", Self::OPEN_CONFLICT_CAP)));
        }
        Ok(())
    }

    /// 🎯️ MEDIUM-3: oldest-first eviction of RESOLVED (`Accepted`/`Discarded`) conflicts once the
    /// total exceeds [`Self::RESOLVED_CONFLICT_CAP`] — `Open` conflicts are never touched. See the
    /// const's own doc for the reasoning.
    async fn prune_resolved_conflicts(&mut self) {
        while self.envelope.conflicts.len() > Self::RESOLVED_CONFLICT_CAP {
            let Some(index) = self.envelope.conflicts.iter().position(|conflict| conflict.status != crate::os_spr::ConflictStatus::Open) else { break };
            self.envelope.conflicts.remove(index);
        }
    }

    /// @emoji ⚔️ Every conflict this store has ever raised (`Open`, `Accepted`, and `Discarded`
    /// alike) — see {@link conflicts} field doc.
    pub async fn conflicts(&self) -> &[crate::os_spr::Conflict] {
        &self.envelope.conflicts
    }

    /// @emoji ⚔️ The subset of {@link conflicts} still `Open` — what a UI's Conflicts panel lists.
    pub async fn open_conflicts(&self) -> impl Iterator<Item = &crate::os_spr::Conflict> {
        self.envelope.conflicts.iter().filter(|conflict| conflict.status == crate::os_spr::ConflictStatus::Open)
    }

    /// @emoji ⚖️ This store's local `crate::os_spr::MergePolicy` (see the field doc).
    pub async fn merge_policy(&self) -> crate::os_spr::MergePolicy {
        self.merge_policy
    }

    /// @emoji ⚖️ Sets this store's local `crate::os_spr::MergePolicy`. Local/authority state only —
    /// never wire-carried, never part of shared history; takes effect starting with the NEXT
    /// `dispatch`/`ingest_remote` call.
    pub async fn set_merge_policy(&mut self, policy: crate::os_spr::MergePolicy) {
        self.merge_policy = policy;
    }

    /// @emoji 📒️ Every `crate::os_spr::MutationMessage` `edit_id`'s own replay raised — empty for an
    /// edit that raised none (including an unknown `edit_id`), never an error (see the field doc).
    pub async fn messages_for_edit(&self, edit_id: &str) -> &[crate::os_spr::MutationMessage] {
        self.edit_messages.get(edit_id).map(Vec::as_slice).unwrap_or(&[])
    }

    pub async fn dispatch(&mut self, command: ArtifactCommand<Mutation>) -> Result<CommandReceipt, VcsError> {
        self.pump().await?;
        // 🌀️ The unawaited future borrows `command`, which is then moved into `dispatch_inner`
        // below — awaited immediately instead of deferred to avoid a move-while-borrowed (E0505).
        let projection_cause = command.projection_cause().await;
        // 🎯️ `SetMergePolicy`/`ResolveConflict` never need a full snapshot re-broadcast: the policy
        // is local-only (never wire-carried) and a conflict resolution either replays already-
        // announced remote envelopes (no new local content to announce) or only flips a status.
        let skip_flush = matches!(command, ArtifactCommand::IngestRemote { .. } | ArtifactCommand::PruneDrafts | ArtifactCommand::SetMergePolicy { .. } | ArtifactCommand::ResolveConflict { .. });
        let is_apply = matches!(command, ArtifactCommand::Apply { .. });
        let before = self.applied_edit_ids.len();
        self.pending_report = PendingCommandReport::default();
        self.dispatch_inner(command).await?;
        self.last_projection_cause = projection_cause;
        if !skip_flush {
            self.flush_outbound(is_apply).await?;
        }
        // Undo/redo shrink `applied_edit_ids`; Apply/AmendLast append past `before`; a merge that
        // rebased mid-history overrides this tail-diff guess via `pending_report.edit_ids`.
        let edit_ids = self.pending_report.edit_ids.take().unwrap_or_else(|| if self.applied_edit_ids.len() >= before { self.applied_edit_ids[before..].to_vec() } else { Vec::new() });
        Ok(CommandReceipt { edit_ids, generation: self.generation().await, messages: std::mem::take(&mut self.pending_report.messages), worst: self.pending_report.worst.take() })
    }

    async fn dispatch_inner(&mut self, command: ArtifactCommand<Mutation>) -> Result<(), VcsError> {
        match command {
            // 🌀️ Mutual recursion with `dispatch` (which itself awaits `dispatch_inner`) — `Box::pin`
            // breaks the cycle so neither opaque future type has to embed the other (R10 shape 3).
            ArtifactCommand::Undo => Box::pin(self.dispatch(ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::ExactBaseOnly, semantic_command: None })).await.map(|_| ()),
            // 🛤️ Both branches below now search for the nearest match FROM THE TAIL rather than
            // requiring the literal last id, so a run of trailing `Interaction`-lane entries (e.g.
            // a selection change recorded after the last document edit) is transparently skipped
            // rather than blocking undo entirely — see `HistoryLane`'s doc and `undo_lane_position`.
            ArtifactCommand::UndoWithPolicy { policy, semantic_command } => match policy {
                UndoPolicy::ExactBaseOnly => {
                    // 🌀️ `edit_lane`/`edit_is_local` are async; `rposition`'s closure is sync (R10
                    // shape 1), so lane membership is resolved into a plain `Vec<HistoryLane>` first.
                    let mut lanes = Vec::with_capacity(self.applied_edit_ids.len());
                    for id in &self.applied_edit_ids {
                        lanes.push(self.edit_lane(id).await);
                    }
                    let position = lanes.iter().rposition(|lane| *lane == HistoryLane::Document).ok_or(VcsError::NothingToUndo)?;
                    self.undo_lane_position(position).await
                }
                UndoPolicy::TransformAgainstConcurrent => {
                    let mut candidates = Vec::with_capacity(self.applied_edit_ids.len());
                    for id in &self.applied_edit_ids {
                        candidates.push((self.edit_is_local(id).await, self.edit_lane(id).await));
                    }
                    let position = candidates.iter().rposition(|(is_local, lane)| *is_local && *lane == HistoryLane::Document).ok_or(VcsError::NothingToUndo)?;
                    let removed = self.applied_edit_ids.remove(position);
                    self.redo_edit_ids.push(removed);
                    // 🔂️ Removing a MID-history edit has no cheap incremental inverse; cold-path replay.
                    self.tail_undo_cache = None;
                    self.current = self.fold_current().await?;
                    self.bump();
                    Ok(())
                }
                UndoPolicy::SemanticUndo | UndoPolicy::CompensatingAction => {
                    let command = semantic_command.ok_or_else(|| VcsError::Backbone("semantic undo requires compensating command".into()))?;
                    // 🌀️ Self-recursive async fn (R10 shape 3) — `Box::pin` at the recursive call.
                    Box::pin(self.dispatch_inner(*command)).await
                }
            },
            ArtifactCommand::Redo => {
                let mut lanes = Vec::with_capacity(self.redo_edit_ids.len());
                for id in &self.redo_edit_ids {
                    lanes.push(self.edit_lane(id).await);
                }
                let position = lanes.iter().rposition(|lane| *lane == HistoryLane::Document).ok_or(VcsError::NothingToRedo)?;
                self.redo_lane_position(position).await
            }
            // 🛤️ Explicit lane-scoped mirrors of `UndoWithPolicy { ExactBaseOnly }`/`Redo` above,
            // filtering on an arbitrary caller-chosen `lane` instead of hard-coding `Document` — the
            // "walk a specific lane on purpose" half of the mechanism (see `HistoryLane`'s doc).
            ArtifactCommand::UndoInLane { lane } => {
                let mut lanes = Vec::with_capacity(self.applied_edit_ids.len());
                for id in &self.applied_edit_ids {
                    lanes.push(self.edit_lane(id).await);
                }
                let position = lanes.iter().rposition(|edit_lane| *edit_lane == lane).ok_or(VcsError::NothingToUndo)?;
                self.undo_lane_position(position).await
            }
            ArtifactCommand::RedoInLane { lane } => {
                let mut lanes = Vec::with_capacity(self.redo_edit_ids.len());
                for id in &self.redo_edit_ids {
                    lanes.push(self.edit_lane(id).await);
                }
                let position = lanes.iter().rposition(|edit_lane| *edit_lane == lane).ok_or(VcsError::NothingToRedo)?;
                self.redo_lane_position(position).await
            }
            ArtifactCommand::CommitCheckpoint { message, authors } => {
                // 🌀️ A future is consumed by one `.await` (R10 shape 2) — awaited once into a plain
                // `Vec<String>` instead of re-awaiting `pending` at each use below.
                let pending = uncommitted_edit_ids(&self.envelope, &self.applied_edit_ids).await;
                if pending.is_empty() {
                    return Err(VcsError::ValidationFailed(EMPTY_CHECKPOINT_MESSAGE.to_string()));
                }
                let change = Change { id: mint_change_id(&pending, message.as_deref()).await, edit_ids: pending, description: message.clone(), saved_at: now_iso().await };
                let parent = self.current_checkpoint_id.as_ref().and_then(|id| self.envelope.vcs.checkpoints.iter().find(|cp| cp.id == *id));
                let mut change_ids = parent.map(|cp| cp.change_ids.clone()).unwrap_or_default();
                let parent_id = parent.map(|cp| cp.id.clone());
                change_ids.push(change.id.clone());
                // 🎞️ CW3: the new change is pushed BEFORE computing the checkpoint id (was after),
                // so `content_addressed_checkpoint_id` can hash its actual content, not a placeholder.
                self.envelope.vcs.changes.push(change);
                // 🌀️ `timestamp` is borrowed AND moved below — a future can only be awaited once
                // (R10 shape 2), so it is resolved to a plain `String` before either use.
                let timestamp = now_iso().await;
                // 🎯️ `&[]`: `ArtifactStore<P, Mutation>` has no notion of owned children yet — the
                // `CompositionCoordinator` that dispatches across parent + child stores and
                // populates real `CompositionPin`s here is a later wave (see design doc §1).
                let id = content_addressed_checkpoint_id(parent_id.as_deref(), &change_ids, &self.envelope.vcs.changes, message.as_deref(), &authors, &timestamp, &[]).await;
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
                    if self.applied_edit_ids.is_empty() {
                        return Err(VcsError::NoCheckpoint);
                    }
                    // 🌀️ Same `dispatch`/`dispatch_inner` mutual-recursion cycle as the `Undo` arm.
                    Box::pin(self.dispatch(ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() })).await?;
                }
                let checkpoint_id = self.current_checkpoint_id.clone().or_else(|| self.envelope.vcs.checkpoints.last().map(|cp| cp.id.clone())).ok_or(VcsError::NoCheckpoint)?;
                let alt_id = mint_alternative_id(&name, &[checkpoint_id.clone()]).await;
                self.envelope.vcs.alternatives.push(Alternative { id: alt_id.clone(), name, checkpoint_ids: vec![checkpoint_id.clone()] });
                self.envelope.active_alternative_id = Some(alt_id);
                self.checkout_checkpoint_internal(checkpoint_id).await?;
                self.bump();
                Ok(())
            }
            ArtifactCommand::SwitchAlternative { alternative_id } => {
                let alternative = self.envelope.vcs.alternatives.iter().find(|alt| alt.id == alternative_id).ok_or_else(|| VcsError::UnknownAlternative(alternative_id.clone()))?.clone();
                let checkpoint_id = alternative.checkpoint_ids.last().ok_or(VcsError::NoCheckpoint)?.clone();
                if !self.envelope.vcs.checkpoints.iter().any(|cp| cp.id == checkpoint_id) {
                    return Err(VcsError::NoCheckpoint);
                }
                self.checkout_checkpoint_internal(checkpoint_id).await?;
                self.envelope.active_alternative_id = Some(alternative_id);
                self.bump();
                Ok(())
            }
            ArtifactCommand::CheckoutCheckpoint { checkpoint_id } => {
                if !self.envelope.vcs.checkpoints.iter().any(|cp| cp.id == checkpoint_id) {
                    return Err(VcsError::UnknownChange(checkpoint_id.clone()));
                }
                self.checkout_checkpoint_internal(checkpoint_id.clone()).await?;
                self.envelope.active_alternative_id = self.envelope.vcs.alternatives.iter().find(|alt| alt.checkpoint_ids.last() == Some(&checkpoint_id)).map(|alt| alt.id.clone());
                self.bump();
                Ok(())
            }
            ArtifactCommand::Apply { mutations, description } => self.apply_command(mutations, description, HistoryLane::Document).await,
            // 🛤️ Same edit-recording path as `Apply`, tagging the fresh edit into `lane` instead of
            // the implicit `Document` default — see `HistoryLane`'s doc.
            ArtifactCommand::ApplyInLane { mutations, description, lane } => self.apply_command(mutations, description, lane).await,
            ArtifactCommand::AmendLast { mutations, coalesce_key } => self.amend_command(mutations, coalesce_key, HistoryLane::Document).await,
            ArtifactCommand::AmendLastInLane { mutations, coalesce_key, lane } => self.amend_command(mutations, coalesce_key, lane).await,
            ArtifactCommand::IngestRemote { envelope } => {
                let report = self.ingest_remote(envelope).await?;
                self.absorb_merge_report(&report);
                Ok(())
            }
            ArtifactCommand::PruneDrafts => Err(VcsError::ValidationFailed("draft pruning is not implemented by ArtifactStore; no draft history was removed".to_string())),
            ArtifactCommand::SetMergePolicy { policy } => {
                self.merge_policy = policy;
                Ok(())
            }
            ArtifactCommand::ResolveConflict { conflict_id, resolution } => {
                let report = self.resolve_conflict(&conflict_id, resolution).await?;
                self.absorb_merge_report(&report);
                Ok(())
            }
        }
    }

    /// 📨️ Copies a `MergeReport`'s messages/worst level into `pending_report` for `dispatch` to
    /// hand back as the resulting `CommandReceipt` — shared by the `IngestRemote`/`ResolveConflict`
    /// arms above.
    async fn absorb_merge_report(&mut self, report: &crate::os_spr::MergeReport) {
        if self.pending_report.edit_ids.is_none() {
            self.pending_report.edit_ids = Some(if report.accepted { report.replayed.iter().map(|edit_messages| edit_messages.edit_id.clone()).collect() } else { Vec::new() });
        }
        self.pending_report.messages = report.replayed.clone();
        self.pending_report.worst = report.worst;
    }

    //#region 🔖️HistoryLane
    /// @emoji 🛤️ The lane `edit_id` was recorded under. Absence from `envelope.lanes` means
    /// `Document` by construction — `apply_command`/`amend_command` below only ever insert a map
    /// entry for a NON-`Document` lane, so an ordinary document edit (and every edit that predates
    /// this field) never gets one. `HistoryLane` is `Copy`, so this returns by value like
    /// `edit_is_local` returns `bool`.
    async fn edit_lane(&self, edit_id: &str) -> HistoryLane {
        self.envelope.lanes.get(edit_id).copied().unwrap_or_default()
    }

    /// @emoji 🛤️ Shared tail of every undo path (`UndoWithPolicy::ExactBaseOnly`, `UndoInLane`):
    /// `position` has already been chosen by the caller's own lane/locality search — this just
    /// performs the actual removal, foreign-edit rejection, and `current` recompute (O(1) via
    /// `tail_undo_cache` when `position` really is the tail, cold-path `fold_current` otherwise,
    /// exactly mirroring the pre-lane `ExactBaseOnly` arm's own fast path).
    async fn undo_lane_position(&mut self, position: usize) -> Result<(), VcsError> {
        let target = self.applied_edit_ids[position].clone();
        if !self.edit_is_local(&target).await {
            return Err(VcsError::ForeignEdit(target));
        }
        let is_tail = position + 1 == self.applied_edit_ids.len();
        self.applied_edit_ids.remove(position);
        self.redo_edit_ids.push(target.clone());
        match self.tail_undo_cache.take() {
            Some((cached_id, cached_pre)) if is_tail && cached_id == target => {
                self.current = cached_pre;
            }
            _ => {
                self.current = self.fold_current().await?;
            }
        }
        self.bump();
        Ok(())
    }

    /// @emoji 🛤️ Shared tail of every redo path (plain `Redo`, `RedoInLane`): `position` indexes
    /// `redo_edit_ids` (already chosen by the caller's own lane search), removed from wherever it
    /// sits — not necessarily the stack top, since a lane-filtered search can land mid-vec once
    /// undos from more than one lane have interleaved — folded onto `current` and re-appended to
    /// `applied_edit_ids`, mirroring the pre-lane `Redo` arm's own logic exactly.
    async fn redo_lane_position(&mut self, position: usize) -> Result<(), VcsError> {
        let next = self.redo_edit_ids.remove(position);
        self.applied_edit_ids.push(next.clone());
        if let Some(edit) = self.envelope.vcs.edits.iter().find(|entry| entry.id == next) {
            let pre = self.current.clone();
            let mut folded = pre.clone();
            for operation in &edit.forwards {
                // 🧮️ Mechanical wrap only — see `replay_mutations`'s matching note.
                folded = apply_mutation(&folded, operation).await?.0;
            }
            self.current = folded;
            self.tail_undo_cache = Some((next, pre));
        }
        self.bump();
        Ok(())
    }

    /// @emoji 🛤️ Shared body of `Apply`/`ApplyInLane`: identical edit-recording logic to the
    /// pre-lane `Apply` arm, plus tagging the fresh edit into `envelope.lanes` when `lane` isn't
    /// the default `Document` (kept sparse — see `ArtifactEnvelope.lanes`'s doc).
    async fn apply_command(&mut self, mutations: Vec<Mutation>, description: Option<String>, lane: HistoryLane) -> Result<(), VcsError> {
        if mutations.is_empty() {
            return Err(VcsError::EmptyApply);
        }
        let started_at = now_iso();
        // ⚡️ `current` is always up to date (maintained by every mutating command below), so this
        // is an O(1) clone instead of a full replay — see the `current` field doc.
        let pre_snapshot = self.current.clone();
        let (forwards, inverse, mutation_meta, post, messages) = self.replay_mutations(&pre_snapshot, mutations).await?;
        let actor = edit_actor_from_meta(&mutation_meta).await;
        self.local_actor_id = actor.clone();
        self.edit_sequence += 1;
        let forwards_fingerprint = serde_json::to_vec(&forwards).map_err(|error| VcsError::Serialize(error.to_string()))?;
        let mut edit = Edit {
            id: mint_edit_id(actor.as_deref(), self.edit_sequence, &forwards_fingerprint).await,
            actor,
            forwards,
            inverse,
            mutation_meta,
            description,
            coalesce_key: None,
            sequence_number: self.edit_sequence,
            started_at: started_at.await,
            finished_at: Some(now_iso().await),
        };
        stamp_primary_operation_identity(&mut edit);
        if !lane.is_document().await {
            self.envelope.lanes.insert(edit.id.clone(), lane);
        }
        self.record_edit_messages(&edit.id, messages);
        self.tail_undo_cache = Some((edit.id.clone(), pre_snapshot));
        self.applied_edit_ids.push(edit.id.clone());
        self.envelope.vcs.edits.push(edit);
        self.current = post;
        self.redo_edit_ids.clear();
        self.bump();
        Ok(())
    }

    /// @emoji 🛤️ Shared body of `AmendLast`/`AmendLastInLane`: identical edit-recording logic to
    /// the pre-lane `AmendLast` arm. Only the FRESH-edit branch can tag a lane — an edit absorbed
    /// into an already-coalescing target keeps whatever lane it was first created under (amending
    /// never changes an edit's lane after the fact).
    async fn amend_command(&mut self, mutations: Vec<Mutation>, coalesce_key: Option<String>, lane: HistoryLane) -> Result<(), VcsError> {
        if mutations.is_empty() {
            return Err(VcsError::EmptyApply);
        }
        let uncommitted = uncommitted_edit_ids(&self.envelope, &self.applied_edit_ids).await;
        let amend_target = self.applied_edit_ids.last().cloned().filter(|last_id| {
            coalesce_key.is_some() && uncommitted.contains(last_id) && self.envelope.vcs.edits.iter().find(|edit| edit.id == *last_id).map(|edit| edit.coalesce_key == coalesce_key).unwrap_or(false)
        });
        if let Some(edit_id) = amend_target {
            // ⚡️ `current` already reflects this edit's existing forwards (it was folded in when
            // the edit was created or last amended), so it's always the correct base for the NEW
            // operations — O(1) instead of the old cache-validity dance.
            let pre_snapshot = self.current.clone();
            let (new_forwards, new_inverse, new_mutation_meta, post, messages) = self.replay_mutations(&pre_snapshot, mutations).await?;
            if let Some(edit) = self.envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id) {
                edit.forwards.extend(new_forwards);
                edit.inverse.extend(new_inverse);
                edit.mutation_meta.extend(new_mutation_meta);
                edit.finished_at = Some(now_iso().await);
            }
            self.record_edit_messages(&edit_id, messages);
            self.current = post;
            self.redo_edit_ids.clear();
            self.bump();
            Ok(())
        } else {
            let started_at = now_iso();
            let pre_snapshot = self.current.clone();
            let (forwards, inverse, mutation_meta, post, messages) = self.replay_mutations(&pre_snapshot, mutations).await?;
            let actor = edit_actor_from_meta(&mutation_meta).await;
            self.local_actor_id = actor.clone();
            self.edit_sequence += 1;
            let forwards_fingerprint = serde_json::to_vec(&forwards).map_err(|error| VcsError::Serialize(error.to_string()))?;
            let edit_id = mint_edit_id(actor.as_deref(), self.edit_sequence, &forwards_fingerprint).await;
            let mut edit = Edit { id: edit_id.clone(), actor, forwards, inverse, mutation_meta, description: None, coalesce_key, sequence_number: self.edit_sequence, started_at: started_at.await, finished_at: Some(now_iso().await) };
            stamp_primary_operation_identity(&mut edit);
            if !lane.is_document().await {
                self.envelope.lanes.insert(edit_id.clone(), lane);
            }
            self.record_edit_messages(&edit_id, messages);
            self.tail_undo_cache = Some((edit_id.clone(), pre_snapshot));
            self.applied_edit_ids.push(edit.id.clone());
            self.envelope.vcs.edits.push(edit);
            self.current = post;
            self.redo_edit_ids.clear();
            self.bump();
            Ok(())
        }
    }
    //#endregion 🔖️HistoryLane

    /// 📨️ Records `edit_id`'s replay messages into the durable ledger and `pending_report`, shared
    /// by `apply_command`/`amend_command` — every local dispatch's own `DispatchReport`-shaped tail.
    async fn record_edit_messages(&mut self, edit_id: &str, messages: Vec<crate::os_spr::MutationMessage>) {
        if messages.is_empty() {
            return;
        }
        self.pending_report.worst = crate::os_spr::worst_level(&messages).await.max(self.pending_report.worst);
        self.edit_messages.entry(edit_id.to_string()).or_default().extend(messages.clone());
        if let Some(entry) = self.envelope.edit_messages.iter_mut().find(|entry| entry.edit_id == edit_id) {
            entry.messages.extend(messages.clone());
        } else {
            self.envelope.edit_messages.push(crate::os_spr::EditMessages { edit_id: edit_id.to_string(), messages: messages.clone() });
        }
        self.pending_report.messages.push(crate::os_spr::EditMessages { edit_id: edit_id.to_string(), messages });
    }

    async fn replace_edit_messages(&mut self, edit_id: &str, messages: Vec<crate::os_spr::MutationMessage>) {
        if messages.is_empty() {
            self.edit_messages.remove(edit_id);
            self.envelope.edit_messages.retain(|entry| entry.edit_id != edit_id);
            return;
        }
        self.edit_messages.insert(edit_id.to_string(), messages.clone());
        if let Some(entry) = self.envelope.edit_messages.iter_mut().find(|entry| entry.edit_id == edit_id) {
            entry.messages = messages;
        } else {
            self.envelope.edit_messages.push(crate::os_spr::EditMessages { edit_id: edit_id.to_string(), messages });
        }
    }

    /// @emoji 🔂️ Replays `operations` over `pre_snapshot`, returning forwards, reversed-inverse,
    /// per-operation metadata, the resulting snapshot, and every `crate::os_spr::MutationMessage`
    /// the replay raised. Shared by `Apply` and `AmendLast`. This IS the artifact engine —
    /// `crate::os_engine::ArtifactEngine` never existed as a live trait (see
    /// `.claude/plans/the-mutations-are-extremely-compiled-pumpkin.md`), so `Mutation::diff`/
    /// `inverse` are called directly here on purpose, not as a placeholder for a future indirection.
    /// ATOMIC: if `self.merge_policy.rejects(worst)`, returns `Err` before this method's caller has
    /// touched any store field — nothing about the attempted batch is applied.
    async fn replay_mutations(&mut self, pre_snapshot: &P, mutations: Vec<Mutation>) -> Result<(Vec<Mutation>, Vec<Mutation>, Vec<MutationMeta>, P, Vec<crate::os_spr::MutationMessage>), VcsError> {
        let mut snapshot = pre_snapshot.clone();
        let mut candidate_clock = self.clock;
        let mut forwards = Vec::with_capacity(mutations.len());
        let mut inverse = Vec::new();
        let mut mutation_meta = Vec::with_capacity(mutations.len());
        let mut messages = Vec::new();
        for (op_index, mutation) in mutations.into_iter().enumerate() {
            let encoded = mutation.encode_op().await.map_err(|error| VcsError::ValidationFailed(error.to_string()))?;
            // 🌀️ `.await` resolves to an owned value — `.reverse()` on the unawaited future's
            // result was mutating a throwaway temporary, never `back` itself.
            let mut back = mutation.inverse(&snapshot).await;
            back.reverse();
            inverse.extend(back);
            // 🌀️ `mint_mutation_id` is async; `Option::unwrap_or_else`'s closure is sync
            // (R10 shape 1), so it's written as an explicit match instead.
            let mutation_id = match mutation.mutation_id().await {
                Some(id) => id,
                None => MutationId(mint_mutation_id(&encoded).await),
            };
            mutation_meta.push(MutationMeta {
                mutation_id: Some(mutation_id),
                dependencies: mutation.dependencies().await,
                base_version: mutation.base_version().await.map(|version| version.0).unwrap_or(0),
                author_id: Some(mutation.author_id().await.unwrap_or_else(|| ActorId("local".into()))),
                // 🎯️ An authored timestamp is durable as authored; the local clock observes it
                // so its next generated timestamp remains causally later.
                timestamp: match mutation.timestamp().await {
                    Some(timestamp) => {
                        candidate_clock.merge(&timestamp).await;
                        timestamp
                    }
                    None => {
                        candidate_clock.tick(now_ms().await).await;
                        candidate_clock
                    }
                },
                undo_policy: mutation.undo_policy().await,
                // 🎞️ CW3: direct blake3 (same primitive `crate::os_pack::ContentHash` uses) replaces the
                // old `framework_hash::hash_bytes` String hash — `crate::os_spr::PayloadHash` is
                // now `[u8; 32]`, not a hex string. NOT `crate::os_pack::content_hash`, which reads a pack
                // FILE's footer rather than hashing arbitrary bytes. 🎯️ B2: hashes the real
                // `OpBinary` encoding, not a JSON serialization — two ops that encode identically
                // via `encode_op()` but differ in JSON shape (or vice versa) must hash identically.
                payload_hash: Some(crate::os_spr::PayloadHash(*blake3::hash(&encoded).as_bytes())),
                semantic_kind: None,
                label: None,
                group_id: None,
                origin: Default::default(),
            });
            let outcome = mutation.diff(&snapshot).await.stamp_op_index(op_index as u32);
            let (diff, op_messages) = outcome.await.into_parts().await;
            messages.extend(op_messages);
            snapshot = diff.apply(&snapshot).await?;
            forwards.push(mutation);
        }
        if let Some(level) = crate::os_spr::worst_level(&messages).await {
            if self.merge_policy.rejects(level).await {
                return Err(VcsError::Rejected { policy: self.merge_policy, messages });
            }
        }
        self.clock = candidate_clock;
        Ok((forwards, inverse, mutation_meta, snapshot, messages))
    }

    /// @emoji 🕹️ Parses `command_text` via [`parse_command`] and dispatches it — the op-line
    /// textual entry point (op-efficient one-line-per-structural-field commands, indented op
    /// lines for `Apply`/`AmendLast`).
    pub async fn dispatch_text(&mut self, command_text: &str) -> Result<CommandReceipt, VcsError>
    where
        Mutation: OpText,
    {
        let command = parse_command(command_text).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
        self.dispatch(command).await
    }

    /// @emoji 🕹️ Decodes `command_bytes` via [`decode_command`] and dispatches it — the binary
    /// entry point used for both communication (backbone/semio_hub) and storage (`.spr`).
    pub async fn dispatch_binary(&mut self, command_bytes: &[u8]) -> Result<CommandReceipt, VcsError>
    where
        Mutation: OpBinary,
    {
        let command = <ArtifactCommand<Mutation> as OpBinary>::decode_op(command_bytes).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
        self.dispatch(command).await
    }

    /// @emoji 📸️ The whole-document snapshot as real `pack`+`spr` bytes — what `flush_outbound`
    /// sends over `BackboneMessage::Snapshot` and what any other caller needing a full-fidelity
    /// binary snapshot (never JSON) should call.
    pub async fn snapshot_pack(&self) -> Result<ArtifactPackFiles, VcsError> {
        print_document_pack(&self.envelope).await
    }

    pub async fn snapshot_json(&self) -> Result<String, VcsError> {
        let snapshot = self.snapshot().await?;
        serde_json::to_string(&snapshot).map_err(|e| VcsError::Serialize(e.to_string()))
    }

    /// @emoji 📦️ Serializes the full document envelope (snapshot + VCS history) as JSON.
    pub async fn envelope_json(&self) -> Result<String, VcsError> {
        serde_json::to_string(&self.envelope).map_err(|e| VcsError::Serialize(e.to_string()))
    }

    /// @emoji 🔗️ Attaches a backbone channel, reconciling any already-persisted state before
    /// seeding it with this store's current snapshot.
    pub async fn attach_backbone(&mut self, backbone: Backbones) -> Result<(), VcsError> {
        self.envelope.backbone = Some(backbone.descriptor().await);
        self.backbone = Some(backbone);
        self.pump().await?;
        self.flush_outbound(false).await?;
        self.bump();
        Ok(())
    }

    /// @emoji 🔗️ Resolves a backbone URI and attaches it. Only available inside the wasm sandbox,
    /// where every scheme forwards to the host over the injected {@link BackboneChannelPort} (a pure
    /// queue). On native targets, callers attach an explicit `Backbones` value via
    /// {@link attach_backbone} — the `framework/sync` actor layer owns all IO-performing endpoints.
    #[cfg(target_arch = "wasm32")]
    pub async fn attach_backbone_uri(&mut self, uri: &str) -> Result<(), VcsError> {
        self.attach_backbone(resolve_backbone(uri).await?).await
    }

    /// @emoji ✂️ Detaches the backbone; the WIP graph stays in memory, simply unsynchronized.
    pub async fn detach_backbone(&mut self) -> Option<Backbones> {
        self.envelope.backbone = None;
        self.bump();
        self.backbone.take()
    }

    pub async fn backbone_ref(&self) -> Option<&ArtifactBackboneRef> {
        self.envelope.backbone.as_ref()
    }

    /// @emoji 📡️ Drains inbound backbone messages into the edit timeline. Safe to call anytime;
    /// `dispatch` already calls this before every command.
    pub async fn tick(&mut self) -> Result<bool, VcsError> {
        self.pump().await
    }

    /// @emoji 🕸️ Feeds a remote {@link MutationEnvelope} through the causal DAG, applying it (and any
    /// now-unblocked dependents) into the edit timeline. Closes the sync gap between
    /// `framework/sync`'s `MutationDag` and the vcs edit history. Sole public remote write gate —
    /// parallel to `dispatch` for causal envelopes. Implements the ticket's 9-step algorithm
    /// (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6): buffer in the
    /// dag, drain whatever it (and anything it unblocks) makes ready, HLC-sort that batch into
    /// `applied_edit_ids`, replay only the divergent suffix, then either commit atomically (dag,
    /// history, ledger, `applied_edit_ids` all move together) or quarantine the whole batch as an
    /// `Open` `Conflict` — state and the dag's own applied-set never move on rejection.
    pub async fn ingest_remote(&mut self, envelope: crate::os_spr::MutationEnvelope) -> Result<crate::os_spr::MergeReport, VcsError> {
        let no_op_report = |policy: crate::os_spr::MergePolicy, insertion_index: usize| crate::os_spr::MergeReport { policy, accepted: true, insertion_index: insertion_index as u32, replayed: Vec::new(), worst: None, conflict: None };
        // 1
        let mut candidate_dag = self.dag.clone();
        if matches!(candidate_dag.insert(envelope.clone()).map_err(|error| VcsError::Backbone(error.to_string()))?, crate::os_spr::InsertResult::AlreadyApplied) {
            self.assert_equivalent_remote_mutation(&envelope).await?;
            return Ok(no_op_report(self.merge_policy, self.applied_edit_ids.len()));
        }
        // 2
        let ready = candidate_dag.drain_applied_envelopes();
        let mut batch: Vec<Edit<Mutation>> = Vec::new();
        for ready_envelope in &ready {
            let mut edit = edit_from_operation_envelope::<Mutation>(ready_envelope).await?;
            edit.actor = Some(ready_envelope.actor.0.clone());
            if let Some(existing) = self.envelope.vcs.edits.iter().find(|existing| existing.id == edit.id) {
                self.assert_equivalent_remote_envelope(existing, ready_envelope).await?;
                continue;
            }
            if batch.iter().any(|existing: &Edit<Mutation>| existing.id == edit.id) {
                return Err(VcsError::ValidationFailed(format!("remote ingest repeats authoritative edit {}", edit.id)));
            }
            self.clock.merge(&ready_envelope.timestamp).await;
            batch.push(edit);
        }
        if batch.is_empty() {
            self.dag = candidate_dag;
            return Ok(no_op_report(self.merge_policy, self.applied_edit_ids.len()));
        }
        // 3 — an edit's HLC is its first forward op's stamped meta timestamp.
        let edit_hlc = |edit: &Edit<Mutation>| edit.mutation_meta.first().map(|meta| meta.timestamp).unwrap_or_else(|| HybridLogicalTimestamp { actor: 0, physical_ms: 0, logical: 0 });
        // 🌀️ `HybridLogicalTimestamp::cmp_key` is async (📡️replication); `sort_by_key`/
        // `partition_point`/`position` all need sync predicates (R10 shape 1), so every key below
        // is resolved via an explicit `.await` first, then compared as a plain `(u64, u64, u64)`.
        let mut batch_keys = Vec::with_capacity(batch.len());
        for edit in &batch {
            batch_keys.push(edit_hlc(edit).cmp_key().await);
        }
        let mut batch_order: Vec<usize> = (0..batch.len()).collect();
        batch_order.sort_by_key(|&i| batch_keys[i]);
        batch = batch_order.iter().map(|&i| batch[i].clone()).collect();
        batch_keys = batch_order.iter().map(|&i| batch_keys[i]).collect();
        let known_hlc = |edit_id: &str, edits: &[Edit<Mutation>]| edits.iter().find(|edit| edit.id == *edit_id).map(edit_hlc);
        let min_batch_key = batch_keys[0];
        // Binary search replicating `partition_point`'s algorithm (assumes `applied_edit_ids` is
        // already HLC-sorted, exactly as `partition_point` itself would have assumed).
        let mut lo = 0usize;
        let mut hi = self.applied_edit_ids.len();
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let candidate_key = match known_hlc(&self.applied_edit_ids[mid], &self.envelope.vcs.edits) {
                Some(hlc) => Some(hlc.cmp_key().await),
                None => None,
            };
            let still_before = candidate_key.map(|key| key < min_batch_key).unwrap_or(true);
            if still_before {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        let mut k = lo;
        for edit in &batch {
            for dependency in edit.mutation_meta.first().map(|meta| meta.dependencies.clone()).unwrap_or_default() {
                if let Some(position) = self.applied_edit_ids.iter().position(|id| *id == dependency.0) {
                    k = k.max(position + 1);
                }
            }
        }
        // 4 — stable HLC merge of `batch` into `applied_edit_ids[k..]`.
        let mut order: Vec<String> = self.applied_edit_ids.clone();
        for (edit, &hlc_key) in batch.iter().zip(batch_keys.iter()) {
            let mut insert_at = order.len();
            for offset in k..order.len() {
                let existing_key = match known_hlc(&order[offset], &self.envelope.vcs.edits) {
                    Some(existing) => Some(existing.cmp_key().await),
                    None => None,
                };
                if existing_key.map(|key| key > hlc_key).unwrap_or(false) {
                    insert_at = offset;
                    break;
                }
            }
            order.insert(insert_at, edit.id.clone());
        }
        // 5
        let base = if k == self.applied_edit_ids.len() { self.current.clone() } else { Self::fold_history(&self.envelope, &order[..k]).await? };
        // 6 — walk order[k..] one edit at a time (H1 determinism fix): an already-committed edit
        // that this rewind proves invalid gets the SAME accept/quarantine verdict a fresh arrival
        // in true HLC order would have given it, instead of the whole suffix being judged
        // atomically. See `replay_suffix_partitioned`'s doc for why this makes the merge a pure
        // function of the envelope SET and `self.merge_policy`, never of arrival order.
        let mut edits_by_id: HashMap<String, Edit<Mutation>> = self.envelope.vcs.edits.iter().map(|edit| (edit.id.clone(), edit.clone())).collect();
        for edit in &batch {
            edits_by_id.insert(edit.id.clone(), edit.clone());
        }
        let (state, committed_ids, quarantined_ids, rebased_inverse, replayed) = Self::replay_suffix_partitioned(&base, &order, k, &edits_by_id, self.merge_policy).await?;
        // 7
        let worst = replayed.iter().flat_map(|edit_messages| edit_messages.messages.iter()).map(|message| message.level).max();
        let document_id = ArtifactId(self.envelope.id.clone());
        let schema = SchemaId(self.envelope.schema.clone());
        // 🎯️ MEDIUM-3: `degraded_ids` only needs `committed_ids`/`replayed` (both already computed
        // above, before any mutation) — hoisted from below `# 9` so the open-conflict capacity check
        // can see BOTH conflicts this call might mint before touching any store field, keeping the
        // whole call atomic on a refusal exactly like a policy rejection already is.
        // 🌀️ `worst_level` is async; `Iterator::filter`'s closure is sync (R10 shape 1), so it's
        // hoisted into an explicit loop.
        let mut degraded_ids: Vec<String> = Vec::new();
        for id in &committed_ids {
            let is_degraded = match replayed.iter().find(|entry| &entry.edit_id == id) {
                Some(entry) => crate::os_spr::worst_level(&entry.messages).await.map(|level| level >= crate::os_dsl::Severity::Warning).unwrap_or(false),
                None => false,
            };
            if is_degraded {
                degraded_ids.push(id.clone());
            }
        }
        self.ensure_open_conflict_capacity(usize::from(!quarantined_ids.is_empty()) + usize::from(!degraded_ids.is_empty())).await?;
        // 8 — quarantine: every edit whose OWN outcome the policy rejects — whether newly-arrived
        // this call or retroactively invalidated by this rewind — is pulled out of history. Its
        // forward ops/`MutationMeta` are left untouched wherever they already live in `vcs.edits`
        // (never rewritten, exactly like `Undo`); only its id drops out of `applied_edit_ids`, so
        // a future `resolve_conflict`/redelivery still finds its established payload.
        let mut quarantine_conflict_id = None;
        if !quarantined_ids.is_empty() {
            // 🎯️ HIGH-2: `edits_for_ids` errors loudly instead of silently filtering — see its doc.
            let quarantined_edits = Self::edits_for_ids(&quarantined_ids, &edits_by_id).await?;
            let mut envelopes: Vec<crate::os_spr::MutationEnvelope> = Vec::new();
            for edit in &quarantined_edits {
                envelopes.extend(crate::os_spr::mutation_envelope_from_edit::<P, Mutation>(edit, &document_id, &schema).await.map_err(|error| VcsError::Serialize(error.to_string()))?);
            }
            // 🌀️ The unawaited future would borrow `envelopes`, which is moved into `kind` below —
            // awaited immediately instead of deferred to avoid a move-while-borrowed (E0505).
            let conflict_actors = canonical_conflict_actors(envelopes.iter().map(|envelope| envelope.actor.clone())).await;
            // 🌀️ `mutation_ids_for_edit` is async (📡️replication); `flat_map`'s closure is sync
            // (R10 shape 1), so it's hoisted into an explicit loop.
            let mut quarantine_mutation_ids: Vec<MutationId> = Vec::new();
            for edit in &quarantined_edits {
                quarantine_mutation_ids.extend(crate::os_spr::mutation_ids_for_edit(edit).await);
            }
            let quarantine_messages = conflict_messages_for_edits(&quarantined_edits, &replayed).await?;
            let kind = crate::os_spr::ConflictKind::Quarantined { envelopes };
            // 🎯️ H1: `hlc`/`timestamp` come from the quarantined edits' OWN stamped HLCs, never
            // `self.clock` — `self.clock.merge` ticks its `logical` counter on every call, so its
            // running value depends on how many merges happened and in what order, not just on
            // this envelope set; hashing that into `ConflictId` reintroduced arrival-order
            // dependence even after state/`applied_edit_ids` converged.
            // 🌀️ `HybridLogicalTimestamp::cmp_key` is async; `max_by_key` needs a sync key extractor
            // (R10 shape 1), so the max is found by an explicit fold over resolved keys instead.
            let mut conflict_hlc = self.clock;
            let mut conflict_hlc_key: Option<(u64, u64, u64)> = None;
            for edit in &quarantined_edits {
                let hlc = edit_hlc(edit);
                let key = hlc.cmp_key().await;
                // `>=` (not `>`): `Iterator::max_by_key`'s documented tie-break keeps the LAST
                // equally-maximum element, so ties must still overwrite.
                if conflict_hlc_key.map(|current| key >= current).unwrap_or(true) {
                    conflict_hlc_key = Some(key);
                    conflict_hlc = hlc;
                }
            }
            let id = crate::os_spr::ConflictId::new(&kind, &ArtifactId(self.envelope.id.clone()), &quarantine_mutation_ids, &conflict_hlc).await;
            self.envelope.conflicts.push(crate::os_spr::Conflict { id: id.clone(), kind, status: crate::os_spr::ConflictStatus::Open, messages: quarantine_messages, actors: conflict_actors, timestamp: conflict_hlc });
            self.prune_resolved_conflicts();
            for quarantined_id in &quarantined_ids {
                self.replace_edit_messages(quarantined_id, Vec::new());
            }
            quarantine_conflict_id = Some(id);
        }
        // 9 — commit the survivors: history, ledger, `applied_edit_ids`, `current` together. The
        // dag only advances when nothing in this suffix was quarantined — matching how a reject
        // never touched it before this fix — so a quarantined-but-not-yet-committed envelope can
        // still be retried on redelivery; an already-committed edit's `vcs.edits` dedup on step 2
        // makes that safe even for a `batch` edit that *did* commit under a since-discarded dag.
        if quarantined_ids.is_empty() {
            self.dag = candidate_dag;
        }
        for edit in &batch {
            if committed_ids.contains(&edit.id) && !self.envelope.vcs.edits.iter().any(|existing| existing.id == edit.id) {
                self.envelope.vcs.edits.push(edit.clone());
            }
        }
        for (edit_id, inverse) in rebased_inverse {
            if let Some(edit) = self.envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id) {
                edit.inverse = inverse;
            }
        }
        self.applied_edit_ids = self.applied_edit_ids[..k].iter().cloned().chain(committed_ids.iter().cloned()).collect();
        for edit_messages in &replayed {
            if committed_ids.contains(&edit_messages.edit_id) {
                self.replace_edit_messages(&edit_messages.edit_id, edit_messages.messages.clone());
            }
        }
        for (index, edit_id) in self.applied_edit_ids.iter().enumerate() {
            if let Some(edit) = self.envelope.vcs.edits.iter_mut().find(|edit| edit.id == *edit_id) {
                edit.sequence_number = index as i32 + 1;
            }
        }
        self.edit_sequence = self.applied_edit_ids.len() as i32;
        self.tail_undo_cache = None;
        self.current = state;
        let mut degraded_conflict_id = None;
        if !degraded_ids.is_empty() {
            // 🎯️ HIGH-2: `edits_for_ids` errors loudly instead of silently filtering — see its doc.
            // Reads from `edits_by_id` (not `self.envelope.vcs.edits`): both already contain every
            // `degraded_ids` entry at this point, but `edits_by_id` doesn't depend on the `vcs.edits`
            // push above having already run, so this block's own correctness never depends on
            // sequencing against that mutation.
            let degraded_edits = Self::edits_for_ids(&degraded_ids, &edits_by_id).await?;
            let degraded_messages = conflict_messages_for_edits(&degraded_edits, &replayed).await?;
            let degraded_actors = canonical_conflict_actors(degraded_edits.iter().filter_map(|edit| edit.actor.clone()).map(ActorId));
            // 🌀️ `mutation_ids_for_edit` is async (📡️replication); `flat_map`'s closure is sync
            // (R10 shape 1), so it's hoisted into an explicit loop.
            let mut degraded_mutation_ids: Vec<MutationId> = Vec::new();
            for edit in &degraded_edits {
                degraded_mutation_ids.extend(crate::os_spr::mutation_ids_for_edit(edit).await);
            }
            let kind = crate::os_spr::ConflictKind::Degraded { edit_ids: degraded_ids.clone() };
            // 🎯️ H1: same reasoning as the quarantine conflict above — derive from the degraded
            // edits' own HLCs, not the arrival-order-dependent `self.clock`.
            // 🌀️ `HybridLogicalTimestamp::cmp_key` is async; `max_by_key` needs a sync key extractor
            // (R10 shape 1), so the max is found by an explicit fold over resolved keys instead.
            let mut conflict_hlc = self.clock;
            let mut conflict_hlc_key: Option<(u64, u64, u64)> = None;
            for edit in &degraded_edits {
                let hlc = edit_hlc(edit);
                let key = hlc.cmp_key().await;
                // `>=`: `Iterator::max_by_key` keeps the LAST equally-maximum element on a tie.
                if conflict_hlc_key.map(|current| key >= current).unwrap_or(true) {
                    conflict_hlc_key = Some(key);
                    conflict_hlc = hlc;
                }
            }
            let id = crate::os_spr::ConflictId::new(&kind, &ArtifactId(self.envelope.id.clone()), &degraded_mutation_ids, &conflict_hlc).await;
            self.envelope.conflicts.push(crate::os_spr::Conflict { id: id.clone(), kind, status: crate::os_spr::ConflictStatus::Open, messages: degraded_messages, actors: degraded_actors.await, timestamp: conflict_hlc });
            self.prune_resolved_conflicts();
            degraded_conflict_id = Some(id);
        }
        self.pending_report.edit_ids = Some(batch.iter().filter(|edit| committed_ids.contains(&edit.id)).map(|edit| edit.id.clone()).collect());
        self.bump();
        self.last_projection_cause = Some(ArtifactProjectionCause::RemoteIngest);
        let accepted = batch.iter().all(|edit| committed_ids.contains(&edit.id));
        Ok(crate::os_spr::MergeReport { policy: self.merge_policy, accepted, insertion_index: k as u32, replayed, worst, conflict: quarantine_conflict_id.or(degraded_conflict_id) })
    }

    async fn assert_equivalent_remote_envelope(&self, existing: &Edit<Mutation>, incoming: &crate::os_spr::MutationEnvelope) -> Result<(), VcsError> {
        let document_id = ArtifactId(self.envelope.id.clone());
        let schema = SchemaId(self.envelope.schema.clone());
        let established = crate::os_spr::mutation_envelope_from_edit::<P, Mutation>(existing, &document_id, &schema).await.map_err(|error| VcsError::Serialize(error.to_string()))?;
        if established.len() == 1 && established.first().is_some_and(|candidate| Self::same_operation_identity_and_payload(candidate, incoming)) {
            return Ok(());
        }
        Err(VcsError::ValidationFailed(format!("remote mutation id {} conflicts with its established payload", incoming.mutation_id.0)))
    }

    async fn assert_equivalent_remote_mutation(&self, incoming: &crate::os_spr::MutationEnvelope) -> Result<(), VcsError> {
        let document_id = ArtifactId(self.envelope.id.clone());
        let schema = SchemaId(self.envelope.schema.clone());
        for edit in &self.envelope.vcs.edits {
            for established in crate::os_spr::mutation_envelope_from_edit::<P, Mutation>(edit, &document_id, &schema).await.map_err(|error| VcsError::Serialize(error.to_string()))? {
                if established.mutation_id == incoming.mutation_id {
                    return if Self::same_operation_identity_and_payload(&established, incoming) { Ok(()) } else { Err(VcsError::ValidationFailed(format!("remote mutation id {} conflicts with its established payload", incoming.mutation_id.0))) };
                }
            }
        }
        Err(VcsError::ValidationFailed(format!("remote mutation id {} was marked applied without an established payload", incoming.mutation_id.0)))
    }

    // 🚫️async: E1 pure field comparison, consumed only through `is_some_and`/`Iterator::all`
    // sync closures — see R9.
    fn same_operation_identity_and_payload(left: &crate::os_spr::MutationEnvelope, right: &crate::os_spr::MutationEnvelope) -> bool {
        left.mutation_id == right.mutation_id && left.document_id == right.document_id && left.diff.schema == right.diff.schema && left.diff.payload == right.diff.payload
    }

    async fn same_edit_operation_identities_and_payloads(&self, left: &Edit<Mutation>, right: &Edit<Mutation>) -> Result<bool, VcsError> {
        let document_id = ArtifactId(self.envelope.id.clone());
        let schema = SchemaId(self.envelope.schema.clone());
        let left = crate::os_spr::mutation_envelope_from_edit::<P, Mutation>(left, &document_id, &schema).await.map_err(|error| VcsError::Serialize(error.to_string()))?;
        let right = crate::os_spr::mutation_envelope_from_edit::<P, Mutation>(right, &document_id, &schema).await.map_err(|error| VcsError::Serialize(error.to_string()))?;
        Ok(left.len() == right.len() && left.iter().zip(&right).all(|(left, right)| Self::same_operation_identity_and_payload(left, right)))
    }

    async fn resolution_candidate(&self) -> Self {
        Self {
            envelope: self.envelope.clone(),
            backbone: None,
            dag: self.dag.clone(),
            applied_edit_ids: self.applied_edit_ids.clone(),
            redo_edit_ids: self.redo_edit_ids.clone(),
            edit_sequence: self.edit_sequence,
            generation: self.generation,
            last_projection_cause: self.last_projection_cause,
            current_checkpoint_id: self.current_checkpoint_id.clone(),
            local_actor_id: self.local_actor_id.clone(),
            merge_policy: crate::os_spr::MergePolicy::LaissezFaire,
            edit_messages: self.edit_messages.clone(),
            clock: self.clock,
            current: self.current.clone(),
            tail_undo_cache: self.tail_undo_cache.clone(),
            pending_report: PendingCommandReport::default(),
        }
    }

    async fn adopt_resolution_candidate(&mut self, candidate: Self) {
        self.envelope = candidate.envelope;
        self.dag = candidate.dag;
        self.applied_edit_ids = candidate.applied_edit_ids;
        self.redo_edit_ids = candidate.redo_edit_ids;
        self.edit_sequence = candidate.edit_sequence;
        self.last_projection_cause = candidate.last_projection_cause;
        self.current_checkpoint_id = candidate.current_checkpoint_id;
        self.local_actor_id = candidate.local_actor_id;
        self.edit_messages = candidate.edit_messages;
        self.clock = candidate.clock;
        self.current = candidate.current;
        self.tail_undo_cache = candidate.tail_undo_cache;
    }

    async fn aggregate_resolution_reports(reports: impl IntoIterator<Item = crate::os_spr::MergeReport>, conflict: crate::os_spr::ConflictId) -> crate::os_spr::MergeReport {
        let reports: Vec<crate::os_spr::MergeReport> = reports.into_iter().collect();
        let insertion_index = reports.first().map(|report| report.insertion_index).unwrap_or(0);
        let mut aggregate = crate::os_spr::MergeReport { policy: crate::os_spr::MergePolicy::LaissezFaire, accepted: true, insertion_index, replayed: Vec::new(), worst: None, conflict: Some(conflict) };
        for report in reports {
            aggregate.accepted &= report.accepted;
            aggregate.worst = aggregate.worst.max(report.worst);
            aggregate.replayed.extend(report.replayed);
        }
        aggregate
    }

    /// @emoji ⚖️ Resolves an `Open` `crate::os_spr::Conflict` by id (`26/08/16/MUTATION-OUTCOMES-
    /// MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6). `Quarantined`+`Accept` reruns
    /// `ingest_remote` for the conflict's own quarantined envelopes under `LaissezFaire` (a `Fatal`
    /// message still rejects — `LaissezFaire` never waives that), landing the same state
    /// `ingest_remote` would have produced under `LaissezFaire` in the first place, and marks the
    /// conflict `Accepted` WITHOUT raising a second one (any `Degraded` conflict the re-ingest would
    /// otherwise have raised is discarded). `Quarantined`+`Discard` seeds the quarantined ids into
    /// the causal dag as already-seen — never applied, never relayed — and marks the conflict
    /// `Discarded`. `Degraded`+`Accept` only flips the status: the batch is already durable history,
    /// resolving never rewrites it. `Degraded`+`Discard` is refused outright — shared history is
    /// never rewritten.
    pub async fn resolve_conflict(&mut self, conflict_id: &str, resolution: crate::os_spr::ConflictResolution) -> Result<crate::os_spr::MergeReport, VcsError> {
        let index = self.envelope.conflicts.iter().position(|conflict| conflict.id.0 == conflict_id && conflict.status == crate::os_spr::ConflictStatus::Open).ok_or_else(|| VcsError::UnknownConflict(conflict_id.to_string()))?;
        let conflict = self.envelope.conflicts[index].clone();
        match (conflict.kind.clone(), resolution) {
            (crate::os_spr::ConflictKind::Quarantined { envelopes, .. }, crate::os_spr::ConflictResolution::Accept) => {
                // 🌀️ A future is consumed by one `.await` (R10 shape 2) — `candidate` is used many
                // times below (including across loop iterations), so it's resolved once here.
                let mut candidate = self.resolution_candidate().await;
                let pre_conflicts_len = candidate.envelope.conflicts.len();
                let mut reports = Vec::with_capacity(envelopes.len());
                for envelope in envelopes {
                    let report = candidate.ingest_remote(envelope).await?;
                    let accepted = report.accepted;
                    reports.push(report);
                    if !accepted {
                        self.pending_report.edit_ids = Some(Vec::new());
                        return Ok(Self::aggregate_resolution_reports(reports, conflict.id.clone()).await);
                    }
                }
                candidate.envelope.conflicts.truncate(pre_conflicts_len);
                candidate.envelope.conflicts[index].status = crate::os_spr::ConflictStatus::Accepted;
                validate_durable_history(&candidate.envelope).await?;
                let prior_ids: HashSet<&str> = self.applied_edit_ids.iter().map(String::as_str).collect();
                let accepted_ids: Vec<String> = candidate.applied_edit_ids.iter().filter(|edit_id| !prior_ids.contains(edit_id.as_str())).cloned().collect();
                self.adopt_resolution_candidate(candidate).await;
                self.pending_report.edit_ids = Some(accepted_ids);
                self.bump();
                self.last_projection_cause = Some(ArtifactProjectionCause::RemoteIngest);
                // 🎯️ MEDIUM-3: this conflict just turned `Accepted` — a resolved conflict is
                // prunable, so give the cap a chance to reclaim it without waiting for the next
                // `ingest_remote` push.
                self.prune_resolved_conflicts();
                Ok(Self::aggregate_resolution_reports(reports, conflict.id.clone()).await)
            }
            (crate::os_spr::ConflictKind::Quarantined { envelopes, .. }, crate::os_spr::ConflictResolution::Discard) => {
                for envelope in &envelopes {
                    self.dag.seed_applied(envelope.mutation_id.clone());
                }
                self.envelope.conflicts[index].status = crate::os_spr::ConflictStatus::Discarded;
                self.pending_report.edit_ids = Some(Vec::new());
                self.bump();
                self.last_projection_cause = Some(ArtifactProjectionCause::RemoteIngest);
                self.prune_resolved_conflicts();
                Ok(crate::os_spr::MergeReport { policy: self.merge_policy, accepted: false, insertion_index: 0, replayed: Vec::new(), worst: None, conflict: Some(conflict.id.clone()) })
            }
            (crate::os_spr::ConflictKind::Degraded { .. }, crate::os_spr::ConflictResolution::Accept) => {
                self.envelope.conflicts[index].status = crate::os_spr::ConflictStatus::Accepted;
                self.pending_report.edit_ids = Some(Vec::new());
                self.bump();
                self.last_projection_cause = Some(ArtifactProjectionCause::RemoteIngest);
                self.prune_resolved_conflicts();
                Ok(crate::os_spr::MergeReport { policy: self.merge_policy, accepted: true, insertion_index: 0, replayed: Vec::new(), worst: None, conflict: Some(conflict.id.clone()) })
            }
            (crate::os_spr::ConflictKind::Degraded { .. }, crate::os_spr::ConflictResolution::Discard) => Err(VcsError::ValidationFailed("a Degraded conflict's batch is already durable history and can never be discarded".to_string())),
        }
    }

    /// 🪪️ Resolves each source operation to the one wire-derived local edit that already owns it.
    /// An Operations delivery splits a multi-forward source edit into one local edit per wire
    /// operation, so the durable source ledger is partitioned by source `op_index` and rekeyed to
    /// those local edits before snapshot preflight.
    async fn snapshot_ledger_targets(&self, source: &Edit<Mutation>) -> Result<Option<Vec<(String, u32)>>, VcsError> {
        let mut full_match = None;
        for local in &self.envelope.vcs.edits {
            if self.same_edit_operation_identities_and_payloads(local, source).await? {
                if full_match.replace(local).is_some() {
                    return Err(VcsError::ValidationFailed(format!("snapshot edit {} has ambiguous complete local ownership", source.id)));
                }
            }
        }
        if let Some(local) = full_match {
            return Ok(Some((0..source.forwards.len() as u32).map(|index| (local.id.clone(), index)).collect()));
        }
        let document_id = ArtifactId(self.envelope.id.clone());
        let schema = SchemaId(self.envelope.schema.clone());
        let source_envelopes = crate::os_spr::mutation_envelope_from_edit::<P, Mutation>(source, &document_id, &schema).await.map_err(|error| VcsError::Serialize(error.to_string()))?;
        let mut targets = Vec::with_capacity(source_envelopes.len());
        for source_envelope in &source_envelopes {
            let mut candidates = Vec::new();
            for local in &self.envelope.vcs.edits {
                let local_envelopes = crate::os_spr::mutation_envelope_from_edit::<P, Mutation>(local, &document_id, &schema).await.map_err(|error| VcsError::Serialize(error.to_string()))?;
                if local_envelopes.len() != 1 {
                    continue;
                }
                if Self::same_operation_identity_and_payload(&local_envelopes[0], source_envelope) {
                    candidates.push((local.id.clone(), 0));
                }
            }
            match candidates.len() {
                0 => return Ok(None),
                1 => targets.push(candidates.pop().ok_or_else(|| VcsError::ValidationFailed("snapshot operation candidate disappeared".to_string()))?),
                _ => return Err(VcsError::ValidationFailed(format!("snapshot operation {} has ambiguous established edit ownership", source_envelope.mutation_id.0))),
            }
        }
        if targets.len() != source.forwards.len() {
            return Err(VcsError::ValidationFailed(format!("snapshot edit {} has {} stable operations for {} forwards", source.id, targets.len(), source.forwards.len())));
        }
        Ok(Some(targets))
    }

    async fn remap_snapshot_message_ledger(&self, remote: &mut ArtifactEnvelope<P, Mutation>) -> Result<(), VcsError> {
        let remote_edits = remote.vcs.edits.clone();
        let mut remapped = Vec::new();
        let mut remapped_ids = HashSet::new();
        for entry in std::mem::take(&mut remote.edit_messages) {
            let remote_edit = remote_edits.iter().find(|edit| edit.id == entry.edit_id).ok_or_else(|| VcsError::ValidationFailed(format!("remote message ledger references unknown edit {}", entry.edit_id)))?;
            let Some(targets) = self.snapshot_ledger_targets(remote_edit).await? else {
                if !remapped_ids.insert(entry.edit_id.clone()) {
                    return Err(VcsError::ValidationFailed(format!("remote message ledger repeats edit {}", entry.edit_id)));
                }
                remapped.push(entry);
                continue;
            };
            let mut by_edit: HashMap<String, Vec<crate::os_spr::MutationMessage>> = HashMap::new();
            let mut edit_order = Vec::new();
            for message in entry.messages {
                validate_persisted_message(&message, Some(remote_edit.forwards.len())).await?;
                let source_index = message.op_index.ok_or_else(|| VcsError::ValidationFailed(format!("remote message {} has no source operation index", message.code.0)))? as usize;
                let (edit_id, operation_index) = targets.get(source_index).ok_or_else(|| VcsError::ValidationFailed(format!("remote message {} references unknown source operation {source_index}", message.code.0)))?;
                if !by_edit.contains_key(edit_id) {
                    edit_order.push(edit_id.clone());
                }
                let mut message = message;
                message.op_index = Some(*operation_index);
                by_edit.entry(edit_id.clone()).or_default().push(message);
            }
            for edit_id in edit_order {
                if !remapped_ids.insert(edit_id.clone()) {
                    return Err(VcsError::ValidationFailed(format!("remote message ledger resolves multiple source edits to {edit_id}")));
                }
                let messages = by_edit.remove(&edit_id).ok_or_else(|| VcsError::ValidationFailed(format!("remote message ledger lost remapped edit {edit_id}")))?;
                remapped.push(crate::os_spr::EditMessages { edit_id, messages });
            }
        }
        remote.edit_messages = remapped;
        Ok(())
    }

    async fn merge_remote_snapshot(&mut self, pack: &[u8], spr: &[u8]) -> Result<(), VcsError> {
        let mut remote: ArtifactEnvelope<P, Mutation> = parse_document_pack(pack, spr).await.map_err(|error| VcsError::Deserialize(error.to_string()))?.envelope;
        validate_durable_history(&remote).await?;
        if remote.id != self.envelope.id || remote.schema != self.envelope.schema {
            return Err(VcsError::ValidationFailed(format!("remote snapshot identity {}:{} does not match local {}:{}", remote.id, remote.schema, self.envelope.id, self.envelope.schema)));
        }
        self.remap_snapshot_message_ledger(&mut remote).await?;
        let mut remote_clock = self.clock;
        for edit in &remote.vcs.edits {
            for meta in &edit.mutation_meta {
                remote_clock.merge(&meta.timestamp).await;
            }
        }
        if self.envelope.vcs.edits.is_empty() {
            let edit_hlc = |edit: &Edit<Mutation>| edit.mutation_meta.first().map(|meta| meta.timestamp).unwrap_or_else(|| HybridLogicalTimestamp { actor: 0, physical_ms: 0, logical: 0 });
            let mut applied = remote.cursor.as_ref().map(|cursor| cursor.applied_edit_ids.clone()).unwrap_or_else(|| remote.vcs.edits.iter().map(|edit| edit.id.clone()).collect());
            // 🌀️ `HybridLogicalTimestamp::cmp_key` is async (📡️replication); `sort_by_key`'s
            // closure is sync (R10 shape 1), so keys are precomputed into a parallel Vec first.
            let mut applied_keys = Vec::with_capacity(applied.len());
            for edit_id in &applied {
                let hlc = remote.vcs.edits.iter().find(|edit| edit.id == *edit_id).map(edit_hlc).unwrap_or_else(|| HybridLogicalTimestamp { actor: 0, physical_ms: 0, logical: 0 });
                applied_keys.push(hlc.cmp_key().await);
            }
            let mut applied_order: Vec<usize> = (0..applied.len()).collect();
            applied_order.sort_by_key(|&i| applied_keys[i]);
            applied = applied_order.iter().map(|&i| applied[i].clone()).collect();
            let redo_edit_ids = remote.cursor.as_ref().map(|cursor| cursor.redo_edit_ids.clone()).unwrap_or_default();
            let edits_by_id: HashMap<String, Edit<Mutation>> = remote.vcs.edits.iter().map(|edit| (edit.id.clone(), edit.clone())).collect();
            let (current, _, replayed) = Self::replay_suffix(&remote.vcs.initial_snapshot, &applied, 0, &edits_by_id).await?;
            let worst = replayed.iter().flat_map(|entry| entry.messages.iter()).map(|message| message.level).max();
            let rejects = match worst {
                Some(level) => self.merge_policy.rejects(level).await,
                None => false,
            };
            if rejects {
                // 🎯️ MEDIUM-3: checked before `self.clock`/`self.envelope.conflicts` are touched
                // below, so a refusal here is still atomic.
                self.ensure_open_conflict_capacity(1).await?;
                let document_id = ArtifactId(self.envelope.id.clone());
                let schema = SchemaId(self.envelope.schema.clone());
                let mut envelopes: Vec<crate::os_spr::MutationEnvelope> = Vec::new();
                for edit in &remote.vcs.edits {
                    envelopes.extend(crate::os_spr::mutation_envelope_from_edit::<P, Mutation>(edit, &document_id, &schema).await.map_err(|error| VcsError::Serialize(error.to_string()))?);
                }
                // 🌀️ `mutation_ids_for_edit` is async (📡️replication); `flat_map`'s closure is sync
                // (R10 shape 1), so it's hoisted into an explicit loop.
                let mut mutation_ids: Vec<MutationId> = Vec::new();
                for edit in &remote.vcs.edits {
                    mutation_ids.extend(crate::os_spr::mutation_ids_for_edit(edit).await);
                }
                let messages = conflict_messages_for_edits(&remote.vcs.edits, &replayed).await?;
                // 🌀️ The unawaited future would borrow `envelopes`, which is moved into `kind`
                // below — awaited immediately to avoid a move-while-borrowed (E0505).
                let conflict_actors = canonical_conflict_actors(envelopes.iter().map(|envelope| envelope.actor.clone())).await;
                let kind = crate::os_spr::ConflictKind::Quarantined { envelopes };
                let conflict_id = crate::os_spr::ConflictId::new(&kind, &document_id, &mutation_ids, &remote_clock);
                self.clock = remote_clock;
                self.envelope.conflicts.push(crate::os_spr::Conflict { id: conflict_id.await, kind, status: crate::os_spr::ConflictStatus::Open, messages: messages.clone(), actors: conflict_actors, timestamp: self.clock });
                self.prune_resolved_conflicts();
                self.bump();
                self.last_projection_cause = Some(ArtifactProjectionCause::RemoteIngest);
                return Err(VcsError::Rejected { policy: self.merge_policy, messages });
            }
            let mut candidate_envelope = remote;
            candidate_envelope.backbone = self.envelope.backbone.clone();
            let (candidate_dag, edit_sequence, _) = Self::seed_runtime_state(&candidate_envelope).await;
            let current_checkpoint_id = candidate_envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.clone()).or_else(|| candidate_envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone()));
            self.envelope = candidate_envelope;
            self.dag = candidate_dag;
            self.edit_sequence = edit_sequence;
            self.applied_edit_ids = applied;
            self.redo_edit_ids = redo_edit_ids;
            self.edit_messages = self.envelope.edit_messages.iter().map(|entry| (entry.edit_id.clone(), entry.messages.clone())).collect();
            self.clock = remote_clock;
            self.tail_undo_cache = None;
            self.current = current;
            self.current_checkpoint_id = current_checkpoint_id;
            self.bump();
            self.last_projection_cause = Some(ArtifactProjectionCause::RemoteIngest);
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
            known_ids.extend(crate::os_spr::mutation_ids_for_edit(edit).await.into_iter().map(|id| id.0));
        }
        for remote_edit in &remote.vcs.edits {
            if let Some(local_edit) = self.envelope.vcs.edits.iter().find(|local_edit| local_edit.id == remote_edit.id) {
                if !self.same_edit_operation_identities_and_payloads(local_edit, remote_edit).await? {
                    return Err(VcsError::ValidationFailed(format!("remote history conflicts with established edit {}", remote_edit.id)));
                }
            }
        }
        preflight_merge_by_id(&self.envelope.vcs.changes, &remote.vcs.changes, |change| &change.id).await?;
        preflight_merge_by_id(&self.envelope.vcs.checkpoints, &remote.vcs.checkpoints, |checkpoint| &checkpoint.id).await?;
        preflight_merge_by_id(&self.envelope.vcs.alternatives, &remote.vcs.alternatives, |alternative| &alternative.id).await?;
        preflight_merge_by_id(&self.envelope.edit_messages, &remote.edit_messages, |entry| &entry.edit_id).await?;
        preflight_merge_by_id(&self.envelope.conflicts, &remote.conflicts, |conflict| &conflict.id.0).await?;
        // 🎯️ §C6 item 11: collect the genuinely-new remote edits first (same dedup-by-id-or-op-ids
        // as before), then HLC-sort them into `applied_edit_ids` and replay only the divergent
        // suffix via the shared `replay_suffix` (steps 5–9), exactly like `ingest_remote`.
        let mut candidate_ids = known_ids.clone();
        let mut batch: Vec<Edit<Mutation>> = Vec::new();
        for edit in &remote.vcs.edits {
            // 🌀️ A future is consumed by one `.await` (R10 shape 2) — `operation_ids` is used up to
            // three times below (twice inside the `||`, once after), so it's resolved once here.
            let operation_ids = crate::os_spr::mutation_ids_for_edit(edit).await;
            let already_known = candidate_ids.contains(&edit.id) || (!operation_ids.is_empty() && operation_ids.iter().all(|id| candidate_ids.contains(&id.0)));
            if already_known {
                if !self.envelope.vcs.edits.iter().any(|local_edit| local_edit.id == edit.id) {
                    self.assert_equivalent_remote_edit(edit).await?;
                }
                continue;
            }
            candidate_ids.insert(edit.id.clone());
            candidate_ids.extend(operation_ids.into_iter().map(|id| id.0));
            batch.push(edit.clone());
        }
        if batch.is_empty() {
            let mut candidate_envelope = self.envelope.clone();
            merge_by_id(&mut candidate_envelope.vcs.changes, remote.vcs.changes.clone(), |change| &change.id).await?;
            merge_by_id(&mut candidate_envelope.vcs.checkpoints, remote.vcs.checkpoints.clone(), |checkpoint| &checkpoint.id).await?;
            merge_by_id(&mut candidate_envelope.vcs.alternatives, remote.vcs.alternatives.clone(), |alternative| &alternative.id).await?;
            merge_by_id(&mut candidate_envelope.edit_messages, remote.edit_messages.clone(), |entry| &entry.edit_id).await?;
            merge_by_id(&mut candidate_envelope.conflicts, remote.conflicts.clone(), |conflict| &conflict.id.0).await?;
            validate_durable_history(&candidate_envelope).await?;
            self.envelope = candidate_envelope;
            self.edit_messages = self.envelope.edit_messages.iter().map(|entry| (entry.edit_id.clone(), entry.messages.clone())).collect();
            self.clock = remote_clock;
            self.bump();
            self.last_projection_cause = Some(ArtifactProjectionCause::RemoteIngest);
            return Ok(());
        }
        let edit_hlc = |edit: &Edit<Mutation>| edit.mutation_meta.first().map(|meta| meta.timestamp).unwrap_or_else(|| HybridLogicalTimestamp { actor: 0, physical_ms: 0, logical: 0 });
        // 🌀️ `HybridLogicalTimestamp::cmp_key` is async (📡️replication); `sort_by_key`/
        // `partition_point`/`position` all need sync predicates (R10 shape 1), so every key below
        // is resolved via an explicit `.await` first, then compared as a plain `(u64, u64, u64)`.
        let mut batch_keys = Vec::with_capacity(batch.len());
        for edit in &batch {
            batch_keys.push(edit_hlc(edit).cmp_key().await);
        }
        let mut batch_order: Vec<usize> = (0..batch.len()).collect();
        batch_order.sort_by_key(|&i| batch_keys[i]);
        batch = batch_order.iter().map(|&i| batch[i].clone()).collect();
        batch_keys = batch_order.iter().map(|&i| batch_keys[i]).collect();
        let known_hlc = |edit_id: &str, edits: &[Edit<Mutation>]| edits.iter().find(|edit| edit.id == *edit_id).map(edit_hlc);
        let min_batch_key = batch_keys[0];
        // Binary search replicating `partition_point`'s algorithm (assumes `applied_edit_ids` is
        // already HLC-sorted, exactly as `partition_point` itself would have assumed).
        let mut lo = 0usize;
        let mut hi = self.applied_edit_ids.len();
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let candidate_key = match known_hlc(&self.applied_edit_ids[mid], &self.envelope.vcs.edits) {
                Some(hlc) => Some(hlc.cmp_key().await),
                None => None,
            };
            let still_before = candidate_key.map(|key| key < min_batch_key).unwrap_or(true);
            if still_before {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        let k = lo;
        let mut order: Vec<String> = self.applied_edit_ids.clone();
        for (edit, &hlc_key) in batch.iter().zip(batch_keys.iter()) {
            let mut insert_at = order.len();
            for offset in k..order.len() {
                let existing_key = match known_hlc(&order[offset], &self.envelope.vcs.edits) {
                    Some(existing) => Some(existing.cmp_key().await),
                    None => None,
                };
                if existing_key.map(|key| key > hlc_key).unwrap_or(false) {
                    insert_at = offset;
                    break;
                }
            }
            order.insert(insert_at, edit.id.clone());
        }
        let base = if k == self.applied_edit_ids.len() { self.current.clone() } else { Self::fold_history(&self.envelope, &order[..k]).await? };
        let mut edits_by_id: HashMap<String, Edit<Mutation>> = self.envelope.vcs.edits.iter().map(|edit| (edit.id.clone(), edit.clone())).collect();
        for edit in &batch {
            edits_by_id.insert(edit.id.clone(), edit.clone());
        }
        let (state, rebased_inverse, replayed) = Self::replay_suffix(&base, &order, k, &edits_by_id).await?;
        let worst = replayed.iter().flat_map(|edit_messages| edit_messages.messages.iter()).map(|message| message.level).max();
        let document_id = ArtifactId(self.envelope.id.clone());
        let schema = SchemaId(self.envelope.schema.clone());
        // 🌀️ `mutation_ids_for_edit` is async (📡️replication); `flat_map`'s closure is sync
        // (R10 shape 1), so it's hoisted into an explicit loop.
        let mut mutation_ids: Vec<MutationId> = Vec::new();
        for edit in &batch {
            mutation_ids.extend(crate::os_spr::mutation_ids_for_edit(edit).await);
        }
        let actors = canonical_conflict_actors(batch.iter().filter_map(|edit| edit.actor.clone()).map(ActorId));
        let messages = conflict_messages_for_edits(&batch, &replayed).await?;
        // 🎯️ MEDIUM-3: this call mints AT MOST one conflict — either the reject-quarantine below or
        // the accepted-but-degraded one near the end of this function, never both (mutually
        // exclusive on `worst`) — checked here, before either branch touches `self`, so a refusal is
        // atomic for both.
        let rejects = match worst {
            Some(level) => self.merge_policy.rejects(level).await,
            None => false,
        };
        let would_mint_conflict = match worst {
            Some(level) => rejects || level >= crate::os_dsl::Severity::Warning,
            None => false,
        };
        self.ensure_open_conflict_capacity(usize::from(would_mint_conflict)).await?;
        // 🎯️ Reject ⇒ one snapshot-conflict, envelopes rebuilt via `mutation_envelope_from_edit` —
        // nothing about `self` has been touched above, so state is unchanged by construction.
        if rejects {
            let mut envelopes: Vec<crate::os_spr::MutationEnvelope> = Vec::new();
            for edit in &batch {
                envelopes.extend(crate::os_spr::mutation_envelope_from_edit::<P, Mutation>(edit, &document_id, &schema).await.map_err(|error| VcsError::Serialize(error.to_string()))?);
            }
            // 🌀️ The unawaited future would borrow `envelopes`, which is moved into `kind`
            // below — awaited immediately to avoid a move-while-borrowed (E0505).
            let conflict_actors = canonical_conflict_actors(envelopes.iter().map(|envelope| envelope.actor.clone())).await;
            let kind = crate::os_spr::ConflictKind::Quarantined { envelopes };
            let conflict_id = crate::os_spr::ConflictId::new(&kind, &document_id, &mutation_ids, &remote_clock);
            self.clock = remote_clock;
            self.envelope.conflicts.push(crate::os_spr::Conflict { id: conflict_id.await, kind, status: crate::os_spr::ConflictStatus::Open, messages: messages.clone(), actors: conflict_actors, timestamp: self.clock });
            self.prune_resolved_conflicts();
            self.bump();
            self.last_projection_cause = Some(ArtifactProjectionCause::RemoteIngest);
            return Err(VcsError::Rejected { policy: self.merge_policy, messages });
        }
        let mut candidate_envelope = self.envelope.clone();
        let mut candidate_dag = self.dag.clone();
        let mut candidate_edit_sequence = self.edit_sequence;
        for edit in &batch {
            candidate_edit_sequence = candidate_edit_sequence.max(edit.sequence_number);
            for mutation_id in crate::os_spr::mutation_ids_for_edit(edit).await {
                candidate_dag.seed_applied(mutation_id);
            }
            candidate_dag.seed_applied(MutationId(edit.id.clone()));
            candidate_envelope.vcs.edits.push(edit.clone());
        }
        for (edit_id, inverse) in rebased_inverse {
            if let Some(edit) = candidate_envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id) {
                edit.inverse = inverse;
            }
        }
        merge_by_id(&mut candidate_envelope.vcs.changes, remote.vcs.changes.clone(), |change| &change.id).await?;
        merge_by_id(&mut candidate_envelope.vcs.checkpoints, remote.vcs.checkpoints.clone(), |checkpoint| &checkpoint.id).await?;
        merge_by_id(&mut candidate_envelope.vcs.alternatives, remote.vcs.alternatives.clone(), |alternative| &alternative.id).await?;
        merge_by_id(&mut candidate_envelope.edit_messages, remote.edit_messages.clone(), |entry| &entry.edit_id).await?;
        merge_by_id(&mut candidate_envelope.conflicts, remote.conflicts.clone(), |conflict| &conflict.id.0).await?;
        validate_durable_history(&candidate_envelope).await?;
        self.envelope = candidate_envelope;
        self.edit_messages = self.envelope.edit_messages.iter().map(|entry| (entry.edit_id.clone(), entry.messages.clone())).collect();
        self.dag = candidate_dag;
        self.applied_edit_ids = order;
        self.edit_sequence = candidate_edit_sequence;
        self.clock = remote_clock;
        for edit_messages in &replayed {
            if !self.envelope.edit_messages.iter().any(|entry| entry.edit_id == edit_messages.edit_id) {
                self.replace_edit_messages(&edit_messages.edit_id, edit_messages.messages.clone());
            }
        }
        self.tail_undo_cache = None;
        self.current = state;
        if worst.map(|level| level >= crate::os_dsl::Severity::Warning).unwrap_or(false) {
            let edit_ids: Vec<String> = batch.iter().map(|edit| edit.id.clone()).collect();
            let kind = crate::os_spr::ConflictKind::Degraded { edit_ids };
            let id = crate::os_spr::ConflictId::new(&kind, &document_id, &mutation_ids, &self.clock);
            self.envelope.conflicts.push(crate::os_spr::Conflict { id: id.await, kind, status: crate::os_spr::ConflictStatus::Open, messages, actors: actors.await, timestamp: self.clock });
            self.prune_resolved_conflicts();
        }
        self.bump();
        self.last_projection_cause = Some(ArtifactProjectionCause::RemoteIngest);
        Ok(())
    }

    async fn assert_equivalent_remote_edit(&self, remote: &Edit<Mutation>) -> Result<(), VcsError> {
        let document_id = ArtifactId(self.envelope.id.clone());
        let schema = SchemaId(self.envelope.schema.clone());
        let incoming = crate::os_spr::mutation_envelope_from_edit::<P, Mutation>(remote, &document_id, &schema).await.map_err(|error| VcsError::Serialize(error.to_string()))?;
        let mut established: Vec<crate::os_spr::MutationEnvelope> = Vec::new();
        for edit in &self.envelope.vcs.edits {
            established.extend(crate::os_spr::mutation_envelope_from_edit::<P, Mutation>(edit, &document_id, &schema).await.map_err(|error| VcsError::Serialize(error.to_string()))?);
        }
        for envelope in incoming {
            match established.iter().find(|candidate| candidate.mutation_id == envelope.mutation_id) {
                Some(candidate) if Self::same_operation_identity_and_payload(candidate, &envelope) => {}
                Some(_) => return Err(VcsError::ValidationFailed(format!("remote mutation id {} conflicts with its established payload", envelope.mutation_id.0))),
                None => return Err(VcsError::ValidationFailed(format!("remote history claims known mutation id {} without an established payload", envelope.mutation_id.0))),
            }
        }
        Ok(())
    }

    /// @emoji 📥️ Pumps every queued inbound message from the attached backbone into the timeline.
    async fn pump(&mut self) -> Result<bool, VcsError> {
        let Some(mut backbone) = self.backbone.take() else {
            return Ok(false);
        };
        let received = backbone.receive().await;
        self.backbone = Some(backbone);
        let messages = received?;
        if messages.is_empty() {
            return Ok(false);
        }
        let mut acked_op_ids: Vec<String> = Vec::new();
        for message in messages {
            match message {
                BackboneMessage::Snapshot { pack, spr } => self.merge_remote_snapshot(&pack, &spr).await?,
                BackboneMessage::Mutations { envelopes } => {
                    let envelopes = crate::os_spr::decode_envelopes(&envelopes).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
                    let op_ids: Vec<String> = envelopes.iter().map(|envelope| envelope.mutation_id.0.clone()).collect();
                    for envelope in envelopes {
                        self.ingest_remote(envelope).await?;
                    }
                    acked_op_ids.extend(op_ids);
                }
                // A store never consumes acks (they flow store→actor); drain and ignore any that echo back.
                BackboneMessage::Ack { .. } => {}
            }
        }
        if !acked_op_ids.is_empty() {
            if let Some(mut backbone) = self.backbone.take() {
                let result = backbone.send(BackboneMessage::Ack { op_ids: acked_op_ids }).await;
                self.backbone = Some(backbone);
                result?;
            }
        }
        Ok(true)
    }

    /// @emoji 📤️ Sends the just-applied change outward: one {@link crate::os_spr::MutationEnvelope} per
    /// forward op for `Apply` (`crate::os_spr::mutation_envelope_from_edit`'s per-op fan-out — W5/W6),
    /// or a full snapshot for every structural command (undo/redo/checkpoint/alternative/amend).
    async fn flush_outbound(&mut self, is_apply: bool) -> Result<(), VcsError> {
        let Some(mut backbone) = self.backbone.take() else {
            return Ok(());
        };
        let result = if is_apply {
            match self.envelope.vcs.edits.last() {
                Some(edit) => {
                    let document_id = ArtifactId(self.envelope.id.clone());
                    let schema = SchemaId(self.envelope.schema.clone());
                    match crate::os_spr::mutation_envelope_from_edit::<P, Mutation>(edit, &document_id, &schema).await {
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
                            backbone.send(BackboneMessage::Mutations { envelopes: crate::os_spr::encode_envelopes(&op_envelopes).await }).await
                        }
                        Err(error) => Err(VcsError::Serialize(error.to_string())),
                    }
                }
                None => Ok(()),
            }
        } else {
            match self.snapshot_pack().await {
                Ok(files) => backbone.send(BackboneMessage::Snapshot { pack: files.pack, spr: files.spr }).await,
                Err(error) => Err(error),
            }
        };
        self.backbone = Some(backbone);
        result
    }

    /// @emoji 🖋️ Whether `edit_id` was authored by the local actor. Unauthored (legacy) edits count
    /// as local; every other actor is foreign and must not be undone by this store.
    async fn edit_is_local(&self, edit_id: &str) -> bool {
        self.envelope.vcs.edits.iter().find(|edit| edit.id == edit_id).map(|edit| edit.actor.is_none() || edit.actor.as_deref() == self.local_actor_id.as_deref()).unwrap_or(false)
    }

    /// @emoji 🎯️ Mirrors `applied_edit_ids`/`redo_edit_ids`/`current_checkpoint_id` into
    /// `envelope.cursor` — the single choke point that keeps the persisted cursor in sync with
    /// live undo/redo state. Called from every `bump()`, so every mutating command re-syncs it.
    async fn sync_cursor(&mut self) {
        self.envelope.cursor = Some(ArtifactCursor { applied_edit_ids: self.applied_edit_ids.clone(), redo_edit_ids: self.redo_edit_ids.clone(), checkpoint_id: self.current_checkpoint_id.clone() });
    }

    async fn bump(&mut self) {
        self.generation += 1;
        self.sync_cursor();
    }
}

async fn preflight_merge_by_id<T: PartialEq>(local: &[T], remote: &[T], id_of: impl Fn(&T) -> &String) -> Result<(), VcsError> {
    let existing: HashMap<&str, &T> = local.iter().map(|item| (id_of(item).as_str(), item)).collect();
    for item in remote {
        let id = id_of(item).clone();
        match existing.get(id.as_str()) {
            Some(established) if *established == item => {}
            Some(_) => return Err(VcsError::ValidationFailed(format!("remote history conflicts with established record {id}"))),
            None => {}
        }
    }
    Ok(())
}

async fn merge_by_id<T: Clone + PartialEq>(local: &mut Vec<T>, remote: Vec<T>, id_of: impl Fn(&T) -> &String) -> Result<(), VcsError> {
    preflight_merge_by_id(local, &remote, &id_of).await?;
    let mut existing: HashSet<String> = local.iter().map(|item| id_of(item).clone()).collect();
    for item in remote {
        if existing.insert(id_of(&item).clone()) {
            local.push(item);
        }
    }
    Ok(())
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
pub async fn edit_from_operation_envelope<Mutation: OpBinary>(envelope: &crate::os_spr::MutationEnvelope) -> Result<Edit<Mutation>, VcsError> {
    let forward = Mutation::decode_op(&envelope.diff.payload).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let inverse = if envelope.inverse.payload.is_empty() { Vec::new() } else { vec![Mutation::decode_op(&envelope.inverse.payload).await.map_err(|error| VcsError::Deserialize(error.to_string()))?] };
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
            origin: Default::default(),
        }],
        description: None,
        coalesce_key: None,
        sequence_number: 0,
        started_at: String::new(),
        finished_at: None,
    })
}

async fn fold_history<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, applied_edit_ids: &[String]) -> Result<P, VcsError>
where
    P: Clone,
    Mutation: self::Mutation<P>,
{
    let mut snapshot = envelope.vcs.initial_snapshot.clone();
    let mut seen = HashSet::new();
    for edit_id in applied_edit_ids {
        if !seen.insert(edit_id) {
            return Err(VcsError::ValidationFailed(format!("history repeats applied edit {edit_id}")));
        }
        let edit = envelope.vcs.edits.iter().find(|entry| entry.id == *edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
        for operation in &edit.forwards {
            // 🧮️ Mechanical wrap only — see `replay_mutations`'s matching note.
            snapshot = apply_mutation(&snapshot, operation).await?.0;
        }
    }
    Ok(snapshot)
}
//#endregion 🔖️ArtifactStore

//#region 🔖️Backbone
//#region 🔖️Backbone
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
    Ack { op_ids: Vec<String> },
}

//#region 🔖️OpCodec
/// 🎞️ Handcrafted OpText (P6).
impl OpText for BackboneMessage {
    async fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = crate::os_dsl::parse(line, &spec_fn(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline }).await?;
                return <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record).await;
            }
        }
        Err(crate::os_dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self).await;
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline).await
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl OpBinary for BackboneMessage {
    async fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self).await;
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1)();
        let body = crate::os_pack::encode_record_body(&spec, &record, &PackEncodeOptions::default()).await.map_err(crate::os_spr::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        crate::os_pack::write_varint_u64(&mut out, ordinal as u64).await;
        out.extend_from_slice(&body);
        Ok(out)
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = crate::os_pack::ByteReader::new(bytes).await;
        let format = reader.read_u8().await?;
        if format != OP_BINARY_FORMAT {
            return Err(crate::os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64().await?;
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = spec_fn();
        let body = &bytes[reader.position().await..];
        let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &PackDecodeOptions::default()).await.map_err(crate::os_spr::ProtocolError::from)?;
        let record_offset = reader.position().await as u64;
        <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record).await.map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "op record", offset: record_offset, detail: error.to_string() })
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
// 🧵️ No `Send`/`Sync` supertrait bound (R7 coordinator ruling 2026-08-19): this crate is
// guest-reachable and Send-ness comes structurally from the concrete `Backbones` enum at each spawn
// site, never from a bound named here.
pub trait Backbone {
    async fn descriptor(&self) -> ArtifactBackboneRef;
    async fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError>;
    async fn receive(&mut self) -> Result<Vec<BackboneMessage>, VcsError>;
}

pub trait BackbonePort {
    async fn read(&self, uri: &str) -> Result<String, VcsError>;
    async fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError>;
}

static HOST_BACKBONE_PORT: Mutex<Option<Arc<BackbonePorts>>> = Mutex::new(None);

/// @emoji 🔌️ Injects the browser or dev-server backbone port for wasm file/folder IO.
pub async fn set_host_backbone_port(port: Arc<BackbonePorts>) {
    if let Ok(mut guard) = HOST_BACKBONE_PORT.lock() {
        *guard = Some(port);
    }
}

async fn host_backbone_port() -> Option<Arc<BackbonePorts>> {
    HOST_BACKBONE_PORT.lock().ok().and_then(|guard| guard.clone())
}

#[derive(Default)]
pub struct MemoryBackbonePort {
    files: Mutex<HashMap<String, String>>,
}

impl MemoryBackbonePort {
    pub async fn new() -> Self {
        Self::default()
    }
}

impl BackbonePort for MemoryBackbonePort {
    async fn read(&self, uri: &str) -> Result<String, VcsError> {
        self.files.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?.get(uri).cloned().ok_or_else(|| VcsError::Backbone(format!("missing backbone file {uri}")))
    }

    async fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError> {
        self.files.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?.insert(uri.to_string(), payload.to_string());
        Ok(())
    }
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
async fn local_storage_backbone_key(uri: &str) -> String {
    format!("semio:vcs:{uri}")
}

/// @emoji 💾️ Browser `localStorage` backbone port with in-memory fallback for native tests.
pub struct LocalStorageBackbonePort {
    fallback: MemoryBackbonePort,
}

impl LocalStorageBackbonePort {
    pub async fn new() -> Self {
        Self { fallback: MemoryBackbonePort::new().await }
    }
}

impl Default for LocalStorageBackbonePort {
    // 🚫️async: E1 — `Default` is an externally-declared trait; its signature is fixed sync.
    fn default() -> Self {
        Self { fallback: MemoryBackbonePort::default() }
    }
}

impl BackbonePort for LocalStorageBackbonePort {
    async fn read(&self, uri: &str) -> Result<String, VcsError> {
        if let Some(port) = host_backbone_port().await {
            if let Ok(value) = Box::pin(port.read(uri)).await {
                return Ok(value);
            }
        }
        #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(value)) = storage.get_item(&local_storage_backbone_key(uri).await) {
                        return Ok(value);
                    }
                }
            }
        }
        self.fallback.read(uri).await
    }

    async fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError> {
        self.fallback.write(uri, payload).await?;
        if let Some(port) = host_backbone_port().await {
            let _ = Box::pin(port.write(uri, payload)).await;
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

/// @emoji 🧬️ Enum dispatch over every `BackbonePort` implementor (O1 — dyn dispatch is dropped
/// repo-wide in favor of enum/generated dispatch). The trait stays as the contract; this enum
/// implements it by match-delegation so 🪐️space's blanket `impl<T: BackbonePort>
/// SpaceBackbonePort for T` keeps covering every concrete port through the enum too.
pub enum BackbonePorts {
    Memory(MemoryBackbonePort),
    LocalStorage(LocalStorageBackbonePort),
}

impl BackbonePort for BackbonePorts {
    async fn read(&self, uri: &str) -> Result<String, VcsError> {
        match self {
            Self::Memory(port) => port.read(uri).await,
            Self::LocalStorage(port) => port.read(uri).await,
        }
    }

    async fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError> {
        match self {
            Self::Memory(port) => port.write(uri, payload).await,
            Self::LocalStorage(port) => port.write(uri, payload).await,
        }
    }
}

/// @emoji 🕸️ Injectable duplex transport across the wasm sandbox boundary (program ↔ host process).
/// `message`/the `poll` result are `BackboneMessage::encode_op`/`decode_op` (`crate::os_spr::OpBinary`) bytes.
pub trait BackboneChannelPort: Send + Sync {
    async fn send(&self, uri: &str, message: &[u8]) -> Result<(), VcsError>;
    async fn poll(&self, uri: &str) -> Result<Vec<Vec<u8>>, VcsError>;
}

/// @emoji 🕳️ Enum dispatch over every `BackboneChannelPort` implementor — currently NONE (no caller
/// constructs a real channel yet, see `PortBackbone`'s doc comment), so this is uninhabited, exactly
/// like `NoMembers` for `SpaceMember`. The day a real channel lands (the deferred `EffectBackbone`
/// bridge, A2), it becomes a variant here — never a `dyn` object.
pub enum BackboneChannelPorts {}

impl BackboneChannelPort for BackboneChannelPorts {
    async fn send(&self, _uri: &str, _message: &[u8]) -> Result<(), VcsError> {
        match *self {}
    }

    async fn poll(&self, _uri: &str) -> Result<Vec<Vec<u8>>, VcsError> {
        match *self {}
    }
}

/// @emoji 🧵️ Backbone that forwards messages across the wasm sandbox boundary to the host process,
/// which resolves the real `file://`/`folder://`/`remote://` backbone on its own (native) side. The
/// channel is injected per instance via [`PortBackbone::with_channel`] — a pooled multi-instance
/// actor cannot share one process-global channel (see `important.md`'s "Replace, never wrap" list,
/// `set_host_backbone_channel`). No caller constructs a real channel yet (A2, `🔌️plugin/🦀️component.rs`'s
/// `PLUGIN_INIT_ONCE` doc comment: the async `EffectBackbone` bridge is deferred work), so
/// [`PortBackbone::new`] leaves it unset and `send`/`receive` surface a real "no host backbone
/// linked" error rather than silently no-op'ing.
pub struct PortBackbone {
    uri: String,
    channel: Option<Arc<BackboneChannelPorts>>,
}

impl PortBackbone {
    pub async fn new(uri: &str) -> Self {
        Self { uri: uri.to_string(), channel: None }
    }

    pub async fn with_channel(uri: &str, channel: Arc<BackboneChannelPorts>) -> Self {
        Self { uri: uri.to_string(), channel: Some(channel) }
    }
}

impl Backbone for PortBackbone {
    async fn descriptor(&self) -> ArtifactBackboneRef {
        document_backbone_ref(&self.uri).await
    }

    async fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError> {
        let channel = self.channel.as_ref().ok_or_else(|| VcsError::Backbone("backbone channel requires host port".into()))?;
        let bytes = message.encode_op().await.map_err(|error| VcsError::Serialize(error.to_string()))?;
        channel.send(&self.uri, &bytes).await
    }

    async fn receive(&mut self) -> Result<Vec<BackboneMessage>, VcsError> {
        let channel = self.channel.as_ref().ok_or_else(|| VcsError::Backbone("backbone channel requires host port".into()))?;
        let mut messages = Vec::new();
        for bytes in channel.poll(&self.uri).await? {
            messages.push(BackboneMessage::decode_op(&bytes).await.map_err(|e| VcsError::Deserialize(e.to_string()))?);
        }
        Ok(messages)
    }
}

/// @emoji 🔗️ Two crossed in-memory channel ends: whatever `a` sends, `b` receives, and vice versa.
pub struct MemoryBackbone {
    uri: String,
    inbox: Arc<Mutex<VecDeque<BackboneMessage>>>,
    outbox: Arc<Mutex<VecDeque<BackboneMessage>>>,
}

impl MemoryBackbone {
    pub async fn pair(uri_a: &str, uri_b: &str) -> (Self, Self) {
        let a_to_b = Arc::new(Mutex::new(VecDeque::new()));
        let b_to_a = Arc::new(Mutex::new(VecDeque::new()));
        (Self { uri: uri_a.to_string(), inbox: b_to_a.clone(), outbox: a_to_b.clone() }, Self { uri: uri_b.to_string(), inbox: a_to_b, outbox: b_to_a })
    }
}

impl Backbone for MemoryBackbone {
    async fn descriptor(&self) -> ArtifactBackboneRef {
        document_backbone_ref(&self.uri).await
    }

    async fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError> {
        self.outbox.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?.push_back(message);
        Ok(())
    }

    async fn receive(&mut self) -> Result<Vec<BackboneMessage>, VcsError> {
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
    pub async fn pair(uri: &str) -> (ChannelBackbone, ChannelBackboneRemote) {
        let inbound = Arc::new(Mutex::new(VecDeque::new()));
        let outbound = Arc::new(Mutex::new(VecDeque::new()));
        (ChannelBackbone { uri: uri.to_string(), inbound: inbound.clone(), outbound: outbound.clone() }, ChannelBackboneRemote { uri: uri.to_string(), inbound, outbound })
    }
}

impl Backbone for ChannelBackbone {
    async fn descriptor(&self) -> ArtifactBackboneRef {
        document_backbone_ref(&self.uri).await
    }

    async fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError> {
        self.outbound.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?.push_back(message);
        Ok(())
    }

    async fn receive(&mut self) -> Result<Vec<BackboneMessage>, VcsError> {
        let mut inbound = self.inbound.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?;
        Ok(inbound.drain(..).collect())
    }
}

impl ChannelBackboneRemote {
    pub async fn descriptor(&self) -> ArtifactBackboneRef {
        document_backbone_ref(&self.uri).await
    }

    /// @emoji 📥️ Delivers a message to the store's inbound queue (actor→store).
    pub async fn push(&self, message: BackboneMessage) -> Result<(), VcsError> {
        self.inbound.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?.push_back(message);
        Ok(())
    }

    /// @emoji 📤️ Collects everything the store has sent outbound (store→actor), draining the queue.
    pub async fn drain(&self) -> Result<Vec<BackboneMessage>, VcsError> {
        let mut outbound = self.outbound.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?;
        Ok(outbound.drain(..).collect())
    }
}

/// @emoji 🧬️ Enum dispatch over every `Backbone` implementor (O1 — see `BackbonePorts`' doc comment
/// for why: every former `Box<dyn Backbone>` seam becomes `Backbones` by value, no box needed).
pub enum Backbones {
    Port(PortBackbone),
    Memory(MemoryBackbone),
    Channel(ChannelBackbone),
}

impl Backbone for Backbones {
    async fn descriptor(&self) -> ArtifactBackboneRef {
        match self {
            Self::Port(backbone) => backbone.descriptor().await,
            Self::Memory(backbone) => backbone.descriptor().await,
            Self::Channel(backbone) => backbone.descriptor().await,
        }
    }

    async fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError> {
        match self {
            Self::Port(backbone) => backbone.send(message).await,
            Self::Memory(backbone) => backbone.send(message).await,
            Self::Channel(backbone) => backbone.send(message).await,
        }
    }

    async fn receive(&mut self) -> Result<Vec<BackboneMessage>, VcsError> {
        match self {
            Self::Port(backbone) => backbone.receive().await,
            Self::Memory(backbone) => backbone.receive().await,
            Self::Channel(backbone) => backbone.receive().await,
        }
    }
}

/// @emoji 🔌️ Resolves a backbone URI to a concrete channel implementation. Only available inside the
/// wasm sandbox, where every scheme forwards to the host process over the injected
/// {@link BackboneChannelPort} (a pure in-memory queue). Native IO-performing backbones moved out of
/// this crate entirely — the `framework/sync` actor layer owns them.
#[cfg(target_arch = "wasm32")]
pub async fn resolve_backbone(uri: &str) -> Result<Backbones, VcsError> {
    Ok(Backbones::Port(PortBackbone::new(uri).await))
}
//#endregion 🔖️Backbone

//#region 🔖️BlobStore
//#region 🔖️BlobStore
/// @emoji 📦️ A content-addressed blob's identity + metadata. Never carries the bytes themselves —
/// callers that just put/read a blob already hold those; this is what gets embedded in a document
/// (e.g. an `ArtifactKind::ContentAddressedBlob` field) to reference it durably.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlobRef {
    pub hash: String,
    pub size: u64,
    pub media_type: String,
}

/// @emoji 🗄️ Content-addressed blob persistence backing any `ArtifactKind::ContentAddressedBlob`-
/// shaped field that needs to reference bytes durably without embedding them inline. `put` is
/// idempotent — it dedupes by the Blake3 hash of the bytes ({@link framework_hash::hash_bytes}), so
/// writing the same content twice never rewrites storage. Implementors decide the backing medium
/// (sqlite here, a semio_hub HTTP route in a later ticket, an IndexedDB cache in the browser).
pub trait BlobStore: Send + Sync {
    async fn put(&self, bytes: &[u8], media_type: &str) -> Result<BlobRef, VcsError>;
    async fn get(&self, hash: &str) -> Result<Option<Vec<u8>>, VcsError>;
    async fn has(&self, hash: &str) -> Result<bool, VcsError>;
    async fn delete(&self, hash: &str) -> Result<(), VcsError>;
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
// 🧵️ No `Send` supertrait bound (R7 coordinator ruling 2026-08-19): this crate is guest-reachable
// and `Send` comes structurally from the concrete `space_members!`-generated enum at each spawn
// site, never from a bound named here.
pub trait SpaceMember {
    async fn document_id(&self) -> &str;
    /// @emoji 🩸️ Whether this member has edits applied since its last checkpoint (mirrors the
    /// `CommitCheckpoint` dispatch's own "nothing to commit" check via `uncommitted_edit_ids`).
    async fn is_dirty(&self) -> bool;
    async fn commit_checkpoint(&mut self, message: String, authors: Vec<Author>) -> Result<String, VcsError>;
    async fn current_checkpoint_id(&self) -> Option<String>;
    async fn current_alternative_id(&self) -> Option<String>;
    async fn checkout(&mut self, checkpoint_id: &str, alternative_id: &str) -> Result<(), VcsError>;
    async fn create_alternative(&mut self, name: String) -> Result<String, VcsError>;
    // 🎞️ CW3: `crate::os_spr::HybridLogicalTimestamp` (not `semio_framework`'s local one) — these
    // read `MutationMeta.timestamp`, which is the moved struct's field, typed against protocol_core.
    async fn last_local_edit_timestamp(&self) -> Option<HybridLogicalTimestamp>;
    async fn last_undone_local_edit_timestamp(&self) -> Option<HybridLogicalTimestamp>;
    async fn undo(&mut self) -> Result<(), VcsError>;
    async fn redo(&mut self) -> Result<(), VcsError>;
    /// @emoji 🪄️ Downcast escape hatch: a space host UI (or a test) needs the concrete
    /// `ArtifactStore<P, Mutation>` back out of a `Box<dyn SpaceMember>` — e.g. to `Apply` a
    /// technology-specific `Mutation`, which can't appear in this object-safe trait. `Self: 'static` is
    /// implied by every real `P`/`Mutation` pair, so this never fails for a genuine member.
    async fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    // 🎯️ B2 `CompositionCoordinator` seam (`UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`, `🔖️CompositionCoordinator`
    // region below `🔖️Space`): nine object-safe methods, in two groups. `preview_wire`/
    // `dispatch_wire`/`tail_group_id` are the three the task brief named explicitly (`preview_wire`
    // itself renamed from `validate_wire` by `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-
    // CLASS-CONFLICTS` §C6, once its `Result<(), String>` early-reject shape was replaced by the
    // `Vec<MutationMessage>` dry run every `CompositionCoordinator` phase 1 now unions across
    // members). The other six (`tail_edit_id`, `redo_tail`, `stamp_tail_group_id`, `set_owner`,
    // `merge_policy`) are necessary, deliberate additions beyond that literal list — see
    // `📓️wave1-reports/b2-store-composition-report.md`'s "Design decisions" for why each is
    // unavoidable given the object-safety constraint: `GroupReceipt`/`GroupUndoReport` need real
    // edit ids (not just group membership), `dispatch_group`'s phase 2 needs a way to stamp a shared
    // `group_id` onto a member AFTER an ordinary `Apply` already hard-codes `group_id: None`,
    // genesis needs a way to set `ArtifactEnvelope.owner` on a freshly-created child through the
    // same type-erased interface, and `merge_policy` is what lets phase 1 fold every member's
    // `preview_wire` messages against one authority-chosen `crate::os_spr::MergePolicy` without
    // downcasting.
    /// @emoji 🧪️ Decodes `ops` as a sequence of individually-`OpBinary`-encoded `Mutation`s (the
    /// SAME wire shape `ArtifactCommand::Apply.mutations` bundles — see `dispatch_wire`'s doc
    /// comment) and dry-runs each against a snapshot threaded forward through the whole slice, so
    /// op `i` previews against the state ops `0..i` would produce, never a stale base. Mirrors the
    /// real apply algorithm (`📋️contract-freeze.md` §C6 `replay_mutations`: every op is diffed and
    /// folded in order, nothing stops early — a `Fatal`/`Error` message's diff is a no-op by
    /// construction (§C2 LAWS 1/2), so threading through it is always safe) and returns EVERY
    /// message every op raised, each stamped with its `op_index`; only a structural failure (a
    /// snapshot read error, or an op that fails to even decode) short-circuits the remaining ops,
    /// itself reported as one `mutation.invariant` `Fatal` message. Never applies anything — the
    /// live store is untouched no matter what this returns. `CompositionCoordinator` phase 1 unions
    /// this across every member, computes ONE worst `crate::os_dsl::Severity`, and consults
    /// `merge_policy()` to decide accept/reject for the whole group.
    async fn preview_wire(&self, ops: &[Vec<u8>]) -> Vec<crate::os_spr::MutationMessage>;
    /// @emoji 📡️ Decodes `cmd_bytes` as one binary `ArtifactCommand<Mutation>` and dispatches it via
    /// `dispatch_binary`. `CompositionCoordinator` builds `cmd_bytes` from a member's
    /// `ChildDispatch.ops`/`parent_ops` by replicating `write_command_ops`'s byte layout directly
    /// (length-prefixed already-encoded op bytes) WITHOUT ever decoding an individual op — the
    /// reason this whole family takes/returns bytes instead of a typed `ArtifactCommand<Mutation>`
    /// is that `SpaceMember` itself must stay object-safe (no generic method can appear on a trait
    /// object), so the coordinator stays fully agnostic of every member's concrete `Mutation` type.
    async fn dispatch_wire(&mut self, cmd_bytes: &[u8]) -> Result<CommandReceipt, VcsError>;
    /// @emoji ⚖️ Applies a preflighted group command under the authority-selected policy without
    /// changing this member's local policy after dispatch.
    async fn dispatch_wire_with_policy(&mut self, cmd_bytes: &[u8], policy: crate::os_spr::MergePolicy) -> Result<CommandReceipt, VcsError>;
    /// @emoji 🏷️ The `MutationMeta.group_id` recorded on this member's TAIL applied edit's last
    /// operation, if any — lets `CompositionCoordinator::undo_group` recognize "does this member's
    /// most recent edit belong to composite gesture X" without downcasting to a concrete
    /// `ArtifactStore<P, Mutation>`.
    async fn tail_group_id(&self) -> Option<String>;
    /// @emoji 🆔️ The id of this member's TAIL applied edit, if any — `tail_group_id`'s companion
    /// getter, so `GroupReceipt`/`GroupUndoReport` can report WHICH edit a group touched/undid, not
    /// only that group membership matched.
    async fn tail_edit_id(&self) -> Option<String>;
    /// @emoji ↩️🏷️ `(tail_group_id, tail_edit_id)`'s REDO-direction mirror: the `(edit_id,
    /// group_id)` of whatever edit sits at the top of this member's redo stack (the one a following
    /// `redo()` would reapply), used by `CompositionCoordinator::redo_group` the way `tail_group_id`/
    /// `tail_edit_id` are used by `undo_group`.
    async fn redo_tail(&self) -> Option<(String, Option<String>)>;
    /// @emoji 🖋️ Stamps `group_id` onto every `MutationMeta` entry of this member's TAIL applied
    /// edit — the mechanism `CompositionCoordinator::dispatch_group`'s phase 2 uses to give every
    /// member of one composite gesture the SAME `MutationMeta.group_id` after dispatching each
    /// member's own `Apply` independently (the ordinary `Apply` path has no way to accept an
    /// externally-supplied group id — see `ArtifactStore::replay_mutations`, which always stamps
    /// `group_id: None`). Errors with `VcsError::UnknownEdit` if this member has no applied edits at
    /// all — never true on the path `dispatch_group` actually calls it from, since it always calls
    /// this immediately after a successful `dispatch_wire`.
    async fn stamp_tail_group_id(&mut self, group_id: &str) -> Result<(), VcsError>;
    /// @emoji 🔀️ Stamps `origin` onto every `MutationMeta` entry of this member's TAIL applied
    /// edit — `stamp_tail_group_id`'s provenance-direction twin, used by
    /// `TransactionCoordinator::dispatch_group`'s `Peer` relation to mark a foreign member's edit
    /// `crate::os_spr::MutationOrigin::Transaction { initiator }` after dispatching it (the
    /// initiator's OWN edit is left `Owner` — it is not foreign to itself). Never called on the
    /// `Owned` relation path, which keeps every member's origin at its ordinary `Apply`-assigned
    /// default (`Owner`) — this is precisely what keeps `Owned` byte-identical to its pre-`Peer`
    /// behaviour. Same `VcsError::UnknownEdit` failure mode as `stamp_tail_group_id` (never true on
    /// the path `dispatch_group` calls it from).
    async fn stamp_tail_origin(&mut self, origin: crate::os_spr::MutationOrigin) -> Result<(), VcsError>;
    /// @emoji 🏠️ Sets (or clears) this member's own envelope `owner` stamp — the mechanism
    /// `CompositionCoordinator::dispatch_group`'s phase 2 uses to record a freshly-`ChildGenesis`-
    /// created child's `OwnerRef` directly on the child's own envelope (see
    /// `ArtifactEnvelope.owner`'s doc comment for why ownership must be queryable from the child
    /// side, not only from the parent's `ArtifactChild` handle). Not part of the ordinary
    /// VCS/dispatch surface — no ordinary `Apply` mutation can reach envelope metadata — so it needs
    /// its own object-safe setter.
    async fn set_owner(&mut self, owner: Option<OwnerRef>);

    // 📖️ Object-safe READ surface (this ticket's CW1-1b). Everything above either mutates a member
    // or reports a scalar about it; nothing could get a member's CONTENT back out without
    // downcasting through `as_any_mut` to a concrete `ArtifactStore<P, Mutation>` the caller must
    // already know the types of. That is exactly what a composition parent cannot do (its children
    // are heterogeneous and plugin-defined), so `ArtifactView.children` and `LinkResolver` both
    // dead-ended here. All three return PACK bytes rather than a typed value for the same reason
    // `dispatch_wire` takes bytes: no generic method can appear on a trait object.
    /// @emoji 📦️ This member's CURRENT materialized snapshot, pack-encoded — the live content a
    /// composition parent reads through `ArtifactView.children` and a `LinkPin::Head` resolves to.
    /// Reads through the live store, so it cannot go stale behind an undo/redo/checkout the way the
    /// `thread_local!` child caches this replaces did.
    async fn document_pack_bytes(&self) -> Result<Vec<u8>, VcsError>;
    /// @emoji 🗄️ This member's WHOLE envelope (initial snapshot pack + `.spr` op log, in
    /// `encode_document_pack_bytes` framing) — what gets persisted for a child and handed back to
    /// `ChildStoreFactory::open` on reload. The full history, not just the current content.
    async fn envelope_pack_bytes(&self) -> Result<Vec<u8>, VcsError>;
    /// @emoji ⏮️📦️ This member's snapshot AS OF `checkpoint_id`, pack-encoded, without disturbing the
    /// live cursor — replays exactly the edit ids that checkpoint's changes cover, the same set
    /// `checkout_checkpoint_internal` would install. This is what makes `LinkPin::Checkpoint` real:
    /// a pinned reference resolves to the target's historical content rather than silently
    /// degrading to its head.
    async fn pack_at_checkpoint(&self, checkpoint_id: &str) -> Result<Vec<u8>, VcsError>;

    /// @emoji ⚖️ This member's own `crate::os_spr::MergePolicy` — local/authority state (§C3: never
    /// wire-carried, never part of shared history), consulted by `CompositionCoordinator` phase 1
    /// to decide whether the group's unioned `preview_wire` messages are accepted or rejected.
    /// Defaults to `MergePolicy::default()` (`Normal`) so every member reports a policy even before
    /// an authority has configured one; `ArtifactStore::merge_policy`/`set_merge_policy` (§C6, lane
    /// 1-A's `🔖️ArtifactStore` region) is the real per-store override this default defers to once
    /// landed — override this method in the blanket impl below to delegate to it the moment that
    /// inherent method exists (Rust picks the inherent method over this trait one automatically, the
    /// same pattern `current_checkpoint_id` already relies on just above).
    async fn merge_policy(&self) -> crate::os_spr::MergePolicy {
        crate::os_spr::MergePolicy::default()
    }
}

impl<P, Mutation> SpaceMember for ArtifactStore<P, Mutation>
where
    P: Clone + Serialize + DeserializeOwned + ArtifactPack + Send + 'static,
    Mutation: Clone + Serialize + DeserializeOwned + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    async fn document_id(&self) -> &str {
        self.envelope().await.id.as_str()
    }

    async fn is_dirty(&self) -> bool {
        !uncommitted_edit_ids(&self.envelope, self.applied_edit_ids().await).await.is_empty()
    }

    async fn commit_checkpoint(&mut self, message: String, authors: Vec<Author>) -> Result<String, VcsError> {
        self.dispatch(ArtifactCommand::CommitCheckpoint { message: Some(message), authors }).await?;
        // `self.current_checkpoint_id()` resolves to the inherent method (`Option<&str>`), not this
        // trait method — Rust prefers inherent methods over trait methods of the same name.
        self.current_checkpoint_id().await.map(|id| id.to_string()).ok_or(VcsError::NoCheckpoint)
    }

    async fn current_checkpoint_id(&self) -> Option<String> {
        self.current_checkpoint_id().await.map(|id| id.to_string())
    }

    async fn current_alternative_id(&self) -> Option<String> {
        self.envelope().await.active_alternative_id.clone()
    }

    async fn checkout(&mut self, checkpoint_id: &str, alternative_id: &str) -> Result<(), VcsError> {
        if !alternative_id.is_empty() {
            let is_alternative_tip = self.envelope().await.vcs.alternatives.iter().find(|alternative| alternative.id == alternative_id).map(|alternative| alternative.checkpoint_ids.last().map(String::as_str) == Some(checkpoint_id)).unwrap_or(false);
            if is_alternative_tip {
                return self.dispatch(ArtifactCommand::SwitchAlternative { alternative_id: alternative_id.to_string() }).await.map(|_| ());
            }
        }
        self.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: checkpoint_id.to_string() }).await.map(|_| ())
    }

    async fn create_alternative(&mut self, name: String) -> Result<String, VcsError> {
        self.dispatch(ArtifactCommand::CreateAlternative { name }).await?;
        self.envelope().await.active_alternative_id.clone().ok_or(VcsError::NoCheckpoint)
    }

    // 🌀️ `edit_is_local`/`envelope` are async; `find_map`'s closure is sync (R10 shape 1), so both
    // functions below use an explicit loop instead.
    async fn last_local_edit_timestamp(&self) -> Option<HybridLogicalTimestamp> {
        let envelope = self.envelope().await;
        for edit_id in self.applied_edit_ids().await.iter().rev() {
            if !self.edit_is_local(edit_id).await {
                continue;
            }
            if let Some(timestamp) = envelope.vcs.edits.iter().find(|edit| edit.id == *edit_id).and_then(|edit| edit.mutation_meta.last()).map(|meta| meta.timestamp) {
                return Some(timestamp);
            }
        }
        None
    }

    async fn last_undone_local_edit_timestamp(&self) -> Option<HybridLogicalTimestamp> {
        let envelope = self.envelope().await;
        for edit_id in self.redo_edit_ids().await.iter().rev() {
            if !self.edit_is_local(edit_id).await {
                continue;
            }
            if let Some(timestamp) = envelope.vcs.edits.iter().find(|edit| edit.id == *edit_id).and_then(|edit| edit.mutation_meta.last()).map(|meta| meta.timestamp) {
                return Some(timestamp);
            }
        }
        None
    }

    async fn undo(&mut self) -> Result<(), VcsError> {
        self.dispatch(ArtifactCommand::Undo).await.map(|_| ())
    }

    async fn redo(&mut self) -> Result<(), VcsError> {
        self.dispatch(ArtifactCommand::Redo).await.map(|_| ())
    }

    async fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    async fn preview_wire(&self, ops: &[Vec<u8>]) -> Vec<crate::os_spr::MutationMessage> {
        let mut running = match self.snapshot().await {
            Ok(snapshot) => snapshot,
            Err(error) => return vec![crate::os_spr::MutationMessage::fatal("mutation.invariant", error.to_string()).await],
        };
        let mut all_messages = Vec::new();
        for (index, op) in ops.iter().enumerate() {
            let mutation = match <Mutation as OpBinary>::decode_op(op).await {
                Ok(mutation) => mutation,
                Err(error) => {
                    all_messages.push(crate::os_spr::MutationMessage::fatal("mutation.invariant", error.to_string()).await.at_op(index as u32).await);
                    break;
                }
            };
            let (next, messages) = match apply_mutation(&running, &mutation).await {
                Ok(applied) => applied,
                Err(error) => {
                    all_messages.push(crate::os_spr::MutationMessage::fatal("mutation.invariant", error.message).await.at(error.target).await.at_op(index as u32).await);
                    break;
                }
            };
            // 🌀️ `MutationMessage::at_op` is async (📡️replication); `Iterator::map`'s closure is
            // sync (R10 shape 1), so it's hoisted into an explicit loop.
            for message in messages {
                all_messages.push(message.at_op(index as u32).await);
            }
            running = next;
        }
        all_messages
    }

    async fn dispatch_wire(&mut self, cmd_bytes: &[u8]) -> Result<CommandReceipt, VcsError> {
        self.dispatch_binary(cmd_bytes).await
    }

    async fn dispatch_wire_with_policy(&mut self, cmd_bytes: &[u8], policy: crate::os_spr::MergePolicy) -> Result<CommandReceipt, VcsError> {
        let local_policy = self.merge_policy;
        self.merge_policy = policy;
        // 🌀️ The unawaited future holds `&mut self`, so `self.merge_policy` cannot be reassigned
        // while it's still live (E0506) — awaited immediately instead of deferred, matching the
        // original synchronous "restore after dispatch completes" intent.
        let result = self.dispatch_binary(cmd_bytes).await;
        self.merge_policy = local_policy;
        result
    }

    async fn tail_group_id(&self) -> Option<String> {
        let edit_id = self.applied_edit_ids().await.last()?;
        self.envelope().await.vcs.edits.iter().find(|edit| edit.id == *edit_id)?.mutation_meta.last()?.group_id.clone()
    }

    async fn tail_edit_id(&self) -> Option<String> {
        self.applied_edit_ids().await.last().cloned()
    }

    async fn redo_tail(&self) -> Option<(String, Option<String>)> {
        let edit_id = self.redo_edit_ids().await.last()?.clone();
        let group_id = self.envelope().await.vcs.edits.iter().find(|edit| edit.id == edit_id).and_then(|edit| edit.mutation_meta.last()).and_then(|meta| meta.group_id.clone());
        Some((edit_id, group_id))
    }

    async fn stamp_tail_group_id(&mut self, group_id: &str) -> Result<(), VcsError> {
        let edit_id = self.applied_edit_ids().await.last().cloned().ok_or(VcsError::NothingToUndo)?;
        let edit = self.envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
        for meta in edit.mutation_meta.iter_mut() {
            meta.group_id = Some(group_id.to_string());
        }
        Ok(())
    }

    async fn stamp_tail_origin(&mut self, origin: crate::os_spr::MutationOrigin) -> Result<(), VcsError> {
        let edit_id = self.applied_edit_ids().await.last().cloned().ok_or(VcsError::NothingToUndo)?;
        let edit = self.envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
        for meta in edit.mutation_meta.iter_mut() {
            meta.origin = origin.clone();
        }
        Ok(())
    }

    async fn set_owner(&mut self, owner: Option<OwnerRef>) {
        self.envelope.owner = owner;
    }

    async fn document_pack_bytes(&self) -> Result<Vec<u8>, VcsError> {
        Ok(self.snapshot().await?.encode_pack().await)
    }

    async fn envelope_pack_bytes(&self) -> Result<Vec<u8>, VcsError> {
        let files = print_document_pack(self.envelope().await).await?;
        Ok(encode_document_pack_bytes(&files.pack, &files.spr).await)
    }

    async fn pack_at_checkpoint(&self, checkpoint_id: &str) -> Result<Vec<u8>, VcsError> {
        let envelope = self.envelope().await;
        let checkpoint = envelope.vcs.checkpoints.iter().find(|checkpoint| checkpoint.id == checkpoint_id).ok_or_else(|| VcsError::UnknownChange(checkpoint_id.to_string()))?;
        let edit_ids = edit_ids_for_changes(envelope, &checkpoint.change_ids).await;
        Ok(materialize_document_snapshot(envelope, &edit_ids).await?.encode_pack().await)
    }

    /// 🎯️ Overrides the trait default (`MergePolicy::default()`) now that lane 1-A's real
    /// `ArtifactStore::merge_policy` inherent method has landed (§C6) — `self.merge_policy()`
    /// resolves to the INHERENT method here (Rust prefers it over the trait one of the same name),
    /// exactly the `current_checkpoint_id` precedent this file already documents.
    async fn merge_policy(&self) -> crate::os_spr::MergePolicy {
        self.merge_policy().await
    }
}

/// @emoji 🏭️ Replaces the old `ChildStoreFactory` object (O1 — a global `Arc<dyn ChildStoreFactory>`
/// registry keyed by a runtime kind string is exactly the dyn-dispatched seam the program drops).
/// A `space_members!`-generated enum implements this by matching `kind` over its own variants — the
/// registry's kind-keying moves INTO the enum, closed and known at the composing plugin's own
/// compile time, rather than living in a process-global mutable table of trait objects.
pub trait MemberFactory: Sized {
    async fn create(kind: &str, id: &str, dialect: &crate::os_io::ArtifactDialect, initial_pack: &[u8]) -> Result<Self, VcsError>;
    async fn open(kind: &str, envelope_pack: &[u8]) -> Result<Self, VcsError>;
}

/// @emoji 🕳️ Uninhabited default `SpaceHost`/`CompositionCoordinator` member type — the STABLE
/// struct-param default so the many plugins that never compose members change nothing. Same
/// stand-in shape as `BackboneChannelPorts`/`NoBlobStore`.
pub enum NoMembers {}

impl SpaceMember for NoMembers {
    async fn document_id(&self) -> &str {
        match *self {}
    }

    async fn is_dirty(&self) -> bool {
        match *self {}
    }

    async fn commit_checkpoint(&mut self, _message: String, _authors: Vec<Author>) -> Result<String, VcsError> {
        match *self {}
    }

    async fn current_checkpoint_id(&self) -> Option<String> {
        match *self {}
    }

    async fn current_alternative_id(&self) -> Option<String> {
        match *self {}
    }

    async fn checkout(&mut self, _checkpoint_id: &str, _alternative_id: &str) -> Result<(), VcsError> {
        match *self {}
    }

    async fn create_alternative(&mut self, _name: String) -> Result<String, VcsError> {
        match *self {}
    }

    async fn last_local_edit_timestamp(&self) -> Option<HybridLogicalTimestamp> {
        match *self {}
    }

    async fn last_undone_local_edit_timestamp(&self) -> Option<HybridLogicalTimestamp> {
        match *self {}
    }

    async fn undo(&mut self) -> Result<(), VcsError> {
        match *self {}
    }

    async fn redo(&mut self) -> Result<(), VcsError> {
        match *self {}
    }

    async fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        match *self {}
    }

    async fn preview_wire(&self, _ops: &[Vec<u8>]) -> Vec<crate::os_spr::MutationMessage> {
        match *self {}
    }

    async fn dispatch_wire(&mut self, _cmd_bytes: &[u8]) -> Result<CommandReceipt, VcsError> {
        match *self {}
    }

    async fn dispatch_wire_with_policy(&mut self, _cmd_bytes: &[u8], _policy: crate::os_spr::MergePolicy) -> Result<CommandReceipt, VcsError> {
        match *self {}
    }

    async fn tail_group_id(&self) -> Option<String> {
        match *self {}
    }

    async fn tail_edit_id(&self) -> Option<String> {
        match *self {}
    }

    async fn redo_tail(&self) -> Option<(String, Option<String>)> {
        match *self {}
    }

    async fn stamp_tail_group_id(&mut self, _group_id: &str) -> Result<(), VcsError> {
        match *self {}
    }

    async fn stamp_tail_origin(&mut self, _origin: crate::os_spr::MutationOrigin) -> Result<(), VcsError> {
        match *self {}
    }

    async fn set_owner(&mut self, _owner: Option<OwnerRef>) {
        match *self {}
    }

    async fn document_pack_bytes(&self) -> Result<Vec<u8>, VcsError> {
        match *self {}
    }

    async fn envelope_pack_bytes(&self) -> Result<Vec<u8>, VcsError> {
        match *self {}
    }

    async fn pack_at_checkpoint(&self, _checkpoint_id: &str) -> Result<Vec<u8>, VcsError> {
        match *self {}
    }
}

impl MemberFactory for NoMembers {
    async fn create(kind: &str, _id: &str, _dialect: &crate::os_io::ArtifactDialect, _initial_pack: &[u8]) -> Result<Self, VcsError> {
        Err(VcsError::ValidationFailed(format!("no member kind '{kind}' registered (this composition is NoMembers — composition disabled for this store)")))
    }

    async fn open(kind: &str, _envelope_pack: &[u8]) -> Result<Self, VcsError> {
        Err(VcsError::ValidationFailed(format!("no member kind '{kind}' registered (this composition is NoMembers — composition disabled for this store)")))
    }
}

/// @emoji 🧬️ Generates a per-plugin `SpaceMember`+`MemberFactory` enum spanning several document
/// kinds composed under one `SpaceHost`/`CompositionCoordinator` — the O1 replacement for a
/// `Box<dyn SpaceMember>` heterogeneous registry. Delegation arms are written ONCE here, next to
/// `trait SpaceMember` itself, so drift between the trait and the macro is a compile error, never a
/// silent bug. Usage:
/// ```ignore
/// space_members! {
///     pub enum NoteMembers {
///         Text("s.note.text", "note.text/v1") => ArtifactStore<TextSnapshot, TextMutation>,
///         Sketch("s.note.sketch", "note.sketch/v1") => ArtifactStore<SketchSnapshot, SketchMutation>,
///     }
/// }
/// ```
/// expands to the enum, `impl SpaceMember for NoteMembers` (match-delegation over all 22 non-default
/// methods), and `impl MemberFactory for NoteMembers` (`create`/`open` matching `kind` against each
/// variant's kind string via `create_member_store`/`open_member_store`).
#[macro_export]
macro_rules! space_members {
    (pub enum $enum_name:ident { $($variant:ident($kind:literal, $schema:literal) => $inner:ty),+ $(,)? }) => {
        pub enum $enum_name {
            $($variant($inner)),+
        }

        impl $crate::os_store::SpaceMember for $enum_name {
            async fn document_id(&self) -> &str {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::document_id(m).await),+ }
            }
            async fn is_dirty(&self) -> bool {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::is_dirty(m).await),+ }
            }
            async fn commit_checkpoint(&mut self, message: String, authors: Vec<$crate::os_store::Author>) -> Result<String, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::commit_checkpoint(m, message, authors).await),+ }
            }
            async fn current_checkpoint_id(&self) -> Option<String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::current_checkpoint_id(m).await),+ }
            }
            async fn current_alternative_id(&self) -> Option<String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::current_alternative_id(m).await),+ }
            }
            async fn checkout(&mut self, checkpoint_id: &str, alternative_id: &str) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::checkout(m, checkpoint_id, alternative_id).await),+ }
            }
            async fn create_alternative(&mut self, name: String) -> Result<String, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::create_alternative(m, name).await),+ }
            }
            async fn last_local_edit_timestamp(&self) -> Option<$crate::os_store::HybridLogicalTimestamp> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::last_local_edit_timestamp(m).await),+ }
            }
            async fn last_undone_local_edit_timestamp(&self) -> Option<$crate::os_store::HybridLogicalTimestamp> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::last_undone_local_edit_timestamp(m).await),+ }
            }
            async fn undo(&mut self) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::undo(m).await),+ }
            }
            async fn redo(&mut self) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::redo(m).await),+ }
            }
            async fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::as_any_mut(m).await),+ }
            }
            async fn preview_wire(&self, ops: &[Vec<u8>]) -> Vec<$crate::os_spr::MutationMessage> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::preview_wire(m, ops).await),+ }
            }
            async fn dispatch_wire(&mut self, cmd_bytes: &[u8]) -> Result<$crate::os_store::CommandReceipt, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::dispatch_wire(m, cmd_bytes).await),+ }
            }
            async fn dispatch_wire_with_policy(&mut self, cmd_bytes: &[u8], policy: $crate::os_spr::MergePolicy) -> Result<$crate::os_store::CommandReceipt, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::dispatch_wire_with_policy(m, cmd_bytes, policy).await),+ }
            }
            async fn tail_group_id(&self) -> Option<String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::tail_group_id(m).await),+ }
            }
            async fn tail_edit_id(&self) -> Option<String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::tail_edit_id(m).await),+ }
            }
            async fn redo_tail(&self) -> Option<(String, Option<String>)> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::redo_tail(m).await),+ }
            }
            async fn stamp_tail_group_id(&mut self, group_id: &str) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::stamp_tail_group_id(m, group_id).await),+ }
            }
            async fn stamp_tail_origin(&mut self, origin: $crate::os_spr::MutationOrigin) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::stamp_tail_origin(m, origin).await),+ }
            }
            async fn set_owner(&mut self, owner: Option<$crate::os_store::OwnerRef>) {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::set_owner(m, owner).await),+ }
            }
            async fn document_pack_bytes(&self) -> Result<Vec<u8>, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::document_pack_bytes(m).await),+ }
            }
            async fn envelope_pack_bytes(&self) -> Result<Vec<u8>, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::envelope_pack_bytes(m).await),+ }
            }
            async fn pack_at_checkpoint(&self, checkpoint_id: &str) -> Result<Vec<u8>, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::pack_at_checkpoint(m, checkpoint_id).await),+ }
            }
            async fn merge_policy(&self) -> $crate::os_spr::MergePolicy {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::merge_policy(m).await),+ }
            }
        }

        impl $crate::os_store::MemberFactory for $enum_name {
            async fn create(kind: &str, id: &str, dialect: &$crate::os_io::ArtifactDialect, initial_pack: &[u8]) -> Result<Self, $crate::os_store::VcsError> {
                match kind {
                    $($kind => Ok(Self::$variant($crate::os_store::create_member_store($schema, id, dialect, initial_pack).await?)),)+
                    other => Err($crate::os_store::VcsError::ValidationFailed(format!("no member kind '{other}' registered in {}", stringify!($enum_name)))),
                }
            }
            async fn open(kind: &str, envelope_pack: &[u8]) -> Result<Self, $crate::os_store::VcsError> {
                match kind {
                    $($kind => Ok(Self::$variant($crate::os_store::open_member_store(envelope_pack).await?)),)+
                    other => Err($crate::os_store::VcsError::ValidationFailed(format!("no member kind '{other}' registered in {}", stringify!($enum_name)))),
                }
            }
        }
    };
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
    async fn apply(&self, snapshot: &SpaceHistorySnapshot) -> crate::os_spr::MutationApplyResult<SpaceHistorySnapshot> {
        let mut next = snapshot.clone();
        if let Some(checkpoint) = &self.add_checkpoint {
            if next.checkpoints.iter().any(|existing| existing.id == checkpoint.id) {
                return Err(crate::os_spr::MutationApplyError::new("mutation.apply.duplicate-target", format!("checkpoint {} already exists", checkpoint.id)).await.at(["checkpoints", checkpoint.id.as_str()]).await);
            }
            next.checkpoints.push(checkpoint.clone());
        }
        if let Some(checkpoint_id) = &self.remove_checkpoint_id {
            if !next.checkpoints.iter().any(|checkpoint| checkpoint.id == *checkpoint_id) {
                return Err(crate::os_spr::MutationApplyError::new("mutation.apply.missing-target", format!("checkpoint {checkpoint_id} does not exist")).await.at(["checkpoints", checkpoint_id.as_str()]).await);
            }
            next.checkpoints.retain(|checkpoint| checkpoint.id != *checkpoint_id);
        }
        if let Some(alternative) = &self.add_alternative {
            if next.alternatives.iter().any(|existing| existing.id == alternative.id) {
                return Err(crate::os_spr::MutationApplyError::new("mutation.apply.duplicate-target", format!("alternative {} already exists", alternative.id)).await.at(["alternatives", alternative.id.as_str()]).await);
            }
            next.alternatives.push(alternative.clone());
        }
        if let Some(alternative_id) = &self.remove_alternative_id {
            if !next.alternatives.iter().any(|alternative| alternative.id == *alternative_id) {
                return Err(crate::os_spr::MutationApplyError::new("mutation.apply.missing-target", format!("alternative {alternative_id} does not exist")).await.at(["alternatives", alternative_id.as_str()]).await);
            }
            next.alternatives.retain(|alternative| alternative.id != *alternative_id);
        }
        if let Some(active) = &self.set_active_alternative_id {
            if let Some(alternative_id) = active {
                if !next.alternatives.iter().any(|alternative| alternative.id == *alternative_id) {
                    return Err(crate::os_spr::MutationApplyError::new("mutation.apply.missing-target", format!("active alternative {alternative_id} does not exist")).await.at(["activeAlternativeId"]).await);
                }
            }
            next.active_alternative_id = active.clone();
        }
        Ok(next)
    }

    async fn absorb(&mut self, other: Self) {
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

    /// 🧮️ Mechanical wrap only (26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-
    /// CONFLICTS W0): no `Error`/`Warning`/`Fatal` messages added here yet.
    async fn diff(&self, _snapshot: &SpaceHistorySnapshot) -> crate::os_spr::MutationOutcome<SpaceHistoryDiff> {
        let diff = match self {
            SpaceHistoryMutation::CommitSpaceCheckpoint { checkpoint } => SpaceHistoryDiff { add_checkpoint: Some(checkpoint.clone()), ..Default::default() },
            SpaceHistoryMutation::CreateSpaceAlternative { alternative } => SpaceHistoryDiff { add_alternative: Some(alternative.clone()), set_active_alternative_id: Some(Some(alternative.id.clone())), ..Default::default() },
            SpaceHistoryMutation::SwitchSpaceAlternative { alternative_id } => SpaceHistoryDiff { set_active_alternative_id: Some(Some(alternative_id.clone())), ..Default::default() },
            SpaceHistoryMutation::RemoveSpaceCheckpoint { checkpoint_id } => SpaceHistoryDiff { remove_checkpoint_id: Some(checkpoint_id.clone()), ..Default::default() },
            SpaceHistoryMutation::RemoveSpaceAlternative { alternative_id } => SpaceHistoryDiff { remove_alternative_id: Some(alternative_id.clone()), ..Default::default() },
            SpaceHistoryMutation::SetActiveSpaceAlternative { alternative_id } => SpaceHistoryDiff { set_active_alternative_id: Some(alternative_id.clone()), ..Default::default() },
        };
        crate::os_spr::MutationOutcome::new(diff).await
    }

    async fn inverse(&self, snapshot: &SpaceHistorySnapshot) -> Vec<Self> {
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
    async fn print_op(&self) -> String {
        serde_json::to_string(self).expect("SpaceHistoryMutation serializes infallibly")
    }
    async fn parse_op(line: &str) -> Result<Self, TextError> {
        serde_json::from_str(line).map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))
    }
}
impl OpBinary for SpaceHistoryMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        let value = to_dsl_value(self).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "space history op", offset: 0, detail: error })?;
        Ok(pack_rt::encode_pack_value(&value).await)
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        let value = pack_rt::decode_pack_value(bytes).await.map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "space history op", offset: 0, detail: error.to_string() })?;
        from_dsl_value(renormalize_whole_number_floats(value)).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "space history op", offset: 0, detail: error })
    }
}
impl ArtifactDsl for SpaceHistorySnapshot {
    const EXTENSION: &'static str = "space-history";
    async fn parse_dsl(text: &str) -> Result<Self, TextError> {
        serde_json::from_str(text).map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))
    }
    async fn print_dsl(&self) -> String {
        serde_json::to_string(self).expect("SpaceHistorySnapshot serializes infallibly")
    }
}
impl ArtifactPack for SpaceHistorySnapshot {
    async fn encode_pack_with(&self, _options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let value = to_dsl_value(self).map_err(PackError::Schema)?;
        Ok(pack_rt::encode_pack_value(&value).await)
    }
    async fn decode_pack_with(bytes: &[u8], _options: &PackDecodeOptions) -> Result<Self, PackError> {
        let value = pack_rt::decode_pack_value(bytes).await?;
        from_dsl_value(renormalize_whole_number_floats(value)).map_err(PackError::Schema)
    }
}
//#endregion SpaceHistoryDocument

//#region SpaceHost
/// @emoji 🏛️ Composes many `SpaceMember` documents under one space-wide checkpoint/alternative
/// timeline, itself stored in a dogfooded `S_SPACE_HISTORY_SCHEMA` (`"os.space.history"`)
/// meta-document. App-agnostic: this crate has no notion of what a member document *is*, only that
/// it satisfies `SpaceMember`.
pub struct SpaceHost<M = NoMembers> {
    meta: ArtifactStore<SpaceHistorySnapshot, SpaceHistoryMutation>,
    members: HashMap<String, M>,
}

impl<M: SpaceMember> SpaceHost<M> {
    pub async fn new(meta_envelope: ArtifactEnvelope<SpaceHistorySnapshot, SpaceHistoryMutation>) -> Result<Self, VcsError> {
        Ok(Self { meta: ArtifactStore::new(meta_envelope).await?, members: HashMap::new() })
    }

    pub async fn register_member(&mut self, member: M) {
        self.members.insert(member.document_id().await.to_string(), member);
    }

    /// @emoji 📚️ Batch counterpart to `register_member`: registers a space's manifest document, its
    /// collection documents, and any currently-open artifact documents together in one call, so the
    /// very next `commit_space_checkpoint` pins all of them atomically in the SAME space-wide
    /// checkpoint (see `🪐️space`'s `SpaceSnapshot`/`CollectionSnapshot`/document-artifact
    /// stores, W4's storage wiring — this crate stays app-agnostic and never names those types
    /// directly, only their common `SpaceMember` façade). Purely additive sugar over calling
    /// `register_member` three times in this order; no new mechanism.
    pub async fn register_space_documents(&mut self, manifest: M, collections: Vec<M>, artifacts: Vec<M>) {
        self.register_member(manifest).await;
        for collection in collections {
            self.register_member(collection).await;
        }
        for artifact in artifacts {
            self.register_member(artifact).await;
        }
    }

    pub async fn unregister_member(&mut self, document_id: &str) -> Option<M> {
        self.members.remove(document_id)
    }

    pub async fn member(&self, document_id: &str) -> Option<&M> {
        self.members.get(document_id)
    }

    pub async fn member_mut(&mut self, document_id: &str) -> Option<&mut M> {
        self.members.get_mut(document_id)
    }

    pub async fn meta_snapshot(&self) -> Result<SpaceHistorySnapshot, VcsError> {
        self.meta.snapshot().await
    }

    /// @emoji 🔗️ Attaches a backbone to the space-wide meta-document, same runtime-attach/detach
    /// contract as any other `ArtifactStore` — default is unattached, this is always an
    /// explicit call.
    pub async fn attach_backbone(&mut self, backbone: Backbones) -> Result<(), VcsError> {
        self.meta.attach_backbone(backbone).await
    }

    /// @emoji ✂️ Detaches the meta-document's backbone; the space history stays in memory.
    pub async fn detach_backbone(&mut self) -> Option<Backbones> {
        self.meta.detach_backbone().await
    }

    pub async fn backbone_ref(&self) -> Option<&ArtifactBackboneRef> {
        self.meta.backbone_ref().await
    }

    /// @emoji 📡️ Drains inbound backbone messages into the meta-document's edit timeline.
    pub async fn tick(&mut self) -> Result<bool, VcsError> {
        self.meta.tick().await
    }

    /// @emoji 💾️ Commits every dirty member (leaving clean members' existing checkpoints untouched),
    /// pins each member's resulting `(checkpoint, alternative)`, and records one `SpaceCheckpoint`
    /// on the meta-document — applied *and* committed there too, so the space history itself is
    /// durable the moment this returns.
    pub async fn commit_space_checkpoint(&mut self, message: String, authors: Vec<Author>) -> Result<String, VcsError> {
        let mut document_ids: Vec<String> = self.members.keys().cloned().collect();
        document_ids.sort();
        let mut pins = Vec::with_capacity(document_ids.len());
        for document_id in &document_ids {
            let member = self.members.get_mut(document_id).expect("just collected from members");
            if member.is_dirty().await {
                member.commit_checkpoint(message.clone(), authors.clone()).await?;
            }
            let checkpoint_id = member.current_checkpoint_id().await.ok_or(VcsError::NoCheckpoint)?;
            pins.push(SpaceMemberPin { document_id: document_id.clone(), checkpoint_id, alternative_id: member.current_alternative_id().await.unwrap_or_default() });
        }
        let pins_fingerprint = serde_json::to_vec(&pins).unwrap_or_default();
        let mut space_checkpoint_payload = message.as_bytes().to_vec();
        space_checkpoint_payload.push(0);
        space_checkpoint_payload.extend_from_slice(&pins_fingerprint);
        let checkpoint_id = content_addressed_entity_id("space-checkpoint", &space_checkpoint_payload).await;
        let parent_id = self.meta.snapshot().await?.checkpoints.last().map(|checkpoint| checkpoint.id.clone());
        let checkpoint = SpaceCheckpoint { id: checkpoint_id.clone(), parent_id, message: message.clone(), authors, timestamp: HybridLogicalTimestamp::new(0, now_ms().await).await, members: pins };
        // 🎯️ W6: the `Apply` below uses `dispatch_inner` (not `dispatch`), skipping its automatic
        // per-dispatch `flush_outbound` — the very next `CommitCheckpoint` dispatch flushes a full
        // snapshot that already includes this `Apply`'s edit, so a separate incremental flush here
        // would resend the same change twice. Before W5/W6's per-op wire envelopes this was
        // harmless (both flushes tagged the change with the same `edit.id`, so a receiver's
        // id-based dedup silently absorbed the duplicate); now that `Operations` messages carry
        // per-OP ids (distinct from the edit's own id — see `flush_outbound`), the two flushes are
        // no longer accidentally deduplicable, so avoiding the redundant one is the real fix.
        self.meta.dispatch_inner(ArtifactCommand::Apply { mutations: vec![SpaceHistoryMutation::CommitSpaceCheckpoint { checkpoint }], description: Some(message) }).await?;
        self.meta.dispatch(ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }).await?;
        Ok(checkpoint_id)
    }

    /// @emoji 🌿️ Records a `SpaceAlternative` pinned at the current space checkpoint tip (or none,
    /// if nothing has been committed yet), so it can later be switched back into.
    pub async fn create_space_alternative(&mut self, name: String) -> Result<String, VcsError> {
        let checkpoint_ids: Vec<String> = self.meta.snapshot().await?.checkpoints.last().map(|checkpoint| checkpoint.id.clone()).into_iter().collect();
        let mut space_alternative_payload = name.as_bytes().to_vec();
        space_alternative_payload.push(0);
        space_alternative_payload.extend_from_slice(checkpoint_ids.join("\0").as_bytes());
        let alternative_id = content_addressed_entity_id("space-alternative", &space_alternative_payload).await;
        let alternative = SpaceAlternative { id: alternative_id.clone(), name, checkpoint_ids };
        self.meta.dispatch(ArtifactCommand::Apply { mutations: vec![SpaceHistoryMutation::CreateSpaceAlternative { alternative }], description: None }).await?;
        Ok(alternative_id)
    }

    /// @emoji 🔀️ Fans out to every member pinned by `checkpoint_id`'s `SpaceCheckpoint`, restoring
    /// each to its exact recorded `(checkpoint, alternative)`.
    pub async fn checkout_space_checkpoint(&mut self, checkpoint_id: &str) -> Result<(), VcsError> {
        let snapshot = self.meta.snapshot().await?;
        let checkpoint = snapshot.checkpoints.iter().find(|checkpoint| checkpoint.id == checkpoint_id).ok_or(VcsError::NoCheckpoint)?;
        for pin in &checkpoint.members {
            if let Some(member) = self.members.get_mut(&pin.document_id) {
                member.checkout(&pin.checkpoint_id, &pin.alternative_id).await?;
            }
        }
        Ok(())
    }

    /// @emoji 🔀️ Switches the studio's active alternative and fans out to its tip checkpoint's pins.
    pub async fn switch_space_alternative(&mut self, alternative_id: &str) -> Result<(), VcsError> {
        let snapshot = self.meta.snapshot().await?;
        let alternative = snapshot.alternatives.iter().find(|alternative| alternative.id == alternative_id).ok_or_else(|| VcsError::UnknownAlternative(alternative_id.to_string()))?;
        let checkpoint_id = alternative.checkpoint_ids.last().cloned().ok_or(VcsError::NoCheckpoint)?;
        self.meta.dispatch(ArtifactCommand::Apply { mutations: vec![SpaceHistoryMutation::SwitchSpaceAlternative { alternative_id: alternative_id.to_string() }], description: None }).await?;
        self.checkout_space_checkpoint(&checkpoint_id).await
    }

    /// @emoji ↩️ Derived, local-only undo: targets whichever registered member has the most recent
    /// `last_local_edit_timestamp` (by {@link HybridLogicalTimestamp::cmp_key}) and undoes just that
    /// member. Never dispatched against the meta-document — space-level undo has no `SpaceHistoryMutation`
    /// of its own, it is purely a cross-member ordering policy.
    pub async fn undo(&mut self) -> Result<(), VcsError> {
        let mut candidates = Vec::new();
        for (document_id, member) in self.members.iter() {
            if let Some(timestamp) = member.last_local_edit_timestamp().await {
                candidates.push((timestamp.cmp_key().await, document_id.clone()));
            }
        }
        let target = candidates.into_iter().max_by_key(|(cmp_key, _)| *cmp_key).map(|(_, document_id)| document_id);
        let document_id = target.ok_or(VcsError::NothingToUndo)?;
        self.members.get_mut(&document_id).ok_or(VcsError::NothingToUndo)?.undo().await
    }

    /// @emoji ↪️ Derived, local-only redo: mirrors `undo`, targeting the member with the most
    /// recent `last_undone_local_edit_timestamp`.
    pub async fn redo(&mut self) -> Result<(), VcsError> {
        let mut candidates = Vec::new();
        for (document_id, member) in self.members.iter() {
            if let Some(timestamp) = member.last_undone_local_edit_timestamp().await {
                candidates.push((timestamp.cmp_key().await, document_id.clone()));
            }
        }
        let target = candidates.into_iter().max_by_key(|(cmp_key, _)| *cmp_key).map(|(_, document_id)| document_id);
        let document_id = target.ok_or(VcsError::NothingToRedo)?;
        self.members.get_mut(&document_id).ok_or(VcsError::NothingToRedo)?.redo().await
    }
}
//#endregion SpaceHost
//#endregion 🔖️Space

//#region 🔖️CompositionCoordinator
// 🧩️ Atomic composite dispatch across a parent `SpaceMember` and N child `SpaceMember`s — the
// mechanism that makes ONE user gesture spanning a parent and several `🔖️Composition` children
// into ONE undo step, by orchestrating multi-store dispatch through the object-safe
// `SpaceMember::preview_wire`/`dispatch_wire`/`tail_group_id`/`merge_policy`/etc. seam (extended
// just above, `🔖️Space`'s `SpaceMember` region) rather than needing to know any member's concrete
// `P`/`Mutation`. Two-phase: `dispatch_group`'s phase 1 dry-runs every op on every member against
// its CURRENT snapshot with zero side effects, unions the resulting `MutationMessage`s (each
// prefixed with its originating member's path — `prefix_message_target`) into one set, and rejects
// the WHOLE group (`reject_if_policy_rejects`, ALL-OR-NOTHING — no member applies anything) if the
// parent-or-initiator's own `merge_policy()` rejects the union's worst `crate::os_dsl::Severity`
// (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6); a structural failure
// (unknown ownership, a graph cycle, a missing `ChildStoreFactory`) still rejects immediately,
// exactly as before. Phase 2 applies in the fixed order child geneses → child edits → parent ops,
// stamping every resulting edit's `MutationMeta.group_id` with the same minted `invocation_id` —
// the shared stamp a later `undo_group`/`redo_group` call recognizes — and the SAME unioned
// messages phase 1 computed travel through unchanged as `GroupReceipt.messages`. `CompositionGraph`
// tracks the ownership forest/link DAG this all leans on for cycle/ownership validation.

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
///
/// 🎯️ `messages` (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6): the
/// SAME union of every dispatched member's `preview_wire` output phase 1 already computed to decide
/// accept/reject — carried through unchanged rather than recomputed, so a successful receipt's
/// messages are byte-identical to what a `preview_wire` dry run of the same ops would have reported
/// (§C2 LAW 3, determinism). Each message's `target` is prefixed with the ORIGINATING member's own
/// `crate::os_io::ArtifactRef::to_uri()` (outermost segment), so a caller with several members in
/// flight can always tell which member a given message came from — the same discipline §C4 mandates
/// for composite step paths. Empty when every op's outcome was silent.
pub struct GroupReceipt<M> {
    pub invocation_id: String,
    pub member_edits: Vec<(crate::os_io::ArtifactRef, String)>,
    pub created_children: Vec<(crate::os_io::ArtifactRef, M)>,
    pub messages: Vec<crate::os_spr::MutationMessage>,
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
    pub async fn new() -> Self {
        Self::default()
    }

    /// 🔎️ The owning parent's artifact id, if `child_id` is currently tracked as owned.
    pub async fn owner_of(&self, child_id: &str) -> Option<&str> {
        self.owns.get(child_id).map(|(parent_id, _slot)| parent_id.as_str())
    }

    /// 🔎️ The slot `child_id` currently occupies under its owner, if tracked.
    pub async fn slot_of(&self, child_id: &str) -> Option<&str> {
        self.owns.get(child_id).map(|(_parent_id, slot)| slot.as_str())
    }

    /// ✅️ Whether owning `child_id` under `parent_id` would create a cycle — walks `parent_id`'s
    /// OWN ancestor chain looking for `child_id`; finding it means `child_id` would become both an
    /// ancestor and a (prospective) descendant of `parent_id`. Also true when `parent_id ==
    /// child_id` (an artifact cannot own itself).
    pub async fn would_cycle_owns(&self, parent_id: &str, child_id: &str) -> bool {
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
    pub async fn insert_owns(&mut self, parent_id: &str, slot: &str, child_id: &str) -> Result<(), String> {
        if let Some((existing_owner, _existing_slot)) = self.owns.get(child_id) {
            if existing_owner != parent_id {
                return Err(format!("{child_id} is already owned by {existing_owner}, cannot also be owned by {parent_id}"));
            }
        }
        if self.would_cycle_owns(parent_id, child_id).await {
            return Err(format!("owning {child_id} under {parent_id} would create a composition cycle"));
        }
        self.owns.insert(child_id.to_string(), (parent_id.to_string(), slot.to_string()));
        Ok(())
    }

    /// ✂️ Removes `child_id`'s ownership edge (e.g. on `extract`/`delete`), returning
    /// `(parent_id, slot)` if it was tracked.
    pub async fn remove_owns(&mut self, child_id: &str) -> Option<(String, String)> {
        self.owns.remove(child_id)
    }

    /// ✅️ Whether linking `from -> to` would create a cycle — true when `to` can already reach
    /// `from` (including `from == to`).
    pub async fn would_cycle_links(&self, from: &str, to: &str) -> bool {
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
    pub async fn insert_link(&mut self, from: &str, to: &str) -> Result<(), String> {
        if self.would_cycle_links(from, to).await {
            return Err(format!("linking {from} -> {to} would create a cycle"));
        }
        self.links.entry(from.to_string()).or_default().insert(to.to_string());
        Ok(())
    }

    /// ✂️ Removes one `from -> to` link edge, if present.
    pub async fn remove_link(&mut self, from: &str, to: &str) {
        if let Some(targets) = self.links.get_mut(from) {
            targets.remove(to);
        }
    }

    /// 🔎️ Every link target currently recorded FROM `from`.
    pub async fn links_from(&self, from: &str) -> Vec<String> {
        self.links.get(from).map(|targets| targets.iter().cloned().collect()).unwrap_or_default()
    }

    /// 🔄️ Rebuilds `artifact_id`'s OWN outgoing edges (both `Owns`-as-parent and `Links`-as-source)
    /// from its live `ArtifactRefs` projection — the incremental-maintenance seam a host (e.g.
    /// `SpaceHost`) calls after every dispatch that might have changed `artifact_id`'s
    /// children/links, instead of ever recomputing the whole graph from scratch. Never touches
    /// edges where `artifact_id` is the TARGET (another artifact's own `sync_member` call owns those).
    pub async fn sync_member<P: ArtifactRefs>(&mut self, artifact_id: &str, snapshot: &P) -> Result<(), String> {
        self.owns.retain(|_child_id, (parent_id, _slot)| parent_id != artifact_id);
        for child in snapshot.child_refs().await {
            self.insert_owns(artifact_id, &child.slot, &child.child_id).await?;
        }
        self.links.remove(artifact_id);
        for link in snapshot.links().await {
            self.insert_link(artifact_id, &link.target.artifact_id).await?;
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
async fn build_apply_command_bytes(ops: &[Vec<u8>], description: Option<&str>) -> Vec<u8> {
    let mut out = vec![COMMAND_BINARY_FORMAT];
    crate::os_pack::write_varint_u64(&mut out, 0).await;
    out.push(if description.is_some() { 0b01 } else { 0 });
    if let Some(text) = description {
        write_command_str(&mut out, text).await;
    }
    crate::os_pack::write_varint_u64(&mut out, ops.len() as u64).await;
    for op in ops {
        crate::os_pack::write_varint_u64(&mut out, op.len() as u64).await;
        out.extend_from_slice(op);
    }
    out
}

/// @emoji 🧮️ Deterministic order-and-length-sensitive fingerprint of a raw op-bytes slice — the
/// `parent_edit_fingerprint` ingredient `mint_child_id`/`mint_invocation_id` hash into a new id, so
/// two replicas that receive the identical `parent_ops`/`ChildDispatch.ops` converge on identical
/// ids without ever needing to actually apply anything first.
async fn concat_ops_fingerprint(ops: &[Vec<u8>]) -> Vec<u8> {
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
pub async fn mint_child_id(parent_id: &str, slot: &str, parent_edit_fingerprint: &[u8], ordinal: u32) -> String {
    let mut payload = parent_id.as_bytes().to_vec();
    payload.push(0);
    payload.extend_from_slice(slot.as_bytes());
    payload.push(0);
    payload.extend_from_slice(parent_edit_fingerprint);
    payload.push(0);
    payload.extend_from_slice(&ordinal.to_le_bytes());
    content_addressed_entity_id("child", &payload).await
}

/// @emoji 🆔️ Deterministic group/invocation id: hashes the parent id, the parent ops' fingerprint,
/// and every dispatched child's `(child_id, ops fingerprint)` pair (sorted by child id first, so
/// caller-supplied `children` order never affects convergence) — two replicas performing the
/// identical composite gesture (same parent, same ops everywhere) converge on the identical
/// `GroupReceipt.invocation_id`/`MutationMeta.group_id` stamp without any coordination.
async fn mint_invocation_id(parent_id: &str, parent_edit_fingerprint: &[u8], child_fingerprints: &[(String, Vec<u8>)]) -> String {
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
    content_addressed_entity_id("invocation", &payload).await
}

/// @emoji 🏷️ Prepends `member_path` (that member's own `crate::os_io::ArtifactRef::to_uri()`) as the
/// OUTERMOST segment of every message's `target` — the discipline §C4 mandates for composite step
/// paths, generalized to a composition group: a caller unioning messages from several members must
/// still be able to tell which member produced which message.
async fn prefix_message_target(messages: Vec<crate::os_spr::MutationMessage>, member_path: &str) -> Vec<crate::os_spr::MutationMessage> {
    messages
        .into_iter()
        .map(|mut message| {
            message.target.insert(0, member_path.to_string());
            message
        })
        .collect()
}

/// @emoji ⚖️ Phase 1's policy gate (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-
/// CONFLICTS` §C6): folds every already-prefixed message `dispatch_relation_group` collected via
/// `SpaceMember::preview_wire` into ONE worst `crate::os_dsl::Severity` and consults `policy` — the
/// parent-or-initiator's own `SpaceMember::merge_policy()`, since it is the member driving/owning
/// this atomic gesture (a policy is authority-local state per §C3, so SOME single member's policy
/// must govern one shared accept/reject decision; the entry point is the natural, and only
/// unambiguous, choice among a group that may otherwise mix policies). `Ok(())` when `messages` is
/// empty or `policy` accepts the worst level; `Err(VcsError::Rejected { policy, messages })`
/// otherwise — phase 2 never runs, so nothing has been dispatched anywhere yet (the same "zero side
/// effects" law every other phase-1 failure upholds).
///
/// 🎯️ This is the GROUP-level gate only. Each member's own real `dispatch_wire` (Phase 2) ALSO
/// independently enforces THAT member's own `merge_policy()` inside `ArtifactStore::dispatch`
/// (lane 1-A's C6 `replay_mutations`) — a member with a stricter policy than the parent's can still
/// reject its own op during Phase 2 even after this gate accepted the union under the parent's more
/// lenient policy. Both checks return the SAME `VcsError::Rejected` shape; a caller relying on
/// "accepted by the parent's policy" alone must also configure every other participating member's
/// own policy consistently if it wants Phase 2 to actually complete (see this file's own
/// `dispatch_group_phase1_accepts_the_same_error_scenario_under_laissez_faire` test, which sets
/// `LaissezFaire` on BOTH members for exactly this reason).
async fn reject_if_policy_rejects(policy: crate::os_spr::MergePolicy, messages: &[crate::os_spr::MutationMessage]) -> Result<(), VcsError> {
    let Some(worst) = crate::os_spr::worst_level(messages).await else {
        return Ok(());
    };
    if !policy.rejects(worst).await {
        return Ok(());
    }
    Err(VcsError::Rejected { policy, messages: messages.to_vec() })
}

/// @emoji 🧯️ Folds a post-validation dispatch failure with its compensation report: if every
/// already-applied member rolled back cleanly (`report.skipped` empty), the ORIGINAL error is
/// returned unchanged (compensation is a transparent implementation detail on the success path);
/// otherwise wraps both into `VcsError::CompensationFailed` so the caller sees the full picture —
/// what failed AND what could not be rolled back — rather than either fact silently.
async fn fold_compensation_error(original: VcsError, report: GroupUndoReport) -> VcsError {
    if report.skipped.is_empty() {
        original
    } else {
        // 🌀️ `ArtifactRef::to_uri` is async (🚪️io, out of scope) — hoisted out of `Iterator::map`'s
        // sync closure into an explicit loop (R10 shape 1).
        let mut skipped_desc: Vec<String> = Vec::with_capacity(report.skipped.len());
        for (reference, error) in &report.skipped {
            skipped_desc.push(format!("{}: {error}", reference.to_uri().await));
        }
        VcsError::CompensationFailed(format!("original error: {original}; members that failed to roll back: [{}]", skipped_desc.join(", ")))
    }
}

/// @emoji 🔀️ Which structural relationship holds between a `TransactionCoordinator::dispatch_group`
/// (or `dispatch_peer_group`) call's `parent`/initiator and its `children`/peers —
/// `PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS` W1-C's generalization of
/// the pre-existing composition machinery (contract-freeze §5, `📓️scout-2-group-undo-and-hosts.md`
/// §2). `Owned` is the pre-existing, byte-identical-unchanged behaviour: `CompositionGraph::
/// owner_of` is checked and `ChildGenesis` is allowed. `Peer` is new: every member is an equal
/// party to a cross-artifact transaction with NO ownership relation — no `owner_of` check, no
/// genesis — but the SAME cycle guard (`CompositionGraph::would_cycle_links` in place of
/// `would_cycle_owns`, persisting a `Links` edge per successfully-dispatched peer so a LATER
/// transaction that would close a cycle across separate calls is also caught), the same shared
/// `invocation_id`, the same one-edit-per-member rule, and the same reverse-order compensation.
/// Every peer's tail edit additionally gets `MutationMeta.origin` stamped
/// `crate::os_spr::MutationOrigin::Transaction { initiator }` (`SpaceMember::stamp_tail_origin`) —
/// the initiator's own edit is left at its ordinary `Owner` default, since it is not foreign to
/// itself. `Owned` never stamps `origin` at all, which is exactly what keeps it byte-identical.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemberRelation {
    Owned,
    Peer,
}

/// @emoji 🧩️ Atomic composite/transactional dispatch across a parent-or-initiator + N children-
/// or-peers — see this region's doc comment for the two-phase protocol and `MemberRelation` for
/// what differs between `Owned` (`dispatch_group`) and `Peer` (`dispatch_peer_group`). Holds a
/// `CompositionGraph` incrementally maintained across calls (`graph`/`graph_mut` for a host to
/// `sync_member` into, or to consult directly for UI-level "would this cycle" checks without
/// dispatching anything).
///
/// Named `TransactionCoordinator` as of W1-C — `CompositionCoordinator` just below is kept as an
/// exact type alias (same type, not a wrapper) so every existing call site across the tree,
/// including out-of-lease files under concurrent edit by sibling W1 lanes, keeps compiling and
/// behaving identically unchanged. Prefer the new name in code this ticket authors.
#[derive(Debug, Default)]
pub struct TransactionCoordinator {
    graph: CompositionGraph,
}

/// @emoji 🪪️ Source-compatible alias — see `TransactionCoordinator`'s own doc comment for why this
/// exists rather than a rename-in-place.
pub type CompositionCoordinator = TransactionCoordinator;

impl TransactionCoordinator {
    pub async fn new() -> Self {
        Self::default()
    }

    pub async fn graph(&self) -> &CompositionGraph {
        &self.graph
    }

    pub async fn graph_mut(&mut self) -> &mut CompositionGraph {
        &mut self.graph
    }

    /// ✂️ Undoes `undo`/re-dispatches `undo` on the applied-so-far members in REVERSE application
    /// order (parent — if it was itself applied — first, then children in reverse dispatch order),
    /// collecting a `GroupUndoReport` rather than propagating the first failure — see
    /// `dispatch_group`'s doc comment for why every already-applied member must still get a
    /// best-effort rollback attempt even if an earlier one in this same pass failed. Relation-
    /// agnostic: undoing a member does not need to know WHY it was dispatched.
    async fn compensate<M: SpaceMember>(parent_ref: &crate::os_io::ArtifactRef, parent: &mut M, children: &mut [(&mut M, ChildDispatch)], applied_children: &[(usize, String)], parent_applied: Option<&str>) -> GroupUndoReport {
        let mut undone = Vec::new();
        let mut skipped = Vec::new();
        if let Some(edit_id) = parent_applied {
            match parent.undo().await {
                Ok(()) => undone.push((parent_ref.clone(), edit_id.to_string())),
                Err(error) => skipped.push((parent_ref.clone(), error)),
            }
        }
        for (index, edit_id) in applied_children.iter().rev() {
            let (member, dispatch) = &mut children[*index];
            match member.undo().await {
                Ok(()) => undone.push((dispatch.child.clone(), edit_id.clone())),
                Err(error) => skipped.push((dispatch.child.clone(), error)),
            }
        }
        GroupUndoReport { undone, skipped }
    }

    /// 🧩️ `Owned`-relation dispatch: `parent` truly owns every `children` entry (checked against
    /// `self.graph`) and may create brand-new `genesis` children. See `dispatch_relation_group` for
    /// the shared two-phase/compensation engine this delegates to unchanged — this method's own
    /// behaviour is byte-identical to before `MemberRelation` existed.
    pub async fn dispatch_group<M: SpaceMember + MemberFactory>(
        &mut self,
        parent_ref: &crate::os_io::ArtifactRef,
        parent: &mut M,
        children: &mut [(&mut M, ChildDispatch)],
        parent_ops: Vec<Vec<u8>>,
        genesis: Vec<ChildGenesis>,
        meta: GroupMeta,
    ) -> Result<GroupReceipt<M>, VcsError> {
        self.dispatch_relation_group(MemberRelation::Owned, parent_ref, parent, children, parent_ops, genesis, meta).await
    }

    /// 🤝️ `Peer`-relation dispatch (contract-freeze §5): `initiator_ref`/`initiator` is the
    /// transaction's member #0, `peers` are every OTHER member with NO ownership relation to it.
    /// No `genesis` parameter — a peer transaction never creates a child, only touches existing
    /// artifacts. See `MemberRelation::Peer`'s doc comment for exactly what differs from
    /// `dispatch_group`, and `dispatch_relation_group` for the shared engine.
    pub async fn dispatch_peer_group<M: SpaceMember + MemberFactory>(
        &mut self,
        initiator_ref: &crate::os_io::ArtifactRef,
        initiator: &mut M,
        peers: &mut [(&mut M, ChildDispatch)],
        initiator_ops: Vec<Vec<u8>>,
        meta: GroupMeta,
    ) -> Result<GroupReceipt<M>, VcsError> {
        self.dispatch_relation_group(MemberRelation::Peer, initiator_ref, initiator, peers, initiator_ops, Vec::new(), meta).await
    }

    /// 🧩️ Dispatches one composite/transactional gesture spanning `parent` + `children` (+ any
    /// brand-new `genesis` children, `Owned` only) as a single atomic unit. Shared by
    /// `dispatch_group`/`dispatch_peer_group` — `relation` is the ONLY thing that changes phase-1's
    /// per-child check and whether phase-2 stamps `MutationOrigin::Transaction`; everything else
    /// (order, `invocation_id` minting, `group_id` stamping, compensation) is identical, which is
    /// what "generalize, don't replace" means concretely in this method.
    ///
    /// **Phase 1 — preview-all, zero side effects.** Every non-empty op slice (`parent_ops`, each
    /// `ChildDispatch.ops`) is dry-run via `SpaceMember::preview_wire` against that member's CURRENT
    /// snapshot and its messages unioned into one `all_messages` list, each stamped with its
    /// originating member's own `crate::os_io::ArtifactRef::to_uri()` as the outermost `target`
    /// segment (`prefix_message_target`). Under `Owned`, every `children` entry's claimed ownership
    /// is ALSO checked against `self.graph` (`VcsError::OwnershipViolation` if the graph does not
    /// currently track `parent_ref` as that child's owner) — a STRUCTURAL failure, returned
    /// immediately, never folded into `all_messages`. Under `Peer`, no ownership is checked; instead
    /// `self.graph.would_cycle_links(parent_ref, child)` guards against a peer transaction that
    /// would close a link cycle (`VcsError::CompositionCycle`), including the trivial self-cycle of
    /// an artifact transacting with itself. Every `genesis` slot's (Owned-only) deterministic id is
    /// minted and checked for a cycle (`VcsError::CompositionCycle`) and a registered
    /// `ChildStoreFactory` (`VcsError::ValidationFailed` if none) — also structural, also immediate.
    /// Once every member's ops have been previewed and every structural check has passed,
    /// `reject_if_policy_rejects(parent.merge_policy(), &all_messages)` computes ONE worst
    /// `crate::os_dsl::Severity` across the whole union and consults the parent-or-initiator's own
    /// `merge_policy()` (§C6: a `MergePolicy` is authority-local state, so ONE member's policy must
    /// govern the shared group decision — the entry point is the natural choice), returning
    /// `VcsError::Rejected { policy, messages }` on rejection. A reject here is ALL-OR-NOTHING
    /// exactly like every other phase-1 failure: NOTHING has been dispatched anywhere yet, for ANY
    /// member, no matter which member's ops raised the worst message. This is a GROUP-level gate
    /// only — Phase 2 below still runs each member's real `dispatch_wire`, which independently
    /// enforces THAT member's own `merge_policy()` too (lane 1-A's C6 `ArtifactStore::dispatch`), so
    /// a caller must configure every participating member's policy consistently to guarantee Phase 2
    /// completes even after this gate accepts.
    ///
    /// **Phase 2 — apply in fixed order: child geneses → child edits → parent ops.** This order
    /// guarantees a parent's own `Apply` (which typically ADDS the `ArtifactChild` handle pointing
    /// at a just-created genesis child) never references a child that does not exist locally yet.
    /// Every member that receives an `Apply` gets its tail edit's `MutationMeta.group_id` stamped
    /// with the same minted `invocation_id` right after dispatching it. Under `Peer`, each
    /// dispatched child ALSO gets `MutationMeta.origin` stamped `MutationOrigin::Transaction {
    /// initiator: parent_ref }` and a `self.graph.insert_link(parent_ref, child)` edge recorded (so
    /// a later, separate transaction's cycle guard sees it too).
    ///
    /// **Compensation.** A failure during phase 2 (a `dispatch_wire`/`stamp_tail_group_id`/
    /// `stamp_tail_origin` call that phase 1's validation did not catch — e.g. a `Mutation::
    /// validate` that is not fully exhaustive, or a genuinely unexpected `VcsError`) triggers
    /// `compensate`: `Undo` on every already-applied member in reverse order. This is sound under
    /// `&mut` on every member for the whole call (single-threaded per-app actor discipline) because
    /// each such member's group edit IS its tail — exact-base undo is mechanical, never a
    /// mid-history removal. If compensation itself fails to fully roll back, the returned error is
    /// `VcsError::CompensationFailed` (`fold_compensation_error`) carrying both the original
    /// failure and which members could not be rolled back, rather than silently leaving partial
    /// state unreported.
    async fn dispatch_relation_group<M: SpaceMember + MemberFactory>(
        &mut self,
        relation: MemberRelation,
        parent_ref: &crate::os_io::ArtifactRef,
        parent: &mut M,
        children: &mut [(&mut M, ChildDispatch)],
        parent_ops: Vec<Vec<u8>>,
        genesis: Vec<ChildGenesis>,
        meta: GroupMeta,
    ) -> Result<GroupReceipt<M>, VcsError> {
        //#region Phase1Validate
        let mut all_messages: Vec<crate::os_spr::MutationMessage> = Vec::new();
        if !parent_ops.is_empty() {
            all_messages.extend(prefix_message_target(parent.preview_wire(&parent_ops).await, &parent_ref.to_uri().await).await);
        }
        for (member, dispatch) in children.iter() {
            match relation {
                MemberRelation::Owned => match self.graph.owner_of(&dispatch.child.artifact_id).await {
                    Some(owner_id) if owner_id == parent_ref.artifact_id => {}
                    _ => return Err(VcsError::OwnershipViolation(format!("{} is not a currently-tracked owned child of {}", dispatch.child.artifact_id, parent_ref.artifact_id))),
                },
                MemberRelation::Peer => {
                    if self.graph.would_cycle_links(&parent_ref.artifact_id, &dispatch.child.artifact_id).await {
                        return Err(VcsError::CompositionCycle(format!("transacting {} with {} would create a peer cycle", parent_ref.artifact_id, dispatch.child.artifact_id)));
                    }
                }
            }
            if !dispatch.ops.is_empty() {
                all_messages.extend(prefix_message_target(member.preview_wire(&dispatch.ops).await, &dispatch.child.to_uri().await).await);
            }
        }
        let parent_edit_fingerprint = concat_ops_fingerprint(&parent_ops).await;
        let mut minted_child_ids: Vec<String> = Vec::with_capacity(genesis.len());
        for (ordinal, spec) in genesis.iter().enumerate() {
            let child_id = mint_child_id(&parent_ref.artifact_id, &spec.slot, &parent_edit_fingerprint, ordinal as u32).await;
            if self.graph.would_cycle_owns(&parent_ref.artifact_id, &child_id).await {
                return Err(VcsError::CompositionCycle(format!("creating child {child_id} in slot {} under {} would cycle", spec.slot, parent_ref.artifact_id)));
            }
            minted_child_ids.push(child_id);
        }
        let group_policy = parent.merge_policy().await;
        reject_if_policy_rejects(group_policy, &all_messages).await?;
        //#endregion Phase1Validate

        //#region Phase2Apply
        // 🌀️ `concat_ops_fingerprint` is async; `Iterator::map`'s closure is sync (R10 shape 1),
        // so it's hoisted into an explicit loop.
        let mut child_fingerprints: Vec<(String, Vec<u8>)> = Vec::with_capacity(children.len());
        for (_, dispatch) in children.iter() {
            child_fingerprints.push((dispatch.child.artifact_id.clone(), concat_ops_fingerprint(&dispatch.ops).await));
        }
        let invocation_id = mint_invocation_id(&parent_ref.artifact_id, &parent_edit_fingerprint, &child_fingerprints).await;

        // 🎯️ O1: no more `ChildStoreFactory` global registry lookup — `M::create` (the
        // `space_members!`-generated `MemberFactory` impl) matches `kind` against M's OWN closed
        // variant set directly. Nothing to compensate on a genesis failure: no `dispatch_wire` has
        // run yet in this call, and any earlier-succeeding genesis member in this same loop was
        // never registered/dispatched to anywhere — it simply gets dropped along with this `Err`.
        let mut created_children: Vec<(crate::os_io::ArtifactRef, M)> = Vec::with_capacity(genesis.len());
        for (ordinal, spec) in genesis.into_iter().enumerate() {
            let child_id = minted_child_ids[ordinal].clone();
            let mut member = M::create(&spec.dialect.artifact_kind, &child_id, &spec.dialect, &spec.initial_pack).await?;
            let target = crate::os_io::ArtifactRef { artifact_id: child_id.clone(), dialect: spec.dialect.clone() };
            member.set_owner(Some(OwnerRef { parent: parent_ref.clone(), slot: spec.slot.clone(), child_id: child_id.clone() })).await;
            self.graph.insert_owns(&parent_ref.artifact_id, &spec.slot, &child_id).await.map_err(VcsError::OwnershipViolation)?;
            created_children.push((target, member));
        }

        let mut applied_children: Vec<(usize, String)> = Vec::new();
        for index in 0..children.len() {
            if children[index].1.ops.is_empty() {
                continue;
            }
            let command_bytes = build_apply_command_bytes(&children[index].1.ops, meta.description.as_deref()).await;
            let receipt = match children[index].0.dispatch_wire_with_policy(&command_bytes, group_policy).await {
                Ok(receipt) => receipt,
                Err(error) => {
                    let report = Self::compensate(parent_ref, parent, children, &applied_children, None).await;
                    return Err(fold_compensation_error(error, report).await);
                }
            };
            let edit_id = receipt.edit_ids.last().cloned().unwrap_or_default();
            applied_children.push((index, edit_id));
            if let Err(error) = children[index].0.stamp_tail_group_id(&invocation_id).await {
                let report = Self::compensate(parent_ref, parent, children, &applied_children, None).await;
                return Err(fold_compensation_error(error, report).await);
            }
            if relation == MemberRelation::Peer {
                let initiator = crate::os_spr::ForeignTarget { artifact_id: parent_ref.artifact_id.clone(), artifact_kind: parent_ref.dialect.artifact_kind.clone(), dialect: Some(parent_ref.dialect.to_coordinate().await) };
                if let Err(error) = children[index].0.stamp_tail_origin(crate::os_spr::MutationOrigin::Transaction { initiator }).await {
                    let report = Self::compensate(parent_ref, parent, children, &applied_children, None).await;
                    return Err(fold_compensation_error(error, report).await);
                }
                // 🔗️ Best-effort: `would_cycle_links` already cleared this exact edge in phase 1, so
                // this can only fail if a CONCURRENT graph mutation raced us between the two — not
                // worth failing an already-applied transaction over; the cycle guard still holds for
                // every edge that DID get recorded.
                let _ = self.graph.insert_link(&parent_ref.artifact_id, &children[index].1.child.artifact_id).await;
            }
        }

        let mut parent_edit_id: Option<String> = None;
        if !parent_ops.is_empty() {
            let command_bytes = build_apply_command_bytes(&parent_ops, meta.description.as_deref()).await;
            let receipt = match parent.dispatch_wire_with_policy(&command_bytes, group_policy).await {
                Ok(receipt) => receipt,
                Err(error) => {
                    let report = Self::compensate(parent_ref, parent, children, &applied_children, None).await;
                    return Err(fold_compensation_error(error, report).await);
                }
            };
            let edit_id = receipt.edit_ids.last().cloned().unwrap_or_default();
            if let Err(error) = parent.stamp_tail_group_id(&invocation_id).await {
                let report = Self::compensate(parent_ref, parent, children, &applied_children, Some(&edit_id)).await;
                return Err(fold_compensation_error(error, report).await);
            }
            parent_edit_id = Some(edit_id);
        }
        //#endregion Phase2Apply

        let mut member_edits: Vec<(crate::os_io::ArtifactRef, String)> = applied_children.iter().map(|(index, edit_id)| (children[*index].1.child.clone(), edit_id.clone())).collect();
        if let Some(edit_id) = parent_edit_id {
            member_edits.push((parent_ref.clone(), edit_id));
        }
        Ok(GroupReceipt { invocation_id, member_edits, created_children, messages: all_messages })
    }

    /// ↩️ Best-effort group undo: for every `(reference, member)` pair (caller-ordered — put the
    /// parent/initiator first, matching `dispatch_group`/`dispatch_peer_group`'s own "undo
    /// parent-first then children" fixed order), undoes it if and only if `member.tail_group_id()
    /// == Some(group_id)`; a member whose tail belongs to a different (or no) group, or whose own
    /// `undo()` call errors, is SKIPPED and recorded in the returned report rather than aborting the
    /// rest — see `GroupUndoReport`'s doc comment for why abort-all would be actively harmful here.
    /// Relation-agnostic by construction: it never consults `self.graph`/ownership at all, only
    /// each member's own `tail_group_id()`, so a `Peer`-relation group reverses as one exactly the
    /// same way an `Owned`-relation group does — no separate code path needed.
    pub async fn undo_group<M: SpaceMember>(members: &mut [(&crate::os_io::ArtifactRef, &mut M)], group_id: &str) -> GroupUndoReport {
        let mut undone = Vec::new();
        let mut skipped = Vec::new();
        for (reference, member) in members.iter_mut() {
            let tail_group = member.tail_group_id().await;
            if tail_group.as_deref() == Some(group_id) {
                let edit_id = member.tail_edit_id().await.unwrap_or_default();
                match member.undo().await {
                    Ok(()) => undone.push(((*reference).clone(), edit_id)),
                    Err(error) => skipped.push(((*reference).clone(), error)),
                }
            } else {
                skipped.push(((*reference).clone(), VcsError::ForeignEdit(member.document_id().await.to_string())));
            }
        }
        GroupUndoReport { undone, skipped }
    }

    /// ↪️ `undo_group`'s redo-direction mirror: caller orders `members` children-first (matching
    /// `dispatch_group`/`dispatch_peer_group`'s apply order, so redo re-establishes the group in
    /// the same order it was originally applied), redoing each member whose `redo_tail()` group id
    /// matches. Relation-agnostic for the same reason `undo_group` is.
    pub async fn redo_group<M: SpaceMember>(members: &mut [(&crate::os_io::ArtifactRef, &mut M)], group_id: &str) -> GroupUndoReport {
        let mut undone = Vec::new();
        let mut skipped = Vec::new();
        for (reference, member) in members.iter_mut() {
            match member.redo_tail().await {
                Some((edit_id, Some(tail_group))) if tail_group == group_id => match member.redo().await {
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
    pub async fn assert_operation_round_trip<P, Mutation>(pre: &P, operation: Mutation)
    where
        P: Clone + PartialEq + std::fmt::Debug,
        Mutation: self::Mutation<P>,
    {
        let post = apply_mutation(pre, &operation).await.expect("test operation diff applies").0;
        let mut inverse = operation.inverse(pre).await;
        inverse.reverse();
        let mut restored = post;
        for back_operation in &inverse {
            restored = apply_mutation(&restored, back_operation).await.expect("test inverse diff applies").0;
        }
        assert_eq!(&restored, pre, "operation inverse did not restore pre-state");
    }

    /// @emoji 🗄️ Asserts a full store round trip: Apply→Undo restores `initial`, Redo restores the
    /// post-apply snapshot, and replay-materialization agrees with the live store snapshot.
    pub async fn assert_store_roundtrip<P, Mutation>(initial: P, operation: Mutation)
    where
        P: Clone + Serialize + DeserializeOwned + ArtifactPack + PartialEq + std::fmt::Debug,
        Mutation: Clone + Serialize + DeserializeOwned + self::Mutation<P> + OpBinary + OpText,
    {
        let envelope = create_document_envelope("test/v1", "test", initial.clone(), None);
        let mut store = ArtifactStore::new(envelope.await).await.expect("test support store construction");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![operation], description: None }).await.expect("apply");
        let post = store.snapshot().await.expect("post snapshot");
        store.dispatch(ArtifactCommand::Undo).await.expect("undo");
        assert_eq!(store.snapshot().await.expect("undo snapshot"), initial, "undo did not restore initial snapshot");
        store.dispatch(ArtifactCommand::Redo).await.expect("redo");
        assert_eq!(store.snapshot().await.expect("redo snapshot"), post, "redo did not restore post snapshot");
        let replayed = materialize_document_snapshot(store.envelope().await, store.applied_edit_ids().await).await.expect("replay");
        assert_eq!(replayed, post, "materialization from replay diverged from store snapshot");
    }

    /// @emoji 📜️ Asserts a DSL round trip: `P::parse_dsl(&snapshot.print_dsl())` recovers an equal
    /// snapshot. The compile-time validation ground truth for every technology's `🔖️Dsl` region —
    /// call this from a `#[test]` over every `include_str!` fixture.
    pub async fn assert_dsl_round_trip<P>(snapshot: &P)
    where
        P: ArtifactDsl + PartialEq + std::fmt::Debug,
    {
        let printed = snapshot.print_dsl().await;
        let parsed = P::parse_dsl(&printed).await.unwrap_or_else(|error| panic!("dsl parse failed: {error}"));
        assert_eq!(&parsed, snapshot, "dsl round trip diverged;\nprinted:\n{printed}");
    }

    /// @emoji 🧮️ Config artifact twin of [`assert_dsl_round_trip`] — same law for `ConfigRecord` snapshots.
    pub async fn assert_config_round_trip<C>(snapshot: &C)
    where
        C: ConfigRecord + PartialEq + std::fmt::Debug,
    {
        assert_dsl_round_trip(snapshot).await;
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
    pub async fn check_dsl_fixture_text_laws<P>(text: &str) -> Result<(), String>
    where
        P: ArtifactDsl + PartialEq,
    {
        let first = P::parse_dsl(text).await.map_err(|error| format!("parse failed: {error}"))?;
        let printed_once = first.print_dsl().await;
        let second = P::parse_dsl(&printed_once).await.map_err(|error| format!("reparse failed: {error}\nprinted:\n{printed_once}"))?;
        if first != second {
            return Err(format!("parse->print->reparse fixpoint diverged;\nprinted:\n{printed_once}"));
        }
        let printed_twice = second.print_dsl().await;
        if printed_once != printed_twice {
            return Err(format!("canonicalize is not idempotent;\nonce:\n{printed_once}\ntwice:\n{printed_twice}"));
        }
        Ok(())
    }

    /// @emoji 📦️ Asserts a pack round trip: `P::decode_pack(&snapshot.encode_pack())` recovers an
    /// equal snapshot — the pack sibling of `assert_dsl_round_trip`.
    pub async fn assert_pack_round_trip<P>(snapshot: &P)
    where
        P: ArtifactPack + PartialEq + std::fmt::Debug,
    {
        let bytes = snapshot.encode_pack().await;
        let decoded = P::decode_pack(&bytes).await.unwrap_or_else(|error| panic!("pack decode failed: {error}"));
        assert_eq!(&decoded, snapshot, "pack round trip diverged");
    }

    /// @emoji ⚖️ Asserts dsl and pack are two encodings of the SAME value: `decode_pack(
    /// encode_pack(p)) == parse_dsl(print_dsl(p)) == p` — the compile-time validation ground truth
    /// for the whole pack rollout's central LAW (see `ArtifactPack`'s doc comment).
    pub async fn assert_dsl_pack_equivalence<P>(snapshot: &P)
    where
        P: ArtifactDsl + ArtifactPack + Clone + PartialEq + std::fmt::Debug,
    {
        let via_pack = P::decode_pack(&snapshot.encode_pack().await).await.unwrap_or_else(|error| panic!("pack decode failed: {error}"));
        let via_dsl = P::parse_dsl(&snapshot.print_dsl().await).await.unwrap_or_else(|error| panic!("dsl parse failed: {error}"));
        assert_eq!(&via_pack, snapshot, "pack round trip diverged from source snapshot");
        assert_eq!(&via_dsl, snapshot, "dsl round trip diverged from source snapshot");
        assert_eq!(via_pack, via_dsl, "pack and dsl round trips diverged from each other");
    }

    /// @emoji ⚡️ Asserts an op-text round trip for a single operation: `print_op` contains no newline
    /// and `Op::parse_op` recovers an equal operation from it. The compile-time validation ground
    /// truth for every technology's `🔖️OpText` region — call this once per `Mutation` variant.
    pub async fn assert_op_line_round_trip<Op>(operation: &Op)
    where
        Op: OpText + PartialEq + std::fmt::Debug,
    {
        let printed = operation.print_op().await;
        assert!(!printed.contains('\n'), "print_op must be one line, got: {printed:?}");
        let parsed = Op::parse_op(&printed).await.unwrap_or_else(|error| panic!("op parse failed: {error}"));
        assert_eq!(&parsed, operation, "op-text round trip diverged; printed: {printed:?}");
    }

    /// @emoji ⚖️ Asserts op text and op binary are two encodings of the SAME operation:
    /// `decode_op(encode_op(op)) == parse_op(print_op(op)) == op`, and the binary encoding is
    /// deterministic. The compile-time validation ground truth for every technology's `OpBinary`
    /// impl — the op-level mirror of {@link assert_dsl_pack_equivalence}.
    pub async fn assert_op_text_binary_equivalence<Op>(operation: &Op)
    where
        Op: OpText + OpBinary + PartialEq + std::fmt::Debug,
    {
        assert_op_line_round_trip(operation).await;
        let encoded = operation.encode_op().await.unwrap_or_else(|error| panic!("op encode failed: {error}"));
        let encoded_again = operation.encode_op().await.unwrap_or_else(|error| panic!("op re-encode failed: {error}"));
        assert_eq!(encoded, encoded_again, "op binary encoding is not deterministic");
        let decoded = Op::decode_op(&encoded).await.unwrap_or_else(|error| panic!("op decode failed: {error}"));
        assert_eq!(&decoded, operation, "op-binary round trip diverged from source operation");
    }

    /// @emoji ⚖️ Asserts command text and command binary are two encodings of the SAME command:
    /// `ArtifactCommand::decode_op(&c.encode_op()) == parse_command(print_command(c)) == c`, and the
    /// binary encoding is deterministic. The compile-time validation ground truth for
    /// `ArtifactCommand`'s text/binary pair — the command-level mirror of
    /// `assert_op_text_binary_equivalence`.
    pub async fn assert_command_text_binary_equivalence<Op>(command: &ArtifactCommand<Op>)
    where
        Op: OpText + OpBinary + Clone + PartialEq + std::fmt::Debug,
    {
        let printed = print_command(command).await.unwrap_or_else(|error| panic!("command print failed: {error}"));
        let parsed: ArtifactCommand<Op> = parse_command(&printed).await.unwrap_or_else(|error| panic!("command parse failed: {error}"));
        assert_eq!(&parsed, command, "command text round trip diverged; printed:\n{printed}");
        let encoded = command.encode_op().await.unwrap_or_else(|error| panic!("command encode failed: {error}"));
        let encoded_again = command.encode_op().await.unwrap_or_else(|error| panic!("command re-encode failed: {error}"));
        assert_eq!(encoded, encoded_again, "command binary encoding is not deterministic");
        let decoded: ArtifactCommand<Op> = ArtifactCommand::<Op>::decode_op(&encoded).await.unwrap_or_else(|error| panic!("command decode failed: {error}"));
        assert_eq!(&decoded, command, "command binary round trip diverged from source command");
    }

    /// @emoji 📄️ Asserts that printing a store's envelope to text and parsing it back yields the same
    /// live snapshot the store already holds — the ground truth for {@link print_document_text}/
    /// {@link parse_document_text} on any technology once it implements `ArtifactDsl` + `OpText`.
    pub async fn assert_document_text_round_trip<P, Mutation>(store: &ArtifactStore<P, Mutation>)
    where
        P: Clone + ArtifactDsl + ArtifactPack + PartialEq + std::fmt::Debug + Serialize + DeserializeOwned,
        Mutation: Clone + OpText + self::Mutation<P> + PartialEq + Serialize + DeserializeOwned + OpBinary,
    {
        let live = store.snapshot().await.expect("store snapshot");
        let files = print_document_text(store.envelope().await).await.expect("print document text");
        let parsed: ParsedDocumentText<P, Mutation> = parse_document_text(&files.dsl, &files.ops).await.unwrap_or_else(|error| panic!("parse document text failed: {error}"));
        assert!(parsed.envelope == store.envelope().await.clone(), "document-text round trip lost durable history");
        assert_eq!(parsed.snapshot, live, "document-text round trip diverged from store snapshot");
    }

    /// @emoji 🗄️ Asserts a full pack-based document round trip: mirrors
    /// `assert_document_text_round_trip` but via `print_document_pack`/`parse_document_pack`, and
    /// additionally asserts the pack path's parsed snapshot agrees with the text path's — the two
    /// storage formats must never diverge on the same store.
    pub async fn assert_document_pack_round_trip<P, Mutation>(store: &ArtifactStore<P, Mutation>)
    where
        P: Clone + ArtifactDsl + ArtifactPack + PartialEq + std::fmt::Debug + Serialize + DeserializeOwned,
        Mutation: Clone + OpText + OpBinary + self::Mutation<P> + PartialEq + Serialize + DeserializeOwned,
    {
        let live = store.snapshot().await.expect("store snapshot");
        let pack_files = print_document_pack(store.envelope().await).await.expect("print document pack");
        let parsed_pack: ParsedDocumentText<P, Mutation> = parse_document_pack(&pack_files.pack, &pack_files.spr).await.unwrap_or_else(|error| panic!("parse document pack failed: {error}"));
        assert_eq!(parsed_pack.snapshot, live, "document-pack round trip diverged from store snapshot");

        let text_files = print_document_text(store.envelope().await).await.expect("print document text");
        let parsed_text: ParsedDocumentText<P, Mutation> = parse_document_text(&text_files.dsl, &text_files.ops).await.unwrap_or_else(|error| panic!("parse document text failed: {error}"));
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
    pub async fn assert_command_envelope_round_trip<P, Mutation>(edit: &Edit<Mutation>, document_id: &ArtifactId, schema: &SchemaId)
    where
        P: Clone + PartialEq + std::fmt::Debug,
        Mutation: self::Mutation<P> + PartialEq + std::fmt::Debug + OpBinary,
    {
        let envelopes = crate::os_spr::mutation_envelope_from_edit::<P, Mutation>(edit, document_id, schema).await.unwrap_or_else(|error| panic!("mutation_envelope_from_edit must succeed for a well-formed edit: {error}"));
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
            let recovered_forward = Mutation::decode_op(&envelope.diff.payload).await.unwrap_or_else(|error| panic!("envelope diff payload must decode back into an equal operation: {error}"));
            assert_eq!(&recovered_forward, &edit.forwards[index], "envelope diff payload did not decode back into an equal forward operation");
            match edit.inverse.get(index) {
                Some(backward) => {
                    let recovered_backward = Mutation::decode_op(&envelope.inverse.payload).await.unwrap_or_else(|error| panic!("envelope inverse payload must decode back into an equal operation: {error}"));
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
    pub async fn assert_live_equals_replay<P, Mutation>(store: &ArtifactStore<P, Mutation>)
    where
        P: Clone + ArtifactPack + PartialEq + std::fmt::Debug + Serialize + DeserializeOwned,
        Mutation: Clone + Serialize + DeserializeOwned + self::Mutation<P> + OpBinary + OpText,
    {
        let live = store.snapshot().await.expect("store snapshot");
        let replayed = materialize_document_snapshot(store.envelope().await, store.applied_edit_ids().await).await.expect("replay");
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
        async fn dialect() -> crate::os_io::ArtifactDialect;
        async fn fidelity() -> IoFidelityClass;
        async fn drops() -> &'static [&'static str];
        async fn parse_native(asset: &ExampleAsset<'_>) -> Result<Self::Snapshot, String>;
        async fn export_native(snapshot: &Self::Snapshot) -> Result<Vec<u8>, String>;
        async fn reimport_native(bytes: &[u8]) -> Result<Self::Snapshot, String>;
        async fn infer(snapshot: &Self::Snapshot) -> Self::Inference;
        async fn sample_mutations(snapshot: &Self::Snapshot) -> Vec<Self::Mutation>;
        async fn validate_payload(bytes: &[u8]) -> Result<(), Vec<String>>;
        async fn validate_negative(bytes: &[u8]) -> Result<Vec<String>, String>;
        async fn is_derived() -> bool {
            false
        }
    }

    async fn skip_stage(error: &str) -> bool {
        error.starts_with("SKIP:")
    }

    async fn skip_validation(codes: &[String]) -> bool {
        codes.len() == 1 && skip_stage(&codes[0]).await
    }

    /// 🧪 Assert import/export byte fidelity matches the declared class at the raw-byte layer.
    pub async fn assert_import_export_fidelity_bytes(original: &[u8], exported: &[u8], class: IoFidelityClass) {
        match class {
            IoFidelityClass::Exact => assert_eq!(exported, original, "exact fidelity requires byte-identical export"),
            IoFidelityClass::Canonical | IoFidelityClass::Semantic | IoFidelityClass::Lossy => {}
        }
    }

    /// 🎯 Assert two inference runs are identical (determinism law S6).
    pub async fn assert_inference_determinism<I: PartialEq + std::fmt::Debug>(a: &I, b: &I) {
        assert_eq!(a, b, "inference is not deterministic across two runs on the same snapshot");
    }

    /// 🔁 Drive S0–S10 subset roundtrip stages. Stages that need unavailable hooks are skipped only when
    /// the corresponding trait method returns an explicit Err starting with "SKIP:"; otherwise failures panic with stage id.
    pub async fn assert_subset_roundtrip<S: SubsetRoundtripSpec>(example: &ExampleAsset<'_>, negative: Option<&ExampleAsset<'_>>) {
        assert!(!example.bytes.is_empty(), "S0: example bytes must be non-empty");
        assert!(!example.provenance.is_empty(), "S0: example provenance must be non-empty");

        let dialect = S::dialect().await;
        assert!(!dialect.artifact_kind.is_empty(), "S1: dialect artifact_kind must be non-empty");
        assert!(!dialect.standard.is_empty(), "S1: dialect standard must be non-empty");
        assert!(!dialect.subset.is_empty(), "S1: dialect subset must be non-empty");

        let snapshot = match S::parse_native(example).await {
            Ok(snapshot) => snapshot,
            Err(error) if skip_stage(&error).await => return,
            Err(error) => panic!("S2 failed: {error}"),
        };

        assert_dsl_round_trip(&snapshot).await;
        assert_pack_round_trip(&snapshot).await;
        assert_dsl_pack_equivalence(&snapshot).await;

        let mutations = S::sample_mutations(&snapshot).await;
        for mutation in &mutations {
            assert_operation_round_trip(&snapshot, mutation.clone()).await;
            assert_op_line_round_trip(mutation).await;
            assert_op_text_binary_equivalence(mutation).await;
        }

        let inference_a = S::infer(&snapshot).await;
        let inference_b = S::infer(&snapshot).await;
        assert_inference_determinism(&inference_a, &inference_b).await;

        if let Some(first) = mutations.first() {
            assert_store_roundtrip(snapshot.clone(), first.clone()).await;
        }

        match S::export_native(&snapshot).await {
            Ok(exported) => {
                assert_import_export_fidelity_bytes(example.bytes, &exported, S::fidelity().await).await;
                match S::reimport_native(&exported).await {
                    Ok(reimported) => match S::fidelity().await {
                        IoFidelityClass::Exact => {}
                        IoFidelityClass::Canonical | IoFidelityClass::Semantic => {
                            assert_eq!(&reimported, &snapshot, "S8: canonical/semantic fidelity requires equal snapshot after reimport");
                        }
                        IoFidelityClass::Lossy if S::drops().await.is_empty() => {
                            assert_eq!(&reimported, &snapshot, "S8: lossy fidelity with empty drop set requires equal snapshot after reimport");
                        }
                        IoFidelityClass::Lossy => {}
                    },
                    Err(error) if skip_stage(&error).await => {}
                    Err(error) => panic!("S8 reimport failed: {error}"),
                }
            }
            Err(error) if skip_stage(&error).await => {}
            Err(error) => panic!("S8 export failed: {error}"),
        }

        match S::validate_payload(example.bytes).await {
            Ok(()) => {}
            Err(codes) if skip_validation(&codes).await => {}
            Err(codes) => panic!("S9 validate_payload failed: {codes:?}"),
        }

        if S::is_derived().await {
            if let Some(negative) = negative {
                match S::validate_negative(negative.bytes).await {
                    Ok(codes) => assert!(!codes.is_empty(), "S9: derived negative must yield non-empty diagnostic codes"),
                    Err(error) if skip_stage(&error).await => {}
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

    struct ArtifactStore<P, Mutation>(super::ArtifactStore<P, Mutation>)
    where
        P: Clone + Serialize + DeserializeOwned,
        Mutation: Clone + Serialize + DeserializeOwned + super::Mutation<P>;

    impl<P, Mutation> ArtifactStore<P, Mutation>
    where
        P: Clone + Serialize + DeserializeOwned + ArtifactPack,
        Mutation: Clone + Serialize + DeserializeOwned + super::Mutation<P> + OpBinary + OpText,
    {
        async fn new(envelope: ArtifactEnvelope<P, Mutation>) -> Self {
            Self(super::ArtifactStore::new(envelope).expect("test fixture history is valid"))
        }

        async fn current_checkpoint_id(&self) -> Option<&str> {
            self.0.current_checkpoint_id()
        }
    }

    impl<P, Mutation> std::ops::Deref for ArtifactStore<P, Mutation>
    where
        P: Clone + Serialize + DeserializeOwned,
        Mutation: Clone + Serialize + DeserializeOwned + super::Mutation<P>,
    {
        type Target = super::ArtifactStore<P, Mutation>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<P, Mutation> std::ops::DerefMut for ArtifactStore<P, Mutation>
    where
        P: Clone + Serialize + DeserializeOwned,
        Mutation: Clone + Serialize + DeserializeOwned + super::Mutation<P>,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl<P, Mutation> SpaceMember for ArtifactStore<P, Mutation>
    where
        P: Clone + Serialize + DeserializeOwned + ArtifactPack + Send + 'static,
        Mutation: Clone + Serialize + DeserializeOwned + super::Mutation<P> + OpBinary + OpText + Send + 'static,
    {
        async fn document_id(&self) -> &str {
            SpaceMember::document_id(&self.0).await
        }
        async fn is_dirty(&self) -> bool {
            SpaceMember::is_dirty(&self.0).await
        }
        async fn commit_checkpoint(&mut self, message: String, authors: Vec<Author>) -> Result<String, VcsError> {
            SpaceMember::commit_checkpoint(&mut self.0, message, authors).await
        }
        async fn current_checkpoint_id(&self) -> Option<String> {
            SpaceMember::current_checkpoint_id(&self.0).await
        }
        async fn current_alternative_id(&self) -> Option<String> {
            SpaceMember::current_alternative_id(&self.0).await
        }
        async fn checkout(&mut self, checkpoint_id: &str, alternative_id: &str) -> Result<(), VcsError> {
            SpaceMember::checkout(&mut self.0, checkpoint_id, alternative_id).await
        }
        async fn create_alternative(&mut self, name: String) -> Result<String, VcsError> {
            SpaceMember::create_alternative(&mut self.0, name).await
        }
        async fn last_local_edit_timestamp(&self) -> Option<HybridLogicalTimestamp> {
            SpaceMember::last_local_edit_timestamp(&self.0).await
        }
        async fn last_undone_local_edit_timestamp(&self) -> Option<HybridLogicalTimestamp> {
            SpaceMember::last_undone_local_edit_timestamp(&self.0).await
        }
        async fn undo(&mut self) -> Result<(), VcsError> {
            SpaceMember::undo(&mut self.0).await
        }
        async fn redo(&mut self) -> Result<(), VcsError> {
            SpaceMember::redo(&mut self.0).await
        }
        async fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        async fn preview_wire(&self, ops: &[Vec<u8>]) -> Vec<crate::os_spr::MutationMessage> {
            SpaceMember::preview_wire(&self.0, ops).await
        }
        async fn dispatch_wire(&mut self, command: &[u8]) -> Result<CommandReceipt, VcsError> {
            SpaceMember::dispatch_wire(&mut self.0, command).await
        }
        async fn dispatch_wire_with_policy(&mut self, command: &[u8], policy: crate::os_spr::MergePolicy) -> Result<CommandReceipt, VcsError> {
            SpaceMember::dispatch_wire_with_policy(&mut self.0, command, policy).await
        }
        async fn tail_group_id(&self) -> Option<String> {
            SpaceMember::tail_group_id(&self.0).await
        }
        async fn tail_edit_id(&self) -> Option<String> {
            SpaceMember::tail_edit_id(&self.0).await
        }
        async fn redo_tail(&self) -> Option<(String, Option<String>)> {
            SpaceMember::redo_tail(&self.0).await
        }
        async fn stamp_tail_group_id(&mut self, group_id: &str) -> Result<(), VcsError> {
            SpaceMember::stamp_tail_group_id(&mut self.0, group_id).await
        }
        async fn stamp_tail_origin(&mut self, origin: crate::os_spr::MutationOrigin) -> Result<(), VcsError> {
            SpaceMember::stamp_tail_origin(&mut self.0, origin).await
        }
        async fn set_owner(&mut self, owner: Option<OwnerRef>) {
            SpaceMember::set_owner(&mut self.0, owner).await
        }
        async fn document_pack_bytes(&self) -> Result<Vec<u8>, VcsError> {
            SpaceMember::document_pack_bytes(&self.0).await
        }
        async fn envelope_pack_bytes(&self) -> Result<Vec<u8>, VcsError> {
            SpaceMember::envelope_pack_bytes(&self.0).await
        }
        async fn pack_at_checkpoint(&self, checkpoint_id: &str) -> Result<Vec<u8>, VcsError> {
            SpaceMember::pack_at_checkpoint(&self.0, checkpoint_id).await
        }
    }

    /// @emoji 🏭️ Test-fixture `MemberFactory`: single "kind" (any `kind` string matches — these
    /// fixtures never register more than one composable kind under one coordinator), schema fixed
    /// at `"demo/v1"`, empty genesis packs default to `P::default()` (mirrors the deleted
    /// `DemoChildFactory`'s fixture-local empty-pack convenience — production `create_member_store`
    /// deliberately rejects an empty pack, this fixture-only default is NOT that).
    impl<P, Mutation> super::MemberFactory for ArtifactStore<P, Mutation>
    where
        P: Clone + Default + Serialize + DeserializeOwned + ArtifactPack + Send + 'static,
        Mutation: Clone + Serialize + DeserializeOwned + super::Mutation<P> + OpBinary + OpText + Send + 'static,
    {
        async fn create(_kind: &str, id: &str, dialect: &crate::os_io::ArtifactDialect, initial_pack: &[u8]) -> Result<Self, VcsError> {
            let seeded = if initial_pack.is_empty() { P::default().encode_pack() } else { initial_pack.to_vec() };
            Ok(Self(super::create_member_store("demo/v1", id, dialect, &seeded).await?))
        }
        async fn open(_kind: &str, envelope_pack: &[u8]) -> Result<Self, VcsError> {
            Ok(Self(super::open_member_store(envelope_pack).await?))
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, crate::os_dsl::DslArtifact)]
    #[dsl(id = "demo.doc", extension = "demo")]
    struct DemoSnapshot {
        n: i32,
    }

    //#region 🔖️ArtifactCodec
    /// 📜️ Handcrafted ArtifactDsl (P6).
    impl ArtifactDsl for DemoSnapshot {
        const EXTENSION: &'static str = Self::__DSL_EXTENSION;
        async fn envelope_id() -> &'static str {
            Self::__DSL_ENVELOPE_ID
        }
        async fn parse_dsl(text: &str) -> Result<Self, TextError> {
            let body = match semio_format::split_text_preamble(text) {
                Ok((_, rest)) => rest,
                Err(_) => text,
            };
            let record = crate::os_dsl::parse(body, &Self::__dsl_spec(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Document })?;
            Self::__dsl_from_record(&record)
        }
        async fn print_dsl(&self) -> String {
            let body = crate::os_dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), crate::os_dsl::JoinMode::Document);
            let envelope = semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), semio_format::Component::Dsl, 1).expect("valid envelope_id");
            semio_format::wrap_text(&envelope, &body)
        }
    }
    /// 📦️ Handcrafted ArtifactPack (P6).
    impl ArtifactPack for DemoSnapshot {
        async fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
            let inner = pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
            let envelope = semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), semio_format::Component::Pack, 1).map_err(|e| PackError::Schema(e.to_string()))?;
            Ok(semio_format::wrap_binary(&envelope, &inner))
        }
        async fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
            let (envelope, inner) = semio_format::unwrap_binary(bytes).map_err(|e| PackError::Schema(e.to_string()))?;
            if envelope.envelope_id() != <Self as ArtifactDsl>::envelope_id() {
                return Err(PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as ArtifactDsl>::envelope_id(), envelope.envelope_id())));
            }
            let (record, _report) = pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
            Self::__dsl_from_record(&record).map_err(text_error_to_pack_error)
        }
        async fn record_spec() -> Option<crate::os_dsl::RecordSpec> {
            Some(Self::__dsl_spec())
        }
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
        async fn apply(&self, snapshot: &DemoSnapshot) -> crate::os_spr::MutationApplyResult<DemoSnapshot> {
            Ok(DemoSnapshot { n: self.n.unwrap_or(snapshot.n) })
        }

        async fn absorb(&mut self, other: Self) {
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
        /// @emoji 🗑️ Deletes `n` — modeled as the `i32::MIN` sentinel (kept on `DemoSnapshot { n:
        /// i32 }` rather than adding a field, so every existing `DemoSnapshot { n: .. }` literal
        /// across this test module keeps compiling unchanged). Lets `🧪️Tests` exercise a real
        /// modify-vs-delete conflict (`SetN` on an already-deleted target ⇒
        /// `mutation.target-missing`) without a second document-shaped test fixture.
        #[dsl(key = "delete-n")]
        DeleteN,
        /// @emoji 🧯 Adds `delta` to `n`, always recording an Info `mutation.cascade` diagnostic (a
        /// real, non-empty ledger entry that still commits — Info is never rejected and never raises
        /// a Degraded conflict) — Error `mutation.target-missing` on an already-deleted target (same
        /// rule as `SetN`). Unlike `SeverityMutation` below (fixed severity regardless of state),
        /// `BumpN`'s severity is STATE-DEPENDENT — the same op can be clean-with-a-message on first
        /// commit and then quarantine once a rewind changes what it replays against. MEDIUM-4's
        /// mixed-batch fixture (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-
        /// CONFLICTS` J1 robustness pass) needs exactly that to prove a retroactively-invalidated
        /// edit's own non-empty message ledger entry gets cleared, not just a never-populated one.
        #[dsl(key = "bump-n")]
        BumpN { delta: i32 },
    }

    //#region 🔖️OpCodec
    /// 🎞️ Handcrafted OpText (P6).
    impl OpText for DemoMutation {
        async fn parse_op(line: &str) -> Result<Self, TextError> {
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
        async fn print_op(&self) -> String {
            let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
            let variants = <Self as crate::os_dsl::DslVariants>::variants();
            let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
            crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)
        }
    }

    /// 🎯️ Handcrafted OpBinary (P6).
    impl OpBinary for DemoMutation {
        async fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
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
        async fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
            const OP_BINARY_FORMAT: u8 = 1;
            let mut reader = crate::os_pack::ByteReader::new(bytes).await;
            let format = reader.read_u8().await?;
            if format != OP_BINARY_FORMAT {
                return Err(crate::os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
            }
            let ordinal = reader.read_varint_u64().await?;
            let variants = <Self as crate::os_dsl::DslVariants>::variants();
            let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
            let spec = spec_fn();
            let body = &bytes[reader.position().await..];
            let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
            <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "op record", offset: reader.position().await as u64, detail: error.to_string() })
        }
    }
    //#endregion 🔖️OpCodec

    impl Mutation<DemoSnapshot> for DemoMutation {
        type Diff = DemoDiff;

        async fn diff(&self, snapshot: &DemoSnapshot) -> crate::os_spr::MutationOutcome<DemoDiff> {
            match self {
                // 🎯️ Verb-family rule (frozen fan-out table): `set` on an absent target ⇒ Error
                // `mutation.target-missing`, empty diff (LAW 2) — the modify-vs-delete conflict case.
                DemoMutation::SetN { .. } if snapshot.n == i32::MIN => crate::os_spr::MutationOutcome::error("mutation.target-missing", "n was deleted by a concurrent edit", ["n"]),
                DemoMutation::SetN { n } => crate::os_spr::MutationOutcome::new(DemoDiff { n: Some(*n) }),
                DemoMutation::DeleteN => crate::os_spr::MutationOutcome::new(DemoDiff { n: Some(i32::MIN) }),
                DemoMutation::BumpN { .. } if snapshot.n == i32::MIN => crate::os_spr::MutationOutcome::error("mutation.target-missing", "n was deleted by a concurrent edit", ["n"]),
                DemoMutation::BumpN { delta } => crate::os_spr::MutationOutcome::new(DemoDiff { n: Some(snapshot.n.saturating_add(*delta)) }).info("mutation.cascade", "n bumped"),
            }
        }

        async fn inverse(&self, snapshot: &DemoSnapshot) -> Vec<Self> {
            vec![DemoMutation::SetN { n: snapshot.n }]
        }
    }

    /// @emoji 🛰️ Builds a foreign {@link MutationEnvelope} (as if authored by `actor` on another peer) by

    /// applying `operation` in a throwaway peer store and stamping the envelope's actor id.
    async fn foreign_mutation_envelope(actor: &str, operation: DemoMutation) -> crate::os_spr::MutationEnvelope {
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
    async fn rejected_remote_ingest_keeps_state_and_dag_unpoisoned() {
        let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: 0 }, None));
        let valid = foreign_mutation_envelope("peer", DemoMutation::SetN { n: 7 });
        let mut malformed = valid.clone();
        malformed.diff.payload = vec![0xff];
        let before = store.envelope().clone();

        assert!(store.ingest_remote(malformed).is_err(), "malformed remote data must reject before committing the DAG or history");
        assert_eq!(store.envelope(), &before);
        assert_eq!(store.snapshot().expect("unchanged snapshot"), DemoSnapshot { n: 0 });

        store.ingest_remote(valid).expect("the rejected envelope must not poison its mutation id in the DAG");
        assert_eq!(store.snapshot().expect("accepted snapshot"), DemoSnapshot { n: 7 });
    }

    #[test]
    async fn remote_ingest_requires_duplicate_mutation_payload_equivalence() {
        let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: 0 }, None));
        let accepted = foreign_mutation_envelope("peer", DemoMutation::SetN { n: 2 });
        store.ingest_remote(accepted.clone()).expect("first remote envelope");
        let before = store.envelope().clone();

        store.ingest_remote(accepted.clone()).expect("an exact duplicate is idempotent");
        let mut conflict = foreign_mutation_envelope("peer", DemoMutation::SetN { n: 9 });
        conflict.mutation_id = accepted.mutation_id.clone();
        let error = store.ingest_remote(conflict).expect_err("the same mutation id may not carry a different payload");
        assert!(matches!(error, VcsError::ValidationFailed(message) if message.contains("conflicts with its established payload")));
        assert_eq!(store.envelope(), &before);
        assert_eq!(store.snapshot().expect("unchanged snapshot"), DemoSnapshot { n: 2 });
    }

    #[test]
    async fn snapshot_merge_preflights_every_conflict_before_committing() {
        let mut local = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: 0 }, None));
        local.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("local edit");
        local.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("local checkpoint".into()), authors: Vec::new() }).expect("local checkpoint");
        local.dispatch(ArtifactCommand::CreateAlternative { name: "local".into() }).expect("local alternative");
        let local_alternative = local.envelope().vcs.alternatives[0].id.clone();
        let before = local.envelope().clone();

        let mut remote = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: 0 }, None));
        remote.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 9 }], description: None }).expect("remote edit");
        remote.0.envelope.vcs.alternatives.push(Alternative { id: local_alternative, name: "conflicting remote alternative".into(), checkpoint_ids: Vec::new() });
        let files = print_document_pack(remote.envelope()).expect("remote pack");

        assert!(local.merge_remote_snapshot(&files.pack, &files.spr).is_err(), "a late registry conflict must reject the whole snapshot merge");
        assert_eq!(local.envelope(), &before);
        assert_eq!(local.snapshot().expect("unchanged snapshot"), DemoSnapshot { n: 1 });
    }

    /// 🐛️ HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS w3-g: the store-level minimal
    /// reproduction of `two_instances_converge_on_disjoint_edits_via_backbone`'s "invalid edit
    /// reference" fault. `b` learns about `a`'s edit only via `ingest_remote`, which reconstructs it
    /// under the WIRE per-op id (`edit_from_operation_envelope`'s `id: envelope.mutation_id.0`, the
    /// `"{edit_id}#{opIndex}"` scheme from `mutation_ids_for_edit`) — never `a`'s own real `Edit.id`.
    /// `b`'s own `CommitCheckpoint` right after that ingest is self-consistent (its `Change`
    /// references the id it actually stored the edit under), so it succeeds — this is NOT yet the
    /// bug. The bug surfaces once `a` commits its OWN checkpoint (`Change.edit_ids` naming `a`'s
    /// edit under `a`'s real, un-suffixed id) and relays a full snapshot — exactly what
    /// `flush_outbound(is_apply: false)` does for every structural command including
    /// `CommitCheckpoint`. `merge_remote_snapshot`'s `batch.is_empty()` fast path (reached because
    /// `b` already recognizes `a`'s edit as "known" via its wire-id-derived operation identity)
    /// merges `a`'s `Change` in verbatim without reconciling ids, so `validate_durable_history`
    /// rightly rejects it: `b`'s own `vcs.edits` never gained an entry under the bare id `a`'s
    /// `Change` names.
    #[test]
    async fn checkpoint_after_ingesting_a_remote_edit_stays_valid_once_the_sender_s_own_checkpoint_snapshot_arrives() {
        let mut a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: 0 }, None));
        a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("a's local edit");
        let a_edit = a.envelope().vcs.edits.last().expect("a has an edit").clone();
        let document_id = ArtifactId(a.envelope().id.clone());
        let schema = SchemaId(a.envelope().schema.clone());
        let wire_envelopes = crate::os_spr::mutation_envelope_from_edit::<DemoSnapshot, DemoMutation>(&a_edit, &document_id, &schema).expect("encode a's edit for the wire");

        let mut b = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: 0 }, None));
        for envelope in wire_envelopes {
            b.ingest_remote(envelope).expect("b ingests a's edit over the wire");
        }
        b.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("b checkpoint".into()), authors: Vec::new() }).expect("b's own checkpoint, self-consistent so far — not yet the bug");
        assert!(!b.envelope().vcs.changes.last().expect("b minted a change").edit_ids.is_empty(), "b's checkpoint must actually cover the ingested edit");

        a.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("a checkpoint".into()), authors: Vec::new() }).expect("a's own checkpoint");
        let files = print_document_pack(a.envelope()).expect("a's pack");

        b.merge_remote_snapshot(&files.pack, &files.spr).expect("b must absorb a's checkpoint even though b only knows a's edit under its wire id");

        for change in &b.envelope().vcs.changes {
            for edit_id in &change.edit_ids {
                assert!(b.envelope().vcs.edits.iter().any(|edit| edit.id == *edit_id), "change {} references edit {edit_id}, which does not exist in b's own vcs.edits", change.id);
            }
        }
    }

    //#region 🔖️MergePolicyTests
    // 🎯️ `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6 acceptance —
    // two-peer merge-policy/conflict tests, written standalone (not depending on 1-D's
    // `📡️spr/🧪️testkit` `🔖️Laws` helpers landing first).

    /// 🛰️ Builds a `MutationEnvelope` with an explicit HLC (bypassing wall-clock timing) so arrival
    /// order and HLC order can be controlled independently — the two-peer tests below need both.
    async fn mutation_envelope_at(actor: &str, mutation_id: &str, operation: DemoMutation, hlc: HybridLogicalTimestamp, dependencies: Vec<MutationId>) -> crate::os_spr::MutationEnvelope {
        crate::os_spr::MutationEnvelope {
            mutation_id: MutationId(mutation_id.to_string()),
            document_id: ArtifactId("demo".to_string()),
            actor: ActorId(actor.to_string()),
            dependencies,
            diff: crate::os_spr::ArtifactDiff { schema: SchemaId("demo/v1".to_string()), payload: operation.encode_op().expect("encode demo mutation") },
            inverse: crate::os_spr::InverseMutation { schema: SchemaId("demo/v1".to_string()), payload: Vec::new() },
            timestamp: hlc,
        }
    }

    async fn fresh_demo_store() -> ArtifactStore<DemoSnapshot, DemoMutation> {
        ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: 0 }, None))
    }

    /// 🛰️ A `DeleteN` at an earlier HLC and a `SetN` at a later HLC — replayed in HLC order, the
    /// `SetN` lands on an already-deleted target and raises `mutation.target-missing` (Error).
    async fn modify_vs_delete_envelopes() -> (crate::os_spr::MutationEnvelope, crate::os_spr::MutationEnvelope) {
        let delete = mutation_envelope_at("deleter", "op-delete", DemoMutation::DeleteN, HybridLogicalTimestamp::new(1, 100), Vec::new());
        let modify = mutation_envelope_at("modifier", "op-modify", DemoMutation::SetN { n: 42 }, HybridLogicalTimestamp::new(2, 200), Vec::new());
        (delete, modify)
    }

    #[test]
    async fn modify_vs_delete_quarantines_under_normal_and_vigilant() {
        for policy in [crate::os_spr::MergePolicy::Normal, crate::os_spr::MergePolicy::Vigilant] {
            let mut store = fresh_demo_store();
            store.set_merge_policy(policy);
            let (delete, modify) = modify_vs_delete_envelopes();
            store.ingest_remote(delete).expect("the delete alone raises no message and always applies");
            let pre_merge = store.snapshot().expect("pre-merge snapshot");
            let pre_merge_ids = store.applied_edit_ids().to_vec();
            let report = store.ingest_remote(modify).expect("a policy rejection is a MergeReport, not an Err");
            assert!(!report.accepted, "{policy:?} must reject an Error-level modify-vs-delete conflict");
            assert_eq!(report.worst, Some(crate::os_dsl::Severity::Error));
            assert_eq!(store.snapshot().expect("unchanged snapshot"), pre_merge, "{policy:?}: state stays pre-merge on reject");
            assert_eq!(store.applied_edit_ids(), pre_merge_ids.as_slice(), "{policy:?}: applied_edit_ids stays pre-merge on reject");
            let conflict_id = report.conflict.clone().expect("a reject must raise a conflict");
            let conflict = store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("conflict recorded on the store");
            assert_eq!(conflict.status, crate::os_spr::ConflictStatus::Open);
            assert!(matches!(conflict.kind, crate::os_spr::ConflictKind::Quarantined { .. }), "{policy:?}: a rejected batch quarantines, it never degrades");
            assert!(store.open_conflicts().any(|conflict| conflict.id == conflict_id));
        }
    }

    #[test]
    async fn modify_vs_delete_applies_under_laissez_faire_with_a_degraded_conflict() {
        let mut store = fresh_demo_store();
        store.set_merge_policy(crate::os_spr::MergePolicy::LaissezFaire);
        let (delete, modify) = modify_vs_delete_envelopes();
        store.ingest_remote(delete).expect("delete applies cleanly");
        let report = store.ingest_remote(modify).expect("LaissezFaire only rejects Fatal");
        assert!(report.accepted);
        assert_eq!(report.worst, Some(crate::os_dsl::Severity::Error));
        assert_eq!(store.snapshot().expect("snapshot").n, i32::MIN, "the modify's part is absent — LAW 2 (an Error message ⇒ no change to its target)");
        assert!(report.replayed.iter().any(|edit_messages| edit_messages.messages.iter().any(|message| message.code.0 == "mutation.target-missing")), "an Error message must be reported");
        let conflict_id = report.conflict.expect("worst >= Warning must raise a Degraded conflict");
        let conflict = store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("conflict recorded");
        assert!(matches!(conflict.kind, crate::os_spr::ConflictKind::Degraded { .. }));
        assert_eq!(conflict.status, crate::os_spr::ConflictStatus::Open);
    }

    #[test]
    async fn chronological_determinism_any_arrival_order_converges() {
        let a = mutation_envelope_at("actor-a", "op-a", DemoMutation::SetN { n: 10 }, HybridLogicalTimestamp::new(1, 100), Vec::new());
        let b = mutation_envelope_at("actor-b", "op-b", DemoMutation::SetN { n: 20 }, HybridLogicalTimestamp::new(2, 300), Vec::new());

        let mut forward = fresh_demo_store();
        forward.ingest_remote(a.clone()).expect("a");
        forward.ingest_remote(b.clone()).expect("b");

        let mut reversed = fresh_demo_store();
        reversed.ingest_remote(b).expect("b");
        reversed.ingest_remote(a).expect("a");

        assert_eq!(forward.snapshot().expect("snapshot"), reversed.snapshot().expect("snapshot"));
        assert_eq!(forward.applied_edit_ids(), reversed.applied_edit_ids(), "both must land in the same HLC order regardless of arrival order");
        assert_eq!(forward.applied_edit_ids(), &["op-a".to_string(), "op-b".to_string()]);
        assert_eq!(forward.conflicts().len(), reversed.conflicts().len());
        assert!(forward.conflicts().is_empty(), "two non-conflicting SetN edits raise no conflict");
    }

    #[test]
    async fn empty_store_snapshot_merge_replays_hlc_order_and_preserves_local_policy() {
        let mut remote = fresh_demo_store();
        let later = mutation_envelope_at("later-peer", "later-op", DemoMutation::SetN { n: 20 }, HybridLogicalTimestamp::new(2, 300), Vec::new());
        let earlier = mutation_envelope_at("earlier-peer", "earlier-op", DemoMutation::SetN { n: 10 }, HybridLogicalTimestamp::new(1, 100), Vec::new());
        remote.ingest_remote(later).expect("remote receives later edit first");
        remote.ingest_remote(earlier).expect("remote receives earlier edit second");
        let files = remote.snapshot_pack().expect("remote snapshot");

        let mut local = fresh_demo_store();
        local.set_merge_policy(crate::os_spr::MergePolicy::Vigilant);
        local.merge_remote_snapshot(&files.pack, &files.spr).expect("empty local history adopts valid remote history");

        assert_eq!(local.0.merge_policy(), crate::os_spr::MergePolicy::Vigilant, "the receiving store's local-only policy is never serialized or overwritten");
        assert_eq!(local.snapshot().expect("adopted snapshot"), DemoSnapshot { n: 20 });
        assert_eq!(local.applied_edit_ids(), &["earlier-op".to_string(), "later-op".to_string()], "adoption replays authoritative history by HLC rather than arrival order");
        assert_eq!(local.envelope().edit_messages, remote.envelope().edit_messages);
        assert_eq!(local.conflicts(), remote.conflicts());
    }

    /// 🎯️ w3-g id-domain unification: a single-op edit's wire id now literally EQUALS the edit's own
    /// real id (`stamp_primary_operation_identity`), so there is nothing left to "remap" for this
    /// case — `remap_snapshot_message_ledger` degenerates to an identity lookup. Kept (renamed from
    /// `..._remaps_the_durable_message_ledger_to_the_wire_edit_id`, which asserted the now-fixed
    /// divergence as its own fixture precondition) as the durable proof that a single-op edit's
    /// message ledger survives ingest-then-snapshot under that shared id; the genuinely divergent
    /// multi-op case is covered separately by
    /// `operations_then_snapshot_partitions_a_multi_forward_ledger_by_wire_edit`.
    #[test]
    async fn operations_then_snapshot_keeps_the_durable_message_ledger_on_the_shared_edit_id() {
        let mut remote = fresh_demo_store();
        remote.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 7 }], description: Some("remote source edit".into()) }).expect("remote apply");
        let source_edit = remote.envelope().vcs.edits.last().expect("source edit").clone();
        let document_id = ArtifactId(remote.envelope().id.clone());
        let schema = SchemaId(remote.envelope().schema.clone());
        let operation = crate::os_spr::mutation_envelope_from_edit::<DemoSnapshot, DemoMutation>(&source_edit, &document_id, &schema).expect("wire operation").pop().expect("one operation");
        assert_eq!(source_edit.id, operation.mutation_id.0, "a single-op edit's wire id must equal its own real id");
        let durable_message = crate::os_spr::MutationMessage::info("mutation.cascade", "remote diagnostic").at(["n"]).at_op(0);
        remote.0.envelope.edit_messages = vec![crate::os_spr::EditMessages { edit_id: source_edit.id.clone(), messages: vec![durable_message.clone()] }];

        let mut local = fresh_demo_store();
        local.ingest_remote(operation.clone()).expect("operations delivery");
        let local_edit_id = operation.mutation_id.0.clone();
        let files = remote.snapshot_pack().expect("snapshot delivery");
        local.merge_remote_snapshot(&files.pack, &files.spr).expect("snapshot converges after operations");

        assert_eq!(local.envelope().vcs.edits.len(), 1, "snapshot must not duplicate the wire operation");
        assert_eq!(local.envelope().edit_messages, vec![crate::os_spr::EditMessages { edit_id: local_edit_id, messages: vec![durable_message] }]);
    }

    #[test]
    async fn operations_then_snapshot_partitions_a_multi_forward_ledger_by_wire_edit() {
        let mut remote = fresh_demo_store();
        remote.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 7 }, DemoMutation::SetN { n: 8 }], description: Some("two source operations".into()) }).expect("remote apply");
        let source_edit = remote.envelope().vcs.edits.last().expect("source edit").clone();
        let document_id = ArtifactId(remote.envelope().id.clone());
        let schema = SchemaId(remote.envelope().schema.clone());
        let operations = crate::os_spr::mutation_envelope_from_edit::<DemoSnapshot, DemoMutation>(&source_edit, &document_id, &schema).expect("wire operations");
        assert_eq!(operations.len(), 2, "fixture source edit has two independent wire operations");
        remote.0.envelope.edit_messages = vec![crate::os_spr::EditMessages {
            edit_id: source_edit.id.clone(),
            messages: vec![crate::os_spr::MutationMessage::info("mutation.cascade", "first source diagnostic").at(["n"]).at_op(0), crate::os_spr::MutationMessage::info("mutation.cascade", "second source diagnostic").at(["n"]).at_op(1)],
        }];

        let mut local = fresh_demo_store();
        for operation in &operations {
            local.ingest_remote(operation.clone()).expect("operations delivery");
        }
        let files = remote.snapshot_pack().expect("snapshot delivery");
        local.merge_remote_snapshot(&files.pack, &files.spr).expect("snapshot converges after operations");

        assert_eq!(local.envelope().vcs.edits.len(), 2, "snapshot must not restore the multi-forward source edit beside its two wire edits");
        assert_eq!(local.envelope().edit_messages.len(), 2, "one source ledger is deterministically split into its two established wire owners");
        for (entry, operation) in local.envelope().edit_messages.iter().zip(&operations) {
            assert_eq!(entry.edit_id, operation.mutation_id.0);
            assert_eq!(entry.messages.len(), 1);
            assert_eq!(entry.messages[0].op_index, Some(0), "the local one-operation edit owns index zero after redistribution");
        }
        assert_eq!(local.envelope().edit_messages[0].messages[0].message, "first source diagnostic");
        assert_eq!(local.envelope().edit_messages[1].messages[0].message, "second source diagnostic");
    }

    #[test]
    async fn snapshot_ledger_remap_rejects_ambiguous_established_operation_ownership() {
        let mut remote = fresh_demo_store();
        remote.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 7 }, DemoMutation::SetN { n: 8 }], description: None }).expect("remote apply");
        let source_edit = remote.envelope().vcs.edits.last().expect("source edit").clone();
        let document_id = ArtifactId(remote.envelope().id.clone());
        let schema = SchemaId(remote.envelope().schema.clone());
        let operations = crate::os_spr::mutation_envelope_from_edit::<DemoSnapshot, DemoMutation>(&source_edit, &document_id, &schema).expect("wire operations");

        let mut local = fresh_demo_store();
        for operation in operations {
            local.ingest_remote(operation).expect("operations delivery");
        }
        let mut duplicate = local.envelope().vcs.edits.first().expect("first wire edit").clone();
        duplicate.id = "ambiguous-wire-owner".into();
        local.0.envelope.vcs.edits.push(duplicate);

        assert!(matches!(local.snapshot_ledger_targets(&source_edit), Err(VcsError::ValidationFailed(message)) if message.contains("ambiguous established edit ownership")));
    }

    #[test]
    async fn empty_store_snapshot_merge_rejects_document_or_schema_mismatch_without_mutation() {
        for (schema, document_id) in [("demo/v1", "foreign"), ("foreign/v1", "demo")] {
            let remote = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>(schema, document_id, DemoSnapshot { n: 0 }, None));
            let files = remote.snapshot_pack().expect("foreign snapshot");
            let mut target = fresh_demo_store();
            let before = target.envelope().clone();
            let generation = target.generation();
            assert!(matches!(target.merge_remote_snapshot(&files.pack, &files.spr), Err(VcsError::ValidationFailed(_))));
            assert_eq!(target.envelope(), &before);
            assert_eq!(target.generation(), generation);
        }
    }

    #[test]
    async fn quarantine_accept_equals_laissez_faire_result() {
        let mut quarantined = fresh_demo_store();
        quarantined.set_merge_policy(crate::os_spr::MergePolicy::Normal);
        let (delete, modify) = modify_vs_delete_envelopes();
        quarantined.ingest_remote(delete.clone()).expect("delete applies cleanly");
        let reject_report = quarantined.ingest_remote(modify.clone()).expect("reject is a report");
        assert!(!reject_report.accepted);
        let conflict_id = reject_report.conflict.expect("conflict raised");
        quarantined.resolve_conflict(&conflict_id.0, crate::os_spr::ConflictResolution::Accept).expect("accept");

        let mut laissez_faire = fresh_demo_store();
        laissez_faire.set_merge_policy(crate::os_spr::MergePolicy::LaissezFaire);
        laissez_faire.ingest_remote(delete).expect("delete applies cleanly");
        laissez_faire.ingest_remote(modify).expect("modify applies under LaissezFaire");

        assert_eq!(quarantined.snapshot().expect("snapshot"), laissez_faire.snapshot().expect("snapshot"));
        assert_eq!(quarantined.applied_edit_ids(), laissez_faire.applied_edit_ids());
        assert_eq!(quarantined.conflicts().iter().filter(|conflict| conflict.status == crate::os_spr::ConflictStatus::Open).count(), 0, "no Open conflict remains — accept must not raise a second conflict");
        assert_eq!(quarantined.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("original conflict kept").status, crate::os_spr::ConflictStatus::Accepted);
    }

    #[test]
    async fn quarantine_discard_preserves_state() {
        let mut store = fresh_demo_store();
        store.set_merge_policy(crate::os_spr::MergePolicy::Normal);
        let (delete, modify) = modify_vs_delete_envelopes();
        store.ingest_remote(delete).expect("delete applies cleanly");
        let pre_discard = store.snapshot().expect("pre-discard snapshot");
        let pre_discard_ids = store.applied_edit_ids().to_vec();
        let reject_report = store.ingest_remote(modify).expect("reject is a report");
        let conflict_id = reject_report.conflict.expect("conflict raised");
        store.resolve_conflict(&conflict_id.0, crate::os_spr::ConflictResolution::Discard).expect("discard");
        assert_eq!(store.snapshot().expect("snapshot"), pre_discard, "a discarded batch must never be applied");
        assert_eq!(store.applied_edit_ids(), pre_discard_ids.as_slice());
        assert_eq!(store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("conflict kept").status, crate::os_spr::ConflictStatus::Discarded);
    }

    #[test]
    async fn ledger_matches_a_fresh_replay_of_the_same_envelopes() {
        let (delete, modify) = modify_vs_delete_envelopes();
        let modify_edit_id = modify.mutation_id.0.clone();

        let mut first = fresh_demo_store();
        first.set_merge_policy(crate::os_spr::MergePolicy::LaissezFaire);
        first.ingest_remote(delete.clone()).expect("delete applies cleanly");
        first.ingest_remote(modify.clone()).expect("modify applies under LaissezFaire");

        let mut replay = fresh_demo_store();
        replay.set_merge_policy(crate::os_spr::MergePolicy::LaissezFaire);
        replay.ingest_remote(delete).expect("delete applies cleanly");
        replay.ingest_remote(modify).expect("modify applies under LaissezFaire");

        assert_eq!(first.messages_for_edit(&modify_edit_id), replay.messages_for_edit(&modify_edit_id));
        assert!(!first.messages_for_edit(&modify_edit_id).is_empty(), "the modify edit must have raised a message");
        assert_eq!(first.snapshot().expect("snapshot"), replay.snapshot().expect("snapshot"));
    }

    #[test]
    async fn applied_edit_ids_stay_sorted_by_hlc_after_a_backdated_remote_insert() {
        let mut store = fresh_demo_store();
        // Local edits get large physical-ms HLCs (the local clock ticks off the real wall clock).
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("local apply 1");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).expect("local apply 2");
        let local_ids = store.applied_edit_ids().to_vec();

        // A remote edit stamped with a tiny HLC — guaranteed to sort before both local edits.
        let backdated = mutation_envelope_at("backdated-actor", "op-backdated", DemoMutation::SetN { n: 99 }, HybridLogicalTimestamp::new(9, 1), Vec::new());
        store.ingest_remote(backdated).expect("backdated insert");

        assert_eq!(store.applied_edit_ids()[0], "op-backdated", "the backdated edit must sort before both local edits");
        assert_eq!(&store.applied_edit_ids()[1..], local_ids.as_slice());

        let hlcs: Vec<HybridLogicalTimestamp> = store.applied_edit_ids().iter().map(|id| store.envelope().vcs.edits.iter().find(|edit| edit.id == *id).and_then(|edit| edit.mutation_meta.first()).map(|meta| meta.timestamp).expect("meta")).collect();
        assert!(hlcs.windows(2).all(|pair| pair[0].cmp_key() <= pair[1].cmp_key()), "applied_edit_ids must stay HLC-sorted: {hlcs:?}");
    }

    //#region 🔖️TestkitLawWiring
    // 🎯️ G2 verification barrier (26/08/16 MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-
    // CONFLICTS) — the tests above assert the same scenarios by hand; these route the SAME
    // scenarios through the frozen `📡️spr/🧪️testkit` `🔖️Merge`/`🔖️Conflict` law helpers so the
    // laws are proven to hold against this store's real `ingest_remote`/`resolve_conflict`/
    // `messages_for_edit`/`.spr` codec, not only against testkit's own synthetic self-tests.

    #[test]
    async fn testkit_law_modify_vs_delete_holds_under_normal_and_vigilant() {
        for policy in [crate::os_spr::MergePolicy::Normal, crate::os_spr::MergePolicy::Vigilant] {
            let mut store = fresh_demo_store();
            store.set_merge_policy(policy);
            let (delete, modify) = modify_vs_delete_envelopes();
            store.ingest_remote(delete).expect("delete applies cleanly");
            let pre_merge = store.snapshot().expect("pre-merge snapshot");
            let report = store.ingest_remote(modify).expect("a policy rejection is a MergeReport, not an Err");
            let post_merge = store.snapshot().expect("post-merge snapshot");
            crate::os_spr::testkit::assert_modify_vs_delete(policy, &pre_merge, &post_merge, &report, store.conflicts(), |snapshot: &DemoSnapshot| snapshot.n != i32::MIN);
        }
    }

    #[test]
    async fn testkit_law_modify_vs_delete_holds_under_laissez_faire() {
        let mut store = fresh_demo_store();
        store.set_merge_policy(crate::os_spr::MergePolicy::LaissezFaire);
        let (delete, modify) = modify_vs_delete_envelopes();
        store.ingest_remote(delete).expect("delete applies cleanly");
        let pre_merge = store.snapshot().expect("pre-merge snapshot");
        let report = store.ingest_remote(modify).expect("LaissezFaire only rejects Fatal");
        let post_merge = store.snapshot().expect("post-merge snapshot");
        crate::os_spr::testkit::assert_modify_vs_delete(crate::os_spr::MergePolicy::LaissezFaire, &pre_merge, &post_merge, &report, store.conflicts(), |snapshot: &DemoSnapshot| snapshot.n != i32::MIN);
    }

    #[test]
    async fn testkit_law_chronological_determinism_holds_for_a_real_modify_vs_delete_batch() {
        let (delete, modify) = modify_vs_delete_envelopes();
        let envelopes = [delete, modify];
        crate::os_spr::testkit::assert_chronological_determinism(envelopes.len(), 7, 6, |order| {
            let mut store = fresh_demo_store();
            for &index in order {
                store.ingest_remote(envelopes[index].clone()).expect("real store ingest must not hard-error even when the batch ends up quarantined");
            }
            let snapshot = store.snapshot().expect("snapshot");
            let applied = store.applied_edit_ids().to_vec();
            let conflict_ids: Vec<crate::os_spr::ConflictId> = store.conflicts().iter().map(|conflict| conflict.id.clone()).collect();
            (snapshot, applied, conflict_ids)
        });
    }

    #[test]
    async fn testkit_law_quarantine_accept_equals_laissez_faire_via_real_store() {
        let mut quarantined = fresh_demo_store();
        quarantined.set_merge_policy(crate::os_spr::MergePolicy::Normal);
        let (delete, modify) = modify_vs_delete_envelopes();
        quarantined.ingest_remote(delete.clone()).expect("delete applies cleanly");
        let reject_report = quarantined.ingest_remote(modify.clone()).expect("reject is a report");
        let conflict_id = reject_report.conflict.expect("conflict raised");
        quarantined.resolve_conflict(&conflict_id.0, crate::os_spr::ConflictResolution::Accept).expect("accept");

        let mut laissez_faire = fresh_demo_store();
        laissez_faire.set_merge_policy(crate::os_spr::MergePolicy::LaissezFaire);
        laissez_faire.ingest_remote(delete).expect("delete applies cleanly");
        laissez_faire.ingest_remote(modify).expect("modify applies under LaissezFaire");

        let accepted_state = quarantined.snapshot().expect("accepted snapshot");
        let laissez_faire_state = laissez_faire.snapshot().expect("laissez-faire snapshot");
        crate::os_spr::testkit::assert_quarantine_accept_equals_laissez_faire(&accepted_state, &laissez_faire_state);
    }

    #[test]
    async fn testkit_law_quarantine_discard_preserves_state_via_real_store() {
        let mut store = fresh_demo_store();
        store.set_merge_policy(crate::os_spr::MergePolicy::Normal);
        let (delete, modify) = modify_vs_delete_envelopes();
        store.ingest_remote(delete).expect("delete applies cleanly");
        let pre_discard = store.snapshot().expect("pre-discard snapshot");
        let reject_report = store.ingest_remote(modify.clone()).expect("reject is a report");
        let conflict_id = reject_report.conflict.expect("conflict raised");
        store.resolve_conflict(&conflict_id.0, crate::os_spr::ConflictResolution::Discard).expect("discard");
        let post_discard = store.snapshot().expect("post-discard snapshot");
        // `relayed`: every edit id this store's persisted history (`applied_edit_ids`) could ever
        // ship onward via `flush_outbound`/`snapshot_pack` — a discarded batch is only `seed_
        // applied` on the dag, never added to `applied_edit_ids`/`vcs.edits`, so it can never appear
        // here; this is the real set flush_outbound draws from, not a fabricated stand-in.
        let relayed = store.applied_edit_ids().to_vec();
        crate::os_spr::testkit::assert_quarantine_discard_preserves_state(&pre_discard, &post_discard, &[modify.mutation_id.0.clone()], &relayed);
    }

    #[test]
    async fn testkit_law_ledger_matches_replay_via_real_store() {
        let (delete, modify) = modify_vs_delete_envelopes();
        let modify_edit_id = modify.mutation_id.0.clone();

        let mut first = fresh_demo_store();
        first.set_merge_policy(crate::os_spr::MergePolicy::LaissezFaire);
        first.ingest_remote(delete.clone()).expect("delete applies cleanly");
        first.ingest_remote(modify.clone()).expect("modify applies under LaissezFaire");

        let mut replay = fresh_demo_store();
        replay.set_merge_policy(crate::os_spr::MergePolicy::LaissezFaire);
        replay.ingest_remote(delete).expect("delete applies cleanly");
        replay.ingest_remote(modify).expect("modify applies under LaissezFaire");

        let mut ledger = HashMap::new();
        ledger.insert(modify_edit_id.clone(), first.messages_for_edit(&modify_edit_id).to_vec());
        let mut replayed = HashMap::new();
        replayed.insert(modify_edit_id.clone(), replay.messages_for_edit(&modify_edit_id).to_vec());
        assert!(!ledger[&modify_edit_id].is_empty(), "the modify edit must have raised a message for this law to be meaningful");

        crate::os_spr::testkit::assert_ledger_matches_replay(&ledger, &replayed);
    }

    #[test]
    async fn testkit_law_conflict_spr_round_trip_via_real_store() {
        let mut store = fresh_demo_store();
        store.set_merge_policy(crate::os_spr::MergePolicy::Normal);
        let (delete, modify) = modify_vs_delete_envelopes();
        store.ingest_remote(delete).expect("delete applies cleanly");
        let report = store.ingest_remote(modify).expect("reject is a report");
        let conflict_id = report.conflict.expect("conflict raised");
        let conflict = store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("conflict recorded").clone();

        let pack_bytes = DemoSnapshot { n: 0 }.encode_pack();
        let base_envelope = store.envelope().clone();
        let encode = |conflict: &crate::os_spr::Conflict| -> Vec<u8> {
            let mut envelope = base_envelope.clone();
            envelope.conflicts = vec![conflict.clone()];
            print_document_spr(&envelope).expect("encode conflict via the real .spr codec")
        };
        let decode = |bytes: &[u8]| -> crate::os_spr::Conflict {
            let parsed = parse_document_spr::<DemoSnapshot, DemoMutation>(&pack_bytes, bytes).expect("decode conflict via the real .spr codec");
            parsed.envelope.conflicts.into_iter().next().expect("one conflict round-tripped")
        };
        crate::os_spr::testkit::assert_conflict_spr_round_trip(&conflict, encode, decode);
    }
    //#endregion 🔖️TestkitLawWiring
    //#endregion 🔖️MergePolicyTests

    //#region 🔖️RobustnessTests
    // 🎯️ `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` J1 — security/
    // robustness audit findings HIGH-1, HIGH-2, MEDIUM-3, MEDIUM-4 (`📓️j1-robustness-fixes.md`).
    // Every test here FAILS without its corresponding fix.

    #[test]
    async fn replay_suffix_partitioned_errors_loudly_on_a_ghost_edit_id_instead_of_silently_dropping_it() {
        // HIGH-1: an id named in `order[k..]` but absent from `edits` means `applied_edit_ids`/
        // `vcs.edits` fell out of sync (crash/recovery, a partially-applied ingest) — a bare
        // `continue` here used to silently compute a WRONG snapshot instead of failing loudly.
        let edits: HashMap<String, Edit<DemoMutation>> = HashMap::new();
        let order = vec!["ghost-edit".to_string()];
        let error = super::ArtifactStore::<DemoSnapshot, DemoMutation>::replay_suffix_partitioned(&DemoSnapshot { n: 0 }, &order, 0, &edits, crate::os_spr::MergePolicy::Normal).expect_err("a ghost edit id must be a loud, typed VcsError");
        assert_eq!(error, VcsError::UnknownEdit("ghost-edit".into()));
    }

    #[test]
    async fn replay_suffix_errors_loudly_on_a_ghost_edit_id_instead_of_silently_dropping_it() {
        // HIGH-1, sibling function — `replay_suffix` is `merge_remote_snapshot`'s own replay
        // primitive and shares the exact same bare-`continue` defect before this fix.
        let edits: HashMap<String, Edit<DemoMutation>> = HashMap::new();
        let order = vec!["ghost-edit".to_string()];
        let error = super::ArtifactStore::<DemoSnapshot, DemoMutation>::replay_suffix(&DemoSnapshot { n: 0 }, &order, 0, &edits).expect_err("a ghost edit id must be a loud, typed VcsError");
        assert_eq!(error, VcsError::UnknownEdit("ghost-edit".into()));
    }

    #[test]
    async fn edits_for_ids_errors_loudly_on_a_ghost_edit_id_instead_of_silently_filtering_it() {
        // HIGH-2: minting a `ConflictId` from a `filter_map` that silently drops missing ids can
        // (if every id is missing) hash an EMPTY mutation-id set into a content-addressed conflict
        // id — a content address that addresses no content, so unrelated conflicts can collide on
        // it. `edits_for_ids` is the strict replacement both `ingest_remote` mint sites now use.
        let edits: HashMap<String, Edit<DemoMutation>> = HashMap::new();
        let error = super::ArtifactStore::<DemoSnapshot, DemoMutation>::edits_for_ids(&["ghost-edit".to_string()], &edits).expect_err("an id that resolves to nothing must fail loudly, never vanish from the mutation-id set");
        assert_eq!(error, VcsError::UnknownEdit("ghost-edit".into()));
    }

    /// 🛰️ A synthetic `Open` conflict for capacity/pruning fixtures — content doesn't matter, only
    /// that its `ConflictId`/timestamp are distinct per `seed` and it never gets touched by real
    /// replay logic (these tests push it directly onto `envelope.conflicts`, bypassing `ingest_
    /// remote`/`resolve_conflict` entirely, exactly like the existing hand-built conflict fixtures
    /// in `🔖️PreviewWireTests` above).
    async fn synthetic_open_conflict(seed: u64) -> crate::os_spr::Conflict {
        let kind = crate::os_spr::ConflictKind::Degraded { edit_ids: vec![format!("synthetic-edit-{seed}")] };
        let artifact = ArtifactId("demo".into());
        let mutation_ids = vec![MutationId(format!("synthetic-op-{seed}"))];
        let timestamp = HybridLogicalTimestamp::new(seed, seed);
        crate::os_spr::Conflict {
            id: crate::os_spr::ConflictId::new(&kind, &artifact, &mutation_ids, &timestamp),
            kind,
            status: crate::os_spr::ConflictStatus::Open,
            messages: Vec::new(),
            actors: vec![ActorId(format!("synthetic-actor-{seed}"))],
            timestamp,
        }
    }

    #[test]
    async fn ingest_remote_refuses_a_new_open_conflict_once_the_backlog_is_at_capacity() {
        // MEDIUM-3: a peer that keeps sending a batch this replica keeps quarantining can grow
        // `envelope.conflicts` without bound (the dag never advances on quarantine, so the SAME
        // envelope is eligible for redelivery forever). Hitting the open-conflict cap must be a
        // loud, typed refusal — atomic, nothing applied — never a silent drop or overwrite.
        let mut store = fresh_demo_store();
        store.set_merge_policy(crate::os_spr::MergePolicy::Normal);
        let (delete, modify) = modify_vs_delete_envelopes();
        store.ingest_remote(delete).expect("the delete alone raises no message and always applies");

        let cap = super::ArtifactStore::<DemoSnapshot, DemoMutation>::OPEN_CONFLICT_CAP;
        for seed in 0..cap as u64 {
            store.0.envelope.conflicts.push(synthetic_open_conflict(seed));
        }
        assert_eq!(store.open_conflicts().count(), cap, "fixture must actually be at capacity for this test to be meaningful");

        let before = store.envelope().clone();
        let before_snapshot = store.snapshot().expect("pre-attempt snapshot");
        let before_ids = store.applied_edit_ids().to_vec();

        let error = store.ingest_remote(modify).expect_err("minting one more Open conflict past the cap must be a loud, typed refusal");
        assert!(matches!(&error, VcsError::ValidationFailed(message) if message.contains("capacity")), "got {error:?}");
        assert_eq!(store.envelope(), &before, "a refused mint must be fully atomic — nothing about the attempted batch applied");
        assert_eq!(store.snapshot().expect("snapshot"), before_snapshot);
        assert_eq!(store.applied_edit_ids(), before_ids.as_slice());
        assert_eq!(store.open_conflicts().count(), cap, "the backlog must stay exactly at capacity, never silently grow past it");
    }

    #[test]
    async fn resolved_conflicts_are_pruned_oldest_first_once_the_ledger_exceeds_its_cap_while_open_ones_survive() {
        // MEDIUM-3: resolved (`Accepted`/`Discarded`) conflicts are closed historical facts, so
        // unlike `Open` ones they are prunable — oldest push order evicted first, capped, while an
        // `Open` conflict is never touched no matter how far over cap the resolved backlog grows.
        let mut store = fresh_demo_store();
        let cap = super::ArtifactStore::<DemoSnapshot, DemoMutation>::RESOLVED_CONFLICT_CAP;
        let open = synthetic_open_conflict(u64::MAX);
        store.0.envelope.conflicts.push(open.clone());
        let overflow: u64 = 20;
        let resolved_count = cap as u64 + overflow;
        for seed in 0..resolved_count {
            let mut resolved = synthetic_open_conflict(seed);
            resolved.status = crate::os_spr::ConflictStatus::Accepted;
            store.0.envelope.conflicts.push(resolved);
        }
        assert_eq!(store.conflicts().len(), resolved_count as usize + 1);

        store.0.prune_resolved_conflicts();

        assert_eq!(store.conflicts().len(), cap, "pruning must bring the ledger back down to the cap");
        assert!(store.conflicts().iter().any(|conflict| conflict.id == open.id && conflict.status == crate::os_spr::ConflictStatus::Open), "the Open conflict must never be evicted, no matter how far over cap the resolved backlog grows");
        let surviving_seeds: HashSet<u64> = store.conflicts().iter().filter(|conflict| conflict.status == crate::os_spr::ConflictStatus::Accepted).map(|conflict| conflict.timestamp.physical_ms).collect();
        assert_eq!(surviving_seeds.len(), cap - 1);
        for evicted_seed in 0..=overflow {
            assert!(!surviving_seeds.contains(&evicted_seed), "seed {evicted_seed} was among the oldest resolved conflicts and must be pruned first");
        }
        assert!(surviving_seeds.contains(&(resolved_count - 1)), "the newest resolved conflict must survive pruning");
    }

    #[test]
    async fn quarantine_message_clearing_is_correct_for_a_mixed_new_and_retroactive_batch() {
        // MEDIUM-4: one `ingest_remote` batch that quarantines BOTH a brand-new edit (`op-c`, never
        // committed before) AND a previously-committed edit a rewind now retroactively invalidates
        // (`op-a`, which carried a REAL non-empty `mutation.cascade` message from its first commit)
        // — proving `replace_edit_messages(.., empty)` clears the stale ledger entry correctly even
        // when it runs in the same pass as a never-populated one, regardless of which kind an id is.
        let mut store = fresh_demo_store();
        store.set_merge_policy(crate::os_spr::MergePolicy::Normal);

        let a = mutation_envelope_at("actor-a", "op-a", DemoMutation::BumpN { delta: 5 }, HybridLogicalTimestamp::new(1, 100), Vec::new());
        store.ingest_remote(a).expect("op-a commits cleanly the first time, on a fresh n=0 target");
        assert!(!store.messages_for_edit("op-a").is_empty(), "fixture must carry real prior ledger content for this test to be meaningful");
        assert_eq!(store.snapshot().expect("snapshot"), DemoSnapshot { n: 5 });

        // `op-b` (earlier HLC than `op-a`) forces a rewind that replays `op-a` against a DELETED
        // target the second time around. `op-c` (later HLC, brand new) is submitted FIRST but
        // depends on `op-b`, so it buffers in the dag and is released alongside `op-b` in the SAME
        // `ingest_remote` call/batch — the mixed-kind scenario MEDIUM-4 asks for.
        let b = mutation_envelope_at("actor-b", "op-b", DemoMutation::DeleteN, HybridLogicalTimestamp::new(1, 50), Vec::new());
        let c = mutation_envelope_at("actor-c", "op-c", DemoMutation::BumpN { delta: 1 }, HybridLogicalTimestamp::new(2, 150), vec![MutationId("op-b".into())]);
        store.ingest_remote(c).expect("op-c buffers behind its unmet dependency on op-b");
        let report = store.ingest_remote(b).expect("op-b's arrival releases op-b and op-c into the same batch");

        assert!(!report.accepted, "op-c never committed");
        assert_eq!(store.applied_edit_ids(), &["op-b".to_string()], "only op-b committed — op-a dropped out on retroactive invalidation, op-c never entered");
        assert_eq!(store.snapshot().expect("snapshot"), DemoSnapshot { n: i32::MIN });

        assert!(store.messages_for_edit("op-a").is_empty(), "op-a's stale non-empty ledger entry must be cleared, not left stale, once it is retroactively quarantined");
        assert!(store.messages_for_edit("op-c").is_empty(), "op-c never committed, so it must never gain a ledger entry");
        assert!(store.envelope().edit_messages.iter().all(|entry| entry.edit_id != "op-a"), "a cleared entry must be fully removed from the durable ledger, not left behind as an empty Vec");
        assert!(store.envelope().edit_messages.iter().all(|entry| entry.edit_id != "op-c"), "op-c must never appear in the durable ledger at all");

        let conflict_id = report.conflict.expect("the mixed quarantine batch must raise one conflict");
        let conflict = store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("conflict recorded on the store");
        assert_eq!(conflict.status, crate::os_spr::ConflictStatus::Open);
        match &conflict.kind {
            crate::os_spr::ConflictKind::Quarantined { envelopes } => assert_eq!(envelopes.len(), 2, "both op-a (retroactive) and op-c (new) must be quarantined TOGETHER in one conflict"),
            other => panic!("expected a Quarantined conflict, got {other:?}"),
        }
        assert_eq!(store.conflicts().len(), 1, "op-a's clean first commit never raised a conflict of its own — only this one mixed-batch conflict must exist");
    }
    //#endregion 🔖️RobustnessTests

    #[test]
    async fn composition_pins_rederive_checkpoint_identity_without_partial_mutation() {
        let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: 0 }, None));
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("checkpoint".into()), authors: Vec::new() }).expect("checkpoint");
        let original_checkpoint_id = store.envelope().vcs.checkpoints[0].id.clone();
        let before = store.envelope().clone();
        let invalid = crate::os_vcs::CompositionPin { child_ref: crate::os_io::ArtifactRef { artifact_id: String::new(), dialect: demo_child_dialect() }, checkpoint_id: "child-checkpoint".into() };

        assert!(store.set_checkpoint_composition_pins(&original_checkpoint_id, vec![invalid]).is_err());
        assert_eq!(store.envelope(), &before);

        let pin = crate::os_vcs::CompositionPin { child_ref: crate::os_io::ArtifactRef { artifact_id: "child".into(), dialect: demo_child_dialect() }, checkpoint_id: "child-checkpoint".into() };
        store.set_checkpoint_composition_pins(&original_checkpoint_id, vec![pin]).expect("valid pin update");
        let rederived = &store.envelope().vcs.checkpoints[0];
        assert_ne!(rederived.id, original_checkpoint_id);
        assert_eq!(rederived.composition_pins.len(), 1);
        assert!(super::ArtifactStore::<DemoSnapshot, DemoMutation>::new(store.envelope().clone()).is_ok(), "a persisted pinned checkpoint must validate its rederived identity");
    }

    #[test]
    async fn materialize_replays_forward_mutations() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply");
        assert_eq!(store.snapshot().expect("snapshot").n, 1);
        assert_eq!(store.envelope().vcs.edits.len(), 1);
    }

    #[test]
    async fn undo_redo_round_trip() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply");
        store.dispatch(ArtifactCommand::Undo).expect("undo");
        assert_eq!(store.snapshot().expect("snapshot").n, 0);
        store.dispatch(ArtifactCommand::Redo).expect("redo");
        assert_eq!(store.snapshot().expect("snapshot").n, 1);
    }

    //#region 🔖️HistoryLaneTests
    #[test]
    async fn history_lane_defaults_to_document() {
        assert_eq!(HistoryLane::default(), HistoryLane::Document);
    }

    /// @emoji 🛤️ The design's headline acceptance case: undoing after an interleaved run of
    /// document/interaction edits reverts the last DOCUMENT edit, skipping past trailing (and even
    /// mid-history) `Interaction`-lane entries in both directions, which stay applied throughout.
    #[test]
    async fn history_lane_default_undo_and_redo_skip_interaction_entries() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply doc1");
        let doc1_id = store.applied_edit_ids()[0].clone();
        store.dispatch(ArtifactCommand::ApplyInLane { mutations: vec![DemoMutation::SetN { n: 100 }], description: None, lane: HistoryLane::Interaction }).expect("apply interaction1");
        let interaction1_id = store.applied_edit_ids()[1].clone();
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).expect("apply doc2");
        let doc2_id = store.applied_edit_ids()[2].clone();
        store.dispatch(ArtifactCommand::ApplyInLane { mutations: vec![DemoMutation::SetN { n: 200 }], description: None, lane: HistoryLane::Interaction }).expect("apply interaction2");
        let interaction2_id = store.applied_edit_ids()[3].clone();

        assert_eq!(store.envelope().lanes.get(&interaction1_id), Some(&HistoryLane::Interaction));
        assert_eq!(store.envelope().lanes.get(&interaction2_id), Some(&HistoryLane::Interaction));
        assert!(store.envelope().lanes.get(&doc1_id).is_none(), "an ordinary Document-lane edit never gets a `lanes` entry (sparse ledger)");
        assert!(store.envelope().lanes.get(&doc2_id).is_none());

        // Default undo skips the TRAILING interaction2 edit to revert doc2 instead.
        store.dispatch(ArtifactCommand::Undo).expect("undo skips interaction2 to revert doc2");
        assert_eq!(store.applied_edit_ids(), &[doc1_id.clone(), interaction1_id.clone(), interaction2_id.clone()], "doc2 removed; both interaction edits remain applied");
        assert_eq!(store.redo_edit_ids(), std::slice::from_ref(&doc2_id));

        // A second default undo reverts doc1 — the only remaining Document-lane entry — even though
        // it now sits BEFORE two still-applied interaction edits in `applied_edit_ids`.
        store.dispatch(ArtifactCommand::Undo).expect("undo doc1 despite interaction edits between it and the tail");
        assert_eq!(store.applied_edit_ids(), &[interaction1_id.clone(), interaction2_id.clone()]);
        assert_eq!(store.redo_edit_ids(), &[doc2_id.clone(), doc1_id.clone()]);

        // Default redo mirrors it: restores doc1 first (nearest Document entry in the redo stack),
        // then doc2, never touching either interaction edit's own applied/redo membership.
        store.dispatch(ArtifactCommand::Redo).expect("redo doc1");
        assert_eq!(store.applied_edit_ids(), &[interaction1_id.clone(), interaction2_id.clone(), doc1_id.clone()]);
        assert_eq!(store.redo_edit_ids(), std::slice::from_ref(&doc2_id));
        store.dispatch(ArtifactCommand::Redo).expect("redo doc2");
        assert_eq!(store.applied_edit_ids(), &[interaction1_id.clone(), interaction2_id.clone(), doc1_id.clone(), doc2_id.clone()]);
        assert!(store.redo_edit_ids().is_empty());
    }

    /// @emoji 🛤️ The completing half of the mechanism: `UndoInLane`/`RedoInLane` walk a NON-`Document`
    /// lane explicitly and independently of the document lane's own cursor position.
    #[test]
    async fn history_lane_undo_in_lane_and_redo_in_lane_walk_only_the_requested_lane() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply doc");
        store.dispatch(ArtifactCommand::ApplyInLane { mutations: vec![DemoMutation::SetN { n: 99 }], description: None, lane: HistoryLane::Interaction }).expect("apply interaction");
        let doc_id = store.applied_edit_ids()[0].clone();
        let interaction_id = store.applied_edit_ids()[1].clone();

        // Explicit lane-scoped undo reverts ONLY the interaction edit, leaving the document edit
        // applied — the mirror image of default `Undo` skipping it.
        store.dispatch(ArtifactCommand::UndoInLane { lane: HistoryLane::Interaction }).expect("undo in interaction lane");
        assert_eq!(store.applied_edit_ids(), std::slice::from_ref(&doc_id));
        assert_eq!(store.redo_edit_ids(), std::slice::from_ref(&interaction_id));
        assert_eq!(store.snapshot().expect("snapshot").n, 1, "reverting the interaction edit restores the document edit's own value");

        // Redoing the Document lane from here has nothing to redo — only the Interaction lane's
        // cursor moved, proving the two lanes' redo stacks are independent, not one shared position.
        assert_eq!(store.dispatch(ArtifactCommand::RedoInLane { lane: HistoryLane::Document }).unwrap_err(), VcsError::NothingToRedo);

        store.dispatch(ArtifactCommand::RedoInLane { lane: HistoryLane::Interaction }).expect("redo in interaction lane");
        assert_eq!(store.applied_edit_ids(), &[doc_id.clone(), interaction_id.clone()]);
        assert!(store.redo_edit_ids().is_empty());
    }

    /// @emoji 🛤️ Acceptance: a history made ENTIRELY of `Interaction`-lane edits is a no-op for
    /// default `Undo` (no `Document`-lane entry exists at all), while the lane-scoped API still
    /// reaches them.
    #[test]
    async fn history_lane_default_undo_is_a_no_op_when_every_edit_is_interaction_lane() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::ApplyInLane { mutations: vec![DemoMutation::SetN { n: 1 }], description: None, lane: HistoryLane::Interaction }).expect("apply interaction1");
        store.dispatch(ArtifactCommand::ApplyInLane { mutations: vec![DemoMutation::SetN { n: 2 }], description: None, lane: HistoryLane::Interaction }).expect("apply interaction2");
        assert_eq!(store.applied_edit_ids().len(), 2);

        let error = store.dispatch(ArtifactCommand::Undo).unwrap_err();
        assert_eq!(error, VcsError::NothingToUndo, "no Document-lane entry exists to undo; both interaction edits must stay untouched");
        assert_eq!(store.applied_edit_ids().len(), 2, "default undo must not remove either interaction edit");

        // The explicit lane-scoped API can still walk them.
        store.dispatch(ArtifactCommand::UndoInLane { lane: HistoryLane::Interaction }).expect("undo in interaction lane");
        assert_eq!(store.applied_edit_ids().len(), 1);
        assert_eq!(store.snapshot().expect("snapshot").n, 1);
    }

    /// @emoji 🛤️ `Interaction`-lane entries are ordinary persisted `Edit`s — they survive a plain
    /// JSON envelope round trip (`ArtifactStore::envelope_json`, the in-scope persistence path for
    /// this store-level mechanism; `.pack`+`.spr` reload is a follow-up, see `parse_document_spr`'s
    /// `lanes` field-construction comment), and a reloaded store's default undo still skips them.
    #[test]
    async fn history_lane_interaction_entries_survive_envelope_json_round_trip() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply doc");
        let doc_id = store.applied_edit_ids()[0].clone();
        store.dispatch(ArtifactCommand::ApplyInLane { mutations: vec![DemoMutation::SetN { n: 42 }], description: None, lane: HistoryLane::Interaction }).expect("apply interaction");
        let interaction_id = store.applied_edit_ids()[1].clone();
        assert_eq!(store.envelope().lanes.get(&interaction_id), Some(&HistoryLane::Interaction));

        let json = store.envelope_json().expect("envelope json");
        let reloaded_envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = serde_json::from_str(&json).expect("parse envelope json");
        assert_eq!(reloaded_envelope.lanes.get(&interaction_id), Some(&HistoryLane::Interaction), "lane tag must survive a plain JSON envelope round trip");

        let mut reloaded = ArtifactStore::new(reloaded_envelope);
        assert_eq!(reloaded.applied_edit_ids(), store.applied_edit_ids(), "reload seeds applied_edit_ids from the persisted cursor, same as any other edit");
        reloaded.dispatch(ArtifactCommand::Undo).expect("undo on the reloaded store still skips the interaction edit");
        assert_eq!(reloaded.applied_edit_ids(), std::slice::from_ref(&interaction_id), "the document edit was removed; the interaction edit is the only one left applied");
        assert!(reloaded.redo_edit_ids().contains(&doc_id), "the reverted document edit now sits on the redo stack");
    }
    //#endregion 🔖️HistoryLaneTests

    //#region 🔖️InteractionStoreTests
    /// @emoji 🕹️ `InteractionStore::apply` mutates the local hover value and bumps `generation`,
    /// mirroring `PresenceStore`/`TransientStore` — the same `Mutation<S>::diff().apply()` seam,
    /// reused here with the file's existing `DemoSnapshot`/`DemoMutation` fixtures standing in for
    /// an app's hover-shaped type.
    #[test]
    async fn interaction_store_apply_updates_hover_and_bumps_generation() {
        let mut store = InteractionStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: 0 });
        assert_eq!(store.generation(), 0);
        assert_eq!(store.hover().n, 0);

        store.apply(&[DemoMutation::SetN { n: 7 }]).expect("valid interaction mutation");
        assert_eq!(store.hover().n, 7, "apply routes through Mutation::diff/Diff::apply like PresenceStore");
        assert_eq!(store.generation(), 1);

        store.apply(&[]).expect("empty interaction batch");
        assert_eq!(store.generation(), 1, "an empty mutation batch must not bump generation, same as PresenceStore/TransientStore");
    }

    /// @emoji 🔄️ `reset` discards the current hover outright (a host clears hover when a
    /// view/window closes) and still bumps `generation` so a pending broadcast reflects the clear.
    #[test]
    async fn interaction_store_reset_discards_hover_and_bumps_generation() {
        let mut store = InteractionStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: 0 });
        store.apply(&[DemoMutation::SetN { n: 3 }]).expect("valid interaction mutation");
        assert_eq!(store.generation(), 1);

        store.reset(DemoSnapshot { n: 0 });
        assert_eq!(store.hover().n, 0);
        assert_eq!(store.generation(), 2, "reset bumps generation even though the value returns to default");
    }

    /// @emoji 🏗️ `Default` seeds from `S::default()`, same convention as `PresenceStore`/`TransientStore`.
    #[test]
    async fn interaction_store_default_seeds_from_hover_default() {
        let store = InteractionStore::<DemoSnapshot, DemoMutation>::default();
        assert_eq!(store.hover().n, 0);
        assert_eq!(store.generation(), 0);
    }
    //#endregion 🔖️InteractionStoreTests

    #[test]
    async fn apply_computes_backwards_from_pre_state() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 5 }], description: None }).expect("apply");
        let edit = &store.envelope().vcs.edits[0];
        assert_eq!(edit.inverse, vec![DemoMutation::SetN { n: 0 }]);
    }

    #[test]
    async fn commit_checkpoint_wraps_edits_into_change() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("init".into()), authors: vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }] }).expect("commit");
        assert_eq!(store.envelope().vcs.changes.len(), 1);
        assert_eq!(store.envelope().vcs.checkpoints.len(), 1);
        assert_eq!(store.envelope().vcs.checkpoints[0].message, Some("init".into()));
    }

    #[test]
    async fn checkout_checkpoint_restores_applied_edits() {
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
    async fn alternatives_switch_restores_checkpoint_chain() {
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
    async fn checkout_old_checkpoint_then_commit_creates_a_fork() {
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
    async fn create_alternative_appends_commits_to_its_own_checkpoint_chain() {
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
    async fn history_columns_orders_newest_first_and_labels_trunk_root() {
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
    async fn history_columns_assigns_distinct_lanes_and_pulls_main_only_descendants_to_trunk() {
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
    async fn backbone_message_binary_round_trips_every_variant() {
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

    async fn sample_envelope_for_backbone_test() -> crate::os_spr::MutationEnvelope {
        crate::os_spr::MutationEnvelope {
            mutation_id: MutationId("op-1".to_string()),
            document_id: ArtifactId("doc-1".to_string()),
            actor: ActorId("actor-1".to_string()),
            dependencies: Vec::new(),
            diff: crate::os_spr::ArtifactDiff { schema: SchemaId("demo/v1".to_string()), payload: vec![1, 2, 3] },
            inverse: crate::os_spr::InverseMutation { schema: SchemaId("demo/v1".to_string()), payload: Vec::new() },
            timestamp: HybridLogicalTimestamp { actor: 0, physical_ms: 0, logical: 0 },
        }
    }

    #[test]
    async fn no_backbone_by_default() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        assert!(envelope.backbone.is_none(), "a fresh document has no attached backbone");
        let store = ArtifactStore::new(envelope);
        assert!(store.backbone_ref().is_none());
    }

    #[test]
    async fn memory_backbone_pair_propagates_edits_bidirectionally() {
        let (backbone_a, backbone_b) = MemoryBackbone::pair("peer-a", "peer-b");
        let envelope_a: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let envelope_b: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store_a = ArtifactStore::new(envelope_a);
        let mut store_b = ArtifactStore::new(envelope_b);
        store_a.attach_backbone(Backbones::Memory(backbone_a)).expect("attach a");
        store_b.attach_backbone(Backbones::Memory(backbone_b)).expect("attach b");

        store_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply on a");
        store_b.tick().expect("tick b");
        assert_eq!(store_b.snapshot().expect("snapshot b").n, 1, "b receives a's edit");

        store_b.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).expect("apply on b");
        store_a.tick().expect("tick a");
        assert_eq!(store_a.snapshot().expect("snapshot a").n, 2, "a receives b's edit");
    }

    #[test]
    async fn detach_backbone_stops_synchronizing_but_keeps_the_wip_graph() {
        let (backbone_a, backbone_b) = MemoryBackbone::pair("peer-a", "peer-b");
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store_a = ArtifactStore::new(envelope.clone());
        let mut store_b = ArtifactStore::new(envelope);
        store_a.attach_backbone(Backbones::Memory(backbone_a)).expect("attach a");
        store_b.attach_backbone(Backbones::Memory(backbone_b)).expect("attach b");
        store_a.detach_backbone();
        assert!(store_a.backbone_ref().is_none());

        store_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 9 }], description: None }).expect("apply after detach still works on the in-memory graph");
        assert_eq!(store_a.snapshot().expect("snapshot a").n, 9);
        store_b.tick().expect("tick b");
        assert_eq!(store_b.snapshot().expect("snapshot b").n, 0, "detached edits never reach the peer");
    }

    #[test]
    async fn deserialized_envelope_with_stale_backbone_ref_never_auto_attaches() {
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

    #[test]
    async fn document_codec_of_round_trips_dsl_and_pack_and_edit_text() {
        let codec = ArtifactCodec::of::<DemoSnapshot, DemoMutation>("test.document-codec-roundtrip/v1");
        assert_eq!(codec.schema, "test.document-codec-roundtrip/v1");
        assert_eq!(codec.extension, "demo.doc");

        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("test.document-codec-roundtrip/v1", "demo", DemoSnapshot { n: 4 }, None);
        let text_files = print_document_text(&envelope).expect("print document text");

        let (pack_files, dsl_mirror) = (codec.compile_dsl)(&text_files.dsl, &text_files.ops).expect("codec compile_dsl");
        assert_eq!(dsl_mirror, DemoSnapshot { n: 4 }.print_dsl(), "dsl mirror matches the initial snapshot's print_dsl");

        let mirrored = (codec.print_mirror)(&pack_files.pack, &pack_files.spr).expect("codec print_mirror");
        assert_eq!(mirrored.dsl, dsl_mirror, "print_mirror's dsl text agrees with compile_dsl's own mirror, no JSON round trip");

        let document_id = ArtifactId("demo".to_string());
        let schema = SchemaId("test.document-codec-roundtrip/v1".to_string());
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

        preflight_document_codecs(std::slice::from_ref(&codec)).expect("preflight accepts an unclaimed full descriptor without publishing it");
        assert!(document_codec("test.document-codec-roundtrip/v1").expect("registry availability").is_none(), "preflight must not publish a codec");
        let _ = register_document_codec(codec).expect("first document codec registration");
        assert!(document_codec("test.document-codec-roundtrip/v1").expect("registry availability").is_some(), "registered codec is discoverable by schema string");
        assert!(document_codec("no-such-schema").expect("registry availability").is_none());
    }

    #[test]
    async fn register_document_codec_rejects_a_duplicate_schema_without_replacing_the_first() {
        let first = ArtifactCodec::of::<DemoSnapshot, DemoMutation>("test.duplicate-id-probe/v1");
        let second = ArtifactCodec { pack_schema_hash: [7u8; 32], ..first.clone() };
        assert_ne!(first.pack_schema_hash, second.pack_schema_hash, "fixture precondition: the two codecs must be distinguishable");

        let _ = register_document_codec(first.clone()).expect("first registration");
        let _ = register_document_codec(first.clone()).expect("an identical descriptor and executable is idempotent");
        let conflict = match register_document_codec(second).expect_err("a schema collision must reject rather than replace") {
            DocumentCodecRegistryError::Conflict(conflict) => conflict,
            DocumentCodecRegistryError::Unavailable => panic!("document codec registry unavailable"),
        };
        assert_eq!(conflict.schema, "test.duplicate-id-probe/v1");
        let resolved = document_codec("test.duplicate-id-probe/v1").expect("registry availability").expect("still registered after the second call");
        assert_eq!(resolved.pack_schema_hash, first.pack_schema_hash, "the first codec remains authoritative after a conflict");
    }

    #[test]
    async fn dialect_migration_preflight_and_batch_commit_are_conflict_free_or_noop() {
        async fn append_marker(bytes: &[u8]) -> Result<Vec<u8>, String> {
            Ok([bytes, b"-migrated"].concat())
        }

        let from = crate::os_io::ArtifactDialect { artifact_kind: "test.runtime-migration".into(), standard: "1".into(), subset: "*".into() };
        let to = crate::os_io::ArtifactDialect { artifact_kind: "test.runtime-migration".into(), standard: "2".into(), subset: "*".into() };
        let migration = DialectMigration { from: from.clone(), to: to.clone(), lossless: true, migrate_pack: append_marker };
        preflight_dialect_migrations(std::slice::from_ref(&migration)).expect("preflight accepts an unclaimed dialect pair without mutation");
        assert!(matches!(migrate_document(&from, &to, b"seed"), Err(DialectMigrationError::Missing { .. })), "preflight must not publish a migration");
        register_dialect_migrations(vec![migration.clone()]).expect("batch migration registration");
        assert_eq!(migrate_document(&from, &to, b"seed").expect("registered migration"), b"seed-migrated");

        let conflict = DialectMigration { lossless: false, ..migration };
        assert!(matches!(preflight_dialect_migrations(std::slice::from_ref(&conflict)), Err(DialectMigrationRegistryError::Conflict { .. })), "preflight must reject a descriptor change before batch commit");
        assert_eq!(migrate_document(&from, &to, b"seed").expect("first migration remains authoritative"), b"seed-migrated");
    }

    async fn projection_probe(store: &ArtifactStore<DemoSnapshot, DemoMutation>, cause: ArtifactProjectionCause) -> ArtifactProjectionResult<i32, DemoDiff> {
        let event = store.projection_event(cause, Some(DemoSnapshot { n: -1 }), ArtifactProjectionCacheMode::ValidatePrevious, "deterministic").expect("capture projection event");
        assert_eq!(event.previous, Some(DemoSnapshot { n: -1 }));
        assert_eq!(event.cache_mode, ArtifactProjectionCacheMode::ValidatePrevious);
        event.result(event.state.n, None)
    }

    async fn assert_projection_is_stale(store: &ArtifactStore<DemoSnapshot, DemoMutation>, result: ArtifactProjectionResult<i32, DemoDiff>) {
        assert!(matches!(store.accept_projection_result(result), Err(StaleArtifactProjection { .. })), "a result for an older projection stamp must be rejected");
    }

    #[test]
    async fn projection_result_gate_rejects_results_after_every_invalidating_store_transition() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "projection", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);

        let before_apply = projection_probe(&store, ArtifactProjectionCause::Apply);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply");
        assert_projection_is_stale(&store, before_apply);

        let before_undo = projection_probe(&store, ArtifactProjectionCause::Undo);
        store.dispatch(ArtifactCommand::Undo).expect("undo");
        assert_projection_is_stale(&store, before_undo);

        let before_redo = projection_probe(&store, ArtifactProjectionCause::Redo);
        store.dispatch(ArtifactCommand::Redo).expect("redo");
        assert_projection_is_stale(&store, before_redo);

        let before_remote = projection_probe(&store, ArtifactProjectionCause::RemoteIngest);
        store.dispatch(ArtifactCommand::IngestRemote { envelope: foreign_mutation_envelope("projection-peer", DemoMutation::SetN { n: 2 }) }).expect("remote ingest");
        assert_projection_is_stale(&store, before_remote);

        let before_reset = projection_probe(&store, ArtifactProjectionCause::Reset);
        let reset_envelope = store.envelope().clone();
        let reset_applied = store.applied_edit_ids().to_vec();
        let reset_redo = store.redo_edit_ids().to_vec();
        store.reset(reset_envelope, reset_applied, reset_redo).expect("reset");
        assert_projection_is_stale(&store, before_reset);

        store.dispatch(ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }).expect("checkpoint");
        let checkpoint_id = store.current_checkpoint_id().expect("checkpoint id").to_string();
        let before_checkout = projection_probe(&store, ArtifactProjectionCause::Checkout);
        store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id }).expect("checkout");
        assert_projection_is_stale(&store, before_checkout);
    }

    #[test]
    async fn projection_result_gate_rejects_results_after_dependency_and_checkpoint_transitions() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "projection-dependencies", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);

        let before_replay = projection_probe(&store, ArtifactProjectionCause::Replay);
        assert_eq!(store.invalidate_after_replay().cause, ArtifactProjectionCause::Replay);
        assert_projection_is_stale(&store, before_replay);

        let before_policy = projection_probe(&store, ArtifactProjectionCause::PolicyChange);
        assert_eq!(store.invalidate_after_policy_change().cause, ArtifactProjectionCause::PolicyChange);
        assert_projection_is_stale(&store, before_policy);

        let before_resource = projection_probe(&store, ArtifactProjectionCause::ExternalResourceChange);
        assert_eq!(store.invalidate_after_external_resource_change().cause, ArtifactProjectionCause::ExternalResourceChange);
        assert_projection_is_stale(&store, before_resource);

        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply before checkpoint");
        let before_checkpoint = projection_probe(&store, ArtifactProjectionCause::Checkpoint);
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }).expect("non-empty checkpoint");
        assert_projection_is_stale(&store, before_checkpoint);
        assert_eq!(store.last_projection_invalidation().expect("checkpoint invalidation").cause, ArtifactProjectionCause::Checkpoint);

        let generation = store.generation();
        let error = store.dispatch(ArtifactCommand::PruneDrafts).expect_err("draft pruning is explicitly unavailable");
        assert!(matches!(error, VcsError::ValidationFailed(_)));
        assert_eq!(store.generation(), generation, "a rejected prune cannot invalidate or report a success");
    }

    #[test]
    // 🎞️ The final paragraph this test used to have (`ValidatedMutation`/`Mutation::validate`
    // rejecting `n < 0` before persisting) is gone — `Mutation::validate` is deleted (§C4/C10); the
    // real outcome-messages/`MergePolicy` rejection this replaces is lane 1-A's C6 work.
    async fn reset_and_apply_reject_malformed_history_before_persisting() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "reset-invalid", DemoSnapshot { n: 0 }, None);
        let mut malformed_constructor = envelope.clone();
        malformed_constructor.cursor = Some(ArtifactCursor { applied_edit_ids: vec!["missing".into()], redo_edit_ids: Vec::new(), checkpoint_id: None });
        assert!(matches!(super::ArtifactStore::new(malformed_constructor), Err(VcsError::UnknownEdit(id)) if id == "missing"), "construction must reject malformed cursor history before any mutation applies");

        let mut legacy_seed = super::ArtifactStore::new(envelope.clone()).expect("valid seed history");
        legacy_seed.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 3 }], description: None }).expect("seed edit");
        let mut cursorless_history = legacy_seed.envelope().clone();
        cursorless_history.cursor = None;
        assert_eq!(super::ArtifactStore::new(cursorless_history.clone()).expect("cursorless authoritative history is validated and folded").snapshot().expect("cursorless snapshot"), DemoSnapshot { n: 3 });
        cursorless_history.vcs.edits.push(cursorless_history.vcs.edits[0].clone());
        assert!(matches!(super::ArtifactStore::new(cursorless_history), Err(VcsError::ValidationFailed(message)) if message.contains("repeats authoritative edit")), "duplicate authoritative edits cannot be hidden by first-match replay");

        let mut store = ArtifactStore::new(envelope.clone());
        let generation = store.generation();
        assert!(matches!(store.reset(envelope, vec!["missing".into()], Vec::new()), Err(VcsError::UnknownEdit(id)) if id == "missing"));
        assert_eq!(store.generation(), generation, "failed reset must preserve the live store");
    }

    #[test]
    async fn attach_reconciles_a_pushed_snapshot() {
        let (channel, remote) = ChannelBackbone::pair("chan");
        let seeded: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut seed_store = ArtifactStore::new(seeded);
        seed_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 5 }], description: None }).expect("apply");
        let seed_files = seed_store.snapshot_pack().expect("seed snapshot");
        remote.push(BackboneMessage::Snapshot { pack: seed_files.pack, spr: seed_files.spr }).expect("push snapshot");

        let fresh: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(fresh);
        store.attach_backbone(Backbones::Channel(channel)).expect("attach reconciles the pushed snapshot");
        assert_eq!(store.snapshot().expect("snapshot").n, 5, "adopted the pushed snapshot's edit");
    }

    #[test]
    async fn channel_backbone_round_trips_between_store_and_actor() {
        let (channel, remote) = ChannelBackbone::pair("chan");
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.attach_backbone(Backbones::Channel(channel)).expect("attach");
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
    async fn pump_acks_ingested_operations() {
        let (channel, remote) = ChannelBackbone::pair("chan");
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.attach_backbone(Backbones::Channel(channel)).expect("attach");
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
    async fn exact_base_only_undo_refuses_a_foreign_tail() {
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
    async fn transform_against_concurrent_undo_skips_over_a_foreign_tail() {
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
    async fn compensating_undo_dispatches_semantic_command() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 5 }], description: None }).expect("apply");
        let undo_apply = ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 0 }], description: Some("compensate".into()) };
        store.dispatch(ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::CompensatingAction, semantic_command: Some(Box::new(undo_apply)) }).expect("compensating undo");
        assert_eq!(store.snapshot().expect("snapshot").n, 0);
    }

    #[test]
    async fn edit_mutations_exposes_the_latest_edit() {
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
    async fn amend_last_absorbs_into_matching_coalesce_key() {
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
    async fn amend_last_incremental_path_matches_full_replay_over_many_amends() {
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
    async fn amend_last_incremental_cache_survives_undo_redo_round_trip() {
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
    async fn amend_last_starts_new_edit_when_coalesce_key_differs() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN { n: 1 }], coalesce_key: Some("drag-a".into()) }).expect("first drag");
        store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN { n: 2 }], coalesce_key: Some("drag-b".into()) }).expect("second drag");
        assert_eq!(store.envelope().vcs.edits.len(), 2, "distinct gestures are separate edits");
    }

    #[test]
    async fn amend_last_does_not_absorb_into_committed_edit() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN { n: 1 }], coalesce_key: Some("drag".into()) }).expect("amend");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }).expect("commit");
        store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN { n: 2 }], coalesce_key: Some("drag".into()) }).expect("amend after commit");
        assert_eq!(store.envelope().vcs.edits.len(), 2, "committed edits are never amended, even with a matching coalesce key");
    }

    #[test]
    async fn assert_subset_harness_fidelity_and_inference_helpers() {
        test_support::assert_import_export_fidelity_bytes(b"fixture", b"fixture", test_support::IoFidelityClass::Exact);
        test_support::assert_import_export_fidelity_bytes(b"fixture", b"other", test_support::IoFidelityClass::Canonical);
        test_support::assert_inference_determinism(&7_i32, &7_i32);
    }

    #[test]
    #[should_panic(expected = "exact fidelity requires byte-identical export")]
    async fn assert_import_export_fidelity_bytes_exact_rejects_divergence() {
        test_support::assert_import_export_fidelity_bytes(b"fixture", b"other", test_support::IoFidelityClass::Exact);
    }

    #[test]
    async fn test_support_round_trip_helpers_pass_for_demo_operation() {
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
                origin: Default::default(),
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
    async fn command_envelope_round_trip_panics_on_a_lossy_operation() {
        #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
        struct LossyDiff;

        impl MutationDiff<DemoSnapshot> for LossyDiff {
            async fn apply(&self, snapshot: &DemoSnapshot) -> crate::os_spr::MutationApplyResult<DemoSnapshot> {
                Ok(snapshot.clone())
            }
            async fn absorb(&mut self, _other: Self) {}
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
            async fn diff(&self, _snapshot: &DemoSnapshot) -> crate::os_spr::MutationOutcome<LossyDiff> {
                crate::os_spr::MutationOutcome::new(LossyDiff)
            }
            async fn inverse(&self, _snapshot: &DemoSnapshot) -> Vec<Self> {
                vec![self.clone()]
            }
        }

        impl OpBinary for LossyMutation {
            async fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
                Ok(self.n.to_le_bytes().to_vec())
            }
            async fn decode_op(_bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
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
    async fn demo_dsl_round_trips() {
        test_support::assert_dsl_round_trip(&DemoSnapshot { n: 42 });
    }

    #[test]
    async fn demo_dsl_pack_equivalence() {
        test_support::assert_dsl_pack_equivalence(&DemoSnapshot { n: 42 });
    }

    #[test]
    async fn demo_op_text_round_trips() {
        test_support::assert_op_line_round_trip(&DemoMutation::SetN { n: 7 });
    }

    #[test]
    async fn demo_op_binary_round_trips_and_matches_text() {
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
    async fn demo_op_binary_rejects_unknown_format_and_ordinal() {
        let operation = DemoMutation::SetN { n: 7 };
        let mut wrong_format = operation.encode_op().expect("op encode");
        wrong_format[0] = 9;
        assert!(DemoMutation::decode_op(&wrong_format).is_err(), "format 9 must be rejected");
        let out_of_range = [pack_rt::OP_BINARY_FORMAT, 0x7E];
        assert!(DemoMutation::decode_op(&out_of_range).is_err(), "ordinal beyond declared variants must be rejected");
    }

    #[test]
    async fn print_edit_lines_emits_one_indented_line_per_forward_op() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply");
        let edit = store.envelope().vcs.edits.last().expect("edit");
        let printed = print_edit_lines(edit).expect("print edit lines");
        assert!(printed.starts_with("edit "), "got {printed:?}");
        assert!(printed.contains("\n  set-n n=1\n"));
    }

    #[test]
    async fn document_text_round_trips_after_apply_and_checkpoint() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 3 }], description: Some("bump".into()) }).expect("apply");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c1".into()), authors: vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }] }).expect("commit");
        test_support::assert_document_text_round_trip(&store);
        test_support::assert_document_pack_round_trip(&store);
    }

    #[test]
    async fn parse_document_text_rejects_invalid_op_line_with_span() {
        let files = ArtifactTextFiles { dsl: "n=0\n".to_string(), ops: "doc demo schema=demo/v1\nedit e1 sequence=1 started=\"1\"\n  not-an-op\n".to_string() };
        let error = parse_document_text::<DemoSnapshot, DemoMutation>(&files.dsl, &files.ops).unwrap_err();
        assert_eq!(error.span.line, 3);
    }

    /// @emoji 🩺️ Stresses the stateful `current`/`tail_undo_cache` fast paths — multi-op edits, amend
    /// gestures, undo/redo, and a checkpoint (cold-path recompute) all interleaved — against the
    /// full-replay differential oracle, so any divergence between the incremental paths and a
    /// from-scratch replay fails loudly here rather than surfacing as a silent snapshot bug later.
    #[test]
    async fn stateful_current_matches_full_replay_across_interleaved_commands() {
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
        async fn parse_op(line: &str) -> Result<Self, TextError> {
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
        async fn print_op(&self) -> String {
            let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
            let variants = <Self as crate::os_dsl::DslVariants>::variants();
            let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
            crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)
        }
    }

    /// 🎯️ Handcrafted OpBinary (P6).
    impl OpBinary for TimestampedMutation {
        async fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
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
        async fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
            const OP_BINARY_FORMAT: u8 = 1;
            let mut reader = crate::os_pack::ByteReader::new(bytes).await;
            let format = reader.read_u8().await?;
            if format != OP_BINARY_FORMAT {
                return Err(crate::os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
            }
            let ordinal = reader.read_varint_u64().await?;
            let variants = <Self as crate::os_dsl::DslVariants>::variants();
            let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
            let spec = spec_fn();
            let body = &bytes[reader.position().await..];
            let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
            <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "op record", offset: reader.position().await as u64, detail: error.to_string() })
        }
    }
    //#endregion 🔖️OpCodec

    impl Mutation<DemoSnapshot> for TimestampedMutation {
        type Diff = DemoDiff;

        async fn diff(&self, _snapshot: &DemoSnapshot) -> crate::os_spr::MutationOutcome<DemoDiff> {
            let diff = match self {
                TimestampedMutation::SetN { n, .. } => DemoDiff { n: Some(*n) },
            };
            crate::os_spr::MutationOutcome::new(diff)
        }

        async fn inverse(&self, snapshot: &DemoSnapshot) -> Vec<Self> {
            vec![TimestampedMutation::SetN { n: snapshot.n, physical_ms: 0 }]
        }

        async fn timestamp(&self) -> Option<HybridLogicalTimestamp> {
            match self {
                TimestampedMutation::SetN { physical_ms, .. } => Some(HybridLogicalTimestamp::new(0, *physical_ms)),
            }
        }
    }

    /// @emoji 🪄️ Downcasts a registered `dyn SpaceMember` back to its concrete demo store.
    async fn demo_member<'a, Mutation: self::Mutation<DemoSnapshot> + 'static>(host: &'a mut SpaceHost, document_id: &str) -> &'a mut ArtifactStore<DemoSnapshot, Mutation> {
        host.member_mut(document_id).expect("member registered").as_any_mut().downcast_mut::<ArtifactStore<DemoSnapshot, Mutation>>().expect("concrete member type matches")
    }

    #[test]
    async fn register_space_documents_registers_manifest_collections_and_artifacts_together() {
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

        let mut host = SpaceHost::new(create_document_envelope(&format!("{S_SPACE_HISTORY_SCHEMA}/v1"), "studio", SpaceHistorySnapshot::default(), None)).expect("valid space host history");
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
    async fn space_checkpoint_commits_dirty_members_and_pins_their_checkpoints() {
        let mut member_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "member-a", DemoSnapshot { n: 0 }, None));
        member_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply a");

        let mut member_b = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "member-b", DemoSnapshot { n: 0 }, None));
        member_b.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 5 }], description: None }).expect("apply b");
        member_b.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("b-init".into()), authors: Vec::new() }).expect("commit b upfront, so it starts clean");
        let member_b_checkpoint = member_b.current_checkpoint_id().expect("b checkpoint").to_string();

        let mut host = SpaceHost::new(create_document_envelope(&format!("{S_SPACE_HISTORY_SCHEMA}/v1"), "studio", SpaceHistorySnapshot::default(), None)).expect("valid space host history");
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
    async fn space_vcs_host_meta_document_is_backbone_attachable_and_detachable() {
        let (backbone_a, backbone_b) = MemoryBackbone::pair("studio-a", "studio-b");
        let meta_envelope: ArtifactEnvelope<SpaceHistorySnapshot, SpaceHistoryMutation> = create_document_envelope(&format!("{S_SPACE_HISTORY_SCHEMA}/v1"), "studio", SpaceHistorySnapshot::default(), None);
        let mut host_a = SpaceHost::new(meta_envelope.clone()).expect("valid first space host history");
        let mut host_b = SpaceHost::new(meta_envelope).expect("valid second space host history");
        assert!(host_a.backbone_ref().is_none(), "default is unattached, like any other ArtifactStore");

        host_a.attach_backbone(Backbones::Memory(backbone_a)).expect("attach a");
        host_b.attach_backbone(Backbones::Memory(backbone_b)).expect("attach b");
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
    async fn space_checkout_checkpoint_fans_out_and_restores_pinned_member_state() {
        let member_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "member-a", DemoSnapshot { n: 0 }, None));
        let mut host = SpaceHost::new(create_document_envelope(&format!("{S_SPACE_HISTORY_SCHEMA}/v1"), "studio", SpaceHistorySnapshot::default(), None)).expect("valid space host history");
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
    async fn space_switch_alternative_fans_out_and_restores_pinned_member_state() {
        let member_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "member-a", DemoSnapshot { n: 0 }, None));
        let mut host = SpaceHost::new(create_document_envelope(&format!("{S_SPACE_HISTORY_SCHEMA}/v1"), "studio", SpaceHistorySnapshot::default(), None)).expect("valid space host history");
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
    async fn space_undo_and_redo_target_the_member_with_the_most_recent_local_edit_by_hlt() {
        let mut member_early = ArtifactStore::new(create_document_envelope::<DemoSnapshot, TimestampedMutation>("demo-ts/v1", "member-early", DemoSnapshot { n: 0 }, None));
        member_early.dispatch(ArtifactCommand::Apply { mutations: vec![TimestampedMutation::SetN { n: 1, physical_ms: 1_000 }], description: None }).expect("apply early");

        let mut member_late = ArtifactStore::new(create_document_envelope::<DemoSnapshot, TimestampedMutation>("demo-ts/v1", "member-late", DemoSnapshot { n: 0 }, None));
        member_late.dispatch(ArtifactCommand::Apply { mutations: vec![TimestampedMutation::SetN { n: 9, physical_ms: 2_000 }], description: None }).expect("apply late");

        let mut host = SpaceHost::new(create_document_envelope(&format!("{S_SPACE_HISTORY_SCHEMA}/v1"), "studio", SpaceHistorySnapshot::default(), None)).expect("valid space host history");
        host.register_member(Box::new(member_early));
        host.register_member(Box::new(member_late));

        host.undo().expect("space undo targets the member with the higher HLT");
        assert_eq!(demo_member::<TimestampedMutation>(&mut host, "member-early").snapshot().expect("early snapshot").n, 1, "earlier local edit (lower HLT) is untouched");
        assert_eq!(demo_member::<TimestampedMutation>(&mut host, "member-late").snapshot().expect("late snapshot").n, 0, "later local edit (higher HLT) is the one undone");

        host.redo().expect("studio redo targets the most recently undone edit");
        assert_eq!(demo_member::<TimestampedMutation>(&mut host, "member-late").snapshot().expect("late snapshot after redo").n, 9, "redo restores the member's most recently undone edit");
    }

    #[test]
    // 🎞️ `Mutation::reconcile`/`reconcile_with_last`/`snapshot_with_conflicts` are all deleted
    // (26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS §C4/C6/C10) — concurrent-
    // merge arbitration is now `ingest_remote`/`resolve_conflict` against first-class `Conflict`s.
    async fn snapshot_matches_materialize_and_conflicts_stay_empty_absent_remote_ingestion() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 3 }], description: None }).expect("apply");
        let replayed = materialize_document_snapshot(store.envelope(), store.applied_edit_ids()).expect("replay");
        assert_eq!(replayed.n, 3);
        assert_eq!(store.snapshot().expect("snapshot").n, 3);
        assert!(store.conflicts().is_empty(), "no remote ingestion happened, so the store's conflict buffer stays empty");
        assert!(store.open_conflicts().next().is_none());
    }

    #[test]
    async fn space_history_op_round_trips() {
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

        let with_alternative_active = SpaceHistorySnapshot {
            alternatives: vec![
                SpaceAlternative { id: "sa-1".into(), name: "branch".into(), checkpoint_ids: vec!["sc-1".into()] },
                SpaceAlternative { id: "sa-other".into(), name: "other".into(), checkpoint_ids: vec!["sc-1".into()] },
            ],
            active_alternative_id: Some("sa-1".into()),
            ..with_checkpoint
        };
        test_support::assert_operation_round_trip(&with_alternative_active, SpaceHistoryMutation::SwitchSpaceAlternative { alternative_id: "sa-other".into() });
    }

    //#endregion 🏛️StudioTests

    //#region 🔖️PreviewWireTests
    /// @emoji 🧪️ Fixture producing one message per non-clean variant, each using one of the frozen
    /// seven `mutation.*` codes (`📋️contract-freeze.md` §C2's table) — `preview_wire`'s and
    /// `CompositionCoordinator` phase 1's shared dry-run fixture. `WarnN` ⇒ `mutation.clamped`
    /// (Warning, non-empty diff), `ErrorN` ⇒ `mutation.target-missing` (Error, empty diff — LAW 2),
    /// `FatalN` ⇒ `mutation.invariant` (Fatal, empty diff — LAW 1), `CleanN` ⇒ silent.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, crate::os_dsl::DslOps)]
    #[serde(tag = "operation")]
    enum SeverityMutation {
        #[dsl(key = "clean-n")]
        CleanN { n: i32 },
        #[dsl(key = "warn-n")]
        WarnN { n: i32 },
        #[dsl(key = "error-n")]
        ErrorN { n: i32 },
        #[dsl(key = "fatal-n")]
        FatalN { n: i32 },
    }

    /// 🎞️ Handcrafted OpText (P6).
    impl OpText for SeverityMutation {
        async fn parse_op(line: &str) -> Result<Self, TextError> {
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
        async fn print_op(&self) -> String {
            let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
            let variants = <Self as crate::os_dsl::DslVariants>::variants();
            let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
            crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)
        }
    }

    /// 🎯️ Handcrafted OpBinary (P6).
    impl OpBinary for SeverityMutation {
        async fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
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
        async fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
            const OP_BINARY_FORMAT: u8 = 1;
            let mut reader = crate::os_pack::ByteReader::new(bytes).await;
            let format = reader.read_u8().await?;
            if format != OP_BINARY_FORMAT {
                return Err(crate::os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
            }
            let ordinal = reader.read_varint_u64().await?;
            let variants = <Self as crate::os_dsl::DslVariants>::variants();
            let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
            let spec = spec_fn();
            let body = &bytes[reader.position().await..];
            let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
            <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "op record", offset: reader.position().await as u64, detail: error.to_string() })
        }
    }

    impl Mutation<DemoSnapshot> for SeverityMutation {
        type Diff = DemoDiff;
        async fn diff(&self, _snapshot: &DemoSnapshot) -> crate::os_spr::MutationOutcome<DemoDiff> {
            match self {
                SeverityMutation::CleanN { n } => crate::os_spr::MutationOutcome::new(DemoDiff { n: Some(*n) }),
                SeverityMutation::WarnN { n } => crate::os_spr::MutationOutcome::new(DemoDiff { n: Some(*n) }).warn("mutation.clamped", "n was clamped to a safe range"),
                SeverityMutation::ErrorN { .. } => crate::os_spr::MutationOutcome::error("mutation.target-missing", "target n is missing", ["n"]),
                SeverityMutation::FatalN { .. } => crate::os_spr::MutationOutcome::fatal("mutation.invariant", "n invariant violated", ["n"]),
            }
        }
        async fn inverse(&self, snapshot: &DemoSnapshot) -> Vec<Self> {
            vec![SeverityMutation::CleanN { n: snapshot.n }]
        }
    }

    /// 🛰️ Builds an explicitly stamped Severity envelope for policy and quarantine law tests.
    async fn severity_mutation_envelope_at(document_id: &str, actor: &str, mutation_id: &str, operation: SeverityMutation, timestamp: HybridLogicalTimestamp) -> crate::os_spr::MutationEnvelope {
        crate::os_spr::MutationEnvelope {
            mutation_id: MutationId(mutation_id.to_string()),
            document_id: ArtifactId(document_id.to_string()),
            actor: ActorId(actor.to_string()),
            dependencies: Vec::new(),
            diff: crate::os_spr::ArtifactDiff { schema: SchemaId("demo/v1".to_string()), payload: operation.encode_op().expect("encode severity mutation") },
            inverse: crate::os_spr::InverseMutation { schema: SchemaId("demo/v1".to_string()), payload: Vec::new() },
            timestamp,
        }
    }

    #[test]
    async fn document_text_round_trips_authoritative_metadata_messages_conflicts_and_cursor() {
        let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", "severity-text", DemoSnapshot { n: 0 }, None));
        store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::WarnN { n: 3 }], description: Some("durable warning".into()) }).expect("apply warning");
        let edit = store.envelope().vcs.edits.last().expect("one durable edit").clone();
        let messages = store.envelope().edit_messages.last().expect("durable outcome ledger").messages.clone();
        let kind = crate::os_spr::ConflictKind::Degraded { edit_ids: vec![edit.id.clone()] };
        let mutation_ids = stable_mutation_ids_for_edit(&edit).expect("stable operation identity");
        let timestamp = edit.mutation_meta.first().expect("metadata timestamp").timestamp;
        store.0.envelope.conflicts.push(crate::os_spr::Conflict {
            id: crate::os_spr::ConflictId::new(&kind, &ArtifactId(store.envelope().id.clone()), &mutation_ids, &timestamp),
            kind,
            status: crate::os_spr::ConflictStatus::Open,
            messages,
            actors: vec![ActorId(edit.actor.clone().expect("edit actor"))],
            timestamp,
        });
        let files = print_document_text(store.envelope()).expect("print full-fidelity text");
        for record in ["inverse ", "metadata ", "message ", "conflict ", "cursor "] {
            assert!(files.ops.lines().any(|line| line.starts_with(record)), "missing {record:?} record: {}", files.ops);
        }
        let parsed = parse_document_text::<DemoSnapshot, SeverityMutation>(&files.dsl, &files.ops).expect("parse full-fidelity text");
        assert_eq!(parsed.envelope, store.envelope().clone());
        assert_eq!(parsed.snapshot, store.snapshot().expect("live snapshot"));
    }

    #[test]
    async fn document_text_rejects_missing_metadata_and_unknown_cursor_without_synthesis() {
        let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", "strict-text", DemoSnapshot { n: 0 }, None));
        store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::WarnN { n: 3 }], description: None }).expect("apply warning");
        let files = print_document_text(store.envelope()).expect("print strict text");
        let missing_metadata = files.ops.lines().filter(|line| !line.starts_with("metadata ")).collect::<Vec<_>>().join("\n");
        assert!(matches!(parse_document_text::<DemoSnapshot, SeverityMutation>(&files.dsl, &missing_metadata), Err(error) if error.message.contains("no metadata records")));

        let unknown_cursor = files.ops.lines().map(|line| if line.starts_with("cursor ") { "cursor applied=[ unknown-edit ] redo=[]".to_string() } else { line.to_string() }).collect::<Vec<_>>().join("\n");
        assert!(matches!(parse_document_text::<DemoSnapshot, SeverityMutation>(&files.dsl, &unknown_cursor), Err(error) if error.message.contains("unknown or duplicate applied edit")));
    }

    #[test]
    async fn document_text_preserves_non_contiguous_edit_sequences_and_rejects_invalid_ones() {
        let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", "strict-sequence", DemoSnapshot { n: 0 }, None));
        store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::CleanN { n: 1 }], description: None }).expect("first edit");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::CleanN { n: 2 }], description: None }).expect("second edit");
        store.0.envelope.vcs.edits[0].sequence_number = 4;
        store.0.envelope.vcs.edits[1].sequence_number = 17;

        let files = print_document_text(store.envelope()).expect("print strict text");
        let parsed = parse_document_text::<DemoSnapshot, SeverityMutation>(&files.dsl, &files.ops).expect("parse strict text");
        assert_eq!(parsed.envelope.vcs.edits.iter().map(|edit| edit.sequence_number).collect::<Vec<_>>(), vec![4, 17]);

        let invalid = files.ops.replacen("sequence=4", "sequence=-1", 1);
        assert!(matches!(parse_document_text::<DemoSnapshot, SeverityMutation>(&files.dsl, &invalid), Err(error) if error.message.contains("invalid edit sequence -1")));
    }

    #[test]
    async fn persisted_conflict_requires_a_global_owned_operation_index() {
        let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", "conflict-index", DemoSnapshot { n: 0 }, None));
        store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::WarnN { n: 3 }], description: None }).expect("apply warning");
        let edit = store.envelope().vcs.edits.last().expect("one edit").clone();
        let kind = crate::os_spr::ConflictKind::Degraded { edit_ids: vec![edit.id.clone()] };
        let mutation_ids = stable_mutation_ids_for_edit(&edit).expect("stable mutation ids");
        let timestamp = edit.mutation_meta.first().expect("timestamp").timestamp;
        let mut messages = store.envelope().edit_messages.last().expect("outcome message").messages.clone();
        messages[0].op_index = Some(1);
        store.0.envelope.conflicts.push(crate::os_spr::Conflict {
            id: crate::os_spr::ConflictId::new(&kind, &ArtifactId(store.envelope().id.clone()), &mutation_ids, &timestamp),
            kind,
            status: crate::os_spr::ConflictStatus::Open,
            messages,
            actors: vec![ActorId(edit.actor.clone().expect("actor"))],
            timestamp,
        });
        assert!(matches!(validate_persisted_conflicts(store.envelope()), Err(VcsError::ValidationFailed(message)) if message.contains("operation index")));
    }

    #[test]
    async fn conflict_generation_and_validation_canonicalize_repeated_actors() {
        let document_id = "repeated-conflict-actor";
        let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", document_id, DemoSnapshot { n: 0 }, None));
        let clean = severity_mutation_envelope_at(document_id, "same-peer", "same-clean", SeverityMutation::CleanN { n: 1 }, HybridLogicalTimestamp::new(1, 100));
        let mut fatal = severity_mutation_envelope_at(document_id, "same-peer", "same-fatal", SeverityMutation::FatalN { n: 2 }, HybridLogicalTimestamp::new(2, 200));
        fatal.dependencies = vec![clean.mutation_id.clone()];

        store.ingest_remote(fatal).expect("dependent fatal is buffered");
        let report = store.ingest_remote(clean).expect("ready batch is reported");
        let conflict_id = report.conflict.expect("fatal batch is quarantined");
        assert!(!report.accepted);
        let conflict = store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("durable conflict");
        assert_eq!(conflict.actors, vec![ActorId("same-peer".into())]);
        validate_persisted_conflicts(store.envelope()).expect("generated actors use the canonical unique participant set");

        let mut malformed = store.envelope().clone();
        malformed.conflicts[0].actors.push(ActorId("same-peer".into()));
        assert!(matches!(validate_persisted_conflicts(&malformed), Err(VcsError::ValidationFailed(message)) if message.contains("malformed actor identities")));
    }

    #[test]
    async fn quarantined_accept_is_atomic_when_a_later_envelope_remains_fatal() {
        let document_id = "severity-atomic";
        let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", document_id, DemoSnapshot { n: 0 }, None));
        let clean = severity_mutation_envelope_at(document_id, "clean-peer", "clean-op", SeverityMutation::CleanN { n: 1 }, HybridLogicalTimestamp::new(1, 100));
        let fatal = severity_mutation_envelope_at(document_id, "fatal-peer", "fatal-op", SeverityMutation::FatalN { n: 2 }, HybridLogicalTimestamp::new(2, 200));
        let kind = crate::os_spr::ConflictKind::Quarantined { envelopes: vec![clean.clone(), fatal.clone()] };
        let timestamp = HybridLogicalTimestamp::new(9, 300);
        let mutation_ids = vec![clean.mutation_id.clone(), fatal.mutation_id.clone()];
        let conflict_id = crate::os_spr::ConflictId::new(&kind, &ArtifactId(document_id.to_string()), &mutation_ids, &timestamp);
        store.0.envelope.conflicts.push(crate::os_spr::Conflict {
            id: conflict_id.clone(),
            kind,
            status: crate::os_spr::ConflictStatus::Open,
            messages: vec![crate::os_spr::MutationMessage {
                level: crate::os_dsl::Severity::Fatal,
                code: crate::os_dsl::FaultCode("mutation.invariant".to_string()),
                message: "n invariant violated".to_string(),
                target: vec!["n".to_string()],
                op_index: Some(0),
            }],
            actors: vec![clean.actor.clone(), fatal.actor.clone()],
            timestamp,
        });
        validate_persisted_conflicts(store.envelope()).expect("well-formed quarantine fixture");
        let before = store.envelope().clone();
        let generation = store.generation();

        let report = store.resolve_conflict(&conflict_id.0, crate::os_spr::ConflictResolution::Accept).expect("fatal outcome is reported, not an infrastructure error");

        assert!(!report.accepted);
        assert_eq!(report.replayed.len(), 2, "the aggregate report must retain the clean and fatal replay reports");
        assert_eq!(store.envelope(), &before, "the candidate's earlier clean mutation must never leak through a later fatal rejection");
        assert_eq!(store.snapshot().expect("unchanged snapshot"), DemoSnapshot { n: 0 });
        assert_eq!(store.generation(), generation, "a rejected candidate does not invalidate the source store");
        assert_eq!(store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("original conflict retained").status, crate::os_spr::ConflictStatus::Open);
    }

    #[test]
    async fn empty_store_snapshot_policy_rejection_keeps_remote_history_quarantined() {
        let document_id = "severity-snapshot";
        let fatal = severity_mutation_envelope_at(document_id, "fatal-peer", "fatal-op", SeverityMutation::FatalN { n: 2 }, HybridLogicalTimestamp::new(2, 200));
        let mut remote = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", document_id, DemoSnapshot { n: 0 }, None));
        remote.0.envelope.vcs.edits.push(edit_from_operation_envelope::<SeverityMutation>(&fatal).expect("fatal edit"));
        remote.0.envelope.cursor = Some(ArtifactCursor { applied_edit_ids: vec![fatal.mutation_id.0.clone()], redo_edit_ids: Vec::new(), checkpoint_id: None });
        let files = remote.snapshot_pack().expect("remote snapshot");

        let mut local = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", document_id, DemoSnapshot { n: 0 }, None));
        let before = local.envelope().clone();
        assert!(matches!(local.merge_remote_snapshot(&files.pack, &files.spr), Err(VcsError::Rejected { policy: crate::os_spr::MergePolicy::Normal, .. })));
        assert_eq!(local.snapshot().expect("local content remains unchanged"), DemoSnapshot { n: 0 });
        assert!(local.envelope().vcs.edits.is_empty(), "rejected remote edits are never adopted into local history");
        assert_eq!(local.conflicts().len(), 1);
        assert_eq!(local.conflicts()[0].status, crate::os_spr::ConflictStatus::Open);
        assert_eq!(local.envelope().id, before.id);
        assert_eq!(local.envelope().schema, before.schema);
    }

    /// @emoji 🧪️ `preview_wire`'s headline law (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-
    /// FIRST-CLASS-CONFLICTS` §C6): its output is exactly what independently folding the same ops
    /// through the diff engine (`apply_mutation`, the same primitive `replay_mutations` itself calls
    /// on the real apply path) computes, stamped with `op_index`, and the live store is byte-
    /// identical before and after — a pure dry run all the way through, never applying anything even
    /// though it threads state forward internally to preview op `i` against `0..i`'s outcome.
    #[test]
    async fn preview_wire_reports_the_same_messages_the_real_apply_would_produce_and_changes_nothing() {
        let envelope: ArtifactEnvelope<DemoSnapshot, SeverityMutation> = create_document_envelope("demo/v1", "preview-demo", DemoSnapshot { n: 0 }, None);
        let store = ArtifactStore::new(envelope);
        let before = store.envelope().clone();

        let ops: Vec<Vec<u8>> = vec![
            SeverityMutation::CleanN { n: 1 }.encode_op().expect("encode clean"),
            SeverityMutation::WarnN { n: 2 }.encode_op().expect("encode warn"),
            SeverityMutation::ErrorN { n: 99 }.encode_op().expect("encode error"),
            SeverityMutation::FatalN { n: 99 }.encode_op().expect("encode fatal"),
        ];

        let messages = store.preview_wire(&ops);

        // 🧮️ Independently folds the SAME ops through the exact primitive the real apply path
        // (`replay_mutations`) itself calls (`apply_mutation`) — the ground truth `preview_wire`
        // must match. `replay_mutations` does not yet surface its own messages (lane 1-A's pending
        // C6 work, see `📓️w1-e-report.md`), so this recomputation is the closest available proof.
        let mut expected = Vec::new();
        let mut running = DemoSnapshot { n: 0 };
        for (index, op) in ops.iter().enumerate() {
            let mutation = SeverityMutation::decode_op(op).expect("decode");
            let (next, op_messages) = apply_mutation(&running, &mutation).expect("preview fixture diff applies");
            expected.extend(op_messages.into_iter().map(|message| message.at_op(index as u32)));
            running = next;
        }
        assert_eq!(messages, expected, "preview_wire must report exactly the messages the real diff engine computes");
        assert_eq!(messages.len(), 3, "one message each for warn/error/fatal; the clean op is silent");
        assert_eq!(messages[0].level, crate::os_dsl::Severity::Warning);
        assert_eq!(messages[0].op_index, Some(1));
        assert_eq!(messages[1].level, crate::os_dsl::Severity::Error);
        assert_eq!(messages[1].op_index, Some(2));
        assert_eq!(messages[2].level, crate::os_dsl::Severity::Fatal);
        assert_eq!(messages[2].op_index, Some(3));

        assert_eq!(store.envelope(), &before, "preview_wire is a pure dry run: the live store is byte-identical afterward");
        assert_eq!(store.snapshot().expect("snapshot unaffected"), DemoSnapshot { n: 0 }, "preview_wire never advances the live cursor");
    }

    #[test]
    async fn preview_wire_reports_a_fatal_message_for_undecodable_op_bytes_and_stops_there() {
        let envelope: ArtifactEnvelope<DemoSnapshot, SeverityMutation> = create_document_envelope("demo/v1", "preview-malformed", DemoSnapshot { n: 0 }, None);
        let store = ArtifactStore::new(envelope);
        let ops: Vec<Vec<u8>> = vec![SeverityMutation::CleanN { n: 1 }.encode_op().expect("encode"), vec![0xff, 0xff, 0xff]];

        let messages = store.preview_wire(&ops);

        assert_eq!(messages.len(), 1, "the clean op is silent; the malformed op reports one structural message and stops");
        assert_eq!(messages[0].level, crate::os_dsl::Severity::Fatal);
        assert_eq!(messages[0].code.0, "mutation.invariant");
        assert_eq!(messages[0].op_index, Some(1));
    }

    #[test]
    async fn spr_round_trip_preserves_edit_messages_and_conflicts() {
        let initial = DemoSnapshot { n: 0 };
        let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", "durable-outcomes", initial.clone(), None));
        store.set_merge_policy(crate::os_spr::MergePolicy::LaissezFaire);
        let receipt = store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::WarnN { n: 2 }], description: None }).expect("warning is accepted");
        let edit_id = receipt.edit_ids.first().expect("one durable edit").clone();
        let messages = store.messages_for_edit(&edit_id).to_vec();
        let edit = store.envelope().vcs.edits.iter().find(|edit| edit.id == edit_id).expect("durable edit");
        let mutation_ids = stable_mutation_ids_for_edit(edit).expect("durable edit carries operation identity");
        let actors = vec![ActorId(edit.actor.clone().expect("durable edit has an actor"))];
        let timestamp = edit.mutation_meta.first().expect("durable edit carries timestamp").timestamp;
        let kind = crate::os_spr::ConflictKind::Degraded { edit_ids: vec![edit_id.clone()] };
        let conflict_id = crate::os_spr::ConflictId::new(&kind, &ArtifactId(store.envelope().id.clone()), &mutation_ids, &timestamp);
        store.0.envelope.conflicts.push(crate::os_spr::Conflict { id: conflict_id, kind, status: crate::os_spr::ConflictStatus::Open, messages: messages.clone(), actors, timestamp });

        let pack = initial.encode_pack();
        let spr = print_document_spr(store.envelope()).expect("outcome history encodes");
        let parsed = parse_document_spr::<DemoSnapshot, SeverityMutation>(&pack, &spr).expect("outcome history decodes");
        assert_eq!(parsed.envelope.edit_messages, store.envelope().edit_messages);
        assert_eq!(parsed.envelope.conflicts, store.envelope().conflicts);

        let restored = ArtifactStore::new(parsed.envelope);
        assert_eq!(restored.messages_for_edit(&edit_id), messages);
        assert_eq!(restored.conflicts(), store.conflicts());
    }

    #[test]
    async fn spr_parse_rejects_history_without_authoritative_operation_metadata() {
        let initial = DemoSnapshot { n: 0 };
        let history = crate::os_spr::HistoryLog {
            doc_id: "metadata-required".to_string(),
            schema: "demo/v1".to_string(),
            edits: vec![crate::os_spr::HistoryEdit {
                id: "edit-1".to_string(),
                actor: Some("author".to_string()),
                started_at: String::new(),
                finished_at: None,
                coalesce_key: None,
                description: None,
                ops: vec![crate::os_spr::OpPayload { text: None, binary: Some(SeverityMutation::CleanN { n: 1 }.encode_op().expect("encode")) }],
                inverse: Vec::new(),
                meta: None,
            }],
            ..Default::default()
        };
        let spr = crate::os_spr::encode_history(&history, &crate::os_spr::EncodeOptions::default()).expect("encode fixture");
        let error = parse_document_spr::<DemoSnapshot, SeverityMutation>(&initial.encode_pack(), &spr).expect_err("authoritative history must never synthesize operation identity");
        assert!(error.message.contains("authoritative operation metadata"));
    }
    //#endregion 🔖️PreviewWireTests

    //#region 🔖️TextFormatHelpers
    #[test]
    async fn ops_author_conversion_drops_avatar_matching_the_ops_text_format() {
        let author = Author { id: "a1".into(), name: "Alice".into(), avatar: Some("http://example/a1.png".into()) };
        let round_tripped: Author = OpsAuthor::from(&author).into();
        assert_eq!(round_tripped, Author { id: "a1".into(), name: "Alice".into(), avatar: None }, "OpsAuthor never carries avatar — it is not part of the .ops text format");
    }

    #[test]
    async fn ops_header_line_checkpoint_round_trips_including_delimiter_and_quote_characters_in_authors() {
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
    async fn ops_header_line_edit_round_trips_including_a_quoted_description() {
        let header = OpsHeaderLine::Edit { id: "e1".to_string(), sequence: 42, started: "1".to_string(), actor: None, finished: None, key: None, description: Some("hello \"world\"".to_string()) };
        let printed = header.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line: {printed:?}");
        assert!(!printed.contains("actor="), "an absent optional field must be omitted: {printed}");
        let parsed = OpsHeaderLine::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed for {printed:?}: {e}"));
        assert_eq!(parsed, header, "OpsHeaderLine::Edit round trip diverged for {printed:?}");
    }

    #[test]
    async fn ops_header_line_cursor_round_trips_the_full_applied_and_redo_lists() {
        let header = OpsHeaderLine::Cursor { applied: vec!["e1".to_string(), "e3".to_string()], redo: vec!["e2".to_string()], checkpoint: Some("ck-1".to_string()) };
        let printed = header.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line: {printed:?}");
        let parsed = OpsHeaderLine::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed for {printed:?}: {e}"));
        assert_eq!(parsed, header, "OpsHeaderLine::Cursor round trip diverged for {printed:?}");
    }

    #[test]
    async fn ops_header_line_parse_op_rejects_a_line_with_no_known_keyword() {
        let error = OpsHeaderLine::parse_op("not a structural line").unwrap_err();
        assert!(error.message.contains("unknown operation line"), "got {error:?}");
    }

    #[test]
    async fn parse_document_text_rejects_a_header_line_missing_its_required_positional_id() {
        let files = ArtifactTextFiles { dsl: "n=0\n".to_string(), ops: "active\n".to_string() };
        let error = parse_document_text::<DemoSnapshot, DemoMutation>(&files.dsl, &files.ops).unwrap_err();
        assert!(error.message.contains("expected Text"), "got {error:?}");
        assert_eq!(error.span.line, 1);
    }

    #[test]
    async fn parse_document_text_rejects_an_unknown_header_line_keyword() {
        let files = ArtifactTextFiles { dsl: "n=0\n".to_string(), ops: "doc demo schema=demo/v1\nbogus id=x\n".to_string() };
        let error = parse_document_text::<DemoSnapshot, DemoMutation>(&files.dsl, &files.ops).unwrap_err();
        assert!(error.message.contains("unknown operation line"), "got {error:?}");
        assert_eq!(error.span.line, 2);
    }

    #[test]
    async fn document_text_round_trips_with_an_active_alternative_and_a_quoted_description() {
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
    async fn document_text_round_trips_a_cursor_after_undo_then_apply_interleaving() {
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
    async fn save_load_undo_proof_pack_spr_round_trip_preserves_undo_redo_position() {
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

    #[test]
    async fn document_codecs_share_complete_authoritative_history_validation() {
        let envelope = create_document_envelope("demo/v1", "codec-validation", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("checkpoint".into()), authors: Vec::new() }).expect("commit");
        let text = print_document_text(store.envelope()).expect("text fixture");
        let duplicate_change = text.ops.lines().find(|line| line.starts_with("change ")).expect("one persisted change");
        let malformed_text = format!("{}\n{duplicate_change}", text.ops);
        assert!(matches!(parse_document_text::<DemoSnapshot, DemoMutation>(&text.dsl, &malformed_text), Err(error) if error.message.contains("repeats authoritative change")));

        let pack = print_document_pack(store.envelope()).expect("binary fixture");
        let mut history = crate::os_spr::decode_history(&pack.spr, &crate::os_spr::DecodeOptions::default()).expect("decode history");
        history.changes.push(history.changes.first().expect("one persisted change").clone());
        let malformed_spr = crate::os_spr::encode_history(&history, &crate::os_spr::EncodeOptions { write_backwards_section: true, ..crate::os_spr::EncodeOptions::default() }).expect("encode malformed history");
        assert!(matches!(parse_document_spr::<DemoSnapshot, DemoMutation>(&pack.pack, &malformed_spr), Err(error) if error.message.contains("repeats authoritative change")));
    }

    //#endregion 🔖️TextFormatHelpers

    //#region 🔖️CommandErrorPaths
    #[test]
    async fn apply_with_no_mutations_is_rejected() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        let error = store.dispatch(ArtifactCommand::Apply { mutations: Vec::new(), description: None }).unwrap_err();
        assert_eq!(error, VcsError::EmptyApply);
    }

    #[test]
    async fn amend_last_with_no_mutations_is_rejected() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        let error = store.dispatch(ArtifactCommand::AmendLast { mutations: Vec::new(), coalesce_key: None }).unwrap_err();
        assert_eq!(error, VcsError::EmptyApply);
    }

    #[test]
    async fn undo_with_nothing_applied_is_rejected() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        assert_eq!(store.dispatch(ArtifactCommand::Undo).unwrap_err(), VcsError::NothingToUndo);
    }

    #[test]
    async fn redo_with_nothing_undone_is_rejected() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        assert_eq!(store.dispatch(ArtifactCommand::Redo).unwrap_err(), VcsError::NothingToRedo);
    }

    #[test]
    async fn checkout_of_an_unknown_checkpoint_is_rejected() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        let error = store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: "nope".into() }).unwrap_err();
        assert_eq!(error, VcsError::UnknownChange("nope".into()));
    }

    #[test]
    async fn switch_to_an_unknown_alternative_is_rejected() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        let error = store.dispatch(ArtifactCommand::SwitchAlternative { alternative_id: "nope".into() }).unwrap_err();
        assert_eq!(error, VcsError::UnknownAlternative("nope".into()));
    }

    #[test]
    async fn malformed_alternative_checkpoint_pin_is_rejected_at_construction() {
        let mut envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        envelope.vcs.alternatives.push(Alternative { id: "alt-dangling".into(), name: "dangling".into(), checkpoint_ids: vec!["checkpoint-that-was-never-recorded".into()] });
        let error = match super::ArtifactStore::<DemoSnapshot, DemoMutation>::new(envelope) {
            Ok(_) => panic!("the alternative's pinned checkpoint id must actually exist"),
            Err(error) => error,
        };
        assert!(matches!(error, VcsError::ValidationFailed(message) if message.contains("alt-dangling") && message.contains("checkpoint-that-was-never-recorded")));
    }

    #[test]
    async fn create_alternative_with_no_edits_and_no_checkpoints_is_rejected() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        let error = store.dispatch(ArtifactCommand::CreateAlternative { name: "x".into() }).unwrap_err();
        assert_eq!(error, VcsError::NoCheckpoint, "the auto-commit has nothing pending, so there is still no checkpoint to branch from");
    }

    #[test]
    async fn compensating_undo_without_a_semantic_command_is_rejected() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply");
        let error = store.dispatch(ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::CompensatingAction, semantic_command: None }).unwrap_err();
        assert!(matches!(error, VcsError::Backbone(_)), "got {error:?}");
    }

    #[test]
    async fn materialize_document_snapshot_rejects_an_unknown_edit_id() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let error = materialize_document_snapshot(&envelope, &["missing-edit".to_string()]).unwrap_err();
        assert_eq!(error, VcsError::UnknownEdit("missing-edit".into()));
    }

    #[test]
    async fn dispatch_text_applies_a_command_block_and_snapshot_json_reflects_it() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        let command_text = print_command(&ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 7 }], description: None }).expect("print command");
        store.dispatch_text(&command_text).expect("dispatch text");
        assert_eq!(store.snapshot_json().expect("snapshot json"), serde_json::to_string(&DemoSnapshot { n: 7 }).unwrap());

        let error = store.dispatch_text("not a command").unwrap_err();
        assert!(matches!(error, VcsError::Deserialize(_)), "got {error:?}");
    }

    #[test]
    async fn dispatch_binary_applies_an_encoded_command_and_rejects_wrong_format() {
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
    async fn command_text_binary_equivalence_holds_for_every_document_command_variant() {
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
            ArtifactCommand::ApplyInLane { mutations: vec![DemoMutation::SetN { n: 9 }], description: Some("select".to_string()), lane: HistoryLane::Interaction },
            ArtifactCommand::ApplyInLane { mutations: vec![DemoMutation::SetN { n: 9 }], description: None, lane: HistoryLane::Document },
            ArtifactCommand::AmendLastInLane { mutations: vec![DemoMutation::SetN { n: 4 }], coalesce_key: Some("hover".to_string()), lane: HistoryLane::Interaction },
            ArtifactCommand::UndoInLane { lane: HistoryLane::Interaction },
            ArtifactCommand::RedoInLane { lane: HistoryLane::Interaction },
        ];
        for command in &commands {
            test_support::assert_command_text_binary_equivalence(command);
        }
    }

    //#endregion 🔖️CommandErrorPaths

    //#region 🔖️ReconcileAlternative
    #[test]
    async fn reconcile_alternative_requires_an_existing_checkpoint() {
        let mut envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let error = reconcile_alternative(&mut envelope, "reconciled", None, Vec::new()).unwrap_err();
        assert_eq!(error, VcsError::NoCheckpoint);
    }

    #[test]
    async fn reconcile_alternative_pins_the_latest_checkpoint_and_optionally_records_a_reconciliation_checkpoint() {
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
    async fn commit_checkpoint_mints_distinct_content_addressed_ids_for_distinct_commits() {
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
    async fn merge_base_finds_the_nearest_common_ancestor_across_a_fork() {
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
    async fn merge_base_is_none_for_a_dangling_unknown_checkpoint_id() {
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
    async fn snapshot_merge_into_a_nonempty_store_adds_only_the_new_remote_edits_and_records() {
        let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
        let mut store = ArtifactStore::new(envelope);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("local apply");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("local".into()), authors: Vec::new() }).expect("local commit");

        let mut remote_store = ArtifactStore::new(store.envelope().clone());
        remote_store.reset(store.envelope().clone(), store.applied_edit_ids().to_vec(), Vec::new()).expect("reset remote");
        remote_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).expect("remote apply");
        remote_store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("remote".into()), authors: Vec::new() }).expect("remote commit");

        let (channel, remote_end) = ChannelBackbone::pair("chan");
        store.attach_backbone(Backbones::Channel(channel)).expect("attach");
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
    async fn space_member_checkout_switches_at_the_alternative_tip_and_falls_back_to_checkout_when_stale() {
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
    async fn memory_backbone_port_round_trips_and_reports_a_missing_file() {
        let port = MemoryBackbonePort::new();
        let error = port.read("file://nowhere").unwrap_err();
        assert!(matches!(error, VcsError::Backbone(_)), "got {error:?}");
        port.write("file://a", "payload-1").expect("write");
        assert_eq!(port.read("file://a").expect("read"), "payload-1");
        port.write("file://a", "payload-2").expect("overwrite");
        assert_eq!(port.read("file://a").expect("read after overwrite"), "payload-2", "write is an upsert");
    }

    #[test]
    async fn local_storage_backbone_port_falls_back_to_its_in_memory_store() {
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
    async fn pack_value_fixture_corpus() -> Vec<(&'static str, DslValue)> {
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

    async fn dsl_value_numeric_insensitive_eq(a: &DslValue, b: &DslValue) -> bool {
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
    async fn pack_value_fixture_corpus_hex_dump() {
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
    async fn pack_wire_value_fixture_corpus_hex_dump() {
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
        async fn parse_op(line: &str) -> Result<Self, TextError> {
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
        async fn print_op(&self) -> String {
            let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
            let variants = <Self as crate::os_dsl::DslVariants>::variants();
            let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
            crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)
        }
    }

    impl OpBinary for ValidatedMutation {
        async fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
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
        async fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
            const OP_BINARY_FORMAT: u8 = 1;
            let mut reader = crate::os_pack::ByteReader::new(bytes).await;
            let format = reader.read_u8().await?;
            if format != OP_BINARY_FORMAT {
                return Err(crate::os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
            }
            let ordinal = reader.read_varint_u64().await?;
            let variants = <Self as crate::os_dsl::DslVariants>::variants();
            let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
            let spec = spec_fn();
            let body = &bytes[reader.position().await..];
            let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
            <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "op record", offset: reader.position().await as u64, detail: error.to_string() })
        }
    }

    /// @emoji 🛂️ The composition fixture's negative value is a fatal outcome, so the dry-run
    /// rejects it before group dispatch can alter either member's history.
    impl Mutation<DemoSnapshot> for ValidatedMutation {
        type Diff = DemoDiff;
        async fn diff(&self, _snapshot: &DemoSnapshot) -> crate::os_spr::MutationOutcome<DemoDiff> {
            match self {
                ValidatedMutation::SetN { n } if *n < 0 => crate::os_spr::MutationOutcome::fatal("mutation.invariant", "n must be non-negative", ["n"]),
                ValidatedMutation::SetN { n } => crate::os_spr::MutationOutcome::new(DemoDiff { n: Some(*n) }),
            }
        }
        async fn inverse(&self, snapshot: &DemoSnapshot) -> Vec<Self> {
            vec![ValidatedMutation::SetN { n: snapshot.n }]
        }
    }

    /// 🎯️ The dialect every composition fixture below mints children under.
    async fn demo_child_dialect() -> crate::os_io::ArtifactDialect {
        crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.mesh".into(), standard: "1".into(), subset: "*".into() }
    }

    #[test]
    async fn artifact_child_dsl_field_round_trips_via_pack_and_value() {
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
    async fn owner_ref_dsl_field_round_trips_via_pack() {
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
    async fn artifact_link_dsl_field_round_trips_every_link_pin_variant() {
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
    async fn artifact_refs_defaults_to_empty_for_a_leaf_snapshot() {
        struct LeafSnapshot;
        impl ArtifactRefs for LeafSnapshot {}
        let snapshot = LeafSnapshot;
        assert!(snapshot.child_refs().is_empty());
        assert!(snapshot.links().is_empty());
    }

    #[test]
    async fn typed_child_store_factory_round_trips_a_child_through_create_persist_open() {
        let dialect = demo_child_dialect();

        let mut child = super::create_member_store::<DemoSnapshot, DemoMutation>("demo/v1", "child-round-trip", &dialect, &DemoSnapshot { n: 7 }.encode_pack()).expect("create");
        child.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 9 }], description: Some("bump".into()) }).expect("apply");
        child.dispatch(ArtifactCommand::Undo).expect("undo");
        child.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 11 }], description: None }).expect("re-apply");

        let files = print_document_pack(child.envelope()).expect("print");
        let persisted = encode_document_pack_bytes(&files.pack, &files.spr);
        let reopened = super::open_member_store::<DemoSnapshot, DemoMutation>(&persisted).expect("open");

        assert_eq!(reopened.envelope().id, "child-round-trip");
        assert_eq!(reopened.snapshot().expect("head snapshot"), child.snapshot().expect("head snapshot"), "reopened child's live content diverged from the persisted one");
        assert_eq!(reopened.snapshot().expect("head snapshot"), DemoSnapshot { n: 11 }, "reopen restored the wrong cursor position");
    }

    #[test]
    async fn typed_child_store_factory_rejects_empty_genesis_and_dialect_less_owned_child() {
        assert!(matches!(super::create_member_store::<DemoSnapshot, DemoMutation>("demo/v1", "child-empty", &demo_child_dialect(), &[]), Err(VcsError::Deserialize(_))), "an empty genesis pack must never silently default");

        // 🏠️ owner ⇒ dialect: an envelope that is somebody's child but names no dialect cannot be
        // typed by its parent, so `open` must fail closed rather than hand back an untypable member.
        let mut envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "child-no-dialect", DemoSnapshot { n: 1 }, None);
        envelope.owner = Some(OwnerRef { parent: crate::os_io::ArtifactRef { artifact_id: "parent".into(), dialect: demo_child_dialect() }, slot: "mesh".into(), child_id: "child-no-dialect".into() });
        let files = print_document_pack(&envelope).expect("print");
        let orphan = encode_document_pack_bytes(&files.pack, &files.spr);
        assert!(matches!(super::open_member_store::<DemoSnapshot, DemoMutation>(&orphan), Err(VcsError::Deserialize(_))), "an owned child with no dialect must fail closed");
    }

    #[test]
    async fn pack_at_checkpoint_reads_history_without_moving_the_live_cursor() {
        let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "pinned-target", DemoSnapshot { n: 1 }, None));
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).expect("apply");
        let checkpoint = SpaceMember::commit_checkpoint(&mut store, "v1".into(), Vec::new()).expect("checkpoint");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 3 }], description: None }).expect("apply after checkpoint");

        let historical = DemoSnapshot::decode_pack(&store.pack_at_checkpoint(&checkpoint).expect("pack at checkpoint")).expect("decode");
        let live = DemoSnapshot::decode_pack(&store.document_pack_bytes().expect("head pack")).expect("decode");
        assert_eq!(historical, DemoSnapshot { n: 2 }, "checkpoint read did not return the pinned content");
        assert_eq!(live, DemoSnapshot { n: 3 }, "reading a checkpoint moved the live cursor");
        assert!(matches!(store.pack_at_checkpoint("no-such-checkpoint"), Err(VcsError::UnknownChange(_))));
    }

    #[test]
    async fn member_link_resolver_resolves_head_checkpoint_and_degrades_snapshot_pins() {
        struct FixtureDirectory {
            member: ArtifactStore<DemoSnapshot, DemoMutation>,
        }
        impl MemberDirectory for FixtureDirectory {
            async fn head_pack(&self, artifact_id: &str) -> Option<Result<Vec<u8>, VcsError>> {
                (artifact_id == "linked-doc").then(|| self.member.document_pack_bytes())
            }
            async fn checkpoint_pack(&self, artifact_id: &str, checkpoint_id: &str) -> Option<Result<Vec<u8>, VcsError>> {
                (artifact_id == "linked-doc").then(|| self.member.pack_at_checkpoint(checkpoint_id))
            }
        }

        let mut member = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "linked-doc", DemoSnapshot { n: 1 }, None));
        member.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 42 }], description: None }).expect("apply");
        let pinned = SpaceMember::commit_checkpoint(&mut member, "pinned".into(), Vec::new()).expect("checkpoint");
        member.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 99 }], description: None }).expect("apply after pin");

        let resolver = MemberLinkResolver::new(FixtureDirectory { member });
        let target = crate::os_io::ArtifactRef { artifact_id: "linked-doc".into(), dialect: demo_child_dialect() };
        let link_of = |pin: LinkPin| ArtifactLink { target: target.clone(), pin, role: "representation".into() };
        let decode = |state: LinkState| match state {
            LinkState::Resolved { pack_bytes, .. } => DemoSnapshot::decode_pack(&pack_bytes).expect("decode"),
            other => panic!("expected Resolved, found {other:?}"),
        };

        assert_eq!(decode(resolver.resolve(&link_of(LinkPin::Head))), DemoSnapshot { n: 99 }, "a Head pin must follow the target's live tip");
        assert_eq!(decode(resolver.resolve(&link_of(LinkPin::Checkpoint { id: pinned }))), DemoSnapshot { n: 42 }, "a Checkpoint pin must keep resolving to the pinned history, not the tip");

        let blob = BlobRef { hash: "deadbeef".into(), size: 3, media_type: "application/octet-stream".into() };
        assert!(matches!(resolver.resolve(&link_of(LinkPin::Snapshot { blob })), LinkState::PinnedOnly { .. }), "a snapshot pin with no blob store must degrade to PinnedOnly, never Missing");

        let absent = ArtifactLink { target: crate::os_io::ArtifactRef { artifact_id: "gone".into(), dialect: demo_child_dialect() }, pin: LinkPin::Head, role: "representation".into() };
        assert_eq!(resolver.resolve(&absent), LinkState::Missing);
    }

    #[test]
    async fn link_resolver_reports_resolved_missing_and_pinned_only_states() {
        struct DemoResolver;
        impl LinkResolver for DemoResolver {
            async fn resolve(&self, link: &ArtifactLink) -> LinkState {
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
    async fn composition_graph_owns_forest_rejects_second_owner_and_cycle() {
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
    async fn composition_graph_links_reject_cycle_but_allow_converging_dag_edges() {
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
    async fn mint_child_id_converges_across_two_replicas_and_varies_by_ordinal_and_slot() {
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

    /// @emoji 🧪️ One fatal preview anywhere ⇒ nothing applied on ANY member, parent included.
    #[test]
    async fn dispatch_group_validate_all_atomicity_one_bad_member_applies_nothing() {
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
        let mut children = [(&mut child_store, child_dispatch)];

        let result = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, parent_ops, Vec::new(), GroupMeta::default());
        match result {
            Ok(_) => panic!("expected the group dispatch to fail phase-1 validation, but it succeeded"),
            // 🎞️ `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6: an
            // ordinary mutation-level (message-based) rejection now travels as `VcsError::Rejected`,
            // not `ValidationFailed` (reserved for structural failures only — see its own doc
            // comment). The default policy (`Normal`, via `SpaceMember::merge_policy`'s trait
            // default, since neither member's policy is configured here) rejects Fatal.
            Err(VcsError::Rejected { policy, messages }) => {
                assert_eq!(policy, crate::os_spr::MergePolicy::Normal);
                assert!(messages.iter().any(|message| message.level == crate::os_dsl::Severity::Fatal));
            }
            Err(other) => panic!("expected Rejected, got a different VcsError: {other}"),
        }
        assert!(parent_store.envelope().vcs.edits.is_empty(), "parent must have zero edits after a failed group dispatch");
        assert!(child_store.envelope().vcs.edits.is_empty(), "child must have zero edits after a failed group dispatch");
    }

    /// @emoji 🧪️ TASK 2's ownership-check law: `dispatch_group` refuses to touch a `ChildDispatch`
    /// whose claimed parent the coordinator's own `CompositionGraph` does not currently track —
    /// zero side effects, same as any other phase-1 failure.
    #[test]
    async fn dispatch_group_rejects_a_child_the_graph_does_not_track_as_owned() {
        let parent_ref = crate::os_io::ArtifactRef { artifact_id: "parent-unowned-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
        let child_ref = crate::os_io::ArtifactRef { artifact_id: "child-unowned-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

        let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, ValidatedMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, ValidatedMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        let mut coordinator = CompositionCoordinator::new();
        // Deliberately NOT seeding `coordinator.graph_mut().insert_owns(..)` — the graph has no
        // record that `parent_ref` owns `child_ref`.

        let op = ValidatedMutation::SetN { n: 1 }.encode_op().expect("encode");
        let child_dispatch = ChildDispatch { child: child_ref.clone(), ops: vec![op], op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
        let mut children = [(&mut child_store, child_dispatch)];

        let result = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, Vec::new(), Vec::new(), GroupMeta::default());
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
    async fn compensate_undoes_applied_members_in_reverse_order() {
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
        let mut children = [(&mut child_a, dispatch_a), (&mut child_b, dispatch_b)];
        let applied_children = vec![(0usize, child_a_edit_id), (1usize, child_b_edit_id)];

        let report = CompositionCoordinator::compensate(&parent_ref, &mut parent_store, &mut children, &applied_children, Some(&parent_edit_id));

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
    async fn compensate_reports_skipped_when_a_members_own_undo_fails_and_folds_to_compensation_failed() {
        let parent_ref = crate::os_io::ArtifactRef { artifact_id: "parent-comp-fail-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
        let child_ref = crate::os_io::ArtifactRef { artifact_id: "child-comp-fail-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

        // Parent has NOTHING applied, so `parent.undo()` deterministically fails with
        // `NothingToUndo` — simulating a member whose own rollback errors mid-compensation.
        let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        child_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 4 }], description: None }).expect("apply child");
        let child_edit_id = child_store.envelope().vcs.edits.last().expect("child edit").id.clone();

        let dispatch = ChildDispatch { child: child_ref.clone(), ops: Vec::new(), op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
        let mut children = [(&mut child_store, dispatch)];
        let applied_children = vec![(0usize, child_edit_id)];

        let report = CompositionCoordinator::compensate(&parent_ref, &mut parent_store, &mut children, &applied_children, Some("bogus-parent-edit-id"));

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
    async fn dispatch_group_mints_genesis_child_ids_deterministically_across_replicas() {
        // 🎯️ O1: no more registry — `ArtifactStore<DemoSnapshot, DemoMutation>`'s `MemberFactory`
        // impl (above) matches ANY `kind` string, so genesis creation just works.
        let parent_ref = crate::os_io::ArtifactRef { artifact_id: "parent-genesis-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
        let genesis_dialect = crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() };
        let genesis = vec![ChildGenesis { slot: "mesh-slot".into(), dialect: genesis_dialect, initial_pack: Vec::new() }];
        let parent_ops: Vec<Vec<u8>> = vec![DemoMutation::SetN { n: 1 }.encode_op().expect("encode")];

        let mut parent_1 = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        let mut coordinator_1 = CompositionCoordinator::new();
        let mut children_1 = [];
        let receipt_1 = coordinator_1.dispatch_group(&parent_ref, &mut parent_1, &mut children_1, parent_ops.clone(), genesis.clone(), GroupMeta::default()).expect("replica 1 dispatch");

        let mut parent_2 = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        let mut coordinator_2 = CompositionCoordinator::new();
        let mut children_2 = [];
        let receipt_2 = coordinator_2.dispatch_group(&parent_ref, &mut parent_2, &mut children_2, parent_ops, genesis, GroupMeta::default()).expect("replica 2 dispatch");

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
    async fn undo_group_skips_a_foreign_tail_member_but_still_undoes_the_rest() {
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

        let mut members =
            [(&parent_ref, &mut parent_store), (&child_ref, &mut child_store), (&foreign_ref, &mut foreign_store)];

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
    async fn redo_group_skips_a_foreign_tail_member_but_still_redoes_the_rest() {
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

        let mut members = [(&parent_ref, &mut parent_store), (&foreign_ref, &mut foreign_store)];
        let report = CompositionCoordinator::redo_group(&mut members, group_id);

        assert_eq!(report.undone.len(), 1, "only the matching-group member is redone");
        assert_eq!(report.undone[0].0.artifact_id, parent_ref.artifact_id);
        assert_eq!(report.skipped.len(), 1);
        assert_eq!(report.skipped[0].0.artifact_id, foreign_ref.artifact_id);

        assert_eq!(parent_store.snapshot().expect("parent snapshot").n, 1, "parent's edit was reapplied");
        assert_eq!(foreign_store.snapshot().expect("foreign snapshot").n, 0, "the foreign member's redo stack was left untouched");
    }

    //#region 🔖️TransactionPeerTests
    /// @emoji 🧪️ W1-C's headline `Peer`-relation law (contract-freeze §5): two artifacts with NO
    /// ownership relation commit through `dispatch_peer_group` as ONE atomic transaction — both
    /// members end up carrying the SAME `MutationMeta.group_id` (the shared minted
    /// `invocation_id`), the peer's tail edit is stamped `MutationOrigin::Transaction { initiator }`,
    /// and the initiator's own edit stays at its ordinary `Owner` default (it is not foreign to
    /// itself). Deliberately does NOT seed any `CompositionGraph::insert_owns` edge — `Peer` never
    /// consults `owner_of` at all.
    #[test]
    async fn dispatch_peer_group_commits_both_members_with_one_shared_group_id() {
        let initiator_ref = crate::os_io::ArtifactRef { artifact_id: "peer-initiator-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
        let peer_ref = crate::os_io::ArtifactRef { artifact_id: "peer-member-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

        let mut initiator_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &initiator_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        let mut peer_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &peer_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        let mut coordinator = TransactionCoordinator::new();

        let initiator_ops: Vec<Vec<u8>> = vec![DemoMutation::SetN { n: 1 }.encode_op().expect("encode initiator op")];
        let peer_op = DemoMutation::SetN { n: 2 }.encode_op().expect("encode peer op");
        let peer_dispatch = ChildDispatch { child: peer_ref.clone(), ops: vec![peer_op], op_schema: SchemaId("demo/v1".into()), labels: vec!["peer".into()] };
        let mut peers = [(&mut peer_store, peer_dispatch)];

        let receipt = coordinator.dispatch_peer_group(&initiator_ref, &mut initiator_store, &mut peers, initiator_ops, GroupMeta::default()).expect("peer transaction dispatch");

        assert_eq!(receipt.member_edits.len(), 2, "both the initiator and the one peer got a real edit");
        assert_eq!(initiator_store.snapshot().expect("initiator snapshot").n, 1);
        assert_eq!(peer_store.snapshot().expect("peer snapshot").n, 2);

        let initiator_group_id = initiator_store.tail_group_id().expect("initiator tail group id");
        let peer_group_id = peer_store.tail_group_id().expect("peer tail group id");
        assert_eq!(initiator_group_id, peer_group_id, "both members share the SAME minted invocation id as their group id");
        assert_eq!(initiator_group_id, receipt.invocation_id);

        let initiator_origin = initiator_store.envelope().vcs.edits.last().expect("initiator edit").mutation_meta.last().expect("initiator meta").origin.clone();
        assert_eq!(initiator_origin, crate::os_spr::MutationOrigin::Owner, "the initiator's own edit is not foreign to itself, so it stays at the ordinary Owner default");

        let peer_origin = peer_store.envelope().vcs.edits.last().expect("peer edit").mutation_meta.last().expect("peer meta").origin.clone();
        match peer_origin {
            crate::os_spr::MutationOrigin::Transaction { initiator } => assert_eq!(initiator.artifact_id, initiator_ref.artifact_id, "the peer's origin names the real initiator"),
            other => panic!("expected MutationOrigin::Transaction on the peer's tail edit, got {other:?}"),
        }
    }

    /// @emoji 🧪️ W1-C's compensation law for `Peer`: `compensate` is relation-agnostic (see its own
    /// doc comment) — exercising it with a two-PEER (no ownership) scenario proves the SAME
    /// reverse-order rollback `dispatch_peer_group`'s phase 2 falls back to on a late failure works
    /// identically to the `Owned` case `compensate_undoes_applied_members_in_reverse_order` already
    /// covers. Peer B is deliberately left with no applied edit at all, modeling "the SECOND
    /// member's own dispatch failed" — the exact trigger `dispatch_peer_group`'s real error branch
    /// calls `compensate` from, passing only the members that DID get applied (the initiator + peer
    /// A) as `applied_children`/`parent_applied`.
    #[test]
    async fn compensate_undoes_applied_peer_members_in_reverse_order() {
        let initiator_ref = crate::os_io::ArtifactRef { artifact_id: "peer-comp-initiator".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
        let peer_a_ref = crate::os_io::ArtifactRef { artifact_id: "peer-comp-a".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };
        let peer_b_ref = crate::os_io::ArtifactRef { artifact_id: "peer-comp-b".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

        let mut initiator_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &initiator_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        initiator_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).expect("apply initiator");
        let initiator_edit_id = initiator_store.envelope().vcs.edits.last().expect("initiator edit").id.clone();

        let mut peer_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &peer_a_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        peer_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).expect("apply peer a — the member the transaction already reached");
        let peer_a_edit_id = peer_a.envelope().vcs.edits.last().expect("a edit").id.clone();

        let mut peer_b = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &peer_b_ref.artifact_id, DemoSnapshot { n: 0 }, None));

        let dispatch_a = ChildDispatch { child: peer_a_ref.clone(), ops: Vec::new(), op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
        let dispatch_b = ChildDispatch { child: peer_b_ref.clone(), ops: Vec::new(), op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
        let mut peers = [(&mut peer_a, dispatch_a), (&mut peer_b, dispatch_b)];
        let applied_so_far = vec![(0usize, peer_a_edit_id)];

        let report = TransactionCoordinator::compensate(&initiator_ref, &mut initiator_store, &mut peers, &applied_so_far, Some(&initiator_edit_id));

        assert!(report.skipped.is_empty(), "every already-applied member should have undone cleanly");
        assert_eq!(report.undone.len(), 2, "the initiator and peer A (the only two that were actually applied) are both rolled back");
        assert_eq!(report.undone[0].0.artifact_id, initiator_ref.artifact_id, "initiator undone first");
        assert_eq!(report.undone[1].0.artifact_id, peer_a_ref.artifact_id, "then the one applied peer");

        assert_eq!(initiator_store.snapshot().expect("initiator snapshot").n, 0, "initiator's edit was compensated");
        assert_eq!(peer_a.snapshot().expect("a snapshot").n, 0, "peer A's edit was compensated");
    }

    /// @emoji 🧪️ Task 2's group-undo law, exercised through a REAL `Peer` transaction end-to-end:
    /// `undo_group` (unmodified — see its own doc comment on being relation-agnostic) reverses BOTH
    /// members of a `dispatch_peer_group` group as ONE, using the exact `invocation_id` that call
    /// minted, with no code path specific to `Peer` needed.
    #[test]
    async fn undo_group_reverses_both_members_of_a_real_peer_transaction() {
        let initiator_ref = crate::os_io::ArtifactRef { artifact_id: "peer-undo-initiator".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
        let peer_ref = crate::os_io::ArtifactRef { artifact_id: "peer-undo-member".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

        let mut initiator_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &initiator_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        let mut peer_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &peer_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        let mut coordinator = TransactionCoordinator::new();

        let initiator_ops = vec![DemoMutation::SetN { n: 5 }.encode_op().expect("encode initiator op")];
        let peer_op = DemoMutation::SetN { n: 7 }.encode_op().expect("encode peer op");
        let peer_dispatch = ChildDispatch { child: peer_ref.clone(), ops: vec![peer_op], op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
        let receipt = {
            let mut peers = [(&mut peer_store, peer_dispatch)];
            coordinator.dispatch_peer_group(&initiator_ref, &mut initiator_store, &mut peers, initiator_ops, GroupMeta::default()).expect("peer transaction dispatch")
        };
        assert_eq!(initiator_store.snapshot().expect("initiator snapshot").n, 5);
        assert_eq!(peer_store.snapshot().expect("peer snapshot").n, 7);

        let mut members = [(&initiator_ref, &mut initiator_store), (&peer_ref, &mut peer_store)];
        let report = TransactionCoordinator::undo_group(&mut members, &receipt.invocation_id);

        assert!(report.skipped.is_empty(), "both real transaction members must belong to the group");
        assert_eq!(report.undone.len(), 2);
        assert_eq!(initiator_store.snapshot().expect("initiator snapshot after undo").n, 0);
        assert_eq!(peer_store.snapshot().expect("peer snapshot after undo").n, 0);
    }

    /// @emoji 🧪️ `Peer`'s cycle guard law (`MemberRelation::Peer`'s doc comment): a SECOND, separate
    /// `dispatch_peer_group` call that would close a link cycle across the coordinator's persisted
    /// `Links` graph is rejected — `CompositionGraph::would_cycle_links`, the same primitive
    /// `would_cycle_owns` is to `Owned`'s genesis cycle guard. Transaction 1 (A initiates, B is the
    /// peer) succeeds and records a `Links` edge A -> B; transaction 2 (B initiates, A is the peer)
    /// would close A -> B -> A and is rejected with zero side effects.
    #[test]
    async fn dispatch_peer_group_rejects_a_transaction_that_would_close_a_peer_link_cycle() {
        let artifact_a_ref = crate::os_io::ArtifactRef { artifact_id: "peer-cycle-a".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
        let artifact_b_ref = crate::os_io::ArtifactRef { artifact_id: "peer-cycle-b".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

        let mut store_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &artifact_a_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        let mut store_b = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &artifact_b_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        let mut coordinator = TransactionCoordinator::new();

        let op_ab = DemoMutation::SetN { n: 1 }.encode_op().expect("encode a->b op");
        let dispatch_ab = ChildDispatch { child: artifact_b_ref.clone(), ops: vec![op_ab], op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
        {
            let mut peers = [(&mut store_b, dispatch_ab)];
            coordinator.dispatch_peer_group(&artifact_a_ref, &mut store_a, &mut peers, Vec::new(), GroupMeta::default()).expect("first transaction A -> B");
        }
        assert_eq!(store_b.snapshot().expect("b snapshot").n, 1);

        let op_ba = DemoMutation::SetN { n: 9 }.encode_op().expect("encode b->a op");
        let dispatch_ba = ChildDispatch { child: artifact_a_ref.clone(), ops: vec![op_ba], op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
        let mut peers = [(&mut store_a, dispatch_ba)];
        let result = coordinator.dispatch_peer_group(&artifact_b_ref, &mut store_b, &mut peers, Vec::new(), GroupMeta::default());
        match result {
            Ok(_) => panic!("expected a CompositionCycle rejection, but the dispatch succeeded"),
            Err(VcsError::CompositionCycle(_)) => {}
            Err(other) => panic!("expected CompositionCycle, got a different VcsError: {other}"),
        }
        assert_eq!(store_a.snapshot().expect("a snapshot").n, 0, "A must have zero new edits after a rejected cycle");
    }

    /// @emoji 🧪️ W1-C's "Owned reproduces today's behaviour EXACTLY" law: `CompositionCoordinator`
    /// (the `TransactionCoordinator` alias) dispatching through the ordinary `Owned`-relation
    /// `dispatch_group` produces EXACTLY what it did before `MemberRelation` existed — including
    /// that `MutationMeta.origin` is NEVER touched (stays the ordinary `Apply`-assigned `Owner`
    /// default), because `stamp_tail_origin` is only ever called on the `Peer` path. Every other
    /// `dispatch_group_*`/`compensate_*`/`undo_group_*`/`redo_group_*` test above this region
    /// already re-proves the rest of `Owned`'s behaviour (ownership check, genesis, atomicity,
    /// compensation order) is untouched, unmodified, still green.
    #[test]
    async fn dispatch_group_owned_path_never_stamps_a_transaction_origin() {
        let parent_ref = crate::os_io::ArtifactRef { artifact_id: "parent-owned-origin-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
        let child_ref = crate::os_io::ArtifactRef { artifact_id: "child-owned-origin-1".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

        let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: 0 }, None));
        let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: 0 }, None));

        let mut coordinator = CompositionCoordinator::new();
        coordinator.graph_mut().insert_owns(&parent_ref.artifact_id, "slot-1", &child_ref.artifact_id).expect("seed ownership");

        let parent_ops = vec![DemoMutation::SetN { n: 1 }.encode_op().expect("encode parent op")];
        let child_op = DemoMutation::SetN { n: 2 }.encode_op().expect("encode child op");
        let dispatch = ChildDispatch { child: child_ref.clone(), ops: vec![child_op], op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
        let mut children = [(&mut child_store, dispatch)];

        let receipt = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, parent_ops, Vec::new(), GroupMeta::default()).expect("owned dispatch");

        assert_eq!(receipt.member_edits.len(), 2);
        let parent_origin = parent_store.envelope().vcs.edits.last().expect("parent edit").mutation_meta.last().expect("parent meta").origin.clone();
        let child_origin = child_store.envelope().vcs.edits.last().expect("child edit").mutation_meta.last().expect("child meta").origin.clone();
        assert_eq!(parent_origin, crate::os_spr::MutationOrigin::Owner, "Owned relation never stamps a Transaction origin on the parent");
        assert_eq!(child_origin, crate::os_spr::MutationOrigin::Owner, "Owned relation never stamps a Transaction origin on an owned child either");
    }
    //#endregion 🔖️TransactionPeerTests

    //#region 🔖️PhasePolicyTests
    /// @emoji 🧪️ Normal (the default policy) rejects an Error-level message exactly like it already
    /// rejects Fatal (`dispatch_group_validate_all_atomicity_one_bad_member_applies_nothing` above)
    /// — the SAME all-or-nothing law, now driven by `reject_if_policy_rejects` off the UNIONED
    /// `preview_wire` messages rather than an immediate per-member `Err`. Exercises lane 1-A's REAL
    /// `ArtifactStore::set_merge_policy` (§C6, landed) end to end — no test-only policy fixture
    /// needed now that it exists.
    #[test]
    async fn dispatch_group_phase1_rejects_under_normal_when_a_member_yields_an_error_and_nothing_applies() {
        let parent_ref = crate::os_io::ArtifactRef { artifact_id: "policy-parent-normal".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
        let child_ref = crate::os_io::ArtifactRef { artifact_id: "policy-child-normal".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

        // 🎯️ The REAL `super::ArtifactStore` (not this module's `SpaceMember`-object-safe test
        // wrapper): the wrapper's own `impl SpaceMember for ArtifactStore` block does not override
        // `merge_policy` (it would just re-delegate to this exact same real inherent method — see
        // that block's own comment), so coercing the WRAPPER to `&mut dyn SpaceMember` would read
        // the trait's `Normal` default via its vtable instead of whatever `set_merge_policy` below
        // sets. Going straight to the real type sidesteps that indirection entirely.
        let mut parent_store = super::ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: 0 }, None)).expect("valid parent envelope");
        parent_store.set_merge_policy(crate::os_spr::MergePolicy::Normal);
        assert_eq!(parent_store.merge_policy(), crate::os_spr::MergePolicy::Normal, "Normal is also the default, but set it explicitly so this test does not rely on that");
        let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: 0 }, None));

        let mut coordinator = CompositionCoordinator::new();
        coordinator.graph_mut().insert_owns(&parent_ref.artifact_id, "slot-1", &child_ref.artifact_id).expect("seed ownership");

        let parent_ops = vec![SeverityMutation::CleanN { n: 1 }.encode_op().expect("encode parent op")];
        let child_op = SeverityMutation::ErrorN { n: 99 }.encode_op().expect("encode child op");
        let dispatch = ChildDispatch { child: child_ref.clone(), ops: vec![child_op], op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
        let mut children = [(&mut child_store, dispatch)];

        let result = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, parent_ops, Vec::new(), GroupMeta::default());
        match result {
            Ok(_) => panic!("expected Normal policy to reject an Error-level message, but the dispatch succeeded"),
            Err(VcsError::Rejected { policy, messages }) => {
                assert_eq!(policy, crate::os_spr::MergePolicy::Normal, "the rejection must name the policy that actually rejected it");
                assert!(messages.iter().any(|message| message.level == crate::os_dsl::Severity::Error), "the rejection must carry the Error message that triggered it");
            }
            Err(other) => panic!("expected Rejected, got a different VcsError: {other}"),
        }
        assert!(parent_store.envelope().vcs.edits.is_empty(), "parent must have zero edits after a policy-rejected group dispatch");
        assert!(child_store.envelope().vcs.edits.is_empty(), "child must have zero edits after a policy-rejected group dispatch");
    }

    /// @emoji 🧪️ The SAME Error-level scenario `dispatch_group_phase1_rejects_under_normal_when_a_
    /// member_yields_an_error_and_nothing_applies` rejects is accepted end-to-end under
    /// `LaissezFaire` (only `Fatal` is rejected) — both members get a real edit, and the child's own
    /// diff stays empty (§C2 LAW 2: an `Error` message's diff carries no change for the target) even
    /// though the group as a whole succeeded.
    ///
    /// 🎯️ Sets `LaissezFaire` on BOTH members, not just the parent: `reject_if_policy_rejects` is
    /// only the coordinator's GROUP-level gate (checked once, against the parent's own policy,
    /// before Phase 2 starts). Phase 2 still dispatches through each member's REAL `dispatch_wire`,
    /// which (lane 1-A's now-landed C6 `ArtifactStore::dispatch`) independently enforces THAT
    /// member's own `merge_policy()` — a child left at the Normal default would still reject its own
    /// Error-level op right here, even though the coordinator's gate already accepted the group.
    #[test]
    async fn dispatch_group_phase1_accepts_the_same_error_scenario_under_laissez_faire() {
        let parent_ref = crate::os_io::ArtifactRef { artifact_id: "policy-parent-lf".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
        let child_ref = crate::os_io::ArtifactRef { artifact_id: "policy-child-lf".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

        // 🎯️ The REAL `super::ArtifactStore` (not this module's `SpaceMember`-object-safe test
        // wrapper): the wrapper's own `impl SpaceMember for ArtifactStore` block does not override
        // `merge_policy` (it would just re-delegate to this exact same real inherent method — see
        // that block's own comment), so coercing the WRAPPER to `&mut dyn SpaceMember` would read
        // the trait's `Normal` default via its vtable instead of whatever `set_merge_policy` below
        // sets. Going straight to the real type sidesteps that indirection entirely.
        let mut parent_store = super::ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: 0 }, None)).expect("valid parent envelope");
        parent_store.set_merge_policy(crate::os_spr::MergePolicy::LaissezFaire);
        let mut child_store = super::ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: 0 }, None)).expect("valid child envelope");
        child_store.set_merge_policy(crate::os_spr::MergePolicy::LaissezFaire);

        let mut coordinator = CompositionCoordinator::new();
        coordinator.graph_mut().insert_owns(&parent_ref.artifact_id, "slot-1", &child_ref.artifact_id).expect("seed ownership");

        let parent_ops = vec![SeverityMutation::CleanN { n: 1 }.encode_op().expect("encode parent op")];
        let child_op = SeverityMutation::ErrorN { n: 99 }.encode_op().expect("encode child op");
        let dispatch = ChildDispatch { child: child_ref.clone(), ops: vec![child_op], op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
        let mut children = [(&mut child_store, dispatch)];

        let receipt = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, parent_ops, Vec::new(), GroupMeta::default()).expect("LaissezFaire accepts an Error-level message");

        assert_eq!(receipt.member_edits.len(), 2, "both parent and child got a real edit despite the child's Error message");
        assert_eq!(parent_store.snapshot().expect("parent snapshot").n, 1, "the parent's own clean op still applied");
        assert_eq!(child_store.snapshot().expect("child snapshot").n, 0, "the child's Error-level op's diff carries no change for its target (LAW 2), even though the group was accepted");
        assert!(receipt.messages.iter().any(|message| message.code.0 == "mutation.target-missing" && message.level == crate::os_dsl::Severity::Error), "the child's Error message must still be reported, even on acceptance");
    }

    /// @emoji 🧪️ `Vigilant` rejects a plain `Warning` — the strictest of the three policies, and the
    /// one level `Normal` (the OTHER two policy tests above/below use) would have accepted.
    #[test]
    async fn dispatch_group_phase1_rejects_under_vigilant_on_a_members_warning() {
        let parent_ref = crate::os_io::ArtifactRef { artifact_id: "policy-parent-vigilant".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
        let child_ref = crate::os_io::ArtifactRef { artifact_id: "policy-child-vigilant".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

        // 🎯️ The REAL `super::ArtifactStore` (not this module's `SpaceMember`-object-safe test
        // wrapper): the wrapper's own `impl SpaceMember for ArtifactStore` block does not override
        // `merge_policy` (it would just re-delegate to this exact same real inherent method — see
        // that block's own comment), so coercing the WRAPPER to `&mut dyn SpaceMember` would read
        // the trait's `Normal` default via its vtable instead of whatever `set_merge_policy` below
        // sets. Going straight to the real type sidesteps that indirection entirely.
        let mut parent_store = super::ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: 0 }, None)).expect("valid parent envelope");
        parent_store.set_merge_policy(crate::os_spr::MergePolicy::Vigilant);
        let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: 0 }, None));

        let mut coordinator = CompositionCoordinator::new();
        coordinator.graph_mut().insert_owns(&parent_ref.artifact_id, "slot-1", &child_ref.artifact_id).expect("seed ownership");

        let child_op = SeverityMutation::WarnN { n: 5 }.encode_op().expect("encode child op");
        let dispatch = ChildDispatch { child: child_ref.clone(), ops: vec![child_op], op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
        let mut children = [(&mut child_store, dispatch)];

        let result = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, Vec::new(), Vec::new(), GroupMeta::default());
        match result {
            Ok(_) => panic!("expected Vigilant policy to reject a Warning-level message, but the dispatch succeeded"),
            Err(VcsError::Rejected { policy, messages }) => {
                assert_eq!(policy, crate::os_spr::MergePolicy::Vigilant, "the rejection must name the policy that actually rejected it");
                assert!(messages.iter().any(|message| message.level == crate::os_dsl::Severity::Warning), "the rejection must carry the Warning message that triggered it");
            }
            Err(other) => panic!("expected Rejected, got a different VcsError: {other}"),
        }
        assert!(child_store.envelope().vcs.edits.is_empty(), "child must have zero edits after a policy-rejected group dispatch");
    }

    /// @emoji 🧪️ `GroupReceipt.messages` carries the FULL union (both parent's own and the child's),
    /// each `target` prefixed with the ORIGINATING member's own `crate::os_io::ArtifactRef::
    /// to_uri()` — the discipline that lets a caller with several members in flight tell messages
    /// apart. Parent-first ordering matches phase 1's own collection order.
    #[test]
    async fn group_receipt_messages_contains_the_union_with_member_path_prefixed_targets() {
        let parent_ref = crate::os_io::ArtifactRef { artifact_id: "policy-parent-union".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
        let child_ref = crate::os_io::ArtifactRef { artifact_id: "policy-child-union".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

        // 🎯️ The REAL `super::ArtifactStore` (not this module's `SpaceMember`-object-safe test
        // wrapper): the wrapper's own `impl SpaceMember for ArtifactStore` block does not override
        // `merge_policy` (it would just re-delegate to this exact same real inherent method — see
        // that block's own comment), so coercing the WRAPPER to `&mut dyn SpaceMember` would read
        // the trait's `Normal` default via its vtable instead of whatever `set_merge_policy` below
        // sets. Going straight to the real type sidesteps that indirection entirely.
        let mut parent_store = super::ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: 0 }, None)).expect("valid parent envelope");
        parent_store.set_merge_policy(crate::os_spr::MergePolicy::LaissezFaire);
        let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: 0 }, None));

        let mut coordinator = CompositionCoordinator::new();
        coordinator.graph_mut().insert_owns(&parent_ref.artifact_id, "slot-1", &child_ref.artifact_id).expect("seed ownership");

        let parent_ops = vec![SeverityMutation::WarnN { n: 3 }.encode_op().expect("encode parent op")];
        let child_op = SeverityMutation::WarnN { n: 5 }.encode_op().expect("encode child op");
        let dispatch = ChildDispatch { child: child_ref.clone(), ops: vec![child_op], op_schema: SchemaId("demo/v1".into()), labels: Vec::new() };
        let mut children = [(&mut child_store, dispatch)];

        let receipt = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, parent_ops, Vec::new(), GroupMeta::default()).expect("LaissezFaire accepts a Warning-level message");

        assert_eq!(receipt.messages.len(), 2, "one Warning from the parent, one from the child");
        assert_eq!(receipt.messages[0].level, crate::os_dsl::Severity::Warning);
        assert_eq!(receipt.messages[0].code.0, "mutation.clamped");
        assert_eq!(receipt.messages[0].target, vec![parent_ref.to_uri()], "the parent's own message is prefixed with the parent's own path, collected before any child's");
        assert_eq!(receipt.messages[1].level, crate::os_dsl::Severity::Warning);
        assert_eq!(receipt.messages[1].code.0, "mutation.clamped");
        assert_eq!(receipt.messages[1].target, vec![child_ref.to_uri()], "the child's message is prefixed with the CHILD's own path, not the parent's");
    }
    //#endregion 🔖️PhasePolicyTests
    //#endregion 🔖️CompositionTests
}
//#endregion 🧪️Tests
//#endregion 🧪️Tests

//#region 🔖️InteractionStatePack
/// 📦️ `ArtifactPack` for `InteractionState` — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM
/// W3b: lets `semio-framework-plugin`'s `VcsArtifactApp` own a real `store::ConfigStore<InteractionState,
/// _>` (persisting selection + active mode/granularity per app instance through the `HistoryLane::Interaction`
/// mechanism above) exactly like any other `ArtifactStore<P, _>`. `InteractionState` has no `RecordSpec`/
/// `dsl_derive` record lowering of its own (a small, framework-internal `BTreeMap`-keyed value, not an app
/// document), so this bridges through the same schema-less `serde_json::Value` pack codec `os_store`'s
/// own `impl ArtifactPack for DslValue` ("Compose-only pack bridge") already uses, rather than hand-rolling a
/// codec. MUST live here, not in `semio-framework-plugin`: the orphan rule requires an impl of a foreign
/// trait for a foreign type to sit in the crate owning one of the two, and both `ArtifactPack` (`os_store`)
/// and `InteractionState` (this region) are this crate's own — `semio-framework-plugin` only sees both
/// through its `store`/`protocol` aliases.
impl crate::os_store::ArtifactPack for protocol::InteractionState {
    async fn encode_pack_with(&self, options: &crate::os_store::PackEncodeOptions) -> Result<Vec<u8>, crate::os_store::PackError> {
        let value = serde_json::to_value(self).map_err(|error| crate::os_store::PackError::Schema(error.to_string()))?;
        crate::os_store::ArtifactPack::encode_pack_with(&value, options).await
    }
    async fn decode_pack_with(bytes: &[u8], options: &crate::os_store::PackDecodeOptions) -> Result<Self, crate::os_store::PackError> {
        let value = <serde_json::Value as crate::os_store::ArtifactPack>::decode_pack_with(bytes, options).await?;
        serde_json::from_value(value).map_err(|error| crate::os_store::PackError::Schema(error.to_string()))
    }
}
//#endregion 🔖️InteractionStatePack

