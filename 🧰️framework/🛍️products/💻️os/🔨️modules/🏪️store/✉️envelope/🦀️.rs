//! ✉ Envelope records, captured group reads and terminal ownership independent of Store execution.
use semio_framework_os_vcs::{ArtifactVcs, io::binary::genesis::AdmittedArtifactGenesis};
use protocol::Mutation;
use semio_framework_value::{DslValue, RetainedCloneGrant, RetainedCloneProgress, ToValue, ValueError};
use std::sync::Arc;
use semio_framework_os_store_cursor::{ArtifactCursor, ArtifactCursorOwners};
use semio_framework_os_store_command::{HistoryLane, OpenToolTransaction, retire_scratch_edits};
use semio_framework_os_store_projection::retire_shared_projection;
use semio_framework_os_store_message_ledger::ArtifactEditMessageLedger;
#[path="../🪆️child/🏠️owner/🧬️schema/🦀️.rs"]
pub mod owner;
use owner::OwnerRef;

/// 🔗️ Identifies the channel a document synchronizes through, when one is attached.
/// 🌱️ `Serialize`/`Deserialize` dropped outright (not even `cfg_attr(test, …)`): no production
/// consumer (`🖥️host/🔌️plugin-host/🦀️.rs`'s `BackboneDocument`, `🔌️plugin/🦀️.rs`'s `backbone_ref()`) requires
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
    pub conflicts: Vec<protocol::Conflict>,
    /// 🔀️ Every structural history event — undo, redo, checkpoint commit, branch, checkout,
    /// repin — as one causal `protocol::HISTORY_TRANSITION_SCHEMA` envelope, in HLC order.
    /// Together with `vcs.edits` this is the document's complete semantic event log. The
    /// change/checkpoint/alternative ledgers are its projection. `active_alternative_id` and
    /// `viewer_checkpoint_id` are this replica's head, persisted local-only, not that projection.
    pub transitions: Vec<protocol::MutationEnvelope>,
    /// 🗂️ The transitions this history holds ([`protocol::HistoryShape`]): a config store's history holds undo and
    /// redo only, so its store refuses every other transition at dispatch and at remote admission — the same law its
    /// retained loader applies.
    pub history_shape: protocol::HistoryShape,
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
    vcs: semio_framework_os_vcs::ArtifactVcsRead<'a, AdmittedArtifactGenesis<P>, Mutation>,
    backbone: Option<&'a ArtifactBackboneRef>,
    active_alternative_id: Option<&'a String>,
    cursor: Option<&'a ArtifactCursorOwners>,
    dialect: Option<&'a semio_framework_artifact_reference::ArtifactDialect>,
    migrated_from: Option<&'a MigrationProvenance>,
    owner: Option<&'a OwnerRef>,
    lanes: &'a protocol::HistoryFoldIndex<String, HistoryLane>,
    viewer_checkpoint_id: Option<&'a String>,
    edit_messages: &'a ArtifactEditMessageLedger,
    conflicts: &'a Vec<protocol::Conflict>,
}

/// 🌉️ Hand-written, not derived: every field but `vcs` is a plain reference or `Option<&T>`, and
/// no blanket `impl<T: ToValue> ToValue for &T` exists in this codebase (see `ArtifactVcsRead`'s
/// own docstring) — mirrors the former `#[derive(Serialize)]`'s `skip_serializing_if` shape by
/// simply omitting the key when the option is `None`/the map is empty.
impl<P: ToValue, Mutation: ToValue> ToValue for ArtifactEnvelopeRead<'_, P, Mutation> {
    fn to_value(&self) -> DslValue {
        let mut entries = vec![("schema".to_string(), self.schema.to_value()), ("id".to_string(), self.id.to_value()), ("vcs".to_string(), semio_framework_os_vcs::io::text::vcs::native_vcs_read_value(&self.vcs))];
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
        let decision = history_visibility.or(cursor_visibility).map(semio_framework_os_vcs::ArtifactGroupVisibility::capture);
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




semio_framework_value::artifact_retire_struct!(ArtifactBackboneRef { uri });
semio_framework_value::artifact_retire_struct!(MigrationProvenance { document_id, dialect, checkpoint_id, migrated_at });
