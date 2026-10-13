//! 🧬️ Store schemas: the command vocabulary (`ArtifactCommand` and friends), cursors and envelopes, history lanes,
//! projection stamps and events. Plain data every layer of the Store reads.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
use super::operation_envelope_serde;
use crate::os_vcs::io::binary::genesis::{AdmittedArtifactGenesis, ArtifactGenesisCodec};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use crate::os_spr::command::ArtifactMessageSource;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use crate::os_spr::{MutationId, UndoPolicy};
use crate::os_spr::{DiffRegions, Mutation, MutationDiff, OpBinary, OpText};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::os_vcs::Author;
use crate::os_vcs::{
    ArtifactVcs, Identified, Patchable,
};
use super::artifact_child_owner_schema::OwnerRef;
use crate::os_pack::pack_rt;
use super::ledger::ArtifactEditMessageLedger;
use super::ledger::retire_scratch_edits;
use super::canonical_edit::{
    ArtifactCanonicalJson, ArtifactCanonicalJsonTree,
};
use super::{COMMAND_BINARY_FORMAT, CommandDecodeError, EditReplay, ReplayProgress, conflict_resolution_from_ordinal, conflict_resolution_ordinal, history_lane_from_ordinal, history_lane_ordinal, read_command_ops, read_command_str, read_command_transaction, read_supersede_inputs, retire_command, retire_shared_projection, undo_policy_from_ordinal, undo_policy_ordinal, write_command_ops, write_command_str, write_command_transaction, write_supersede_inputs};

/// 🔗️ Identifies the channel a document synchronizes through, when one is attached.
/// 🌱️ `Serialize`/`Deserialize` dropped outright (not even `cfg_attr(test, …)`): no production
/// consumer (`🖥️host/🦀️.rs`'s `BackboneDocument`, `🔌️plugin/🦀️.rs`'s `backbone_ref()`) requires
/// `ArtifactBackboneRef: Serialize`, and no test in this crate uses `serde_json` as a differential
/// oracle on it — confirmed by repo-wide grep before removing the derive.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ArtifactBackboneRef {
    pub uri: String,
}

/// 🔗️ Builds a backbone reference from a channel URI.
pub async fn document_backbone_ref(uri: &str) -> ArtifactBackboneRef {
    ArtifactBackboneRef { uri: uri.to_string() }
}

/// 🎯️ Undo/redo/checkout position — derived by folding the event log, never persisted.
/// Carries the FULL applied-edit list (not just the tail edit id): an edit undone mid-history
/// precedes later-applied edits in file order, and the redo stack can contain edits in any order
/// relative to `applied_edit_ids` — a single marker id cannot represent that. `checkpoint_id`
/// mirrors `ArtifactStore::current_checkpoint_id`.
/// 🌱️ `serde` is TEST-ONLY (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01):
/// `BackboneDocument` in `💻️os/🖥️host` — the only production consumer that forced `ArtifactCursor`/
/// `ArtifactCursorOwners` to stay dual — moved to a hand-written `ToValue`/`FromValue` impl (see
/// `📓️os-kernel-serde-final.md`). Production already routes through `ToValue`/`FromValue`; this
/// wave's own test fixtures (`serde_json::from_value` oracle comparisons below) still need the
/// derive, hence `cfg_attr(test, …)` rather than deleting it outright.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ArtifactCursorOwners {
    #[value(default, skip_serializing_if = "crate::os_vcs::history_id_stack_is_empty")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "crate::os_vcs::history_id_stack_is_empty"))]
    pub applied_edit_ids: crate::os_vcs::HistoryPageStack<String>,
    #[value(default, skip_serializing_if = "crate::os_vcs::history_id_stack_is_empty")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "crate::os_vcs::history_id_stack_is_empty"))]
    pub redo_edit_ids: crate::os_vcs::HistoryPageStack<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub checkpoint_id: Option<String>,
}

/// 🧭 Read-visible cursor root; prepared group owners stay private until their shared decision.
pub struct ArtifactCursor {
    pub(crate) owners: std::mem::ManuallyDrop<ArtifactCursorOwners>,
    pub(crate) group: std::mem::ManuallyDrop<Option<ArtifactCursorGroupRoot>>,
}

pub(crate) struct ArtifactCursorGroupRoot {
    pub(crate) visibility: Arc<crate::os_vcs::ArtifactGroupVisibility>,
    pub(crate) owners: ArtifactCursorOwners,
}

impl ArtifactCursor {
    pub fn new(applied_edit_ids: impl Into<crate::os_vcs::HistoryPageStack<String>>, redo_edit_ids: impl Into<crate::os_vcs::HistoryPageStack<String>>, checkpoint_id: Option<String>) -> Self {
        Self::from_owners(ArtifactCursorOwners { applied_edit_ids: applied_edit_ids.into(), redo_edit_ids: redo_edit_ids.into(), checkpoint_id })
    }

    pub fn from_owners(owners: ArtifactCursorOwners) -> Self {
        Self { owners: std::mem::ManuallyDrop::new(owners), group: std::mem::ManuallyDrop::new(None) }
    }

    pub(crate) fn stage_group_owned(&mut self, owners: ArtifactCursorOwners, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>) -> Result<(), ArtifactCursorOwners> {
        if self.group.is_some() || !visibility.pending() {
            return Err(owners);
        }
        *self.group = Some(ArtifactCursorGroupRoot { visibility: Arc::clone(visibility), owners });
        Ok(())
    }

    pub(crate) fn adopt_group_owned(&mut self, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>) -> Result<ArtifactCursorOwners, ()> {
        if !visibility.committed() || !self.group.as_ref().is_some_and(|group| Arc::ptr_eq(&group.visibility, visibility)) {
            return Err(());
        }
        let group = self.group.take().expect("validated committed cursor root remains owned");
        Ok(std::mem::replace(&mut *self.owners, group.owners))
    }

    pub(crate) fn abort_group_owned(&mut self, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>) -> Result<ArtifactCursorOwners, ()> {
        if visibility.pending() || visibility.committed() || !self.group.as_ref().is_some_and(|group| Arc::ptr_eq(&group.visibility, visibility)) {
            return Err(());
        }
        Ok(self.group.take().expect("validated aborted cursor root remains owned").owners)
    }

    pub(crate) fn group_visibility(&self) -> Option<&crate::os_vcs::ArtifactGroupVisibility> {
        self.group.as_ref().map(|group| group.visibility.as_ref())
    }

