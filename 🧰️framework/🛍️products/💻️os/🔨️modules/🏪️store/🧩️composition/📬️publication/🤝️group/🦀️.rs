//! 🤝️ Retains one private member root until its shared visibility owner decides.

use super::*;

pub(super) struct MemberGroupPreparation {
    visibility: Arc<crate::os_vcs::ArtifactGroupVisibility>,
    pub(super) history: Option<ArtifactStoreHistoryCommitReservation>,
    pub(super) displaced: Option<ArtifactStoreDisplacedOwnerReservation>,
    pub(super) applied: crate::os_vcs::HistoryPageStack<String>,
    pub(super) cursor: crate::os_vcs::HistoryPageStack<String>,
    pub(super) revision: Option<CursorRevisionAccumulator>,
    pub(super) checkpoint: Option<String>,
    phase: u8,
    ordinal: usize,
    operation: usize,
    demand: usize,
    completed: u32,
}

impl MemberGroupPreparation {
    pub(super) fn new(visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, history: Option<ArtifactStoreHistoryCommitReservation>, displaced: Option<ArtifactStoreDisplacedOwnerReservation>) -> Self {
        Self { visibility: Arc::clone(visibility), history, displaced, applied: crate::os_vcs::HistoryPageStack::empty(), cursor: crate::os_vcs::HistoryPageStack::empty(), revision: None, checkpoint: None, phase: 0, ordinal: 0, operation: 0, demand: 1, completed: 0 }
    }

    pub(super) fn next_byte_demand(&self) -> usize { self.demand }

    pub(super) fn visibility(&self) -> Arc<crate::os_vcs::ArtifactGroupVisibility> { Arc::clone(&self.visibility) }

    pub(super) fn completed_items(&self) -> u32 { self.completed }
}

fn retain_group_owner(retirements: &mut ArtifactStoreDisplacedRetirements, reservation: &mut ArtifactStoreDisplacedOwnerReservation, owner: Box<dyn ErasedSnapshotRetirement>) {
    if let Err(owner) = retirements.push_owner_reserved(reservation, owner) { retirements.push_reserved(owner); }
}

