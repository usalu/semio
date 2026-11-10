//! 🧯️ The close witnesses and cursor admission of one store over its component bundle.

use semio_os_spr::{OpBinary, OpText};
use semio_os_store_engine_live::lifted_store::{StoreParts, StorePartsRef};
use semio_os_store_ledger::{ArtifactStorePendingReportRetirement, CursorRevisionAccumulator, PendingCommandReport};
use semio_os_store_read::{ArtifactStoreCursorDisposerPhase, ArtifactStoreMutationDagRetirement, ArtifactStoreRevisionAccumulatorRetirement, ArtifactStoreStringRetirement, ArtifactStoreStringVectorRetirement, ArtifactStoreSupersessionRetirement, retire_document_envelope};
use semio_os_store_replay::{EffectiveSupersessions, PrefixSnapshot};
use semio_framework_value::factory::ArcRootRetirement as ReturnedSnapshotReadRetirement;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
use semio_framework_value::{ErasedSnapshotRetirement, FromValue, ToValue, ValueError};
use std::sync::Arc;

/// 🌳️ Admits one resident snapshot root and its snapshot issuer as a returned retirement frame through the caller's effect.
fn admit_snapshot_root<P: Send + Sync + 'static>(factory: &Arc<dyn semio_framework_value::SnapshotRetirementFactory<P>>, snapshot: &mut Option<Arc<P>>, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> semio_framework_value::OriginalRetirementPoll<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress)> {
    let mut issuer = Some(Arc::clone(factory));
    let mut output = None;
    match semio_framework_value::factory::ArcRootRetirement::admit_snapshot(snapshot, &mut issuer, &mut output, effect) {
        semio_framework_value::RetainedWorkPoll::Pending { work } => semio_framework_value::RetainedWorkPoll::Pending { work },
        semio_framework_value::RetainedWorkPoll::Complete(turn) => semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome {
            result: turn.result.and_then(|step| output.map(|owner| (owner, step.progress())).ok_or_else(|| semio_framework_value::OriginalRetirementFailure::from(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "snapshot root admission was not funded")))),
            work: turn.work,
        }),
    }
}
/// 🏁️ Settles one snapshot-factory turn: a pending turn keeps its resident snapshot and retries on the next close turn.
fn settle_snapshot_factory(poll: semio_framework_value::OriginalRetirementPoll<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress)>) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), semio_framework_value::RetirementPollExit> {
    semio_framework_value::RetirementPollExit::resume(poll)
}
use super::{StoreCloseAdmission, StoreCloseWitness};