    fn read_group(&self, decision: Option<&crate::os_vcs::ArtifactGroupReadDecision<'_>>) -> Result<&ArtifactCursorOwners, ()> {
        match self.group.as_ref() {
            Some(group) if decision.ok_or(())?.committed_for(&group.visibility)? => Ok(&group.owners),
            _ => Ok(&*self.owners),
        }
    }
}

impl Default for ArtifactCursor {
    fn default() -> Self {
        Self::from_owners(ArtifactCursorOwners::default())
    }
}

impl std::ops::Deref for ArtifactCursor {
    type Target = ArtifactCursorOwners;

    fn deref(&self) -> &Self::Target {
        self.group.as_ref().filter(|group| group.visibility.committed()).map_or(&*self.owners, |group| &group.owners)
    }
}

impl std::ops::DerefMut for ArtifactCursor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        assert!(self.group.is_none(), "mutable cursor access requires its staged group to be adopted or aborted");
        &mut self.owners
    }
}

impl Clone for ArtifactCursor {
    fn clone(&self) -> Self {
        Self::from_owners((**self).clone())
    }
}

impl std::fmt::Debug for ArtifactCursor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (**self).fmt(formatter)
    }
}

impl PartialEq for ArtifactCursor {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl ToValue for ArtifactCursor {
    fn to_value(&self) -> DslValue {
        (**self).to_value()
    }
}

impl FromValue for ArtifactCursor {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        Ok(Self::from_owners(ArtifactCursorOwners::from_value(value)?))
    }
}

/// 🌉️ serde twin of the `ToValue`/`FromValue` pair above — TEST-ONLY
/// (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01): `BackboneDocument` in
/// `💻️os/🖥️host`, the only production consumer that forced this to stay unconditional, moved to a
/// hand-written `ToValue`/`FromValue` impl (see `📓️os-kernel-serde-final.md`). Delegates through
/// the same `ArtifactCursorOwners` view (itself now also `cfg_attr(test, …)`-gated) so both
/// encodings emit an identical shape. Hand-written rather than derived because the private fields
/// are `ManuallyDrop` (the staged group root must be adopted or retired before `Drop`), which no
/// derive can see through.
#[cfg(test)]
impl Serialize for ArtifactCursor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Serialize::serialize(&**self, serializer)
    }
}

/// 🌉️ Mirror of the `Serialize` twin directly above — also test-only.
#[cfg(test)]
impl<'de> Deserialize<'de> for ArtifactCursor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        ArtifactCursorOwners::deserialize(deserializer).map(Self::from_owners)
    }
}

impl Drop for ArtifactCursor {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.group.is_none()), "artifact cursor reached Drop before its exact staged root was adopted or returned for retirement");
        unsafe { std::mem::ManuallyDrop::drop(&mut self.owners) };
    }
}

/// 🛤️ Which undo/redo cursor a recorded edit belongs to. `Document` is the default — the
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub enum HistoryLane {
    #[default]
    Document,
    Interaction,
}

impl HistoryLane {
    pub(crate) async fn is_document(&self) -> bool {
        matches!(self, HistoryLane::Document)
    }
}

#[derive(Debug, PartialEq)]
pub struct ArtifactEnvelopeOwners<P, Mutation> {
    pub schema: String,
    pub id: String,
    pub vcs: ArtifactVcs<AdmittedArtifactGenesis<P>, Mutation>,
    pub backbone: Option<ArtifactBackboneRef>,
    pub active_alternative_id: Option<String>,
    /// 🎯️ Undo/redo/checkout position, present only once a store has synced it (see
    /// `ArtifactStore::sync_cursor`) — absent for a freshly-constructed envelope or one loaded
    /// from a source that predates this field, in which case position stays runtime-only exactly
    /// as before.
    pub cursor: Option<ArtifactCursor>,
    /// 🗣️ The dialect this envelope's `vcs.initial_snapshot` (and every replayed edit) is
    /// currently in — absent for envelopes minted before dialect-tracking existed, or for document
    /// kinds that never adopted more than one dialect. See `26/08/10` D4 evolution slice; nothing
    /// in `ArtifactStore::dispatch` reads or writes this yet (that wiring is later scope) — it is
    /// purely a persisted fact a future migration-aware caller can act on.
    pub dialect: Option<semio_framework_artifact_reference::ArtifactDialect>,
    /// 🧬️ Set once, the first time this envelope's snapshot was produced by migrating a
    /// prior document's dialect (see `migrate_document` below) rather than being authored directly
    /// in `dialect`. Absent for every envelope that was never migrated.
    pub migrated_from: Option<MigrationProvenance>,
    /// 🏠️ Present exactly when this envelope is a CHILD in a composition — the ownership
    /// stamp naming which parent/slot/`child_id` created it (see `🔖️Composition` below). Placed on
    /// the CHILD's own envelope (not only on the parent's `ArtifactChild` handle) so ownership is
    /// queryable directly from the child side — e.g. "is this document embeddable standalone, or
    /// does deleting it require going through its owner". Absent for every independent artifact
    /// (the overwhelming majority) and for every envelope minted before this ticket.
    pub owner: Option<OwnerRef>,
    /// 🛤️ Sparse `Edit.id → HistoryLane` ledger: only entries recorded under a NON-`Document`
    /// lane are ever inserted (an ordinary document edit never gets a map entry at all), so an id
    /// absent from this map is `HistoryLane::Document` by construction — see
    /// `ArtifactStore::edit_lane`. Additive — absent (empty map) for every envelope minted before
    /// this field existed, all of which therefore decode as if every edit were `Document` lane,
    /// matching prior undo/redo behavior exactly. Lives on the envelope (not on `Edit<Mutation>`
    /// itself, which this crate's per-technology `Mutation` types don't own) so it survives a plain
    /// JSON round trip (`ArtifactStore::envelope_json`) alongside `cursor`/`owner`/`dialect`.
    pub lanes: protocol::HistoryFoldIndex<String, HistoryLane>,
    /// 👁 Explicit checkpoint this viewer is looking at. `None` is the tip of the active line.
    /// Local to this replica: a collaborator's checkout does not write it.
    pub viewer_checkpoint_id: Option<String>,
    pub edit_messages: ArtifactEditMessageLedger,
    pub conflicts: Vec<crate::os_spr::Conflict>,
    /// 🔀️ Every structural history event — undo, redo, checkpoint commit, branch, checkout,
    /// repin — as one causal `crate::os_spr::HISTORY_TRANSITION_SCHEMA` envelope, in HLC order.
    /// Together with `vcs.edits` this is the document's complete semantic event log. The
    /// change/checkpoint/alternative ledgers are its projection. `active_alternative_id` and
    /// `viewer_checkpoint_id` are this replica's head, persisted local-only, not that projection.
    pub transitions: Vec<crate::os_spr::MutationEnvelope>,
    /// 🗂️ The transitions this history holds ([`crate::os_spr::HistoryShape`]): a config store's history holds undo and
    /// redo only, so its store refuses every other transition at dispatch and at remote admission — the same law its
    /// retained loader applies.
    pub history_shape: crate::os_spr::HistoryShape,
    /// 🎐️ The tool transaction whose edit is open in `vcs.edits` ([`ArtifactCommand::AppendTransaction`]). The store folds
    /// that edit as its applied tail, but neither the shared log ([`ArtifactStore::event_log`]) nor a persisted form
    /// ([`print_document_spr`], [`print_document_text`]) holds it before its commit, so an abort or a lost session leaves
    /// no trace.
    pub open_transaction: Option<OpenToolTransaction>,
}

