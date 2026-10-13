//! 📒️ The edit ledger beneath the Store engine: the paged edit-message ledger and its retirement, the cursor revision
//! accumulator with its records and digest chains, the history commit reservation and pending command report, and the
//! envelope fold and projection helpers every history, replay, codec and composition facet reads.
//! It names the vocabulary below it (schemas, snapshot-read retirement, effective forwards) and never the
//! `ArtifactStore` engine, so the engine and every facet above it depend on it in one direction.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
#[cfg(test)] use super::hot_path_census;
use semio_framework_value::ErasedSnapshotRetirement;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use crate::os_spr::command::ArtifactMessageSource;
use semio_framework_value::DslValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use crate::os_spr::{ArtifactId, HybridLogicalTimestamp, MutationId, SchemaId, UndoPolicy};
use crate::os_spr::{Edit, Mutation, MutationDiff, MutationMeta, OpBinary};
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use crate::os_spr::command::MutationMessageLedgerRetirement as ArtifactStoreMessageLedgerRetirement;
use crate::os_spr::conflict::ProtocolConflictRetirement as ArtifactStoreConflictRetirement;
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
use crate::os_vcs::VcsError;
use crate::os_vcs::{
    ArtifactHistoryKey, ArtifactHistoryLedger, ArtifactHistoryReservation, Identified,
};
use crate::os_pack::pack_rt;
use super::{ArtifactEnvelope, ArtifactEnvelopeOwners, ArtifactStoreDisplacedOwnerReservation, ArtifactStoreStringRetirement, EffectiveSupersessions, FoldProjection, SupersedeInput, artifact_retirement_box_close_step, artifact_retirement_box_demands, artifact_retirement_owner_close, artifact_retirement_owner_demands, edit_supersession_digest};

#[derive(Clone)]
pub(crate) struct CursorRevisionRecord {
    pub(crate) ledger_key: Option<ArtifactHistoryKey>,
    pub(crate) id_digest: [u8; 32],
    pub(crate) edit_digest: [u8; 32],
    pub(crate) prefix_digest: [u8; 32],
}

#[derive(Clone)]
pub(crate) struct CursorRevisionAccumulator {
    pub(crate) identity_digest: [u8; 32],
    pub(crate) applied: crate::os_vcs::HistoryPageStack<CursorRevisionRecord>,
    pub(crate) redo: crate::os_vcs::HistoryPageStack<CursorRevisionRecord>,
    pub(crate) applied_tail_chains: Option<([u8; 32], EditDigestChains)>,
    pub(crate) mutation_positions: protocol::HistoryFoldIndex<MutationId, (usize, usize, [u8; 32])>,
    pub(crate) indexed_edits: protocol::HistoryFoldIndex<[u8; 32], (usize, usize)>,
    pub(crate) unit_flags: protocol::HistoryFoldIndex<([u8; 32], usize), bool>,
}

