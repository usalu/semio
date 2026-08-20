//! 🗄️ Generic document version-graph algebra — Author/Change/Checkpoint/Alternative/ArtifactVcs,
//! `VcsError`, content-addressed checkpoint ids, and the raw collection-diff/operation helpers. Pure
//! data plus pure functions: nothing here touches a live document (that's `store::ArtifactStore`,
//! which depends on this crate — see `26/07/28/EXTRACT-STORE-INTO-ITS-OWN-TECHNOLOGY`).

use serde::{Deserialize, Serialize};
use thiserror::Error;

// This crate's own body spells the trait name bare (`self::Mutation<P>` in `apply_mutation`
// below, disambiguating the trait from the same-named generic parameter) — a private (non-`pub`)
// import keeps that ergonomics without re-exposing `crate::os_spr::Mutation` on `vcs`'s own public API
// (dependents import `crate::os_spr::Mutation` directly). `MutationDiff` is imported for its `apply`
// method, called on `Mutation::Diff` inside `apply_mutation`.
use crate::os_spr::{Edit, Mutation, MutationApplyError, MutationDiff};

//#region 🆔️Ids
/// @emoji 🔑 Content-addressed entity id: `{prefix}-{hex16(blake3(prefix || 0 || payload))}`.
pub async fn content_addressed_entity_id(prefix: &str, payload: &[u8]) -> String {
    let mut input = prefix.as_bytes().to_vec();
    input.push(0);
    input.extend_from_slice(payload);
    let digest = *blake3::hash(&input).as_bytes();
    let hex16: String = digest[..8].iter().map(|byte| format!("{byte:02x}")).collect();
    format!("{prefix}-{hex16}")
}

/// @emoji 🆔️ Deterministic child id scoped to an edit: blake3(`{edit_id}:{ordinal}`).
pub async fn edit_scoped_id(edit_id: &str, ordinal: u32) -> String {
    let digest = blake3::hash(format!("{edit_id}:{ordinal}").as_bytes());
    let hex16: String = digest.as_bytes()[..8].iter().map(|byte| format!("{byte:02x}")).collect();
    format!("scoped-{hex16}")
}

/// @emoji ✏️ Content-addressed edit id from actor + sequence + forwards fingerprint (no global counter).
pub async fn mint_edit_id(actor: Option<&str>, sequence: i32, forwards_fingerprint: &[u8]) -> String {
    let mut payload = Vec::new();
    payload.extend_from_slice(actor.unwrap_or("").as_bytes());
    payload.push(0);
    payload.extend_from_slice(&sequence.to_le_bytes());
    payload.push(0);
    payload.extend_from_slice(forwards_fingerprint);
    content_addressed_entity_id("edit", &payload).await
}

/// @emoji 📦️ Content-addressed change id from ordered edit ids (+ optional description distinguisher).
pub async fn mint_change_id(edit_ids: &[String], description: Option<&str>) -> String {
    let mut payload = edit_ids.join("\0").into_bytes();
    payload.push(0);
    payload.extend_from_slice(description.unwrap_or("").as_bytes());
    content_addressed_entity_id("change", &payload).await
}

/// @emoji 🌿️ Content-addressed alternative id from name + ordered checkpoint ids.
pub async fn mint_alternative_id(name: &str, checkpoint_ids: &[String]) -> String {
    let mut payload = name.as_bytes().to_vec();
    payload.push(0);
    payload.extend_from_slice(checkpoint_ids.join("\0").as_bytes());
    content_addressed_entity_id("alternative", &payload).await
}

/// @emoji ⚙️ Content-addressed operation id from the operation's binary (or other) fingerprint bytes.
pub async fn mint_mutation_id(mutation_bytes: &[u8]) -> String {
    content_addressed_entity_id("mutation", mutation_bytes).await
}

/// @emoji 🆔️ Legacy-compatible prefix-only mint — identical inputs collide.
/// Prefer [`mint_edit_id`] / [`mint_change_id`] / [`mint_alternative_id`] / [`mint_mutation_id`] /
/// [`content_addressed_entity_id`] with a distinguishing payload.
pub async fn create_document_vcs_id(prefix: &str) -> String {
    content_addressed_entity_id(prefix, prefix.as_bytes()).await
}
//#endregion 🆔️Ids

//#region 🔖️Schemas
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

// 🎞️ `MutationMeta` lives in `protocol_command`; `Edit<Mutation>` (imported above) is this
// crate's own field type for `ArtifactVcs.edits` below.

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Change {
    pub id: String,
    pub edit_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub saved_at: String,
}