/// 📸️ One immutable envelope observation; cursor and every history ledger share one captured decision.
pub struct ArtifactEnvelopeRead<'a, P, Mutation> {
    schema: &'a str,
    id: &'a str,
    vcs: crate::os_vcs::ArtifactVcsRead<'a, AdmittedArtifactGenesis<P>, Mutation>,
    backbone: Option<&'a ArtifactBackboneRef>,
    active_alternative_id: Option<&'a String>,
    cursor: Option<&'a ArtifactCursorOwners>,
    dialect: Option<&'a semio_framework_artifact_reference::ArtifactDialect>,
    migrated_from: Option<&'a MigrationProvenance>,
    owner: Option<&'a OwnerRef>,
    lanes: &'a protocol::HistoryFoldIndex<String, HistoryLane>,
    viewer_checkpoint_id: Option<&'a String>,
    edit_messages: &'a ArtifactEditMessageLedger,
    conflicts: &'a Vec<crate::os_spr::Conflict>,
}

/// 🌉️ Hand-written, not derived: every field but `vcs` is a plain reference or `Option<&T>`, and
/// no blanket `impl<T: ToValue> ToValue for &T` exists in this codebase (see `ArtifactVcsRead`'s
/// own docstring) — mirrors the former `#[derive(Serialize)]`'s `skip_serializing_if` shape by
/// simply omitting the key when the option is `None`/the map is empty.
impl<P: ToValue, Mutation: ToValue> ToValue for ArtifactEnvelopeRead<'_, P, Mutation> {
    fn to_value(&self) -> DslValue {
        let mut entries = vec![("schema".to_string(), self.schema.to_value()), ("id".to_string(), self.id.to_value()), ("vcs".to_string(), crate::os_vcs::io::text::vcs::native_vcs_read_value(&self.vcs))];
        if let Some(backbone) = self.backbone {
            entries.push(("backbone".to_string(), backbone.to_value()));
        }
        if let Some(active_alternative_id) = self.active_alternative_id {
            entries.push(("activeAlternativeId".to_string(), active_alternative_id.to_value()));
        }
        if let Some(cursor) = self.cursor {
            entries.push(("cursor".to_string(), cursor.to_value()));
        }
        if let Some(dialect) = self.dialect {
            entries.push(("dialect".to_string(), dialect.to_value()));
        }
        if let Some(migrated_from) = self.migrated_from {
            entries.push(("migratedFrom".to_string(), migrated_from.to_value()));
        }
        if let Some(owner) = self.owner {
            entries.push(("owner".to_string(), owner.to_value()));
        }
        if !self.lanes.is_empty() {
            entries.push(("lanes".to_string(), self.lanes.to_value()));
        }
        if let Some(viewer_checkpoint_id) = self.viewer_checkpoint_id {
            entries.push(("viewerCheckpointId".to_string(), viewer_checkpoint_id.to_value()));
        }
        entries.push(("editMessages".to_string(), self.edit_messages.to_value()));
        entries.push(("conflicts".to_string(), self.conflicts.to_value()));
        DslValue::Object(entries)
    }
}

impl<P, Mutation> ArtifactEnvelopeOwners<P, Mutation> {
    /// 🖇️ Whether `edit_id` is committed history: every edit but the open tool transaction's, which the shared log and
    /// every persisted form leave out until its commit.
    pub fn holds_committed_edit(&self, edit_id: &str) -> bool {
        self.open_transaction.as_ref().is_none_or(|open| open.edit_id != edit_id)
    }

    /// 📸️ Captures read roots before any serializer can cross a shared group decision.
    pub fn capture_read(&self) -> Result<ArtifactEnvelopeRead<'_, P, Mutation>, &'static str> {
        let history_visibility = self.vcs.group_visibility().map_err(|()| "envelope history contains different group authorities")?;
        let cursor_visibility = self.cursor.as_ref().and_then(ArtifactCursor::group_visibility);
        if history_visibility.zip(cursor_visibility).is_some_and(|(history, cursor)| !std::ptr::eq(history, cursor)) {
            return Err("envelope cursor and history have different group authorities");
        }
        let decision = history_visibility.or(cursor_visibility).map(crate::os_vcs::ArtifactGroupVisibility::capture);
        let vcs = self.vcs.read_group(decision.as_ref()).map_err(|()| "envelope history lost its captured group authority")?;
        let cursor = self.cursor.as_ref().map(|cursor| cursor.read_group(decision.as_ref())).transpose().map_err(|()| "envelope cursor lost its captured group authority")?;
        Ok(ArtifactEnvelopeRead {
            schema: &self.schema,
            id: &self.id,
            vcs,
            backbone: self.backbone.as_ref(),
            active_alternative_id: self.active_alternative_id.as_ref(),
            cursor,
            dialect: self.dialect.as_ref(),
            migrated_from: self.migrated_from.as_ref(),
            owner: self.owner.as_ref(),
            lanes: &self.lanes,
            viewer_checkpoint_id: self.viewer_checkpoint_id.as_ref(),
            edit_messages: &self.edit_messages,
            conflicts: &self.conflicts,
        })
    }
}

/// 🛡️ Terminal shell for one exact document record. Deep fields are ManuallyDrop from
/// birth and can leave the shell only through the bounded store/completed-record owner protocols.
pub struct ArtifactEnvelope<P, Mutation> {
    owners: std::mem::ManuallyDrop<ArtifactEnvelopeOwners<P, Mutation>>,
    owners_detached: bool,
}