/// 📍️ The first applied and redo positions whose ids changed since [`ArtifactStore::bump`] last mirrored both stacks into the
/// persisted cursor and the revision records — what keeps both mirrors O(change) per mutation. `usize::MAX` is clean.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct StackMirrorDirt {
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
pub(crate) struct EditDigestChains {
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
        Self { identity_digest, applied: crate::os_vcs::HistoryPageStack::new(), redo: crate::os_vcs::HistoryPageStack::new(), applied_tail_chains: None, mutation_positions: protocol::HistoryFoldIndex::new(), indexed_edits: protocol::HistoryFoldIndex::new(), unit_flags: protocol::HistoryFoldIndex::new() }
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
    pub(crate) fn edit_digest<Mutation: ToValue>(edit: &Edit<Mutation>) -> [u8; 32] {
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
        records: &mut crate::os_vcs::HistoryPageStack<CursorRevisionRecord>,
        ids: &crate::os_vcs::HistoryPageStack<String>,
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
        #[cfg(test)]
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
    pub(crate) fn reconcile<P, Mutation: ToValue + self::Mutation<P>>(
        &mut self,
        applied_ids: &crate::os_vcs::HistoryPageStack<String>,
        redo_ids: &crate::os_vcs::HistoryPageStack<String>,
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

    pub(crate) fn index_applied_edit<P, Mu: Mutation<P>>(&mut self, position: usize, edit: &Edit<Mu>) {
        let digest = Self::hash_record(b"edit-id", &[edit.id.as_bytes()]);
        let mut from = self.indexed_edits.get(&digest).filter(|(known, _)| *known == position).map_or(0, |(_, count)| *count).min(edit.forwards.len());
        if from > 0 {
            let first = crate::os_spr::mutation_id_for_edit_operation::<P, Mu>(edit, 0).expect("indexed operation exists");
            if self.mutation_positions.get(&first).is_none_or(|(known, _, _)| *known != position) { from = 0; }
        }
        for index in from..edit.forwards.len() {
            let mutation = crate::os_spr::mutation_id_for_edit_operation::<P, Mu>(edit, index).expect("indexed operation exists");
            self.mutation_positions.insert(mutation, (position, index, digest));
            if edit.forwards[index].may_emit_foreign_steps() && !self.unit_flags.contains_key(&(digest, index)) { self.unit_flags.insert((digest, index), false); }
        }
        self.indexed_edits.insert(digest, (position, edit.forwards.len()));
    }

    pub(crate) fn revision(&self, checkpoint_id: Option<&str>) -> [u8; 32] {
        let applied = self.applied.last().map_or(self.identity_digest, |record| record.prefix_digest);
        let redo = self.redo.last().map_or(self.identity_digest, |record| record.prefix_digest);
        let checkpoint = checkpoint_id.unwrap_or_default().as_bytes();
        Self::hash_record(b"cursor", &[&applied, &(self.applied.len() as u64).to_be_bytes(), &redo, &(self.redo.len() as u64).to_be_bytes(), checkpoint])
    }
}

pub(crate) const ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY: usize = 8_192;

pub(crate) const ARTIFACT_EDIT_MESSAGE_ID_BYTES: usize = 256;

pub(crate) const ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES: usize = crate::os_spr::command::MUTATION_MESSAGE_ENTRY_BYTES;

pub(crate) const ARTIFACT_EDIT_MESSAGE_OWNER_BYTES: usize = crate::os_spr::command::MUTATION_MESSAGE_OWNER_BYTES;

pub(crate) const ARTIFACT_EDIT_MESSAGE_TARGET_BYTES: usize = crate::os_spr::command::MUTATION_MESSAGE_TARGET_BYTES;

pub(crate) const ARTIFACT_EDIT_MESSAGE_LEDGER_BYTES: usize = ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY * ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES;

pub(crate) const ARTIFACT_EDIT_MESSAGE_INDEX_EMPTY: u16 = u16::MAX;

pub(crate) const ARTIFACT_EDIT_MESSAGE_INDEX_TOMBSTONE: u16 = u16::MAX - 1;

pub(crate) const ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE: u16 = u16::MAX;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ArtifactEditMessageTicket {
    pub(crate) slot: u16,
    pub(crate) generation: u64,
}

pub(crate) const ARTIFACT_EDIT_MESSAGE_PAGE_SLOTS: usize = 64;

pub(crate) const ARTIFACT_EDIT_MESSAGE_SLOT_PAGES: usize = ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY / ARTIFACT_EDIT_MESSAGE_PAGE_SLOTS;

/// 🧱 Retains fixed message ticket slots in independently admitted physical pages.
pub(crate) struct ArtifactEditMessageSlots {
    pub(crate) pages: [Option<Box<[std::mem::MaybeUninit<crate::os_spr::EditMessages>]>>; ARTIFACT_EDIT_MESSAGE_SLOT_PAGES],
    admitted: usize,
}

impl ArtifactEditMessageSlots {
    fn empty() -> Self { Self { pages: std::array::from_fn(|_| None), admitted: 0 } }
    fn new() -> Self { Self { pages: std::array::from_fn(|_| Some(Box::<[crate::os_spr::EditMessages]>::new_uninit_slice(ARTIFACT_EDIT_MESSAGE_PAGE_SLOTS))), admitted: ARTIFACT_EDIT_MESSAGE_SLOT_PAGES } }
    fn len(&self) -> usize { self.admitted * ARTIFACT_EDIT_MESSAGE_PAGE_SLOTS }
    fn is_empty(&self) -> bool { self.pages.iter().all(Option::is_none) }
    fn admission_demands(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.admitted == ARTIFACT_EDIT_MESSAGE_SLOT_PAGES { return Ok(Default::default()); }
        let capacity_bytes = size_of::<std::mem::MaybeUninit<crate::os_spr::EditMessages>>().checked_mul(ARTIFACT_EDIT_MESSAGE_PAGE_SLOTS).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "message page extent overflowed"))?;
        Ok(semio_framework_value::RetirementDemand { capacity_bytes, depth: 1, ..Default::default() })
    }
    fn admit_next(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        if self.admitted == ARTIFACT_EDIT_MESSAGE_SLOT_PAGES { return Ok(Default::default()); }
        let demand = self.admission_demands()?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "message page birth requires original depth")); }
        if grant.maximum_items == 0 || grant.maximum_capacity_bytes < demand.capacity_bytes { return Ok(Default::default()); }
        self.pages[self.admitted] = Some(Box::<[crate::os_spr::EditMessages]>::new_uninit_slice(ARTIFACT_EDIT_MESSAGE_PAGE_SLOTS));
        self.admitted += 1;
        let progress = RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: demand.capacity_bytes, ..Default::default() };
        semio_framework_value::retained_clone::admit_retained_clone_progress(grant, progress, "original message slot page birth")?;
        Ok(progress)
    }
    fn release_demands(&self) -> semio_framework_value::RetirementDemand {
        self.pages.iter().rev().find_map(|page| page.as_ref()).map_or(Default::default(), |page| semio_framework_value::RetirementDemand { release_bytes: size_of_val(page.as_ref()), depth: 1, ..Default::default() })
    }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let demand = self.release_demands();
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "message page release requires original depth")); }
        if grant.maximum_items == 0 || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let index = self.pages.iter().rposition(Option::is_some).expect("original message page remains retained");
        drop(self.pages[index].take());
        self.admitted = index;
        let progress = RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() };
        Ok(if self.is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }
}