//#region 🔖️ArtifactStore
/// 🧯️ Admits the non-snapshot owned roots of one store; the snapshot phases carry their effect in [`StoreCloseAdmission::close_cursor_admit`].
trait StoreCloseOwned {
    fn close_cursor_admit_owned(&mut self, phase: ArtifactStoreCursorDisposerPhase, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError>;
}

impl<P, Mutation> StoreCloseOwned for StoreParts<'_, P, Mutation>
where
    P: semio_os_spr::StoreState,
    Mutation: semio_os_spr::StoreMutationFor<P>,
{
    fn close_cursor_admit_owned(&mut self, phase: ArtifactStoreCursorDisposerPhase, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> {
        let structural = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        match phase {
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(0) => {
                let original = std::mem::replace(&mut *self.log.applied_edit_ids, semio_os_vcs::HistoryPageStack::empty());
                match admit_artifact_retirement(original, grant, ArtifactStoreStringVectorRetirement::new) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { *self.log.applied_edit_ids = original; Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(1) => {
                let original = std::mem::replace(&mut *self.log.redo_edit_ids, semio_os_vcs::HistoryPageStack::empty());
                match admit_artifact_retirement(original, grant, ArtifactStoreStringVectorRetirement::new) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { *self.log.redo_edit_ids = original; Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(2) => {
                let original = std::mem::replace(&mut *self.cursor.revision_accumulator, CursorRevisionAccumulator { identity_digest: [0; 32], applied: semio_os_vcs::HistoryPageStack::empty(), redo: semio_os_vcs::HistoryPageStack::empty(), applied_tail_chains: None, mutation_positions: protocol::HistoryFoldIndex::new(), indexed_edits: protocol::HistoryFoldIndex::new(), unit_flags: protocol::HistoryFoldIndex::new() });
                match admit_artifact_retirement(original, grant, ArtifactStoreRevisionAccumulatorRetirement::new) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { *self.cursor.revision_accumulator = original; Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(6) => {
                let original = std::mem::replace(&mut *self.live.supersessions, EffectiveSupersessions::default());
                match admit_artifact_retirement(original, grant, ArtifactStoreSupersessionRetirement::new) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { *self.live.supersessions = original; Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(7) => {
                let original = std::mem::replace(&mut *self.replay.pending_report, PendingCommandReport::default());
                match admit_artifact_retirement(original, grant, ArtifactStorePendingReportRetirement::new) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { *self.replay.pending_report = original; Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(3) => {
                let Some(original) = self.log.current_checkpoint_id.take() else { return Ok((None, structural)); };
                match admit_artifact_retirement(original, grant, ArtifactStoreStringRetirement::new) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { *self.log.current_checkpoint_id = Some(original); Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(4) => {
                let original = std::mem::take(&mut self.log.local_actor_id.0);
                match admit_artifact_retirement(original, grant, |original|semio_framework_value::retirement::controlled::ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("shared UTF8 declares controlled retirement"))) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { self.log.local_actor_id.0 = original; Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(5) => {
                let Some((edit_id, _)) = self.live.tail_undo_cache.as_mut() else { return Ok((None, structural)); };
                let original = std::mem::take(edit_id);
                match admit_artifact_retirement(original, grant, ArtifactStoreStringRetirement::new) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { *edit_id = original; Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::Backbone => self.close_take_backbone_retirement(grant),
            ArtifactStoreCursorDisposerPhase::Causal => {
                let original = std::mem::replace(&mut *self.log.dag, semio_os_spr::MutationDag::empty());
                match admit_artifact_retirement(original, grant, ArtifactStoreMutationDagRetirement::new) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { *self.log.dag = original; Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::Envelope => {
                if self.log.envelope_detached { return Ok((None, structural)); }
                if !self.live.current_detached { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "whole envelope requires detached current owner")); }
                let snapshots = self.owners.initial_snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "whole envelope requires initial factory"))?.clone();
                let mutations = self.owners.mutation_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "whole envelope requires mutation factory"))?.clone();
                let original = unsafe { std::mem::ManuallyDrop::take(&mut self.log.envelope) };
                match retire_document_envelope(original, snapshots, mutations, grant) {
                    Ok((owner, receipt)) => { self.log.envelope_detached = true; Ok((Some(owner), receipt)) }
                    Err((error, original, snapshots, mutations)) => { self.log.envelope = std::mem::ManuallyDrop::new(original); drop((snapshots, mutations)); Err(error) }
                }
            }
            _ => Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "installed close addressed a non-admission phase")),
        }
    }
}

impl<P, Mutation> StoreCloseAdmission for StoreParts<'_, P, Mutation>
where
    P: semio_os_spr::StoreState,
    Mutation: semio_os_spr::StoreMutationFor<P>,
{
    fn close_take_backbone_retirement(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> {
        self.wire.close_take_backbone_retirement(grant)
    }

    fn close_cursor_admit(&mut self, phase: ArtifactStoreCursorDisposerPhase, grant: RetainedCloneGrant, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), semio_framework_value::RetirementPollExit> {
        let structural = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        match phase {
            ArtifactStoreCursorDisposerPhase::PrefixSnapshots => {
                if self.live.prefix_ring.is_empty() { return Ok((None, structural)); }
                let factory = self.owners.snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "prefix close requires installed factory"))?;
                let entry = self.live.prefix_ring.pop().expect("observed original prefix owner");
                if Arc::strong_count(&entry.snapshot) > 1 { drop(entry); return Ok((None, structural)); }
                let mut snapshot = Some(entry.snapshot);
                match settle_snapshot_factory(admit_snapshot_root(factory, &mut snapshot, effect)) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err(exit) => { self.live.prefix_ring.push(PrefixSnapshot { snapshot: snapshot.take().expect("a refused factory keeps its snapshot"), ..entry }); Err(exit) }
                }
            }
            ArtifactStoreCursorDisposerPhase::TailSnapshot => {
                if self.live.tail_undo_cache.as_ref().is_some_and(|(id, _)| id.capacity() != 0) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "tail snapshot still retains original edit-id backing").into()); }
                if self.live.tail_undo_cache.is_none() { return Ok((None, structural)); }
                let factory = self.owners.snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "tail close requires installed factory"))?;
                let (id, snapshot) = self.live.tail_undo_cache.take().expect("observed original tail owner");
                if Arc::strong_count(&snapshot) > 1 { drop((id, snapshot)); return Ok((None, structural)); }
                let mut snapshot = Some(snapshot);
                match settle_snapshot_factory(admit_snapshot_root(factory, &mut snapshot, effect)) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err(exit) => { *self.live.tail_undo_cache = Some((id, snapshot.take().expect("a refused factory keeps its snapshot"))); Err(exit) }
                }
            }
            ArtifactStoreCursorDisposerPhase::CurrentSnapshot => {
                if self.live.current_detached { return Ok((None, structural)); }
                if self.live.tail_undo_cache.is_some() || !self.owners.snapshot_read_leases_terminal_is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "current snapshot still holds tail or read owners").into()); }
                let genesis = std::ptr::eq(self.live.current.as_ref(), self.log.envelope.vcs.genesis.facts().snapshot());
                let result = if genesis {
                    let factory = self.owners.initial_snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "genesis alias requires exact initial factory"))?.clone();
                    let original = unsafe { std::mem::ManuallyDrop::take(&mut self.live.current) };
                    admit_artifact_retirement(original, grant, |original| ReturnedSnapshotReadRetirement::new(original, factory)).map_err(|(error, original)| (error.into(), original))
                } else {
                    let factory = self.owners.snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "current snapshot requires exact factory"))?;
                    let original = unsafe { std::mem::ManuallyDrop::take(&mut self.live.current) };
                    let mut snapshot = Some(original);
                    match settle_snapshot_factory(admit_snapshot_root(factory, &mut snapshot, effect)) {
                        Ok(admitted) => Ok(admitted),
                        Err(exit) => Err((exit, snapshot.take().expect("a refused factory keeps its snapshot"))),
                    }
                };
                match result {
                    Ok((owner, receipt)) => { self.live.current_detached = true; Ok((Some(owner), receipt)) }
                    Err((exit, original)) => { self.live.current = std::mem::ManuallyDrop::new(original); Err(exit) }
                }
            }
            phase => self.close_cursor_admit_owned(phase, grant).map_err(Into::into),
        }
    }
}

