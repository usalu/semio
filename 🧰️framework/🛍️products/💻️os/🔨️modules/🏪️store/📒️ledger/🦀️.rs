//! 📒️ The edit ledger beneath the Store engine: the paged edit-message ledger and its retirement, the cursor revision
//! accumulator with its records and digest chains, the history commit reservation and pending command report, and the
//! envelope fold and projection helpers every history, replay, codec and composition facet reads.
//! It names the vocabulary below it (schemas, snapshot-read retirement, effective forwards) and never the
//! `ArtifactStore` engine, so the engine and every facet above it depend on it in one direction.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
#[path = "📥️lifted/🏪️store/🦀️.rs"]
pub mod lifted_store;
#[path = "📥️lifted/♻️retirement/🦀️.rs"]
pub mod lifted_retirement;
#[cfg(any(test, feature = "hot-path-census"))]
#[path = "🧮️hot-path-census/🦀️.rs"]
pub mod hot_path_census;
#[path = "♻️retirement/🦀️.rs"]
pub mod edit_retirement;
#[path = "../📨️messages/✂️clamp/🦀️.rs"]
pub mod edit_message_clamp;
#[path = "📥️lifted/📝️text/🦀️.rs"]
pub mod lifted_text_format;
#[path = "📥️lifted/🔁️replay/🦀️.rs"]
pub mod lifted_replay;
#[path = "📥️lifted/🔗️read/🦀️.rs"]
pub mod lifted_snapshot_read;
use semio_framework_value::ErasedSnapshotRetirement;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_os_spr::command::ArtifactMessageSource;
use semio_framework_value::DslValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_os_spr::{ArtifactId, HybridLogicalTimestamp, MutationId, SchemaId, UndoPolicy};
use semio_framework_os_spr::{Edit, Mutation, MutationDiff, MutationMeta, OpBinary};
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use semio_framework_os_spr::command::MutationMessageLedgerRetirement as ArtifactStoreMessageLedgerRetirement;
use semio_framework_os_spr::conflict::ProtocolConflictRetirement as ArtifactStoreConflictRetirement;
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
use semio_framework_os_vcs::VcsError;
use semio_framework_os_vcs::{ArtifactHistoryKey, ArtifactHistoryLedger, ArtifactHistoryReservation, Identified};
use semio_framework_os_pack::pack_rt;
use semio_framework_os_store_envelope::{ArtifactEnvelope, ArtifactEnvelopeOwners};
use crate::lifted_snapshot_read::{ArtifactStoreDisplacedOwnerReservation, ArtifactStoreStringRetirement};
use crate::lifted_replay::{EffectiveSupersessions, FoldProjection, edit_supersession_digest};
use semio_framework_os_store_command::SupersedeInput;
use semio_framework_value_snapshot_read::{artifact_retirement_box_close_step, artifact_retirement_box_demands};
use semio_framework_value_retirement_owner::{retirement_owner_close, retirement_owner_demands};

#[derive(Clone)]
pub struct CursorRevisionRecord {
    pub(crate) ledger_key: Option<ArtifactHistoryKey>,
    pub(crate) id_digest: [u8; 32],
    pub(crate) edit_digest: [u8; 32],
    pub(crate) prefix_digest: [u8; 32],
}

#[derive(Clone)]
pub struct CursorRevisionAccumulator {
    pub(crate) identity_digest: [u8; 32],
    pub(crate) applied: semio_framework_os_vcs::HistoryPageStack<CursorRevisionRecord>,
    pub(crate) redo: semio_framework_os_vcs::HistoryPageStack<CursorRevisionRecord>,
    pub(crate) applied_tail_chains: Option<([u8; 32], EditDigestChains)>,
    pub(crate) mutation_positions: protocol::HistoryFoldIndex<MutationId, (usize, usize, [u8; 32])>,
    pub(crate) indexed_edits: protocol::HistoryFoldIndex<[u8; 32], (usize, usize)>,
    pub(crate) unit_flags: protocol::HistoryFoldIndex<([u8; 32], usize), bool>,
}

/// 📍️ The first applied and redo positions whose ids changed since [`ArtifactStore::bump`] last mirrored both stacks into the
/// persisted cursor and the revision records — what keeps both mirrors O(change) per mutation. `usize::MAX` is clean.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StackMirrorDirt {
    pub(crate) applied: usize,
    pub(crate) redo: usize,
}