/// @emoji 🧩️ One owned child's checkpoint pin, captured on the parent's checkpoint so checking out
/// the parent can restore the whole composition. `child_ref` is the pinned child artifact's real
/// `crate::os_io::ArtifactRef` — **correction, `UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/📓️wave1-reports/
/// b2-store-composition-report.md`**: the prior wave (`b1-spr-vcs-report.md`) believed `ArtifactRef`
/// (defined in `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`) was unreachable from this crate and fell
/// back to the wire URI `String`. That was wrong: `io/🦀️component.rs` is dual-mounted — the
/// `semio-framework` crate mounts it as `io`, and THIS crate (`semio-framework-os-kernel`) mounts the
/// very same source file as `os_io` (see `💻️os/📦️packages/🦀️rust/📦️glue.rs:237-238`,
/// `pub mod os_io;`) — no cross-crate dependency-direction problem exists; `store` already reaches
/// `crate::os_io::ArtifactDialect` directly (`🏪️store/🦀️component.rs:88/105/662`). Sorting for
/// [`content_addressed_checkpoint_id`] below is therefore by `child_ref.to_uri()` (the same
/// deterministic string this field used to store literally), not by any `Ord` on `ArtifactRef`
/// itself (which does not implement one).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompositionPin {
    pub child_ref: crate::os_io::ArtifactRef,
    pub checkpoint_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Checkpoint {
    pub id: String,
    pub change_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub authors: Vec<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub timestamp: String,
    /// @emoji 🧩️ Which checkpoint each owned child was at when this checkpoint was committed —
    /// empty for a non-composite artifact (every checkpoint before this ticket, and every leaf
    /// artifact after it). Additive; see [`CompositionPin`].
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub composition_pins: Vec<CompositionPin>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alternative {
    pub id: String,
    pub name: String,
    pub checkpoint_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactVcs<P, Mutation> {
    pub initial_snapshot: P,
    pub edits: Vec<Edit<Mutation>>,
    pub changes: Vec<Change>,
    pub checkpoints: Vec<Checkpoint>,
    pub alternatives: Vec<Alternative>,
}
//#endregion 🔖️Schemas
//#region 🔖️Errors
// 🎞️ `Eq` dropped (was `#[derive(Debug, Error, PartialEq, Eq)]`): `Rejected` below carries
// `Vec<crate::os_spr::MutationMessage>`, and `MutationMessage` itself only derives `PartialEq`
// (not `Eq`) — see `📡️spr/🎮️command`'s `🔖️Message` region.
#[derive(Debug, Error, PartialEq)]
pub enum VcsError {
    #[error("unknown edit id: {0}")]
    UnknownEdit(String),
    #[error("unknown change id: {0}")]
    UnknownChange(String),
    #[error("unknown alternative id: {0}")]
    UnknownAlternative(String),
    #[error("no checkpoint for alternative")]
    NoCheckpoint,
    #[error("empty apply command")]
    EmptyApply,
    #[error("mutation diff rejected: {0}")]
    MutationApply(#[from] MutationApplyError),
    #[error("nothing to undo")]
    NothingToUndo,
    #[error("cannot undo edit authored by another actor: {0}")]
    ForeignEdit(String),
    #[error("nothing to redo")]
    NothingToRedo,
    #[error("serialize error: {0}")]
    Serialize(String),
    #[error("deserialize error: {0}")]
    Deserialize(String),
    #[error("backbone error: {0}")]
    Backbone(String),
    /// @emoji 🧬️ A migration/replay/merge was attempted across two envelopes/mutations whose
    /// `dialect` coordinates don't match (see `store::ArtifactEnvelope::dialect`, `26/08/10` D4
    /// evolution slice). Not yet raised by any call site in this pass — additive only.
    #[error("dialect mismatch: {0}")]
    DialectMismatch(String),
    /// @emoji 🧬️ An operation needs a dialect migration to run first (see `store::migrate_document`)
    /// before it can proceed. Not yet raised by any call site in this pass — additive only.
    #[error("migration required: {0}")]
    MigrationRequired(String),
    /// @emoji 🧬️ A registered dialect migration ran but failed. Not yet raised by any call site in
    /// this pass — additive only.
    #[error("migration failed: {0}")]
    MigrationFailed(String),
    /// @emoji 🔁️ A composition-pin graph traversal (parent → child → …) found a cycle back to an
    /// ancestor — an owned-child forest must stay acyclic. Raised by `store::CompositionGraph::
    /// would_cycle_owns`/`would_cycle_links` via `store::CompositionCoordinator::dispatch_group`'s
    /// phase-1 validation (`UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` `🔖️CompositionCoordinator`, wave B2).
    #[error("composition cycle: {0}")]
    CompositionCycle(String),
    /// @emoji 🚫️ An operation would violate composition's single-ownership invariant (e.g.
    /// adopting a child that already has a different owner, or dispatching to a child a group's
    /// stated parent does not actually own). Raised by `store::CompositionGraph::insert_owns` and
    /// `store::CompositionCoordinator::dispatch_group`'s phase-1 ownership check.
    #[error("ownership violation: {0}")]
    OwnershipViolation(String),
    /// @emoji 🛂️ A structural failure rejected an operation during
    /// `store::CompositionCoordinator::dispatch_group`'s phase-1 pass (or the object-safe
    /// `store::SpaceMember::preview_wire`/`dispatch_wire` bridge that pass uses) — the group is
    /// aborted with zero side effects anywhere. Reserved for structural failures only (ticket
    /// `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6): an ordinary
    /// mutation-level rejection now travels as a `MutationMessage` on the op's own
    /// `MutationOutcome`, never through this variant. Additive; not raised anywhere else in this
    /// crate (every other command path reports its own more specific `VcsError` variant).
    #[error("validation failed: {0}")]
    ValidationFailed(String),
    /// @emoji 🧯️ A `CompositionCoordinator::dispatch_group` call failed AFTER some members were
    /// already applied, and the reverse-order `Undo` compensation pass (see that method's doc
    /// comment) itself failed on at least one member — i.e. the group could not be fully rolled
    /// back. The message embeds a human-readable rollback report (which members compensated
    /// cleanly, which did not, and why) so a caller can surface/log the exact partial state rather
    /// than silently losing it. This is the one path in this crate where a command's Result can
    /// legitimately leave a multi-member gesture inconsistent — every other `VcsError` variant is
    /// raised BEFORE any mutation lands.
    #[error("group dispatch failed and rollback also failed: {0}")]
    CompensationFailed(String),
    /// @emoji 🛑️ A command was rejected WHOLESALE by the authority's own `crate::os_spr::MergePolicy`
    /// — nothing in the command was applied (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-
    /// CLASS-CONFLICTS` §C6). Carries the policy that rejected it and every message the rejected
    /// replay produced, so a caller can explain the rejection without re-running anything.
    /// `ValidationFailed` survives ONLY for structural failures now — an ordinary mutation-level
    /// rejection travels through this variant instead.
    #[error("rejected by merge policy {policy:?}")]
    Rejected { policy: crate::os_spr::MergePolicy, messages: Vec<crate::os_spr::MutationMessage> },
    /// @emoji ❓️ `store::ArtifactStore::resolve_conflict` was called with an id that names no
    /// currently-`Open` conflict on this store.
    #[error("unknown conflict id: {0}")]
    UnknownConflict(String),
}

protocol::fault_from_thiserror!(VcsError, crate::os_dsl::FaultOrigin::Module, "module.vcs");

//#endregion 🔖️Errors
//#region 🔖️CollectionDiff
/// @emoji 🧩️ Sparse collection patch entry (mirrors semio_compose_rs `XModified`).
///
/// 🎞️ Canonical collection patch entry for sparse collection diffs (re-exported by `crate::os_spr`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemPatch<TId, TPatch> {
    pub id: TId,
    pub patch: TPatch,
}

/// @emoji 🧩️ Sparse collection diff (mirrors semio_compose_rs `XCollectionDiff`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionDiff<TId, TPatch, TAdded> {
    pub removed: Vec<TId>,
    pub modified: Vec<ItemPatch<TId, TPatch>>,
    pub added: Vec<TAdded>,
}

impl<TId, TPatch, TAdded> Default for CollectionDiff<TId, TPatch, TAdded> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
}
//#endregion 🔖️CollectionDiff