impl<P, Mutation> ArtifactEnvelope<P, Mutation> {
    /// 🧮 Quotes one original document catalog birth without deriving caller authority.
    pub fn document_admission_demands(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        macro_rules! ledger { ($field:ident) => { if !self.owners.vcs.$field.initial_page_is_admitted() { return self.owners.vcs.$field.initial_page_birth_demand(); } }; }
        ledger!(edits); ledger!(changes); ledger!(checkpoints); ledger!(alternatives);
        let cursor = self.owners.cursor.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "document admission retains its original cursor"))?;
        if cursor.applied_edit_ids.capacity() == 0 { return cursor.applied_edit_ids.initial_page_birth_demand(); }
        if cursor.redo_edit_ids.capacity() == 0 { return cursor.redo_edit_ids.initial_page_birth_demand(); }
        self.owners.edit_messages.admission_demands()
    }
    /// 🎟️ Joins every genuine document page before hydrated payload insertion.
    pub fn document_admission_is_complete(&self) -> bool {
        self.owners.vcs.edits.initial_page_is_admitted() && self.owners.vcs.changes.initial_page_is_admitted() && self.owners.vcs.checkpoints.initial_page_is_admitted() && self.owners.vcs.alternatives.initial_page_is_admitted() && self.owners.cursor.as_ref().is_some_and(|cursor| cursor.applied_edit_ids.capacity() != 0 && cursor.redo_edit_ids.capacity() != 0) && self.owners.edit_messages.admission_is_complete()
    }
    /// 📥 Installs only one funded original document backing per turn.
    pub fn admit_document_next(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        macro_rules! ledger { ($field:ident) => { if !self.owners.vcs.$field.initial_page_is_admitted() { return self.owners.vcs.$field.admit_initial_page(grant); } }; }
        ledger!(edits); ledger!(changes); ledger!(checkpoints); ledger!(alternatives);
        let cursor = self.owners.cursor.as_mut().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "document admission retains its original cursor"))?;
        if cursor.applied_edit_ids.capacity() == 0 { return cursor.applied_edit_ids.admit_initial_page(grant); }
        if cursor.redo_edit_ids.capacity() == 0 { return cursor.redo_edit_ids.admit_initial_page(grant); }
        self.owners.edit_messages.admit_next(grant)
    }

    pub fn from_owners(owners: ArtifactEnvelopeOwners<P, Mutation>) -> Self {
        Self { owners: std::mem::ManuallyDrop::new(owners), owners_detached: false }
    }

    /// 🧺️ Consumes the terminal shell and hands back its owners. This is the ONLY way a caller
    /// outside this module can read an envelope's fields: the shell's `Drop` asserts that its owners
    /// were detached first, so reading fields by reference and letting the shell fall out of scope
    /// aborts at runtime ("terminal shell reached Drop before ... detached every nested owner").
    /// `🖥️host`'s `BackboneDocument` construction goes through here for exactly that reason.
    pub fn into_owners(mut self) -> ArtifactEnvelopeOwners<P, Mutation> {
        assert!(!self.owners_detached, "artifact envelope owners were detached twice");
        self.owners_detached = true;
        unsafe { std::mem::ManuallyDrop::take(&mut self.owners) }
    }

    /// 🧹️ Retires a candidate envelope that no store adopted. {@link into_owners} hands back
    /// fixed ledgers (`vcs.edits`/`changes`/`checkpoints`/`alternatives`, `edit_messages`) that each
    /// carry their own terminal-empty `Drop` witness, so dropping a POPULATED candidate aborts the
    /// process on that witness instead of surfacing the refusal. Every entry is popped here, in the
    /// same tail-first order {@link ArtifactStore::close_take_final_envelope_retirement} leaves the
    /// ledgers in, and nothing else about the refusal changes. The initial snapshot and every popped
    /// edit are retired through the technology's own vocabulary, never dropped: a procedural
    /// snapshot's `OrderedMap` layout and its operations' roots abort the guest on a bare drop (hub
    /// genesis of `3d.generation`, 2026-09-28, ticket 26/09/23 slice S19).
    pub fn retire_unadopted(self)
    where
        Mutation: self::Mutation<P>,
    {
        let ArtifactEnvelopeOwners { schema, id, vcs, backbone, active_alternative_id, cursor, dialect, migrated_from, owner, lanes, viewer_checkpoint_id, mut edit_messages, conflicts, transitions, history_shape: _, open_transaction: _ } =
            self.into_owners();
        let ArtifactVcs { genesis, mut edits, mut changes, mut checkpoints, mut alternatives } = vcs;
        while let Some(edit) = edits.pop() {
            retire_scratch_edits::<P, Mutation>([edit]);
        }
        while changes.pop().is_some() {}
        while checkpoints.pop().is_some() {}
        while alternatives.pop().is_some() {}
        while edit_messages.pop().is_some() {}
        drop((schema, id, edits, changes, checkpoints, alternatives, backbone, active_alternative_id, cursor, dialect, migrated_from, owner, lanes, viewer_checkpoint_id, edit_messages, conflicts, transitions));
        let (snapshot, pack) = genesis.into_owners();
        retire_shared_projection::<P, Mutation>(snapshot);
        drop(pack);
    }

    /// 🌱️ Moves the sole snapshot from a decoder-proven fresh envelope. Rejection
    /// returns the exact shell untouched; accepted metadata is definitionally bounded and empty.
    pub fn try_into_fresh_snapshot(self) -> Result<P, Self> {
        if self.schema.len() > 256
            || self.id.len() > 256
            || !self.vcs.edits.is_empty()
            || !self.vcs.changes.is_empty()
            || !self.vcs.checkpoints.is_empty()
            || !self.vcs.alternatives.is_empty()
            || self.backbone.is_some()
            || self.active_alternative_id.is_some()
            || self.cursor.is_some()
            || self.dialect.is_some()
            || self.migrated_from.is_some()
            || self.owner.is_some()
            || !self.lanes.is_empty()
            || self.viewer_checkpoint_id.is_some()
            || !self.edit_messages.is_empty()
            || !self.conflicts.is_empty()
            || !self.transitions.is_empty()
        {
            return Err(self);
        }
        let ArtifactEnvelopeOwners { schema, id, vcs, backbone, active_alternative_id, cursor, dialect, migrated_from, owner, lanes, viewer_checkpoint_id, edit_messages, conflicts, transitions, history_shape: _, open_transaction: _ } =
            self.into_owners();
        let ArtifactVcs { genesis, edits, changes, checkpoints, alternatives } = vcs;
        drop((schema, id, edits, changes, checkpoints, alternatives, backbone, active_alternative_id, cursor, dialect, migrated_from, owner, lanes, viewer_checkpoint_id, edit_messages, conflicts, transitions));
        let (snapshot, pack) = genesis.into_owners();
        drop(pack);
        Ok(Arc::try_unwrap(snapshot).unwrap_or_else(|_| panic!("fresh snapshot cannot have an external alias")))
    }
}