impl StackMirrorDirt {
    pub(crate) const CLEAN: Self = Self { applied: usize::MAX, redo: usize::MAX };
    pub(crate) const ALL: Self = Self { applied: 0, redo: 0 };
}

/// 🔗️ Running per-list digests of a coalesced edit's operations: what lets an amend extend its edit's revision identity by
/// the operations it appended instead of re-encoding the whole run (a typing run amends one edit per key).
#[derive(Clone, Copy, Default)]
pub struct EditDigestChains {
    pub(crate) forwards: usize,
    pub(crate) inverse: usize,
    pub(crate) meta: usize,
    pub(crate) forwards_digest: [u8; 32],
    pub(crate) inverse_digest: [u8; 32],
    pub(crate) meta_digest: [u8; 32],
}

impl EditDigestChains {
    /// 🔗️ The chains over every operation of `edit`, continued from these; `None` when `edit` is not an extension of the lists
    /// these were taken over (a list shrank).
    fn extended<Mutation: ToValue>(mut self, edit: &Edit<Mutation>) -> Option<Self> {
        if self.forwards > edit.forwards.len() || self.inverse > edit.inverse.len() || self.meta > edit.mutation_meta.len() {
            return None;
        }
        for operation in &edit.forwards[self.forwards..] {
            self.forwards_digest = CursorRevisionAccumulator::hash_record(b"edit-forward", &[&self.forwards_digest, semio_framework_pack_json::to_json_string(operation).as_bytes()]);
        }
        for operation in edit.inverse.iter().skip(self.inverse) {
            self.inverse_digest = CursorRevisionAccumulator::hash_record(b"edit-inverse", &[&self.inverse_digest, semio_framework_pack_json::to_json_string(operation).as_bytes()]);
        }
        for meta in &edit.mutation_meta[self.meta..] {
            self.meta_digest = CursorRevisionAccumulator::hash_record(b"edit-meta", &[&self.meta_digest, semio_framework_pack_json::to_json_string(meta).as_bytes()]);
        }
        self.forwards = edit.forwards.len();
        self.inverse = edit.inverse.len();
        self.meta = edit.mutation_meta.len();
        Some(self)
    }
}

impl CursorRevisionAccumulator {
    pub(crate) fn hash_record(domain: &[u8], parts: &[&[u8]]) -> [u8; 32] {
        let mut digest = semio_framework_hash::Sha256::new();
        digest.update(b"semio.artifact.cursor.v2");
        digest.update(&(domain.len() as u64).to_be_bytes());
        digest.update(domain);
        for part in parts {
            digest.update(&(part.len() as u64).to_be_bytes());
            digest.update(part);
        }
        digest.finalize()
    }