//#region 🔖️CollectionMutation
/// @emoji 🏷️ Identifies an item within a `Vec` by a stable id, for generic collection operations.
pub trait Identified<TId> {
    // 🚫️async: E1 pure accessor — every real caller is a std `Iterator`/`Vec` closure
    // (`retain`/`position`/`find`), `FnMut(&T) -> bool` signature fixed outside this repo and
    // cannot be async — see R9, R10 residue shape #1. Two of the three known implementors
    // (🌊️flow/🌿️vcs, ♾️infinite/…/dag) already converged on sync independently.
    fn id(&self) -> &TId;
}

/// @emoji 🩹️ Applies a patch in place and returns the patch that undoes it (captured from prior state).
pub trait Patchable<TPatch>: Sized {
    async fn apply_patch(&mut self, patch: &TPatch);
    async fn diff_patch(&self, other: &Self) -> Option<TPatch>;
}

/// @emoji 🧺️ Generic ordered-collection operation (add/remove/move/patch) with mechanical pre-state inverses.
///
/// 🎞️ `crate::os_spr::command` re-exports this very type, so `index`/`to_index` is the one wire shape
/// every caller sees — there is no second spr-side schema to keep in step.
///
/// 🗣️ Semantic-mutations overhaul ruling (`.claude/plans/the-mutations-are-extremely-compiled-pumpkin.md`):
/// this type and its three helper fns below are an INTERNAL diff/inverse ENGINE for a
/// `🧬️mutations/<kind>/{🔺️diff,↩️inverse}` triad leaf to call — e.g. a `remove-stakeholder` leaf's
/// `inverse` fn may call [`inverse_collection_mutation`] to compute the captured-item re-add. They
/// are NOT public mutation vocabulary: no `pub enum *Mutation` dispatch variant may wrap
/// `CollectionMutation<..>` directly (that erases the verb — `Add`/`Remove`/`Move`/`Patch` say
/// nothing about *why*). `policySemanticVocabularyBreaches` in `📜️script.ts` enforces this on
/// `✏️s/**/🧬️mutations/**` dispatch enums once the fan-out wave lands.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CollectionMutation<TId, TItem, TPatch> {
    Add { index: usize, item: TItem },
    Remove { id: TId },
    Move { id: TId, to_index: usize },
    Patch { id: TId, patch: TPatch },
}

