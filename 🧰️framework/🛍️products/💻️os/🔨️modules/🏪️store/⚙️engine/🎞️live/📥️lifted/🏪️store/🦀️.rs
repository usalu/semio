//! 📥️ Items of `store` that belong to `os-store-engine-live` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🦀️.rs`.
use semio_framework_os_store_text_io::ArtifactStoreSnapshot as _;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
use semio_framework_os_store_replay::replay_preparation::ArtifactReplayPreparationProgress;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_os_spr::{HybridLogicalTimestamp, MutationId};
use semio_framework_os_spr::{Edit, Mutation, MutationMeta, OpBinary, OpText};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::sync::Arc;
use semio_framework_os_store_read::{ArtifactDerivedHistoryPreview, ArtifactDerivedReplay, ArtifactDerivedSnapshot, ArtifactStoreDisplacedSnapshotRetirement, SnapshotRead};
use semio_framework_os_store_read::{ArtifactStoreCursorDisposerPhase, ArtifactStoreEnvelopeRetirement, ArtifactStoreMutationDagRetirement, ArtifactStoreRevisionAccumulatorRetirement, ArtifactStoreStringRetirement, ArtifactStoreStringVectorRetirement, ArtifactStoreSupersessionRetirement, HistoryReadPlan, SnapshotReadLeaseRefusal};
use semio_framework_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirement;
use semio_framework_os_spr::command::MutationMessageLedgerRetirement as ArtifactStoreMessageLedgerRetirement;
use semio_framework_value::factory::FactorySharedRetirement as ReturnedSnapshotReadRetirement;
use semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
use semio_framework_os_vcs::io::binary::entity_identity::mint_edit_id;
use semio_framework_os_vcs::VcsError;
use semio_framework_os_vcs::ArtifactHistoryKey;
use semio_framework_os_store_space_schema::{ArtifactEnvelope, ArtifactProjectionCacheMode, ArtifactProjectionCause, ArtifactProjectionEvent, ArtifactProjectionInvalidation, HistoryLane, OpenToolTransaction, SupersedeInput};
use semio_framework_os_store_replay::ReplayTurnBudget;
use semio_framework_os_store_replay::{PendingLocalStep, PendingReprojection, ReprojectionAdvance, TailStep};
//#region 🔖️EphemeralLanes

use semio_framework_os_store_ephemeral::*;
use semio_framework_os_store_engine_core::lifted_document_materialize::now_iso;
use semio_framework_os_store_initialization_runtime::lifted_document_materialize::now_ms;
use crate::lifted_document_materialize::stamp_primary_operation_identity;
use semio_framework_os_store_text_io::text_format::{common_address, mutation_ids};
use semio_framework_os_store_command::retire_operations;
use semio_framework_os_store_ledger::{edit_from_operation_envelope, fold_envelope_history, fold_event_log};
use semio_framework_os_store_ledger::{ARTIFACT_EDIT_MESSAGE_ID_BYTES, ArtifactStoreHistoryCommitReservation, ArtifactStorePendingReportRetirement, CursorRevisionAccumulator, PendingCommandReport, StackMirrorDirt, edit_timestamp, envelope_viewer_head, retire_scratch_edits, retire_scratch_operations, retire_supersede_inputs, stamp_edit_semantics};
#[cfg(test)]
use semio_framework_os_store_ledger::hot_path_census;
use semio_framework_os_store_one_item_batch::lifted_one_item_publication::ArtifactStoreBatchPublication;
use semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPublicationPhase;
use semio_framework_os_store_operation_wire::{ArtifactPreparedOperationCursor, ArtifactPreparedOperationProgress, ArtifactPreparedOperationError};
use semio_framework_os_store_replay::{DRAFT_SUPERSESSION_ID, EditReplay, EditReplayResult, EffectiveSupersessions, ReplayEdits, ReplayMode, ReplayProgress, ReplayStep, admit_replacement, draft_supersessions, effective_operation, fold_operation};
use semio_framework_os_store_replay::{FoldProjection, IndexedAppliedReplayEdits, PREFIX_SNAPSHOT_RING_CAPACITY, PREFIX_SNAPSHOT_RING_FLOOR, PrefixSnapshot, ReplayConvergence, effective_operation_with_supersession, fold_effective_edit, fold_recorded, forward_prefix_digests, forward_prefix_genesis, forward_prefix_step, retire_shared_projection, same_effective_inputs, shared_prefix_len, superseded_positions};
//#region 🔖️Backbone
//#region 🔖️Backbone
use semio_framework_os_store_backbone as backbone;
use backbone::{Backbone, BackboneMessage, Backbones};
use semio_framework_os_store_backbone::backbone_retirement::ArtifactStoreBackboneRetirement;
use semio_framework_os_store_engine_core::lifted_store::MIRROR_ORACLE_HISTORY;
use semio_framework_os_store_engine_core::lifted_store::MIRROR_ORACLE_EVERY_HISTORY;
use semio_framework_os_store_engine_core::lifted_store::MIRROR_ORACLE_STRIDE;
use semio_framework_os_store_initialization_runtime::lifted_store::ArtifactStoreInitializationOwnerCatalog;
use semio_framework_os_store_initialization_runtime::lifted_store::bound_edit_messages;
use semio_framework_os_store_engine_core::lifted_store::StoreLog;
use semio_framework_os_store_engine_core::lifted_store::StoreOwners;
use semio_framework_os_store_engine_core::lifted_store::StoreWire;
use semio_framework_os_store_engine_core::lifted_store::StoreCursor;
use semio_framework_os_store_engine_core::lifted_store::ScratchOwner;
use semio_framework_os_store_engine_core::lifted_store::fold_history;
//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
/// ⚡️ The incrementally maintained fold of one store: the live snapshot, the tail undo cache, the supersessions, the prefix ring and the fold frontier.
pub struct StoreLive<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    /// ⚡️ The live, incrementally-maintained RAW fold of `initial_snapshot` over every
    /// `forwards` operation in `applied_edit_ids` order — i.e. exactly what a full
    /// {@link materialize_document_snapshot} replay computes BEFORE its single final
    /// {@link Mutation::reconcile} call. Kept in lock-step by every mutating command below instead of
    /// replaying on every read, so `snapshot()`/`Apply`/`AppendTransaction` are O(new work) instead of
    /// O(total history). Cold-path commands (checkout/switch/set_state, which reassign
    /// `applied_edit_ids` wholesale rather than appending) fall back to a full raw-fold recompute —
    /// see `fold_current`. Differential ground truth: `test_support::assert_live_equals_replay`.
    pub current: std::mem::ManuallyDrop<Arc<P>>,
    pub current_detached: bool,
    /// 🪢️ `(edit_id, snapshot right before that edit's forwards were first applied)` for
    /// whichever edit is CURRENTLY the tail of `applied_edit_ids` — refreshed by `Apply`/`AppendTransaction`
    /// (fresh-edit branch)/`Redo`, left untouched by further amends to the same edit (so it always
    /// points at the state before the edit as a whole, not before its latest increment). Powers an
    /// O(1) `Undo` of exactly this edit; any other undo (not the cached tail, or `None`) falls back
    /// to `fold_current` — always correct, just not always O(1).
    pub tail_undo_cache: std::mem::ManuallyDrop<Option<(String, Arc<P>)>>,
    /// ✏️ The effective supersessions `current` was projected with ([`crate::os_spr::HistoryFold::supersessions`]) —
    /// what {@link reproject} compares a fresh fold against to find the first position whose effective input changed.
    pub supersessions: std::mem::ManuallyDrop<EffectiveSupersessions>,
    /// 🧭️ Retained projections after applied prefixes ([`PrefixSnapshot`]), at most [`PREFIX_SNAPSHOT_RING_CAPACITY`]
    /// at an adaptive stride: "the state before operation X" costs at most one stride of folds. An entry is valid only
    /// while its forwards-only digest names the live effective prefix; every evicted entry retires through the snapshot
    /// retirement factory (or only drops its alias while another owner holds the root), never a bare drop.
    pub prefix_ring: std::mem::ManuallyDrop<Vec<PrefixSnapshot<P>>>,
    /// ✂️ The first applied position whose id, forwards or effective input changed since the prefix ring was last pruned: every
    /// ring entry past it is stale ({@link prune_prefix_ring}).
    pub prefix_ring_dirty_from: usize,
    /// 🧭️ The greatest `(hlc)` event key of the log while the live cursor — applied, redo, checkpoint, alternative and
    /// supersessions — is exactly that log's fold at this replica's head; `None` until the next full fold proves it again. What
    /// lets a local edit sorting after every event join the fold as its tail without folding the log ({@link folded_tail}).
    pub fold_frontier: Option<(u64, u64, u64)>,
    /// ⚡️ The generation right after an undo or redo took the tail path ({@link tail_step}): while it still is the
    /// generation, nothing happened since, so the redo top is the edit that path moved and redoing it is a tail step again.
    pub tail_step_generation: Option<u64>,
    /// 🟰️ Snapshot equality that lets a Report replay stop where it meets the previous projection
    /// ({@link enable_convergence_early_exit}).
    pub snapshot_equality: Option<fn(&P, &P) -> bool>,
    pub marker: std::marker::PhantomData<fn() -> (P, Mutation)>,
}

/// 🐢️ The deferred reprojection of one store: the remote and local replay budgets, the pending reprojection and the pending command report.
pub struct StoreReplay<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    /// 🐢️ How far one turn of a deferred remote reprojection replays; `None` replays inside the ingest.
    pub replay_budget: Option<ReplayTurnBudget>,
    /// ⏱️ How far one turn of a deferred local history step replays; `None` replays inside the dispatch.
    pub local_replay_budget: Option<ReplayTurnBudget>,
    /// 🐢️ The history change admitted but not adopted yet ([`Self::step_reprojection`]).
    pub pending_reprojection: Option<PendingReprojection<P, Mutation>>,
    pub pending_report: std::mem::ManuallyDrop<PendingCommandReport>,
}
//#endregion 🔖️ArtifactStore
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority

//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
/// 🧩️ The six components of one store borrowed at once: what staging code below the composer takes instead of `&mut ArtifactStore`.
pub struct StoreParts<'a, P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    pub log: &'a mut StoreLog<P, Mutation>,
    pub owners: &'a mut StoreOwners<P, Mutation>,
    pub wire: &'a mut StoreWire<P, Mutation>,
    pub cursor: &'a mut StoreCursor<P, Mutation>,
    pub live: &'a mut StoreLive<P, Mutation>,
    pub replay: &'a mut StoreReplay<P, Mutation>,
}

/// 🧩️ The shared counterpart of [`StoreParts`] for code that only reads the store.
pub struct StorePartsRef<'a, P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    pub log: &'a StoreLog<P, Mutation>,
    pub owners: &'a StoreOwners<P, Mutation>,
    pub wire: &'a StoreWire<P, Mutation>,
    pub cursor: &'a StoreCursor<P, Mutation>,
    pub live: &'a StoreLive<P, Mutation>,
    pub replay: &'a StoreReplay<P, Mutation>,
}

impl<'a, P, Mutation> StoreParts<'a, P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    /// 🧩️ Reborrows the components for read-only helpers.
    #[inline]
    pub fn shared(&self) -> StorePartsRef<'_, P, Mutation> {
        StorePartsRef { log: &*self.log, owners: &*self.owners, wire: &*self.wire, cursor: &*self.cursor, live: &*self.live, replay: &*self.replay }
    }
}
//#endregion 🔖️ArtifactStore
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority

//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
pub struct ArtifactStoreDocumentRootCommitAuthority<P, Mutation> {
    pub snapshot: Arc<dyn SnapshotRetirementFactory<P>>,
    pub initial_snapshot: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
    pub mutation: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
}

/// 🖋️ Derives an edit's authoring actor from its per-operation metadata (the author of its
/// first operation), so a local edit records who produced it for later `UndoPolicy` classification.
pub async fn edit_actor_from_meta(mutation_meta: &[MutationMeta]) -> Option<semio_framework_value::SharedUtf8> {
    mutation_meta.first().and_then(|meta| meta.author_id.clone()).map(|actor_id| actor_id.0)
}
//#endregion 🔖️ArtifactStore
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority

//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
impl<P, Mutation> StoreLive<P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    /// 📬️ Captures one immutable projection or inference input from the current reconciled snapshot.
    pub fn projection_event<Previous, Policy>(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, cause: ArtifactProjectionCause, previous: Option<Previous>, cache_mode: ArtifactProjectionCacheMode, policy: Policy) -> Result<ArtifactProjectionEvent<P, Previous, Policy>, VcsError> {
        Ok(ArtifactProjectionEvent { stamp: store_cursor.projection_stamp(store_log, store_wire), cause, state: self.snapshot(store_wire)?, previous, cache_mode, policy })
    }

    /// ⚡️ The live snapshot: the incrementally-maintained `current` fold, as-is. Always `Ok`
    /// in practice (kept as `Result` for API stability); O(1) instead of a full replay. See the
    /// `current` field doc for the maintenance invariant.
    pub fn snapshot(&self, store_wire: &StoreWire<P, Mutation>) -> Result<P, VcsError> {
        Ok(store_wire.durable_group_read_root().map_or(self.current.as_ref(), |root| root.current.as_ref().expect("unadopted durable group root retains its current snapshot").as_ref()).clone())
    }

    /// 🔎️ Borrows the current immutable fold without issuing or cloning a snapshot owner.
    pub fn snapshot_ref(&self, store_wire: &StoreWire<P, Mutation>) -> &P {
        store_wire.durable_group_read_root().map_or(self.current.as_ref(), |root| root.current.as_ref().expect("unadopted durable group root retains its current snapshot").as_ref())
    }

    /// 🌱️ Captures the current Store projection as the base of a provisional member fold.
    pub fn derived_snapshot_head(&self, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>) -> ArtifactDerivedSnapshot<P> {
        store_cursor.seal_derived_snapshot(store_owners, store_wire, self.snapshot_owner(store_wire))
    }

    /// 🎞️ Derives a history-edit preview base from this Store's actual history and accepted draft inputs.
    pub fn derived_snapshot_before(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, target: &MutationId, drafts: &protocol::HistoryInputDrafts) -> Result<ArtifactDerivedSnapshot<P>, VcsError> {
        let owner = self.state_before(store_log, store_owners, store_wire, store_cursor, target, drafts)?;
        Ok(store_cursor.seal_derived_snapshot(store_owners, store_wire, owner))
    }

    /// ⏭️ Resolves identities or folds at most one operation between deadline checks; downstream operations are excluded.
    pub fn step_derived_history_preview(&self, store_log: &StoreLog<P, Mutation>, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, preview: &mut ArtifactDerivedHistoryPreview<P, Mutation>, retirement_grant:RetainedCloneGrant, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
        store_cursor.admit_derived_source(store_owners, store_wire, &preview.registry, preview.generation, preview.revision)?;
        if preview.plan_retirement.is_some() {
            preview.retire_completed_plan(retirement_grant).map_err(|error|VcsError::ValidationFailed(error.into_message()))?;
            return Ok(ReplayStep::Pending(preview.progress));
        }
        if let Some(plan) = preview.plan.as_mut() {
            if !self.step_history_read_plan(store_log, store_wire, store_cursor, plan, deadline)? {
                return Ok(ReplayStep::Pending(plan.progress));
            }
            let live_until=preview.retain_completed_plan();
            let (start, base) = self.live_base(store_log, store_cursor, live_until);
            preview.cursor = start;
            preview.state = Some(base);
            return Ok(ReplayStep::Pending(preview.progress));
        }
        let (position, op_index) = preview.target.ok_or_else(|| VcsError::ValidationFailed("history preview lost its target".into()))?;
        while !preview.finished {
            if preview.cursor == position && preview.operation == op_index {
                preview.finished = true;
                break;
            }
            let edit = store_cursor.applied_edit(store_log, store_wire, preview.cursor)?;
            if preview.operation == edit.forwards.len() {
                preview.cursor += 1;
                preview.operation = 0;
                continue;
            }
            let index = preview.operation;
            if index == 0 {
                let count = if preview.cursor == position { op_index } else { edit.forwards.len() };
                preview.progress.total = preview.progress.total.saturating_add(u32::try_from(count).unwrap_or(u32::MAX));
            }
            let id = semio_framework_os_spr::mutation_id_for_edit_operation::<P, Mutation>(edit, index).expect("recorded operation identity");
            let draft = preview.drafts.get(&id).map(|replacement| protocol::EffectiveSupersession { transition_id: DRAFT_SUPERSESSION_ID.into(), actor: semio_framework_value::SharedUtf8::default(), timestamp: HybridLogicalTimestamp::new(0, 0), scope: None, replacement: replacement.clone() });
            let effective = effective_operation_with_supersession::<P, Mutation>(&edit.forwards[index], index as u32, id.clone(), &store_log.envelope.schema, draft.as_ref().or_else(|| self.supersessions.get(&id)));
            if let Some(operation) = effective.operation() {
                let running = preview.state.as_mut().expect("preview running projection");
                if let (Some(next), _) = fold_operation::<P, Mutation>(running, operation, index as u32) {
                    <Arc<P> as FoldProjection<P, Mutation>>::advance(running, next);
                }
            }
            preview.operation += 1;
            preview.progress.done = preview.progress.done.saturating_add(1);
            if deadline() {
                return Ok(ReplayStep::Pending(preview.progress));
            }
        }
        Ok(ReplayStep::Finished(preview.progress))
    }

    /// ⏭️ Steps sealed planning and replay against this store's actual immutable frontier.
    pub fn step_derived_report_replay(&self, store_log: &StoreLog<P, Mutation>, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, owner: &mut ArtifactDerivedReplay<P, Mutation>, retirement_grant:RetainedCloneGrant, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
        store_cursor.admit_derived_source(store_owners, store_wire, &owner.registry, owner.generation, owner.revision)?;
        if owner.plan_retirement.is_some() {
            owner.retire_completed_plan(retirement_grant).map_err(|error|VcsError::ValidationFailed(error.into_message()))?;
            return Ok(ReplayStep::Pending(owner.replay.as_ref().map_or_else(ReplayProgress::default,|replay|replay.progress())));
        }
        if let Some(plan) = owner.plan.as_mut() {
            if !self.step_history_read_plan(store_log, store_wire, store_cursor, plan, deadline)? {
                return Ok(ReplayStep::Pending(plan.progress));
            }
            let generation=owner.generation;let revision=owner.revision;
            let plan=owner.retain_completed_plan();
            let from = plan.first_input;
            let horizon = plan.last_draft;
            let (start, base) = self.live_base(store_log, store_cursor, from);
            let total = plan.operations.saturating_sub(plan.counts[from]);
            let mut replay = EditReplay::new_counted(ReplayMode::Report, base, std::mem::replace(&mut plan.order,semio_framework_os_vcs::HistoryPageStack::empty()), from, &store_log.envelope.schema, std::mem::take(&mut plan.supersessions), total)?;
            replay.cursor = start;
            replay.prefix_work_progress = true;
            replay.progress.total = replay.progress.total.saturating_add(plan.counts[from].saturating_sub(plan.counts[start]));
            replay.progress.done = plan.progress.done;
            replay.progress.total = replay.progress.total.saturating_add(plan.progress.done);
            replay = replay.with_base(generation, revision).with_drafts(std::mem::take(&mut plan.drafts));
            replay.operation_preparation_factory = (*store_owners.replay_preparation_factory).clone();
            if let (Some(snapshots), Some(mutations)) = ((*store_owners.initial_snapshot_retirement_factory).clone(), (*store_owners.mutation_retirement_factory).clone()) {
                replay = replay.with_retirement_factories(snapshots, mutations);
            } else if replay.operation_preparation_factory.is_some() {
                let _ = replay.retire_unscheduled();
                return Err(VcsError::ValidationFailed("interactive operation preparation requires exact replay retirement factories".into()));
            }
            if let (Some(horizon), Some(equal)) = (horizon, self.snapshot_equality) {
                match self.convergence_checkpoints(store_log, store_cursor, horizon, equal) {
                    Ok(checkpoints) => replay = replay.with_convergence(Some(checkpoints)),
                    Err(error) => {
                        let _ = replay.retire_unscheduled();
                        return Err(error);
                    }
                }
            }
            owner.replay = Some(replay);
            return Ok(ReplayStep::Pending(owner.replay.as_ref().expect("planned replay").progress()));
        }
        owner.replay.as_mut().ok_or_else(|| VcsError::ValidationFailed("derived replay lost its cursor".into()))?.step(&IndexedAppliedReplayEdits { edits: &store_log.envelope.vcs.edits, revision: &store_cursor.revision_accumulator, ids: &store_log.applied_edit_ids }, deadline)
    }

    fn step_history_read_plan(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, plan: &mut HistoryReadPlan, deadline: &mut dyn FnMut() -> bool) -> Result<bool, VcsError> {
        use std::ops::Bound::{Excluded, Unbounded};
        if plan.progress.total == 0 {
            let work = usize::from(plan.target.is_some()).saturating_add(plan.drafts.len()).saturating_add(if plan.report { store_log.applied_edit_ids.len().saturating_add(self.supersessions.len()).saturating_add(plan.drafts.len()) } else { 0 });
            plan.progress.total = u32::try_from(work).unwrap_or(u32::MAX);
        }
        while !plan.finished {
            if !plan.located_target {
                plan.located_target = true;
                if let Some(target) = plan.target.as_ref() {
                    let position = store_cursor.locate_mutation(store_log, store_wire, target)?;
                    plan.first_input = plan.first_input.min(position.0);
                    plan.positions.insert(target.clone(), position);
                } else { continue; }
            } else if !plan.located_inputs {
                let next = match plan.located_draft.as_ref() { Some(key) => plan.drafts.range((Excluded(key), Unbounded)).next(), None => plan.drafts.iter().next() }.map(|(target, _)| target.clone());
                if let Some(target) = next {
                    let position = store_cursor.locate_mutation(store_log, store_wire, &target)?;
                    plan.first_input = plan.first_input.min(position.0);
                    plan.last_draft = Some(plan.last_draft.unwrap_or(0).max(position.0 + 1));
                    plan.positions.insert(target.clone(), position);
                    plan.located_draft = Some(target);
                } else {
                    plan.located_inputs = true;
                    if !plan.report { plan.finished = true; }
                    continue;
                }
            } else if plan.cursor < store_log.applied_edit_ids.len() {
                let edit = store_cursor.applied_edit(store_log, store_wire, plan.cursor)?;
                plan.counts.push(plan.operations);
                plan.operations = plan.operations.saturating_add(u32::try_from(edit.forwards.len()).unwrap_or(u32::MAX));
                plan.order.push(edit.id.clone());
                plan.cursor += 1;
            } else {
                let next = if plan.copying_drafts {
                    match plan.copied.as_ref() { Some(key) => plan.drafts.range((Excluded(key), Unbounded)).next(), None => plan.drafts.iter().next() }
                        .map(|(target, replacement)| (target.clone(), protocol::EffectiveSupersession { transition_id: DRAFT_SUPERSESSION_ID.into(), actor: semio_framework_value::SharedUtf8::default(), timestamp: HybridLogicalTimestamp::new(0, 0), scope: None, replacement: replacement.clone() }))
                } else {
                    match plan.copied.as_ref() { Some(key) => self.supersessions.first_entry_after(key), None => self.supersessions.iter().next() }.map(|(target, supersession)| (target.clone(), supersession.clone()))
                };
                if let Some((target, supersession)) = next {
                    plan.copied = Some(target.clone());
                    plan.supersessions.insert(target, supersession);
                } else if !plan.copying_drafts {
                    plan.copying_drafts = true;
                    plan.copied = None;
                    continue;
                } else { plan.finished = true; continue; }
            }
            plan.progress.done = plan.progress.done.saturating_add(1);
            if deadline() { return Ok(false); }
        }
        if plan.report && plan.counts.len() == store_log.applied_edit_ids.len() { plan.counts.push(plan.operations); }
        Ok(true)
    }

    /// 🧵️ Immutable O(1) snapshot capability for worker and composition boundaries.
    pub fn snapshot_read(&self, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>) -> Result<SnapshotRead<P>, VcsError>
    where
        P: Sync,
    {
        let generation = store_cursor.generation(store_wire);
        let content_revision = store_cursor.content_revision(store_wire);
        if !store_owners.snapshot_read_leases.publish_authority(generation, content_revision) {
            return Err(VcsError::ValidationFailed("snapshot read commit authority is busy or exhausted".into()));
        }
        let owner = self.snapshot_owner(store_wire);
        let lease = store_owners.snapshot_read_leases.try_issue(owner.clone()).map_err(|_| VcsError::ValidationFailed("snapshot read lease registry is busy, saturated, or exhausted".into()))?;
        Ok(SnapshotRead::new(owner, lease))
    }

    pub fn snapshot_owner(&self, store_wire: &StoreWire<P, Mutation>) -> Arc<P> {
        store_wire.durable_group_read_root().map_or_else(|| Arc::clone(&*self.current), |root| Arc::clone(root.current.as_ref().expect("unadopted durable group root retains its current snapshot")))
    }

    /// 🧵️ Captures the immutable event-maintained snapshot root in O(1).
    pub fn snapshot_root(&self, store_wire: &StoreWire<P, Mutation>) -> Arc<P> {
        self.snapshot_owner(store_wire)
    }

    pub fn replace_current_retained(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, next: Arc<P>) -> Result<(), VcsError> {
        if Arc::ptr_eq(&self.current, &next) {
            return Ok(());
        }
        let next = ScratchOwner::new(next, retire_shared_projection::<P, Mutation>);
        let previous_is_retained = !self.current_detached && (self.tail_undo_cache.as_ref().is_some_and(|(_, snapshot)| Arc::ptr_eq(snapshot, &self.current)) || std::ptr::eq(self.current.as_ref(), store_log.envelope.vcs.genesis.facts().snapshot()));
        let factory = if previous_is_retained {
            None
        } else {
            store_owners.displaced_retirements.reserve(1)?;
            Some((*store_owners.snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store current replacement requires its exact snapshot retirement factory".into()))?)
        };
        let previous = std::mem::replace(&mut *self.current, next.adopt());
        if previous_is_retained {
            drop(previous);
        } else {
            store_owners.displaced_retirements.push_reserved(ArtifactStoreDisplacedSnapshotRetirement::boxed(previous, factory.expect("non-tail current factory was validated")));
        }
        Ok(())
    }

    fn replace_document_roots_retained(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, next_envelope: ArtifactEnvelope<P, Mutation>, next_current: Arc<P>) -> Result<(), VcsError> {
        store_owners.displaced_retirements.reserve(2)?;
        let initial_snapshot_factory = (*store_owners.initial_snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store document-root replacement requires its exact initial-snapshot retirement factory".into()))?;
        let mutation_factory = (*store_owners.mutation_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store document-root replacement requires its exact mutation retirement factory".into()))?;
        let snapshot_factory = (*store_owners.snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store document-root replacement requires its exact snapshot retirement factory".into()))?;
        let previous_envelope = std::mem::replace(&mut *store_log.envelope, next_envelope);
        let previous_current = std::mem::replace(&mut *self.current, next_current);
        let current_is_genesis = std::ptr::eq(previous_current.as_ref(), previous_envelope.vcs.genesis.facts().snapshot());
        store_owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreEnvelopeRetirement::new(previous_envelope, initial_snapshot_factory, mutation_factory)));
        if current_is_genesis {
            drop(previous_current);
        } else {
            store_owners.displaced_retirements.push_reserved(ArtifactStoreDisplacedSnapshotRetirement::boxed(previous_current, snapshot_factory));
        }
        Ok(())
    }

    pub fn prepare_document_root_commit(&mut self, store_owners: &mut StoreOwners<P, Mutation>, additional_displaced_owners: usize) -> Result<ArtifactStoreDocumentRootCommitAuthority<P, Mutation>, VcsError> {
        let tail_slots = self.tail_undo_cache.as_ref().map_or(0, |(edit_id, snapshot)| usize::from(!edit_id.is_empty()) + usize::from(self.current_detached || !Arc::ptr_eq(snapshot, &self.current)));
        store_owners.displaced_retirements
            .reserve(3usize.checked_add(tail_slots).and_then(|slots| slots.checked_add(additional_displaced_owners)).ok_or_else(|| VcsError::ValidationFailed("artifact store document-root retirement reservation overflowed".into()))?)?;
        Ok(ArtifactStoreDocumentRootCommitAuthority {
            snapshot: (*store_owners.snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store document-root transaction requires its exact snapshot retirement factory".into()))?,
            initial_snapshot: (*store_owners.initial_snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store document-root transaction requires its exact initial-snapshot retirement factory".into()))?,
            mutation: (*store_owners.mutation_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store document-root transaction requires its exact mutation retirement factory".into()))?,
        })
    }

    pub fn commit_document_roots_retained(
        &mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>,
        next_envelope: ArtifactEnvelope<P, Mutation>,
        next_current: Arc<P>,
        next_dag: semio_framework_os_spr::MutationDag,
        next_tail: Option<(String, Arc<P>)>,
        authority: ArtifactStoreDocumentRootCommitAuthority<P, Mutation>,
    ) {
        if let Some((edit_id, snapshot)) = self.tail_undo_cache.take() {
            if !edit_id.is_empty() {
                store_owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreStringRetirement::new(edit_id)));
            }
            if (!self.current_detached && Arc::ptr_eq(&snapshot, &self.current)) || std::ptr::eq(snapshot.as_ref(), store_log.envelope.vcs.genesis.facts().snapshot()) {
                drop(snapshot);
            } else {
                store_owners.displaced_retirements.push_reserved(ArtifactStoreDisplacedSnapshotRetirement::boxed(snapshot, Arc::clone(&authority.snapshot)));
            }
        }
        let previous_envelope = std::mem::replace(&mut *store_log.envelope, next_envelope);
        let previous_current = std::mem::replace(&mut *self.current, next_current);
        let previous_dag = std::mem::replace(&mut *store_log.dag, next_dag);
        let current_is_genesis = std::ptr::eq(previous_current.as_ref(), previous_envelope.vcs.genesis.facts().snapshot());
        store_owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreEnvelopeRetirement::new(previous_envelope, authority.initial_snapshot, authority.mutation)));
        if current_is_genesis {
            drop(previous_current);
        } else {
            store_owners.displaced_retirements.push_reserved(ArtifactStoreDisplacedSnapshotRetirement::boxed(previous_current, Arc::clone(&authority.snapshot)));
        }
        store_owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreMutationDagRetirement::new(previous_dag)));
        *self.tail_undo_cache = next_tail;
    }

    pub fn string_vector_owner_slots(values: &semio_framework_os_vcs::HistoryPageStack<String>) -> usize {
        usize::from(!values.is_empty() || values.capacity() != 0)
    }

    pub fn string_owner_slots(value: &Option<String>) -> usize {
        usize::from(value.is_some())
    }

    pub fn take_string_vector_replacement(target: &mut std::mem::ManuallyDrop<semio_framework_os_vcs::HistoryPageStack<String>>, next: semio_framework_os_vcs::HistoryPageStack<String>) -> Option<Box<dyn ErasedSnapshotRetirement>> {
        let previous = std::mem::replace(&mut **target, next);
        (!previous.is_empty() || previous.capacity() != 0).then(|| Box::new(ArtifactStoreStringVectorRetirement::new(previous)) as Box<dyn ErasedSnapshotRetirement>)
    }

    pub fn take_string_replacement(target: &mut std::mem::ManuallyDrop<Option<String>>, next: Option<String>) -> Option<Box<dyn ErasedSnapshotRetirement>> {
        let previous = std::mem::replace(&mut **target, next);
        previous.map(|previous| Box::new(ArtifactStoreStringRetirement::new(previous)) as Box<dyn ErasedSnapshotRetirement>)
    }

    /// 🔁️ Replaces the applied stack. From the first position whose id changed, the persisted cursor and the revision
    /// records are stale ([`StackMirrorDirt`]), the prefix ring too, and the live cursor is no longer a proven fold.
    pub fn replace_applied_edit_ids_retained(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, next: Vec<String>) -> Result<(), VcsError> {
        let common = shared_prefix_len(&store_log.applied_edit_ids, &next);
        let changed = common != store_log.applied_edit_ids.len() || common != next.len();
        let next = ArtifactStoreInitializationOwnerCatalog::retain_id_capacity(next);
        store_owners.displaced_retirements.reserve(Self::string_vector_owner_slots(&store_log.applied_edit_ids))?;
        if changed {
            store_cursor.mirror_dirt.applied = store_cursor.mirror_dirt.applied.min(common);
            self.prefix_ring_dirty_from = self.prefix_ring_dirty_from.min(common);
            self.fold_frontier = None;
        }
        if let Some(owner) = Self::take_string_vector_replacement(&mut store_log.applied_edit_ids, next) {
            store_owners.displaced_retirements.push_reserved(owner);
        }
        Ok(())
    }

    /// 🔂️ Replaces the redo stack; its mirrors are stale from the first position whose id changed.
    pub fn replace_redo_edit_ids_retained(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, next: Vec<String>) -> Result<(), VcsError> {
        let common = shared_prefix_len(&store_log.redo_edit_ids, &next);
        let changed = common != store_log.redo_edit_ids.len() || common != next.len();
        let next = ArtifactStoreInitializationOwnerCatalog::retain_id_capacity(next);
        store_owners.displaced_retirements.reserve(Self::string_vector_owner_slots(&store_log.redo_edit_ids))?;
        if changed {
            store_cursor.mirror_dirt.redo = store_cursor.mirror_dirt.redo.min(common);
            self.fold_frontier = None;
        }
        if let Some(owner) = Self::take_string_vector_replacement(&mut store_log.redo_edit_ids, next) {
            store_owners.displaced_retirements.push_reserved(owner);
        }
        Ok(())
    }

    fn replace_current_checkpoint_retained(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, next: Option<String>) -> Result<(), VcsError> {
        store_owners.displaced_retirements.reserve(Self::string_owner_slots(&store_log.current_checkpoint_id))?;
        if next != *store_log.current_checkpoint_id {
            self.fold_frontier = None;
        }
        if let Some(owner) = Self::take_string_replacement(&mut store_log.current_checkpoint_id, next) {
            store_owners.displaced_retirements.push_reserved(owner);
        }
        Ok(())
    }

    pub fn insert_reserved_edit_history(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, mut reservation: ArtifactStoreHistoryCommitReservation, edit: Edit<Mutation>) -> Result<ArtifactHistoryKey, VcsError> {
        let factory = (*store_owners.mutation_retirement_factory).clone().expect("edit history admission validated its exact mutation retirement factory");
        self.fold_frontier = None;
        match store_log.envelope.vcs.edits.insert_reserved(reservation.history, edit) {
            Ok(key) => {
                store_owners.displaced_retirements.release_owner_slots(reservation.rejected_owner)?;
                Ok(key)
            }
            Err((_, edit)) => {
                let owner = Box::new(ArtifactStoreDecodedEditRetirement::new(edit, factory));
                if let Err(owner) = store_owners.displaced_retirements.push_owner_reserved(&mut reservation.rejected_owner, owner) {
                    store_owners.displaced_retirements.push_reserved(owner);
                }
                Err(VcsError::ValidationFailed("edit history reservation is stale".into()))
            }
        }
    }

    pub fn replace_tail_undo_cache_retained(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, next: Option<(String, Arc<P>)>) -> Result<(), VcsError> {
        let Some((edit_id, snapshot)) = self.tail_undo_cache.as_ref() else {
            *self.tail_undo_cache = next;
            return Ok(());
        };
        let snapshot_is_retained = (!self.current_detached && Arc::ptr_eq(snapshot, &self.current)) || std::ptr::eq(snapshot.as_ref(), store_log.envelope.vcs.genesis.facts().snapshot());
        store_owners.displaced_retirements.reserve(usize::from(!edit_id.is_empty()) + usize::from(!snapshot_is_retained))?;
        let factory =
            if snapshot_is_retained { None } else { Some((*store_owners.snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store tail replacement requires its exact snapshot retirement factory".into()))?) };
        let (edit_id, snapshot) = self.tail_undo_cache.take().expect("validated tail undo owner remains present");
        if !edit_id.is_empty() {
            store_owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreStringRetirement::new(edit_id)));
        }
        if snapshot_is_retained {
            drop(snapshot);
        } else {
            store_owners.displaced_retirements.push_reserved(ArtifactStoreDisplacedSnapshotRetirement::boxed(snapshot, factory.expect("nonshared tail snapshot factory was validated")));
        }
        *self.tail_undo_cache = next;
        Ok(())
    }

    pub fn close_cursor_demands(&self, store_log: &StoreLog<P, Mutation>, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, phase: ArtifactStoreCursorDisposerPhase, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        let birth = |capacity_bytes| RetirementDemand { capacity_bytes, depth: 2, ..Default::default() };
        let structural = RetirementDemand { depth: 1, ..Default::default() };
        Ok(match phase {
            ArtifactStoreCursorDisposerPhase::ReturnedReads if store_owners.snapshot_read_leases.has_returned() => store_owners.snapshot_read_leases.returned_admission_demands::<P>(|_| birth(ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes())).map_err(SnapshotReadLeaseRefusal::into_value_error)?,
            ArtifactStoreCursorDisposerPhase::Displaced if !store_owners.maintenance_retirements_terminal_is_empty() => store_owners.maintenance_retirements_demands(body)?,
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(lane) => birth(match lane {
                0 | 1 => size_of::<ArtifactStoreStringVectorRetirement>(),
                2 => size_of::<ArtifactStoreRevisionAccumulatorRetirement>(),
                3 | 5 => size_of::<ArtifactStoreStringRetirement>(),
                4 => size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_value::SharedUtf8>>(),
                6 => size_of::<ArtifactStoreSupersessionRetirement>(),
                _ => size_of::<ArtifactStorePendingReportRetirement>(),
            }),
            ArtifactStoreCursorDisposerPhase::PrefixSnapshots => match self.prefix_ring.last() {
                Some(entry) if Arc::strong_count(&entry.snapshot) == 1 => birth(store_owners.snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "prefix retirement requires exact snapshot factory"))?.retirement_birth_bytes(&entry.snapshot)),
                _ => structural,
            },
            ArtifactStoreCursorDisposerPhase::TailSnapshot => match self.tail_undo_cache.as_ref() {
                Some((_, snapshot)) if Arc::strong_count(snapshot) == 1 => birth(store_owners.snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "tail retirement requires exact snapshot factory"))?.retirement_birth_bytes(snapshot)),
                _ => structural,
            },
            ArtifactStoreCursorDisposerPhase::CurrentSnapshot if !self.current_detached => {
                if std::ptr::eq(self.current.as_ref(), store_log.envelope.vcs.genesis.facts().snapshot()) { birth(ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes()) }
                else { birth(store_owners.snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "current retirement requires exact snapshot factory"))?.retirement_birth_bytes(&self.current)) }
            }
            ArtifactStoreCursorDisposerPhase::Backbone if store_wire.member_inbox.capacity() != 0 || store_wire.backbone.is_some() => birth(ArtifactStoreBackboneRetirement::constructor_capacity_bytes()),
            ArtifactStoreCursorDisposerPhase::Causal => birth(size_of::<ArtifactStoreMutationDagRetirement>()),
            ArtifactStoreCursorDisposerPhase::Envelope if !store_log.envelope_detached => birth(size_of::<ArtifactStoreEnvelopeRetirement<P, Mutation>>()),
            ArtifactStoreCursorDisposerPhase::Complete => Default::default(),
            _ => structural,
        })
    }

    /// 🔂️ Full raw fold of `initial_snapshot` over every `forwards` op in `applied_edit_ids`
    /// order, WITHOUT the final `Mutation::reconcile` pass — the from-scratch computation `current`
    /// is an incrementally-maintained cache of. Used to recompute `current` on the cold paths that
    /// reassign `applied_edit_ids` wholesale instead of appending/popping its tail.
    pub async fn fold_current(&self, store_log: &StoreLog<P, Mutation>) -> Result<P, VcsError> {
        fold_history(&store_log.envelope, &*store_log.applied_edit_ids, &self.supersessions).await
    }

    //#region 🔖️TimeTravelReads
    /// 🗺️ The effective supersessions the live projection folds with.
    pub fn supersessions(&self) -> &EffectiveSupersessions {
        &self.supersessions
    }

    /// 🧾️ Every applied operation in applied order: its identity, owning edit and applied position, operation index,
    /// recorded forward, metadata, tool transaction and effective supersession — O(applied operations).
    pub fn mutation_ops(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>) -> Result<Vec<AppliedMutation<'_, Mutation>>, VcsError> {
        let mut operations = Vec::new();
        for position in 0..store_wire.applied_edit_ids(store_log).len() {
            operations.extend(self.applied_edit_mutations(store_log, store_wire, store_cursor, position)?);
        }
        Ok(operations)
    }

    /// 🧾️ One applied operation without constructing the rest of the history's rows.
    pub fn applied_mutation(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, target: &MutationId) -> Result<AppliedMutation<'_, Mutation>, VcsError> {
        let (position, index) = store_cursor.locate_mutation(store_log, store_wire, target)?;
        let edit = store_cursor.applied_edit(store_log, store_wire, position)?;
        let operation = &edit.forwards[index];
        let meta = edit.mutation_meta.get(index);
        let supersession = self.supersessions.get(target);
        let effective = effective_operation::<P, Mutation>(operation, index as u32, target.clone(), &store_log.envelope.schema, &self.supersessions);
        let descriptor = effective.operation().unwrap_or(operation).descriptor();
        let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit.id.as_bytes()]);
        let spans = store_cursor.applied_unit_flag(store_wire, digest, index) || matches!(meta.map(|meta| &meta.origin), Some(semio_framework_os_spr::MutationOrigin::Transaction { .. }));
        let unit = spans.then(|| meta.and_then(|meta| meta.group_id.clone()).unwrap_or_else(|| target.0.clone()));
        Ok(AppliedMutation { mutation_id: target.clone(), edit_id: &edit.id, position, op_index: index as u32, operation, meta, transaction: meta.and_then(|meta| meta.transaction.as_ref()), supersession, effective: descriptor, unit })
    }

    /// 🧾️ The operations of the applied edit at `position` — what one history row reads, O(that edit) (amortized O(1)
    /// for the edit lookup when rows are read in applied order).
    pub fn applied_edit_mutations(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, position: usize) -> Result<Vec<AppliedMutation<'_, Mutation>>, VcsError> {
        let edit = store_cursor.applied_edit(store_log, store_wire, position)?;
        let units = store_cursor.applied_edit_units(store_log, store_wire, position)?;
        let mut operations = Vec::with_capacity(edit.forwards.len());
        for (index, (mutation_id, operation)) in semio_framework_os_spr::mutation_ids_for_edit::<P, Mutation>(edit).into_iter().zip(edit.forwards.iter()).enumerate() {
            let meta = edit.mutation_meta.get(index);
            let supersession = self.supersessions.get(&mutation_id);
            let effective = match supersession {
                None => operation.descriptor(),
                Some(_) => {
                    let effective = effective_operation::<P, Mutation>(operation, index as u32, mutation_id.clone(), &store_log.envelope.schema, &self.supersessions);
                    effective.operation().unwrap_or(operation).descriptor()
                }
            };
            operations.push(AppliedMutation { mutation_id, edit_id: edit.id.as_str(), position, op_index: index as u32, operation, meta, transaction: meta.and_then(|meta| meta.transaction.as_ref()), supersession, effective, unit: units.get(index).cloned().flatten() });
        }
        Ok(operations)
    }

    /// 📋️ The durable per-mutation outcomes of the applied history: the message ledger split by operation, with every
    /// operation's supersession state.
    pub fn mutation_outcomes(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>) -> Result<Vec<protocol::MutationReplayOutcome>, VcsError> {
        self.durable_outcomes(store_log, store_wire, store_cursor, 0)
    }

    /// 🗄️ The durable outcomes of `applied_edit_ids[from..]`.
    fn durable_outcomes(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, from: usize) -> Result<Vec<protocol::MutationReplayOutcome>, VcsError> {
        let mut outcomes = Vec::new();
        for position in from.min(store_log.applied_edit_ids.len())..store_log.applied_edit_ids.len() {
            outcomes.extend(self.applied_edit_outcomes(store_log, store_wire, store_cursor, position)?);
        }
        Ok(outcomes)
    }

    /// 🗄️ The durable outcomes of the applied edit at `position` — what one history row reads, O(that edit and its
    /// messages).
    pub fn applied_edit_outcomes(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, position: usize) -> Result<Vec<protocol::MutationReplayOutcome>, VcsError> {
        let edit = store_cursor.applied_edit(store_log, store_wire, position)?;
        let messages = store_log.messages_for_edit(&edit.id);
        let mut by_operation: Vec<Vec<semio_framework_os_spr::MutationMessage>> = vec![Vec::new(); edit.forwards.len()];
        for message in messages {
            if let Some(slot) = message.op_index.and_then(|index| by_operation.get_mut(index as usize)) {
                slot.push(message.clone());
            }
        }
        Ok(semio_framework_os_spr::mutation_ids_for_edit::<P, Mutation>(edit)
            .into_iter()
            .zip(by_operation)
            .enumerate()
            .map(|(index, (mutation_id, operation_messages))| {
                let supersession = self.supersessions.get(&mutation_id);
                protocol::MutationReplayOutcome {
                    worst: semio_framework_os_spr::worst_level(&operation_messages),
                    messages: operation_messages,
                    superseded: supersession.is_some(),
                    withdrawn: supersession.is_some_and(|supersession| supersession.replacement == protocol::InputReplacement::Withdrawn),
                    mutation_id,
                    edit_id: edit.id.clone(),
                    op_index: index as u32,
                }
            })
            .collect())
    }

    /// 🫂️ The cross-artifact unit `mutation_id` belongs to; `None` for an operation that is its own unit
    /// ([`Self::applied_edit_units`]). A unit is superseded in all of its documents at once, which no replica can do yet
    /// ([`VcsError::UnitSpansDocuments`]).
    pub fn unit_id(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, mutation_id: &MutationId) -> Result<Option<String>, VcsError> {
        Ok(self.applied_mutation(store_log, store_wire, store_cursor, mutation_id)?.unit)
    }

    /// 🕰️ The projection right before operation `mutation_id` with `drafts` laid over the effective inputs — nothing
    /// downstream applied: the time-travel preview base. Folds at most one prefix-ring stride (plus the operation's own
    /// edit) and retains the live prefix projections it folds through.
    pub fn state_before(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, mutation_id: &MutationId, drafts: &protocol::HistoryInputDrafts) -> Result<Arc<P>, VcsError> {
        let (position, op_index) = store_cursor.locate_mutation(store_log, store_wire, mutation_id)?;
        let mut live_until = position;
        for target in drafts.keys() {
            live_until = live_until.min(store_cursor.locate_mutation(store_log, store_wire, target)?.0);
        }
        let supersessions = draft_supersessions(&self.supersessions, drafts);
        let applied: Vec<String> = store_log.applied_edit_ids.to_vec();
        let (base, recorded) = self.prefix_state_recorded(store_log, store_cursor, &applied, position, &supersessions, Self::prefix_stride(applied.len()))?;
        let (mut live, stale): (Vec<(usize, Arc<P>)>, Vec<(usize, Arc<P>)>) = recorded.into_iter().partition(|(length, _)| *length <= live_until);
        for (_, snapshot) in stale {
            retire_shared_projection::<P, Mutation>(snapshot);
        }
        if position <= live_until {
            live.push((position, Arc::clone(&base)));
        }
        if let Err(error) = self.retain_prefix_snapshots(store_log, store_owners, store_cursor, live) {
            retire_shared_projection::<P, Mutation>(base);
            return Err(error);
        }
        let Some(edit) = store_log.envelope.vcs.edits.find_near(|edit| edit.id == applied[position]) else {
            retire_shared_projection::<P, Mutation>(base);
            return Err(VcsError::UnknownEdit(applied[position].clone()));
        };
        let mut running = base;
        fold_effective_edit(&mut running, edit, &store_log.envelope.schema, &supersessions, op_index);
        Ok(running)
    }

    /// 🎞️ A Report replay of the applied history with `drafts` laid over the effective inputs, from `from` (or the first
    /// operation a draft changes, whichever is earlier). A job steps it against {@link replay_edits} under its deadline
    /// with progress; cancelling drops it and leaves this store untouched; {@link replay_report} reads the result.
    pub fn begin_report_replay(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, drafts: &protocol::HistoryInputDrafts, from: Option<&MutationId>) -> Result<EditReplay<P, Mutation>, VcsError> {
        let supersessions = draft_supersessions(&self.supersessions, drafts);
        let applied: Vec<String> = store_log.applied_edit_ids.to_vec();
        let (window, horizon) = self.replay_window(store_log, &applied, &supersessions)?;
        let from = match from {
            Some(mutation_id) => store_cursor.locate_mutation(store_log, store_wire, mutation_id)?.0.min(window),
            None => window,
        };
        Ok(self.report_replay(store_log, store_cursor, &applied, from, supersessions, horizon, usize::MAX)?.with_drafts(drafts.clone()))
    }

    /// 🛡️ Judges the folded history as a whole under `MergePolicy::Normal`: one Report replay of every applied edit from the
    /// genesis, refused with its blocking messages when the report blocks finalize.
    pub fn refuse_blocking_history(&self, store_log: &StoreLog<P, Mutation>) -> Result<(), VcsError> {
        let result = EditReplay::new(ReplayMode::Report, store_log.envelope.vcs.genesis.facts().share_snapshot(), store_log.applied_edit_ids.to_vec(), 0, &store_log.envelope.schema, (*self.supersessions).clone(), &store_log.envelope.vcs.edits)?.run(&store_log.envelope.vcs.edits)?;
        let blocks = result.report().blocks_finalize();
        let messages: Vec<_> = if blocks { result.report().outcomes.iter().flat_map(|outcome| outcome.messages.iter()).filter(|message| semio_framework_os_spr::MergePolicy::Normal.rejects(message.level)).cloned().collect() } else { Vec::new() };
        result.retire_unscheduled().map_err(|error| VcsError::ValidationFailed(error.into_message()))?;
        if !blocks {
            return Ok(());
        }
        Err(VcsError::Rejected { policy: semio_framework_os_spr::MergePolicy::Normal, messages })
    }

    /// 📑️ A finished replay's per-mutation report; a replay that converged gets the unreplayed tail's durable outcomes
    /// spliced in, so an early exit reports exactly what a full replay would.
    pub fn replay_report(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, result: &EditReplayResult<P, Mutation>) -> Result<protocol::ReplayReport, VcsError> {
        let mut report = result.report().clone();
        if let Some(length) = result.converged_at() {
            report.outcomes.extend(self.durable_outcomes(store_log, store_wire, store_cursor, length)?);
            report.worst = report.outcomes.iter().filter_map(|outcome| outcome.worst).max();
        }
        Ok(report)
    }

    //#endregion 🔖️TimeTravelReads
    //#region 📬️OneItemPublication
    /// 🧵️ Mints the base read one staged item prepares against: the store's own committed root for
    /// the first item, and the previous item's post root — leased through the same exact registry —
    /// for every later one.
    pub fn batch_item_base(&self, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, staged_root: Option<&Arc<P>>) -> Result<SnapshotRead<P>, VcsError>
    where
        P: Sync,
    {
        let Some(root) = staged_root else { return self.snapshot_read(store_owners, store_wire, store_cursor) };
        if !store_owners.snapshot_read_leases.publish_authority(store_cursor.generation, store_cursor.content_revision) {
            return Err(VcsError::ValidationFailed("staged batch read commit authority is busy or exhausted".into()));
        }
        let owner = Arc::clone(root);
        let lease = store_owners.snapshot_read_leases.try_issue(owner.clone()).map_err(|_| VcsError::ValidationFailed("staged batch read lease registry is busy, saturated, or exhausted".into()))?;
        Ok(SnapshotRead::new(owner, lease))
    }

    /// 🫳️ Keeps the genuine prepared forward and original staged/current prebase borrowed together.
    fn original_proposal_forward<'a>(&'a self, store_cursor: &StoreCursor<P, Mutation>,publication:&'a ArtifactStoreBatchPublication<P,Mutation>)->Result<(&'a Mutation,&'a P),ValueError>{
        use semio_framework_value::ValueRefusalKind;
        if publication.phase!=ArtifactStoreOneItemPublicationPhase::Preparing||publication.item_closing||publication.cancel_requested||publication.close_started||store_cursor.generation!=publication.expected_generation||store_cursor.content_revision!=publication.expected_revision||publication.stage.as_ref().is_some_and(|stage|stage.folding.is_some()){
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"foreign proposal requires the original unfurled prepared candidate and live prebase"));
        }
        let candidate=publication.preparation.as_ref().and_then(|owner|owner.prepared()).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"foreign proposal original domain candidate is not prepared"))?;
        if candidate.edit().forwards.len()!=1{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"foreign proposal requires exactly one original prepared forward"));}
        let original=&candidate.edit().forwards[0];
        let prebase=publication.stage.as_ref().and_then(|stage|stage.post.as_ref()).map_or(self.current.as_ref(),|root|root.as_ref());
        Ok((original,prebase))
    }

    /// 🌐️ Advances original borrowed foreign rows before any fold transfer or replacement of their prebase.
    pub fn advance_prepared_foreign_steps(
        &self, store_cursor: &StoreCursor<P, Mutation>, publication:&ArtifactStoreBatchPublication<P,Mutation>,
        output:&mut protocol::ForeignStepsPreparation, grant:RetainedCloneGrant,
    )->Result<RetainedCloneStep,ValueError> {
        let(original,prebase)=self.original_proposal_forward(store_cursor, publication)?;
        let source=if original.may_emit_foreign_steps(){original.foreign_step_source(prebase,output.source_index())?}else{None};
        let step=output.advance_source(source,grant)?;
        semio_framework_value::retained_clone::admit_retained_clone_progress(grant,step.progress(),"original prepared foreign row turn")?;
        Ok(step)
    }

    /// ✍️ Uses the original installed typed operation codec and caller-funded output prefix without eager encoding.
    pub fn advance_prepared_operation_wire(
        &self, store_cursor: &StoreCursor<P, Mutation>, publication:&ArtifactStoreBatchPublication<P,Mutation>,
        cursor:&mut ArtifactPreparedOperationCursor,output:&mut[u8],grant:RetainedCloneGrant,
    )->Result<ArtifactPreparedOperationProgress,ArtifactPreparedOperationError>{
        let(original,_)=self.original_proposal_forward(store_cursor, publication)?;
        let source=publication.source.as_ref().and_then(|owner|owner.operation_wire_source(original)).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"proposal bytes require their original installed typed codec issuer"))?;
        cursor.advance(source,output,grant)
    }

    //#endregion 📬️OneItemPublication
    //#region 🔖️EventSourcedHistory
    /// 📥️ Records admitted remote transitions in the log and materializes the projection they fold to — adopting `replayed`
    /// when it is their finished Report replay — or records none of them.
    async fn adopt_remote_transitions(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, transitions: Vec<semio_framework_os_spr::MutationEnvelope>, replayed: Option<EditReplayResult<P, Mutation>>) -> Result<Vec<semio_framework_os_spr::EditMessages>, VcsError> {
        let admitted: Vec<MutationId> = transitions.iter().map(|envelope| envelope.mutation_id.clone()).collect();
        for envelope in transitions {
            self.insert_transition(store_log, envelope);
        }
        let projected = match replayed {
            Some(result) => self.reproject_replayed(store_log, store_owners, store_cursor, result),
            None => self.reproject(store_log, store_owners, store_cursor).await,
        };
        projected.inspect_err(|_| store_log.envelope.transitions.retain(|known| !admitted.contains(&known.mutation_id)))
    }

    /// 📎️ Records `envelope` in the transition log at its `(hlc, id)` position.
    pub fn insert_transition(&mut self, store_log: &mut StoreLog<P, Mutation>, envelope: semio_framework_os_spr::MutationEnvelope) {
        self.fold_frontier = None;
        let key = envelope.timestamp.cmp_key();
        let position = store_log.envelope.transitions.partition_point(|known| (known.timestamp.cmp_key(), known.mutation_id.0.as_str()) < (key, envelope.mutation_id.0.as_str()));
        store_log.envelope.transitions.insert(position, envelope);
    }

    /// ♻️ Re-derives every history position from the event log (`crate::os_spr::fold_history`)
    /// and materializes the payload projection for it. The single path local commands and remote
    /// ingest converge through: the document is always the fold of its events, never a merge.
    /// Transactional: the projection is computed and every capacity its adoption needs is checked before the first fact
    /// is adopted. Answers the messages of every edit a replay re-evaluated.
    pub async fn reproject(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>) -> Result<Vec<semio_framework_os_spr::EditMessages>, VcsError> {
        self.reproject_now(store_log, store_owners, store_cursor)
    }

    /// ♻️ [`Self::reproject`]'s body, which never suspends: for the history steps that must not.
    fn reproject_now(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>) -> Result<Vec<semio_framework_os_spr::EditMessages>, VcsError> {
        self.fold_frontier = None;
        let fold = store_log.fold_live_log()?;
        let replay = self.reprojection_replay(store_log, store_cursor, &fold)?;
        self.adopt_reprojection(store_log, store_owners, store_cursor, fold, replay)
    }

    /// 🧷️ Adopts a local edit already appended as the fold's tail ([`Self::folded_tail`]) without folding the log: the fold
    /// keeps exactly the redo entries `actor` did not author and changes nothing else.
    fn adopt_folded_tail(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, actor: Option<&str>, key: (u64, u64, u64)) -> Result<(), VcsError> {
        let kept: Vec<String> = store_log.redo_edit_ids.iter().filter(|redo| store_log.envelope.vcs.edits.find_near(|edit| edit.id == **redo).and_then(|edit| edit.actor.as_deref()) != actor).cloned().collect();
        if kept.len() != store_log.redo_edit_ids.len() {
            self.replace_redo_edit_ids_retained(store_log, store_owners, store_cursor, kept)?;
        }
        self.fold_frontier = Some(key);
        Ok(())
    }

    /// 🔭️ The projection after the first `length` live applied edits: the head, the tail snapshot, or the nearest retained
    /// prefix — exact, since every bump evicts each entry past the first changed position ([`Self::prune_prefix_ring`]) — or
    /// the genesis, folded forward over the live order (one stride at most once the ring holds the prefix). Answers the
    /// prefixes it folded through on the ring's stride with their forwards-only digests, chained from the base's digest
    /// instead of hashing the whole history.
    pub fn live_prefix_state(&self, store_log: &StoreLog<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, length: usize) -> Result<(Arc<P>, Vec<(usize, Option<[u8; 32]>, Arc<P>)>), VcsError> {
        let applied = store_log.applied_edit_ids.len();
        if !self.current_detached && length == applied {
            return Ok((Arc::clone(&*self.current), Vec::new()));
        }
        if let Some((_, snapshot)) = self.tail_undo_cache.as_ref().filter(|(edit_id, _)| length + 1 == applied && store_log.applied_edit_ids.last() == Some(edit_id)) {
            return Ok((Arc::clone(snapshot), Vec::new()));
        }
        let (start, mut digest, mut running) = match self.prefix_ring.iter().rev().find(|entry| entry.length <= length) {
            Some(entry) => {
                #[cfg(test)]
                self.assert_prefix_entry_is_live(store_log, store_cursor, entry);
                (entry.length, entry.digest, Arc::clone(&entry.snapshot))
            }
            None => (0, forward_prefix_genesis(store_cursor.revision_accumulator.identity_digest), store_log.envelope.vcs.genesis.facts().share_snapshot()),
        };
        let stride = Self::prefix_stride(applied);
        let last_recorded = length.saturating_sub(1) / stride * stride;
        let mut recorded = Vec::new();
        for (offset, edit_id) in store_log.applied_edit_ids.iter_from(start).take(length - start).enumerate() {
            let Some(edit) = store_log.envelope.vcs.edits.find_near(|edit| edit.id == *edit_id) else {
                retire_shared_projection::<P, Mutation>(running);
                for (_, _, snapshot) in recorded {
                    retire_shared_projection::<P, Mutation>(snapshot);
                }
                return Err(VcsError::UnknownEdit(edit_id.clone()));
            };
            fold_effective_edit(&mut running, edit, &store_log.envelope.schema, &self.supersessions, usize::MAX);
            let reached = start + offset + 1;
            if reached <= last_recorded {
                digest = forward_prefix_step::<P, Mutation>(&digest, edit, &self.supersessions);
                if reached % stride == 0 {
                    recorded.push((reached, Some(digest), Arc::clone(&running)));
                }
            }
        }
        Ok((running, recorded))
    }

    /// 🧱️ Where a replay of the live order from `length` starts: the head, the tail snapshot, or the nearest retained prefix
    /// at or before `length` (exact: [`Self::prune_prefix_ring`]), else the genesis; the replay folds the rest of the prefix
    /// itself ([`EditReplay::new_after_prefix`]).
    fn live_base(&self, store_log: &StoreLog<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, length: usize) -> (usize, Arc<P>) {
        let applied = store_log.applied_edit_ids.len();
        if !self.current_detached && length == applied {
            return (applied, Arc::clone(&*self.current));
        }
        if let Some((_, snapshot)) = self.tail_undo_cache.as_ref().filter(|(edit_id, _)| applied > 0 && length + 1 >= applied && store_log.applied_edit_ids.last() == Some(edit_id)) {
            return (applied - 1, Arc::clone(snapshot));
        }
        match self.prefix_ring.iter().rev().find(|entry| entry.length <= length) {
            Some(entry) => {
                #[cfg(test)]
                self.assert_prefix_entry_is_live(store_log, store_cursor, entry);
                (entry.length, Arc::clone(&entry.snapshot))
            }
            None => (0, store_log.envelope.vcs.genesis.facts().share_snapshot()),
        }
    }

    /// 🔬️ Test-build oracle of [`Self::live_prefix_state`]'s trust in the pruned ring: `entry` names the live prefix of its
    /// length (checked outside the hot-path census).
    #[cfg(test)]
    fn assert_prefix_entry_is_live(&self, store_log: &StoreLog<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, entry: &PrefixSnapshot<P>) {
        let saved = hot_path_census::take();
        let digests = forward_prefix_digests::<P, Mutation, _, _>(store_cursor.revision_accumulator.identity_digest, store_log.applied_edit_ids.iter().take(entry.length), &store_log.envelope.vcs.edits, &self.supersessions).expect("the live prefix digests");
        hot_path_census::count(|census| *census = saved);
        assert_eq!(digests.last(), Some(&entry.digest), "a pruned prefix ring holds only entries of the live prefix (length {})", entry.length);
    }

    /// 🩺️ The Report replay `fold` needs: none when its applied order and effective inputs are live or it only drops applied
    /// edits from the tail (every remaining edit keeps its prefix, so its durable messages and inverses stay
    /// replay-consistent), otherwise a replay from the first position whose order or effective input differs — an interior
    /// undo, redo or checkout rewrites every downstream edit's messages and inverses exactly as a supersession does.
    /// Synchronous: one command or ingest replays the applied history in place. A remote change under
    /// [`Self::defer_remote_replays`] and a local step under [`Self::defer_local_replays`] replay resumably instead
    /// ([`Self::step_reprojection`]).
    fn reprojection_replay(&self, store_log: &StoreLog<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, fold: &semio_framework_os_spr::HistoryFold) -> Result<Option<EditReplayResult<P, Mutation>>, VcsError> {
        if fold.supersessions == *self.supersessions && store_log.applied_edit_ids.starts_with(&fold.applied) {
            return Ok(None);
        }
        let (from, horizon) = self.replay_window(store_log, &fold.applied, &fold.supersessions)?;
        self.report_replay(store_log, store_cursor, &fold.applied, from, fold.supersessions.clone(), horizon, Self::prefix_stride(fold.applied.len()))?.run(&store_log.envelope.vcs.edits).map(Some)
    }

    /// 🚀️ Adopts an already finished Report replay of exactly the folded event log instead of replaying again — the fast path
    /// of a supersession whose replay was computed before it was authored (a dry run, or a time-travel session). A replay of
    /// another order or other effective inputs is dropped and the log is replayed as usual.
    pub fn reproject_replayed(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, result: EditReplayResult<P, Mutation>) -> Result<Vec<semio_framework_os_spr::EditMessages>, VcsError> {
        self.fold_frontier = None;
        let fold = match store_log.fold_live_log() {
            Ok(fold) => fold,
            Err(error) => {
                let _ = store_owners.retire_replay_result(result);
                return Err(error);
            }
        };
        if fold.applied != result.order || !same_effective_inputs(&fold.supersessions, &result.supersessions) {
            store_owners.retire_replay_result(result)?;
            let replay = self.reprojection_replay(store_log, store_cursor, &fold)?;
            return self.adopt_reprojection(store_log, store_owners, store_cursor, fold, replay);
        }
        self.adopt_reprojection(store_log, store_owners, store_cursor, fold, Some(result))
    }

    /// 🧷️ Adopts `fold` and the replay it needed ([`Self::reprojection_replay`]): a tail-only removal re-projects through
    /// the tail snapshot or the nearest retained prefix, a replay publishes its projection, inverses and messages. Nothing
    /// is adopted unless [`Self::preflight_reprojection`] admits all of it.
    fn adopt_reprojection(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, fold: semio_framework_os_spr::HistoryFold, replay: Option<EditReplayResult<P, Mutation>>) -> Result<Vec<semio_framework_os_spr::EditMessages>, VcsError> {
        let prefix = match &replay {
            None if fold.applied != *store_log.applied_edit_ids => Some(self.live_prefix_state(store_log, store_cursor, fold.applied.len())?),
            _ => None,
        };
        let recorded = prefix.as_ref().map_or(0, |(_, recorded)| recorded.len()) + replay.as_ref().map_or(0, |result| result.recorded.len());
        if let Err(error) = store_owners.preflight_reprojection(store_log, &fold, replay.as_ref(), recorded) {
            if let Some((current, recorded)) = prefix {
                retire_shared_projection::<P, Mutation>(current);
                for (_, _, snapshot) in recorded {
                    retire_shared_projection::<P, Mutation>(snapshot);
                }
            }
            if let Some(result) = replay {
                let _ = store_owners.retire_replay_result(result);
            }
            return Err(error);
        }
        if let Err(error) = store_owners.adopt_history_facts(store_log, &fold) {
            if let Some((current, recorded)) = prefix {
                retire_shared_projection::<P, Mutation>(current);
                for (_, _, snapshot) in recorded {
                    retire_shared_projection::<P, Mutation>(snapshot);
                }
            }
            if let Some(result) = replay {
                let _ = store_owners.retire_replay_result(result);
            }
            return Err(error);
        }
        let semio_framework_os_spr::HistoryFold { applied, redo, checkpoint, alternative, supersessions, .. } = fold;
        let replayed = match (replay, prefix) {
            (Some(result), _) => self.adopt_replayed_projection(store_log, store_owners, store_cursor, applied, supersessions, result)?,
            (None, Some((current, recorded))) => {
                self.replace_current_retained(store_log, store_owners, current)?;
                self.replace_tail_undo_cache_retained(store_log, store_owners, None)?;
                self.replace_applied_edit_ids_retained(store_log, store_owners, store_cursor, applied)?;
                self.admit_prefix_snapshots(store_log, store_owners, recorded)?;
                Vec::new()
            }
            (None, None) => Vec::new(),
        };
        self.adopt_history_positions(store_log, store_owners, store_cursor, redo, checkpoint, alternative)?;
        self.fold_frontier = Some(store_log.event_frontier());
        Ok(replayed)
    }

    /// 🪄️ Publishes a finished Report replay of `applied` under `supersessions`. A replay that only appended one edit to the
    /// live tail keeps the live head as that edit's tail snapshot (an undo answers from it again).
    fn adopt_replayed_projection(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, applied: Vec<String>, supersessions: EffectiveSupersessions, mut result: EditReplayResult<P, Mutation>) -> Result<Vec<semio_framework_os_spr::EditMessages>, VcsError> {
        let from = result.from;
        let appended = from == store_log.applied_edit_ids.len() && from + 1 == applied.len() && store_log.applied_edit_ids.is_prefix_of_slice(&applied) && supersessions == *self.supersessions && !self.current_detached;
        let tail = appended.then(|| (applied[from].clone(), Arc::clone(&*self.current)));
        let recorded = result.take_recorded();
        let replayed = self.adopt_report_replay(store_log, store_owners, store_cursor, result, tail)?;
        self.replace_applied_edit_ids_retained(store_log, store_owners, store_cursor, applied)?;
        self.replace_supersessions_retained(store_owners, supersessions)?;
        self.prefix_ring_dirty_from = self.prefix_ring_dirty_from.min(from);
        self.retain_prefix_snapshots(store_log, store_owners, store_cursor, recorded)?;
        store_cursor.revision_dirty_from = Some(store_cursor.revision_dirty_from.map_or(from, |dirty| dirty.min(from)));
        store_cursor.last_projection_cause = Some(ArtifactProjectionCause::Replay);
        Ok(replayed)
    }

    /// 📌️ Adopts the fold's redo stack, checkpoint and active alternative.
    fn adopt_history_positions(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, redo: Vec<String>, checkpoint: Option<String>, alternative: Option<String>) -> Result<(), VcsError> {
        if redo != *store_log.redo_edit_ids {
            self.replace_redo_edit_ids_retained(store_log, store_owners, store_cursor, redo)?;
        }
        if checkpoint != *store_log.current_checkpoint_id {
            self.replace_current_checkpoint_retained(store_log, store_owners, checkpoint)?;
        }
        store_log.envelope.active_alternative_id = alternative;
        Ok(())
    }

    /// 🪟️ Where a replay of `applied` under `supersessions` must start (the first position whose order or effective
    /// input differs from the live projection) and, when the order is unchanged, the prefix length from which it may
    /// converge onto the live projection (just past the last changed input).
    fn replay_window(&self, store_log: &StoreLog<P, Mutation>, applied: &[String], supersessions: &EffectiveSupersessions) -> Result<(usize, Option<usize>), VcsError> {
        let changed = superseded_positions::<P, Mutation>(applied, &store_log.envelope.vcs.edits, &self.supersessions, supersessions)?;
        let common = applied.iter().zip(store_log.applied_edit_ids.iter()).take_while(|(next, live)| next == live).count();
        let reordered = common != applied.len() || common != store_log.applied_edit_ids.len();
        let from = changed.first().copied().unwrap_or(applied.len()).min(if reordered { common } else { applied.len() });
        let horizon = if reordered { None } else { changed.last().map(|position| position + 1) };
        Ok((from, horizon))
    }

    /// 📽️ A Report replay of `applied[from..]` under `supersessions`, starting from the live projection nearest before
    /// `from` ([`Self::live_base`]; every caller's `from` lies at or before the first position where `applied` or
    /// `supersessions` leave the live history, so the live prefix is the replayed one). With snapshot equality enabled and the live order unchanged it may converge onto the live
    /// projection from prefix length `horizon` on.
    fn report_replay(&self, store_log: &StoreLog<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, applied: &[String], from: usize, supersessions: EffectiveSupersessions, horizon: Option<usize>, stride: usize) -> Result<EditReplay<P, Mutation>, VcsError> {
        let (start, base) = self.live_base(store_log, store_cursor, from);
        let replay = EditReplay::new_after_prefix(ReplayMode::Report, start, base, applied.to_vec(), from, &store_log.envelope.schema, supersessions, &store_log.envelope.vcs.edits)?.with_base(store_cursor.generation, store_cursor.content_revision).recording(stride, Vec::new());
        let convergence = match (horizon, self.snapshot_equality) {
            (Some(horizon), Some(equal)) if store_log.applied_edit_ids.eq_slice(applied) => match self.convergence_checkpoints(store_log, store_cursor, horizon, equal) {
                Ok(checkpoints) => Some(checkpoints),
                Err(error) => {
                    let _ = replay.retire_unscheduled();
                    return Err(error);
                }
            },
            _ => None,
        };
        Ok(replay.with_convergence(convergence))
    }

    /// 🚩️ The live projection's retained states from prefix length `horizon` on — the pruned ring's entries (exact:
    /// [`Self::prune_prefix_ring`]), the tail snapshot and the head — as a convergence probe. Every state is an alias of a root this
    /// store keeps, so the probe never owns a last reference.
    fn convergence_checkpoints(&self, store_log: &StoreLog<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, horizon: usize, equal: fn(&P, &P) -> bool) -> Result<ReplayConvergence<P>, VcsError> {
        let length = store_log.applied_edit_ids.len();
        let mut checkpoints: Vec<(usize, Arc<P>)> = self
            .prefix_ring
            .iter()
            .filter(|entry| entry.length >= horizon && entry.length < length)
            .map(|entry| {
                #[cfg(test)]
                self.assert_prefix_entry_is_live(store_log, store_cursor, entry);
                (entry.length, Arc::clone(&entry.snapshot))
            })
            .collect();
        if let Some((edit_id, snapshot)) = self.tail_undo_cache.as_ref() {
            if length > horizon && store_log.applied_edit_ids.last() == Some(edit_id) && !checkpoints.iter().any(|(known, _)| *known == length - 1) {
                checkpoints.push((length - 1, Arc::clone(snapshot)));
            }
        }
        checkpoints.push((length, Arc::clone(&self.current)));
        checkpoints.sort_by_key(|(prefix, _)| *prefix);
        Ok(ReplayConvergence { horizon, checkpoints, equal })
    }

    /// 🔭️ The projection after `applied[..from]` under `supersessions` ([`Self::prefix_state_recorded`]).
    fn prefix_state(&self, store_log: &StoreLog<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, applied: &[String], from: usize, supersessions: &EffectiveSupersessions) -> Result<Arc<P>, VcsError> {
        Ok(self.prefix_state_recorded(store_log, store_cursor, applied, from, supersessions, usize::MAX)?.0)
    }

    /// 🔬️ The projection after `applied[..from]` under `supersessions`: the live head, the tail snapshot or the nearest
    /// retained prefix whose digest names the same prefix, folded forward — the genesis when none lies before `from` —
    /// with the projections it folded through at every multiple of `stride`.
    fn prefix_state_recorded(&self, store_log: &StoreLog<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, applied: &[String], from: usize, supersessions: &EffectiveSupersessions, stride: usize) -> Result<(Arc<P>, Vec<(usize, Arc<P>)>), VcsError> {
        let identity = store_cursor.revision_accumulator.identity_digest;
        let digests = forward_prefix_digests::<P, Mutation, _, _>(identity, &applied[..from], &store_log.envelope.vcs.edits, supersessions)?;
        let live = forward_prefix_digests::<P, Mutation, _, _>(identity, &*store_log.applied_edit_ids, &store_log.envelope.vcs.edits, &self.supersessions)?;
        let length = store_log.applied_edit_ids.len();
        let mut candidates: Vec<(usize, [u8; 32], &Arc<P>)> = self.prefix_ring.iter().map(|entry| (entry.length, entry.digest, &entry.snapshot)).collect();
        if !self.current_detached {
            candidates.push((length, live[length], &*self.current));
        }
        if let Some((edit_id, snapshot)) = self.tail_undo_cache.as_ref() {
            if store_log.applied_edit_ids.last() == Some(edit_id) {
                candidates.push((length - 1, live[length - 1], snapshot));
            }
        }
        let best = candidates.into_iter().filter(|(prefix, digest, _)| *prefix <= from && digests.get(*prefix) == Some(digest)).max_by_key(|(prefix, _, _)| *prefix).map(|(prefix, _, snapshot)| (prefix, snapshot));
        let (length, base) = match best {
            Some((length, snapshot)) => (length, Arc::clone(snapshot)),
            None => (0, store_log.envelope.vcs.genesis.facts().share_snapshot()),
        };
        if length == from {
            return Ok((base, Vec::new()));
        }
        fold_recorded::<P, Mutation>(base, applied, length, from, &store_log.envelope.vcs.edits, &store_log.envelope.schema, supersessions, stride)
    }

    /// 📣️ Publishes a finished Report replay: every replayed edit gets its rebased inverse and its messages (bounded by
    /// the replay itself, [`bound_edit_messages`]), `tail` becomes the tail snapshot and the reached projection becomes
    /// `current`.
    fn adopt_report_replay(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, mut result: EditReplayResult<P, Mutation>, tail: Option<(String, Arc<P>)>) -> Result<Vec<semio_framework_os_spr::EditMessages>, VcsError> {
        let taken = store_cursor.take_report_replay(store_log, &mut result);
        let retired = store_owners.retire_replay_result(result);
        let (replayed, state) = taken?;
        if let Err(error) = retired {
            retire_shared_projection::<P, Mutation>(state);
            return Err(error);
        }
        self.replace_tail_undo_cache_retained(store_log, store_owners, tail)?;
        self.replace_current_retained(store_log, store_owners, state)?;
        for entry in &replayed {
            store_owners.replace_edit_messages(store_log, &entry.edit_id, entry.messages.clone())?;
        }
        Ok(replayed)
    }

    /// 🔄️ Replaces the supersessions `current` is projected with; the displaced map retires one byte grant at a time.
    pub fn replace_supersessions_retained(&mut self, store_owners: &mut StoreOwners<P, Mutation>, next: EffectiveSupersessions) -> Result<(), VcsError> {
        if *self.supersessions == next {
            return Ok(());
        }
        store_owners.displaced_retirements.reserve(usize::from(!self.supersessions.is_empty()))?;
        self.fold_frontier = None;
        let previous = std::mem::replace(&mut *self.supersessions, next);
        if !previous.is_empty() {
            store_owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreSupersessionRetirement::new(previous)));
        }
        Ok(())
    }

    /// 📏️ The prefix-ring stride for `length` applied edits: [`Self::prefix_ring_capacity`] entries span them all.
    fn prefix_stride(length: usize) -> usize {
        length.div_ceil(Self::prefix_ring_capacity(length)).max(1)
    }

    /// 📐️ How many prefix projections a history of `length` applied edits retains: about √length, so a `state_before` or a
    /// replay start folds at most one stride of about √length edits while the retained projections grow only with the square
    /// root — never fewer than [`PREFIX_SNAPSHOT_RING_FLOOR`], never more than [`PREFIX_SNAPSHOT_RING_CAPACITY`].
    fn prefix_ring_capacity(length: usize) -> usize {
        length.isqrt().clamp(PREFIX_SNAPSHOT_RING_FLOOR, PREFIX_SNAPSHOT_RING_CAPACITY)
    }

    /// 🗃️ Admits prefix projections `(length, projection)` of the live applied history into the ring at the current
    /// stride, evicting whatever a stale digest or a coarser stride no longer justifies. Without a snapshot retirement
    /// factory nothing is retained; a refused admission retires every offered projection.
    fn retain_prefix_snapshots(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, snapshots: Vec<(usize, Arc<P>)>) -> Result<(), VcsError> {
        if snapshots.is_empty() || (*store_owners.snapshot_retirement_factory).is_none() {
            for (_, snapshot) in snapshots {
                retire_shared_projection::<P, Mutation>(snapshot);
            }
            return Ok(());
        }
        match forward_prefix_digests::<P, Mutation, _, _>(store_cursor.revision_accumulator.identity_digest, &*store_log.applied_edit_ids, &store_log.envelope.vcs.edits, &self.supersessions) {
            Ok(digests) => self.admit_prefix_snapshots(store_log, store_owners, snapshots.into_iter().map(|(length, snapshot)| (length, digests.get(length).copied(), snapshot)).collect()),
            Err(error) => {
                for (_, snapshot) in snapshots {
                    retire_shared_projection::<P, Mutation>(snapshot);
                }
                Err(error)
            }
        }
    }

    /// 🗃️ Admits prefix projections `(length, digest, projection)` of the live applied history whose forwards-only digests
    /// the caller already holds ([`Self::retain_prefix_snapshots`], [`Self::live_prefix_state`]); one without a digest, off
    /// the stride, or offered without a snapshot retirement factory retires at once.
    pub fn admit_prefix_snapshots(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, snapshots: Vec<(usize, Option<[u8; 32]>, Arc<P>)>) -> Result<(), VcsError> {
        if snapshots.is_empty() {
            return Ok(());
        }
        let admitted = match self.prune_prefix_ring(store_owners) {
            Err(error) => Err(error),
            Ok(()) => match (*store_owners.snapshot_retirement_factory).clone() {
                Some(factory) => store_owners.displaced_retirements.reserve(2 * snapshots.len()).map(|()| Some(factory)),
                None => Ok(None),
            },
        };
        let factory = match admitted {
            Ok(Some(factory)) => factory,
            refused => {
                for (_, _, snapshot) in snapshots {
                    retire_shared_projection::<P, Mutation>(snapshot);
                }
                return refused.map(|_| ());
            }
        };
        let stride = Self::prefix_stride(store_log.applied_edit_ids.len());
        for (length, digest, snapshot) in snapshots {
            let Some(digest) = digest.filter(|_| length != 0 && length % stride == 0) else {
                retire_shared_projection::<P, Mutation>(snapshot);
                continue;
            };
            if let Some(position) = self.prefix_ring.iter().position(|entry| entry.length == length) {
                if self.prefix_ring[position].digest == digest {
                    retire_shared_projection::<P, Mutation>(snapshot);
                    continue;
                }
                self.evict_prefix_snapshot_reserved(store_owners, position, &factory);
            }
            if self.prefix_ring.len() >= Self::prefix_ring_capacity(store_log.applied_edit_ids.len()) {
                let victim = self.prefix_ring.iter().position(|entry| entry.length % stride != 0).unwrap_or(0);
                self.evict_prefix_snapshot_reserved(store_owners, victim, &factory);
            }
            let position = self.prefix_ring.partition_point(|entry| entry.length < length);
            self.prefix_ring.insert(position, PrefixSnapshot { length, digest, snapshot });
        }
        Ok(())
    }

    /// 🧹️ Evicts ring entry `position` into a displaced slot the caller reserved: an alias another owner holds only
    /// drops its count, the last owner — decided atomically by [`Arc::try_unwrap`] — retires through `factory`.
    fn evict_prefix_snapshot_reserved(&mut self, store_owners: &mut StoreOwners<P, Mutation>, position: usize, factory: &Arc<dyn SnapshotRetirementFactory<P>>) {
        let PrefixSnapshot { snapshot, .. } = self.prefix_ring.remove(position);
        if let Ok(unique) = Arc::try_unwrap(snapshot) {
            store_owners.displaced_retirements.push_reserved(ArtifactStoreDisplacedSnapshotRetirement::boxed(Arc::new(unique), Arc::clone(factory)));
        }
    }

    /// 🪓️ Evicts every ring entry past [`Self::prefix_ring_dirty_from`] — the first applied position whose id, forwards or
    /// effective input changed since the last prune — in O(ring); every reader still checks an entry's digest before folding
    /// from it.
    fn prune_prefix_ring(&mut self, store_owners: &mut StoreOwners<P, Mutation>) -> Result<(), VcsError> {
        let dirty = std::mem::replace(&mut self.prefix_ring_dirty_from, usize::MAX);
        let stale = self.prefix_ring.len() - self.prefix_ring.partition_point(|entry| entry.length <= dirty);
        if stale == 0 {
            return Ok(());
        }
        store_owners.displaced_retirements.reserve(stale)?;
        let factory = (*store_owners.snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store prefix ring eviction requires its exact snapshot retirement factory".into()))?;
        for _ in 0..stale {
            self.evict_prefix_snapshot_reserved(store_owners, self.prefix_ring.len() - 1, &factory);
        }
        Ok(())
    }

    /// 🧼️ Evicts every ring entry (a reload adopts a different history).
    pub fn clear_prefix_ring(&mut self, store_owners: &mut StoreOwners<P, Mutation>) -> Result<(), VcsError> {
        if self.prefix_ring.is_empty() {
            return Ok(());
        }
        store_owners.displaced_retirements.reserve(self.prefix_ring.len())?;
        let factory = (*store_owners.snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store prefix ring eviction requires its exact snapshot retirement factory".into()))?;
        while !self.prefix_ring.is_empty() {
            self.evict_prefix_snapshot_reserved(store_owners, self.prefix_ring.len() - 1, &factory);
        }
        Ok(())
    }

    //#endregion 🔖️EventSourcedHistory
    //#region 🔖️ToolTransactions
    /// 🪂️ Takes the open tool transaction's edit off the applied tail before a remote merge: the ledger, the message ledger
    /// and the projection return to the history without it.
    pub async fn lift_open_transaction(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>) -> Result<Option<(OpenToolTransaction, Edit<Mutation>)>, VcsError> {
        let Some(open) = store_log.envelope.open_transaction.take() else {
            return Ok(None);
        };
        let mut removed = store_log.envelope.vcs.edits.extract_if(|edit| edit.id == open.edit_id).map_err(|fault| VcsError::ValidationFailed(format!("the open transaction edit cannot leave its ledger: {fault:?}")))?;
        let edit = removed.pop().ok_or_else(|| VcsError::UnknownEdit(open.edit_id.clone()))?;
        retire_scratch_edits::<P, Mutation>(removed);
        store_owners.replace_edit_messages(store_log, &open.edit_id, Vec::new())?;
        self.reproject(store_log, store_owners, store_cursor).await?;
        Ok(Some((open, edit)))
    }

    //#endregion 🔖️ToolTransactions
    //#region 🔖️Supersede
    /// 🧪️ The Report replay of everything `fold` changes against the live projection, driven to completion, touching nothing.
    /// Synchronous: the supersede command API (agents, text, hub) gets its verdict inside the dispatch. Under
    /// [`Self::defer_local_replays`] [`Self::author_supersession`] steps the same replay across turns instead.
    pub fn dry_run(&self, store_log: &StoreLog<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, fold: &semio_framework_os_spr::HistoryFold) -> Result<EditReplayResult<P, Mutation>, VcsError> {
        let (from, horizon) = self.replay_window(store_log, &fold.applied, &fold.supersessions)?;
        self.report_replay(store_log, store_cursor, &fold.applied, from, fold.supersessions.clone(), horizon, Self::prefix_stride(fold.applied.len()))?.run(&store_log.envelope.vcs.edits)
    }

    /// 🛃️ Validates and encodes supersede inputs: every target is an applied operation and every input obeys the supersede
    /// law ([`admit_replacement`]). Also answers the conflict target the transition envelope declares — the longest address
    /// every superseded and replacing operation shares. Every replacement retires once encoded, on success and refusal alike.
    pub fn supersede_inputs(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, inputs: Vec<SupersedeInput<Mutation>>) -> Result<(Vec<protocol::SupersededInput>, Vec<String>), VcsError> {
        let mut applied: HashMap<MutationId, &Mutation> = HashMap::new();
        let mut failure = None;
        for edit_id in store_log.applied_edit_ids.iter() {
            match store_log.envelope.vcs.edits.find_near(|edit| edit.id == *edit_id) {
                Some(edit) => applied.extend(semio_framework_os_spr::mutation_ids_for_edit::<P, Mutation>(edit).into_iter().zip(edit.forwards.iter())),
                None => {
                    failure = Some(VcsError::UnknownEdit(edit_id.clone()));
                    break;
                }
            }
        }
        let mut encoded = Vec::with_capacity(inputs.len());
        let mut address: Option<Vec<String>> = None;
        for SupersedeInput { target, replacement } in inputs {
            let checked = match (failure.is_some(), applied.get(&target)) {
                (true, _) => None,
                (false, None) => Some(Err(VcsError::ValidationFailed(format!("supersede target {} is not an applied operation", target.0)))),
                (false, Some(original)) => Some(self.supersede_input(store_log, store_wire, store_cursor, &target, original, replacement.as_ref())),
            };
            if let Some(replacement) = replacement {
                self::Mutation::<P>::retire_cold(replacement);
            }
            match checked {
                Some(Ok((input, written))) => {
                    address = Some(match address {
                        Some(known) => common_address(&known, &written),
                        None => written,
                    });
                    encoded.push(input);
                }
                Some(Err(error)) => failure = Some(error),
                None => {}
            }
        }
        match failure {
            Some(error) => Err(error),
            None => Ok((encoded, address.unwrap_or_default())),
        }
    }

    /// 🧩️ One supersede input: its encoded replacement, held to the supersede law ([`admit_replacement`]) every replica
    /// folds it under, and the address it and the original operation share. An operation of a cross-artifact unit takes no
    /// supersession in one document alone ([`Self::unit_id`]).
    fn supersede_input(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, target: &MutationId, original: &Mutation, replacement: Option<&Mutation>) -> Result<(protocol::SupersededInput, Vec<String>), VcsError> {
        if let Some(unit) = self.unit_id(store_log, store_wire, store_cursor, target)? {
            return Err(VcsError::UnitSpansDocuments { mutation_id: target.0.clone(), unit });
        }
        let replacement = match replacement {
            None => protocol::InputReplacement::Withdrawn,
            Some(operation) => protocol::InputReplacement::Input { schema: store_log.envelope.schema.clone(), payload: operation.encode_op().map_err(|error| VcsError::Serialize(error.to_string()))? },
        };
        let admitted = admit_replacement::<P, Mutation>(original, &replacement, &store_log.envelope.schema).map_err(|reason| VcsError::ValidationFailed(format!("the supersession of {} {reason}", target.0)))?;
        let mut address = original.conflict_target();
        if let Some(admitted) = admitted {
            address = common_address(&address, &admitted.conflict_target());
            self::Mutation::<P>::retire_cold(admitted);
        }
        Ok((protocol::SupersededInput { target: target.clone(), replacement }, address))
    }

    /// ✒️ Draft inputs a time-travel session previewed, validated exactly like typed supersede inputs.
    pub fn draft_inputs(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, drafts: &protocol::HistoryInputDrafts) -> Result<(Vec<protocol::SupersededInput>, Vec<String>), VcsError> {
        let mut typed = Vec::with_capacity(drafts.len());
        let mut failure = None;
        for (target, replacement) in drafts {
            let decoded = match replacement {
                protocol::InputReplacement::Withdrawn => Ok(None),
                protocol::InputReplacement::Input { schema, payload } if *schema == store_log.envelope.schema => Mutation::decode_op(payload).map(Some).map_err(|error| VcsError::Deserialize(format!("draft of {} does not decode: {error}", target.0))),
                protocol::InputReplacement::Input { schema, .. } => Err(VcsError::ValidationFailed(format!("draft of {} names schema {schema}, not {}", target.0, store_log.envelope.schema))),
            };
            match decoded {
                Ok(replacement) => typed.push(SupersedeInput { target: target.clone(), replacement }),
                Err(error) => {
                    failure.get_or_insert(error);
                }
            }
        }
        if let Some(error) = failure {
            retire_supersede_inputs::<P, Mutation>(typed);
            return Err(error);
        }
        self.supersede_inputs(store_log, store_wire, store_cursor, typed)
    }

    //#endregion 🔖️Supersede
    pub fn snapshot_json(&self, store_wire: &StoreWire<P, Mutation>) -> Result<String, VcsError> {
        let snapshot = self.snapshot(store_wire)?;
        Ok(semio_framework_pack_json::to_json_string(&snapshot))
    }

    fn take_candidate_current(&mut self) -> Arc<P> {
        assert!(!self.current_detached, "resolution candidate snapshot authority was already detached");
        self.current_detached = true;
        unsafe { std::mem::ManuallyDrop::take(&mut self.current) }
    }

    /// ➖️ Pops the applied tail; its mirrors and the prefix ring are stale past the new length.
    pub fn pop_applied_edit_id(&mut self, store_log: &mut StoreLog<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>) -> Option<String> {
        let edit_id = store_log.applied_edit_ids.pop()?;
        let length = store_log.applied_edit_ids.len();
        store_cursor.mirror_dirt.applied = store_cursor.mirror_dirt.applied.min(length);
        self.prefix_ring_dirty_from = self.prefix_ring_dirty_from.min(length);
        Some(edit_id)
    }
}

impl<P, Mutation> StoreReplay<P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    /// 🔄️ Invalidates projections after a verified replay that did not otherwise change history.
    pub fn invalidate_after_replay(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>) -> Result<ArtifactProjectionInvalidation, VcsError> {
        self.invalidate_projections(store_log, store_owners, store_wire, store_cursor, store_live, ArtifactProjectionCause::Replay)
    }

    /// 🛂️ Invalidates projections after a policy change outside semantic event history.
    pub fn invalidate_after_policy_change(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>) -> Result<ArtifactProjectionInvalidation, VcsError> {
        self.invalidate_projections(store_log, store_owners, store_wire, store_cursor, store_live, ArtifactProjectionCause::PolicyChange)
    }

    /// 🔗️ Invalidates projections after an external resource changed beneath the artifact.
    pub fn invalidate_after_external_resource_change(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>) -> Result<ArtifactProjectionInvalidation, VcsError> {
        self.invalidate_projections(store_log, store_owners, store_wire, store_cursor, store_live, ArtifactProjectionCause::ExternalResourceChange)
    }

    pub fn invalidate_projections(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, cause: ArtifactProjectionCause) -> Result<ArtifactProjectionInvalidation, VcsError> {
        store_wire.ensure_durable_group_idle()?;
        self.bump(store_log, store_owners, store_cursor, store_live)?;
        store_cursor.last_projection_cause = Some(cause);
        Ok(ArtifactProjectionInvalidation { cause, stamp: store_cursor.projection_stamp(store_log, store_wire) })
    }

    fn pending_report_owner_slots(value: &PendingCommandReport) -> usize {
        usize::from(
            value.edit_ids.as_ref().is_some_and(|ids| !ids.is_empty() || ids.capacity() != 0) || !value.messages.is_empty() || value.messages.capacity() != 0 || value.worst.is_some() || !value.outbound.is_empty() || value.outbound.capacity() != 0,
        )
    }

    fn push_pending_report_replacement_reserved(&mut self, store_owners: &mut StoreOwners<P, Mutation>, next: PendingCommandReport) {
        let previous = std::mem::replace(&mut *self.pending_report, next);
        if Self::pending_report_owner_slots(&previous) != 0 {
            store_owners.displaced_retirements.push_reserved(Box::new(ArtifactStorePendingReportRetirement::new(previous)));
        }
    }

    pub fn replace_pending_report_retained(&mut self, store_owners: &mut StoreOwners<P, Mutation>, next: PendingCommandReport) -> Result<(), VcsError> {
        store_owners.displaced_retirements.reserve(Self::pending_report_owner_slots(&self.pending_report))?;
        self.push_pending_report_replacement_reserved(store_owners, next);
        Ok(())
    }

    /// 🔀️ Folds `order[k..]` from `base`, recomputing each op's `diff` against the shifted state (so
    /// its inverse is rebased and its messages collected) and deciding accept-vs-quarantine PER EDIT
    /// rather than atomically over the whole suffix (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-
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
    /// The loop itself is the [`EditReplay`] stepper in `Merge(policy)` mode, driven to completion here; every operation
    /// reads its effective input under `supersessions`.
    pub fn replay_suffix_partitioned(
        base: Arc<P>,
        order: &[String],
        k: usize,
        edits: &impl ReplayEdits<Mutation>,
        schema: &str,
        supersessions: &EffectiveSupersessions,
        policy: semio_framework_os_spr::MergePolicy,
    ) -> Result<EditReplayResult<P, Mutation>, VcsError> {
        EditReplay::new(ReplayMode::Merge(policy), base, order.to_vec(), k, schema, supersessions.clone(), edits)?.run(edits)
    }

    //#region 📬️OneItemPublication
    /// 📤️ Announces the ONE edit an outbound batched publication just made visible as ONE
    /// `BackboneMessage::Mutations` batch. `false` when the publication is not an outbound one awaiting
    /// its announcement; the ACK becomes admissible only after this returns `true`.
    pub async fn flush_published_apply_batch(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, publication: &mut ArtifactStoreBatchPublication<P, Mutation>) -> Result<bool, VcsError> {
        if publication.phase != ArtifactStoreOneItemPublicationPhase::AwaitingAck || !publication.outbound {
            return Ok(false);
        }
        publication.outbound = false;
        self.flush_apply_outbound(store_log, store_owners, store_wire, publication.announce_from, publication.announce_items).await?;
        Ok(true)
    }

    //#endregion 📬️OneItemPublication
    //#region 🔖️EventSourcedHistory
    /// 📤️ Marks locally authored `operations` applied in the causal graph, stamps each with the
    /// foreign operation its author had observed (advisory, never an ordering constraint) and queues
    /// them for this command's outbound announcement. Announcement happens when the command authors
    /// them, so an edit made during a connection shortage names what its author saw THEN.
    fn announce_operations(&mut self, store_log: &mut StoreLog<P, Mutation>, mut operations: Vec<semio_framework_os_spr::MutationEnvelope>) -> Result<(), VcsError> {
        for operation in &mut operations {
            if operation.observed.is_none() {
                operation.observed = store_log.observed_foreign_operation(&operation.actor.0);
            }
        }
        for operation in &operations {
            match store_log.dag.seed_applied(operation.mutation_id.clone()) {
                Ok(()) => {}
                Err(rejected) if rejected.error == semio_framework_os_spr::MutationDagError::Duplicate => {}
                Err(rejected) => return Err(VcsError::ValidationFailed(rejected.error.to_string())),
            }
        }
        self.pending_report.outbound.extend(operations);
        Ok(())
    }

    /// 📥️ Admits remote history transitions the causal graph released: known ones must repeat
    /// their established payload, new ones join the log, and the projection follows the fold.
    pub async fn admit_remote_transitions(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, transitions: Vec<semio_framework_os_spr::MutationEnvelope>) -> Result<Vec<semio_framework_os_spr::EditMessages>, VcsError> {
        let mut fresh: Vec<semio_framework_os_spr::MutationEnvelope> = Vec::new();
        for envelope in transitions {
            store_log.admit_history_shape(std::slice::from_ref(&envelope))?;
            let pending = self.pending_reprojection.as_ref().map_or(&[][..], |pending| pending.transitions.as_slice());
            if let Some(known) = store_log.envelope.transitions.iter().chain(pending).chain(fresh.iter()).find(|known| known.mutation_id == envelope.mutation_id) {
                if !StoreLog::<P, Mutation>::same_operation_identity_and_payload(known, &envelope) {
                    return Err(VcsError::ValidationFailed(format!("remote transition {} conflicts with its established payload", envelope.mutation_id.0)));
                }
                continue;
            }
            store_log.clock.merge(&envelope.timestamp);
            fresh.push(envelope);
        }
        if self.pending_reprojection.is_none() && (self.replay_budget.is_none() || fresh.is_empty()) {
            return store_live.adopt_remote_transitions(store_log, store_owners, store_cursor, fresh, None).await;
        }
        let mut pending = self.pending_reprojection.take().unwrap_or_else(PendingReprojection::empty);
        let known = pending.transitions.len();
        pending.transitions.extend(fresh);
        if let Err(error) = store_log.pending_fold(&pending) {
            pending.transitions.truncate(known);
            self.pending_reprojection = (!pending.transitions.is_empty() || pending.local.is_some()).then_some(pending);
            return Err(error);
        }
        store_owners.retire_pending_replay(pending.replay.take())?;
        self.pending_reprojection = Some(pending);
        match self.advance_reprojection(store_log, store_owners, store_wire, store_cursor, store_live, None).await? {
            Some(ReprojectionAdvance::Adopted(replayed)) => Ok(replayed),
            Some(ReprojectionAdvance::Pending(_)) | None => Ok(Vec::new()),
        }
    }

    /// 🔙️ `BackboneMessage::Retract`: removes from the event log every transition this replica authored that `refused`
    /// names, with every transition of this replica causally depending on one ([`transition_retraction_closure`]), and
    /// folds the projection again — transactional: a log that would not fold keeps every transition. A name this log does
    /// not hold, or one another replica authored, retracts nothing: only this replica's own speculative head can be
    /// refused to it. Answers the retracted ids.
    pub async fn retract_transitions(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, refused: &[String]) -> Result<Vec<String>, VcsError> {
        let actor = store_log.transition_actor();
        let own: Vec<&semio_framework_os_spr::MutationEnvelope> = store_log.envelope.transitions.iter().filter(|transition| transition.actor == actor).collect();
        let retracted = transition_retraction_closure(&own, |transition| transition.mutation_id.0.as_str(), |transition| transition.dependencies.iter().map(|dependency| dependency.0.as_str()).collect(), refused);
        if retracted.is_empty() {
            return Ok(retracted);
        }
        let removed: Vec<semio_framework_os_spr::MutationEnvelope> = store_log.envelope.transitions.iter().filter(|transition| retracted.contains(&transition.mutation_id.0)).cloned().collect();
        store_log.envelope.transitions.retain(|transition| !retracted.contains(&transition.mutation_id.0));
        if let Err(error) = store_live.reproject(store_log, store_owners, store_cursor).await {
            for envelope in removed {
                store_live.insert_transition(store_log, envelope);
            }
            return Err(error);
        }
        store_cursor.last_projection_cause = Some(ArtifactProjectionCause::RemoteIngest);
        self.bump(store_log, store_owners, store_cursor, store_live)?;
        Ok(retracted)
    }

    /// 🪝️ `edit`'s `(hlc)` key when appending it — a fresh local edit tagged to this replica's line — makes the log's fold exactly
    /// the live cursor with `edit` applied last ([`crate::os_spr::fold_history_for`]): the live cursor is the proven fold of the
    /// log ([`Self::fold_frontier`]), `edit` sorts after every event, no history step waits and the head is its line's tip, where
    /// every uncommitted edit of the line is visible.
    fn folded_tail(&self, store_log: &StoreLog<P, Mutation>, store_live: &StoreLive<P, Mutation>, edit: &Edit<Mutation>) -> Option<(u64, u64, u64)> {
        let key = edit_timestamp(edit).cmp_key();
        let tail = store_live.fold_frontier.is_some_and(|frontier| key > frontier) && self.pending_reprojection.is_none() && store_log.envelope.viewer_checkpoint_id.is_none() && edit.line == store_log.envelope.active_alternative_id;
        tail.then_some(key)
    }

    /// ⚡️ The tail step `envelope` — one locally authored `Revert` or `Reinstate` — makes when it changes nothing but the
    /// applied tail: the live cursor is the proven fold of the log ([`Self::fold_is_live`]) at this replica's line tip, the
    /// transition sorts after every event, and it names exactly the operations of the applied tail edit (`Undo`) or of the redo
    /// top a tail step just moved there with nothing happening since (`Redo`), authored by that edit's own actor. The fold then
    /// moves exactly that edit between the stacks (`crate::os_spr::fold_history_for`): a `Revert` pushes it onto the redo
    /// stack, a `Reinstate` takes it back at its own HLC position, which is the tail (design §20.14, D22).
    pub fn tail_step(&self, store_log: &StoreLog<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, store_live: &StoreLive<P, Mutation>, envelope: &semio_framework_os_spr::MutationEnvelope) -> Option<TailStep> {
        let frontier = store_live.fold_frontier.filter(|_| self.fold_is_live(store_live) && !store_live.current_detached && store_log.envelope.viewer_checkpoint_id.is_none() && store_log.envelope.open_transaction.is_none())?;
        if envelope.timestamp.cmp_key() <= frontier {
            return None;
        }
        let (step, mutation_ids, edit_id) = match semio_framework_os_spr::history_transition_from_envelope(envelope).ok()?? {
            semio_framework_os_spr::HistoryTransition::Revert { mutation_ids } if store_log.redo_edit_ids.admits_one() => (TailStep::Undo, mutation_ids, store_log.applied_edit_ids.last()?),
            semio_framework_os_spr::HistoryTransition::Reinstate { mutation_ids } if store_live.tail_step_generation == Some(store_cursor.generation) && store_log.applied_edit_ids.admits_one() => (TailStep::Redo, mutation_ids, store_log.redo_edit_ids.last()?),
            _ => return None,
        };
        let edit = store_log.envelope.vcs.edits.find_near(|edit| edit.id == *edit_id)?;
        let owned: HashSet<MutationId> = semio_framework_os_spr::mutation_ids_for_edit::<P, Mutation>(edit).into_iter().collect();
        let exact = mutation_ids.len() == owned.len() && mutation_ids.iter().all(|mutation_id| owned.contains(mutation_id));
        (exact && edit.actor.as_deref().is_none_or(|actor| actor == envelope.actor.0.as_str())).then_some(step)
    }

    //#endregion 🔖️EventSourcedHistory
    //#region 🔖️ToolTransactions
    /// 🪡️ `AppendTransaction`: the first append opens one edit stamped with the ref — no description — and
    /// every later one folds its operations onto the projection and appends them to that edit, every operation stamped with
    /// the ref. The open edit is always the applied tail ([`Self::ingest_remote`] lifts it over remote edits), and neither
    /// the shared log nor a persisted form holds it before its commit, so an abort leaves no trace anywhere.
    pub async fn append_transaction(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, mutations: Vec<Mutation>, transaction: protocol::TransactionRef, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        if mutations.is_empty() {
            return Err(VcsError::EmptyApply);
        }
        let Some(open) = store_log.envelope.open_transaction.clone() else {
            return self.open_transaction_edit(store_log, store_owners, store_cursor, store_live, mutations, transaction, identity).await;
        };
        let before = store_cursor.content_revision;
        let pre_snapshot = Arc::clone(&*store_live.current);
        let (forwards, inverse, mutation_meta, post, mut messages, unit_flags) = store_owners.replay_mutations(store_log, &pre_snapshot, mutations, Some(&transaction), identity).await?;
        let edit = store_log.envelope.vcs.edits.iter_mut().find(|edit| edit.id == open.edit_id).ok_or_else(|| VcsError::UnknownEdit(open.edit_id.clone()))?;
        let offset = edit.forwards.len() as u32;
        for message in messages.iter_mut() {
            message.op_index = message.op_index.map(|index| index + offset);
        }
        let appended = forwards.len();
        edit.forwards.extend(forwards.adopt());
        edit.inverse.extend(inverse.adopt());
        edit.mutation_meta.extend(mutation_meta);
        self.record_edit_messages(store_log, store_owners, &open.edit_id, messages)?;
        store_live.replace_current_retained(store_log, store_owners, Arc::new(post.adopt()))?;
        if !self.fold_is_live(store_live) {
            store_live.reproject(store_log, store_owners, store_cursor).await?;
        }
        let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[open.edit_id.as_bytes()]);
        store_cursor.revision_accumulator.unit_flags.extend(unit_flags.into_iter().map(|(index, flag)| ((digest, offset as usize + index), flag)));
        self.bump(store_log, store_owners, store_cursor, store_live)?;
        self.continue_pending_replay_across_append(store_cursor, before, &open.edit_id, appended);
        Ok(())
    }

    /// 🧵️ A tick appended to the open transaction's edit — the applied tail — changes nothing a waiting replay already
    /// replayed and nothing it reads by id before the tail: the waiting replay started against the revision `before` the tick
    /// keeps running against the grown edit instead of restarting, so a tool streaming every turn never starves a waiting
    /// history step (audit W1G-5). A replay started against any other revision restarts as before.
    pub fn continue_pending_replay_across_append(&mut self, store_cursor: &mut StoreCursor<P, Mutation>, before: [u8; 32], edit_id: &str, appended: usize) {
        let revision = store_cursor.content_revision;
        if let Some((started, replay)) = self.pending_reprojection.as_mut().and_then(|pending| pending.replay.as_mut()).filter(|(started, _)| *started == before) {
            *started = revision;
            replay.absorb_tail_growth(edit_id, appended);
        }
    }

    /// 🪁️ The first append of a tool transaction: one fresh edit at the applied tail holding its operations, open and
    /// unannounced.
    async fn open_transaction_edit(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, mutations: Vec<Mutation>, transaction: protocol::TransactionRef, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let started_at = now_iso();
        let pre_snapshot = Arc::clone(&*store_live.current);
        let (forwards, inverse, mutation_meta, post, messages, unit_flags) = store_owners.replay_mutations(store_log, &pre_snapshot, mutations, Some(&transaction), identity).await?;
        let actor = edit_actor_from_meta(&mutation_meta).await;

        store_log.edit_sequence += 1;
        let forwards_fingerprint = identity.encode(|control|semio_framework_pack_json::to_json_string_controlled(&*forwards,control))?.into_bytes();
        let reservation = store_owners.reserve_edit_history_slot(store_log)?;
        let edit_id = identity.encode(|control|mint_edit_id(store_log.clock.actor, store_log.edit_sequence, &forwards_fingerprint, control))?;
        let edit = Edit {
            id: edit_id.clone(),
            actor,
            forwards: forwards.adopt(),
            inverse: inverse.adopt().into(),
            mutation_meta,
            verb: store_log.authoring_verb.clone(),
            line: store_log.envelope.active_alternative_id.clone(),
            sequence_number: store_log.edit_sequence,
            started_at,
            finished_at: None,
        };
        let tail = self.folded_tail(store_log, store_live, &edit);
        let actor = edit.actor.clone();
        store_live.insert_reserved_edit_history(store_log, store_owners, reservation, edit)?;
        self.record_edit_messages(store_log, store_owners, &edit_id, messages)?;
        store_live.replace_tail_undo_cache_retained(store_log, store_owners, Some((edit_id.clone(), pre_snapshot)))?;
        store_cursor.push_applied_edit_id(store_log, edit_id.clone());
        store_live.replace_current_retained(store_log, store_owners, Arc::new(post.adopt()))?;
        let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit_id.as_bytes()]);
        store_cursor.revision_accumulator.unit_flags.extend(unit_flags.into_iter().map(|(index, flag)| ((digest, index), flag)));
        store_log.envelope.open_transaction = Some(OpenToolTransaction { transaction, edit_id });
        match tail {
            Some(key) => store_live.adopt_folded_tail(store_log, store_owners, store_cursor, actor.as_deref(), key)?,
            None => drop(store_live.reproject(store_log, store_owners, store_cursor).await?),
        }
        self.bump(store_log, store_owners, store_cursor, store_live)
    }

    /// 🎗️ `CommitTransaction`: the open edit closes and every operation it holds is queued for announcement in one batch —
    /// exactly the edit one `Apply` of the same operations under the same ref would have recorded. Never suspends, so a
    /// runtime's one-page publisher closes a streamed transaction in place ([`Self::flush_announcements`] sends it).
    pub fn commit_open_transaction(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, transaction_id: &str) -> Result<(), VcsError> {
        let open = store_log.open_transaction_named(transaction_id)?;
        let edit = store_log.envelope.vcs.edits.iter_mut().find(|edit| edit.id == open.edit_id).ok_or_else(|| VcsError::UnknownEdit(open.edit_id.clone()))?;
        edit.finished_at = Some(now_iso());
        stamp_primary_operation_identity(edit);
        let edit = store_log.envelope.vcs.edits.find_near(|edit| edit.id == open.edit_id).ok_or_else(|| VcsError::UnknownEdit(open.edit_id.clone()))?;
        let operations = store_log.operation_envelopes(edit)?;
        store_log.envelope.open_transaction = None;
        self.announce_operations(store_log, operations)?;
        self.bump(store_log, store_owners, store_cursor, store_live)
    }

    /// 🪃️ `AbortTransaction`: the open edit leaves the ledger, its messages and its operations retire, the edit sequence it
    /// took is returned, and the projection folds back to the history without it — the projection and revision of before
    /// its first append. Never suspends.
    pub fn abort_open_transaction(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, transaction_id: &str) -> Result<(), VcsError> {
        let open = store_log.open_transaction_named(transaction_id)?;
        let removed = store_log.envelope.vcs.edits.extract_if(|edit| edit.id == open.edit_id).map_err(|fault| VcsError::ValidationFailed(format!("the open transaction edit cannot leave its ledger: {fault:?}")))?;
        if removed.iter().any(|edit| edit.sequence_number == store_log.edit_sequence) {
            store_log.edit_sequence -= 1;
        }
        retire_scratch_edits::<P, Mutation>(removed);
        store_owners.replace_edit_messages(store_log, &open.edit_id, Vec::new())?;
        store_log.envelope.open_transaction = None;
        store_live.reproject_now(store_log, store_owners, store_cursor)?;
        self.bump(store_log, store_owners, store_cursor, store_live)
    }

    /// 📤️ Sends every announcement a non-suspending history step queued ([`Self::commit_open_transaction`]) as one
    /// `BackboneMessage::Mutations` batch; nothing queued sends nothing.
    pub async fn flush_announcements(&mut self, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>) -> Result<(), VcsError> {
        self.flush_outbound(store_owners, store_wire).await
    }

    /// 🛞️ Folds a lifted tool transaction's edit back as the applied tail of the merged history: its operations take fresh
    /// clock stamps after everything this replica has seen and its next edit sequence, and the Report replay recomputes
    /// their inverses and messages against the merged head (keep-and-record: nothing is quarantined or refused). When the
    /// ledger can no longer hold it the transaction is aborted — zero trace — and the refusal is answered.
    pub async fn restore_open_transaction(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, open: OpenToolTransaction, mut edit: Edit<Mutation>) -> Result<(), VcsError> {
        for meta in edit.mutation_meta.iter_mut() {
            store_log.clock.tick(now_ms());
            meta.timestamp = store_log.clock;
        }
        store_log.edit_sequence += 1;
        edit.sequence_number = store_log.edit_sequence;
        let reservation = match store_owners.reserve_edit_history_slot(store_log) {
            Ok(reservation) => reservation,
            Err(error) => {
                store_log.edit_sequence -= 1;
                retire_scratch_edits::<P, Mutation>([edit]);
                return Err(error);
            }
        };
        store_live.insert_reserved_edit_history(store_log, store_owners, reservation, edit)?;
        store_log.envelope.open_transaction = Some(open);
        store_live.reproject(store_log, store_owners, store_cursor).await?;
        self.bump(store_log, store_owners, store_cursor, store_live)
    }

    //#endregion 🔖️ToolTransactions
    //#region 🔖️DeferredReprojection
    /// 🐢️ Steps the Report replay a remote history change needs — a supersession, undo or redo another replica authored —
    /// one `budget` per turn ([`ReplayTurnBudget`]: a wall deadline first, an operation cap second;
    /// [`Self::step_reprojection`], also every [`Self::tick`]) instead of inside the ingest that admitted it, so a long
    /// downstream replay never freezes this replica. Until the change is adopted the replica shows the history before it,
    /// and the adoption is exactly the one an ingest without a budget makes. `None` replays inside the ingest.
    pub fn defer_remote_replays(&mut self, budget: Option<ReplayTurnBudget>) {
        self.replay_budget = budget.filter(|budget| budget.operations > 0 && budget.wall_us > 0);
    }

    /// 🐌️ Steps the Report replay a local history step needs — an interior undo or redo, a checkout or alternative switch
    /// away from the applied tail, a supersession (finalize) authored without a finished replay (design §16.6) — one
    /// `budget` per turn ([`ReplayTurnBudget`]) instead of inside its dispatch, which replays the first turn and answers. Until
    /// the step is adopted ([`Self::step_reprojection`], also every [`Self::tick`]) the replica shows the history before it,
    /// nothing of the step is recorded or announced, every further history step is refused
    /// ([`VcsError::HistoryReplaying`]) and [`Self::discard_local_step`] drops it with zero trace; edits and remote changes
    /// meanwhile restart its replay. The adoption is exactly the one an undeferred dispatch makes, a supersession whose
    /// report blocks finalizing included (refused with `Rejected { policy: Normal }`). Steps that need no replay adopt
    /// inside their dispatch. `None` replays inside the dispatch.
    pub fn defer_local_replays(&mut self, budget: Option<ReplayTurnBudget>) {
        self.local_replay_budget = budget.filter(|budget| budget.operations > 0 && budget.wall_us > 0);
    }

    /// ⏳️ Whether a local history step still replays ([`Self::defer_local_replays`]).
    pub fn local_step_pending(&self) -> bool {
        self.pending_reprojection.as_ref().is_some_and(|pending| pending.local.is_some())
    }

    /// 🗑️ Drops the local history step that still replays ([`Self::defer_local_replays`]): nothing of it was recorded,
    /// announced or shown, so the store is exactly as before its dispatch; a waiting remote change keeps waiting and
    /// replays again from its start. Answers whether a step was waiting.
    pub fn discard_local_step(&mut self, store_owners: &mut StoreOwners<P, Mutation>) -> bool {
        let Some(mut pending) = self.pending_reprojection.take() else {
            return false;
        };
        let discarded = pending.local.take().is_some();
        if discarded {
            let _ = store_owners.retire_pending_replay(pending.replay.take());
        }
        self.pending_reprojection = (!pending.transitions.is_empty() || pending.local.is_some()).then_some(pending);
        discarded
    }

    /// 📶️ The progress of the deferred reprojection — a remote change, a local step or both (`done` 0 until its replay
    /// starts, and again once an edit or another change made its replay restart), `None` when nothing waits.
    pub fn reprojection_progress(&self, store_cursor: &StoreCursor<P, Mutation>) -> Option<ReplayProgress> {
        self.pending_reprojection.as_ref().map(|pending| pending.replay.as_ref().filter(|(revision, _)| *revision == store_cursor.content_revision).map_or_else(ReplayProgress::default, |(_, replay)| replay.progress()))
    }

    /// 📈️ Reports cumulative semantic work from the revision-fenced reprojection owner.
    pub fn reprojection_operation_preparation_progress(&self, store_cursor: &StoreCursor<P, Mutation>) -> Option<ArtifactReplayPreparationProgress> {
        self.pending_reprojection.as_ref().map(|pending| pending.replay.as_ref().filter(|(revision, _)| *revision == store_cursor.content_revision).map_or_else(ArtifactReplayPreparationProgress::default, |(_, replay)| replay.operation_preparation_progress()))
    }

    /// ✋️ Drops the running replay of the deferred reprojection: the store is untouched, the change stays admitted (a
    /// local step too — [`Self::discard_local_step`] drops it) and the next step replays it from the start. Answers whether a
    /// replay was running.
    pub fn cancel_reprojection(&mut self, store_owners: &mut StoreOwners<P, Mutation>) -> bool {
        let Some(replay) = self.pending_reprojection.as_mut().and_then(|pending| pending.replay.take()) else {
            return false;
        };
        let _ = store_owners.retire_pending_replay(Some(replay));
        true
    }

    /// ⏭️ Advances the deferred reprojection by one turn — until `deadline_us` on the budget's clock when the runtime names
    /// its turn deadline, else one [`ReplayTurnBudget`] — and adopts it once its replay finished, announcing a local step's
    /// transitions. Answers its progress while it still waits, `None` once nothing waits.
    pub async fn step_reprojection(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, deadline_us: Option<u64>) -> Result<Option<ReplayProgress>, VcsError> {
        match self.advance_reprojection(store_log, store_owners, store_wire, store_cursor, store_live, deadline_us).await? {
            Some(ReprojectionAdvance::Pending(progress)) => Ok(Some(progress)),
            Some(ReprojectionAdvance::Adopted(_)) => {
                self.bump(store_log, store_owners, store_cursor, store_live)?;
                store_cursor.last_projection_cause = Some(ArtifactProjectionCause::Replay);
                Ok(None)
            }
            None => Ok(None),
        }
    }

    /// 🛗️ Takes a local history step whose Report replay must not run inside its dispatch ([`Self::defer_local_replays`]):
    /// joins it to the waiting change (with any remote transitions already waiting), advances the clock past its
    /// transitions, steps one budget and adopts everything once the replay finished — here or in a later turn. Answers the
    /// step back when it is adopted synchronously instead: local replays are not deferred, or a plain step needs no replay.
    pub async fn defer_local_step(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, step: PendingLocalStep, clock: HybridLogicalTimestamp) -> Result<Option<PendingLocalStep>, VcsError> {
        if self.local_replay_budget.is_none() {
            return Ok(Some(step));
        }
        if !step.finalizing {
            let fold = store_log.prospective_fold_at(&step.envelopes, &store_log.viewer_head(&step.head))?;
            if fold.supersessions == *store_live.supersessions && store_log.applied_edit_ids.starts_with(&fold.applied) {
                return Ok(Some(step));
            }
        }
        let mut pending = self.pending_reprojection.take().unwrap_or_else(PendingReprojection::empty);
        if pending.local.is_some() {
            self.pending_reprojection = Some(pending);
            return Err(VcsError::HistoryReplaying);
        }
        store_owners.retire_pending_replay(pending.replay.take())?;
        pending.local = Some(step);
        self.pending_reprojection = Some(pending);
        store_log.clock = clock;
        match self.advance_reprojection(store_log, store_owners, store_wire, store_cursor, store_live, None).await? {
            Some(ReprojectionAdvance::Adopted(_)) => {
                self.bump(store_log, store_owners, store_cursor, store_live)?;
                store_cursor.last_projection_cause = Some(ArtifactProjectionCause::Replay);
                Ok(None)
            }
            Some(ReprojectionAdvance::Pending(_)) | None => Ok(None),
        }
    }

    /// 🧮️ One turn of the deferred reprojection (until `deadline_us`, else one [`ReplayTurnBudget`]): (re)starts its Report
    /// replay when the history it started from changed, steps it, and adopts the change with the finished replay; a change
    /// that needs no replay is adopted at once. A refused adoption drops the change — a refused local step only, its remote
    /// transitions wait again — and answers the refusal.
    async fn advance_reprojection(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, deadline_us: Option<u64>) -> Result<Option<ReprojectionAdvance>, VcsError> {
        let Some(mut pending) = self.pending_reprojection.take() else {
            return Ok(None);
        };
        if pending.replay.as_ref().is_none_or(|(revision, _)| *revision != store_cursor.content_revision) {
            let _ = store_owners.retire_pending_replay(pending.replay.take());
            let started = store_log.pending_fold(&pending).and_then(|fold| {
                if fold.supersessions == *store_live.supersessions && store_log.applied_edit_ids.starts_with(&fold.applied) {
                    return Ok(None);
                }
                let (from, horizon) = store_live.replay_window(store_log, &fold.applied, &fold.supersessions)?;
                store_live.report_replay(store_log, store_cursor, &fold.applied, from, fold.supersessions.clone(), horizon, StoreLive::<P, Mutation>::prefix_stride(fold.applied.len())).map(Some)
            });
            match started {
                Ok(Some(replay)) => pending.replay = Some((store_cursor.content_revision, replay)),
                Ok(None) => return self.adopt_pending(store_log, store_owners, store_wire, store_cursor, store_live, pending, None).await.map(|replayed| Some(ReprojectionAdvance::Adopted(replayed))),
                Err(error) => {
                    self.refuse_local_step(store_owners, pending);
                    return Err(error);
                }
            }
        }
        let budget = pending.local.as_ref().and(self.local_replay_budget).or(self.replay_budget).unwrap_or(ReplayTurnBudget::operations(usize::MAX));
        let deadline = budget.turn_deadline(deadline_us);
        let (_, replay) = pending.replay.as_mut().expect("a started deferred replay");
        let step = replay.step_operations(&store_log.envelope.vcs.edits, budget.operations, &mut || budget.turn_ends(0, deadline));
        match step {
            Ok(ReplayStep::Pending(progress)) => {
                self.pending_reprojection = Some(pending);
                Ok(Some(ReprojectionAdvance::Pending(progress)))
            }
            Ok(ReplayStep::Finished(_)) => {
                let (_, mut replay) = pending.replay.take().expect("a finished deferred replay");
                match replay.finish() {
                    Ok(result) => self.adopt_pending(store_log, store_owners, store_wire, store_cursor, store_live, pending, Some(result)).await.map(|replayed| Some(ReprojectionAdvance::Adopted(replayed))),
                    Err(error) => {
                        let _ = store_owners.retire_raw_replay(replay);
                        self.refuse_local_step(store_owners, pending);
                        Err(error)
                    }
                }
            }
            Err(error) => {
                self.refuse_local_step(store_owners, pending);
                Err(error)
            }
        }
    }

    /// ⛔️ Drops a refused waiting change: its local step is refused with nothing of it recorded, and its remote transitions
    /// wait again (their replay restarts) — or, refused themselves, are dropped.
    fn refuse_local_step(&mut self, store_owners: &mut StoreOwners<P, Mutation>, mut pending: PendingReprojection<P, Mutation>) {
        let _ = store_owners.retire_pending_replay(pending.replay.take());
        if pending.local.take().is_some() && !pending.transitions.is_empty() {
            self.pending_reprojection = Some(pending);
        }
    }

    /// 🧶️ Adopts a waiting change with its finished replay (`None`: it needs none): its remote transitions alone through
    /// [`Self::adopt_remote_transitions`]; with a local step, both are recorded in the log, this replica moves to the step's
    /// head, the projection follows and the step's transitions are announced. A finalizing step whose report blocks is
    /// refused (`Rejected { policy: Normal }`, every replay message) with nothing of it recorded while its remote transitions
    /// wait again. Atomic: a refused adoption keeps the log and the head.
    async fn adopt_pending(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, pending: PendingReprojection<P, Mutation>, replayed: Option<EditReplayResult<P, Mutation>>) -> Result<Vec<semio_framework_os_spr::EditMessages>, VcsError> {
        let PendingReprojection { transitions, local, replay } = pending;
        store_owners.retire_pending_replay(replay)?;
        let Some(local) = local else {
            return store_live.adopt_remote_transitions(store_log, store_owners, store_cursor, transitions, replayed).await;
        };
        let report = match replayed.as_ref().filter(|_| local.finalizing).map(|result| store_live.replay_report(store_log, store_wire, store_cursor, result)).transpose() {
            Ok(report) => report,
            Err(error) => {
                if let Some(replayed) = replayed {
                    let _ = store_owners.retire_replay_result(replayed);
                }
                self.refuse_local_step(store_owners, PendingReprojection { transitions, local: Some(local), replay: None });
                return Err(error);
            }
        };
        if let Some(report) = report.as_ref().filter(|report| report.blocks_finalize()) {
            if let Some(replayed) = replayed {
                let _ = store_owners.retire_replay_result(replayed);
            }
            let messages = report.outcomes.iter().flat_map(|outcome| outcome.messages.iter().cloned()).collect();
            self.refuse_local_step(store_owners, PendingReprojection { transitions, local: Some(local), replay: None });
            return Err(VcsError::Rejected { policy: semio_framework_os_spr::MergePolicy::Normal, messages });
        }
        let PendingLocalStep { envelopes, head, finalizing } = local;
        let saved = store_log.viewer_head_now();
        let remote: Vec<MutationId> = transitions.iter().map(|envelope| envelope.mutation_id.clone()).collect();
        let authored: Vec<MutationId> = envelopes.iter().map(|envelope| envelope.mutation_id.clone()).collect();
        for envelope in transitions.into_iter().chain(envelopes.iter().cloned()) {
            store_live.insert_transition(store_log, envelope);
        }
        (store_log.envelope.active_alternative_id, store_log.envelope.viewer_checkpoint_id) = head.clone();
        let projected = match replayed {
            Some(result) => store_live.reproject_replayed(store_log, store_owners, store_cursor, result),
            None => store_live.reproject(store_log, store_owners, store_cursor).await,
        };
        let replayed = match projected {
            Ok(replayed) => replayed,
            Err(error) => {
                let waiting: Vec<semio_framework_os_spr::MutationEnvelope> = store_log.envelope.transitions.extract_if(.., |known| remote.contains(&known.mutation_id)).collect();
                store_log.envelope.transitions.retain(|known| !authored.contains(&known.mutation_id));
                (store_log.envelope.active_alternative_id, store_log.envelope.viewer_checkpoint_id) = saved;
                self.refuse_local_step(store_owners, PendingReprojection { transitions: waiting, local: Some(PendingLocalStep { envelopes, head, finalizing }), replay: None });
                return Err(error);
            }
        };
        for envelope in &envelopes {
            store_log.dag.seed_applied(envelope.mutation_id.clone()).map_err(|error| VcsError::ValidationFailed(error.to_string()))?;
        }
        if let Some(report) = report {
            self.record_replay_report(&report);
        }
        self.pending_report.outbound.extend(envelopes);
        self.flush_outbound(store_owners, store_wire).await?;
        Ok(replayed)
    }

    //#endregion 🔖️DeferredReprojection
    //#region 🔖️Supersede
    /// 📮️ Hands a supersession's replay messages to the dispatch receipt, grouped by edit.
    pub fn record_replay_report(&mut self, report: &protocol::ReplayReport) {
        let mut grouped: Vec<semio_framework_os_spr::EditMessages> = Vec::new();
        for outcome in report.outcomes.iter().filter(|outcome| !outcome.messages.is_empty()) {
            match grouped.last_mut() {
                Some(entry) if entry.edit_id == outcome.edit_id => entry.messages.extend(outcome.messages.iter().cloned()),
                _ => grouped.push(semio_framework_os_spr::EditMessages { edit_id: outcome.edit_id.clone(), messages: outcome.messages.clone() }),
            }
        }
        self.pending_report.messages = grouped;
        self.pending_report.worst = report.worst;
    }

    //#endregion 🔖️Supersede
    /// 📨️ Copies a `MergeReport`'s messages/worst level into `pending_report` for `dispatch` to
    /// hand back as the resulting `CommandReceipt` — shared by the `IngestRemote`/`ResolveConflict`
    /// arms above.
    pub fn absorb_merge_report(&mut self, report: &semio_framework_os_spr::MergeReport) {
        if self.pending_report.edit_ids.is_none() {
            self.pending_report.edit_ids = Some(if report.accepted { report.replayed.iter().map(|edit_messages| edit_messages.edit_id.clone()).collect() } else { Vec::new() });
        }
        self.pending_report.messages = report.replayed.clone();
        self.pending_report.worst = report.worst;
    }

    //#region 🔖️HistoryLane
    /// 👁 Moves this replica onto `alternative_id` at its tip. Registration stays the shared log;
    /// no checkout event is authored, so every other replica keeps its own head.
    pub async fn switch_local_alternative(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, alternative_id: String) -> Result<(), VcsError> {
        let trunk = store_log.trunk_alternative_id();
        if alternative_id != trunk {
            let alternative = store_log.envelope.vcs.alternatives.iter().find(|alternative| alternative.id == alternative_id).ok_or_else(|| VcsError::UnknownAlternative(alternative_id.clone()))?;
            let checkpoint_id = alternative.checkpoint_ids.last().ok_or(VcsError::NoCheckpoint)?.clone();
            if !store_log.envelope.vcs.checkpoints.iter().any(|checkpoint| checkpoint.id == checkpoint_id) {
                return Err(VcsError::NoCheckpoint);
            }
            self.move_head(store_log, store_owners, store_wire, store_cursor, store_live, (Some(alternative_id), None)).await
        } else {
            self.move_head(store_log, store_owners, store_wire, store_cursor, store_live, (None, None)).await
        }
    }

    /// 👁 Looks at `checkpoint_id` on this replica only. The line stays the current one when that
    /// checkpoint is on it; otherwise it is the first registered alternative whose chain contains it.
    pub async fn checkout_local_checkpoint(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, checkpoint_id: String) -> Result<(), VcsError> {
        if !store_log.envelope.vcs.checkpoints.iter().any(|checkpoint| checkpoint.id == checkpoint_id) {
            return Err(VcsError::UnknownChange(checkpoint_id));
        }
        let current = store_log.active_line_id();
        let line = if store_log.envelope.vcs.alternatives.iter().any(|alternative| alternative.id == current && alternative.checkpoint_ids.iter().any(|id| id == &checkpoint_id)) {
            current
        } else {
            store_log.envelope.vcs.alternatives.iter().find(|alternative| alternative.checkpoint_ids.iter().any(|id| id == &checkpoint_id)).map(|alternative| alternative.id.clone()).unwrap_or(current)
        };
        self.move_head(store_log, store_owners, store_wire, store_cursor, store_live, (store_log.branched_alternative(line), Some(checkpoint_id))).await
    }

    /// 🚶️ Moves this replica's head to `(line, checkpoint)` and materializes the projection; a refused move keeps the head it
    /// had. Under [`Self::defer_local_replays`] a move whose replay is longer than one budget waits for later turns.
    async fn move_head(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, head: (Option<String>, Option<String>)) -> Result<(), VcsError> {
        let Some(PendingLocalStep { head, .. }) = self.defer_local_step(store_log, store_owners, store_wire, store_cursor, store_live, PendingLocalStep { envelopes: Vec::new(), head, finalizing: false }, store_log.clock).await? else {
            return Ok(());
        };
        let saved = store_log.viewer_head_now();
        (store_log.envelope.active_alternative_id, store_log.envelope.viewer_checkpoint_id) = head;
        if let Err(error) = store_live.reproject(store_log, store_owners, store_cursor).await {
            (store_log.envelope.active_alternative_id, store_log.envelope.viewer_checkpoint_id) = saved;
            return Err(error);
        }
        self.bump(store_log, store_owners, store_cursor, store_live)
    }

    /// 🛤️ Shared body of `Apply`/`ApplyInLane`: identical edit-recording logic to the
    /// pre-lane `Apply` arm, plus tagging the fresh edit into `envelope.lanes` when `lane` isn't
    /// the default `Document` (kept sparse — see `ArtifactEnvelope.lanes`'s doc).
    ///
    /// ⚡️ `current` is always up to date (maintained by every mutating command below), so this
    /// is an O(1) clone instead of a full replay — see the `current` field doc.
    pub async fn apply_command(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, mutations: Vec<Mutation>, lane: HistoryLane, transaction: Option<protocol::TransactionRef>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        if mutations.is_empty() {
            return Err(VcsError::EmptyApply);
        }
        if let Some(open) = store_log.envelope.open_transaction.as_ref() {
            let transaction_id = open.transaction.id.clone();
            retire_operations::<P, Mutation>(mutations);
            return Err(VcsError::TransactionOpen { transaction_id });
        }
        let started_at = now_iso();
        let pre_snapshot = Arc::clone(&*store_live.current);
        let (forwards, inverse, mutation_meta, post, messages, unit_flags) = store_owners.replay_mutations(store_log, &pre_snapshot, mutations, transaction.as_ref(), identity).await?;
        let actor = edit_actor_from_meta(&mutation_meta).await;

        store_log.edit_sequence += 1;
        let forwards_fingerprint = identity.encode(|control|semio_framework_pack_json::to_json_string_controlled(&*forwards,control))?.into_bytes();
        let reservation = store_owners.reserve_edit_history_slot(store_log)?;
        let mut edit = ScratchOwner::new(
            Edit {
                id: identity.encode(|control|mint_edit_id(store_log.clock.actor, store_log.edit_sequence, &forwards_fingerprint, control))?,
                actor,
                forwards: forwards.adopt(),
                inverse: inverse.adopt().into(),
                mutation_meta,
                verb: store_log.authoring_verb.clone(),
                line: store_log.envelope.active_alternative_id.clone(),
                sequence_number: store_log.edit_sequence,
                started_at,
                finished_at: Some(now_iso()),
            },
            |edit| retire_scratch_edits::<P, Mutation>([edit]),
        );
        stamp_primary_operation_identity(&mut *edit);
        let edit_id = edit.id.clone();
        let operations = store_log.operation_envelopes(&*edit)?;
        let tail = self.folded_tail(store_log, store_live, &*edit);
        let actor = edit.actor.clone();
        store_live.insert_reserved_edit_history(store_log, store_owners, reservation, edit.adopt())?;
        if !lane.is_document().await {
            store_log.envelope.lanes.insert(edit_id.clone(), lane);
        }
        self.record_edit_messages(store_log, store_owners, &edit_id, messages)?;
        store_live.replace_tail_undo_cache_retained(store_log, store_owners, Some((edit_id.clone(), pre_snapshot)))?;
        let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit_id.as_bytes()]);
        store_cursor.push_applied_edit_id(store_log, edit_id);
        store_live.replace_current_retained(store_log, store_owners, Arc::new(post.adopt()))?;
        self.announce_operations(store_log, operations)?;
        match tail {
            Some(key) => store_live.adopt_folded_tail(store_log, store_owners, store_cursor, actor.as_deref(), key)?,
            None => drop(store_live.reproject(store_log, store_owners, store_cursor).await?),
        }
        store_cursor.revision_accumulator.unit_flags.extend(unit_flags.into_iter().map(|(index, flag)| ((digest, index), flag)));
        self.bump(store_log, store_owners, store_cursor, store_live)?;
        Ok(())
    }

    //#endregion 🔖️HistoryLane
    /// 📨️ Records `edit_id`'s replay messages into the durable ledger and `pending_report`, shared
    /// by `apply_command` and the transaction appends — every local dispatch's own `DispatchReport`-shaped tail.
    fn record_edit_messages(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, edit_id: &str, messages: Vec<semio_framework_os_spr::MutationMessage>) -> Result<(), VcsError> {
        if messages.is_empty() {
            return Ok(());
        }
        if edit_id.len() > ARTIFACT_EDIT_MESSAGE_ID_BYTES {
            return Err(VcsError::ValidationFailed("artifact edit-message id exceeds its fixed byte authority".into()));
        }
        let mut candidate = if let Some(established) = store_log.envelope.edit_messages.get_by_id(edit_id) {
            let mut combined = established.messages.clone();
            combined.extend(messages.clone());
            semio_framework_os_spr::EditMessages { edit_id: edit_id.to_string(), messages: combined }
        } else {
            semio_framework_os_spr::EditMessages { edit_id: edit_id.to_string(), messages: messages.clone() }
        };
        bound_edit_messages(edit_id, &mut candidate.messages);
        store_owners.displaced_retirements.reserve(1)?;
        if let Some(established) = store_log.envelope.edit_messages.get_mut_by_id(edit_id) {
            let previous = std::mem::replace(established, candidate);
            store_owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreMessageLedgerRetirement::new(previous.edit_id, previous.messages)));
        } else if let Err(rejected) = store_log.envelope.edit_messages.admit(candidate) {
            store_owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreMessageLedgerRetirement::new(rejected.edit_id, rejected.messages)));
            return Err(VcsError::ValidationFailed("artifact edit-message fixed ledger rejected its exact candidate owner".into()));
        }
        self.pending_report.worst = semio_framework_os_spr::worst_level(&messages).max(self.pending_report.worst);
        self.pending_report.messages.push(semio_framework_os_spr::EditMessages { edit_id: edit_id.to_string(), messages });
        Ok(())
    }

    /// ✂️ Detaches the backbone; the WIP graph stays in memory, simply unsynchronized.
    ///
    /// 🛡️ {@link bump} runs FIRST so a refusal leaves the store exactly as it was. Clearing
    /// the persisted descriptor before the bump meant a store whose displaced-retirement
    /// destination was full (or whose generation was exhausted) reported a refusal while having
    /// already forgotten which backbone it was attached to — the descriptor was gone but the live
    /// transport, its queued payload and its generation were all still in place.
    pub fn detach_backbone(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>) -> Result<Option<Backbones>, VcsError> {
        store_wire.ensure_durable_group_idle()?;
        self.bump(store_log, store_owners, store_cursor, store_live)?;
        store_log.envelope.backbone = None;
        Ok(store_wire.backbone.take())
    }

    /// ⚖️ Resolves an `Open` `crate::os_spr::Conflict` by id (`26/08/16/MUTATION-OUTCOMES-
    /// MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6). `Quarantined`+`Accept` admits the conflict's
    /// withheld events into the log and re-folds it, validated under `LaissezFaire` (a `Fatal`
    /// message still rejects — `LaissezFaire` never waives that), landing the same state
    /// `ingest_remote` would have produced under `LaissezFaire` in the first place, and marks the
    /// conflict `Accepted` WITHOUT raising a second one. `Quarantined`+`Discard` seeds the quarantined ids into
    /// the causal dag as already-seen — never applied, never relayed — and marks the conflict
    /// `Discarded`. `Degraded`+`Accept` only flips the status: the batch is already durable history,
    /// resolving never rewrites it. `Degraded`+`Discard` is refused outright — shared history is
    /// never rewritten.
    pub async fn resolve_conflict(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, conflict_id: &str, resolution: semio_framework_os_spr::ConflictResolution) -> Result<semio_framework_os_spr::MergeReport, VcsError>
    where
        P: Sync,
    {
        store_wire.ensure_durable_group_idle()?;
        let index = store_log.envelope.conflicts.iter().position(|conflict| conflict.id.0 == conflict_id && conflict.status == semio_framework_os_spr::ConflictStatus::Open).ok_or_else(|| VcsError::UnknownConflict(conflict_id.to_string()))?;
        let conflict = store_log.envelope.conflicts[index].clone();
        match (conflict.kind.clone(), resolution) {
            (semio_framework_os_spr::ConflictKind::Quarantined { envelopes, .. }, semio_framework_os_spr::ConflictResolution::Accept) => self.accept_quarantined_events(store_log, store_owners, store_cursor, store_live, index, conflict.id.clone(), envelopes).await,
            (semio_framework_os_spr::ConflictKind::Quarantined { envelopes, .. }, semio_framework_os_spr::ConflictResolution::Discard) => {
                for envelope in &envelopes {
                    store_log.dag.seed_applied(envelope.mutation_id.clone()).map_err(|error| VcsError::ValidationFailed(error.to_string()))?;
                }
                store_log.envelope.conflicts[index].status = semio_framework_os_spr::ConflictStatus::Discarded;
                self.pending_report.edit_ids = Some(Vec::new());
                self.bump(store_log, store_owners, store_cursor, store_live)?;
                store_cursor.last_projection_cause = Some(ArtifactProjectionCause::RemoteIngest);
                store_owners.prune_resolved_conflicts(store_log)?;
                Ok(semio_framework_os_spr::MergeReport { policy: store_log.merge_policy, accepted: false, insertion_index: 0, replayed: Vec::new(), worst: None, conflict: Some(conflict.id.clone()) })
            }
            (semio_framework_os_spr::ConflictKind::Degraded { .. }, semio_framework_os_spr::ConflictResolution::Accept) => {
                store_log.envelope.conflicts[index].status = semio_framework_os_spr::ConflictStatus::Accepted;
                self.pending_report.edit_ids = Some(Vec::new());
                self.bump(store_log, store_owners, store_cursor, store_live)?;
                store_cursor.last_projection_cause = Some(ArtifactProjectionCause::RemoteIngest);
                store_owners.prune_resolved_conflicts(store_log)?;
                Ok(semio_framework_os_spr::MergeReport { policy: store_log.merge_policy, accepted: true, insertion_index: 0, replayed: Vec::new(), worst: None, conflict: Some(conflict.id.clone()) })
            }
            (semio_framework_os_spr::ConflictKind::Degraded { .. }, semio_framework_os_spr::ConflictResolution::Discard) => Err(VcsError::ValidationFailed("a Degraded conflict's batch is already durable history and can never be discarded".to_string())),
        }
    }

    /// ✅️ Admits a quarantined conflict's withheld events: the log is re-folded as if the
    /// conflict were accepted and the resulting order is replayed from genesis under `LaissezFaire`.
    /// A `Fatal` outcome reports the replay and changes nothing; otherwise the withheld edits join the
    /// log, the conflict turns `Accepted`, and the projection follows the fold.
    async fn accept_quarantined_events(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, index: usize, conflict_id: semio_framework_os_spr::ConflictId, envelopes: Vec<semio_framework_os_spr::MutationEnvelope>) -> Result<semio_framework_os_spr::MergeReport, VcsError> {
        let mut admitted: Vec<Edit<Mutation>> = Vec::with_capacity(envelopes.len());
        for envelope in &envelopes {
            let mut edit = edit_from_operation_envelope::<Mutation>(envelope).await?;
            edit.actor = Some(envelope.actor.0.clone());
            stamp_edit_semantics::<P, Mutation>(&mut edit, &store_log.envelope.schema);
            if !store_log.envelope.vcs.edits.iter().any(|known| known.id == edit.id) && !admitted.iter().any(|known| known.id == edit.id) {
                admitted.push(edit);
            } else {
                retire_scratch_edits::<P, Mutation>([edit]);
            }
        }
        let mut conflicts = store_log.envelope.conflicts.clone();
        conflicts[index].status = semio_framework_os_spr::ConflictStatus::Accepted;
        let mut edits: Vec<&Edit<Mutation>> = store_log.envelope.vcs.edits.iter().collect();
        edits.extend(admitted.iter());
        let head = envelope_viewer_head(&store_log.envelope);
        let fold = match fold_event_log::<P, Mutation>(&store_log.envelope.id, &edits, &store_log.envelope.transitions, &conflicts, &head) {
            Ok(fold) => fold,
            Err(error) => {
                retire_scratch_edits::<P, Mutation>(admitted);
                return Err(error);
            }
        };
        let edits_by_id: HashMap<String, Edit<Mutation>> = edits.iter().map(|edit| (edit.id.clone(), (*edit).clone())).collect();
        let replay = Self::replay_suffix_partitioned(store_log.envelope.vcs.genesis.facts().share_snapshot(), &fold.applied, 0, &edits_by_id, &store_log.envelope.schema, &fold.supersessions, semio_framework_os_spr::MergePolicy::LaissezFaire);
        retire_scratch_edits::<P, Mutation>(edits_by_id.into_values());
        let (state, quarantined_ids, mut rebased_inverse, replayed) = match replay {
            Ok(mut replay) => {
                let taken = (replay.take_state(), replay.take_quarantined(), replay.take_rebased_undo_rows().into_iter().collect::<HashMap<String, semio_framework_value::list::PagedList<Mutation, {usize::MAX}>>>(), replay.take_replayed());
                let _ = store_owners.retire_replay_result(replay);
                taken
            }
            Err(error) => {
                retire_scratch_edits::<P, Mutation>(admitted);
                return Err(error);
            }
        };
        if let Some(state) = state {
            retire_shared_projection::<P, Mutation>(state);
        }
        let admitted_replay: Vec<semio_framework_os_spr::EditMessages> = replayed.into_iter().filter(|entry| admitted.iter().any(|edit| edit.id == entry.edit_id)).collect();
        let worst = admitted_replay.iter().flat_map(|entry| entry.messages.iter()).map(|message| message.level).max();
        let report = semio_framework_os_spr::MergeReport { policy: semio_framework_os_spr::MergePolicy::LaissezFaire, accepted: quarantined_ids.is_empty(), insertion_index: 0, replayed: admitted_replay, worst, conflict: Some(conflict_id) };
        if !report.accepted {
            retire_scratch_edits::<P, Mutation>(admitted);
            for (_, inverse) in rebased_inverse {
                retire_scratch_operations::<P, Mutation>(inverse);
            }
            self.pending_report.edit_ids = Some(Vec::new());
            return Ok(report);
        }
        let admitted_ids: Vec<String> = admitted.iter().map(|edit| edit.id.clone()).collect();
        for mut edit in admitted {
            if let Some(inverse) = rebased_inverse.remove(&edit.id) {
                retire_scratch_operations::<P, Mutation>(std::mem::replace(&mut edit.inverse, inverse));
            }
            let reservation = store_owners.reserve_edit_history_slot(store_log)?;
            store_live.insert_reserved_edit_history(store_log, store_owners, reservation, edit)?;
        }
        for (_, inverse) in rebased_inverse {
            retire_scratch_operations::<P, Mutation>(inverse);
        }
        for entry in &report.replayed {
            store_owners.replace_edit_messages(store_log, &entry.edit_id, entry.messages.clone())?;
        }
        for envelope in &envelopes {
            match store_log.dag.seed_applied(envelope.mutation_id.clone()) {
                Ok(()) => {}
                Err(rejected) if rejected.error == semio_framework_os_spr::MutationDagError::Duplicate => {}
                Err(rejected) => return Err(VcsError::ValidationFailed(rejected.error.to_string())),
            }
        }
        store_log.envelope.conflicts[index].status = semio_framework_os_spr::ConflictStatus::Accepted;
        store_live.reproject(store_log, store_owners, store_cursor).await?;
        self.pending_report.edit_ids = Some(admitted_ids);
        self.bump(store_log, store_owners, store_cursor, store_live)?;
        store_cursor.last_projection_cause = Some(ArtifactProjectionCause::RemoteIngest);
        store_owners.prune_resolved_conflicts(store_log)?;
        Ok(report)
    }

    /// 📤️ Announces every event the finished command authored — edit operations and history
    /// transitions alike — as one `BackboneMessage::Mutations` batch.
    pub async fn flush_outbound(&mut self, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>) -> Result<(), VcsError> {
        if self.pending_report.outbound.is_empty() {
            return Ok(());
        }
        let Some(mut backbone) = store_wire.backbone.take() else {
            return Ok(());
        };
        let mut result = Ok(());
        for batch in document_backbone_batches(std::mem::take(&mut self.pending_report.outbound)) {
            result = backbone.send(BackboneMessage::Mutations { envelopes: semio_framework_os_spr::encode_envelopes(&batch) }).await;
            if result.is_err() {
                break;
            }
        }
        store_wire.replace_backbone_retained(store_owners, Some(backbone))?;
        result
    }

    /// 📤️ Announces exactly the `items` operations ONE outbound batched publication appended at
    /// position `from` of the tail applied edit, through the command outbound queue that
    /// `flush_outbound` drains. A closing transaction append extends the open edit whose earlier ticks were
    /// already announced; announcing the whole edit re-sent every earlier, already Accepted operation and the
    /// hub refused the batch as a replay (ticket 26/09/23 C10 09:4x: `[edit-A#0, edit-B#0]` on the
    /// second keystroke of a hub writer).
    async fn flush_apply_outbound(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, from: usize, items: usize) -> Result<(), VcsError> {
        if store_wire.backbone.is_none() {
            return Ok(());
        }
        let tail = store_log.applied_edit_ids.last().ok_or_else(|| VcsError::ValidationFailed("outbound batched publication has no applied tail edit".into()))?;
        let edit = store_log.envelope.vcs.edits.find_near(|edit| edit.id == *tail).ok_or_else(|| VcsError::UnknownEdit(tail.clone()))?;
        if edit.forwards.len() != from.saturating_add(items) {
            return Err(VcsError::ValidationFailed("outbound batched publication no longer names the tail edit it appended to".into()));
        }
        let operations = store_log.operation_envelopes_since(edit, from)?;
        self.announce_operations(store_log, operations)?;
        self.flush_outbound(store_owners, store_wire).await
    }

    /// 🧭️ Whether the live cursor is the proven fold of the log as it stands ([`Self::fold_frontier`]) with no history step
    /// waiting.
    fn fold_is_live(&self, store_live: &StoreLive<P, Mutation>) -> bool {
        store_live.fold_frontier.is_some() && self.pending_reprojection.is_none()
    }

    pub fn bump(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>) -> Result<(), VcsError> {
        let next_generation = store_cursor.generation.checked_add(1).ok_or_else(|| VcsError::ValidationFailed("artifact store generation counter is exhausted".into()))?;
        store_owners.displaced_retirements.reserve(3)?;
        store_cursor.generation = next_generation;
        let dirt = std::mem::replace(&mut store_cursor.mirror_dirt, StackMirrorDirt::CLEAN);
        store_owners.sync_cursor(store_log, dirt);
        let regrown = store_cursor.revision_accumulator.reconcile::<P, Mutation>(&store_log.applied_edit_ids, &store_log.redo_edit_ids, &store_log.envelope.vcs.edits, &store_live.supersessions, dirt, store_cursor.revision_dirty_from.take());
        store_cursor.content_revision = store_cursor.revision_accumulator.revision(store_log.current_checkpoint_id.as_deref());
        assert!(store_owners.snapshot_read_leases.publish_authority(store_cursor.generation, store_cursor.content_revision), "single-owner store bump must advance its snapshot commit authority");
        #[cfg(test)]
        self.assert_mirrors_are_live(store_log, store_cursor, store_live);
        if let Some(position) = regrown {
            store_live.prefix_ring_dirty_from = store_live.prefix_ring_dirty_from.min(position);
        }
        store_live.prune_prefix_ring(store_owners)?;
        store_owners.relieve_displaced_pressure()
    }

    /// 🔬️ Test-build oracle of every incremental mirror on short histories ([`MIRROR_ORACLE_HISTORY`]): the persisted cursor
    /// equals the live stacks, the content revision equals a revision rebuilt from scratch, and a claimed fold frontier is
    /// the fold of the log.
    #[doc(hidden)]
    pub fn assert_mirrors_are_live(&self, store_log: &StoreLog<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, store_live: &StoreLive<P, Mutation>) {
        let length = store_log.applied_edit_ids.len();
        if length > MIRROR_ORACLE_HISTORY || (length > MIRROR_ORACLE_EVERY_HISTORY && length % MIRROR_ORACLE_STRIDE != 0) {
            return;
        }
        let cursor = store_log.envelope.cursor.as_ref().expect("a bumped store holds its cursor");
        assert!(cursor.applied_edit_ids == *store_log.applied_edit_ids && cursor.redo_edit_ids == *store_log.redo_edit_ids && cursor.checkpoint_id == *store_log.current_checkpoint_id, "the persisted cursor mirrors the live stacks");
        let mut rebuilt = CursorRevisionAccumulator { identity_digest: store_cursor.revision_accumulator.identity_digest, applied: semio_framework_os_vcs::HistoryPageStack::new(), redo: semio_framework_os_vcs::HistoryPageStack::new(), applied_tail_chains: None, mutation_positions: protocol::HistoryFoldIndex::new(), indexed_edits: protocol::HistoryFoldIndex::new(), unit_flags: protocol::HistoryFoldIndex::new() };
        rebuilt.reconcile::<P, Mutation>(&store_log.applied_edit_ids, &store_log.redo_edit_ids, &store_log.envelope.vcs.edits, &store_live.supersessions, StackMirrorDirt::ALL, None);
        for (stack, live, scratch) in [("applied", &store_cursor.revision_accumulator.applied, &rebuilt.applied), ("redo", &store_cursor.revision_accumulator.redo, &rebuilt.redo)] {
            let stale = live.iter().zip(scratch.iter()).position(|(live, scratch)| live.id_digest != scratch.id_digest || live.edit_digest != scratch.edit_digest);
            assert!(stale.is_none() && live.len() == scratch.len(), "the incremental {stack} revision records equal records rebuilt from scratch: first stale record {stale:?}, {} live vs {} rebuilt", live.len(), scratch.len());
        }
        assert_eq!(rebuilt.revision(store_log.current_checkpoint_id.as_deref()), store_cursor.content_revision, "the incremental content revision equals one rebuilt from scratch");
        if self.fold_is_live(store_live) {
            let fold = fold_envelope_history(&store_log.envelope).expect("a proven fold refolds");
            assert!(
                fold.applied == *store_log.applied_edit_ids && fold.redo == *store_log.redo_edit_ids && fold.checkpoint == *store_log.current_checkpoint_id && fold.alternative == store_log.envelope.active_alternative_id && fold.supersessions == *store_live.supersessions,
                "a claimed fold frontier is the fold of the log"
            );
        }
    }
}
//#endregion 🔖️ArtifactStore
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority

//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
impl<P, Mutation> StoreLive<P, Mutation>
where
    P: Clone + PartialEq + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    /// 🟰️ Lets every Report replay of this store stop where its state meets the live projection's state after the same
    /// prefix (`Arc::ptr_eq`, then `PartialEq`), splicing the unchanged tail instead of replaying it.
    pub fn enable_convergence_early_exit(&mut self) {
        let equal: fn(&P, &P) -> bool = |left, right| left == right;
        self.snapshot_equality = Some(equal);
    }
}
//#endregion 🔖️ArtifactStore
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority

//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
/// 🧾️ One applied operation as history editing addresses it. `effective` describes the operation that folds — the
/// replacement's leaf (semantic kind, display name) when a valid supersession replaces the recorded input, which the
/// recorded `meta` never restamps.
pub struct AppliedMutation<'a, Mutation> {
    pub mutation_id: MutationId,
    pub edit_id: &'a str,
    pub position: usize,
    pub op_index: u32,
    pub operation: &'a Mutation,
    pub meta: Option<&'a MutationMeta>,
    pub transaction: Option<&'a protocol::TransactionRef>,
    pub supersession: Option<&'a protocol::EffectiveSupersession>,
    pub effective: &'static semio_framework_os_spr::MutationLeafDescriptor,
    /// 🫷️ The cross-artifact unit the operation belongs to, `None` when it is its own unit ([`ArtifactStore::unit_id`]): an
    /// operation of a unit takes no supersession in one document alone.
    pub unit: Option<String>,
}
//#endregion 🔖️ArtifactStore
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority

//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
/// 🧊️ Cold-retires a scratch edit copy — a decoded arrival that history already holds, a replay
/// lookup clone, a conflict listing — through its operations' [`Mutation::retire_cold`].
/// 🕸️ Every transition among `transitions` that `refused` names or that causally depends — through `dependencies`,
/// transitively — on one that does, as ids in log order: what one refusal retracts. One law for a store's event log and a
/// persisted `.spr` log ([`retract_history_transitions_from_spr`]), so both retract exactly the same transitions.
pub fn transition_retraction_closure<'a, T>(transitions: &'a [T], identity: impl Fn(&'a T) -> &'a str, dependencies: impl Fn(&'a T) -> Vec<&'a str>, refused: &[String]) -> Vec<String> {
    let mut retracted: HashSet<&'a str> = transitions.iter().map(&identity).filter(|id| refused.iter().any(|name| name == id)).collect();
    loop {
        let before = retracted.len();
        for transition in transitions {
            if dependencies(transition).into_iter().any(|dependency| retracted.contains(dependency)) {
                retracted.insert(identity(transition));
            }
        }
        if retracted.len() == before {
            break;
        }
    }
    transitions.iter().map(&identity).filter(|id| retracted.contains(id)).map(str::to_string).collect()
}
//#endregion 🔖️ArtifactStore
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority

//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
/// 📦️ `envelopes`, in authoring order, as the fewest document-backbone batches its declaration admits
/// (`crate::os_spr::DOCUMENT_BACKBONE_BATCH_MAXIMUM_*`): a committed tool transaction of many operations leaves as several
/// batches the hub takes one by one, never as one batch it refuses as undeclared.
pub fn document_backbone_batches(envelopes: Vec<semio_framework_os_spr::MutationEnvelope>) -> Vec<Vec<semio_framework_os_spr::MutationEnvelope>> {
    const FRAMING_BYTES: usize = 16;
    let mut batches = Vec::new();
    let mut batch: Vec<semio_framework_os_spr::MutationEnvelope> = Vec::new();
    let (mut bytes, mut dependencies, mut segments) = (FRAMING_BYTES, 0usize, 0usize);
    for envelope in envelopes {
        let mut encoded = Vec::new();
        semio_framework_os_spr::encode_envelope(&envelope, &mut encoded);
        let full = bytes + encoded.len() > semio_framework_os_spr::DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES
            || batch.len() == semio_framework_os_spr::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES
            || dependencies + envelope.dependencies.len() > semio_framework_os_spr::DOCUMENT_BACKBONE_BATCH_MAXIMUM_DEPENDENCIES
            || segments + envelope.target.len() > semio_framework_os_spr::DOCUMENT_BACKBONE_BATCH_MAXIMUM_TARGET_SEGMENTS;
        if full && !batch.is_empty() {
            batches.push(std::mem::take(&mut batch));
            (bytes, dependencies, segments) = (FRAMING_BYTES, 0, 0);
        }
        bytes += encoded.len();
        dependencies += envelope.dependencies.len();
        segments += envelope.target.len();
        batch.push(envelope);
    }
    if !batch.is_empty() {
        batches.push(batch);
    }
    batches
}
//#endregion 🔖️ArtifactStore
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority

//#endregion 🔖️Backbone
//#endregion 🔖️Backbone
//#endregion 🔖️EphemeralLanes