    pub(crate) fn new<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, initial_digest: [u8; 32]) -> Self {
        let identity_digest = Self::hash_record(b"initial", &[envelope.id.as_bytes(), envelope.schema.as_bytes(), &initial_digest]);
        Self { identity_digest, applied: semio_framework_os_vcs::HistoryPageStack::new(), redo: semio_framework_os_vcs::HistoryPageStack::new(), applied_tail_chains: None, mutation_positions: protocol::HistoryFoldIndex::new(), indexed_edits: protocol::HistoryFoldIndex::new(), unit_flags: protocol::HistoryFoldIndex::new() }
    }

    /// 🥽️ What an edit's revision identity covers: its value without `sequenceNumber`. That member is where the edit stands
    /// in ONE replica's ledger — a remote merge renumbers it, a reload derives it from the stored order — so two replicas
    /// holding the same edit at different positions, and a replica and its own reload, name the same edit (design §22.28).
    pub(crate) fn revision_value<Mutation: ToValue>(edit: &Edit<Mutation>) -> DslValue {
        match edit.to_value() {
            DslValue::Object(mut members) => {
                members.retain(|(key, _)| key != "sequenceNumber");
                DslValue::Object(members)
            }
            value => value,
        }
    }

    /// 🔏️ Revision identity of one edit — a pure function of [`Self::revision_value`]. A single-operation edit hashes that
    /// value's canonical JSON (exactly what the one-item byte sealer streams); a coalesced edit that grew past one operation
    /// hashes its header fields and one running chain per operation list ([`EditDigestChains`]), which an amend extends in
    /// O(appended).
    pub fn edit_digest<Mutation: ToValue>(edit: &Edit<Mutation>) -> [u8; 32] {
        Self::edit_digest_extending(edit, None).0
    }

    /// 🔏️ [`Self::edit_digest`], continuing `known` chains when the edit only grew since they were taken (an amend).
    pub(crate) fn edit_digest_extending<Mutation: ToValue>(edit: &Edit<Mutation>, known: Option<EditDigestChains>) -> ([u8; 32], Option<EditDigestChains>) {
        if edit.forwards.len() <= 1 {
            let encoded = semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&Self::revision_value(edit))).into_bytes();
            return (Self::hash_record(b"edit", &[edit.id.as_bytes(), &encoded]), None);
        }
        let chains = known.and_then(|chains| chains.extended(edit)).unwrap_or_else(|| EditDigestChains::default().extended(edit).expect("empty chains extend every edit"));
        let tag = |present: bool| [u8::from(present)];
        let digest = Self::hash_record(
            b"edit-chained",
            &[
                edit.id.as_bytes(),
                &tag(edit.actor.is_some()),
                edit.actor.as_deref().unwrap_or_default().as_bytes(),
                edit.started_at.as_bytes(),
                &tag(edit.finished_at.is_some()),
                edit.finished_at.as_deref().unwrap_or_default().as_bytes(),
                &(chains.forwards as u64).to_be_bytes(),
                &chains.forwards_digest,
                &(chains.inverse as u64).to_be_bytes(),
                &chains.inverse_digest,
                &(chains.meta as u64).to_be_bytes(),
                &chains.meta_digest,
            ],
        );
        let digest = match edit.verb.as_deref() {
            Some(verb) => Self::hash_record(b"edit-verb", &[&digest, verb.as_bytes()]),
            None => digest,
        };
        let digest = Self::hash_record(b"edit-line", &[&digest, &[u8::from(edit.line.is_some())], edit.line.as_deref().unwrap_or("").as_bytes()]);
        (digest, Some(chains))
    }

    /// 🧮️ Re-derives one cursor stack's records for `ids`, keeping the common prefix. `tail_chains` carries the running
    /// chains of the stack's tail edit between calls, so the amend case (same ids, tail grew) costs O(appended operations).
    /// 🔏️ `digest` of `edit` extended by the digest of the supersessions in effect for its operations: a revision names
    /// the effective forwards, not only the recorded ones.
    pub(crate) fn effective_edit_digest<P, Mutation: self::Mutation<P>>(edit: &Edit<Mutation>, digest: [u8; 32], supersessions: &EffectiveSupersessions) -> [u8; 32] {
        match edit_supersession_digest::<P, Mutation>(edit, supersessions) {
            Some(superseded) => Self::hash_record(b"edit-superseded", &[&digest, &superseded]),
            None => digest,
        }
    }

    /// 🧮️ Rebuilds the records of `ids` from the first position whose id changed (`dirty`, [`StackMirrorDirt`]) and keeps
    /// the common prefix untouched; a tail edit that only grew (an amend) regrows its one record. `dirty_from` names the
    /// first position a replay rewrote (effective input, inverse or messages): records from there on are rebuilt even where
    /// the ids still match. Answers the position of a regrown tail record.
    #[allow(clippy::too_many_arguments)]
    fn reconcile_stack<P, Mutation: ToValue + self::Mutation<P>>(
        records: &mut semio_framework_os_vcs::HistoryPageStack<CursorRevisionRecord>,
        ids: &semio_framework_os_vcs::HistoryPageStack<String>,
        edits: &ArtifactHistoryLedger<Edit<Mutation>>,
        domain: &[u8],
        identity_digest: [u8; 32],
        tail_chains: &mut Option<([u8; 32], EditDigestChains)>,
        supersessions: &EffectiveSupersessions,
        dirty: usize,
        dirty_from: Option<usize>,
    ) -> Option<usize> {
        let mut common = records.len().min(ids.len()).min(dirty);
        if let Some(dirty_from) = dirty_from {
            common = common.min(dirty_from);
            if common < records.len() {
                *tail_chains = None;
            }
        }
        while records.len() > common {
            records.pop().expect("revision suffix record remains present");
        }
        let mut regrown = None;
        if common == ids.len() && common != 0 {
            let id = &ids[common - 1];
            let edit = edits.find_near(|edit| edit.id == *id).expect("validated cursor edit exists");
            let known = tail_chains.filter(|(id_digest, _)| *id_digest == records[common - 1].id_digest).map(|(_, chains)| chains);
            let (edit_digest, chains) = Self::edit_digest_extending(edit, known);
            let edit_digest = Self::effective_edit_digest::<P, Mutation>(edit, edit_digest, supersessions);
            *tail_chains = chains.map(|chains| (records[common - 1].id_digest, chains));
            if records[common - 1].edit_digest != edit_digest {
                records.pop().expect("validated revision record remains present");
                common -= 1;
                regrown = Some(edit_digest);
            }
        }
        let regrown_at = regrown.is_some().then_some(common);
        #[cfg(any(test, feature = "hot-path-census"))]
        hot_path_census::count(|census| census.rebuilt += ids.len() - common);
        for id in ids.iter_from(common) {
            let id_digest = Self::hash_record(b"edit-id", &[id.as_bytes()]);
            let edit_digest = match regrown.take() {
                Some(edit_digest) => edit_digest,
                None => {
                    let edit = edits.find_near(|edit| edit.id == *id).expect("validated cursor edit exists");
                    let (edit_digest, chains) = Self::edit_digest_extending(edit, None);
                    *tail_chains = chains.map(|chains| (id_digest, chains));
                    Self::effective_edit_digest::<P, Mutation>(edit, edit_digest, supersessions)
                }
            };
            let previous = records.last().map_or(identity_digest, |record| record.prefix_digest);
            let prefix_digest = Self::hash_record(domain, &[&previous, &edit_digest]);
            let ledger_key = edits.find_near_key(|edit| edit.id == *id).map(|(key, _)| key).expect("validated cursor generation exists");
            records.push(CursorRevisionRecord { ledger_key: Some(ledger_key), id_digest, edit_digest, prefix_digest });
        }
        regrown_at
    }

    /// 🧮️ Reconciles both cursor stacks ([`Self::reconcile_stack`]); a replay rewrite rebuilds the whole redo stack. Answers
    /// the position of a regrown applied tail record.
    pub fn reconcile<P, Mutation: ToValue + self::Mutation<P>>(
        &mut self,
        applied_ids: &semio_framework_os_vcs::HistoryPageStack<String>,
        redo_ids: &semio_framework_os_vcs::HistoryPageStack<String>,
        edits: &ArtifactHistoryLedger<Edit<Mutation>>,
        supersessions: &EffectiveSupersessions,
        dirty: StackMirrorDirt,
        dirty_from: Option<usize>,
    ) -> Option<usize> {
        let regrown = Self::reconcile_stack::<P, Mutation>(&mut self.applied, applied_ids, edits, b"applied", self.identity_digest, &mut self.applied_tail_chains, supersessions, dirty.applied, dirty_from);
        let start = dirty.applied.min(dirty_from.unwrap_or(usize::MAX)).min(regrown.unwrap_or(usize::MAX));
        for (position, id) in applied_ids.iter_from(start.min(applied_ids.len())).enumerate() {
            let edit = edits.find_near(|edit| edit.id == *id).expect("validated indexed edit exists");
            self.index_applied_edit::<P, Mutation>(position + start, edit);
        }
        Self::reconcile_stack::<P, Mutation>(&mut self.redo, redo_ids, edits, b"redo", self.identity_digest, &mut None, supersessions, dirty.redo, dirty_from.map(|_| 0));
        regrown
    }

    pub fn index_applied_edit<P, Mu: Mutation<P>>(&mut self, position: usize, edit: &Edit<Mu>) {
        let digest = Self::hash_record(b"edit-id", &[edit.id.as_bytes()]);
        let mut from = self.indexed_edits.get(&digest).filter(|(known, _)| *known == position).map_or(0, |(_, count)| *count).min(edit.forwards.len());
        if from > 0 {
            let first = semio_framework_os_spr::mutation_id_for_edit_operation::<P, Mu>(edit, 0).expect("indexed operation exists");
            if self.mutation_positions.get(&first).is_none_or(|(known, _, _)| *known != position) { from = 0; }
        }
        for index in from..edit.forwards.len() {
            let mutation = semio_framework_os_spr::mutation_id_for_edit_operation::<P, Mu>(edit, index).expect("indexed operation exists");
            self.mutation_positions.insert(mutation, (position, index, digest));
            if edit.forwards[index].may_emit_foreign_steps() && !self.unit_flags.contains_key(&(digest, index)) { self.unit_flags.insert((digest, index), false); }
        }
        self.indexed_edits.insert(digest, (position, edit.forwards.len()));
    }

    pub fn revision(&self, checkpoint_id: Option<&str>) -> [u8; 32] {
        let applied = self.applied.last().map_or(self.identity_digest, |record| record.prefix_digest);
        let redo = self.redo.last().map_or(self.identity_digest, |record| record.prefix_digest);
        let checkpoint = checkpoint_id.unwrap_or_default().as_bytes();
        Self::hash_record(b"cursor", &[&applied, &(self.applied.len() as u64).to_be_bytes(), &redo, &(self.redo.len() as u64).to_be_bytes(), checkpoint])
    }
}