impl<P, Mutation> std::ops::Deref for ArtifactEnvelope<P, Mutation> {
    type Target = ArtifactEnvelopeOwners<P, Mutation>;

    fn deref(&self) -> &Self::Target {
        &self.owners
    }
}

impl<P, Mutation> std::ops::DerefMut for ArtifactEnvelope<P, Mutation> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.owners
    }
}

impl<P: std::fmt::Debug, Mutation: std::fmt::Debug> std::fmt::Debug for ArtifactEnvelope<P, Mutation> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.owners.fmt(formatter)
    }
}

impl<P: PartialEq, Mutation: PartialEq> PartialEq for ArtifactEnvelope<P, Mutation> {
    fn eq(&self, other: &Self) -> bool {
        self.owners.eq(&other.owners)
    }
}

impl<P, Mutation> Drop for ArtifactEnvelope<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.owners_detached), "artifact envelope terminal shell reached Drop before its app-owned bounded retirement authority detached every nested owner");
    }
}

/// 🧬️ Provenance stamp for an envelope produced by `migrate_document`: which prior document,
/// which dialect it was in, and (if the migration ran at a specific checkpoint rather than the
/// live tip) which checkpoint. `migrated_at` follows this crate's existing `Checkpoint.timestamp`/
/// `Edit.started_at` convention (a caller-supplied string stamped via `now_iso()`, see below) — no
/// new clock abstraction introduced for this one field.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct MigrationProvenance {
    pub document_id: String,
    pub dialect: semio_framework_artifact_reference::ArtifactDialect,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_id: Option<String>,
    pub migrated_at: String,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ArtifactCommand<Mutation> {
    /// ▶️ Records `mutations` as one edit; `transaction` names the committed tool transaction that authored them, stamped
    /// on every operation's metadata and wire envelope.
    Apply {
        mutations: Vec<Mutation>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        transaction: Option<protocol::TransactionRef>,
    },
    Undo,
    Redo,
    UndoWithPolicy {
        policy: UndoPolicy,
        #[value(default, skip_serializing_if = "Option::is_none")]
        semantic_command: Option<Box<ArtifactCommand<Mutation>>>,
    },
    /// 🛤️ `Apply`'s lane-tagged twin — records the resulting edit under `lane` (via
    /// `ArtifactEnvelope.lanes`) instead of the implicit `HistoryLane::Document` a plain `Apply`
    /// gets.
    ApplyInLane {
        mutations: Vec<Mutation>,
        #[value(default)]
        lane: HistoryLane,
        #[value(default, skip_serializing_if = "Option::is_none")]
        transaction: Option<protocol::TransactionRef>,
    },
    /// 🛤️ Explicit lane-scoped undo: mirrors plain `Undo`'s selective (own-author) semantics
    /// but searches `applied_edit_ids` for the nearest local entry whose `HistoryLane` is exactly
    /// `lane`, instead of `HistoryLane::Document`. Lets a caller walk a non-`Document` lane on
    /// purpose — the completing half of "default `Undo`/`Redo` skip non-`Document` lanes" (see
    /// `ArtifactStore::dispatch`'s `Undo` arm).
    UndoInLane {
        lane: HistoryLane,
    },
    /// 🛤️ `UndoInLane`'s redo-direction sibling — mirrors plain `Redo`, but pops the
    /// nearest-to-top `redo_edit_ids` entry whose `HistoryLane` is exactly `lane`.
    RedoInLane {
        lane: HistoryLane,
    },
    CommitCheckpoint {
        #[value(skip_serializing_if = "Option::is_none")]
        message: Option<String>,
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        authors: Vec<Author>,
    },
    CreateAlternative {
        name: String,
    },
    SwitchAlternative {
        alternative_id: String,
    },
    CheckoutCheckpoint {
        #[value(rename = "checkpointId")]
        checkpoint_id: String,
    },
    /// 🕸️ Feeds a remote MutationEnvelope through the causal DAG into the edit timeline.
    IngestRemote {
        #[value(serialize_with = "operation_envelope_serde::to_value", deserialize_with = "operation_envelope_serde::from_value")]
        envelope: crate::os_spr::MutationEnvelope,
    },
    /// 🧹 Clears volatile draft-lane history that must never enter a Change/Checkpoint.
    PruneDrafts,
    /// ⚖️ Sets this store's local `crate::os_spr::MergePolicy` — authority-local state, never
    /// carried on a `crate::os_spr::MutationEnvelope`/`BackboneMessage`, never part of shared
    /// history (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6). Ordinal
    /// 15 — frozen, appended after `PruneDrafts`.
    SetMergePolicy {
        policy: crate::os_spr::MergePolicy,
    },
    /// ⚔️ Resolves an `Open` `crate::os_spr::Conflict` by id — see
    /// `ArtifactStore::resolve_conflict` for what `Accept`/`Discard` do for each conflict kind.
    /// Ordinal 16 — frozen.
    ResolveConflict {
        conflict_id: String,
        resolution: crate::os_spr::ConflictResolution,
    },
    /// ✏️ Supersedes the effective input of applied operations, non-destructively: each input replaces its target's
    /// input (`None` withdraws it) in every alternative, or only within the alternative `scope`. Refused atomically
    /// with `Rejected { policy: Normal }` when the Report replay of everything downstream raises an `Error` or `Fatal`.
    /// Ordinal 17.
    Supersede {
        #[value(default, skip_serializing_if = "Option::is_none")]
        scope: Option<String>,
        inputs: Vec<SupersedeInput<Mutation>>,
    },
    /// 🌿️ Commits the pending edits, branches the alternative `name` at the head and supersedes `inputs` within it —
    /// one atomic dispatch leaving in one outbound batch. Ordinal 18.
    CreateAlternativeWithSupersede {
        name: String,
        inputs: Vec<SupersedeInput<Mutation>>,
    },
    /// 🎞️ Appends `mutations` to the open edit of the tool transaction `transaction`, opening it with its first append:
    /// a streamed tool's ticks become one edit, every operation stamped with the ref and none announced before
    /// `CommitTransaction`. While it is open the store refuses every command but its appends, its commit or abort, a remote
    /// ingest and a merge-policy change (`VcsError::TransactionOpen`). Ordinal 19.
    AppendTransaction {
        mutations: Vec<Mutation>,
        transaction: protocol::TransactionRef,
    },
    /// ✅️ Closes the open edit of the tool transaction `transaction_id` and announces every operation it holds. Ordinal 20.
    CommitTransaction {
        transaction_id: String,
    },
    /// 🧨️ Reverts the open edit of the tool transaction `transaction_id` with zero trace: no edit, no operation, no
    /// announcement, the projection and revision of before its first append. Ordinal 21.
    AbortTransaction {
        transaction_id: String,
    },
}

