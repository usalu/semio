//! 📥️ Items of `durable_group` that belong to `os-store-engine-stage` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🧩️composition/🗄️durable-group/🦀️.rs`.
#[path = "../../../../🧩️composition/🗄️durable-group/🧵️planning/🦀️.rs"]
pub mod planning;
use semio_framework_os_store_durable_group_wire::lifted_durable_group::*;
use semio_framework_os_store_one_item_publication::lifted_canonical_edit as canonical_edit;
use semio_framework_pack_json::ArtifactCanonicalJsonTree;
use semio_framework_os_store_one_item_publication::diagnostics::ArtifactStoreDiagnosticStage;
use semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPreparedParts;
use semio_framework_os_store_ledger::CursorRevisionAccumulator;
use pack::snapshot::ArtifactPack;
use semio_framework_value::FromValue as ValueFromValue;
use semio_framework_value::ToValue as ValueToValue;
use semio_framework_os_spr::{Edit, HybridLogicalTimestamp, Mutation as StoreMutation};
use semio_framework_os_store_engine_live::lifted_store::{StoreParts, StorePartsRef};
use semio_framework_os_store_envelope::owner::OwnerRef;
use semio_framework_os_pack::{PackDecodeOptions, PackEncodeOptions, PackVerificationLevel};
use semio_framework_os_store_one_item_publication::{ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPrepared};
use std::sync::Arc;
use std::mem::size_of;
use semio_framework_value::ErasedSnapshotRetirement as _;
use semio_framework_os_store_durable_group_wire::lifted_durable_group::DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES;
use semio_framework_os_store_durable_group_wire::lifted_durable_group::DURABLE_OWNED_GROUP_BOUND_OUTCOME_SCHEMA_V1;
use semio_framework_os_store_durable_group_wire::lifted_durable_group::DurableOwnedGroupMemberV1;
use semio_framework_os_store_durable_group_wire::lifted_durable_group::DurableOwnedGroupDecisionError;
use semio_framework_os_store_durable_group_wire::lifted_durable_group::DurableBoundOneItemOutcomeV1;
use semio_framework_os_store_durable_group_wire::lifted_durable_group::DurableStorePreparedOutcomeV1;
use semio_framework_os_store_durable_group_wire::lifted_durable_group::ArtifactStoreDurableGroupRootV1;
use semio_framework_os_store_durable_group_wire::lifted_durable_group::StagedRootRetirement;
use semio_framework_os_store_durable_group_wire::lifted_durable_group::retained_string_stack;
use semio_framework_os_store_durable_group_wire::lifted_durable_group::retained_revision_stack;
use semio_framework_os_store_durable_group_wire::lifted_durable_group::retain_displaced_owner;
use semio_framework_os_store_durable_group_wire::lifted_durable_group::parse_canonical_json_value;
use semio_framework_os_store_durable_group_wire::lifted_durable_group::valid_edit_identities;
use semio_framework_os_store_durable_group_wire::lifted_durable_group::DurableMemberRecoveryFrontier;
use semio_framework_os_store_durable_group_wire::lifted_durable_group::valid_hash;
pub struct DurableStorePreparedOwnerV1<P, Mutation> {
    retirement_progress:semio_framework_value::RetainedCloneProgress,
    #[cfg(feature = "deflate")]
    member_encoding: planning::DurableUnboundPlanningV1,
    #[cfg(feature = "deflate")]
    bound_encoding: planning::DurableBoundPlanningV1,
    prepared: Option<ArtifactStoreOneItemPrepared<P, Mutation>>,
    unbound: Option<DurableStorePreparedOutcomeV1>,
    plan: Option<DurableStoreBindingPlanV1>,
    bound: Option<DurableStoreBoundOutcomeV1<P, Mutation>>,
    previous_authority: Option<semio_framework_os_store_one_item_publication::lifted_canonical_edit::ArtifactStoreOneItemAuthorityRetirement>,
    discarded_pack: Option<semio_framework_value::retirement::controlled::ControlledRetirement<DurablePlanningOutputV1>>,
    mutation_retirement: Option<Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<Mutation>>>,
    snapshot_retirement: Option<Arc<dyn semio_framework_value::SnapshotRetirementFactory<P>>>,
}

struct DurableStoreBindingPlanV1 {
    authority: Arc<ArtifactStoreOneItemLiveAuthority>,
    group_id: Option<String>,
    edit_digest: [u8; 32],
    recovery_pack: Vec<u8>,
    recovery_pack_sha256: String,
    expected_generation: u64,
    expected_revision: [u8; 32],
    post_generation: u64,
    post_revision: [u8; 32],
}