impl<P, Mutation> StoreCloseWitness for StorePartsRef<'_, P, Mutation>
where
    P: semio_os_spr::StoreState,
    Mutation: Clone + ToValue + FromValue + semio_os_spr::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    fn snapshot_read_leases_terminal_is_empty(&self) -> bool {
        self.owners.snapshot_read_leases_terminal_is_empty()
    }

    fn close_structural_owners_terminal_is_empty(&self) -> bool {
        let envelope_shell_is_empty = self.log.envelope_detached
            || (self.log.envelope.conflicts.is_empty()
                && self.log.envelope.transitions.is_empty()
                && self.log.envelope.vcs.edits.is_empty()
                && self.log.envelope.vcs.changes.is_empty()
                && self.log.envelope.vcs.checkpoints.is_empty()
                && self.log.envelope.vcs.alternatives.is_empty()
                && self.log.envelope.edit_messages.is_empty()
                && self.log.envelope.lanes.is_empty());
        let runtime_shell_is_empty = self.wire.backbone.is_none()
            && self.wire.member_inbox.capacity() == 0
            && self.replay.pending_report.is_idle()
            && self.live.supersessions.is_empty()
            && self.live.prefix_ring.is_empty();
        envelope_shell_is_empty && runtime_shell_is_empty && self.wire.durable_group_root.is_none() && self.log.dag.terminal_is_empty() && self.owners.displaced_retirements.terminal_is_empty()
    }

    fn owned_roots_terminal_is_empty(&self) -> bool {
        self.live.current_detached
            && self.log.envelope_detached
            && self.wire.durable_group_root.is_none()
            && self.live.tail_undo_cache.is_none()
            && self.live.prefix_ring.is_empty()
            && self.snapshot_read_leases_terminal_is_empty()
            && self.close_structural_owners_terminal_is_empty()
    }
}
//#endregion 🔖️ArtifactStore