/// 🎞️ The tool transaction a store holds open ([`ArtifactCommand::AppendTransaction`]): its ref, the one edit its
/// appends grow — kept at the tail of the local applied history until its commit announces it or its abort removes it.
#[derive(Clone, Debug, PartialEq)]
pub struct OpenToolTransaction {
    pub transaction: protocol::TransactionRef,
    pub edit_id: String,
}

/// 🐢️ A history change this store admitted but has not adopted yet (design §16.6): the remote `transitions` it admitted
/// ([`ArtifactStore::defer_remote_replays`]) and at most one `local` step this replica authored
/// ([`ArtifactStore::defer_local_replays`]) wait outside the log while their Report replay steps turn by turn; the replay
/// restarts whenever the history it started from changed (the content revision it names).
pub(crate) struct PendingReprojection<P, Mutation>
where
    Mutation: self::Mutation<P>,
{
    pub(crate) transitions: Vec<crate::os_spr::MutationEnvelope>,
    pub(crate) local: Option<PendingLocalStep>,
    pub(crate) replay: Option<([u8; 32], EditReplay<P, Mutation>)>,
}

impl<P, Mutation> PendingReprojection<P, Mutation>
where
    Mutation: self::Mutation<P>,
{
    /// 🫗️ Nothing admitted yet.
    pub(crate) fn empty() -> Self {
        Self { transitions: Vec::new(), local: None, replay: None }
    }
}

/// 🪃️ A local history step whose Report replay outlasts its dispatch: the transitions it authored (neither recorded nor
/// announced yet), the head `(line, checkpoint)` it moves this replica to, and whether it finalizes a supersession — whose
/// report must not block finalizing, or the step is refused with nothing of it recorded.
pub(crate) struct PendingLocalStep {
    pub(crate) envelopes: Vec<crate::os_spr::MutationEnvelope>,
    pub(crate) head: (Option<String>, Option<String>),
    pub(crate) finalizing: bool,
}

/// ⚡️ A local undo or redo that changes nothing but the applied tail ([`ArtifactStore::tail_step`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TailStep {
    Undo,
    Redo,
}

/// ⏩️ Where one advance of a deferred reprojection left it.
pub(crate) enum ReprojectionAdvance {
    Pending(ReplayProgress),
    Adopted(Vec<crate::os_spr::EditMessages>),
}

/// ⏱️ How far one turn of a deferred reprojection replays (design §7: ≤ 4 ms slices): until the turn's wall deadline on
/// `now_us` — `wall_us` after the turn began, or the runtime's own turn deadline ([`ArtifactStore::step_reprojection`]) —
/// and at most `operations` operations, whichever comes first; every turn replays at least one operation. `now_us` is the
/// job clock ([`semio_framework_job::default_now_us`]); laws inject their own.
#[derive(Clone, Copy, Debug)]
pub struct ReplayTurnBudget {
    pub wall_us: u64,
    pub operations: usize,
    pub now_us: fn() -> Option<u64>,
}

impl ReplayTurnBudget {
    /// 🕓️ `wall_us` of the job clock per turn, with `operations` as the secondary cap.
    pub fn wall(wall_us: u64, operations: usize) -> Self {
        Self { wall_us, operations, now_us: semio_framework_job::default_now_us }
    }

    /// 🔢️ `operations` per turn and no wall limit: what a law that counts operations replays with.
    pub fn operations(operations: usize) -> Self {
        Self { wall_us: u64::MAX, operations, now_us: semio_framework_job::default_now_us }
    }

    /// ⌛️ The turn's deadline on `now_us`: the runtime's `deadline_us`, else `wall_us` from now (now when the clock cannot
    /// be read); `None` without a wall limit.
    pub(crate) fn turn_deadline(&self, deadline_us: Option<u64>) -> Option<u64> {
        deadline_us.or_else(|| (self.wall_us != u64::MAX).then(|| (self.now_us)().map_or(0, |now| now.saturating_add(self.wall_us))))
    }

    /// 🛎️ Whether a turn that replayed `replayed` operations ends: its cap is reached or its deadline passed on `now_us`
    /// (an unreadable clock ends it).
    pub(crate) fn turn_ends(&self, replayed: usize, deadline_us: Option<u64>) -> bool {
        replayed >= self.operations || deadline_us.is_some_and(|deadline| (self.now_us)().is_none_or(|now| now >= deadline))
    }
}

/// ✏️ One input of [`ArtifactCommand::Supersede`]: the superseded operation and the replacement it folds as from now
/// on (`None` withdraws it).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SupersedeInput<Mutation> {
    pub target: MutationId,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub replacement: Option<Mutation>,
}

impl<Mutation> ArtifactCommand<Mutation> {
    /// 🔮️ Names the projection-invalidating transition this command performs, if it has one.
    pub async fn projection_cause(&self) -> Option<ArtifactProjectionCause> {
        match self {
            Self::Apply { .. } | Self::ApplyInLane { .. } => Some(ArtifactProjectionCause::Apply),
            Self::IngestRemote { .. } => Some(ArtifactProjectionCause::RemoteIngest),
            Self::Undo | Self::UndoWithPolicy { .. } | Self::UndoInLane { .. } => Some(ArtifactProjectionCause::Undo),
            Self::Redo | Self::RedoInLane { .. } => Some(ArtifactProjectionCause::Redo),
            Self::CheckoutCheckpoint { .. } | Self::CreateAlternative { .. } | Self::SwitchAlternative { .. } => Some(ArtifactProjectionCause::Checkout),
            Self::CommitCheckpoint { .. } => Some(ArtifactProjectionCause::Checkpoint),
            Self::PruneDrafts => Some(ArtifactProjectionCause::PruneDrafts),
            Self::SetMergePolicy { .. } => Some(ArtifactProjectionCause::PolicyChange),
            Self::ResolveConflict { .. } => Some(ArtifactProjectionCause::RemoteIngest),
            Self::Supersede { .. } | Self::CreateAlternativeWithSupersede { .. } => Some(ArtifactProjectionCause::Replay),
            Self::AppendTransaction { .. } | Self::CommitTransaction { .. } => Some(ArtifactProjectionCause::Apply),
            Self::AbortTransaction { .. } => Some(ArtifactProjectionCause::Undo),
        }
    }