impl<P, Mu> ArtifactStore<P, Mu>
where
    P: Clone + ToValue + FromValue + ArtifactPack + Send + Sync + 'static,
    Mu: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    pub(super) fn stage_apply_batch_group(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mu>, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        let group = publication.group_preparation.as_mut().ok_or("member group lacks its retained preparation")?;
        if !Arc::ptr_eq(&group.visibility, visibility) || !visibility.pending() { return Err("member group visibility authority does not match its pending decision".into()); }
        if self.generation != publication.expected_generation || self.content_revision != publication.expected_revision { return Err("member group became stale before its common decision".into()); }
        if group.phase == 12 { return Ok(ArtifactStoreOneItemPreparationStep::Prepared(publication.progress())); }
        if !grant.permits_one() { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
        let stage = publication.stage.as_ref().ok_or("member group lost its staged batch")?;
        if publication.phase != ArtifactStoreOneItemPublicationPhase::Publishing || stage.post.is_none() || self.durable_group_root.is_some() { return Err("member group requires an exact private publication root".into()); }
        if publication.close_started || publication.cancel_requested { return Err("member group candidate was cancelled".into()); }
        if group.displaced.is_none() {
            group.demand = 4096;
            if grant.maximum_bytes < group.demand { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
            group.displaced = Some(self.displaced_retirements.reserve_owner_slots(12).map_err(|error| error.to_string())?);
            group.completed = group.completed.saturating_add(1);
            return Ok(ArtifactStoreOneItemPreparationStep::Progress(publication.progress()));
        }
        if group.history.is_none() {
            group.demand = self.envelope.vcs.edits.next_reservation_allocation_bytes().max(4096);
            if grant.maximum_bytes < group.demand { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
            group.history = Some(self.reserve_edit_history_slot().map_err(|error| error.to_string())?);
            group.completed = group.completed.saturating_add(1);
            return Ok(ArtifactStoreOneItemPreparationStep::Progress(publication.progress()));
        }
        if group.revision.is_none() {
            group.revision = Some(CursorRevisionAccumulator { identity_digest: self.revision_accumulator.identity_digest, applied: crate::os_vcs::HistoryPageStack::empty(), redo: crate::os_vcs::HistoryPageStack::empty(), applied_tail_chains: None, mutation_positions: BTreeMap::new(), indexed_edits: BTreeMap::new(), unit_flags: BTreeMap::new() });
        }
        if group.ordinal < self.applied_edit_ids.len() {
            let source = &self.applied_edit_ids[group.ordinal];
            group.demand = match group.phase { 0 => group.applied.next_push_allocation_bytes().saturating_add(source.len()), 1 => group.cursor.next_push_allocation_bytes().saturating_add(source.len()), _ => group.revision.as_ref().unwrap().applied.next_push_allocation_bytes().saturating_add(std::mem::size_of::<CursorRevisionRecord>()) }.max(1);
            if grant.maximum_bytes < group.demand { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
            match group.phase {
                0 => { group.applied.try_push(source.clone()).map_err(|_| "member applied catalog is exhausted")?; group.phase = 1; }
                1 => { group.cursor.try_push(source.clone()).map_err(|_| "member cursor catalog is exhausted")?; group.phase = 2; }
                _ => { group.revision.as_mut().unwrap().applied.try_push(self.revision_accumulator.applied[group.ordinal].clone()).map_err(|_| "member revision catalog is exhausted")?; group.ordinal += 1; group.phase = 0; }
            }
            group.completed = group.completed.saturating_add(1);
            return Ok(ArtifactStoreOneItemPreparationStep::Progress(publication.progress()));
        }
        if group.phase < 3 { group.phase = 3; }
        group.demand = match group.phase {
            3 => group.applied.next_push_allocation_bytes().saturating_add(stage.edit.id.len()),
            4 => group.cursor.next_push_allocation_bytes().saturating_add(stage.edit.id.len()),
            5 => group.revision.as_ref().unwrap().applied.next_push_allocation_bytes().saturating_add(std::mem::size_of::<CursorRevisionRecord>()),
            6 => self.current_checkpoint_id.as_ref().map_or(1, String::len),
            7 => self.revision_accumulator.mutation_positions.keys().next().map_or(1, |id| id.0.len().saturating_add(4096)),
            8..=10 => ARTIFACT_STORE_ONE_ITEM_ID_BYTES.saturating_add(4096),
            _ => std::mem::size_of::<ArtifactStoreBatchStage<P, Mu>>().saturating_add(stage.unit_flags.capacity().saturating_mul(std::mem::size_of::<(usize, bool)>())).saturating_add(std::mem::size_of::<durable_group::StagedRootRetirement<P>>()).saturating_add(2048),
        }.max(1);
        if grant.maximum_bytes < group.demand { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
        match group.phase {
            3 => { group.applied.try_push(stage.edit.id.clone()).map_err(|_| "member applied catalog is exhausted")?; group.phase = 4; }
            4 => { group.cursor.try_push(stage.edit.id.clone()).map_err(|_| "member cursor catalog is exhausted")?; group.phase = 5; }
            5 => {
                let revision = group.revision.as_mut().unwrap();
                let previous = revision.applied.last().map_or(revision.identity_digest, |record| record.prefix_digest);
                revision.applied.try_push(CursorRevisionRecord { ledger_key: None, id_digest: CursorRevisionAccumulator::hash_record(b"edit-id", &[stage.edit.id.as_bytes()]), edit_digest: stage.digest, prefix_digest: CursorRevisionAccumulator::hash_record(b"applied", &[&previous, &stage.digest]) }).map_err(|_| "member revision catalog is exhausted")?;
                group.phase = 6;
            }
            6 => { group.checkpoint = (*self.current_checkpoint_id).clone(); group.phase = 7; }
            7 => {
                use std::ops::Bound::{Excluded, Unbounded};
                let revision = group.revision.as_mut().unwrap();
                let next = match revision.mutation_positions.last_key_value() {
                    Some((key, _)) => self.revision_accumulator.mutation_positions.range::<MutationId, _>((Excluded(key), Unbounded)).next(),
                    None => self.revision_accumulator.mutation_positions.first_key_value(),
                };
                if let Some((key, value)) = next { revision.mutation_positions.insert(key.clone(), *value); }
                else { group.phase = 8; }
            }
            8 => {
                use std::ops::Bound::{Excluded, Unbounded};
                let revision = group.revision.as_mut().unwrap();
                let next = match revision.indexed_edits.last_key_value() {
                    Some((key, _)) => self.revision_accumulator.indexed_edits.range::<[u8; 32], _>((Excluded(key), Unbounded)).next(),
                    None => self.revision_accumulator.indexed_edits.first_key_value(),
                };
                if let Some((key, value)) = next { revision.indexed_edits.insert(*key, *value); }
                else { group.phase = 9; }
            }
            9 => {
                use std::ops::Bound::{Excluded, Unbounded};
                let revision = group.revision.as_mut().unwrap();
                let next = match revision.unit_flags.last_key_value() {
                    Some((key, _)) => self.revision_accumulator.unit_flags.range::<([u8; 32], usize), _>((Excluded(key), Unbounded)).next(),
                    None => self.revision_accumulator.unit_flags.first_key_value(),
                };
                if let Some((key, value)) = next { revision.unit_flags.insert(*key, *value); }
                else { group.phase = 10; }
            }
            10 if group.operation < stage.edit.forwards.len() => {
                let index = group.operation;
                let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[stage.edit.id.as_bytes()]);
                let id = crate::os_spr::mutation_id_for_edit_operation::<P, Mu>(&stage.edit, index).ok_or("member operation identity is absent")?;
                let revision = group.revision.as_mut().unwrap();
                revision.mutation_positions.insert(id, (group.applied.len() - 1, index, digest));
                revision.indexed_edits.insert(digest, (group.applied.len() - 1, stage.edit.forwards.len()));
                group.operation += 1;
            }
            10 => {
                if let Some((index, flag)) = publication.stage.as_mut().unwrap().unit_flags.pop() {
                    let stage = publication.stage.as_ref().unwrap();
                    let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[stage.edit.id.as_bytes()]);
                    group.revision.as_mut().unwrap().unit_flags.insert((digest, index), flag);
                } else { group.phase = 11; }
            }
            _ => {
                durable_group::stage_prebuilt_batch_root(self, publication)?;
                let group = publication.group_preparation.as_mut().unwrap();
                group.phase = 12;
                group.demand = 4096;
                group.completed = group.completed.saturating_add(1);
                return Ok(ArtifactStoreOneItemPreparationStep::Prepared(publication.progress()));
            }
        }
        publication.group_preparation.as_mut().unwrap().completed = publication.group_preparation.as_ref().unwrap().completed.saturating_add(1);
        Ok(ArtifactStoreOneItemPreparationStep::Progress(publication.progress()))
    }

    pub(super) fn adopt_apply_batch_group(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mu>, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        if publication.published { return Ok(ArtifactStoreOneItemPreparationStep::Prepared(publication.progress())); }
        let group = publication.group_preparation.as_ref().ok_or("member group lacks its retained adoption")?;
        if !Arc::ptr_eq(&group.visibility, visibility) || !visibility.committed() || group.phase != 12 { return Err("member adoption requires its exact committed common decision".into()); }
        if !grant.permits_one() || grant.maximum_bytes < 4096 { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
        let checkpoint = publication.progress();
        durable_group::adopt_staged_store_member(self, visibility).map_err(|error| error.to_string())?;
        *self.durable_group_root = None;
        publication.group_preparation = None;
        publication.retained_checkpoint = checkpoint;
        publication.receipt = Some(LaneItemReceipt { generation_before: publication.expected_generation, generation_after: self.generation });
        publication.expected_generation = self.generation;
        publication.expected_revision = self.content_revision;
        publication.published = true;
        publication.phase = ArtifactStoreOneItemPublicationPhase::AwaitingAck;
        Ok(ArtifactStoreOneItemPreparationStep::Prepared(publication.progress()))
    }

    pub(super) fn abort_apply_batch_group(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mu>, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, ValueError> {
        let error = |message: &str| ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, message);
        if !grant.permits_one() { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        let Some(group) = publication.group_preparation.as_mut() else { return Ok(SnapshotRetirementStep::Complete) };
        if group.visibility.committed() || group.visibility.pending() { return Err(error("member abort requires its exact aborted common decision")); }
        group.demand = 4096;
        if grant.maximum_bytes < group.demand { return Ok(SnapshotRetirementStep::Blocked); }
        if self.durable_group_root.is_some() {
            durable_group::abort_staged_store_member(self, &group.visibility).map_err(|failure| error(&failure.to_string()))?;
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(history) = group.history.take() {
            let ArtifactStoreHistoryCommitReservation { history, rejected_owner } = history;
            if let Err(history) = self.envelope.vcs.edits.cancel_reservation(history) { group.history = Some(ArtifactStoreHistoryCommitReservation { history, rejected_owner }); return Err(error("member abort lost its exact ledger reservation")); }
            self.displaced_retirements.release_owner_slots(rejected_owner).map_err(|failure| error(&failure.to_string()))?;
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(reservation) = group.displaced.as_mut() {
            if group.applied.capacity() != 0 { retain_group_owner(&mut self.displaced_retirements, reservation, Box::new(ArtifactStoreStringVectorRetirement::new(std::mem::replace(&mut group.applied, crate::os_vcs::HistoryPageStack::empty())))); }
            else if group.cursor.capacity() != 0 { retain_group_owner(&mut self.displaced_retirements, reservation, Box::new(ArtifactStoreStringVectorRetirement::new(std::mem::replace(&mut group.cursor, crate::os_vcs::HistoryPageStack::empty())))); }
            else if let Some(revision) = group.revision.take() { retain_group_owner(&mut self.displaced_retirements, reservation, Box::new(ArtifactStoreRevisionAccumulatorRetirement::new(revision))); }
            else if let Some(checkpoint) = group.checkpoint.take() { retain_group_owner(&mut self.displaced_retirements, reservation, Box::new(ArtifactStoreStringRetirement::new(checkpoint))); }
            else { self.displaced_retirements.release_owner_slots(group.displaced.take().unwrap()).map_err(|failure| error(&failure.to_string()))?; }
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        publication.begin_close();
        publication.group_preparation = None;
        Ok(SnapshotRetirementStep::Complete)
    }
}