/// @emoji ▶️ Applies a `CollectionMutation` to a `Vec` in place.
pub async fn apply_collection_mutation<TId, TItem, TPatch>(items: &mut Vec<TItem>, operation: &CollectionMutation<TId, TItem, TPatch>)
where
    TId: PartialEq + Clone,
    TItem: Identified<TId> + Clone + Patchable<TPatch>,
{
    match operation {
        CollectionMutation::Add { index, item } => {
            let at = (*index).min(items.len());
            items.insert(at, item.clone());
        }
        CollectionMutation::Remove { id } => {
            items.retain(|item| item.id() != id);
        }
        CollectionMutation::Move { id, to_index } => {
            if let Some(from) = items.iter().position(|item| item.id() == id) {
                let item = items.remove(from);
                let at = (*to_index).min(items.len());
                items.insert(at, item);
            }
        }
        CollectionMutation::Patch { id, patch } => {
            if let Some(item) = items.iter_mut().find(|item| item.id() == id) {
                item.apply_patch(patch).await;
            }
        }
    }
}

/// @emoji ↩️ Computes the inverse `CollectionMutation` from the pre-state `items`. Panics if `operation` targets
/// an id absent from `items` (Remove/Move/Patch always target an existing item by construction).
pub async fn inverse_collection_mutation<TId, TItem, TPatch>(items: &[TItem], operation: &CollectionMutation<TId, TItem, TPatch>) -> CollectionMutation<TId, TItem, TPatch>
where
    TId: PartialEq + Clone,
    TItem: Identified<TId> + Clone + Patchable<TPatch>,
{
    match operation {
        CollectionMutation::Add { item, .. } => CollectionMutation::Remove { id: item.id().clone() },
        CollectionMutation::Remove { id } => {
            let index = items.iter().position(|item| item.id() == id).expect("remove target must exist in pre-state");
            CollectionMutation::Add { index, item: items[index].clone() }
        }
        CollectionMutation::Move { id, .. } => {
            let index = items.iter().position(|item| item.id() == id).expect("move target must exist in pre-state");
            CollectionMutation::Move { id: id.clone(), to_index: index }
        }
        CollectionMutation::Patch { id, patch } => {
            let prior = items.iter().find(|item| item.id() == id).cloned().expect("patch target must exist in pre-state");
            let mut after = prior.clone();
            after.apply_patch(patch).await;
            let inverse_patch = after.diff_patch(&prior).await.expect("a patch that changed state must yield a computable inverse");
            CollectionMutation::Patch { id: id.clone(), patch: inverse_patch }
        }
    }
}

/// @emoji 🧮️ Projects a `CollectionMutation` onto a sparse {@link CollectionDiff}, so a plugin's
/// `Mutation::diff` can produce a diff in one call instead of hand-writing `removed`/`modified`/
/// `added`. `Add` → `added`, `Remove` → `removed`, `Patch` → `modified`. `CollectionDiff` has no
/// positional-move channel, so `Move` is encoded as `removed` + `added` (delete then re-add by
/// identity); a plugin that keeps items keyed by id reconstructs order from item identity.
pub async fn collection_diff_from_mutation<TId, TItem, TPatch>(items: &[TItem], operation: &CollectionMutation<TId, TItem, TPatch>) -> CollectionDiff<TId, TPatch, TItem>
where
    TId: PartialEq + Clone,
    TItem: Identified<TId> + Clone,
    TPatch: Clone,
{
    let mut diff = CollectionDiff::default();
    match operation {
        CollectionMutation::Add { item, .. } => diff.added.push(item.clone()),
        CollectionMutation::Remove { id } => diff.removed.push(id.clone()),
        CollectionMutation::Patch { id, patch } => diff.modified.push(ItemPatch { id: id.clone(), patch: patch.clone() }),
        CollectionMutation::Move { id, .. } => {
            if let Some(item) = items.iter().find(|item| item.id() == id) {
                diff.removed.push(id.clone());
                diff.added.push(item.clone());
            }
        }
    }
    diff
}
//#endregion 🔖️CollectionMutation
//#region 🔖️Mutation
// 🎞️ `Mutation`/`MutationDiff`/`MutationMessage` live in `protocol_command`; this region just
// replays a snapshot through an operation's forward diff — the pure per-step transform every
// store-level replay uses.