impl std::ops::Index<usize> for ArtifactEditMessageSlots {
    type Output = std::mem::MaybeUninit<crate::os_spr::EditMessages>;
    fn index(&self, index: usize) -> &Self::Output { &self.pages[index / ARTIFACT_EDIT_MESSAGE_PAGE_SLOTS].as_ref().expect("message ticket page remains admitted")[index % ARTIFACT_EDIT_MESSAGE_PAGE_SLOTS] }
}

impl std::ops::IndexMut<usize> for ArtifactEditMessageSlots {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.pages[index / ARTIFACT_EDIT_MESSAGE_PAGE_SLOTS].as_mut().expect("message ticket page remains admitted")[index % ARTIFACT_EDIT_MESSAGE_PAGE_SLOTS] }
}

pub struct ArtifactEditMessageLedger {
    pub(crate) slots: ArtifactEditMessageSlots,
    pub(crate) generations: Box<[u64]>,
    pub(crate) occupied: Box<[bool]>,
    pub(crate) next: Box<[u16]>,
    pub(crate) previous: Box<[u16]>,
    pub(crate) buckets: Box<[u16]>,
    pub(crate) free: Box<[u16]>,
    free_len: usize,
    head: u16,
    tail: u16,
    len: usize,
}

/// 🗃️ Retains every original sparse message array until its exact physical release turn.
pub(crate) struct ArtifactEditMessageLedgerRetirement {
    pub(crate) original: std::mem::ManuallyDrop<ArtifactEditMessageLedger>,
    active: Option<ArtifactStoreMessageLedgerRetirement>,
}

impl ArtifactEditMessageLedgerRetirement {
    pub(crate) fn new(original: ArtifactEditMessageLedger) -> Self { Self { original: std::mem::ManuallyDrop::new(original), active: None } }
    pub(crate) fn demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.active.is_some() { return artifact_retirement_owner_demands(&self.active, body); }
        if self.original.len != 0 { return Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<crate::os_spr::EditMessages>(), depth: 1, ..Default::default() }); }
        macro_rules! extent { ($field:ident) => { if !self.original.$field.is_empty() { return Ok(semio_framework_value::RetirementDemand { release_bytes: std::mem::size_of_val(self.original.$field.as_ref()), depth: 1, ..Default::default() }); } }; }
        if !self.original.slots.is_empty() { return Ok(self.original.slots.release_demands()); }
        extent!(generations); extent!(occupied); extent!(next); extent!(previous); extent!(buckets); extent!(free);
        Ok(Default::default())
    }
}

impl ErasedSnapshotRetirement for ArtifactEditMessageLedgerRetirement {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "message directory requires admitted original depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        if self.active.is_some() { return artifact_retirement_owner_close(&mut self.active, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(entry) = self.original.pop() {
            self.active = Some(ArtifactStoreMessageLedgerRetirement::new(entry.edit_id, entry.messages));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        macro_rules! release { ($field:ident) => {
            if !self.original.$field.is_empty() {
                drop(std::mem::take(&mut self.original.$field));
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty }));
            }
        }; }
        if !self.original.slots.is_empty() { return self.original.slots.close_step(grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        release!(generations); release!(occupied); release!(next); release!(previous); release!(buckets);
        if !self.original.free.is_empty() {
            drop(std::mem::take(&mut self.original.free));
            self.original.free_len = 0;
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty }));
        }
        Ok(RetainedCloneStep::Complete(empty))
    }
    fn terminal_is_empty(&self) -> bool { self.active.is_none() && self.original.len == 0 && self.original.backing_is_empty() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.demands(body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl Drop for ArtifactEditMessageLedgerRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "original message ledger reached Drop before every backing array was physically released");
        if self.terminal_is_empty() { unsafe { std::mem::ManuallyDrop::drop(&mut self.original); } }
    }
}

pub struct ArtifactEditMessageLedgerIter<'a> {
    ledger: &'a ArtifactEditMessageLedger,
    next: u16,
}

impl<'a> Iterator for ArtifactEditMessageLedgerIter<'a> {
    type Item = &'a crate::os_spr::EditMessages;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next == ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE {
            return None;
        }
        let slot = usize::from(self.next);
        self.next = self.ledger.next[slot];
        Some(unsafe { self.ledger.slots[slot].assume_init_ref() })
    }
}

pub struct ArtifactEditMessageLedgerRejected {
    inner: crate::os_spr::command::EditMessageLedgerRetirement,
    reason: &'static str,
}