pub use semio_framework_os_store_message_ledger::*;

pub struct ArtifactStoreHistoryCommitReservation {
    pub history: ArtifactHistoryReservation,
    pub rejected_owner: ArtifactStoreDisplacedOwnerReservation,
}

/// 📨️ See `ArtifactStore::pending_report`'s doc.
#[derive(Default)]
pub struct PendingCommandReport {
    /// 🎯️ Overrides `dispatch`'s tail-diff `edit_ids` guess — needed once a merge can insert
    /// mid-history (`ingest_remote`/`resolve_conflict`), where "everything past `before`" no longer
    /// names only the newly-landed edits.
    pub(crate) edit_ids: Option<Vec<String>>,
    pub(crate) messages: Vec<semio_framework_os_spr::EditMessages>,
    pub(crate) worst: Option<semio_framework_diagnostic::Severity>,
    /// 📤️ Every event this command authored (edit operations, history transitions), in authoring
    /// order — announced outbound as one `BackboneMessage::Mutations` batch when the command ends.
    pub(crate) outbound: Vec<semio_framework_os_spr::MutationEnvelope>,
}

pub struct ArtifactStorePendingReportRetirement {
    pub report: std::mem::ManuallyDrop<Option<PendingCommandReport>>,
    pub active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
}

impl ArtifactStorePendingReportRetirement {
    pub(crate) fn new(report: PendingCommandReport) -> Self { Self { report: std::mem::ManuallyDrop::new(Some(report)), active: std::mem::ManuallyDrop::new(None) } }
    pub fn demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        if let Some(active) = self.active.as_ref() { return artifact_retirement_box_demands(active, body); }
        let Some(report) = self.report.as_ref() else { return Ok(Default::default()); };
        let capacity_bytes = if !report.outbound.is_empty() { size_of::<ArtifactStoreConflictRetirement>() }
        else if !report.messages.is_empty() { size_of::<ArtifactStoreMessageLedgerRetirement>() }
        else if report.edit_ids.as_ref().is_some_and(|ids| !ids.is_empty()) { size_of::<ArtifactStoreStringRetirement>() } else { 0 };
        if capacity_bytes != 0 { return Ok(RetirementDemand { capacity_bytes, depth: 2, ..Default::default() }); }
        let release_bytes = if report.outbound.capacity() != 0 { std::alloc::Layout::array::<semio_framework_os_spr::MutationEnvelope>(report.outbound.capacity()) }
        else if report.messages.capacity() != 0 { std::alloc::Layout::array::<semio_framework_os_spr::EditMessages>(report.messages.capacity()) }
        else { std::alloc::Layout::array::<String>(report.edit_ids.as_ref().map_or(0, Vec::capacity)) }
        .map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "pending report original backing layout overflow"))?.size();
        Ok(RetirementDemand { release_bytes, depth: 1, ..Default::default() })
    }
}