    /// 🚦️ Whether this command moves history positions — an undo, redo, checkpoint, alternative, checkout, supersession or
    /// conflict resolution: what a store whose local history step still replays refuses ([`VcsError::HistoryReplaying`]).
    pub fn moves_history(&self) -> bool {
        !self.history_transition_kinds().is_empty() || matches!(self, Self::ResolveConflict { .. })
    }

    /// 🏷️ Every kind of history transition this command may author, in authoring order: what the store's
    /// [`crate::os_spr::HistoryShape`] must admit before the command runs.
    pub fn history_transition_kinds(&self) -> &'static [crate::os_spr::HistoryTransitionKind] {
        use crate::os_spr::HistoryTransitionKind::{Branch, Checkout, Commit, Reinstate, Revert, Supersede};
        match self {
            Self::Undo | Self::UndoWithPolicy { .. } | Self::UndoInLane { .. } => &[Revert],
            Self::Redo | Self::RedoInLane { .. } => &[Reinstate],
            Self::CommitCheckpoint { .. } => &[Commit],
            Self::CreateAlternative { .. } => &[Commit, Branch],
            Self::SwitchAlternative { .. } | Self::CheckoutCheckpoint { .. } => &[Checkout],
            Self::Supersede { .. } => &[Supersede],
            Self::CreateAlternativeWithSupersede { .. } => &[Commit, Branch, Supersede],
            Self::Apply { .. }
            | Self::ApplyInLane { .. }
            | Self::IngestRemote { .. }
            | Self::PruneDrafts
            | Self::SetMergePolicy { .. }
            | Self::ResolveConflict { .. }
            | Self::AppendTransaction { .. }
            | Self::CommitTransaction { .. }
            | Self::AbortTransaction { .. } => &[],
        }
    }
}

/// 🧬️ History identity for a projection input, independent of the process-local generation.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArtifactRevision {
    pub artifact_id: String,
    pub schema: String,
    pub applied_edit_ids: crate::os_vcs::HistoryPageStack<String>,
    pub redo_edit_ids: crate::os_vcs::HistoryPageStack<String>,
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