impl ArtifactEditMessageLedger {
    /// 🪶 Starts allocation-free fixed ticket admission without transferring any payload.
    pub fn empty() -> Self {
        Self { slots: ArtifactEditMessageSlots::empty(), generations: Box::default(), occupied: Box::default(), next: Box::default(), previous: Box::default(), buckets: Box::default(), free: Box::default(), free_len: 0, head: ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE, tail: ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE, len: 0 }
    }
    /// 🧮 Quotes only the next original backing under independent caller policy.
    pub fn admission_demands(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.slots.len() != ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY { return self.slots.admission_demands(); }
        let width = if self.generations.is_empty() { size_of::<u64>() } else if self.occupied.is_empty() { size_of::<bool>() } else if self.next.is_empty() || self.previous.is_empty() || self.buckets.is_empty() || self.free.is_empty() { size_of::<u16>() } else { return Ok(Default::default()); };
        let capacity_bytes = width.checked_mul(ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "message index backing extent overflowed"))?;
        Ok(semio_framework_value::RetirementDemand { capacity_bytes, depth: 1, ..Default::default() })
    }
    /// 🎟️ True only after every original ticket page and identity backing is retained.
    pub fn admission_is_complete(&self) -> bool {
        self.slots.len() == ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY && self.generations.len() == ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY && self.occupied.len() == ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY && self.next.len() == ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY && self.previous.len() == ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY && self.buckets.len() == ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY && self.free.len() == ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY
    }
    /// 📥 Admits one physical backing, retaining all previous pages on refusal.
    pub fn admit_next(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        if self.admission_is_complete() { return Ok(Default::default()); }
        if self.slots.len() != ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY { return self.slots.admit_next(grant); }
        let demand = self.admission_demands()?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "message index birth requires original depth")); }
        if grant.maximum_items == 0 || grant.maximum_capacity_bytes < demand.capacity_bytes { return Ok(Default::default()); }
        if self.generations.is_empty() { self.generations = vec![0; ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY].into_boxed_slice(); }
        else if self.occupied.is_empty() { self.occupied = vec![false; ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY].into_boxed_slice(); }
        else if self.next.is_empty() { self.next = vec![ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE; ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY].into_boxed_slice(); }
        else if self.previous.is_empty() { self.previous = vec![ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE; ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY].into_boxed_slice(); }
        else if self.buckets.is_empty() { self.buckets = vec![ARTIFACT_EDIT_MESSAGE_INDEX_EMPTY; ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY].into_boxed_slice(); }
        else { self.free = (0..ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY).rev().map(|slot| slot as u16).collect::<Vec<_>>().into_boxed_slice(); self.free_len = ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY; }
        let progress = RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: demand.capacity_bytes, ..Default::default() };
        semio_framework_value::retained_clone::admit_retained_clone_progress(grant, progress, "original message identity backing birth")?;
        Ok(progress)
    }

    pub fn new() -> Self {
        Self {
            slots: ArtifactEditMessageSlots::new(),
            generations: vec![0; ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY].into_boxed_slice(),
            occupied: vec![false; ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY].into_boxed_slice(),
            next: vec![ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE; ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY].into_boxed_slice(),
            previous: vec![ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE; ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY].into_boxed_slice(),
            buckets: vec![ARTIFACT_EDIT_MESSAGE_INDEX_EMPTY; ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY].into_boxed_slice(),
            free: (0..ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY).rev().map(|slot| slot as u16).collect::<Vec<_>>().into_boxed_slice(),
            free_len: ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY,
            head: ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE,
            tail: ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE,
            len: 0,
        }
    }

    pub fn try_from_entries(entries: Vec<crate::os_spr::EditMessages>) -> Result<Self, ArtifactEditMessageLedgerRejected> {
        if let Err(reason) = Self::preflight_entries(&entries) {
            return Err(ArtifactEditMessageLedgerRejected::new(entries, reason));
        }
        let mut ledger = Self::new();
        for entry in entries {
            ledger.push_reserved(entry).expect("preflighted artifact edit-message ledger slot remains admissible");
        }
        Ok(ledger)
    }

    pub fn from_preflighted_entries(entries: Vec<crate::os_spr::EditMessages>) -> Self {
        assert!(Self::preflight_entries(&entries).is_ok(), "preflighted artifact edit-message entries changed before fixed-ledger adoption");
        let mut ledger = Self::new();
        for entry in entries {
            ledger.push_reserved(entry).unwrap_or_else(|_| panic!("preflighted artifact edit-message slot remains admissible"));
        }
        ledger
    }

    fn preflight_entries(entries: &[crate::os_spr::EditMessages]) -> Result<(), &'static str> {
        if entries.len() > ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY {
            return Err("artifact edit-message ledger exceeds its exact fixed item authority");
        }
        let mut preflight_buckets = vec![ARTIFACT_EDIT_MESSAGE_INDEX_EMPTY; ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY].into_boxed_slice();
        let mut admitted_bytes = 0usize;
        for (entry_index, entry) in entries.iter().enumerate() {
            let Some(entry_bytes) = Self::entry_bytes(entry) else {
                return Err("artifact edit-message ledger byte authority overflowed");
            };
            if entry.edit_id.len() > ARTIFACT_EDIT_MESSAGE_ID_BYTES || entry_bytes > ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES {
                return Err("artifact edit-message ledger contains an entry outside its fixed byte authority");
            }
            admitted_bytes = match admitted_bytes.checked_add(entry_bytes) {
                Some(bytes) if bytes <= ARTIFACT_EDIT_MESSAGE_LEDGER_BYTES => bytes,
                _ => return Err("artifact edit-message ledger exceeds its exact aggregate byte authority"),
            };
            let start = Self::hash(&entry.edit_id) % ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY;
            let mut admitted = false;
            for probe in 0..ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY {
                let bucket = (start + probe) % ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY;
                let established = preflight_buckets[bucket];
                if established == ARTIFACT_EDIT_MESSAGE_INDEX_EMPTY {
                    preflight_buckets[bucket] = entry_index as u16;
                    admitted = true;
                    break;
                }
                if entries[usize::from(established)].edit_id == entry.edit_id {
                    return Err("artifact edit-message ledger contains a duplicate identity");
                }
            }
            if !admitted {
                return Err("artifact edit-message ledger fixed identity table is saturated");
            }
        }
        Ok(())
    }

    fn entry_bytes(entry: &crate::os_spr::EditMessages) -> Option<usize> {
        entry.messages.iter().try_fold(entry.edit_id.len(), |bytes, message| message.target.iter().try_fold(bytes.checked_add(ARTIFACT_EDIT_MESSAGE_OWNER_BYTES)?.checked_add(message.code.0.len())?.checked_add(message.message.len())?, |bytes, segment| bytes.checked_add(ARTIFACT_EDIT_MESSAGE_TARGET_BYTES)?.checked_add(segment.len())))
    }

    pub(crate) fn hash(id: &str) -> usize {
        id.as_bytes().iter().fold(0xcbf29ce484222325u64, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)) as usize
    }

    pub(crate) fn find_ticket(&self, id: &str) -> Option<ArtifactEditMessageTicket> {
        if !self.admission_is_complete() || id.len() > ARTIFACT_EDIT_MESSAGE_ID_BYTES {
            return None;
        }
        let start = Self::hash(id) % ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY;
        for probe in 0..ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY {
            let slot = self.buckets[(start + probe) % ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY];
            if slot == ARTIFACT_EDIT_MESSAGE_INDEX_EMPTY {
                return None;
            }
            if slot == ARTIFACT_EDIT_MESSAGE_INDEX_TOMBSTONE {
                continue;
            }
            let slot = usize::from(slot);
            if self.occupied[slot] && unsafe { self.slots[slot].assume_init_ref() }.edit_id == id {
                return Some(ArtifactEditMessageTicket { slot: slot as u16, generation: self.generations[slot] });
            }
        }
        None
    }

    fn reserve_identity(&mut self, slot: u16, id: &str) {
        let start = Self::hash(id) % ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY;
        let mut tombstone = None;
        for probe in 0..ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY {
            let bucket = (start + probe) % ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY;
            match self.buckets[bucket] {
                ARTIFACT_EDIT_MESSAGE_INDEX_EMPTY => {
                    self.buckets[tombstone.unwrap_or(bucket)] = slot;
                    return;
                }
                ARTIFACT_EDIT_MESSAGE_INDEX_TOMBSTONE => {
                    tombstone.get_or_insert(bucket);
                }
                _ => {}
            }
        }
        self.buckets[tombstone.expect("preflighted artifact edit-message identity has a deterministic bucket")] = slot;
    }

    fn reserved_identity_bucket(&self, slot: u16, id: &str) -> Option<usize> {
        let start = Self::hash(id) % ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY;
        for probe in 0..ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY {
            let bucket = (start + probe) % ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY;
            if self.buckets[bucket] == slot {
                return Some(bucket);
            }
            if self.buckets[bucket] == ARTIFACT_EDIT_MESSAGE_INDEX_EMPTY {
                return None;
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> ArtifactEditMessageLedgerIter<'_> {
        ArtifactEditMessageLedgerIter { ledger: self, next: self.head }
    }

    pub fn first(&self) -> Option<&crate::os_spr::EditMessages> {
        (self.head != ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE).then(|| unsafe { self.slots[usize::from(self.head)].assume_init_ref() })
    }

    pub fn last(&self) -> Option<&crate::os_spr::EditMessages> {
        (self.tail != ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE).then(|| unsafe { self.slots[usize::from(self.tail)].assume_init_ref() })
    }

    pub fn get_by_id(&self, id: &str) -> Option<&crate::os_spr::EditMessages> {
        self.find_ticket(id).and_then(|ticket| self.get(ticket))
    }

    pub fn get_mut_by_id(&mut self, id: &str) -> Option<&mut crate::os_spr::EditMessages> {
        let ticket = self.find_ticket(id)?;
        self.get_mut(ticket)
    }

    pub(crate) fn get(&self, ticket: ArtifactEditMessageTicket) -> Option<&crate::os_spr::EditMessages> {
        let slot = usize::from(ticket.slot);
        (slot < ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY && self.occupied[slot] && self.generations[slot] == ticket.generation).then(|| unsafe { self.slots[slot].assume_init_ref() })
    }

    fn get_mut(&mut self, ticket: ArtifactEditMessageTicket) -> Option<&mut crate::os_spr::EditMessages> {
        let slot = usize::from(ticket.slot);
        (slot < ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY && self.occupied[slot] && self.generations[slot] == ticket.generation).then(|| unsafe { self.slots[slot].assume_init_mut() })
    }

    pub(crate) fn try_push(&mut self, entry: crate::os_spr::EditMessages) -> Result<ArtifactEditMessageTicket, crate::os_spr::EditMessages> {
        if !self.admission_is_complete() || self.free_len == 0 || entry.edit_id.len() > ARTIFACT_EDIT_MESSAGE_ID_BYTES || Self::entry_bytes(&entry).is_none_or(|bytes| bytes > ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES) || self.find_ticket(&entry.edit_id).is_some() {
            return Err(entry);
        }
        self.push_reserved(entry)
    }

    pub fn admit(&mut self, entry: crate::os_spr::EditMessages) -> Result<(), crate::os_spr::EditMessages> {
        self.try_push(entry).map(|_| ())
    }

    fn push_reserved(&mut self, entry: crate::os_spr::EditMessages) -> Result<ArtifactEditMessageTicket, crate::os_spr::EditMessages> {
        assert!(self.free_len > 0, "artifact edit-message ledger push lacked its exact fixed-slot admission");
        self.free_len -= 1;
        let slot = usize::from(self.free[self.free_len]);
        let Some(generation) = self.generations[slot].checked_add(1) else {
            self.free[self.free_len] = slot as u16;
            self.free_len += 1;
            return Err(entry);
        };
        self.reserve_identity(slot as u16, &entry.edit_id);
        self.slots[slot].write(entry);
        self.generations[slot] = generation;
        self.occupied[slot] = true;
        self.previous[slot] = self.tail;
        self.next[slot] = ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE;
        if self.tail == ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE {
            self.head = slot as u16;
        } else {
            self.next[usize::from(self.tail)] = slot as u16;
        }
        self.tail = slot as u16;
        self.len += 1;
        Ok(ArtifactEditMessageTicket { slot: slot as u16, generation: self.generations[slot] })
    }

    pub fn pop(&mut self) -> Option<crate::os_spr::EditMessages> {
        (self.tail != ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE).then(|| {
            let slot = usize::from(self.tail);
            self.remove(ArtifactEditMessageTicket { slot: slot as u16, generation: self.generations[slot] }).expect("validated artifact edit-message tail remains live")
        })
    }

    pub(crate) fn remove(&mut self, ticket: ArtifactEditMessageTicket) -> Option<crate::os_spr::EditMessages> {
        self.get(ticket)?;
        let slot = usize::from(ticket.slot);
        let identity_bucket = self.reserved_identity_bucket(ticket.slot, &unsafe { self.slots[slot].assume_init_ref() }.edit_id).expect("artifact edit-message identity remains reserved while its slot is live");
        let previous = self.previous[slot];
        let next = self.next[slot];
        if previous == ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE {
            self.head = next;
        } else {
            self.next[usize::from(previous)] = next;
        }
        if next == ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE {
            self.tail = previous;
        } else {
            self.previous[usize::from(next)] = previous;
        }
        self.occupied[slot] = false;
        self.buckets[identity_bucket] = ARTIFACT_EDIT_MESSAGE_INDEX_TOMBSTONE;
        self.next[slot] = ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE;
        self.previous[slot] = ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE;
        self.free[self.free_len] = ticket.slot;
        self.free_len += 1;
        self.len -= 1;
        Some(unsafe { self.slots[slot].assume_init_read() })
    }

    pub fn remove_id(&mut self, id: &str) -> Option<crate::os_spr::EditMessages> {
        let ticket = self.find_ticket(id)?;
        self.remove(ticket)
    }

    pub(crate) fn backing_is_empty(&self) -> bool { self.slots.is_empty() && self.generations.is_empty() && self.occupied.is_empty() && self.next.is_empty() && self.previous.is_empty() && self.buckets.is_empty() && self.free.is_empty() }
    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.len == 0 && (self.free_len == ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY || (self.free_len == 0 && self.backing_is_empty()))
    }
}