fn durable_member_frame_copy_demand<P: ValueToValue,M:StoreMutation<P>+ValueToValue>(_:&StorePartsRef<'_,P,M>)->usize{size_of::<DurableStorePreparedOwnerV1<P,M>>()+size_of::<Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<M>>>()+size_of::<Arc<dyn semio_framework_value::SnapshotRetirementFactory<P>>>()}

pub struct DurableStoreBoundOutcomeV1<P, Mutation> {
    pub prepared: ArtifactStoreOneItemPrepared<P, Mutation>,
    mutation_retirement: Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<Mutation>>,
    snapshot_retirement: Arc<dyn semio_framework_value::SnapshotRetirementFactory<P>>,
    unbound_sha256: String,
    pub recovery_schema: String,
    pub recovery_pack: Vec<u8>,
    pub recovery_pack_sha256: String,
    pub expected_generation: u64,
    pub expected_revision: [u8; 32],
    pub post_generation: u64,
    pub post_revision: [u8; 32],
}

pub fn stage_store_member<P, Mutation>(
    store: &mut StoreParts<'_, P, Mutation>,
    outcome: DurableStoreBoundOutcomeV1<P, Mutation>,
    visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>,
    grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant,
) -> Result<(), (DurableOwnedGroupDecisionError, DurableStoreBoundOutcomeV1<P, Mutation>)>
where
    P: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    Mutation: StoreMutation<P> + Clone + ValueToValue + ValueFromValue + Send + 'static,
{
    let reject = |reason, outcome| Err((reason, outcome));
    if !grant.permits_one() || grant.maximum_capacity_bytes < size_of::<StagedRootRetirement<P, Mutation>>() { return reject(DurableOwnedGroupDecisionError::InvalidFrontier, outcome); }

    if store.wire.durable_group_root.is_some()
        || !visibility.pending()
        || store.wire.backbone.is_some()
        || store.owners.snapshot_retirement_factory.is_none()
        || store.owners.mutation_retirement_factory.is_none()
        || store.log.envelope.vcs.edits.group_visibility().is_some()
        || store.log.envelope.cursor.as_ref().is_none_or(|cursor| cursor.group_visibility().is_some())
    {
        return reject(DurableOwnedGroupDecisionError::InvalidFrontier, outcome);
    }
    if outcome.prepared.authority().validate_prepared(&outcome.prepared).is_err()
        || store.cursor.generation != outcome.expected_generation
        || store.cursor.content_revision != outcome.expected_revision
        || outcome.expected_generation.checked_add(1) != Some(outcome.post_generation)
        || outcome.prepared.authority().generation().0 != outcome.expected_generation
        || outcome.prepared.authority().base_revision() != outcome.expected_revision
        || outcome.prepared.authority().base_applied_edit_count() != store.log.applied_edit_ids.len()
        || !outcome.prepared.applied_edit_id.eq_str(&outcome.prepared.edit().id)
        || !outcome.prepared.tail_edit_id.eq_str(&outcome.prepared.edit().id)
        || outcome.prepared.edit().sequence_number != outcome.prepared.authority().next_sequence_number()
        || outcome.prepared.next_clock() != outcome.prepared.authority().next_clock()
        || !store.log.applied_edit_ids.admits_one()
    {
        return reject(DurableOwnedGroupDecisionError::InvalidOutcome, outcome);
    }
    let group_id = outcome.prepared.authority().group_id().unwrap_or_default();
    if !valid_hash(group_id) || outcome.prepared.edit().mutation_meta.first().and_then(|meta| meta.group_id.as_deref()) != Some(group_id) {
        return reject(DurableOwnedGroupDecisionError::InvalidHash, outcome);
    }
    let applied_edit_ids = match retained_string_stack(store.log.applied_edit_ids.iter().cloned(), Some(&outcome.prepared.edit().id)) {
        Ok(values) => values,
        Err(error) => return reject(error, outcome),
    };
    let redo_edit_ids = match retained_string_stack(std::iter::empty(), None) {
        Ok(values) => values,
        Err(error) => return reject(error, outcome),
    };
    let cursor_owners = semio_framework_os_store_space_schema::ArtifactCursorOwners {
        applied_edit_ids: match retained_string_stack(store.log.applied_edit_ids.iter().cloned(), Some(&outcome.prepared.edit().id)) {
            Ok(values) => values,
            Err(error) => return reject(error, outcome),
        },
        redo_edit_ids: match retained_string_stack(std::iter::empty(), None) {
            Ok(values) => values,
            Err(error) => return reject(error, outcome),
        },
        checkpoint_id: (*store.log.current_checkpoint_id).clone(),
    };
    let mut revision_accumulator = CursorRevisionAccumulator {
        identity_digest: store.cursor.revision_accumulator.identity_digest,
        applied: match retained_revision_stack(store.cursor.revision_accumulator.applied.iter().cloned()) {
            Ok(values) => values,
            Err(error) => return reject(error, outcome),
        },
        redo: match retained_revision_stack(std::iter::empty()) {
            Ok(values) => values,
            Err(error) => return reject(error, outcome),
        },
        applied_tail_chains: None,
        mutation_positions: store.cursor.revision_accumulator.mutation_positions.clone(),
        indexed_edits: store.cursor.revision_accumulator.indexed_edits.clone(),
        unit_flags: store.cursor.revision_accumulator.unit_flags.clone(),
    };
    revision_accumulator.index_applied_edit::<P, Mutation>(applied_edit_ids.len() - 1, &outcome.prepared.edit());
    let edit_digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[outcome.prepared.edit().id.as_bytes()]);
    for (index, operation) in outcome.prepared.edit().forwards.iter().enumerate().filter(|(_, op)| op.may_emit_foreign_steps()) {
        let presence = match operation.foreign_step_source(&store.live.current, 0) { Ok(source) => source.is_some(), Err(_) => return reject(DurableOwnedGroupDecisionError::InvalidOutcome, outcome) };
        revision_accumulator.unit_flags.insert((edit_digest, index), presence);
    }
    let previous = revision_accumulator.applied.last().map_or(revision_accumulator.identity_digest, |record| record.prefix_digest);
    revision_accumulator.applied.push(semio_framework_os_store_ledger::CursorRevisionRecord::new(
        None,
        CursorRevisionAccumulator::hash_record(b"edit-id", &[outcome.prepared.edit().id.as_bytes()]),
        outcome.prepared.edit_digest(),
        CursorRevisionAccumulator::hash_record(b"applied", &[&previous, &outcome.prepared.edit_digest()]),
    ));
    if revision_accumulator.revision(store.log.current_checkpoint_id.as_deref()) != outcome.post_revision {
        return reject(DurableOwnedGroupDecisionError::InvalidFrontier, outcome);
    }
    let displaced_reservation = match store.owners.displaced_retirements.reserve_owner_slots(12) {
        Ok(reservation) => reservation,
        Err(_) => return reject(DurableOwnedGroupDecisionError::InvalidFrontier, outcome),
    };
    let history_reservation = match store.log.envelope.vcs.edits.reserve_group_one(visibility) {
        Ok(reservation) => reservation,
        Err(_) => {
            store.owners.displaced_retirements.release_owner_slots(displaced_reservation).expect("unconsumed durable group retirement reservation remains exact");
            return reject(DurableOwnedGroupDecisionError::InvalidFrontier, outcome);
        }
    };
    store.log.envelope.cursor.as_mut().expect("durable group stage preflight retained its cursor").stage_group_owned(cursor_owners, visibility).expect("durable group cursor stage remains exact after exclusive preflight");
    let mut retirement = Box::new(StagedRootRetirement::new(Arc::clone(store.owners.snapshot_retirement_factory.as_ref().expect("validated staged snapshot factory"))));
    let post_revision = outcome.post_revision;
    let DurableStoreBoundOutcomeV1 { prepared, mutation_retirement, snapshot_retirement, unbound_sha256, recovery_schema, recovery_pack, recovery_pack_sha256, .. } = outcome;
    let mut retirement = Box::new(StagedRootRetirement::new(snapshot_retirement));
    retirement.planning_output=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(DurablePlanningOutputV1::Bound{unbound_sha256,recovery_schema,recovery_pack,recovery_pack_sha256}).unwrap_or_else(|_|unreachable!("encoded bound fields support controlled retirement")));
    let ArtifactStoreOneItemPreparedParts { outcome_messages, edit, post_snapshot, next_clock, edit_digest: _, local_actor, applied_edit_id, tail_edit_id, authority, .. } = prepared.into_parts();
    let staged_tail_id=tail_edit_id.to_string_owner();
    retirement.external_strings = [local_actor, Some(applied_edit_id.into()),Some(tail_edit_id)];
    let ledger_key = store.log.envelope.vcs.edits.stage_group_reserved(history_reservation, *edit, visibility).unwrap_or_else(|_| panic!("durable group history stage remains exact after its exclusive reservation"));
    let mut tail = revision_accumulator.applied.pop().expect("staged revision retains its exact tail");
    tail.ledger_key = Some(ledger_key);
    revision_accumulator.applied.push(tail);
    *store.wire.durable_group_root = Some(ArtifactStoreDurableGroupRootV1 {
        diagnostics: semio_framework_os_store_one_item_publication::diagnostics::ArtifactStoreDiagnosticStage::from_publication(outcome_messages),
        visibility: Arc::clone(visibility),
        current: Some(post_snapshot),
        generation: store.cursor.generation + 1,
        content_revision: post_revision,
        applied_edit_ids,
        redo_edit_ids,
        last_projection_cause: Some(semio_framework_os_store_space_schema::ArtifactProjectionCause::Apply),
        edit_sequence: authority.next_sequence_number(),
        clock: next_clock,
        revision_accumulator: Some(revision_accumulator),
        tail_undo_cache: Some((tail_edit_id.to_string_owner(), Arc::clone(&store.live.current))),
        authority: Some(authority),
        mutation_retirement,
        displaced_reservation: Some(displaced_reservation),
        retirement: Some(retirement),
        adopted: false,
    });
    Ok(())
}