impl ErasedSnapshotRetirement for ArtifactStorePendingReportRetirement {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "pending report retirement exceeds admitted depth")); }
        if grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_copy_bytes < demand.copy_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        if self.active.is_some() { return artifact_retirement_box_close_step(&mut self.active, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        let report = self.report.as_mut().expect("nonterminal report retains original");
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if let Some(original) = report.outbound.pop() {
            match admit_artifact_retirement(original, child, ArtifactStoreConflictRetirement::causal_envelope) {
                Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, original)) => { report.outbound.push(original); return Err(error); }
            }
        }
        if let Some(original) = report.messages.pop() {
            match admit_artifact_retirement(original, child, |entry| ArtifactStoreMessageLedgerRetirement::new(entry.edit_id, entry.messages)) {
                Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, original)) => { report.messages.push(original); return Err(error); }
            }
        }
        if let Some(original) = report.edit_ids.as_mut().and_then(Vec::pop) {
            match admit_artifact_retirement(original, child, ArtifactStoreStringRetirement::new) {
                Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, original)) => { report.edit_ids.as_mut().expect("original edit-id backing").push(original); return Err(error); }
            }
        }
        if report.outbound.capacity() != 0 { drop(std::mem::take(&mut report.outbound)); }
        else if report.messages.capacity() != 0 { drop(std::mem::take(&mut report.messages)); }
        else if report.edit_ids.is_some() { drop(report.edit_ids.take()); }
        else { report.worst = None; drop(self.report.take()); return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..empty })); }
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty }))
    }
    fn terminal_is_empty(&self) -> bool { self.report.is_none() && self.active.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.demands(body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl Drop for ArtifactStorePendingReportRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal_is_empty()), "artifact store pending-report retirement reached Drop before its exact terminal-empty witness");
    }
}