impl Default for ArtifactEditMessageLedger {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoIterator for &'a ArtifactEditMessageLedger {
    type Item = &'a crate::os_spr::EditMessages;
    type IntoIter = ArtifactEditMessageLedgerIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl std::fmt::Debug for ArtifactEditMessageLedger {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_list().entries(self.iter()).finish()
    }
}

impl PartialEq for ArtifactEditMessageLedger {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.iter().zip(other.iter()).all(|(left, right)| left == right)
    }
}

impl Eq for ArtifactEditMessageLedger {}

impl ToValue for ArtifactEditMessageLedger {
    fn to_value(&self) -> DslValue {
        DslValue::Array(self.iter().map(ToValue::to_value).collect())
    }
}

impl ArtifactEditMessageLedgerRejected {
    /// 🔎️ Keeps the canonical rejected row owner visible through an immutable borrow.
    pub fn retained_entries(&self) -> &[crate::os_spr::EditMessages] { self.inner.retained_entries() }
    fn new(entries: Vec<crate::os_spr::EditMessages>, reason: &'static str) -> Self {
        Self { inner: crate::os_spr::command::EditMessageLedgerRetirement::new(entries), reason }
    }
}

impl ErasedSnapshotRetirement for ArtifactEditMessageLedgerRejected {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.inner.close_step(grant) }
    fn terminal_is_empty(&self) -> bool { self.inner.terminal_is_empty() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.inner.next_copy_byte_demand() }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { self.inner.next_capacity_byte_demand(body) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.inner.next_release_byte_demand() }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { self.inner.next_depth_demand() }
}