/// 🧱️ Transfers already copied catalogs and exact reserved owners without traversing history.
pub fn stage_prebuilt_batch_root<P, Mu>(store: &mut StoreParts<'_, P, Mu>, publication: &mut semio_framework_os_store_one_item_batch::lifted_one_item_publication::ArtifactStoreBatchPublication<P, Mu>, grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::RetainedCloneProgress, String>
where P: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
      Mu: StoreMutation<P> + Clone + ValueToValue + ValueFromValue + Send + 'static,
{
    if store.wire.durable_group_root.is_some() || store.log.envelope.cursor.as_ref().is_none_or(|cursor| cursor.group_visibility().is_some()) { return Err("batch group final root transfer lost its exclusive cursor frontier".into()); }
    let capacity=size_of::<StagedRootRetirement<P,Mu>>();let release=size_of::<Edit<Mu>>();
    if !grant.permits_one() || grant.maximum_capacity_bytes < capacity || grant.maximum_release_bytes < release { return Err("batch staged original retirement and edit frames are not funded".into()); }
    store.cursor.generation.checked_add(1).ok_or_else(|| "batch group generation is exhausted".to_string())?;
    let authority = Arc::clone(publication.authority.as_ref().ok_or_else(|| "batch group final transfer lost its authority".to_string())?);
    let group = publication.group_preparation.as_mut().ok_or_else(|| "batch group final transfer lost its retained preparation".to_string())?;
    let visibility = group.visibility();
    let history = group.history.as_ref().ok_or_else(|| "batch group final transfer lost its history reservation".to_string())?;
    store.log.envelope.vcs.edits.bind_group_reservation(&history.history, &visibility).map_err(|()| "batch group history reservation is stale".to_string())?;
    let mut retirement = Box::new(StagedRootRetirement::new(Arc::clone(store.owners.snapshot_retirement_factory.as_ref().ok_or("batch group lacks original snapshot factory")?)));
    let history = group.history.take().ok_or_else(|| "batch group final transfer lost its history reservation".to_string())?;
    let displaced = group.displaced.take().expect("copied group admits its displaced owner catalog");
    let cursor = semio_framework_os_store_space_schema::ArtifactCursorOwners { applied_edit_ids: std::mem::replace(&mut group.cursor, semio_framework_os_vcs::HistoryPageStack::empty()), redo_edit_ids: semio_framework_os_vcs::HistoryPageStack::empty(), checkpoint_id: group.checkpoint.take() };
    let applied = std::mem::replace(&mut group.applied, semio_framework_os_vcs::HistoryPageStack::empty());
    let mut revision = group.revision.take().expect("copied group retains its complete revision and indexes");
    let stage = publication.stage.as_mut().expect("copied group retains its complete typed candidate");
    let edit = stage.edit.take().expect("staged original edit owner");
    let post=stage.post.take();let next_clock=stage.next_clock;let local_actor=stage.local_actor.take();let tail_edit_id=std::mem::take(&mut stage.tail_edit_id);
    store.log.envelope.cursor.as_mut().expect("exclusive group transfer retains its cursor").stage_group_owned(cursor, &visibility).unwrap_or_else(|_| panic!("exclusive copied group cursor transfer must retain its exact owners"));
    let ledger = store.log.envelope.vcs.edits.stage_group_reserved(history.history, *edit, &visibility).unwrap_or_else(|_| panic!("exclusive copied group edit transfer must retain its reserved slot"));
    store.owners.displaced_retirements.release_owner_slots(history.rejected_owner).expect("exclusive copied group retains its exact rejected-owner reservation");
    retirement.external_strings = [local_actor, None,None,None];
    let mut tail = revision.applied.pop().expect("copied group retains its new revision tail");
    tail.ledger_key = Some(ledger); revision.applied.push(tail);
    let content_revision = revision.revision(store.log.current_checkpoint_id.as_deref());
    *store.wire.durable_group_root = Some(ArtifactStoreDurableGroupRootV1 {

        diagnostics: std::mem::take(&mut stage.diagnostics),
        visibility, current: post, generation: store.cursor.generation + 1, content_revision,
        applied_edit_ids: applied, redo_edit_ids: semio_framework_os_vcs::HistoryPageStack::empty(),
        last_projection_cause: Some(semio_framework_os_store_space_schema::ArtifactProjectionCause::Apply), edit_sequence: authority.next_sequence_number(),
        clock: next_clock, revision_accumulator: Some(revision), tail_undo_cache: Some((tail_edit_id, Arc::clone(&store.live.current))),
        authority: Some(authority), mutation_retirement: Arc::clone(store.mutation_retirement_factory.as_ref().expect("admitted batch retains original mutation issuer")),  mutation_retirement: Arc::clone(store.mutation_retirement_factory.as_ref().expect("admitted batch retains original mutation issuer")), displaced_reservation: Some(displaced), retirement: Some(retirement), adopted: false,
    });
    Ok(semio_framework_value::RetainedCloneProgress{copied_items:1,retained_capacity_bytes:capacity,released_bytes:release,..Default::default()})
}

pub fn abort_staged_store_member<P, Mutation>(store: &mut StoreParts<'_, P, Mutation>, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>) -> Result<(), DurableOwnedGroupDecisionError>
where
    P: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    Mutation: StoreMutation<P> + Clone + ValueToValue + ValueFromValue + Send + 'static,
{
    let Some(root) = store.wire.durable_group_root.as_ref() else { return Ok(()) };
    if !Arc::ptr_eq(&root.visibility, visibility) || root.adopted || visibility.pending() || visibility.committed() {
        return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
    }
    let mutation_factory = (*store.owners.mutation_retirement_factory).clone(&root.mutation_retirement);
    if root.retirement.is_none() || root.displaced_reservation.is_none() { return Err(DurableOwnedGroupDecisionError::InvalidOutcome); }
    let edit = store.log.envelope.vcs.edits.abort_group_one(visibility).map_err(|()| DurableOwnedGroupDecisionError::InvalidFrontier)?.ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    if store.log.envelope.vcs.edits.abort_group_one(visibility).map_err(|()| DurableOwnedGroupDecisionError::InvalidFrontier)?.is_some() { return Err(DurableOwnedGroupDecisionError::InvalidOutcome); }
    let cursor = store.log.envelope.cursor.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?.abort_group_owned(visibility).map_err(|()| DurableOwnedGroupDecisionError::InvalidFrontier)?;
    let mut root = store.wire.durable_group_root.take().expect("validated aborted durable group root remains owned");
    let mut reservation = root.displaced_reservation.take().expect("validated original abort reservation");
    let mut retirement = root.retirement.take().expect("validated preborn abort cursor");
    retirement.diagnostics=Some(std::mem::take(&mut root.diagnostics));
    retirement.edit = Some(semio_framework_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirement::new(edit, mutation_factory));
    retirement.cursor = Some(semio_framework_os_store_read::ArtifactStoreCursorRetirement::new(semio_framework_os_store_space_schema::ArtifactCursor::from_owners(cursor)));
    retirement.snapshots[0] = root.current.take();
    retirement.applied = Some(semio_framework_os_store_read::ArtifactStoreStringVectorRetirement::new(std::mem::replace(&mut root.applied_edit_ids, semio_framework_os_vcs::HistoryPageStack::empty())));
    retirement.redo = Some(semio_framework_os_store_read::ArtifactStoreStringVectorRetirement::new(std::mem::replace(&mut root.redo_edit_ids, semio_framework_os_vcs::HistoryPageStack::empty())));
    retirement.revision = root.revision_accumulator.take().map(semio_framework_os_store_read::ArtifactStoreRevisionAccumulatorRetirement::new);
    if let Some((edit_id, snapshot)) = root.tail_undo_cache.take() { retirement.tail_id = Some(semio_framework_os_store_read::ArtifactStoreStringRetirement::new(edit_id)); if !Arc::ptr_eq(&snapshot, &store.live.current) && !std::ptr::eq(snapshot.as_ref(), store.log.envelope.vcs.genesis.facts().snapshot()) { retirement.snapshots[1] = Some(snapshot); } }
    retirement.authority = root.authority.take().map(semio_framework_os_store_one_item_publication::lifted_canonical_edit::ArtifactStoreOneItemAuthorityRetirement::new);
    retain_displaced_owner(&mut store.owners.displaced_retirements, &mut reservation, retirement);
    store.owners.displaced_retirements.release_owner_slots(reservation).map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?;
    Ok(())
}

/// 🪢️ Staging cached the pre-commit snapshot as this group's tail-undo entry
/// (`Arc::clone(&store.current)`), so the outgoing current and the incoming tail are the SAME
/// owner. Displacing it here would queue a retirement the tail cache still aliases, and the
/// close cursor drains displaced owners nine phases BEFORE `TailSnapshot` — it would meet an
/// `Arc` it cannot unwrap and answer `Blocked` forever. The tail cache keeps the owner instead,
/// exactly as the symmetric guards below and in `commit_document_roots_retained` do.
pub fn adopt_staged_store_member<P, Mutation>(store: &mut StoreParts<'_, P, Mutation>, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, grant:semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(bool,semio_framework_value::retained_clone::RetainedCloneProgress), DurableOwnedGroupDecisionError>
where
    P: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    Mutation: StoreMutation<P> + Clone + ValueToValue + ValueFromValue + Send + 'static,
{
    let Some(root) = store.wire.durable_group_root.as_ref() else { return Err(DurableOwnedGroupDecisionError::InvalidFrontier) };
    if !Arc::ptr_eq(&root.visibility, visibility) || root.adopted || !visibility.committed() {
        return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
    }
    let factory = store.owners.snapshot_retirement_factory.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    if root.retirement.as_ref().and_then(|retirement| retirement.factory.as_ref()).is_none_or(|prepared| !Arc::ptr_eq(prepared, factory)) {
        return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
    }
    if root.diagnostics.row_count()!=0{if !root.diagnostics.prepared(){return Err(DurableOwnedGroupDecisionError::InvalidOutcome);}let copy=semio_framework_os_store_one_item_publication::diagnostics::ArtifactStoreDiagnosticStage::transfer_copy_demand();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy{return Ok((false,Default::default()));}}
    if root.diagnostics.row_count()!=0{if !root.diagnostics.prepared(){return Err(DurableOwnedGroupDecisionError::InvalidOutcome);}let copy=semio_framework_os_store_one_item_publication::diagnostics::ArtifactStoreDiagnosticStage::transfer_copy_demand();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy{return Ok((false,Default::default()));}}
    store.log.envelope.vcs.edits.adopt_group(visibility).map_err(|()| DurableOwnedGroupDecisionError::InvalidFrontier)?;
    let displaced_cursor = store.log.envelope.cursor.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?.adopt_group_owned(visibility).map_err(|()| DurableOwnedGroupDecisionError::InvalidFrontier)?;
    let mut root = store.wire.durable_group_root.take().expect("validated committed durable group root remains owned");
    let mut reservation = root.displaced_reservation.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    let mut retirement = root.retirement.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    retirement.cursor = Some(semio_framework_os_store_read::ArtifactStoreCursorRetirement::new(semio_framework_os_store_space_schema::ArtifactCursor::from_owners(displaced_cursor)));
    retirement.applied = Some(semio_framework_os_store_read::ArtifactStoreStringVectorRetirement::new(std::mem::replace(&mut *store.log.applied_edit_ids, std::mem::replace(&mut root.applied_edit_ids, semio_framework_os_vcs::HistoryPageStack::empty()))));
    retirement.redo = Some(semio_framework_os_store_read::ArtifactStoreStringVectorRetirement::new(std::mem::replace(&mut *store.log.redo_edit_ids, std::mem::replace(&mut root.redo_edit_ids, semio_framework_os_vcs::HistoryPageStack::empty()))));
    let next_revision = root.revision_accumulator.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    retirement.revision = Some(semio_framework_os_store_read::ArtifactStoreRevisionAccumulatorRetirement::new(std::mem::replace(&mut *store.cursor.revision_accumulator, next_revision)));
    let previous_current = std::mem::replace(&mut *store.live.current, root.current.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?);
    let next_tail = root.tail_undo_cache.take();
    if next_tail.as_ref().is_some_and(|(_, snapshot)| Arc::ptr_eq(snapshot, &previous_current)) || std::ptr::eq(previous_current.as_ref(), store.log.envelope.vcs.genesis.facts().snapshot()) {
        drop(previous_current);
    } else {
        retirement.snapshots[0] = Some(previous_current);
    }
    if let Some((edit_id, snapshot)) = store.live.tail_undo_cache.take() {
        retirement.tail_id = Some(semio_framework_os_store_read::ArtifactStoreStringRetirement::new(edit_id));
        if !Arc::ptr_eq(&snapshot, &store.live.current) && !std::ptr::eq(snapshot.as_ref(), store.log.envelope.vcs.genesis.facts().snapshot()) {
            retirement.snapshots[1] = Some(snapshot);
        }
    }
    *store.live.tail_undo_cache = next_tail;
    retirement.report = Some(semio_framework_os_store_ledger::ArtifactStorePendingReportRetirement::new(std::mem::take(&mut *store.replay.pending_report)));
    let diagnostic_receipt=store.publish_prepared_diagnostics(&mut root.diagnostics,grant).map_err(|_|DurableOwnedGroupDecisionError::InvalidOutcome)?;


    let diagnostic_receipt=store.publish_prepared_diagnostics(&mut root.diagnostics,grant).map_err(|_|DurableOwnedGroupDecisionError::InvalidOutcome)?;

    if let Some(authority) = root.authority.take() {
        retirement.authority = Some(semio_framework_os_store_one_item_publication::lifted_canonical_edit::ArtifactStoreOneItemAuthorityRetirement::new(authority));
    }
    retain_displaced_owner(&mut store.owners.displaced_retirements, &mut reservation, retirement);
    store.cursor.generation = root.generation;
    store.cursor.content_revision = root.content_revision;
    store.live.fold_frontier = None;
    store.log.edit_sequence = root.edit_sequence;
    store.log.clock = root.clock;
    store.cursor.last_projection_cause = root.last_projection_cause;
    store.owners.displaced_retirements.release_owner_slots(reservation).map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?;
    root.adopted = true;
    *store.wire.durable_group_root = Some(root);
    Ok((true,diagnostic_receipt))
}

pub fn validate_adopted_store_member<P, Mutation>(store: &StorePartsRef<'_, P, Mutation>, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>) -> Result<(), DurableOwnedGroupDecisionError>
where
    P: ValueToValue,
    Mutation: StoreMutation<P> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue,
{
    let root = store.wire.durable_group_root.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?;
    if !root.adopted
        || !Arc::ptr_eq(&root.visibility, visibility)
        || root.current.is_some()
        || root.revision_accumulator.is_some()
        || root.tail_undo_cache.is_some()
        || root.authority.is_some()
        || root.retirement.is_some()
        || root.displaced_reservation.is_some()
        || !root.applied_edit_ids.is_empty()
        || !root.redo_edit_ids.is_empty()
    {
        return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
    }
    Ok(())
}

pub fn clear_adopted_store_member<P, Mutation>(store: &mut StoreParts<'_, P, Mutation>, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>) -> Result<(), DurableOwnedGroupDecisionError>
where
    P: ValueToValue,
    Mutation: StoreMutation<P> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue,
{
    validate_adopted_store_member(&store.shared(), visibility)?;
    drop(store.wire.durable_group_root.take());
    Ok(())
}

pub fn retire_unstaged_store_member<P, Mutation>(store: &mut StoreParts<'_, P, Mutation>, slot: &mut Option<DurableStoreBoundOutcomeV1<P, Mutation>>, grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant) -> Result<Option<semio_framework_value::RetainedCloneProgress>, DurableOwnedGroupDecisionError>
where
    P: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    Mutation: StoreMutation<P> + Clone + ValueToValue + ValueFromValue + Send + 'static,
{
    if slot.is_none() {return Err(DurableOwnedGroupDecisionError::InvalidOutcome);}
    let copy=size_of::<DurableStoreBoundOutcomeV1<P,Mutation>>() +size_of:: < ArtifactStoreOneItemPrepared<P,Mutation>>()+size_of::<StagedRootRetirement<P, Mutation>>() +size_of::<semio_framework_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirement<Mutation>>()+size_of::<DurablePlanningOutputV1>();
    let capacity=size_of::<StagedRootRetirement<P,Mutation>>();let release=size_of::<Edit<Mutation>>();
    if !grant.permits_one()||grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<capacity||grant.maximum_release_bytes<release{ return Ok(None); }
    let mutation_factory = (*store.owners.mutation_retirement_factory).clone().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    let snapshot_factory = (*store.owners.snapshot_retirement_factory).clone().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    let mut reservation = store.owners.displaced_retirements.reserve_owner_slots(1).map_err(|_| DurableOwnedGroupDecisionError::InvalidFrontier)?;
    let DurableStoreBoundOutcomeV1 { prepared, mutation_retirement, snapshot_retirement, unbound_sha256, recovery_schema, recovery_pack, recovery_pack_sha256, .. } = slot.take().expect("validated unstaged outcome");
    let ArtifactStoreOneItemPreparedParts { outcome_messages, edit, post_snapshot, next_clock: _, edit_digest: _, local_actor, applied_edit_id, tail_edit_id, authority, .. } = prepared.into_parts();
    let mut retirement = Box::new(StagedRootRetirement::new(snapshot_retirement));
    retirement.planning_output=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(DurablePlanningOutputV1::Bound{unbound_sha256,recovery_schema,recovery_pack,recovery_pack_sha256}).unwrap_or_else(|_|unreachable!("encoded bound fields support controlled retirement")));
    retirement.diagnostics=Some(semio_framework_os_store_one_item_publication::diagnostics::ArtifactStoreDiagnosticStage::from_publication(outcome_messages));
    retirement.edit = Some(semio_framework_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirement::new(*edit, mutation_retirement));
    retirement.snapshots[0] = Some(post_snapshot);
    retirement.external_strings = [local_actor, Some(applied_edit_id),Some(tail_edit_id)];
    retirement.authority = Some(semio_framework_os_store_one_item_publication::lifted_canonical_edit::ArtifactStoreOneItemAuthorityRetirement::new(authority));
    retain_displaced_owner(&mut store.owners.displaced_retirements, &mut reservation, retirement);
    store.owners.displaced_retirements.release_owner_slots(reservation).map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)
?;
    Ok(Some(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:capacity,released_bytes:release,..Default::default()}))
}

impl<P, Mutation> DurableStorePreparedOwnerV1<P, Mutation>
where
    P: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot,
    Mutation: ValueToValue + ValueFromValue,
{
    /// 🧷️ Seals one Store-prepared owner with its verified unbound outcome under the member's recovery schema.
    pub fn original(prepared: ArtifactStoreOneItemPrepared<P, Mutation>) -> Self {
        Self { retirement_progress:Default::default(), #[cfg(feature = "deflate")] member_encoding: Default::default(), #[cfg(feature = "deflate")] bound_encoding: Default::default(), prepared: Some(prepared), unbound: None, plan: None, bound: None, previous_authority: None, discarded_pack: None, mutation_retirement: None, snapshot_retirement: None }
    }
}

#[cfg(test)]
impl<P, Mutation> DurableStorePreparedOwnerV1<P, Mutation> {
    fn original_addresses(&self) -> [usize; 6] {
        let Some(original)=self.prepared.as_ref().or_else(||self.bound.as_ref().map(|bound|&bound.prepared))else{return [0;6];};
        [original.edit() as *const Edit<Mutation> as usize, Arc::as_ptr(original.post_snapshot()) as usize, Arc::as_ptr(original.authority()) as usize, original.local_actor().unwrap().chunks().next().unwrap().as_ptr() as usize, original.applied_edit_id().chunks().next().unwrap().as_ptr() as usize, original.tail_edit_id().chunks().next().unwrap().as_ptr() as usize]
    }
}

impl<P, Mutation> DurableStorePreparedOwnerV1<P, Mutation> {
    fn originals_empty(&self) -> bool {
        #[cfg(feature = "deflate")]
        if !self.member_encoding.terminal_is_empty()||!self.bound_encoding.terminal_is_empty(){return false;}
        self.prepared.is_none() && self.unbound.is_none() && self.bound.is_none() && self.previous_authority.is_none() && self.discarded_pack.is_none() && self.plan.is_none() && self.mutation_retirement.is_none() && self.snapshot_retirement.is_none()
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedThreeStorePreparedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
    fn originals_empty(&self) -> bool { self.parent.originals_empty() && self.drawing.originals_empty() && self.value.originals_empty() }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedThreeStorePreparedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + ValueToValue + Send + Sync + 'static,
    ParentMutation: StoreMutation<ParentP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Send + 'static,
    DrawingP: ArtifactPack + ValueToValue + Send + Sync + 'static,
    DrawingMutation: StoreMutation<DrawingP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Send + 'static,
    ValueP: ArtifactPack + ValueToValue + Send + Sync + 'static,
    ValueMutation: StoreMutation<ValueP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Send + 'static,
{
    fn retire_originals_step(&mut self, parent: &mut ArtifactStore<ParentP, ParentMutation>, drawing: &mut ArtifactStore<DrawingP, DrawingMutation>, value: &mut ArtifactStore<ValueP, ValueMutation>, grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant) -> Result<bool, DurableOwnedGroupDecisionError> {
        self.retirement_progress=Default::default();
        if !self.decision_identity.ready(){self.retirement_progress=self.decision_identity.close(grant.retained_grant()).map_err(DurableOwnedGroupDecisionError::Retirement)?;return Ok(false);}
        #[cfg(feature = "deflate")]
        if !self.decision_encoding.terminal_is_empty() { self.retirement_progress=self.decision_encoding.close(grant.retained_grant()).map_err(DurableOwnedGroupDecisionError::Retirement)?;return Ok(false); }
        if !self.parent.originals_empty() { retire_binding_member(&mut self.parent, parent, grant)?;self.retirement_progress=self.parent.retirement_progress; return Ok(false); }
        if !self.drawing.originals_empty() { retire_binding_member(&mut self.drawing, drawing, grant)?;self.retirement_progress=self.drawing.retirement_progress; return Ok(false); }
        if !self.value.originals_empty() { retire_binding_member(&mut self.value, value, grant)?;self.retirement_progress=self.value.retirement_progress; return Ok(false); }
        if let Some(owner)=self.decision_retirement.as_mut(){self.retirement_progress=owner.step(grant.retained_grant()).map_err(DurableOwnedGroupDecisionError::Retirement)?.progress();if owner.terminal_is_empty(){self.decision_retirement=None;}return Ok(false);}
        if let Some(owner)=self.decision_pack_retirement.as_mut(){self.retirement_progress=owner.step(grant.retained_grant()).map_err(DurableOwnedGroupDecisionError::Retirement)?.progress();if owner.terminal_is_empty(){self.decision_pack_retirement=None;}return Ok(false);}
        let copy=size_of::<Box<DurableOwnedThreeMemberDecisionV1>>()+size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<Box<DurableOwnedThreeMemberDecisionV1>>>()+size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<DurablePlanningOutputV1>>()+size_of::<DurablePlanningOutputV1>();
        if self.decision.is_some()||self.decision_pack.is_some(){if !grant.permits_one()||grant.maximum_copy_bytes<copy{return Ok(false);}if let Some(decision)=self.decision.take(){self.decision_retirement=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(decision).map_err(|(reason,_)|DurableOwnedGroupDecisionError::Retirement(reason))?);}else if let Some(pack)=self.decision_pack.take(){self.decision_pack_retirement=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(DurablePlanningOutputV1::Pack(pack)).map_err(|(reason,_)|DurableOwnedGroupDecisionError::Retirement(reason))?);}self.retirement_progress=semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()};return Ok(false);}
        Ok(true)
    }
}

pub fn verify_bound_member_edit<P, Mutation>(member: &DurableOwnedGroupMemberV1, decision_sha256: &str) -> Result<Edit<Mutation>, DurableOwnedGroupDecisionError>
where
    P: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot,
    Mutation: ValueToValue + ValueFromValue,
{
    let outcome = DurableBoundOneItemOutcomeV1::decode_canonical_pack(&member.recovery_pack)?;
    if outcome.recovery_schema != member.recovery_schema
        || outcome.group_id != decision_sha256
        || outcome.base_generation != member.expected_generation
        || outcome.base_revision != member.expected_revision
        || outcome.post_generation != member.post_generation
        || outcome.post_revision != member.post_revision
        || semio_framework_hash::sha256_hex(&outcome.unbound_pack::<Mutation>()?) != member.unbound_outcome_sha256
    {
        return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
    }
    let next_clock: HybridLogicalTimestamp = parse_canonical_json_value(&outcome.next_clock_canonical_json)?;
    let edit: Edit<Mutation> = parse_canonical_json_value(&outcome.edit_canonical_json)?;
    if !valid_edit_identities(&edit) || CursorRevisionAccumulator::edit_digest(&edit) != outcome.edit_digest {
        return Err(DurableOwnedGroupDecisionError::InvalidHash);
    }
    let authority = ArtifactStoreOneItemLiveAuthority::mint(
        semio_framework_job::OperationId(outcome.operation),
        semio_framework_job::Generation(outcome.base_generation),
        outcome.base_revision,
        outcome.base_applied_edit_count.try_into().map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?,
        outcome.next_sequence_number,
        next_clock,
        outcome.actor,
        edit.line.clone(),
        Some(decision_sha256.into()),
        None,
    );
    authority.validate_semantic_edit(&edit).map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?;
    let snapshot = P::decode_pack_with(
        &outcome.post_snapshot_pack,
        &PackDecodeOptions {
            verification: PackVerificationLevel::Full,
            preserve_unknown: true,
            limits: semio_framework_os_pack::PackLimits {
                max_file_len: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                max_segment_len: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                max_symbols: 128,
                max_depth: 32,
                max_items: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                max_total_alloc: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
            },
        },
    )
    .map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
    if snapshot.encode_pack() != outcome.post_snapshot_pack {
        return Err(DurableOwnedGroupDecisionError::NonCanonical);
    }
    Ok(edit)
}

fn store_post_revision<P, Mutation>(store: &StorePartsRef<'_, P, Mutation>, authority: &ArtifactStoreOneItemLiveAuthority, edit_digest: [u8; 32]) -> Result<[u8; 32], DurableOwnedGroupDecisionError>
where
    P: ValueToValue,
    Mutation: ValueToValue + ValueFromValue + StoreMutation<P>,
{
    if store.cursor.generation != authority.generation().0
        || store.cursor.content_revision != authority.base_revision()
        || store.log.applied_edit_ids.len() != authority.base_applied_edit_count()
        || store.cursor.revision_accumulator.applied.len() != authority.base_applied_edit_count()
    {
        return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
    }
    let previous = store.cursor.revision_accumulator.applied.last().map_or(store.cursor.revision_accumulator.identity_digest, |record| record.prefix_digest);
    let applied = CursorRevisionAccumulator::hash_record(b"applied", &[&previous, &edit_digest]);
    let applied_len = authority.base_applied_edit_count().checked_add(1).ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)? as u64;
    let empty_len = 0_u64;
    Ok(CursorRevisionAccumulator::hash_record(b"cursor", &[&applied, &applied_len.to_be_bytes(), &store.cursor.revision_accumulator.identity_digest, &empty_len.to_be_bytes(), store.log.current_checkpoint_id.as_deref().unwrap_or_default().as_bytes()]))
}

impl<P, Mutation> DurableStorePreparedOwnerV1<P, Mutation>
where
    P: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
    Mutation: StoreMutation<P> + ValueToValue + ValueFromValue,
{
    pub fn capture_issuers(&mut self, store: &StorePartsRef<'_, P, Mutation>) -> Result<(), DurableOwnedGroupDecisionError> {
        if self.mutation_retirement.is_none() { self.mutation_retirement = Some((*store.owners.mutation_retirement_factory).clone().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?); }
        if self.snapshot_retirement.is_none() { self.snapshot_retirement = Some((*store.owners.snapshot_retirement_factory).clone().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?); }
        Ok(())
    }

    pub fn transfer(&mut self, group_id: &str, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<semio_framework_value::retained_clone::RetainedCloneProgress>, DurableOwnedGroupDecisionError> {
        let local_copy = size_of::<DurableStoreBoundOutcomeV1<P, Mutation>>() + size_of::<DurableStoreBindingPlanV1>() + size_of::<DurableStorePreparedOutcomeV1>() + size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<DurablePlanningOutputV1>>();
        let copy = semio_framework_os_store_one_item_publication::ArtifactStorePreparedBinding::<P, Mutation>::commit_copy_demand() + local_copy;
        if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < copy { return Ok(None); }
        let plan = self.plan.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        if plan.group_id.as_deref() != Some(group_id) || self.unbound.is_none() || self.prepared.is_none() { return Err(DurableOwnedGroupDecisionError::InvalidOutcome); }
        let group = plan.group_id.take().expect("admitted planning retains its group owner");
        let original = self.prepared.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        let child = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - local_copy, ..grant };
        let (prepared, previous, mut receipt) = match semio_framework_os_store_one_item_publication::ArtifactStorePreparedBinding::commit_group(original, group, Arc::clone(&plan.authority), plan.edit_digest, child) {
            Ok(result) => result,
            Err((error, original, group, _)) => { self.prepared = Some(original); plan.group_id = Some(group); return Err(DurableOwnedGroupDecisionError::Retirement(error)); }
        };
        self.previous_authority = Some(semio_framework_os_store_one_item_publication::lifted_canonical_edit::ArtifactStoreOneItemAuthorityRetirement::new(previous));
        let plan = self.plan.take().unwrap();
        let unbound = self.unbound.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        self.discarded_pack = Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(DurablePlanningOutputV1::Pack(unbound.pack)).unwrap_or_else(|_| unreachable!("native octet output has controlled retirement")));
        self.bound = Some(DurableStoreBoundOutcomeV1 { prepared, mutation_retirement: self.mutation_retirement.take().expect("planning captured original mutation issuer"), snapshot_retirement: self.snapshot_retirement.take().expect("planning captured original snapshot issuer"), unbound_sha256: unbound.sha256, recovery_schema: unbound.recovery_schema, recovery_pack: plan.recovery_pack, recovery_pack_sha256: plan.recovery_pack_sha256, expected_generation: plan.expected_generation, expected_revision: plan.expected_revision, post_generation: plan.post_generation, post_revision: plan.post_revision });
        receipt.copied_bytes += local_copy;
        Ok(Some(receipt))
    }
}

impl<P, Mutation> DurableStoreBoundOutcomeV1<P, Mutation>
where
    P: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
    Mutation: StoreMutation<P> + ValueToValue + ValueFromValue,
{
    pub fn recover(store: &StorePartsRef<'_, P, Mutation>, member: &DurableOwnedGroupMemberV1, group_id: &str) -> Result<Self, DurableOwnedGroupDecisionError> {
        if member.recovery_pack_sha256 != semio_framework_hash::sha256_hex(&member.recovery_pack) {
            return Err(DurableOwnedGroupDecisionError::InvalidHash);
        }
        let outcome = DurableBoundOneItemOutcomeV1::decode_canonical_pack(&member.recovery_pack)?;
        if outcome.recovery_schema != member.recovery_schema
            || outcome.group_id != group_id
            || outcome.base_generation != member.expected_generation
            || outcome.base_revision != member.expected_revision
            || outcome.post_generation != member.post_generation
            || outcome.post_revision != member.post_revision
        {
            return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
        }
        let unbound_pack = outcome.unbound_pack::<Mutation>()?;
        if member.unbound_outcome_sha256 != semio_framework_hash::sha256_hex(&unbound_pack) {
            return Err(DurableOwnedGroupDecisionError::InvalidHash);
        }
        let next_clock: HybridLogicalTimestamp = parse_canonical_json_value(&outcome.next_clock_canonical_json)?;
        let edit: Edit<Mutation> = parse_canonical_json_value(&outcome.edit_canonical_json)?;
        if !valid_edit_identities(&edit) {
            return Err(DurableOwnedGroupDecisionError::InvalidIdentity);
        }
        if CursorRevisionAccumulator::edit_digest(&edit) != outcome.edit_digest {
            return Err(DurableOwnedGroupDecisionError::InvalidHash);
        }
        let post_snapshot = Arc::new(
            P::decode_pack_with(
                &outcome.post_snapshot_pack,
                &PackDecodeOptions {
                    verification: PackVerificationLevel::Full,
                    preserve_unknown: true,
                    limits: semio_framework_os_pack::PackLimits {
                        max_file_len: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                        max_segment_len: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                        max_symbols: 128,
                        max_depth: 32,
                        max_items: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                        max_total_alloc: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                    },
                },
            )
            .map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?,
        );
        if post_snapshot.encode_pack() != outcome.post_snapshot_pack {
            return Err(DurableOwnedGroupDecisionError::NonCanonical);
        }
        let authority = Arc::new(ArtifactStoreOneItemLiveAuthority::mint(
            semio_framework_job::OperationId(outcome.operation),
            semio_framework_job::Generation(outcome.base_generation),
            outcome.base_revision,
            outcome.base_applied_edit_count.try_into().map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?,
            outcome.next_sequence_number,
            next_clock,
            outcome.actor,
            edit.line.clone(),
            Some(group_id.into()),
            None,
        ));
        authority.validate_semantic_edit(&edit).map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?;
        if store_post_revision(store, &authority, outcome.edit_digest)? != outcome.post_revision || outcome.post_generation != outcome.base_generation.checked_add(1).ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)? {
            return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
        }
        let mut admitted = |_| true;
        let mut control = semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES, &mut admitted);
        let identities = [authority.actor(), edit.id.as_str(), edit.id.as_str()].map(|identity| semio_framework_value::paged::PagedUtf8::<{usize::MAX}>::try_from_str_controlled(identity, &mut control));
        let [actor, applied_edit_id, tail_edit_id] = identities;
        let identities = [actor, applied_edit_id, tail_edit_id].map(|identity| identity.map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string())));
        let [actor, applied_edit_id, tail_edit_id] = identities;
        let prepared = authority.seal_prepared_owned(Box::new(edit), post_snapshot, outcome.edit_digest, [actor?, applied_edit_id?, tail_edit_id?]);
        Ok(Self {
            prepared,
            mutation_retirement: (*store.mutation_retirement_factory).clone().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
            snapshot_retirement: (*store.snapshot_retirement_factory).clone().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
            unbound_sha256: member.unbound_outcome_sha256.clone(),
            recovery_schema: member.recovery_schema.clone(),
            recovery_pack: member.recovery_pack.clone(),
            recovery_pack_sha256: member.recovery_pack_sha256.clone(),
            expected_generation: member.expected_generation,
            expected_revision: member.expected_revision,
            post_generation: member.post_generation,
            post_revision: member.post_revision,
        })
    }
}