pub async fn validate_composition_pins(pins: &[semio_framework_os_vcs::CompositionPin]) -> Result<(), VcsError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    let mut children = HashSet::new();
    for pin in pins {
        let child_uri = pin.child_ref.to_uri();
        let reparsed = semio_framework_artifact_reference::ArtifactRef::parse_uri(&child_uri).map_err(VcsError::ValidationFailed)?;
        if reparsed != pin.child_ref || pin.child_ref.artifact_id.trim().is_empty() || pin.checkpoint_id.trim().is_empty() {
            return Err(VcsError::ValidationFailed(format!("invalid composition pin {child_uri}")));
        }
        if !children.insert(child_uri.clone()) {
            return Err(VcsError::ValidationFailed(format!("checkpoint repeats composition child {child_uri}")));
        }
    }
    Ok(())
}

// 🎯️ W6 kernel unification: this crate's own `mutation_envelope_from_edit` (whole-edit-per-
// envelope, JSON payload) is DELETED — `flush_outbound` now calls `crate::os_spr::
// mutation_envelope_from_edit` directly (one `crate::os_spr::MutationEnvelope` per forward op,
// `OpBinary`-encoded payloads — W5's frozen-contract signature). `hash_bytes`'s import above this
// region stays needed elsewhere in this file (`replay_mutations`'s `payload_hash`, unaffected).

/// 📦️ Recovers a single-op `Edit` from one causal wire envelope. `crate::os_spr::causal::
/// MutationEnvelope` carries exactly one op per envelope (W5's binary reshape) — the receiving-
/// side half of the per-op fan-out `crate::os_spr::mutation_envelope_from_edit` performs when sending
/// (see `flush_outbound`). `sequence_number`/`started_at` are placeholders `ingest_envelope`
/// overwrites (mirroring the local-edit convention: `self.edit_sequence += 1` then `now_iso()`).
/// `undo_policy` defaults to `ExactBaseOnly` — not a lossy conversion: `crate::os_spr::causal::
/// MutationEnvelope` carries no undo_policy at all (only the local `Edit`/`MutationMeta` shape
/// does), and a remote edit is always foreign, so this field is never consulted for it anyway
/// (`edit_is_local` gates undo eligibility on authorship, not `undo_policy`).
pub async fn edit_from_operation_envelope<Mutation: OpBinary>(envelope: &semio_framework_os_spr::MutationEnvelope) -> Result<Edit<Mutation>, VcsError> {
    let forward = Mutation::decode_op(&envelope.diff.payload).map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let inverse = if envelope.inverse.payload.is_empty() { Vec::new() } else { vec![Mutation::decode_op(&envelope.inverse.payload).map_err(|error| VcsError::Deserialize(error.to_string()))?] };
    Ok(Edit {
        id: envelope.mutation_id.0.clone(),
        actor: Some(envelope.actor.0.clone()),
        forwards: vec![forward],
        inverse: inverse.into(),
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
            transaction: envelope.transaction.clone(),
        }],
        verb: envelope.verb.clone(),
        line: envelope.line.clone(),
        sequence_number: 0,
        started_at: String::new(),
        finished_at: None,
    })
}