impl Drop for ArtifactEditMessageLedgerRejected {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal_is_empty()), "rejected artifact edit-message authority reached Drop before every exact payload owner was cursor-retired: {}", self.reason);
    }
}

impl Drop for ArtifactEditMessageLedger {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal_is_empty()), "artifact edit-message fixed ledger reached Drop before every exact payload owner was cursor-retired");
    }
}

pub(crate) struct ArtifactStoreHistoryCommitReservation {
    pub(crate) history: ArtifactHistoryReservation,
    pub(crate) rejected_owner: ArtifactStoreDisplacedOwnerReservation,
}

/// 📨️ See `ArtifactStore::pending_report`'s doc.
#[derive(Default)]
pub(crate) struct PendingCommandReport {
    /// 🎯️ Overrides `dispatch`'s tail-diff `edit_ids` guess — needed once a merge can insert
    /// mid-history (`ingest_remote`/`resolve_conflict`), where "everything past `before`" no longer
    /// names only the newly-landed edits.
    pub(crate) edit_ids: Option<Vec<String>>,
    pub(crate) messages: Vec<crate::os_spr::EditMessages>,
    pub(crate) worst: Option<semio_framework_diagnostic::Severity>,
    /// 📤️ Every event this command authored (edit operations, history transitions), in authoring
    /// order — announced outbound as one `BackboneMessage::Mutations` batch when the command ends.
    pub(crate) outbound: Vec<crate::os_spr::MutationEnvelope>,
}