/// @emoji ▶️ Computes `operation.diff(snapshot)`, applies the resulting diff, and returns the new
/// snapshot alongside every [`crate::os_spr::MutationMessage`] the outcome carried. Diff-apply
/// rejection is returned as its structured [`MutationApplyError`] before a snapshot is produced. A `Fatal`
/// message's diff is `D::default()` by construction (§C2 LAW 1), so applying it is always a no-op —
/// callers that must not silently apply a rejected op check `worst_level(&messages)` against their
/// `MergePolicy` themselves (this fn stays policy-agnostic, matching its old unconditional-apply
/// shape).
pub async fn apply_mutation<P, Mutation>(snapshot: &P, operation: &Mutation) -> Result<(P, Vec<crate::os_spr::MutationMessage>), MutationApplyError>
where
    Mutation: self::Mutation<P>,
{
    let (diff, messages) = operation.diff(snapshot).await.into_parts().await;
    Ok((diff.apply(snapshot).await?, messages))
}

//#endregion 🔖️Mutation
//#region 🔖️MergeStrategy
// 🎞️ The CRDT-era concurrent-diff merge helper this region used to point at is deleted
// (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`) — concurrent-merge
// arbitration is now an authority's `MergePolicy`/`📡️spr/⚔️conflict` job. The checkpoint-ancestor/
// merge-base helpers that used to live in this region moved to `store` along with `ArtifactEnvelope`
// (`checkpoint_ancestors`/`merge_base`/`reconcile_alternative` all take an envelope) — only the
// envelope-free id-minting primitive stays here.