pub use semio_framework_os_store_projection::retire_replayed_projection;

/// 🧊️ The live intermediate of a history fold, which retires itself on EVERY exit path. A fold walks
/// `base → mid₁ → … → head`; its loop body already retires the projection each step displaces, but an
/// early return — an unknown edit id, a `MutationDiff::apply` refusal, a saturated owner queue —
/// carried the still-live one straight into a bare drop, and an artifact whose projection owns a
/// fail-closed root (an `OrderedMap`, a neural `Dictionary`) aborts the guest there. That is why
/// 🖨️raster's `redo` kept trapping after its `retire_projection`/`retire_cold` overrides landed:
/// `undo` answers from the tail cache and folds nothing, while `redo` re-applies the reinstated
/// edit's forwards in `ArtifactStore::project_applied` (since folded into [`ArtifactStore::reproject`]) and meets the `?`
/// (measured 2026-09-20).
/// Holding the intermediate here makes the retirement STRUCTURAL — a fold that grows a new early
/// return cannot silently miss it, and no technology has to override anything extra to be safe.
pub struct ReplayProjection<P, Mutation>
where
    Mutation: self::Mutation<P>,
{
    pub projection: Option<P>,
    technology: PhantomData<fn() -> Mutation>,
}

impl<P, Mutation> ReplayProjection<P, Mutation>
where
    Mutation: self::Mutation<P>,
{
    /// 🌱️ Takes ownership of a fold's base projection.
    pub(crate) fn new(projection: P) -> Self {
        Self { projection: Some(projection), technology: PhantomData }
    }

    /// ▶️ Installs the next projection and retires the one it displaces.
    pub fn advance(&mut self, next: P) {
        let displaced = self.projection.replace(next).expect("a live replay projection is present until it is handed out");
        retire_replayed_projection::<P, Mutation>(displaced);
    }

    /// 🎁️ Hands the finished projection to the caller, who owns its retirement from here on.
    pub fn into_inner(mut self) -> P {
        self.projection.take().expect("a live replay projection is handed out exactly once")
    }
}

impl<P, Mutation> std::ops::Deref for ReplayProjection<P, Mutation>
where
    Mutation: self::Mutation<P>,
{
    type Target = P;
    fn deref(&self) -> &P {
        self.projection.as_ref().expect("a live replay projection is present until it is handed out")
    }
}

impl<P, Mutation> Drop for ReplayProjection<P, Mutation>
where
    Mutation: self::Mutation<P>,
{
    fn drop(&mut self) {
        if let Some(projection) = self.projection.take() {
            retire_replayed_projection::<P, Mutation>(projection);
        }
    }
}

pub use semio_framework_os_store_command::{retire_scratch_operations, retire_scratch_edits, retire_supersede_inputs};

/// 🧮️ Folds an envelope's complete event log — its edits and its history transitions — into
/// the history projection (`crate::os_spr::fold_history`). Quarantined edits (every edit owning an
/// operation of a quarantined conflict that was not accepted) stay inactive.
pub fn fold_envelope_history<P, Mutation>(envelope: &ArtifactEnvelopeOwners<P, Mutation>) -> Result<semio_framework_os_spr::HistoryFold, VcsError>
where
    Mutation: self::Mutation<P>,
{
    let head = envelope_viewer_head(envelope);
    fold_event_log::<P, Mutation>(&envelope.id, &envelope.vcs.edits.iter().collect::<Vec<_>>(), &envelope.transitions, &envelope.conflicts, &head)
}