/// 🎯️ B-R6 "one documented generic impl": `ArtifactCommand<Op>` cannot go through
/// `#[derive(crate::os_dsl::DslOps)]` like every concrete per-technology `Mutation` enum does — it is generic
/// over a FOREIGN `Op: OpBinary` from whichever technology is dispatching, and the derive
/// only lowers a CONCRETE type's own fields to a `RecordSpec`; there is no way to describe "some
/// other crate's already-`OpBinary` type" as a `crate::os_dsl::DslField` shape. This hand-written impl is the
/// one place `ArtifactCommand`'s binary layout is still spelled out by hand — everywhere else in the
/// workspace, encoding is derive-generated. Byte layout matches the convention exactly: `format u8
/// (=1, see `COMMAND_BINARY_FORMAT`) | fixed variant ordinal varint (one stable number per variant; LEB128 varints
/// under 128 are single bytes) | body`. The binary twin of `print_command`/`parse_command`. LAW:
/// `ArtifactCommand::decode_command(&command.encode_command()?) == Ok(command)`. Not an `OpBinary`: decoding needs the
/// operations' projection `P` to retire what a refused command decoded, which the P-agnostic `OpBinary` cannot name.
impl<Op: OpBinary> ArtifactCommand<Op> {
    /// 🛫️ The command's binary form.
    pub fn encode_command(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        let mut out = vec![COMMAND_BINARY_FORMAT];
        match self {
            ArtifactCommand::Apply { mutations, transaction } => {
                crate::os_pack::write_varint_u64(&mut out, 0);
                out.push(u8::from(transaction.is_some()) << 1);
                write_command_transaction(&mut out, transaction.as_ref());
                write_command_ops(&mut out, mutations)?;
            }
            ArtifactCommand::Undo => crate::os_pack::write_varint_u64(&mut out, 1),
            ArtifactCommand::Redo => crate::os_pack::write_varint_u64(&mut out, 2),
            ArtifactCommand::UndoWithPolicy { policy, semantic_command } => {
                crate::os_pack::write_varint_u64(&mut out, 3);
                out.push(undo_policy_ordinal(*policy));
                out.push(if semantic_command.is_some() { 0b01 } else { 0 });
                if let Some(nested) = semantic_command {
                    let nested_bytes = nested.encode_command()?;
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
            ArtifactCommand::IngestRemote { envelope } => {
                crate::os_pack::write_varint_u64(&mut out, 9);
                let mut bytes = Vec::new();
                crate::os_spr::encode_envelope(envelope, &mut bytes);
                crate::os_pack::write_varint_u64(&mut out, bytes.len() as u64);
                out.extend_from_slice(&bytes);
            }
            ArtifactCommand::PruneDrafts => crate::os_pack::write_varint_u64(&mut out, 10),
            ArtifactCommand::ApplyInLane { mutations, lane, transaction } => {
                crate::os_pack::write_varint_u64(&mut out, 11);
                out.push(u8::from(transaction.is_some()) << 1);
                write_command_transaction(&mut out, transaction.as_ref());
                out.push(history_lane_ordinal(*lane));
                write_command_ops(&mut out, mutations)?;
            }
            ArtifactCommand::UndoInLane { lane } => {
                crate::os_pack::write_varint_u64(&mut out, 13);
                out.push(history_lane_ordinal(*lane));
            }
            ArtifactCommand::RedoInLane { lane } => {
                crate::os_pack::write_varint_u64(&mut out, 14);
                out.push(history_lane_ordinal(*lane));
            }
            ArtifactCommand::SetMergePolicy { policy } => {
                crate::os_pack::write_varint_u64(&mut out, 15);
                out.push(policy.as_u8());
            }
            ArtifactCommand::ResolveConflict { conflict_id, resolution } => {
                crate::os_pack::write_varint_u64(&mut out, 16);
                write_command_str(&mut out, conflict_id);
                out.push(conflict_resolution_ordinal(*resolution));
            }
            ArtifactCommand::Supersede { scope, inputs } => {
                crate::os_pack::write_varint_u64(&mut out, 17);
                out.push(if scope.is_some() { 0b01 } else { 0 });
                if let Some(scope) = scope {
                    write_command_str(&mut out, scope);
                }
                write_supersede_inputs(&mut out, inputs)?;
            }
            ArtifactCommand::CreateAlternativeWithSupersede { name, inputs } => {
                crate::os_pack::write_varint_u64(&mut out, 18);
                write_command_str(&mut out, name);
                write_supersede_inputs(&mut out, inputs)?;
            }
            ArtifactCommand::AppendTransaction { mutations, transaction } => {
                crate::os_pack::write_varint_u64(&mut out, 19);
                write_command_str(&mut out, &transaction.id);
                write_command_str(&mut out, &transaction.tool);
                write_command_ops(&mut out, mutations)?;
            }
            ArtifactCommand::CommitTransaction { transaction_id } => {
                crate::os_pack::write_varint_u64(&mut out, 20);
                write_command_str(&mut out, transaction_id);
            }
            ArtifactCommand::AbortTransaction { transaction_id } => {
                crate::os_pack::write_varint_u64(&mut out, 21);
                write_command_str(&mut out, transaction_id);
            }
        }
        Ok(out)
    }

    /// 🛬️ Decodes one whole binary command, all-or-nothing: bytes past its end refuse it, and every operation decoded
    /// before a failure retires through its technology's cold disposal — a refusal never drops a live operation.
    pub fn decode_command<P>(bytes: &[u8]) -> Result<Self, CommandDecodeError>
    where
        Op: Mutation<P>,
    {
        let mut reader = crate::os_pack::ByteReader::new(bytes);
        let command = Self::read_command::<P>(&mut reader)?;
        if reader.remaining() != 0 {
            let detail = format!("{} bytes past the command's end", reader.remaining());
            retire_command::<P, Op>(command);
            return Err(CommandDecodeError::Layout(crate::os_spr::ProtocolError::Malformed { what: "command", offset: reader.position() as u64, detail }));
        }
        Ok(command)
    }

    /// 📖️ The command at `reader`; its operation lists come last, so nothing can fail after one was decoded.
    fn read_command<P>(reader: &mut crate::os_pack::ByteReader<'_>) -> Result<Self, CommandDecodeError>
    where
        Op: Mutation<P>,
    {
        let format = reader.read_u8()?;
        if format != COMMAND_BINARY_FORMAT {
            return Err(crate::os_spr::ProtocolError::Malformed { what: "command format", offset: 0, detail: format!("unsupported command format {format}") }.into());
        }
        let ordinal = reader.read_varint_u64()?;
        match ordinal {
            0 => {
                let presence = reader.read_u8()?;
                let transaction = read_command_transaction(reader, presence)?;
                let mutations = read_command_ops::<P, Op>(reader)?;
                Ok(ArtifactCommand::Apply { mutations, transaction })
            }
            1 => Ok(ArtifactCommand::Undo),
            2 => Ok(ArtifactCommand::Redo),
            3 => {
                let policy = undo_policy_from_ordinal(reader.read_u8()?)?;
                let presence = reader.read_u8()?;
                let semantic_command = if presence & 0b01 != 0 {
                    let len = reader.read_varint_u64()?;
                    let nested_bytes = reader.read_bytes(len as usize)?;
                    Some(Box::new(ArtifactCommand::<Op>::decode_command::<P>(nested_bytes)?))
                } else {
                    None
                };
                Ok(ArtifactCommand::UndoWithPolicy { policy, semantic_command })
            }
            4 => {
                let presence = reader.read_u8()?;
                let message = if presence & 0b01 != 0 { Some(read_command_str(reader)?) } else { None };
                let author_count = reader.read_varint_u64()?;
                let mut authors = Vec::with_capacity(author_count as usize);
                for _ in 0..author_count {
                    let id = read_command_str(reader)?;
                    let name = read_command_str(reader)?;
                    authors.push(Author { id, name, avatar: None });
                }
                Ok(ArtifactCommand::CommitCheckpoint { message, authors })
            }
            5 => Ok(ArtifactCommand::CreateAlternative { name: read_command_str(reader)? }),
            6 => Ok(ArtifactCommand::SwitchAlternative { alternative_id: read_command_str(reader)? }),
            7 => Ok(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: read_command_str(reader)? }),
            9 => {
                let len = reader.read_varint_u64()?;
                let bytes = reader.read_bytes(len as usize)?;
                let mut pos = 0;
                let envelope = crate::os_spr::decode_envelope(bytes, &mut pos).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "ingest envelope", offset: 0, detail: error.to_string() })?;
                Ok(ArtifactCommand::IngestRemote { envelope })
            }
            10 => Ok(ArtifactCommand::PruneDrafts),
            11 => {
                let presence = reader.read_u8()?;
                let transaction = read_command_transaction(reader, presence)?;
                let lane = history_lane_from_ordinal(reader.read_u8()?)?;
                let mutations = read_command_ops::<P, Op>(reader)?;
                Ok(ArtifactCommand::ApplyInLane { mutations, lane, transaction })
            }
            13 => Ok(ArtifactCommand::UndoInLane { lane: history_lane_from_ordinal(reader.read_u8()?)? }),
            14 => Ok(ArtifactCommand::RedoInLane { lane: history_lane_from_ordinal(reader.read_u8()?)? }),
            15 => {
                let policy = crate::os_spr::MergePolicy::from_u8(reader.read_u8()?).ok_or_else(|| crate::os_spr::ProtocolError::Malformed { what: "merge policy ordinal", offset: 1, detail: "unknown merge policy ordinal".to_string() })?;
                Ok(ArtifactCommand::SetMergePolicy { policy })
            }
            16 => {
                let conflict_id = read_command_str(reader)?;
                let resolution = conflict_resolution_from_ordinal(reader.read_u8()?)?;
                Ok(ArtifactCommand::ResolveConflict { conflict_id, resolution })
            }
            17 => {
                let presence = reader.read_u8()?;
                let scope = if presence & 0b01 != 0 { Some(read_command_str(reader)?) } else { None };
                Ok(ArtifactCommand::Supersede { scope, inputs: read_supersede_inputs::<P, Op>(reader)? })
            }
            18 => {
                let name = read_command_str(reader)?;
                Ok(ArtifactCommand::CreateAlternativeWithSupersede { name, inputs: read_supersede_inputs::<P, Op>(reader)? })
            }
            19 => {
                let transaction = protocol::TransactionRef { id: read_command_str(reader)?, tool: read_command_str(reader)? };
                Ok(ArtifactCommand::AppendTransaction { mutations: read_command_ops::<P, Op>(reader)?, transaction })
            }
            20 => Ok(ArtifactCommand::CommitTransaction { transaction_id: read_command_str(reader)? }),
            21 => Ok(ArtifactCommand::AbortTransaction { transaction_id: read_command_str(reader)? }),
            other => Err(crate::os_spr::ProtocolError::Malformed { what: "command variant", offset: 1, detail: format!("unknown command ordinal {other}") }.into()),
        }
    }
}