pub fn recovery_frontier<P, Mutation>(store: &StorePartsRef<'_, P, Mutation>, member: &DurableOwnedGroupMemberV1, group_id: &str) -> Result<DurableMemberRecoveryFrontier, DurableOwnedGroupDecisionError>
where
    P: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
    Mutation: StoreMutation<P> + ValueToValue + ValueFromValue,
{
    if (store.cursor.generation, store.cursor.content_revision) == (member.expected_generation, member.expected_revision) {
        return Ok(DurableMemberRecoveryFrontier::Base);
    }
    if (store.cursor.generation, store.cursor.content_revision) != (member.post_generation, member.post_revision) {
        return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
    }
    let outcome = DurableBoundOneItemOutcomeV1::decode_canonical_pack(&member.recovery_pack)?;
    let edit: Edit<Mutation> = parse_canonical_json_value(&outcome.edit_canonical_json)?;
    let tail = store.log.applied_edit_ids.last().and_then(|id| store.log.envelope.vcs.edits.iter().find(|candidate| candidate.id == *id)).ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?;
    let accumulator = store.cursor.revision_accumulator.applied.last().ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?;
    if outcome.group_id != group_id
        || !valid_edit_identities(&edit)
        || outcome.unbound_pack::<Mutation>().map(|pack| semio_framework_hash::sha256_hex(&pack))? != member.unbound_outcome_sha256
        || CursorRevisionAccumulator::edit_digest(&edit) != outcome.edit_digest
        || accumulator.edit_digest != outcome.edit_digest
        || semio_framework_pack_json::to_json_string(tail).as_bytes() != outcome.edit_canonical_json
        || store.live.current.encode_pack() != outcome.post_snapshot_pack
    {
        return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
    }
    Ok(DurableMemberRecoveryFrontier::Post)
}

pub fn store_member_identity<P, Mutation>(store: &StorePartsRef<'_, P, Mutation>) -> Result<(semio_framework_artifact_reference::ArtifactRef, Option<OwnerRef>), DurableOwnedGroupDecisionError>
where
    P: ValueToValue,
    Mutation: ValueToValue + ValueFromValue + StoreMutation<P>,
{
    let envelope = &store.log.envelope;
    let dialect = envelope.dialect.clone().ok_or(DurableOwnedGroupDecisionError::InvalidIdentity)?;
    Ok((semio_framework_artifact_reference::ArtifactRef { artifact_id: envelope.id.clone(), dialect }, envelope.owner.clone()))
}

pub fn prepared_member<P, Mutation>(role: &str, reference: semio_framework_artifact_reference::ArtifactRef, owner: Option<OwnerRef>, prepared: &DurableStorePreparedOwnerV1<P, Mutation>,native:&mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DurableOwnedGroupMemberV1, DurableOwnedGroupDecisionError> {
    let original = prepared.prepared.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    let unbound = prepared.unbound.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    let authority = original.authority();
    authority.validate_prepared(original).map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?;
    if authority.group_id().is_some() {
        return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
    }
    Ok(DurableOwnedGroupMemberV1 {
        role: native.copy_text(role).map_err(DurableOwnedGroupDecisionError::Retirement)?,
        reference,
        owner,
        expected_generation: authority.generation().0,
        expected_revision: authority.base_revision(),
        recovery_schema: native.copy_text(&unbound.recovery_schema).map_err(DurableOwnedGroupDecisionError::Retirement)?,
        recovery_pack: Vec::new(),
        recovery_pack_sha256: String::new(),
        unbound_outcome_sha256: native.copy_text(&unbound.sha256).map_err(DurableOwnedGroupDecisionError::Retirement)?,
        post_generation: authority.generation().0.checked_add(1).ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?,
        post_revision: [0; 32],
    })
}

fn identity_text_bytes(id:&str,dialect:&semio_framework_artifact_reference::ArtifactDialect)->usize{id.len()+dialect.artifact_kind.len()+dialect.standard.len()+dialect.subset.len()}
fn copy_identity(id:&str,dialect:&semio_framework_artifact_reference::ArtifactDialect,native:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_artifact_reference::ArtifactRef,DurableOwnedGroupDecisionError>{let error=DurableOwnedGroupDecisionError::Retirement;Ok(semio_framework_artifact_reference::ArtifactRef{artifact_id:native.copy_text(id).map_err(error)?,dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:native.copy_text(&dialect.artifact_kind).map_err(error)?,standard:native.copy_text(&dialect.standard).map_err(error)?,subset:native.copy_text(&dialect.subset).map_err(error)?}})}
fn copy_owner(owner:Option<&OwnerRef>,native:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<Option<OwnerRef>,DurableOwnedGroupDecisionError>{owner.map(|owner|Ok(OwnerRef{parent:copy_identity(&owner.parent.artifact_id,&owner.parent.dialect,native)?,slot:native.copy_text(&owner.slot).map_err(DurableOwnedGroupDecisionError::Retirement)?,child_id:native.copy_text(&owner.child_id).map_err(DurableOwnedGroupDecisionError::Retirement)?})).transpose()}

pub fn bound_member<P, Mutation>(role: &str, reference: semio_framework_artifact_reference::ArtifactRef, owner: Option<OwnerRef>, bound: &DurableStoreBoundOutcomeV1<P, Mutation>) -> DurableOwnedGroupMemberV1 {
    DurableOwnedGroupMemberV1 {
        role: role.into(),
        reference,
        owner,
        expected_generation: bound.expected_generation,
        expected_revision: bound.expected_revision,
        recovery_schema: bound.recovery_schema.clone(),
        recovery_pack: bound.recovery_pack.clone(),
        recovery_pack_sha256: bound.recovery_pack_sha256.clone(),
        unbound_outcome_sha256: bound.unbound_sha256.clone(),
        post_generation: bound.post_generation,
        post_revision: bound.post_revision,
    }
}

pub fn prepared_base_matches_store<P, Mutation>(store: &StorePartsRef<'_, P, Mutation>, member: &DurableOwnedGroupMemberV1, prepared: &ArtifactStoreOneItemPrepared<P, Mutation>) -> bool
where
    P: ValueToValue,
    Mutation: ValueToValue + ValueFromValue + StoreMutation<P>,
{
    store.cursor.generation == member.expected_generation && store.cursor.content_revision == member.expected_revision && store.log.applied_edit_ids.len() == prepared.authority().base_applied_edit_count()
}