pub(crate) struct ArtifactStorePendingReportRetirement {
    report: std::mem::ManuallyDrop<Option<PendingCommandReport>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
}

impl ArtifactStorePendingReportRetirement {
    pub(crate) fn new(report: PendingCommandReport) -> Self { Self { report: std::mem::ManuallyDrop::new(Some(report)), active: std::mem::ManuallyDrop::new(None) } }
    fn demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        if let Some(active) = self.active.as_ref() { return artifact_retirement_box_demands(active, body); }
        let Some(report) = self.report.as_ref() else { return Ok(Default::default()); };
        let capacity_bytes = if !report.outbound.is_empty() { size_of::<ArtifactStoreConflictRetirement>() }
        else if !report.messages.is_empty() { size_of::<ArtifactStoreMessageLedgerRetirement>() }
        else if report.edit_ids.as_ref().is_some_and(|ids| !ids.is_empty()) { size_of::<ArtifactStoreStringRetirement>() } else { 0 };
        if capacity_bytes != 0 { return Ok(RetirementDemand { capacity_bytes, depth: 2, ..Default::default() }); }
        let release_bytes = if report.outbound.capacity() != 0 { std::alloc::Layout::array::<crate::os_spr::MutationEnvelope>(report.outbound.capacity()) }
        else if report.messages.capacity() != 0 { std::alloc::Layout::array::<crate::os_spr::EditMessages>(report.messages.capacity()) }
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

pub(crate) async fn validate_composition_pins(pins: &[crate::os_vcs::CompositionPin]) -> Result<(), VcsError> {
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
pub async fn edit_from_operation_envelope<Mutation: OpBinary>(envelope: &crate::os_spr::MutationEnvelope) -> Result<Edit<Mutation>, VcsError> {
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

/// 🧊️ Retires one SCRATCH projection a history fold displaced, through the technology's own diff
/// vocabulary. Every replay seam below walks `base → mid₁ → … → head` and throws each intermediate
/// away; an artifact whose projection owns a fail-closed root (an `OrderedMap`, a neural
/// `Dictionary`) aborts the process on that bare drop, which is what made `undo`/`redo` and every
/// `.pack`/`.spr` reload kill the app (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub(crate) fn retire_replayed_projection<P, Mutation>(projection: P)
where
    Mutation: self::Mutation<P>,
{
    <Mutation::Diff as MutationDiff<P>>::retire_projection(projection);
}

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
pub(crate) struct ReplayProjection<P, Mutation>
where
    Mutation: self::Mutation<P>,
{
    projection: Option<P>,
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
    fn advance(&mut self, next: P) {
        let displaced = self.projection.replace(next).expect("a live replay projection is present until it is handed out");
        retire_replayed_projection::<P, Mutation>(displaced);
    }

    /// 🎁️ Hands the finished projection to the caller, who owns its retirement from here on.
    pub(crate) fn into_inner(mut self) -> P {
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

/// 🧊️ Cold-retires scratch operations (a rebased or discarded inverse): an operation may own a
/// fail-closed root, so a displaced copy never reaches `Drop` owning it.
pub(crate) fn retire_scratch_operations<P, Mutation>(operations: impl IntoIterator<Item = Mutation>)
where
    Mutation: self::Mutation<P>,
{
    for operation in operations {
        self::Mutation::<P>::retire_cold(operation);
    }
}

/// 🫗️ Retires every typed replacement of a refused supersession's `inputs`, none of which was encoded.
pub(crate) fn retire_supersede_inputs<P, Mutation>(inputs: Vec<SupersedeInput<Mutation>>)
where
    Mutation: self::Mutation<P>,
{
    for replacement in inputs.into_iter().filter_map(|input| input.replacement) {
        self::Mutation::<P>::retire_cold(replacement);
    }
}

pub(crate) fn retire_scratch_edits<P, Mutation>(edits: impl IntoIterator<Item = Edit<Mutation>>)
where
    Mutation: self::Mutation<P>,
{
    for edit in edits {
        retire_scratch_operations::<P, Mutation>(edit.forwards);
        retire_scratch_operations::<P, Mutation>(edit.inverse);
    }
}

/// 🧮️ Folds an envelope's complete event log — its edits and its history transitions — into
/// the history projection (`crate::os_spr::fold_history`). Quarantined edits (every edit owning an
/// operation of a quarantined conflict that was not accepted) stay inactive.
pub fn fold_envelope_history<P, Mutation>(envelope: &ArtifactEnvelopeOwners<P, Mutation>) -> Result<crate::os_spr::HistoryFold, VcsError>
where
    Mutation: self::Mutation<P>,
{
    let head = envelope_viewer_head(envelope);
    fold_event_log::<P, Mutation>(&envelope.id, &envelope.vcs.edits.iter().collect::<Vec<_>>(), &envelope.transitions, &envelope.conflicts, &head)
}

/// 👁 The head [`fold_history_for`] projects: this replica's alternative and explicit checkpoint.
/// Absent alternative is the canonical trunk. `viewer_checkpoint_id` of `None` is that line's tip.
pub(crate) fn envelope_viewer_head<P, Mutation>(envelope: &ArtifactEnvelopeOwners<P, Mutation>) -> crate::os_spr::ViewerHead {
    let document_id = ArtifactId(envelope.id.clone());
    let trunk = crate::os_spr::trunk_alternative_id(&document_id);
    crate::os_spr::ViewerHead { line_id: envelope.active_alternative_id.clone().filter(|line| line != &trunk).unwrap_or(trunk), checkpoint_id: envelope.viewer_checkpoint_id.clone() }
}

/// 🧮️ [`fold_envelope_history`] over the bare event log: `edits`, `transitions` and the
/// `conflicts` whose quarantine withholds edits — for document shapes that hold the log without a
/// full envelope.
pub fn fold_event_log<P, Mutation>(document_id: &str, edits: &[&Edit<Mutation>], transitions: &[crate::os_spr::MutationEnvelope], conflicts: &[crate::os_spr::Conflict], head: &crate::os_spr::ViewerHead) -> Result<crate::os_spr::HistoryFold, VcsError>
where
    Mutation: self::Mutation<P>,
{
    let mut folded = Vec::with_capacity(edits.len());
    let mut owners: HashMap<String, String> = HashMap::new();
    for edit in edits.iter().copied() {
        let mutation_ids = crate::os_spr::mutation_ids_for_edit::<P, Mutation>(edit);
        for mutation_id in &mutation_ids {
            owners.insert(mutation_id.0.clone(), edit.id.clone());
        }
        folded.push(crate::os_spr::FoldEdit { id: edit.id.clone(), actor: edit.actor.clone(), timestamp: edit_timestamp(edit), mutation_ids, line: edit.line.clone() });
    }
    let mut excluded = HashSet::new();
    for conflict in conflicts {
        if conflict.status == crate::os_spr::ConflictStatus::Accepted {
            continue;
        }
        if let crate::os_spr::ConflictKind::Quarantined { envelopes, .. } = &conflict.kind {
            excluded.extend(envelopes.iter().filter_map(|quarantined| owners.get(&quarantined.mutation_id.0).cloned()));
        }
    }
    crate::os_spr::fold_history_for(&ArtifactId(document_id.to_string()), &folded, transitions, &excluded, head).map_err(|error| VcsError::ValidationFailed(error.to_string()))
}

/// 🗣️ `<document schema>#<semantic kind>` of the leaf `operation` is — what `MutationMeta.semantic_kind` records.
pub(crate) fn operation_semantic_kind<P, Mutation: self::Mutation<P>>(operation: &Mutation, schema: &str) -> SchemaId {
    SchemaId(format!("{schema}#{}", operation.descriptor().semantic_kind))
}

/// 🗣️ Stamps every metadata record of `edit` with its operation's semantic kind and descriptor name — derived facts no
/// persisted form carries, so every load and ingest path re-derives them.
pub(crate) fn stamp_edit_semantics<P, Mutation: self::Mutation<P>>(edit: &mut Edit<Mutation>, schema: &str) {
    for (meta, operation) in edit.mutation_meta.iter_mut().zip(edit.forwards.iter()) {
        meta.semantic_kind = Some(operation_semantic_kind::<P, Mutation>(operation, schema));
        meta.label = Some(operation.descriptor().display_name.to_string());
    }
}

/// ⏰️ An edit's HLC: its first operation's stamped timestamp (the order key every fold uses).
pub(crate) fn edit_timestamp<Mutation>(edit: &Edit<Mutation>) -> HybridLogicalTimestamp {
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