/// 👁 The head [`fold_history_for`] projects: this replica's alternative and explicit checkpoint.
/// Absent alternative is the canonical trunk. `viewer_checkpoint_id` of `None` is that line's tip.
pub fn envelope_viewer_head<P, Mutation>(envelope: &ArtifactEnvelopeOwners<P, Mutation>) -> semio_framework_os_spr::ViewerHead {
    let document_id = ArtifactId(envelope.id.clone());
    let trunk = semio_framework_os_spr::trunk_alternative_id(&document_id);
    semio_framework_os_spr::ViewerHead { line_id: envelope.active_alternative_id.clone().filter(|line| line != &trunk).unwrap_or(trunk), checkpoint_id: envelope.viewer_checkpoint_id.clone() }
}

/// 🧮️ [`fold_envelope_history`] over the bare event log: `edits`, `transitions` and the
/// `conflicts` whose quarantine withholds edits — for document shapes that hold the log without a
/// full envelope.
pub fn fold_event_log<P, Mutation>(document_id: &str, edits: &[&Edit<Mutation>], transitions: &[semio_framework_os_spr::MutationEnvelope], conflicts: &[semio_framework_os_spr::Conflict], head: &semio_framework_os_spr::ViewerHead) -> Result<semio_framework_os_spr::HistoryFold, VcsError>
where
    Mutation: self::Mutation<P>,
{
    let mut folded = Vec::with_capacity(edits.len());
    let mut owners: HashMap<String, String> = HashMap::new();
    for edit in edits.iter().copied() {
        let mutation_ids = semio_framework_os_spr::mutation_ids_for_edit::<P, Mutation>(edit);
        for mutation_id in &mutation_ids {
            owners.insert(mutation_id.0.clone(), edit.id.clone());
        }
        folded.push(semio_framework_os_spr::FoldEdit { id: edit.id.clone(), actor: edit.actor.clone(), timestamp: edit_timestamp(edit), mutation_ids, line: edit.line.clone() });
    }
    let mut excluded = HashSet::new();
    for conflict in conflicts {
        if conflict.status == semio_framework_os_spr::ConflictStatus::Accepted {
            continue;
        }
        if let semio_framework_os_spr::ConflictKind::Quarantined { envelopes, .. } = &conflict.kind {
            excluded.extend(envelopes.iter().filter_map(|quarantined| owners.get(&quarantined.mutation_id.0).cloned()));
        }
    }
    semio_framework_os_spr::fold_history_for(&ArtifactId(document_id.to_string()), &folded, transitions, &excluded, head).map_err(|error| VcsError::ValidationFailed(error.to_string()))
}

/// 🗣️ `<document schema>#<semantic kind>` of the leaf `operation` is — what `MutationMeta.semantic_kind` records.
pub fn operation_semantic_kind<P, Mutation: self::Mutation<P>>(operation: &Mutation, schema: &str) -> SchemaId {
    SchemaId(format!("{schema}#{}", operation.descriptor().semantic_kind))
}

/// 🗣️ Stamps every metadata record of `edit` with its operation's semantic kind and descriptor name — derived facts no
/// persisted form carries, so every load and ingest path re-derives them.
pub fn stamp_edit_semantics<P, Mutation: self::Mutation<P>>(edit: &mut Edit<Mutation>, schema: &str) {
    for (meta, operation) in edit.mutation_meta.iter_mut().zip(edit.forwards.iter()) {
        meta.semantic_kind = Some(operation_semantic_kind::<P, Mutation>(operation, schema));
        meta.label = Some(operation.descriptor().display_name.to_string());
    }
}

/// ⏰️ An edit's HLC: its first operation's stamped timestamp (the order key every fold uses).
pub fn edit_timestamp<Mutation>(edit: &Edit<Mutation>) -> HybridLogicalTimestamp {
    edit.mutation_meta.first().map_or(HybridLogicalTimestamp { actor: 0, physical_ms: 0, logical: 0 }, |meta| meta.timestamp)
}

impl<P, Mutation> FoldProjection<P, Mutation> for ReplayProjection<P, Mutation>
where
    Mutation: self::Mutation<P>,
{
    fn projection(&self) -> &P {
        self
    }

    fn advance(&mut self, next: P) {
        ReplayProjection::advance(self, next);
    }
}