/// @emoji 🔒️ Content-addressed checkpoint id: `ck-<hex16(blake3(parent_id || ordered_change_content_
/// hashes || message || authors || timestamp [|| ordered_pin_content]))>`, replacing the old fully-
/// random counter-string scheme (`create_document_vcs_id("checkpoint")`) — two peers that
/// independently commit the identical checkpoint content (same parent, same changes in the same
/// order, same message/authors/timestamp, same composition pins) now converge on the identical id
/// instead of minting two different ones. `changes` must already contain every entry `change_ids`
/// references (including one freshly created by this same commit, if any) — callers push a new
/// `Change` before calling this.
///
/// 🎯️ `pins` extension (composition-aware checkpoints): appended to the hash input ONLY when
/// non-empty, so a non-composite checkpoint (the overwhelming majority, and every checkpoint ever
/// minted before this ticket) hashes to EXACTLY the pre-existing bytes — this is what keeps old ids
/// stable, not a version bump. `pins` is re-sorted by `child_ref.to_uri()` (see [`CompositionPin`])
/// inside this function rather than trusted in caller-supplied order: a caller
/// building the pin list from a `HashMap`/parallel-dispatch fan-out over owned children has no
/// natural deterministic order of its own, and two peers committing the identical pin SET must
/// still converge on the identical id regardless of which order their local dispatch happened to
/// discover the children in.
pub async fn content_addressed_checkpoint_id(parent_id: Option<&str>, change_ids: &[String], changes: &[Change], message: Option<&str>, authors: &[Author], timestamp: &str, pins: &[CompositionPin]) -> String {
    let mut input = Vec::new();
    input.extend_from_slice(parent_id.unwrap_or("").as_bytes());
    input.push(0);
    for change_id in change_ids {
        let change_hash = changes.iter().find(|change| change.id == *change_id).map(|change| *blake3::hash(&serde_json::to_vec(change).unwrap_or_default()).as_bytes()).unwrap_or([0u8; 32]);
        input.extend_from_slice(&change_hash);
    }
    input.push(0);
    input.extend_from_slice(message.unwrap_or("").as_bytes());
    input.push(0);
    for author in authors {
        input.extend_from_slice(author.id.as_bytes());
        input.push(0);
    }
    input.push(0);
    input.extend_from_slice(timestamp.as_bytes());
    if !pins.is_empty() {
        // 🪡️ `to_uri` (🚪️io, out of this packet's scope) is async — `Iterator::map`'s closure is
        // sync (E0728), so the await is hoisted into a plain loop before the sort (R10 residue #1).
        let mut ordered: Vec<(String, &CompositionPin)> = Vec::with_capacity(pins.len());
        for pin in pins {
            ordered.push((pin.child_ref.to_uri(), pin));
        }
        ordered.sort_by(|(a, _), (b, _)| a.cmp(b));
        input.push(0);
        for (uri, pin) in ordered {
            input.extend_from_slice(uri.as_bytes());
            input.push(0);
            input.extend_from_slice(pin.checkpoint_id.as_bytes());
            input.push(0);
        }
    }
    let digest = *blake3::hash(&input).as_bytes();
    let hex16: String = digest[..8].iter().map(|byte| format!("{byte:02x}")).collect();
    format!("ck-{hex16}")
}
//#endregion 🔖️MergeStrategy

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct DemoItem {
        id: String,
        value: i32,
    }

    impl Identified<String> for DemoItem {
        fn id(&self) -> &String {
            &self.id
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    struct DemoItemPatch {
        value: Option<i32>,
    }

    impl Patchable<DemoItemPatch> for DemoItem {
        async fn apply_patch(&mut self, patch: &DemoItemPatch) {
            if let Some(value) = patch.value {
                self.value = value;
            }
        }

        async fn diff_patch(&self, other: &Self) -> Option<DemoItemPatch> {
            (self.value != other.value).then(|| DemoItemPatch { value: Some(other.value) })
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn collection_diff_from_op_projects_each_variant() {
        let items: Vec<DemoItem> = vec![DemoItem { id: "a".into(), value: 1 }, DemoItem { id: "b".into(), value: 2 }];
        let added = collection_diff_from_mutation::<String, DemoItem, DemoItemPatch>(&items, &CollectionMutation::Add { index: 0, item: DemoItem { id: "c".into(), value: 3 } }).await;
        assert_eq!(added.added.len(), 1);
        assert!(added.removed.is_empty() && added.modified.is_empty());

        let removed = collection_diff_from_mutation::<String, DemoItem, DemoItemPatch>(&items, &CollectionMutation::Remove { id: "a".into() }).await;
        assert_eq!(removed.removed, vec!["a".to_string()]);

        let patched = collection_diff_from_mutation(&items, &CollectionMutation::Patch { id: "b".into(), patch: DemoItemPatch { value: Some(9) } }).await;
        assert_eq!(patched.modified.len(), 1);
        assert_eq!(patched.modified[0].id, "b");

        let moved = collection_diff_from_mutation::<String, DemoItem, DemoItemPatch>(&items, &CollectionMutation::Move { id: "a".into(), to_index: 1 }).await;
        assert_eq!(moved.removed, vec!["a".to_string()], "move is encoded as remove + re-add by identity");
        assert_eq!(moved.added.len(), 1);
        assert_eq!(moved.added[0].id, "a");
    }

    #[semio_framework_async_macros::async_test]
    async fn collection_op_add_and_invert() {
        let items: Vec<DemoItem> = vec![DemoItem { id: "a".into(), value: 1 }];
        let operation = CollectionMutation::Add { index: 1, item: DemoItem { id: "b".into(), value: 2 } };
        let mut applied = items.clone();
        apply_collection_mutation(&mut applied, &operation).await;
        assert_eq!(applied.len(), 2);
        assert_eq!(applied[1].id, "b");
        let inverse = inverse_collection_mutation(&items, &operation);
        apply_collection_mutation(&mut applied, &inverse.await).await;
        assert_eq!(applied, items);
    }

    #[semio_framework_async_macros::async_test]
    async fn collection_op_move_and_invert() {
        let items: Vec<DemoItem> = vec![DemoItem { id: "a".into(), value: 1 }, DemoItem { id: "b".into(), value: 2 }, DemoItem { id: "c".into(), value: 3 }];
        let operation = CollectionMutation::Move { id: "a".into(), to_index: 2 };
        let mut applied = items.clone();
        apply_collection_mutation(&mut applied, &operation).await;
        assert_eq!(applied.iter().map(|i| i.id.clone()).collect::<Vec<_>>(), vec!["b", "c", "a"]);
        let inverse = inverse_collection_mutation(&items, &operation);
        apply_collection_mutation(&mut applied, &inverse.await).await;
        assert_eq!(applied, items);
    }

    #[semio_framework_async_macros::async_test]
    async fn collection_op_patch_and_invert() {
        let items: Vec<DemoItem> = vec![DemoItem { id: "a".into(), value: 1 }];
        let operation = CollectionMutation::Patch { id: "a".into(), patch: DemoItemPatch { value: Some(9) } };
        let mut applied = items.clone();
        apply_collection_mutation(&mut applied, &operation).await;
        assert_eq!(applied[0].value, 9);
        let inverse = inverse_collection_mutation(&items, &operation);
        apply_collection_mutation(&mut applied, &inverse.await).await;
        assert_eq!(applied, items);
    }

    #[semio_framework_async_macros::async_test]
    async fn collection_op_remove_and_invert() {
        let items: Vec<DemoItem> = vec![DemoItem { id: "a".into(), value: 1 }, DemoItem { id: "b".into(), value: 2 }];
        let operation = CollectionMutation::Remove { id: "a".into() };
        let mut applied = items.clone();
        apply_collection_mutation(&mut applied, &operation).await;
        assert_eq!(applied.len(), 1);
        let inverse = inverse_collection_mutation(&items, &operation);
        apply_collection_mutation(&mut applied, &inverse.await).await;
        assert_eq!(applied, items);
    }

    //#endregion 🔖️ReconcileAlternative

    //#region 🔖️ContentAddressedCheckpointAndMergeBase
    #[semio_framework_async_macros::async_test]
    async fn content_addressed_checkpoint_id_is_deterministic_and_content_sensitive() {
        let root_change = Change { id: "change-root".into(), edit_ids: vec!["edit-1".into()], description: Some("root".into()), saved_at: "2026-07-27T00:00:00Z".into() };
        let changes = vec![root_change];
        let change_ids = vec!["change-root".to_string()];
        let authors = vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }];

        let id_a = content_addressed_checkpoint_id(None, &change_ids, &changes, Some("root"), &authors, "2026-07-27T00:00:01Z", &[]).await;
        let id_b = content_addressed_checkpoint_id(None, &change_ids, &changes, Some("root"), &authors, "2026-07-27T00:00:01Z", &[]).await;
        assert_eq!(id_a, id_b, "identical inputs converge on the identical id");
        assert!(id_a.starts_with("ck-"), "got {id_a}");

        let id_different_message = content_addressed_checkpoint_id(None, &change_ids, &changes, Some("other message"), &authors, "2026-07-27T00:00:01Z", &[]).await;
        assert_ne!(id_a, id_different_message, "a different message must change the id");

        let id_different_parent = content_addressed_checkpoint_id(Some("ck-parent"), &change_ids, &changes, Some("root"), &authors, "2026-07-27T00:00:01Z", &[]).await;
        assert_ne!(id_a, id_different_parent, "a different parent must change the id");

        let id_different_timestamp = content_addressed_checkpoint_id(None, &change_ids, &changes, Some("root"), &authors, "2026-07-27T00:00:02Z", &[]).await;
        assert_ne!(id_a, id_different_timestamp, "a different timestamp must change the id");
    }

    /// @emoji 🧩️ `composition_pins`/`CompositionPin` extension to `content_addressed_checkpoint_id`:
    /// the three properties the ticket calls for — pin-set changes flip the id, identical
    /// pins-in-identical-order converge, and (critically) an EMPTY pin list must hash to the exact
    /// same bytes `content_addressed_checkpoint_id` produced before this field existed, so every
    /// checkpoint id ever minted for a non-composite artifact stays valid.
    #[semio_framework_async_macros::async_test]
    async fn content_addressed_checkpoint_id_composition_pins_are_deterministic_and_backward_compatible() {
        let root_change = Change { id: "change-root".into(), edit_ids: vec!["edit-1".into()], description: Some("root".into()), saved_at: "2026-07-27T00:00:00Z".into() };
        let changes = vec![root_change];
        let change_ids = vec!["change-root".to_string()];
        let authors = vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }];
        let args = (None, &change_ids, &changes, Some("root"), &authors, "2026-07-27T00:00:01Z");

        // (1) Empty pins must reproduce the pre-pins hash bytes EXACTLY — recomputed here via the
        // same blake3(parent||changes||message||authors||timestamp) formula
        // `content_addressed_checkpoint_id` used before the `pins` parameter was added, so this is
        // a byte-level backward-compatibility proof, not just "doesn't panic".
        let mut legacy_input = Vec::new();
        legacy_input.extend_from_slice(args.0.unwrap_or("").as_bytes());
        legacy_input.push(0);
        for change_id in args.1 {
            let change_hash = args.2.iter().find(|change| change.id == *change_id).map(|change| *blake3::hash(&serde_json::to_vec(change).unwrap_or_default()).as_bytes()).unwrap_or([0u8; 32]);
            legacy_input.extend_from_slice(&change_hash);
        }
        legacy_input.push(0);
        legacy_input.extend_from_slice(args.3.unwrap_or("").as_bytes());
        legacy_input.push(0);
        for author in args.4 {
            legacy_input.extend_from_slice(author.id.as_bytes());
            legacy_input.push(0);
        }
        legacy_input.push(0);
        legacy_input.extend_from_slice(args.5.as_bytes());
        let legacy_digest = *blake3::hash(&legacy_input).as_bytes();
        let legacy_hex16: String = legacy_digest[..8].iter().map(|byte| format!("{byte:02x}")).collect();
        let legacy_id = format!("ck-{legacy_hex16}");
        let id_no_pins = content_addressed_checkpoint_id(args.0, args.1, args.2, args.3, args.4, args.5, &[]).await;
        assert_eq!(id_no_pins, legacy_id, "an empty pin list must not change a single byte of the pre-existing hash input");

        // (2) A non-empty pin set changes the id relative to no pins at all.
        let child_a_ref = crate::os_io::ArtifactRef::parse_uri("child-a!s.stdio.mesh@87a/mesh").expect("valid test fixture uri");
        let child_b_ref = crate::os_io::ArtifactRef::parse_uri("child-b!s.stdio.image@87a/image").expect("valid test fixture uri");
        let pins_one = vec![CompositionPin { child_ref: child_a_ref.clone(), checkpoint_id: "ck-child-a-1".into() }];
        let id_with_pins = content_addressed_checkpoint_id(args.0, args.1, args.2, args.3, args.4, args.5, &pins_one).await;
        assert_ne!(id_no_pins, id_with_pins, "a non-empty pin list must change the id relative to no composition");

        // (3) Identical pins in identical order converge on the identical id.
        let id_with_pins_again = content_addressed_checkpoint_id(args.0, args.1, args.2, args.3, args.4, args.5, &pins_one).await;
        assert_eq!(id_with_pins, id_with_pins_again, "identical pins in identical order converge on the identical id");

        // (4) A different pin CONTENT (same child, different pinned checkpoint) changes the id.
        let pins_one_moved = vec![CompositionPin { child_ref: child_a_ref.clone(), checkpoint_id: "ck-child-a-2".into() }];
        let id_pin_moved = content_addressed_checkpoint_id(args.0, args.1, args.2, args.3, args.4, args.5, &pins_one_moved);
        assert_ne!(id_with_pins, id_pin_moved.await, "a different pinned checkpoint_id for the same child must change the id");

        // (5) Two peers that discover the same pin SET in different order (e.g. concurrent
        // parallel-child dispatch) still converge — the function sorts by `child_ref.to_uri()` internally.
        let pins_two_ordered = vec![
            CompositionPin { child_ref: child_a_ref.clone(), checkpoint_id: "ck-child-a-1".into() },
            CompositionPin { child_ref: child_b_ref.clone(), checkpoint_id: "ck-child-b-1".into() },
        ];
        let pins_two_reordered = vec![
            CompositionPin { child_ref: child_b_ref, checkpoint_id: "ck-child-b-1".into() },
            CompositionPin { child_ref: child_a_ref, checkpoint_id: "ck-child-a-1".into() },
        ];
        let id_ordered = content_addressed_checkpoint_id(args.0, args.1, args.2, args.3, args.4, args.5, &pins_two_ordered);
        let id_reordered = content_addressed_checkpoint_id(args.0, args.1, args.2, args.3, args.4, args.5, &pins_two_reordered);
        assert_eq!(id_ordered.await, id_reordered.await, "two peers discovering the same pin set in different incidental order must converge on the identical id");
    }

    //#region 🆔️Ids
    #[semio_framework_async_macros::async_test]
    async fn content_addressed_entity_and_mint_helpers_are_deterministic() {
        assert_eq!(content_addressed_entity_id("x", b"payload").await, content_addressed_entity_id("x", b"payload").await);
        assert_ne!(content_addressed_entity_id("x", b"a").await, content_addressed_entity_id("x", b"b").await);
        assert_eq!(edit_scoped_id("edit-1", 0).await, edit_scoped_id("edit-1", 0).await);
        assert_ne!(edit_scoped_id("edit-1", 0).await, edit_scoped_id("edit-1", 1).await);
        assert!(edit_scoped_id("edit-1", 0).await.starts_with("scoped-"));
        assert_eq!(mint_edit_id(Some("alice"), 3, b"fwd").await, mint_edit_id(Some("alice"), 3, b"fwd").await);
        assert_ne!(mint_edit_id(Some("alice"), 3, b"fwd").await, mint_edit_id(Some("bob"), 3, b"fwd").await);
        assert_eq!(mint_change_id(&["e1".into(), "e2".into()], Some("msg")).await, mint_change_id(&["e1".into(), "e2".into()], Some("msg")).await);
        assert_eq!(mint_alternative_id("main", &["ck1".into()]).await, mint_alternative_id("main", &["ck1".into()]).await);
        assert_eq!(mint_mutation_id(b"op-bytes").await, mint_mutation_id(b"op-bytes").await);
        assert_eq!(create_document_vcs_id("draft").await, create_document_vcs_id("draft").await);
        assert!(create_document_vcs_id("draft").await.starts_with("draft-"));
    }
    //#endregion 🆔️Ids
}
//#endregion 🧪️Tests
