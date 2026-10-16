//! 📥️ Items of `store` that belong to `os-store-engine` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🦀️.rs`.
use semio_framework_os_store_text_io::ArtifactStoreSnapshot as _;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_os_store_replay::replay_preparation;
use replay_preparation::ArtifactReplayPreparationProgress;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_os_spr::{ActorId, ArtifactId, HybridLogicalTimestamp, MutationId, SchemaId, UndoPolicy};
use semio_framework_os_spr::{Edit, Mutation, MutationMeta, OpBinary, OpText};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::marker::PhantomData;
use std::sync::Arc;
use semio_framework_os_store_read::{ArtifactDerivedHistoryPreview, ArtifactDerivedReplay, ArtifactDerivedSnapshot, ErasedSnapshotRead, SnapshotRead, artifact_retirement_box_close_step, retire_document_envelope};
use semio_framework_os_store_engine_owners::DocumentStoreOwners;
use semio_framework_os_store_read::{ARTIFACT_STORE_UNSCHEDULED_CLOSE_STEPS, ArtifactStoreCursorDisposerPhase, ArtifactStoreDisplacedRetirements, ArtifactStoreMutationDagRetirement, ArtifactStoreRevisionAccumulatorRetirement, ArtifactStoreStringRetirement, ArtifactStoreStringVectorRetirement, ArtifactStoreSupersessionRetirement, SnapshotReadLeaseRegistryOwner};
use semio_framework_os_store_read::history_read_admission::PreparedHistoryReadRetirement;
use semio_framework_os_store_retirement::resident_backing;
use semio_framework_os_spr::conflict::ProtocolConflictRetirement as ArtifactStoreConflictRetirement;
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
use semio_framework_value::factory::FactorySharedRetirement as ReturnedSnapshotReadRetirement;
use semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
use semio_framework_os_vcs::io::binary::entity_identity::{mint_alternative_id, mint_change_id};
use semio_framework_os_vcs::io::binary::checkpoint_identity::content_addressed_checkpoint_id;
use semio_framework_os_vcs::{Author, VcsError};
use semio_framework_os_vcs::{Alternative, ArtifactHistoryKey, ArtifactHistoryLedger, Change, Checkpoint};
use semio_framework_os_store_space_schema::{AcceptedArtifactProjection, ArtifactBackboneRef, ArtifactCommand, ArtifactCursor, ArtifactEnvelope, ArtifactProjectionCacheMode, ArtifactProjectionCause, ArtifactProjectionEvent, ArtifactProjectionInvalidation, ArtifactProjectionResult, ArtifactProjectionStamp, ArtifactRevision, HistoryLane, OpenToolTransaction, StaleArtifactProjection, SupersedeInput};
use semio_framework_os_store_replay::ReplayTurnBudget;
use semio_framework_os_store_replay::{PendingLocalStep, TailStep};
//#region 🔖️EphemeralLanes

use semio_framework_os_store_ephemeral::*;
use semio_framework_os_store_text_io::ArtifactPackFiles;
use pack::snapshot::ArtifactPack;
use semio_framework_os_store_text_io::ArtifactStoreSnapshot;
use semio_framework_os_store_engine_core::lifted_document_materialize::now_iso;
use semio_framework_os_store_initialization_runtime::lifted_document_materialize::now_ms;
use semio_framework_os_store_initialization_runtime::lifted_document_materialize::fresh_replica_clock;
use semio_framework_os_store_engine_live::lifted_document_materialize::stamp_primary_operation_identity;
use semio_framework_os_store_text_io::text_format::{ParsedDocumentText, parse_decoded_document_spr};
use semio_framework_os_store_text_io::text_format::{canonical_conflict_actors, conflict_messages_for_edits, mutation_ids, validate_durable_history, validate_history_lanes};
use semio_framework_os_store_command::parse_command;
use semio_framework_os_store_command::retire_command;
use semio_framework_os_store_ledger::{edit_from_operation_envelope, fold_envelope_history};
use semio_framework_os_store_ledger::{ArtifactStoreHistoryCommitReservation, ArtifactStorePendingReportRetirement, CursorRevisionAccumulator, CursorRevisionRecord, PendingCommandReport, StackMirrorDirt, retire_replayed_projection, retire_scratch_edits, retire_scratch_operations, retire_supersede_inputs, stamp_edit_semantics, validate_composition_pins};
use semio_framework_os_store_one_item_batch::lifted_one_item_publication::{ArtifactStoreBatchAdmissionRejected, ArtifactStoreBatchPublication};
use semio_framework_os_store_one_item_publication::{ArtifactStoreOneItemAdvance, ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPreparation, ArtifactStoreOneItemPreparationFactory, ArtifactStoreOneItemPublicationPhase};
use semio_framework_os_store_one_item_publication::{ArtifactStoreBatchItemAuthority, ArtifactStoreBatchItemRequest, ArtifactStoreBatchSourceOf};
use semio_framework_os_store_one_item_batch::lifted_one_item_publication::ArtifactStoreBatchStage;
use semio_framework_os_store_operation_wire::{ArtifactOperationWire, ArtifactPreparedOperationCursor, ArtifactPreparedOperationProgress, ArtifactPreparedOperationError};
use semio_framework_os_store_one_item::{ARTIFACT_STORE_ONE_ITEM_ID_BYTES, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, ArtifactStoreOneItemFootprint, ArtifactStoreOneItemGrant, ArtifactStoreOneItemPreparationStep, validate_original_preparation_step};
use semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactStorePublication;
use semio_framework_os_store_replay::{EditReplay, EditReplayResult, EffectiveSupersessions, PairMerge, ReplayEdits, ReplayMode, ReplayProgress, ReplayStep};
use semio_framework_os_store_replay::{PREFIX_SNAPSHOT_RING_CAPACITY, PrefixSnapshot, REPROJECTION_OWNER_SLOTS, fold_effective_edit, retire_shared_projection, same_effective_inputs};
//#region 🔖️Backbone
//#region 🔖️Backbone
use semio_framework_os_store_backbone as backbone;
use backbone::{Backbone, BackboneMessage, Backbones};
#[cfg(target_arch = "wasm32")]
use backbone::resolve_backbone;
use semio_framework_os_store_backbone::backbone_retirement::ArtifactStoreBackboneRetirement;
use semio_framework_os_store_engine_owners::lifted_store::ArtifactStoreCloseHost;
use semio_framework_os_store_engine_owners::lifted_store::ArtifactStoreCloseView;
use semio_framework_os_store_engine_core::lifted_store::CommandReceipt;
use semio_framework_os_store_engine_core::lifted_store::ArtifactEnvelopeView;
use semio_framework_os_store_engine_core::lifted_store::HistoryColumn;
use semio_framework_os_store_initialization_runtime::lifted_store::artifact_initial_digest_of_pack;
use semio_framework_os_store_initialization_runtime::lifted_store::ArtifactStoreInitializationOwnerCatalog;
use semio_framework_os_store_initialization_runtime::lifted_store::ArtifactStoreInitializationRuntime;
use semio_framework_os_store_engine_core::lifted_store::StoreLog;
use semio_framework_os_store_engine_core::lifted_store::StoreOwners;
use semio_framework_os_store_engine_core::lifted_store::StoreWire;
use semio_framework_os_store_engine_core::lifted_store::StoreCursor;
use semio_framework_os_store_engine_live::lifted_store::StoreLive;
use semio_framework_os_store_engine_live::lifted_store::StoreReplay;
use semio_framework_os_store_engine_live::lifted_store::StoreParts;
use semio_framework_os_store_engine_live::lifted_store::StorePartsRef;
use semio_framework_os_store_engine_live::lifted_store::ArtifactStoreDocumentRootCommitAuthority;
use semio_framework_os_store_engine_live::lifted_store::AppliedMutation;
use semio_framework_os_store_engine_live::lifted_store::transition_retraction_closure;
use semio_framework_os_store_engine_core::lifted_store::fold_history;
//#region 🔖️Authority
/// 📝 Draft-lane store alias — same algebra as ArtifactStore; PruneDrafts never enters a Change.
pub type DraftStore<P, Mutation> = ArtifactStore<P, Mutation>;
//#endregion 🔖️Authority

//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️CodecRegistry

/// 🌳️ Walks `checkpoint_id`'s ancestor chain via `parent_id` back to the root, nearest-first
/// (`checkpoint_id` itself is the first entry). Cycle-guarded (a malformed/adversarial parent chain
/// stops instead of looping forever) — every well-formed chain built by `reconcile_alternative`/
/// `CommitCheckpoint` is already acyclic, this is defense in depth, not a documented invariant break.
pub async fn checkpoint_ancestors<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, checkpoint_id: &str) -> Vec<String> {
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

/// 🌳️ The merge-base of checkpoints `a` and `b`: the nearest checkpoint common to both
/// ancestor chains (via `parent_id`), or `None` if their histories share no common ancestor.
/// Supports branch-merge tooling that needs to know "everything since the fork point" on either
/// side. `b`'s chain is walked nearest-to-farthest so the FIRST hit in `a`'s ancestor set is the
/// nearest (not merely *a*) common ancestor.
pub async fn merge_base<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, a: &str, b: &str) -> Option<String> {
    let ancestors_a: HashSet<String> = checkpoint_ancestors(envelope, a).await.into_iter().collect();
    checkpoint_ancestors(envelope, b).await.into_iter().find(|id| ancestors_a.contains(id))
}

/// 🌀️ `alternative_id` is both cloned and returned below — a future can only be awaited once
/// (R10 shape 2), so it is resolved to a plain `String` here.
///
/// 🌀️ Same reasoning as `alternative_id` above — resolved once, before it is borrowed AND moved.
///
/// 🎯️ `&[]`: reconcile-alternative checkpoints carry no composition pins yet — the
/// `CompositionCoordinator` that populates real `CompositionPin`s on commit is a later wave.
pub async fn reconcile_alternative<P, Mutation>(envelope: &mut ArtifactEnvelope<P, Mutation>, alternative_name: &str, checkpoint_message: Option<String>, authors: Vec<Author>, identity: &mut EntityIdentityAuthority<'_>) -> Result<String, VcsError>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue,
{
    if envelope.vcs.checkpoints.is_empty() {
        return Err(VcsError::NoCheckpoint);
    }
    let checkpoint_id = envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone()).ok_or(VcsError::NoCheckpoint)?;
    let alternative_id = identity.encode(|control|mint_alternative_id(alternative_name, std::slice::from_ref(&checkpoint_id), control))?;
    envelope.vcs.alternatives.try_push(Alternative { id: alternative_id.clone(), name: alternative_name.to_string(), checkpoint_ids: vec![checkpoint_id] }).map_err(|_| VcsError::ValidationFailed("alternative history ledger is saturated".into()))?;
    if let Some(message) = checkpoint_message {
        let change = Change { id: identity.encode(|control|mint_change_id(&[], Some(&message), control))?, edit_ids: Vec::new(), description: Some(message), saved_at: now_iso() };
        let parent = envelope.vcs.checkpoints.last();
        let parent_id = parent.map(|checkpoint| checkpoint.id.clone());
        let mut change_ids = parent.map(|checkpoint| checkpoint.change_ids.clone()).unwrap_or_default();
        change_ids.push(change.id.clone());
        envelope.vcs.changes.try_push(change).map_err(|_| VcsError::ValidationFailed("change history ledger is saturated".into()))?;
        let timestamp = now_iso();
        let checkpoint_message = Some("reconciled".to_string());
        let id = content_addressed_checkpoint_id(parent_id.as_deref(), &change_ids, &envelope.vcs.changes, checkpoint_message.as_deref(), &authors, &timestamp, &[]).await;
        envelope
            .vcs
            .checkpoints
            .try_push(Checkpoint { id, change_ids, parent_id, authors, message: checkpoint_message, timestamp, composition_pins: Vec::new() })
            .map_err(|_| VcsError::ValidationFailed("checkpoint history ledger is saturated".into()))?;
    }
    Ok(alternative_id)
}
//#endregion 🔖️CodecRegistry
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
//#region 🔖️Config
pub type ConfigStore<C, ConfigMutation> = ArtifactStore<C, ConfigMutation>;
//#endregion 🔖️Config
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
/// 🗃️ The composer of one store: the six components of [`StoreLog`], [`StoreOwners`], [`StoreWire`], [`StoreCursor`], [`StoreLive`] and [`StoreReplay`], the use cases that need several of them and the delegates of the public surface.
pub struct ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    pub log: StoreLog<P, Mutation>,
    pub owners: StoreOwners<P, Mutation>,
    pub wire: StoreWire<P, Mutation>,
    pub cursor: StoreCursor<P, Mutation>,
    pub live: StoreLive<P, Mutation>,
    pub replay: StoreReplay<P, Mutation>,
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
impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    /// 🧩️ Splits the composer into its component borrows.
    #[inline]
    pub fn parts(&mut self) -> StoreParts<'_, P, Mutation> {
        StoreParts { log: &mut self.log, owners: &mut self.owners, wire: &mut self.wire, cursor: &mut self.cursor, live: &mut self.live, replay: &mut self.replay }
    }

    /// 🧩️ Shares the composer's components.
    #[inline]
    pub fn parts_ref(&self) -> StorePartsRef<'_, P, Mutation> {
        StorePartsRef { log: &self.log, owners: &self.owners, wire: &self.wire, cursor: &self.cursor, live: &self.live, replay: &self.replay }
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
/// 🎚️ Atomically adopts a retained config runtime after typed Pack and history validation.
pub fn config_store_from_initialized_runtime_with_owners<P, Mutation>(mut envelope: ArtifactEnvelope<P, Mutation>, runtime: ArtifactStoreInitializationRuntime<P>, generation: u64, mut owners: DocumentStoreOwners<P, Mutation>) -> ConfigStore<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    let (current, applied_edit_ids, redo_edit_ids, cursor, local_actor_id, dag, edit_sequence, clock, initial_digest, revision_accumulator, supersessions) = runtime.into_parts();
    let current_checkpoint_id = cursor.checkpoint_id.clone();
    envelope.cursor = Some(cursor);
    let content_revision = revision_accumulator.revision(current_checkpoint_id.as_deref());
    let factory_retirement_tickets = owners.take_factory_retirement_tickets();
    ArtifactStore {
        log: StoreLog {
            envelope: std::mem::ManuallyDrop::new(envelope),
            envelope_detached: false,
            dag: std::mem::ManuallyDrop::new(dag),
            applied_edit_ids: std::mem::ManuallyDrop::new(applied_edit_ids),
            redo_edit_ids: std::mem::ManuallyDrop::new(redo_edit_ids),
            edit_sequence,
            current_checkpoint_id: std::mem::ManuallyDrop::new(current_checkpoint_id),
            local_actor_id: std::mem::ManuallyDrop::new(local_actor_id),
            authoring_verb: None,
            merge_policy: semio_framework_os_spr::MergePolicy::default(),
            clock,
            initial_digest,
            operation_wire: None,
        },
        owners: StoreOwners {
            snapshot_retirement_factory: std::mem::ManuallyDrop::new(owners.snapshot_retirement.take()),
            initial_snapshot_retirement_factory: std::mem::ManuallyDrop::new(owners.initial_snapshot_retirement.take()),
            mutation_retirement_factory: std::mem::ManuallyDrop::new(owners.mutation_retirement.take()),
            snapshot_read_leases: std::mem::ManuallyDrop::new(SnapshotReadLeaseRegistryOwner::new()),
            displaced_retirements: std::mem::ManuallyDrop::new(ArtifactStoreDisplacedRetirements::new()),
            owned_disposer: std::mem::ManuallyDrop::new(owners.store_disposer.take()),
            owned_disposer_terminal: false,
            one_item_preparation_factory: std::mem::ManuallyDrop::new(owners.one_item_preparation.take()),
            replay_preparation_factory: std::mem::ManuallyDrop::new(owners.replay_preparation.take()),
            one_item_wire_preparation_factory: std::mem::ManuallyDrop::new(owners.one_item_wire_preparation.take()),
            factory_retirement_tickets: std::mem::ManuallyDrop::new(factory_retirement_tickets),
        },
        wire: StoreWire {
            backbone: std::mem::ManuallyDrop::new(None),
            member_inbox: VecDeque::new(),
            durable_group_root: std::mem::ManuallyDrop::new(None),
        },
        cursor: StoreCursor {
            generation,
            last_projection_cause: None,
            content_revision,
            revision_accumulator: std::mem::ManuallyDrop::new(revision_accumulator),
            revision_dirty_from: None,
            mirror_dirt: StackMirrorDirt::CLEAN,
            marker: std::marker::PhantomData,
        },
        live: StoreLive {
            current: std::mem::ManuallyDrop::new(current),
            current_detached: false,
            tail_undo_cache: std::mem::ManuallyDrop::new(None),
            supersessions: std::mem::ManuallyDrop::new(supersessions),
            prefix_ring: std::mem::ManuallyDrop::new(Vec::with_capacity(PREFIX_SNAPSHOT_RING_CAPACITY)),
            prefix_ring_dirty_from: usize::MAX,
            fold_frontier: None,
            tail_step_generation: None,
            snapshot_equality: None,
            marker: std::marker::PhantomData,
        },
        replay: StoreReplay {
            replay_budget: None,
            local_replay_budget: None,
            pending_reprojection: None,
            pending_report: std::mem::ManuallyDrop::new(PendingCommandReport::default()),
        },
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
pub async fn checkpoint_identity(checkpoint: &Checkpoint, changes: &ArtifactHistoryLedger<Change>) -> String {
    content_addressed_checkpoint_id(checkpoint.parent_id.as_deref(), &checkpoint.change_ids, changes, checkpoint.message.as_deref(), &checkpoint.authors, &checkpoint.timestamp, &checkpoint.composition_pins).await
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
impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    pub fn preflight_runtime_seed(identities: &[MutationId]) -> Result<(), VcsError> {
        let mut unique = HashSet::new();
        for identity in identities {
            if identity.0.len() > semio_framework_os_spr::causal::MUTATION_DAG_IDENTIFIER_BYTES {
                return Err(VcsError::ValidationFailed("runtime causal seed identity exceeds its exact byte capacity".into()));
            }
            unique.insert(identity.0.as_str());
            if unique.len() > semio_framework_os_spr::causal::MUTATION_DAG_CAPACITY {
                return Err(VcsError::ValidationFailed("runtime causal seed exceeds its exact identity capacity".into()));
            }
        }
        Ok(())
    }

    pub fn adopt_runtime_seed(identities: Vec<MutationId>) -> (semio_framework_os_spr::MutationDag, Vec<String>) {
        let mut dag = semio_framework_os_spr::MutationDag::new();
        let mut rejected = Vec::new();
        for identity in identities {
            match dag.seed_applied(identity) {
                Ok(()) => {}
                Err(owner) if owner.error == semio_framework_os_spr::MutationDagError::Duplicate => rejected.push(owner.mutation_id.0),
                Err(_) => unreachable!("prevalidated causal seed admission cannot fail after owner adoption"),
            }
        }
        (dag, rejected)
    }

    fn seed_runtime_state(envelope: &ArtifactEnvelope<P, Mutation>, replica: u64) -> Result<(semio_framework_os_spr::MutationDag, i32, HybridLogicalTimestamp, Vec<String>), VcsError> {
        let mut identities = Vec::new();
        let mut edit_sequence = 0;
        let mut clock = HybridLogicalTimestamp::new(replica, now_ms());
        for edit in &envelope.vcs.edits {
            edit_sequence = edit_sequence.max(edit.sequence_number);
            identities.push(MutationId(edit.id.clone()));
            identities.extend(semio_framework_os_spr::mutation_ids_for_edit(edit));
            for meta in &edit.mutation_meta {
                clock.merge(&meta.timestamp);
            }
        }
        for transition in &envelope.transitions {
            identities.push(transition.mutation_id.clone());
            clock.merge(&transition.timestamp);
        }
        Self::preflight_runtime_seed(&identities)?;
        let (dag, rejected) = Self::adopt_runtime_seed(identities);
        Ok((dag, edit_sequence, clock, rejected))
    }

    /// 🚫️ A store is always constructed with no backbone attached — the envelope's
    /// `backbone` field is a descriptor of the last attachment, never an instruction to
    /// reconnect. Callers attach explicitly via {@link attach_backbone}/{@link attach_backbone_uri}.
    ///
    /// 🎯️ `applied_edit_ids`/`redo_edit_ids`/`current_checkpoint_id`/the active alternative
    /// and `current` are derived by folding the envelope's event log ([`fold_envelope_history`]) —
    /// the persisted history carries events only, so a save/load cycle restores the exact position
    /// by replaying them. The caller supplies the opened actor explicitly; history authors never
    /// determine the identity of the instance loading that history.
    ///
    /// 🛡️ A REJECTED construction retires the candidate envelope it consumed. The envelope is
    /// a terminal shell whose `Drop` asserts its owners were detached ([`ArtifactEnvelope::into_owners`]),
    /// so letting a `?` drop it turned every malformed-document refusal — a dangling cursor edit id, a
    /// duplicated authoritative edit, an alternative pinned to a checkpoint that was never recorded —
    /// into a process abort instead of the `VcsError` the caller is meant to read. The candidate lives
    /// in a slot {@link construct} empties only on success; whatever is left is detached here.
    pub async fn new<C:ArtifactStorePublication<P>>(envelope: ArtifactEnvelope<P, Mutation>, actor: ActorId) -> Result<Self, VcsError>
    {
        let mut candidate = Some(envelope);
        let constructed = Self::construct::<C>(&mut candidate, actor).await;
        if let Some(rejected) = candidate {
            rejected.retire_unadopted();
        }
        constructed
    }

    pub async fn construct<C:ArtifactStorePublication<P>>(candidate: &mut Option<ArtifactEnvelope<P, Mutation>>, actor: ActorId) -> Result<Self, VcsError>
    {
        let envelope = candidate.as_mut().expect("artifact store construction holds its candidate envelope");
        if actor.0.trim().is_empty() { return Err(VcsError::ValidationFailed("opened actor identity cannot be empty".into())); }
        validate_durable_history(envelope).await?;
        let semio_framework_os_spr::HistoryFold { applied: loaded_applied_edit_ids, redo: loaded_redo_edit_ids, checkpoint: current_checkpoint_id, alternative, supersessions, .. } = fold_envelope_history(envelope)?;
        envelope.active_alternative_id = alternative;
        validate_history_lanes(envelope, &loaded_applied_edit_ids, &loaded_redo_edit_ids).await?;
        let current = if loaded_applied_edit_ids.is_empty() { envelope.vcs.genesis.facts().share_snapshot() } else { Arc::new(Self::fold_history(envelope, &loaded_applied_edit_ids, &supersessions).await?) };
        let initial_digest = envelope.vcs.genesis.facts().digest();
        let catalog = ArtifactStoreInitializationOwnerCatalog::try_new().map_err(|reason| VcsError::ValidationFailed(reason.into()))?;
        let ArtifactStoreInitializationOwnerCatalog { mut applied_edit_ids, mut redo_edit_ids, mut cursor_applied_edit_ids, mut cursor_redo_edit_ids, applied_revision, redo_revision } = catalog;
        cursor_applied_edit_ids.extend(loaded_applied_edit_ids.iter().cloned());
        cursor_redo_edit_ids.extend(loaded_redo_edit_ids.iter().cloned());
        applied_edit_ids.extend(loaded_applied_edit_ids);
        redo_edit_ids.extend(loaded_redo_edit_ids);
        envelope.cursor = Some(ArtifactCursor::new(cursor_applied_edit_ids, cursor_redo_edit_ids, current_checkpoint_id.clone()));
        let identity_digest = CursorRevisionAccumulator::hash_record(b"initial", &[envelope.id.as_bytes(), envelope.schema.as_bytes(), &initial_digest]);
        let mut revision_accumulator = CursorRevisionAccumulator { identity_digest, applied: applied_revision, redo: redo_revision, applied_tail_chains: None, mutation_positions: protocol::HistoryFoldIndex::new(), indexed_edits: protocol::HistoryFoldIndex::new(), unit_flags: protocol::HistoryFoldIndex::new() };
        revision_accumulator.reconcile::<P, Mutation>(&applied_edit_ids, &redo_edit_ids, &envelope.vcs.edits, &supersessions, StackMirrorDirt::ALL, None);
        if !revision_accumulator.unit_flags.is_empty() {
            let units = EditReplay::new(ReplayMode::Report, envelope.vcs.genesis.facts().share_snapshot(), applied_edit_ids.to_vec(), 0, &envelope.schema, supersessions.clone(), &envelope.vcs.edits)?.run(&envelope.vcs.edits)?;
            revision_accumulator.unit_flags.extend(units.unit_flags.iter().map(|(key, flag)| (*key, *flag)));
            units.retire_unscheduled().map_err(|error| VcsError::ValidationFailed(error.into_message()))?;
        }
        let content_revision = revision_accumulator.revision(current_checkpoint_id.as_deref());
        let local_actor_id = actor;
        let (dag, edit_sequence, clock, rejected_seed_identities) = Self::seed_runtime_state(envelope, fresh_replica_clock().actor)?;
        C::publish_native_snapshot().map_err(|error| VcsError::ValidationFailed(error.to_string()))?;
        let mut displaced_retirements = ArtifactStoreDisplacedRetirements::new();
        if !rejected_seed_identities.is_empty() {
            displaced_retirements.push_reserved(Box::new(ArtifactStoreStringVectorRetirement::new(rejected_seed_identities)));
        }
        let envelope = candidate.take().expect("an accepted construction takes its validated envelope exactly once");
        Ok(Self {
            log: StoreLog {
                envelope: std::mem::ManuallyDrop::new(envelope),
                envelope_detached: false,
                dag: std::mem::ManuallyDrop::new(dag),
                applied_edit_ids: std::mem::ManuallyDrop::new(applied_edit_ids),
                redo_edit_ids: std::mem::ManuallyDrop::new(redo_edit_ids),
                edit_sequence,
                current_checkpoint_id: std::mem::ManuallyDrop::new(current_checkpoint_id),
                local_actor_id: std::mem::ManuallyDrop::new(local_actor_id),
                authoring_verb: None,
                merge_policy: semio_framework_os_spr::MergePolicy::default(),
                clock,
                initial_digest,
                operation_wire: None,
            },
            owners: StoreOwners {
                snapshot_retirement_factory: std::mem::ManuallyDrop::new(None),
                initial_snapshot_retirement_factory: std::mem::ManuallyDrop::new(None),
                mutation_retirement_factory: std::mem::ManuallyDrop::new(None),
                snapshot_read_leases: std::mem::ManuallyDrop::new(SnapshotReadLeaseRegistryOwner::new()),
                displaced_retirements: std::mem::ManuallyDrop::new(displaced_retirements),
                owned_disposer: std::mem::ManuallyDrop::new(None),
                owned_disposer_terminal: false,
                one_item_preparation_factory: std::mem::ManuallyDrop::new(None),
                replay_preparation_factory: std::mem::ManuallyDrop::new(None),
                one_item_wire_preparation_factory: std::mem::ManuallyDrop::new(None),
                factory_retirement_tickets: std::mem::ManuallyDrop::new(semio_framework_value::FactoryChildTickets { slots: std::array::from_fn(|_| semio_framework_value::FactoryChildSlot::empty()), prepared: 0 }),
            },
            wire: StoreWire {
                backbone: std::mem::ManuallyDrop::new(None),
                member_inbox: VecDeque::new(),
                durable_group_root: std::mem::ManuallyDrop::new(None),
            },
            cursor: StoreCursor {
                generation: 0,
                last_projection_cause: None,
                content_revision,
                revision_accumulator: std::mem::ManuallyDrop::new(revision_accumulator),
                revision_dirty_from: None,
                mirror_dirt: StackMirrorDirt::CLEAN,
                marker: std::marker::PhantomData,
            },
            live: StoreLive {
                current: std::mem::ManuallyDrop::new(current),
                current_detached: false,
                tail_undo_cache: std::mem::ManuallyDrop::new(None),
                supersessions: std::mem::ManuallyDrop::new(supersessions),
                prefix_ring: std::mem::ManuallyDrop::new(Vec::with_capacity(PREFIX_SNAPSHOT_RING_CAPACITY)),
                prefix_ring_dirty_from: usize::MAX,
                fold_frontier: None,
                tail_step_generation: None,
                snapshot_equality: None,
                marker: std::marker::PhantomData,
            },
            replay: StoreReplay {
                replay_budget: None,
                local_replay_budget: None,
                pending_reprojection: None,
                pending_report: std::mem::ManuallyDrop::new(PendingCommandReport::default()),
            },
        })
    }

    /// 🏗️ Atomically adopts a domain-validated initialization runtime. Every
    /// history/reference/snapshot owner was prepared under the caller's StepContext before this
    /// non-suspending move; no validation or collection traversal occurs at publication.
    pub fn from_initialized_runtime_with_owners(envelope: ArtifactEnvelope<P, Mutation>, runtime: ArtifactStoreInitializationRuntime<P>, generation: u64, owners: DocumentStoreOwners<P, Mutation>) -> Self {
        config_store_from_initialized_runtime_with_owners(envelope, runtime, generation, owners)
    }

    /// ♻️ Sole public reload API: adopts `envelope`'s event log and derives every position from
    /// it — a reload replays history, it never injects a cursor.
    pub async fn reset(&mut self, envelope: ArtifactEnvelope<P, Mutation>) -> Result<CommandReceipt, VcsError> {
        self.ensure_durable_group_idle()?;
        self.set_state(envelope).await?;
        self.cursor.last_projection_cause = Some(ArtifactProjectionCause::Reset);
        Ok(CommandReceipt { edit_ids: self.log.applied_edit_ids.to_vec(), generation: self.generation(), messages: Vec::new(), worst: None })
    }

    /// 💾️ Adopts full store state from `envelope`'s event log — applied edits, redo stack,
    /// checkpoint and alternative are the fold of its edits and transitions, so `Redo` survives
    /// round-tripping through a serialized envelope (e.g. one `dispatch` call per request). A history change still waiting
    /// for its replay belonged to the replaced log and is dropped with it.
    pub(crate) async fn set_state(&mut self, envelope: ArtifactEnvelope<P, Mutation>) -> Result<(), VcsError> {
        let mut candidate = Some(envelope);
        let adopted = self.adopt_state(&mut candidate).await;
        if let Some(rejected) = candidate {
            rejected.retire_unadopted();
        }
        adopted
    }

    /// 🛡️ The reload proper. Like {@link new}, a REFUSED reload leaves its candidate in the
    /// slot so {@link set_state} can retire it: the live store must survive a rejected reset, and a
    /// dropped populated envelope aborts on the terminal-shell witness instead of returning the
    /// `VcsError` the caller reads.
    async fn adopt_state(&mut self, candidate: &mut Option<ArtifactEnvelope<P, Mutation>>) -> Result<(), VcsError> {
        self.ensure_durable_group_idle()?;
        let envelope = candidate.as_mut().expect("a reload holds its candidate envelope");
        validate_durable_history(envelope).await?;
        let semio_framework_os_spr::HistoryFold { applied: applied_edit_ids, redo: redo_edit_ids, checkpoint: current_checkpoint_id, alternative, supersessions, .. } = fold_envelope_history(envelope)?;
        envelope.active_alternative_id = alternative;
        validate_history_lanes(envelope, &applied_edit_ids, &redo_edit_ids).await?;
        let current = Self::fold_history(envelope, &applied_edit_ids, &supersessions).await?;
        let applied_edit_ids = ArtifactStoreInitializationOwnerCatalog::retain_id_capacity(applied_edit_ids);
        let redo_edit_ids = ArtifactStoreInitializationOwnerCatalog::retain_id_capacity(redo_edit_ids);
        let initial_digest = envelope.vcs.genesis.facts().digest();
        let revision_accumulator = CursorRevisionAccumulator::new(envelope, initial_digest);
        let runtime_slots = usize::from(self.wire.backbone.is_some())
            + StoreLive::<P, Mutation>::string_owner_slots(&self.log.current_checkpoint_id)
            + StoreLive::<P, Mutation>::string_vector_owner_slots(&self.log.applied_edit_ids)
            + StoreLive::<P, Mutation>::string_vector_owner_slots(&self.log.redo_edit_ids)
            + StoreCursor::<P, Mutation>::revision_owner_slots(&self.cursor.revision_accumulator)
            + 1;
        let commit_authority = self.prepare_document_root_commit(runtime_slots)?;
        let (dag, edit_sequence, clock, rejected_seed_identities) = Self::seed_runtime_state(envelope, self.log.clock.actor)?;
        let envelope = candidate.take().expect("an adopted reload takes its validated envelope exactly once");
        self.clear_prefix_ring()?;
        self.commit_document_roots_retained(envelope, Arc::new(current), dag, None, commit_authority);
        self.replace_supersessions_retained(supersessions)?;
        if !rejected_seed_identities.is_empty() {
            self.owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreStringVectorRetirement::new(rejected_seed_identities)));
        }
        if let Some(previous) = self.wire.backbone.take() {
            self.owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreBackboneRetirement::new(previous)));
        }
        self.log.edit_sequence = edit_sequence;
        if let Some(owner) = StoreLive::<P, Mutation>::take_string_replacement(&mut self.log.current_checkpoint_id, current_checkpoint_id) {
            self.owners.displaced_retirements.push_reserved(owner);
        }
        self.log.clock = clock;
        self.log.initial_digest = initial_digest;
        if let Some(owner) = StoreLive::<P, Mutation>::take_string_vector_replacement(&mut self.log.applied_edit_ids, applied_edit_ids) {
            self.owners.displaced_retirements.push_reserved(owner);
        }
        if let Some(owner) = StoreLive::<P, Mutation>::take_string_vector_replacement(&mut self.log.redo_edit_ids, redo_edit_ids) {
            self.owners.displaced_retirements.push_reserved(owner);
        }
        self.push_revision_replacement_reserved(revision_accumulator);
        if let Some(pending) = self.replay.pending_reprojection.take() {
            self.retire_pending_replay(pending.replay)?;
        }
        self.cursor.mirror_dirt = StackMirrorDirt::ALL;
        self.live.fold_frontier = None;
        self.bump()?;
        Ok(())
    }

    /// 📌️ Records the composed children a checkpoint was taken against. Not reachable through
    /// an ordinary `Apply` (no mutation can touch checkpoint metadata), so — like `set_owner` — it
    /// needs its own setter. Called by `VcsArtifactApp`'s checkpoint cascade right after the
    /// checkpoint is created, since a pin can only name a child checkpoint that already exists.
    /// Re-derives `content_addressed_checkpoint_id` so the checkpoint's identity actually covers its
    /// pins rather than merely storing them beside it, and records the re-identification as a
    /// `Repin` transition so every replica and every reload derives the same identity.
    #[must_use]
    pub async fn set_checkpoint_composition_pins(&mut self, checkpoint_id: &str, pins: Vec<semio_framework_os_vcs::CompositionPin>) -> Result<ArtifactProjectionInvalidation, VcsError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

        self.ensure_durable_group_idle()?;
        validate_composition_pins(&pins).await?;
        let target_index = self.log.envelope.vcs.checkpoints.iter().position(|checkpoint| checkpoint.id == checkpoint_id).ok_or_else(|| VcsError::UnknownChange(checkpoint_id.to_string()))?;
        if self.log.envelope.vcs.checkpoints.iter().any(|checkpoint| checkpoint.parent_id.as_deref() == Some(checkpoint_id)) {
            return Err(VcsError::ValidationFailed(format!("cannot repin checkpoint {checkpoint_id} after a descendant exists")));
        }
        let checkpoint = &self.log.envelope.vcs.checkpoints[target_index];
        let previous_id = checkpoint.id.clone();
        let next_id = content_addressed_checkpoint_id(checkpoint.parent_id.as_deref(), &checkpoint.change_ids, &self.log.envelope.vcs.changes, checkpoint.message.as_deref(), &checkpoint.authors, &checkpoint.timestamp, &pins).await;
        if next_id != previous_id && self.log.envelope.vcs.checkpoints.iter().enumerate().any(|(index, checkpoint)| index != target_index && checkpoint.id == next_id) {
            return Err(VcsError::ValidationFailed(format!("rederived checkpoint identity {next_id} collides with established history")));
        }
        let dependencies = self.checkpoint_origin(&previous_id).into_iter().collect();
        let pins = pins.iter().map(|pin| semio_framework_os_spr::TransitionPin { child_uri: pin.child_ref.to_uri(), checkpoint_id: pin.checkpoint_id.clone() }).collect();
        self.replace_pending_report_retained(PendingCommandReport::default())?;
        self.commit_transition(semio_framework_os_spr::HistoryTransition::Repin { checkpoint_id: previous_id, pinned_checkpoint_id: next_id, pins }, dependencies).await?;
        self.flush_outbound().await?;
        self.invalidate_projections(ArtifactProjectionCause::Checkpoint)
    }

    /// 📏️ Publishes the installed disposer's exact retained payload or terminal allocation demand.
    pub fn close_owned_demands(&self, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        if self.owners.owned_disposer_terminal { return Ok(Default::default()); }
        let owner = self.owners.owned_disposer.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact store has no installed owner disposer"))?;
        if !owner.terminal_is_empty(self) {
            let mut demand = owner.demands(self, maximum_body_bytes)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "store disposer depth overflow"))?;
            return Ok(demand);
        }
        if !self.resident_backings_terminal_is_empty() { return Ok(RetirementDemand { release_bytes: self.next_resident_backing_byte_demand()?, depth: 2, ..Default::default() }); }
        if !self.owners.snapshot_read_leases.closed() { return Ok(RetirementDemand { release_bytes: self.owners.snapshot_read_leases.custody_byte_demand()?, depth: 2, ..Default::default() }); }
        if self.factory_aliases_present() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if !self.owners.factory_retirement_tickets.terminal_is_empty() {
            let mut demand = self.owners.factory_retirement_tickets.demands(maximum_body_bytes)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "store factory ticket depth overflow"))?;
            return Ok(demand);
        }
        Ok(RetirementDemand { release_bytes: size_of_val(owner.as_ref()), depth: 1, ..Default::default() })
    }

    /// 🧹️ Advances the exact store disposer under the caller's retirement grant.
    pub fn close_owned_store_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.owners.owned_disposer_terminal { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.close_owned_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "store close exceeds admitted owner depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let mut disposer = self.owners.owned_disposer.take().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact store has no owner-supplied bounded disposer"))?;
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        let result = (|| {
            if !disposer.terminal_is_empty(self) {
                let step = disposer.close_step(&mut ArtifactStoreCloseView { store: self }, child)?;
                semio_framework_value::retained_clone::admit_retained_clone_close(child, step, disposer.terminal_is_empty(self), "installed store disposer")?;
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            if !self.resident_backings_terminal_is_empty() { return self.close_resident_backing_step(child).map(|step| RetainedCloneStep::Progress(step.progress())); }
            if !self.owners.snapshot_read_leases.closed() { return self.owners.snapshot_read_leases.close_custody_step(child).map(|step| RetainedCloneStep::Progress(step.progress())); }
            if self.factory_aliases_present() {
                self.close_one_factory_alias();
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
            }
            if !self.owners.factory_retirement_tickets.terminal_is_empty() {
                let step = self.owners.factory_retirement_tickets.close_step(child)?;
                semio_framework_value::retained_clone::admit_retained_clone_close(child, step, self.owners.factory_retirement_tickets.terminal_is_empty(), "store factory tickets")?;
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            self.owners.owned_disposer_terminal = true;
            Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: size_of_val(disposer.as_ref()), ..empty }))
        })();
        if self.owners.owned_disposer_terminal { drop(disposer); } else { *self.owners.owned_disposer = Some(disposer); }
        result
    }

    /// 🪹️ Confirms both the store disposer and retained snapshot reads have closed.
    pub fn close_owned_store_terminal_is_empty(&self) -> bool {
        self.owners.owned_disposer_terminal && self.resident_backings_terminal_is_empty() && self.owners.snapshot_read_leases.closed() && !self.factory_aliases_present() && self.owners.factory_retirement_tickets.terminal_is_empty()
    }

    fn next_resident_backing_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.log.dag.backing_is_empty() { return Ok(self.log.dag.next_backing_release_byte_demand()); }
        for demand in [self.log.applied_edit_ids.next_empty_page_release_byte_demand(), self.log.redo_edit_ids.next_empty_page_release_byte_demand(), self.cursor.revision_accumulator.applied.next_empty_page_release_byte_demand(), self.cursor.revision_accumulator.redo.next_empty_page_release_byte_demand()] {
            if let Some(bytes) = demand.filter(|bytes| *bytes != 0) { return Ok(bytes); }
        }
        for bytes in [resident_backing::deque_demand(&self.owners.displaced_retirements.owners), resident_backing::vec_demand(&self.live.prefix_ring), resident_backing::deque_demand(&self.wire.member_inbox), resident_backing::vec_demand(&self.replay.pending_report.messages), resident_backing::vec_demand(&self.replay.pending_report.outbound), self.replay.pending_report.edit_ids.as_ref().map_or(0, resident_backing::vec_demand)] {
            if bytes != 0 { return Ok(bytes); }
        }
        Ok(self.owners.snapshot_read_leases.empty_backing_demands()?.release_bytes)
    }

    fn close_resident_backing_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.close_structural_owners_terminal_is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Store resident backing still holds structural payload owners")); }
        let empty = RetainedCloneProgress::default();
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "resident backing requires owner depth")); }
        if !self.log.dag.backing_is_empty() {
            let (copied_items, released_bytes, _) = self.log.dag.close_backing_step(1, grant.maximum_release_bytes);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items, released_bytes, ..empty }));
        }
        if self.log.applied_edit_ids.capacity() != 0 { return resident_backing::close_history(&mut self.log.applied_edit_ids, grant); }
        if self.log.redo_edit_ids.capacity() != 0 { return resident_backing::close_history(&mut self.log.redo_edit_ids, grant); }
        if self.cursor.revision_accumulator.applied.capacity() != 0 { return resident_backing::close_history(&mut self.cursor.revision_accumulator.applied, grant); }
        if self.cursor.revision_accumulator.redo.capacity() != 0 { return resident_backing::close_history(&mut self.cursor.revision_accumulator.redo, grant); }
        if resident_backing::deque_demand(&self.owners.displaced_retirements.owners) != 0 { return resident_backing::close_deque(&mut self.owners.displaced_retirements.owners, grant); }
        if self.live.prefix_ring.capacity() != 0 { return resident_backing::close_vec(&mut self.live.prefix_ring, grant); }
        if self.wire.member_inbox.capacity() != 0 { return resident_backing::close_deque(&mut self.wire.member_inbox, grant); }
        if self.replay.pending_report.messages.capacity() != 0 { return resident_backing::close_vec(&mut self.replay.pending_report.messages, grant); }
        if self.replay.pending_report.outbound.capacity() != 0 { return resident_backing::close_vec(&mut self.replay.pending_report.outbound, grant); }
        if let Some(ids) = self.replay.pending_report.edit_ids.as_mut().filter(|ids| ids.capacity() != 0) { return resident_backing::close_vec(ids, grant); }
        self.owners.snapshot_read_leases.close_empty_backing_step(grant)
    }

    /// 🧹️ Advances owned-store disposal without requiring this store to be a composed
    /// [`SpaceMember`]. Local lanes use the same exact disposer authority.
    pub fn close_owned_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.close_owned_store_step(grant)
    }

    /// 🧺️ Confirms owned-store disposal for document and local lane snapshots alike.
    pub fn close_owned_terminal_is_empty(&self) -> bool {
        self.close_owned_store_terminal_is_empty()
    }

    /// 🧹️ Drives owned-store disposal to its terminal witness from the store's own quoted demands for callers no scheduler grants.
    pub fn close_owned_unscheduled(&mut self) -> Result<(), ValueError> {
        for _ in 0..ARTIFACT_STORE_UNSCHEDULED_CLOSE_STEPS {
            if self.close_owned_terminal_is_empty() { return Ok(()); }
            let demand = self.close_owned_demands(ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES)?;
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
            if let RetainedCloneStep::Complete(_) = self.close_owned_step(grant)? { return if self.close_owned_terminal_is_empty() { Ok(()) } else { Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "store close completed without its terminal-empty witness")) }; }
        }
        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "store close did not reach its terminal-empty witness within its bounded steps"))
    }

    pub fn close_cursor_admit(&mut self, phase: ArtifactStoreCursorDisposerPhase, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> {
        let structural = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        match phase {
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(0) => {
                let original = std::mem::replace(&mut *self.log.applied_edit_ids, semio_framework_os_vcs::HistoryPageStack::empty());
                match admit_artifact_retirement(original, grant, ArtifactStoreStringVectorRetirement::new) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { *self.log.applied_edit_ids = original; Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(1) => {
                let original = std::mem::replace(&mut *self.log.redo_edit_ids, semio_framework_os_vcs::HistoryPageStack::empty());
                match admit_artifact_retirement(original, grant, ArtifactStoreStringVectorRetirement::new) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { *self.log.redo_edit_ids = original; Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(2) => {
                let original = std::mem::replace(&mut *self.cursor.revision_accumulator, CursorRevisionAccumulator { identity_digest: [0; 32], applied: semio_framework_os_vcs::HistoryPageStack::empty(), redo: semio_framework_os_vcs::HistoryPageStack::empty(), applied_tail_chains: None, mutation_positions: protocol::HistoryFoldIndex::new(), indexed_edits: protocol::HistoryFoldIndex::new(), unit_flags: protocol::HistoryFoldIndex::new() });
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
            ArtifactStoreCursorDisposerPhase::PrefixSnapshots => {
                if self.live.prefix_ring.is_empty() { return Ok((None, structural)); }
                let factory = self.owners.snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "prefix close requires installed factory"))?;
                let entry = self.live.prefix_ring.pop().expect("observed original prefix owner");
                if Arc::strong_count(&entry.snapshot) > 1 { drop(entry); return Ok((None, structural)); }
                match factory.retire(entry.snapshot, grant) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, snapshot)) => { self.live.prefix_ring.push(PrefixSnapshot { snapshot, ..entry }); Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::TailSnapshot => {
                if self.live.tail_undo_cache.as_ref().is_some_and(|(id, _)| id.capacity() != 0) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "tail snapshot still retains original edit-id backing")); }
                if self.live.tail_undo_cache.is_none() { return Ok((None, structural)); }
                let factory = self.owners.snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "tail close requires installed factory"))?;
                let (id, snapshot) = self.live.tail_undo_cache.take().expect("observed original tail owner");
                if Arc::strong_count(&snapshot) > 1 { drop((id, snapshot)); return Ok((None, structural)); }
                match factory.retire(snapshot, grant) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, snapshot)) => { *self.live.tail_undo_cache = Some((id, snapshot)); Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::CurrentSnapshot => {
                if self.live.current_detached { return Ok((None, structural)); }
                if self.live.tail_undo_cache.is_some() || !self.snapshot_read_leases_terminal_is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "current snapshot still holds tail or read owners")); }
                let genesis = std::ptr::eq(self.live.current.as_ref(), self.log.envelope.vcs.genesis.facts().snapshot());
                let result = if genesis {
                    let factory = self.owners.initial_snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "genesis alias requires exact initial factory"))?.clone();
                    let original = unsafe { std::mem::ManuallyDrop::take(&mut self.live.current) };
                    ReturnedSnapshotReadRetirement::admit(original, factory, grant).map_err(|(error, original, alias)| { drop(alias); (error, original) })
                } else {
                    let factory = self.owners.snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "current snapshot requires exact factory"))?;
                    let original = unsafe { std::mem::ManuallyDrop::take(&mut self.live.current) };
                    factory.retire(original, grant)
                };
                match result {
                    Ok((owner, receipt)) => { self.live.current_detached = true; Ok((Some(owner), receipt)) }
                    Err((error, original)) => { self.live.current = std::mem::ManuallyDrop::new(original); Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::Backbone => self.close_take_backbone_retirement(grant),
            ArtifactStoreCursorDisposerPhase::Causal => {
                let original = std::mem::replace(&mut *self.log.dag, semio_framework_os_spr::MutationDag::empty());
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
            && self.replay.pending_report.edit_ids.is_none()
            && self.replay.pending_report.messages.is_empty()
            && self.replay.pending_report.outbound.is_empty()
            && self.live.supersessions.is_empty()
            && self.live.prefix_ring.is_empty();
        envelope_shell_is_empty && runtime_shell_is_empty && self.wire.durable_group_root.is_none() && self.log.dag.terminal_is_empty() && self.owners.displaced_retirements.terminal_is_empty()
    }

    pub fn owned_roots_terminal_is_empty(&self) -> bool {
        self.live.current_detached
            && self.log.envelope_detached
            && self.wire.durable_group_root.is_none()
            && self.live.tail_undo_cache.is_none()
            && self.live.prefix_ring.is_empty()
            && self.snapshot_read_leases_terminal_is_empty()
            && self.close_structural_owners_terminal_is_empty()
    }

    pub async fn fold_history<I, S>(envelope: &ArtifactEnvelope<P, Mutation>, applied_edit_ids: I, supersessions: &EffectiveSupersessions) -> Result<P, VcsError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        fold_history(envelope, applied_edit_ids, supersessions).await
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
    pub fn edits_for_ids(ids: &[String], edits: &HashMap<String, Edit<Mutation>>) -> Result<Vec<Edit<Mutation>>, VcsError> {
        ids.iter().map(|id| edits.get(id).cloned().ok_or_else(|| VcsError::UnknownEdit(id.clone()))).collect()
    }

    /// 🌀️ The unawaited future borrows `command`, which is then moved into `dispatch_inner`
    /// below — awaited immediately instead of deferred to avoid a move-while-borrowed (E0505).
    ///
    /// Undo/redo shrink `applied_edit_ids`; Apply/AppendTransaction append past `before`; a merge that
    /// rebased mid-history overrides this tail-diff guess via `pending_report.edit_ids`.
    pub async fn dispatch(&mut self, command: ArtifactCommand<Mutation>, identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError>
    where
        P: Sync,
    {
        self.ensure_durable_group_idle()?;
        self.pump().await?;
        let projection_cause = command.projection_cause().await;
        let before = self.log.applied_edit_ids.len();
        self.replace_pending_report_retained(PendingCommandReport::default())?;
        self.dispatch_inner(command, identity).await?;
        self.cursor.last_projection_cause = projection_cause;
        self.flush_outbound().await?;
        let edit_ids = self.replay.pending_report.edit_ids.take().unwrap_or_else(|| if self.log.applied_edit_ids.len() >= before { self.log.applied_edit_ids.cloned_from(before) } else { Vec::new() });
        Ok(CommandReceipt { edit_ids, generation: self.generation(), messages: std::mem::take(&mut self.replay.pending_report.messages), worst: self.replay.pending_report.worst.take() })
    }

    pub async fn dispatch_apply_exact(&mut self, mutations: Vec<Mutation>, identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError>
    where
        P: Sync,
    {
        self.ensure_durable_group_idle()?;
        self.pump().await?;
        let before = self.log.applied_edit_ids.len();
        self.replace_pending_report_retained(PendingCommandReport::default())?;
        self.apply_command(mutations, HistoryLane::Document, None, identity).await?;
        self.cursor.last_projection_cause = Some(ArtifactProjectionCause::Apply);
        self.flush_outbound().await?;
        let edit_ids = self.replay.pending_report.edit_ids.take().unwrap_or_else(|| self.log.applied_edit_ids.cloned_from(before));
        Ok(CommandReceipt { edit_ids, generation: self.generation(), messages: std::mem::take(&mut self.replay.pending_report.messages), worst: self.replay.pending_report.worst.take() })
    }

    //#region 📬️OneItemPublication
    /// ⏭️ Advances at most one preparation, fold, cursor, preflight, or atomic move-publication
    /// unit. Exactly one admitted mutation is folded into the staged edit per fold turn; the
    /// post-snapshot each item produced is an already-built `Arc` and is moved into the stage.
    pub fn advance_apply_batch(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mutation>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemAdvance, VcsError>
    where
        P: Sync,
    {
        self.ensure_durable_group_idle()?;
        if publication.group_preparation.is_some() { return Err(VcsError::ValidationFailed("group-preparing batch requires its exact common publication decision".into())); }
        if publication.phase == ArtifactStoreOneItemPublicationPhase::Complete {
            return Ok(ArtifactStoreOneItemAdvance::Complete);
        }
        if publication.phase == ArtifactStoreOneItemPublicationPhase::Closing {
            return Ok(ArtifactStoreOneItemAdvance::Blocked);
        }
        if publication.phase == ArtifactStoreOneItemPublicationPhase::AwaitingAck {
            return Ok(ArtifactStoreOneItemAdvance::AwaitingAck(publication.receipt.expect("awaiting-ACK publication retains its receipt")));
        }
        if publication.cancel_requested {
            publication.begin_close();
            return Ok(ArtifactStoreOneItemAdvance::Blocked);
        }
        if self.cursor.generation != publication.expected_generation || self.cursor.content_revision != publication.expected_revision {
            publication.fault = Some(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"batched publication became stale before commit"));
            publication.phase = ArtifactStoreOneItemPublicationPhase::Fault;
            publication.begin_close();
            return Err(VcsError::ValidationFailed("batched publication became stale before commit".into()));
        }
        if !grant.permits_one() {
            return Ok(ArtifactStoreOneItemAdvance::Blocked);
        }
        let item_grant = ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), ..grant };
        match publication.phase {
            ArtifactStoreOneItemPublicationPhase::Preparing => {
                if !publication.source_census_finished {
                    let copy=size_of::<ArtifactStoreOneItemFootprint>()+size_of::<usize>();
                    if item_grant.maximum_copy_bytes<copy||item_grant.maximum_depth==0{return Ok(ArtifactStoreOneItemAdvance::Blocked);}
                    match publication.source.as_mut().ok_or_else(|| VcsError::ValidationFailed("batched publication lost its retained census source".into()))?.census_one(publication.lane).map_err(VcsError::ValidationFailed)? {
                        Some(footprint) => { publication.footprint = footprint; publication.source_census_finished = true; },
                        None => { publication.source_census_completed = publication.source_census_completed.saturating_add(1); },
                    }
                    return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}));
                }
                if publication.item_closing {
                    if let Some(stage)=publication.stage.as_mut(){if stage.retiring.is_some(){let step=artifact_retirement_box_close_step(&mut stage.retiring,item_grant.retained_grant()).map_err(|error|VcsError::ValidationFailed(error.into_message()))?;return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),step.progress()));}}
                    if publication.preparation.is_none(){return Err(VcsError::ValidationFailed("staged batch item lost its original preparation before closure".into()));}
                    let step=Self::spend_item_close_grant(&mut publication.preparation,item_grant).map_err(|error|VcsError::ValidationFailed(error.into_message()))?;
                    if matches!(step,RetainedCloneStep::Complete(_)){publication.item_closing=false;}
                    return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),step.progress()));
                }
                if publication.stage.as_ref().is_some_and(|stage|stage.folding.is_some()){return self.fold_batch_item(publication,item_grant);}
                if publication.preparation.is_none() {
                    let staged_root = publication.stage.as_ref().and_then(|stage| stage.post.clone());
                    let remaining = publication.source.as_ref().ok_or_else(|| VcsError::ValidationFailed("batched publication lost its admitted item source".into()))?.remaining();
                    if remaining == 0 {
                        if publication.stage.is_none() {
                            return Err(VcsError::ValidationFailed("batched publication staged no edit before its cursor turn".into()));
                        }
                        let copy=size_of::<ArtifactStoreOneItemPublicationPhase>();if item_grant.maximum_copy_bytes<copy||item_grant.maximum_depth==0{return Ok(ArtifactStoreOneItemAdvance::Blocked);}
                        publication.phase = ArtifactStoreOneItemPublicationPhase::PreparingCursor;
                        return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}));
                    }
                    let refused = |error: ValueError| VcsError::ValidationFailed(error.into_message());
                    let demand = publication.source.as_ref().expect("validated batched publication item source").next_begin_demand(publication.lane).map_err(refused)?;
                    if demand.admit(item_grant.retained_grant()).is_err() { return Ok(ArtifactStoreOneItemAdvance::Blocked); }
                    let original_issuer_copy=size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>>()+size_of::<Arc<dyn SnapshotRetirementFactory<P>>>();
                    if item_grant.maximum_copy_bytes<original_issuer_copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}
                    let mutation_retirement=self.owners.mutation_retirement_factory.as_ref().ok_or_else(||VcsError::ValidationFailed("batched preparation lost its installed mutation issuer".into()))?;
                    let snapshot_retirement=self.owners.snapshot_retirement_factory.as_ref().ok_or_else(||VcsError::ValidationFailed("batched preparation lost its installed snapshot issuer".into()))?;
                    let authority = Arc::clone(publication.authority.as_ref().ok_or_else(|| VcsError::ValidationFailed("batched publication lost its live authority".into()))?);
                    let base = self.batch_item_base(staged_root.as_ref())?;
                    let request = ArtifactStoreBatchItemRequest { operation: publication.operation, generation: authority.generation(), base_revision: authority.base_revision(), lane: publication.lane, authority, base, mutation_retirement:Arc::clone(mutation_retirement),snapshot_retirement:Arc::clone(snapshot_retirement) };
                    let source = publication.source.as_mut().expect("validated batched publication item source");
                    match source.begin_next(request, ArtifactStoreOneItemGrant{maximum_copy_bytes:item_grant.maximum_copy_bytes-original_issuer_copy,..item_grant}) {
                        Ok((preparation, progress)) => {
                            publication.preparation = Some(preparation);
                            let progress=RetainedCloneProgress{copied_bytes:progress.copied_bytes.checked_add(original_issuer_copy).ok_or_else(||VcsError::ValidationFailed("batched preparation original issuer copy receipt overflow".into()))?,..progress};
                            if let Err(error)=semio_framework_value::retained_clone::admit_retained_clone_progress(item_grant.retained_grant(), progress, "batched preparation birth") { publication.fault=Some(error.with_retained_progress(progress));publication.begin_close();return Ok(ArtifactStoreOneItemAdvance::PreparationRefused); }
                            return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),progress));
                        }
                        Err((error, request)) => {
                            let ArtifactStoreBatchItemRequest { authority, base, mutation_retirement, snapshot_retirement, .. } = request;
                            publication.refused_base = Some(base);
                            publication.refused_authority = Some(authority);
                            let mutation: Arc<dyn semio_framework_value::FactoryRetirement> = mutation_retirement;
                            let snapshot: Arc<dyn semio_framework_value::FactoryRetirement> = snapshot_retirement;
                            publication.refused_issuers = [Some(semio_framework_value::FactoryAuthority::new(mutation)), Some(semio_framework_value::FactoryAuthority::new(snapshot))];
                            let original_progress = error.retained_progress();
                            let copied_bytes = match original_progress.copied_bytes.checked_add(original_issuer_copy) {
                                Some(bytes) => bytes,
                                None => { publication.fault = Some(error); publication.begin_close(); return Ok(ArtifactStoreOneItemAdvance::PreparationRefused); }
                            };
                            let progress = RetainedCloneProgress { copied_bytes, ..original_progress };
                            publication.fault = Some(error.with_retained_progress(progress));
                            publication.begin_close();
                            return Ok(ArtifactStoreOneItemAdvance::PreparationRefused);
                        }
                    }
                    return Ok(ArtifactStoreOneItemAdvance::Progress(publication.progress()));
                }
                let owner = publication.preparation.as_mut().expect("validated batched publication item preparation");
                if owner.prepared().is_none() {
                    let result=owner.advance(item_grant).and_then(|step|validate_original_preparation_step(step,item_grant,owner.prepared().is_some()));
                    let step=match result {Ok(step)=>step,Err(reason)=>{publication.fault=Some(reason);publication.begin_close();return Ok(ArtifactStoreOneItemAdvance::PreparationRefused)}};
                    return match step {
                        ArtifactStoreOneItemPreparationStep::Progress(checkpoint, ownership) => Ok(ArtifactStoreOneItemAdvance::PreparationProgress(checkpoint, ownership)),
                        ArtifactStoreOneItemPreparationStep::Prepared(checkpoint, ownership) => {
                            Ok(ArtifactStoreOneItemAdvance::PreparationProgress(checkpoint,ownership))
                        }
                        ArtifactStoreOneItemPreparationStep::Blocked => Ok(ArtifactStoreOneItemAdvance::Blocked),
                    };
                }
                self.fold_batch_item(publication, item_grant)
            }
            ArtifactStoreOneItemPublicationPhase::PreparingCursor => {
                let stage = publication.stage.as_ref().ok_or_else(|| VcsError::ValidationFailed("batched cursor preparation lost its staged edit".into()))?;
                let authority = publication.authority.as_ref().ok_or_else(|| VcsError::ValidationFailed("batched cursor preparation lost its live authority".into()))?;
                if stage.edit.as_ref().expect("live staged edit owner").id.is_empty() || stage.edit.as_ref().expect("live staged edit owner").id.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES || authority.base_applied_edit_count() != self.log.applied_edit_ids.len() {
                    return Err(VcsError::ValidationFailed("domain-prepared batched candidate failed its live cursor authority".into()));
                }
                let envelope=&mut **self.log.envelope;
                if let Some(edit_id) = envelope.open_transaction.as_ref().filter(|open|publication.transaction.as_ref().is_some_and(|transaction|open.transaction.id==transaction.id)&&self.log.applied_edit_ids.last()==Some(&open.edit_id)).map(|open|open.edit_id.as_str()) {
                    let existing = envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id).ok_or_else(|| VcsError::ValidationFailed("batched cursor preparation lost its open edit".into()))?;
                    let capacity = existing.inverse.len().checked_add(stage.edit.as_ref().expect("live staged edit owner").inverse.len()).ok_or_else(|| VcsError::ValidationFailed("batched inverse capacity overflow".into()))?;
                    if let Some(bytes) = existing.inverse.next_capacity_allocation_bytes(capacity).map_err(|error| VcsError::ValidationFailed(error.reason.into()))? {
                        let depth=existing.inverse.next_reserve_depth_demand().map_err(|error|VcsError::ValidationFailed(error.reason.into()))?;
                        if bytes>grant.maximum_capacity_bytes||grant.maximum_depth<depth{return Ok(ArtifactStoreOneItemAdvance::Blocked);}
                        let step = existing.inverse.reserve_capacity_one(capacity, grant.maximum_capacity_bytes).map_err(|error| VcsError::ValidationFailed(error.reason.into()))?;
                        if !step.progressed || step.allocated_bytes > grant.maximum_capacity_bytes {
                            return Err(VcsError::ValidationFailed("batched inverse backing failed its exact allocation admission".into()));
                        }
                        return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,retained_capacity_bytes:step.allocated_bytes,..Default::default()}));
                    }
                }
                let copy=size_of::<ArtifactStoreOneItemPublicationPhase>();if grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return Ok(ArtifactStoreOneItemAdvance::Blocked);}
                publication.phase = ArtifactStoreOneItemPublicationPhase::PreflightingCommit;
                Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))
            }
            ArtifactStoreOneItemPublicationPhase::PreflightingCommit => {
                let stage = publication.stage.as_mut().ok_or_else(|| VcsError::ValidationFailed("batched commit preflight lost its staged edit".into()))?;
                let authority = publication.authority.as_ref().ok_or_else(|| VcsError::ValidationFailed("batched commit preflight lost its live authority".into()))?;
                let refusal=|error:ValueError|VcsError::ValidationFailed(error.into_message());
                if let Some(owner)=stage.digest_owner.as_mut(){
                    if owner.ready(){let Some((edit,digest,ownership))=owner.take_edit(grant.retained_grant()).map_err(refusal)?else{return Ok(ArtifactStoreOneItemAdvance::Blocked);};stage.edit=Some(edit);stage.digest=digest;return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),ownership));}
                    if owner.terminal_is_empty(){let released=size_of_val(owner.as_ref());if grant.maximum_release_bytes<released||grant.maximum_depth==0{return Ok(ArtifactStoreOneItemAdvance::Blocked);}stage.digest_owner=None;stage.digest_complete=true;return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,released_bytes:released,..Default::default()}));}
                    let step=owner.advance_digest(grant.retained_grant()).map_err(refusal)?;let ownership=semio_framework_value::retained_clone::admit_retained_clone_progress(grant.retained_grant(),step.progress(),"original batch digest normal step").map_err(refusal)?;return Ok(if ownership==Default::default(){ArtifactStoreOneItemAdvance::Blocked}else{ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),ownership)});
                }

                let meta = stage.edit.as_ref().expect("live staged edit owner").mutation_meta.first().ok_or_else(|| VcsError::ValidationFailed("batched edit lacks its exact authority metadata".into()))?;
                if publication.lane != HistoryLane::Document
                    || stage.folded != publication.admitted_items
                    || stage.edit.as_ref().expect("live staged edit owner").sequence_number != authority.next_sequence_number()
                    || stage.edit.as_ref().expect("live staged edit owner").forwards.len() != publication.admitted_items
                    || stage.edit.as_ref().expect("live staged edit owner").mutation_meta.len() != publication.admitted_items
                    || stage.edit.as_ref().expect("live staged edit owner").id != stage.applied_edit_id
                    || stage.edit.as_ref().expect("live staged edit owner").id != stage.tail_edit_id
                    || stage.edit.as_ref().expect("live staged edit owner").actor.as_deref() != Some(authority.actor())
                    || !stage.local_actor.as_ref().is_some_and(|actor|actor.eq_str(&authority.actor()))
                    || stage.next_clock != authority.next_clock()
                    || meta.author_id.as_ref().map(|actor| actor.0.as_str()) != Some(authority.actor())
                    || meta.timestamp != authority.next_clock()
                    || stage.post.is_none()
                    || (self.wire.backbone.is_some() && !publication.outbound)
                {
                    return Err(VcsError::ValidationFailed("batched prepared candidate failed its exact fixed commit contract".into()));
                }
                let rows = stage.edit.as_ref().expect("live staged edit owner").inverse.len().saturating_add(stage.edit.as_ref().expect("live staged edit owner").forwards.len());
                if rows > publication.footprint.work_items {
                    return Err(VcsError::TooLarge { rows, capacity: publication.footprint.work_items });
                }
                let amending = self.batch_amend_target_ref(publication.transaction.as_ref()).is_some();
                if !amending && (!self.log.applied_edit_ids.admits_one() || !self.cursor.revision_accumulator.applied.admits_one() || self.log.envelope.cursor.as_ref().is_none_or(|cursor| !cursor.applied_edit_ids.admits_one())) {
                    return Err(VcsError::HistoryFull { capacity: self.log.applied_edit_ids.len() });
                }
                if self.cursor.generation == u64::MAX {
                    return Err(VcsError::ValidationFailed("batched store generation is exhausted".into()));
                }
                if !stage.digest_complete{
                    let admitted=match self.owners.one_item_preparation_factory.as_ref(){Some(factory)=>factory.begin_batch_digest(&mut stage.edit,grant.retained_grant()),None=>self.log.operation_wire.as_ref().ok_or_else(||VcsError::ValidationFailed("batch original canonical digest authority is absent".into()))?.begin_batch_digest(&mut stage.edit,grant.retained_grant())};
                    let Some((owner,ownership))=admitted.map_err(refusal)?else{return Ok(ArtifactStoreOneItemAdvance::Blocked);};stage.digest_owner=Some(owner);return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),ownership));
                }
                let copy=size_of::<ArtifactStoreOneItemPublicationPhase>();if grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return Ok(ArtifactStoreOneItemAdvance::Blocked);}
                self.owners.displaced_retirements.reserve(12)?;
                if self.owners.snapshot_retirement_factory.is_none() || self.owners.mutation_retirement_factory.is_none() {
                    return Err(VcsError::ValidationFailed("batched commit lacks exact snapshot or mutation retirement authority".into()));
                }
                publication.phase = ArtifactStoreOneItemPublicationPhase::Publishing;
                Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))
            }
            ArtifactStoreOneItemPublicationPhase::Publishing => {
                if let Some(edit_id) = self.batch_amend_target(publication.transaction.as_ref()) {
                    let stage = publication.take_stage().ok_or_else(|| VcsError::ValidationFailed("batched publication lost its staged edit at atomic transfer".into()))?;
                    let ArtifactStoreBatchStage { edit, post, next_clock, local_actor: _, unit_flags, .. } = *stage;
                    let edit = edit.expect("live staged edit owner");
                    let post = post.ok_or_else(|| VcsError::ValidationFailed("batched publication lost its staged post root at atomic transfer".into()))?;
                    let generation_before = self.cursor.generation;
                    let revision_before = self.cursor.content_revision;
                    let appended = edit.forwards.len();
                    let closes = publication.transaction.is_some() && !publication.transaction_open;
                    let mut offset = 0;
                    if let Some(existing) = self.log.envelope.vcs.edits.iter_mut().find(|candidate| candidate.id == edit_id) {
                        offset = existing.forwards.len();
                        publication.announce_from = if closes { 0 } else { existing.forwards.len() };
                        existing.forwards.extend(edit.forwards);
                        existing.inverse.extend(edit.inverse);
                        existing.mutation_meta.extend(edit.mutation_meta);
                        existing.finished_at = if closes { Some(now_iso()) } else { edit.finished_at };
                        if closes {
                            stamp_primary_operation_identity(existing);
                        }
                        publication.announce_items = existing.forwards.len() - publication.announce_from;
                    } else {
                        return Err(VcsError::ValidationFailed("batched transaction append lost its open edit".into()));
                    }
                    if closes {
                        self.log.envelope.open_transaction = None;
                    } else if publication.transaction_open {
                        publication.outbound = false;
                    }
                    self.replace_pending_report_retained(PendingCommandReport::default())?;
    
                    self.replace_redo_edit_ids_retained(Vec::new())?;
                    self.replace_current_retained(post)?;
                    self.log.clock = next_clock;
                    let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit_id.as_bytes()]);
                    self.cursor.revision_accumulator.unit_flags.extend(unit_flags.into_iter().map(|(index, flag)| ((digest, offset + index), flag)));
                    self.bump()?;
                    self.continue_pending_replay_across_append(revision_before, &edit_id, appended);
                    self.cursor.last_projection_cause = Some(ArtifactProjectionCause::Apply);
                    let receipt = LaneItemReceipt { generation_before, generation_after: self.cursor.generation };
                    publication.expected_generation = self.cursor.generation;
                    publication.expected_revision = self.cursor.content_revision;
                    publication.receipt = Some(receipt);
                    publication.published = true;
                    publication.phase = ArtifactStoreOneItemPublicationPhase::AwaitingAck;
                    return Ok(ArtifactStoreOneItemAdvance::Published(receipt));
                }
                let reservation = self.reserve_edit_history_slot()?;
                let stage = publication.take_stage().ok_or_else(|| VcsError::ValidationFailed("batched publication lost its staged edit at atomic transfer".into()))?;
                let ArtifactStoreBatchStage { edit, post, next_clock, local_actor: _, applied_edit_id, tail_edit_id, digest, unit_flags, .. } = *stage;
                let edit = edit.expect("live staged edit owner");
                let digest = if edit.forwards.len() > 1 { CursorRevisionAccumulator::effective_edit_digest::<P, Mutation>(&edit, CursorRevisionAccumulator::edit_digest(&*edit), &self.live.supersessions) } else { digest };
                publication.announce_from = 0;
                publication.announce_items = publication.admitted_items;
                let post = post.ok_or_else(|| VcsError::ValidationFailed("batched publication lost its staged post root at atomic transfer".into()))?;
                let generation_before = self.cursor.generation;
                let pre_snapshot = Arc::clone(&self.live.current);
                let cursor = self.log.envelope.cursor.as_mut().ok_or_else(|| VcsError::ValidationFailed("batched publication lost its live cursor authority".into()))?;
                let retired_cursor_redo = std::mem::take(&mut cursor.redo_edit_ids);
                cursor.applied_edit_ids.push(edit.id.clone());
                if !retired_cursor_redo.is_empty() || retired_cursor_redo.capacity() != 0 {
                    self.owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreStringVectorRetirement::new(retired_cursor_redo)));
                }
                self.replace_pending_report_retained(PendingCommandReport::default())?;

                self.replace_tail_undo_cache_retained(Some((tail_edit_id, Arc::clone(&pre_snapshot))))?;
                self.log.applied_edit_ids.push(applied_edit_id);
                self.replace_redo_edit_ids_retained(Vec::new())?;
                self.replace_current_retained(post)?;
                self.log.edit_sequence = edit.sequence_number;
                self.log.clock = next_clock;
                let edit_id_digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit.id.as_bytes()]);
                let previous = self.cursor.revision_accumulator.applied.last().map_or(self.cursor.revision_accumulator.identity_digest, |record| record.prefix_digest);
                let prefix_digest = CursorRevisionAccumulator::hash_record(b"applied", &[&previous, &digest]);
                let opened = publication.transaction.clone().filter(|_| publication.transaction_open).map(|transaction| OpenToolTransaction { transaction, edit_id: edit.id.clone() });
                self.cursor.revision_accumulator.index_applied_edit::<P, Mutation>(self.log.applied_edit_ids.len() - 1, &edit);
                self.cursor.revision_accumulator.unit_flags.extend(unit_flags.into_iter().map(|(index, flag)| ((edit_id_digest, index), flag)));
                let ledger_key = self.insert_reserved_edit_history(reservation, *edit)?;
                if opened.is_some() {
                    publication.outbound = false;
                    self.log.envelope.open_transaction = opened;
                }
                self.cursor.revision_accumulator.applied.push(CursorRevisionRecord { ledger_key: Some(ledger_key), id_digest: edit_id_digest, edit_digest: digest, prefix_digest });
                let retired_redo = std::mem::take(&mut self.cursor.revision_accumulator.redo);
                if !retired_redo.is_empty() || retired_redo.capacity() != 0 {
                    self.owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreRevisionAccumulatorRetirement::new(CursorRevisionAccumulator {
                        identity_digest: self.cursor.revision_accumulator.identity_digest,
                        applied: semio_framework_os_vcs::HistoryPageStack::new(),
                        redo: retired_redo,
                        applied_tail_chains: None,
                        mutation_positions: protocol::HistoryFoldIndex::new(),
                        indexed_edits: protocol::HistoryFoldIndex::new(),
                        unit_flags: protocol::HistoryFoldIndex::new(),
                    })));
                }
                self.cursor.generation += 1;
                self.cursor.content_revision = self.cursor.revision_accumulator.revision(self.log.current_checkpoint_id.as_deref());
                assert!(self.owners.snapshot_read_leases.publish_authority(self.cursor.generation, self.cursor.content_revision), "single-owner batched publication must advance its exact snapshot authority");
                self.cursor.last_projection_cause = Some(ArtifactProjectionCause::Apply);
                let receipt = LaneItemReceipt { generation_before, generation_after: self.cursor.generation };
                publication.expected_generation = self.cursor.generation;
                publication.expected_revision = self.cursor.content_revision;
                publication.receipt = Some(receipt);
                publication.published = true;
                publication.phase = ArtifactStoreOneItemPublicationPhase::AwaitingAck;
                Ok(ArtifactStoreOneItemAdvance::Published(receipt))
            }
            ArtifactStoreOneItemPublicationPhase::Fault => Ok(ArtifactStoreOneItemAdvance::PreparationRefused),
            ArtifactStoreOneItemPublicationPhase::AwaitingAck | ArtifactStoreOneItemPublicationPhase::Closing | ArtifactStoreOneItemPublicationPhase::Complete => unreachable!("handled batched terminal control phase"),
        }
    }

    /// 🧹️ Preserves the original preparation receipt before admitting its final owned Box frame.
    fn spend_item_close_grant(owner: &mut Option<Box<dyn ArtifactStoreOneItemPreparation<P, Mutation>>>, grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneStep, RetainedCloneProgress};
        let Some(current) = owner.as_mut() else { return Ok(RetainedCloneStep::Complete(Default::default())); };
        if !current.terminal_is_empty() {
            if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
            let depth = current.next_close_depth_demand()?.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "batch preparation Box depth overflow"))?;
            if grant.maximum_depth < depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "batch preparation exceeds admitted Box depth")); }
            let child_grant = ArtifactStoreOneItemGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = current.close_step(child_grant)?;
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(child_grant.retained_grant(), step, current.terminal_is_empty(), "batch original item preparation")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "batch preparation Box requires admitted depth")); }
        let bytes = size_of_val(current.as_ref());
        if bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        drop(owner.take());
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }))
    }

    //#endregion 📬️OneItemPublication
    pub async fn apply_one(&mut self, expected_generation: u64, mutation: Mutation, lane: HistoryLane, identity: &mut EntityIdentityAuthority<'_>) -> Result<(CommandReceipt, LaneItemReceipt), VcsError>
    where
        P: Sync,
    {
        self.ensure_durable_group_idle()?;
        self.pump().await?;
        if self.cursor.generation != expected_generation {
            return Err(VcsError::ValidationFailed("one-item publication generation is stale".to_string()));
        }
        let generation_before = self.cursor.generation;
        self.replace_pending_report_retained(PendingCommandReport::default())?;
        let before = self.log.applied_edit_ids.len();
        self.apply_command(vec![mutation], lane, None, identity).await?;
        self.cursor.last_projection_cause = Some(ArtifactProjectionCause::Apply);
        self.flush_outbound().await?;
        let edit_ids = self.replay.pending_report.edit_ids.take().unwrap_or_else(|| self.log.applied_edit_ids.cloned_from(before));
        let receipt = CommandReceipt { edit_ids, generation: self.cursor.generation, messages: std::mem::take(&mut self.replay.pending_report.messages), worst: self.replay.pending_report.worst.take() };
        Ok((receipt, LaneItemReceipt { generation_before, generation_after: self.cursor.generation }))
    }

    //#region 🔖️EventSourcedHistory
    /// 🖋️ Authors one local history transition: stamps it on this replica's clock, records it
    /// in the event log, and materializes the projection through the same fold remote transitions
    /// take. The caller's dispatch announces it outbound with every other new event.
    async fn commit_transition(&mut self, transition: semio_framework_os_spr::HistoryTransition, dependencies: Vec<MutationId>) -> Result<(), VcsError> {
        let mut timestamp = self.log.clock;
        timestamp.tick(now_ms());
        let envelope = self.transition_envelope(&transition, dependencies, Vec::new(), timestamp);
        self.install_transitions(vec![envelope], timestamp, None).await
    }

    /// 🏗️ Records locally authored transition envelopes and materializes the projection once — adopting `replayed` when it
    /// is the Report replay of exactly the resulting log, replaying otherwise (under [`Self::defer_local_replays`] a replay
    /// longer than one budget waits for later turns). All of them leave in this dispatch's outbound batch, or none is
    /// recorded.
    async fn install_transitions(&mut self, envelopes: Vec<semio_framework_os_spr::MutationEnvelope>, clock: HybridLogicalTimestamp, replayed: Option<EditReplayResult<P, Mutation>>) -> Result<(), VcsError> {
        if let Err(error) = self.admit_history_shape(&envelopes) {
            if let Some(replayed) = replayed {
                let _ = self.retire_replay_result(replayed);
            }
            return Err(error);
        }
        if let (None, [envelope]) = (&replayed, envelopes.as_slice()) {
            if let Some(step) = self.tail_step(envelope) {
                let envelope = envelopes.into_iter().next().expect("one tail-step envelope");
                return self.adopt_tail_step(step, envelope, clock);
            }
        }
        let envelopes = match replayed {
            Some(_) => envelopes,
            None => match self.defer_local_step(PendingLocalStep { envelopes, head: self.viewer_head_now(), finalizing: false }, clock).await? {
                Some(step) => step.envelopes,
                None => return Ok(()),
            },
        };
        let identities: Vec<MutationId> = envelopes.iter().map(|envelope| envelope.mutation_id.clone()).collect();
        for envelope in &envelopes {
            self.insert_transition(envelope.clone());
        }
        let projected = match replayed {
            Some(result) => self.reproject_replayed(result),
            None => self.reproject().await,
        };
        if let Err(error) = projected {
            self.log.envelope.transitions.retain(|known| !identities.contains(&known.mutation_id));
            return Err(error);
        }
        for identity in identities {
            self.log.dag.seed_applied(identity).map_err(|error| VcsError::ValidationFailed(error.to_string()))?;
        }
        self.log.clock = clock;
        self.replay.pending_report.outbound.extend(envelopes);
        self.bump()
    }

    /// ⚡️ Adopts a tail step ([`Self::tail_step`]) without folding the log: projects through the tail snapshot or the nearest
    /// retained prefix folded forward (undo), or the edit's effective forwards folded onto the head (redo), records `envelope`,
    /// moves the edit between the stacks and keeps the prefixes it folded through. The edit's inverse and messages stand — its
    /// base is the projection it was last replayed on — and the announcement leaves in this dispatch's outbound batch.
    fn adopt_tail_step(&mut self, step: TailStep, envelope: semio_framework_os_spr::MutationEnvelope, clock: HybridLogicalTimestamp) -> Result<(), VcsError> {
        let key = envelope.timestamp.cmp_key();
        let (projection, tail, recorded) = match step {
            TailStep::Undo => {
                let (base, recorded) = self.live_prefix_state(self.log.applied_edit_ids.len() - 1)?;
                (base, None, recorded)
            }
            TailStep::Redo => {
                let edit_id = self.log.redo_edit_ids.last().expect("a redo tail step names the redo top").clone();
                let edit = self.log.envelope.vcs.edits.find_near(|edit| edit.id == edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
                let pre = Arc::clone(&*self.live.current);
                let mut next = Arc::clone(&pre);
                fold_effective_edit(&mut next, edit, &self.log.envelope.schema, &self.live.supersessions, usize::MAX);
                (next, Some((edit_id, pre)), Vec::new())
            }
        };
        if let Err(error) = self.owners.displaced_retirements.reserve(REPROJECTION_OWNER_SLOTS + 2 * recorded.len()) {
            retire_shared_projection::<P, Mutation>(projection);
            if let Some((_, snapshot)) = tail {
                retire_shared_projection::<P, Mutation>(snapshot);
            }
            for (_, _, snapshot) in recorded {
                retire_shared_projection::<P, Mutation>(snapshot);
            }
            return Err(error);
        }
        match step {
            TailStep::Undo => {
                self.replace_current_retained(projection)?;
                self.replace_tail_undo_cache_retained(tail)?;
                let edit_id = self.pop_applied_edit_id().expect("an undo tail step pops the applied tail");
                self.push_redo_edit_id(edit_id);
            }
            TailStep::Redo => {
                self.replace_tail_undo_cache_retained(tail)?;
                self.replace_current_retained(projection)?;
                let edit_id = self.pop_redo_edit_id().expect("a redo tail step pops the redo top");
                self.push_applied_edit_id(edit_id);
            }
        }
        self.admit_prefix_snapshots(recorded)?;
        self.insert_transition(envelope.clone());
        self.live.fold_frontier = Some(key);
        self.log.dag.seed_applied(envelope.mutation_id.clone()).map_err(|error| VcsError::ValidationFailed(error.to_string()))?;
        self.log.clock = clock;
        self.replay.pending_report.outbound.push(envelope);
        self.bump()?;
        self.live.tail_step_generation = Some(self.cursor.generation);
        Ok(())
    }

    //#endregion 🔖️EventSourcedHistory
    /// 🛤️ Same edit-recording path as `Apply`, tagging the fresh edit into `lane` instead of
    /// the implicit `Document` default — see `HistoryLane`'s doc.
    async fn dispatch_inner(&mut self, command: ArtifactCommand<Mutation>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError>
    where
        P: Sync,
    {
        let shape = self.log.envelope.history_shape;
        if let Some(kind) = command.history_transition_kinds().iter().copied().find(|kind| !shape.admits(*kind)) {
            if let ArtifactCommand::Supersede { inputs, .. } | ArtifactCommand::CreateAlternativeWithSupersede { inputs, .. } = command {
                retire_supersede_inputs::<P, Mutation>(inputs);
            }
            return Err(VcsError::HistoryShape { shape, kind });
        }
        if let Some(open) = self.log.envelope.open_transaction.as_ref() {
            let admitted = match &command {
                ArtifactCommand::AppendTransaction { transaction, .. } => transaction.id == open.transaction.id,
                ArtifactCommand::CommitTransaction { .. } | ArtifactCommand::AbortTransaction { .. } | ArtifactCommand::IngestRemote { .. } | ArtifactCommand::SetMergePolicy { .. } => true,
                _ => false,
            };
            if !admitted {
                let transaction_id = open.transaction.id.clone();
                retire_command::<P, Mutation>(command);
                return Err(VcsError::TransactionOpen { transaction_id });
            }
        }
        if command.moves_history() && self.local_step_pending() {
            retire_command::<P, Mutation>(command);
            return Err(VcsError::HistoryReplaying);
        }
        match command {
            ArtifactCommand::Undo => self.undo_with_policy(UndoPolicy::TransformAgainstConcurrent, None, identity).await,
            ArtifactCommand::UndoWithPolicy { policy, semantic_command } => self.undo_with_policy(policy, semantic_command, identity).await,
            ArtifactCommand::Redo => {
                let position = self.redo_position(|lane| lane == HistoryLane::Document).ok_or(VcsError::NothingToRedo)?;
                self.redo_lane_position(position).await
            }
            ArtifactCommand::UndoInLane { lane } => {
                let position = self.log.applied_edit_ids.iter().rposition(|id| self.edit_is_local(id) && self.edit_lane(id) == lane).ok_or(VcsError::NothingToUndo)?;
                self.undo_lane_position(position).await
            }
            ArtifactCommand::RedoInLane { lane } => {
                let position = self.redo_position(|edit_lane| edit_lane == lane).ok_or(VcsError::NothingToRedo)?;
                self.redo_lane_position(position).await
            }
            ArtifactCommand::CommitCheckpoint { message, authors } => self.commit_pending_checkpoint(message, authors, identity).await,
            ArtifactCommand::CreateAlternative { name } => {
                if self.log.envelope.vcs.checkpoints.is_empty() {
                    if self.log.applied_edit_ids.is_empty() {
                        return Err(VcsError::NoCheckpoint);
                    }
                    self.commit_pending_checkpoint(None, Vec::new(), identity).await?;
                }
                let checkpoint_id = (*self.log.current_checkpoint_id).clone().or_else(|| self.log.envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone())).ok_or(VcsError::NoCheckpoint)?;
                let alternative_id = identity.encode(|control|mint_alternative_id(&name, std::slice::from_ref(&checkpoint_id), control))?;
                if self.log.envelope.vcs.alternatives.iter().any(|alternative| alternative.id == alternative_id) {
                    return Err(VcsError::ValidationFailed(format!("alternative {alternative_id} already exists")));
                }
                let dependencies = self.checkpoint_origin(&checkpoint_id).into_iter().collect();
                self.commit_transition(semio_framework_os_spr::HistoryTransition::Branch { alternative_id: alternative_id.clone(), name, checkpoint_id }, dependencies).await?;
                self.log.envelope.active_alternative_id = Some(alternative_id);
                self.log.envelope.viewer_checkpoint_id = None;
                self.reproject().await?;
                self.bump()
            }
            ArtifactCommand::SwitchAlternative { alternative_id } => self.switch_local_alternative(alternative_id).await,
            ArtifactCommand::CheckoutCheckpoint { checkpoint_id } => self.checkout_local_checkpoint(checkpoint_id).await,
            ArtifactCommand::Apply { mutations, transaction } => self.apply_command(mutations, HistoryLane::Document, transaction, identity).await,
            ArtifactCommand::ApplyInLane { mutations, lane, transaction } => self.apply_command(mutations, lane, transaction, identity).await,
            ArtifactCommand::IngestRemote { envelope } => {
                let report = self.ingest_remote(envelope).await?;
                self.absorb_merge_report(&report);
                Ok(())
            }
            ArtifactCommand::PruneDrafts => Err(VcsError::ValidationFailed("draft pruning is not implemented by ArtifactStore; no draft history was removed".to_string())),
            ArtifactCommand::SetMergePolicy { policy } => {
                self.log.merge_policy = policy;
                Ok(())
            }
            ArtifactCommand::ResolveConflict { conflict_id, resolution } => {
                let report = self.resolve_conflict(&conflict_id, resolution).await?;
                self.absorb_merge_report(&report);
                Ok(())
            }
            ArtifactCommand::Supersede { scope, inputs } => self.supersede_command(scope, inputs, identity).await,
            ArtifactCommand::CreateAlternativeWithSupersede { name, inputs } => self.create_alternative_with_supersede(name, inputs, identity).await,
            ArtifactCommand::AppendTransaction { mutations, transaction } => self.append_transaction(mutations, transaction, identity).await,
            ArtifactCommand::CommitTransaction { transaction_id } => self.commit_open_transaction(&transaction_id),
            ArtifactCommand::AbortTransaction { transaction_id } => self.abort_open_transaction(&transaction_id),
        }
    }

    //#region 🔖️Supersede
    /// ✏️ `Supersede`: supersedes typed inputs in every alternative or within one existing alternative
    /// ([`Self::author_supersession`]).
    async fn supersede_command(&mut self, scope: Option<String>, inputs: Vec<SupersedeInput<Mutation>>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let trunk = self.trunk_alternative_id();
        let scope = match scope {
            None => SupersedeScope::Document,
            Some(alternative) if alternative == trunk || self.log.envelope.vcs.alternatives.iter().any(|known| known.id == alternative) => SupersedeScope::Alternative(alternative),
            Some(alternative) => {
                retire_supersede_inputs::<P, Mutation>(inputs);
                return Err(VcsError::UnknownAlternative(alternative));
            }
        };
        let (inputs, target) = self.supersede_inputs(inputs)?;
        self.author_supersession(scope, inputs, target, None, identity).await
    }

    /// 🌿️ `CreateAlternativeWithSupersede`: the pending-edit commit, a `Branch` at the head and the supersession scoped to
    /// the new alternative, authored as one batch ([`Self::author_supersession`]).
    async fn create_alternative_with_supersede(&mut self, name: String, inputs: Vec<SupersedeInput<Mutation>>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let (inputs, target) = self.supersede_inputs(inputs)?;
        self.author_supersession(SupersedeScope::NewAlternative(name), inputs, target, None, identity).await
    }

    /// 🔚️ Finalizes a time-travel session without replaying again: `finished` is the completed Report replay of the
    /// session's drafts ([`Self::begin_report_replay`]). Refused with `Stale` when this store's content moved since the replay
    /// began (its content revision differs; the session replays again — a port rebinding moves no event and keeps the replay) and with `Rejected { policy: Normal }` when its report blocks finalizing; otherwise the
    /// drafts are authored as an overwrite or a new alternative and installed atomically with the replay's projection,
    /// rebased inverses and messages in one outbound batch — the same install the `Supersede` command takes.
    pub async fn commit_finished_replay(&mut self, finished: EditReplayResult<P, Mutation>, finalization: HistoryFinalization, identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError>
    where
        P: Sync,
    {
        let mut slot = Some(finished);
        let outcome = self.commit_slotted_replay(&mut slot, finalization, identity).await;
        if let Some(unused) = slot {
            let _ = self.retire_replay_result(unused);
        }
        outcome
    }

    /// 🔚️ [`Self::commit_finished_replay`] over a slot that keeps the replay with the caller until the supersession takes it.
    async fn commit_slotted_replay(&mut self, slot: &mut Option<EditReplayResult<P, Mutation>>, finalization: HistoryFinalization, identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError>
    where
        P: Sync,
    {
        self.ensure_durable_group_idle()?;
        if let Some(open) = self.log.envelope.open_transaction.as_ref() {
            return Err(VcsError::TransactionOpen { transaction_id: open.transaction.id.clone() });
        }
        if self.local_step_pending() {
            return Err(VcsError::HistoryReplaying);
        }
        let finished = slot.as_ref().expect("a committing replay stays in its slot until its supersession takes it");
        if finished.revision != self.cursor.content_revision {
            return Err(VcsError::Stale { expected_generation: finished.generation, generation: self.cursor.generation });
        }
        if finished.drafts.is_empty() {
            return Err(VcsError::ValidationFailed("a finished replay without drafts supersedes nothing".into()));
        }
        let report = self.replay_report(finished)?;
        if report.blocks_finalize() {
            return Err(VcsError::Rejected { policy: semio_framework_os_spr::MergePolicy::Normal, messages: report.outcomes.into_iter().flat_map(|outcome| outcome.messages).collect() });
        }
        let (inputs, target) = self.draft_inputs(&finished.drafts)?;
        let scope = match finalization {
            HistoryFinalization::Overwrite => SupersedeScope::Document,
            HistoryFinalization::Scope { alternative_id } if alternative_id == self.trunk_alternative_id() || self.log.envelope.vcs.alternatives.iter().any(|known| known.id == alternative_id) => SupersedeScope::Alternative(alternative_id),
            HistoryFinalization::Scope { alternative_id } => return Err(VcsError::UnknownAlternative(alternative_id)),
            HistoryFinalization::Alternative { name } => SupersedeScope::NewAlternative(name),
        };
        self.replace_pending_report_retained(PendingCommandReport::default())?;
        self.author_supersession(scope, inputs, target, slot.take(), identity).await?;
        self.cursor.last_projection_cause = Some(ArtifactProjectionCause::Replay);
        self.flush_outbound().await?;
        Ok(CommandReceipt { edit_ids: Vec::new(), generation: self.generation(), messages: std::mem::take(&mut self.replay.pending_report.messages), worst: self.replay.pending_report.worst.take() })
    }

    /// ✍️ The one authoring path of every supersession: builds its transitions (`Document`/`Alternative`: one `Supersede`;
    /// `NewAlternative`: the pending-edit commit, a `Branch` at the head and the scoped `Supersede`), takes `replayed` when it
    /// is the Report replay of exactly the resulting log and dry-runs it otherwise (under [`Self::defer_local_replays`] the
    /// dry run is a local step stepped across turns), refuses a report that blocks finalizing (`Error`/`Fatal`, the hub
    /// check-in strictness) with `Rejected { policy: Normal }` and every replay message, and installs everything atomically
    /// without replaying again.
    async fn author_supersession(&mut self, scope: SupersedeScope, inputs: Vec<protocol::SupersededInput>, target: Vec<String>, replayed: Option<EditReplayResult<P, Mutation>>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let mut slot = replayed;
        let outcome = self.author_slotted_supersession(scope, inputs, target, &mut slot, identity).await;
        if let Some(unused) = slot {
            let _ = self.retire_replay_result(unused);
        }
        outcome
    }

    /// ✍️ [`Self::author_supersession`] over a slot that keeps `replayed` with the caller until the resulting log takes it.
    async fn author_slotted_supersession(&mut self, scope: SupersedeScope, inputs: Vec<protocol::SupersededInput>, target: Vec<String>, replayed: &mut Option<EditReplayResult<P, Mutation>>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let mut clock = self.log.clock;
        let mut envelopes = Vec::with_capacity(3);
        let saved_line = self.log.envelope.active_alternative_id.clone();
        let saved_checkpoint = self.log.envelope.viewer_checkpoint_id.clone();
        let mut viewing_new = false;
        let scope = match scope {
            SupersedeScope::Document => None,
            SupersedeScope::Alternative(alternative_id) => Some(alternative_id),
            SupersedeScope::NewAlternative(name) => match self.author_head_alternative(name, &mut clock, &mut envelopes, identity).await {
                Ok(alternative_id) => {
                    self.log.envelope.active_alternative_id = Some(alternative_id.clone());
                    self.log.envelope.viewer_checkpoint_id = None;
                    viewing_new = true;
                    Some(alternative_id)
                }
                Err(error) => return Err(error),
            },
        };
        let restore_head = |store: &mut Self| {
            if viewing_new {
                store.log.envelope.active_alternative_id = saved_line.clone();
                store.log.envelope.viewer_checkpoint_id = saved_checkpoint.clone();
            }
        };
        let supersede = protocol::TransitionSupersede { scope, inputs };
        if let Err(error) = supersede.validate() {
            restore_head(self);
            return Err(VcsError::ValidationFailed(error.to_string()));
        }
        let dependencies = supersede.targets();
        clock.tick(now_ms());
        envelopes.push(self.transition_envelope(&semio_framework_os_spr::HistoryTransition::Supersede(supersede), dependencies, target, clock));
        let fold = match self.prospective_fold(&envelopes) {
            Ok(fold) => fold,
            Err(error) => {
                restore_head(self);
                return Err(error);
            }
        };
        let result = match replayed.take() {
            Some(result) if result.order == fold.applied && same_effective_inputs(&result.supersessions, &fold.supersessions) => result,
            other if self.replay.local_replay_budget.is_some() => {
                *replayed = other;
                drop(fold);
                let head = self.viewer_head_now();
                restore_head(self);
                return self.defer_local_step(PendingLocalStep { envelopes, head, finalizing: true }, clock).await.map(|_| ());
            }
            other => {
                *replayed = other;
                match self.dry_run(&fold) {
                    Ok(result) => result,
                    Err(error) => {
                        restore_head(self);
                        return Err(error);
                    }
                }
            }
        };
        let report = match self.replay_report(&result) {
            Ok(report) => report,
            Err(error) => {
                let _ = self.retire_replay_result(result);
                restore_head(self);
                return Err(error);
            }
        };
        if report.blocks_finalize() {
            let _ = self.retire_replay_result(result);
            restore_head(self);
            return Err(VcsError::Rejected { policy: semio_framework_os_spr::MergePolicy::Normal, messages: report.outcomes.iter().flat_map(|outcome| outcome.messages.iter().cloned()).collect() });
        }
        if let Err(error) = self.install_transitions(envelopes, clock, Some(result)).await {
            restore_head(self);
            return Err(error);
        }
        self.record_replay_report(&report);
        Ok(())
    }

    //#endregion 🔖️Supersede
    //#region 🔖️HistoryLane
    /// ⏪️ Shared body of `Undo`/`UndoWithPolicy`: picks the edit the policy names and reverts it.
    async fn undo_with_policy(&mut self, policy: UndoPolicy, semantic_command: Option<Box<ArtifactCommand<Mutation>>>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError>
    where
        P: Sync,
    {
        match policy {
            UndoPolicy::ExactBaseOnly => {
                let position = self.log.applied_edit_ids.iter().rposition(|id| self.edit_lane(id) == HistoryLane::Document).ok_or(VcsError::NothingToUndo)?;
                self.undo_lane_position(position).await
            }
            UndoPolicy::TransformAgainstConcurrent => {
                let position = self.log.applied_edit_ids.iter().rposition(|id| self.edit_is_local(id) && self.edit_lane(id) == HistoryLane::Document).ok_or(VcsError::NothingToUndo)?;
                self.undo_lane_position(position).await
            }
            UndoPolicy::SemanticUndo | UndoPolicy::CompensatingAction => {
                let command = semantic_command.ok_or_else(|| VcsError::Backbone("semantic undo requires compensating command".into()))?;
                Box::pin(self.dispatch_inner(*command, identity)).await
            }
        }
    }

    /// ⏪️ Shared tail of every undo path: the applied edit at `position` must be this actor's
    /// own; it is withdrawn by a `Revert` transition and the projection follows the event log.
    async fn undo_lane_position(&mut self, position: usize) -> Result<(), VcsError> {
        let edit_id = self.log.applied_edit_ids[position].clone();
        if !self.edit_is_local(&edit_id) {
            return Err(VcsError::ForeignEdit(edit_id));
        }
        let mutation_ids = self.edit_mutation_ids(&edit_id)?;
        self.commit_transition(semio_framework_os_spr::HistoryTransition::Revert { mutation_ids: mutation_ids.clone() }, mutation_ids).await
    }

    /// ⏩️ Shared tail of every redo path: the redo entry at `position` is restored by a
    /// `Reinstate` transition, landing back at its own HLC position in the applied order.
    async fn redo_lane_position(&mut self, position: usize) -> Result<(), VcsError> {
        let edit_id = self.log.redo_edit_ids[position].clone();
        let mutation_ids = self.edit_mutation_ids(&edit_id)?;
        self.commit_transition(semio_framework_os_spr::HistoryTransition::Reinstate { mutation_ids: mutation_ids.clone() }, mutation_ids).await
    }

    /// 🚩️ Commits every applied-but-uncommitted edit as one change on a new checkpoint.
    async fn commit_pending_checkpoint(&mut self, message: Option<String>, authors: Vec<Author>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let (transition, dependencies, _) = self.pending_checkpoint_transition(message, authors, identity).await?;
        self.commit_transition(transition, dependencies).await
    }

    //#endregion 🔖️HistoryLane
    /// 🕹️ Parses `command_text` via [`parse_command`] and dispatches it — the op-line
    /// textual entry point (op-efficient one-line-per-structural-field commands, indented op
    /// lines for `Apply`/`AppendTransaction`).
    pub async fn dispatch_text(&mut self, command_text: &str, identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError>
    where
        Mutation: OpText,
        P: Sync,
    {
        let command = parse_command::<P, Mutation>(command_text).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
        self.dispatch(command, identity).await
    }

    /// 🕹️ Decodes `command_bytes` via [`decode_command`] and dispatches it — the binary
    /// entry point used for both communication (backbone/semio_hub) and storage (`.spr`).
    pub async fn dispatch_binary(&mut self, command_bytes: &[u8], identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError>
    where
        Mutation: OpBinary,
        P: Sync,
    {
        let command = ArtifactCommand::<Mutation>::decode_command::<P>(command_bytes).map_err(|error| VcsError::Deserialize(error.to_string()))?;
        self.dispatch(command, identity).await
    }

    /// 🔗️ Attaches a backbone channel: ingests whatever events the other end already queued,
    /// then announces this replica's genesis and its complete event log. The other end folds the
    /// events it lacks and ignores the ones it holds — no snapshot ever crosses the channel.
    pub async fn attach_backbone(&mut self, backbone: Backbones) -> Result<(), VcsError> {
        self.ensure_durable_group_idle()?;
        self.log.envelope.backbone = Some(backbone.descriptor().await);
        self.replace_backbone_retained(Some(backbone))?;
        self.seed_known_events()?;
        self.pump().await?;
        self.announce_history().await?;
        self.bump()?;
        Ok(())
    }

    /// 📥️ Folds an encoded envelope payload a composing parent routed to THIS member's lane.
    /// Inbound twin of {@link announce_tail_edit_payload}; each envelope goes through the same
    /// {@link ingest_remote} gate a parent-lane message does, so causal order and conflict
    /// quarantining are identical on both lanes.
    pub async fn ingest_remote_payload(&mut self, envelopes: &[u8]) -> Result<Vec<semio_framework_os_spr::MergeReport>, VcsError> {
        let envelopes = semio_framework_os_spr::decode_envelopes(envelopes).map_err(|error| VcsError::Deserialize(error.to_string()))?;
        let mut reports = Vec::with_capacity(envelopes.len());
        for envelope in envelopes {
            reports.push(self.ingest_remote(envelope).await?);
        }
        Ok(reports)
    }

    pub async fn attach_hot_backbone(&mut self, backbone: Backbones) -> Result<(), VcsError> {
        self.ensure_durable_group_idle()?;
        self.log.envelope.backbone = Some(backbone.descriptor().await);
        self.replace_backbone_retained(Some(backbone))?;
        self.pump().await?;
        self.bump()?;
        Ok(())
    }

    /// 🔗️ Resolves a backbone URI and attaches it. Only available inside the wasm sandbox,
    /// where every scheme forwards to the host over the injected {@link BackboneChannelPort} (a pure
    /// queue). On native targets, callers attach an explicit `Backbones` value via
    /// {@link attach_backbone} — the `framework/sync` actor layer owns all IO-performing endpoints.
    #[cfg(target_arch = "wasm32")]
    pub async fn attach_backbone_uri(&mut self, uri: &str) -> Result<(), VcsError> {
        self.attach_backbone(resolve_backbone(uri).await?).await
    }

    /// 📡️ Drains inbound backbone messages into the edit timeline. Safe to call anytime;
    /// `dispatch` already calls this before every command.
    pub async fn tick(&mut self) -> Result<bool, VcsError> {
        self.ensure_durable_group_idle()?;
        let pumped = self.pump().await?;
        let pending = self.replay.pending_reprojection.is_some();
        let progress = self.step_reprojection(None).await?;
        Ok(pumped || (pending && progress.is_none()))
    }

    pub async fn tick_backbone_reports(&mut self) -> Result<Vec<semio_framework_os_spr::MergeReport>, VcsError> {
        self.ensure_durable_group_idle()?;
        self.pump_with_reports().await.map(|(_, reports)| reports)
    }

    /// 🛟️ The sole public remote write gate ([`Self::ingest_remote_merge`]). While a tool transaction is open its
    /// unannounced edit is lifted off the applied tail for the merge and folded back onto the merged head after it, so a
    /// remote edit never reorders, quarantines or degrades it and it stays the tail its commit announces.
    pub async fn ingest_remote(&mut self, envelope: semio_framework_os_spr::MutationEnvelope) -> Result<semio_framework_os_spr::MergeReport, VcsError> {
        let Some((open, edit)) = self.lift_open_transaction().await? else {
            return self.ingest_remote_merge(envelope).await;
        };
        let report = self.ingest_remote_merge(envelope).await;
        self.restore_open_transaction(open, edit).await?;
        report
    }

    /// 🕸️ Feeds a remote {@link MutationEnvelope} through the causal DAG, applying it (and any
    /// now-unblocked dependents) into the edit timeline. Closes the sync gap between
    /// `framework/sync`'s `MutationDag` and the vcs edit history. Sole public remote write gate —
    /// parallel to `dispatch` for causal envelopes. Implements the ticket's 9-step algorithm
    /// (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6): buffer in the
    /// dag, drain whatever it (and anything it unblocks) makes ready, HLC-sort that batch into
    /// `applied_edit_ids`, replay only the divergent suffix, then either commit atomically (dag,
    /// history, ledger, `applied_edit_ids` all move together) or quarantine the whole batch as an
    /// `Open` `Conflict` — state and the dag's own applied-set never move on rejection.
    ///
    /// 1
    ///
    /// 2
    ///
    /// 3 — an edit's HLC is its first forward op's stamped meta timestamp.
    ///
    /// 🌀️ `HybridLogicalTimestamp::cmp_key` is async (📡️replication); `sort_by_key`/
    /// `partition_point`/`position` all need sync predicates (R10 shape 1), so every key below
    /// is resolved via an explicit `.await` first, then compared as a plain `(u64, u64, u64)`.
    ///
    /// Binary search replicating `partition_point`'s algorithm (assumes `applied_edit_ids` is
    /// already HLC-sorted, exactly as `partition_point` itself would have assumed).
    ///
    /// 4 — stable HLC merge of `batch` into `applied_edit_ids[k..]`.
    ///
    /// 5
    ///
    /// 6 — walk order[k..] one edit at a time (H1 determinism fix): an already-committed edit
    /// that this rewind proves invalid gets the SAME accept/quarantine verdict a fresh arrival
    /// in true HLC order would have given it, instead of the whole suffix being judged
    /// atomically. See `replay_suffix_partitioned`'s doc for why this makes the merge a pure
    /// function of the envelope SET and `self.merge_policy`, never of arrival order.
    ///
    /// 7
    ///
    /// 🎯️ MEDIUM-3: `degraded_ids` only needs `committed_ids`/`replayed` (both already computed
    /// above, before any mutation) — hoisted from below `# 9` so the open-conflict capacity check
    /// can see BOTH conflicts this call might mint before touching any store field, keeping the
    /// whole call atomic on a refusal exactly like a policy rejection already is.
    /// 🌀️ `worst_level` is async; `Iterator::filter`'s closure is sync (R10 shape 1), so it's
    /// hoisted into an explicit loop.
    ///
    /// 8 — quarantine: every edit whose OWN outcome the policy rejects — whether newly-arrived
    /// this call or retroactively invalidated by this rewind — is pulled out of history. Its
    /// forward ops/`MutationMeta` are left untouched wherever they already live in `vcs.edits`
    /// (never rewritten, exactly like `Undo`); only its id drops out of `applied_edit_ids`, so
    /// a future `resolve_conflict`/redelivery still finds its established payload.
    ///
    /// 🎯️ HIGH-2: `edits_for_ids` errors loudly instead of silently filtering — see its doc.
    ///
    /// 🌀️ The unawaited future would borrow `envelopes`, which is moved into `kind` below —
    /// awaited immediately instead of deferred to avoid a move-while-borrowed (E0505).
    ///
    /// 🌀️ `mutation_ids_for_edit` is async (📡️replication); `flat_map`'s closure is sync
    /// (R10 shape 1), so it's hoisted into an explicit loop.
    ///
    /// 🎯️ H1: `hlc`/`timestamp` come from the quarantined edits' OWN stamped HLCs, never
    /// `self.clock` — `self.clock.merge` ticks its `logical` counter on every call, so its
    /// running value depends on how many merges happened and in what order, not just on
    /// this envelope set; hashing that into `ConflictId` reintroduced arrival-order
    /// dependence even after state/`applied_edit_ids` converged.
    /// 🌀️ `HybridLogicalTimestamp::cmp_key` is async; `max_by_key` needs a sync key extractor
    /// (R10 shape 1), so the max is found by an explicit fold over resolved keys instead.
    ///
    /// `>=` (not `>`): `Iterator::max_by_key`'s documented tie-break keeps the LAST
    /// equally-maximum element, so ties must still overwrite.
    ///
    /// 9 — commit the survivors: history, ledger, `applied_edit_ids`, `current` together. The
    /// dag (decided right after the capacity check above) only advanced when nothing in this
    /// suffix was quarantined, so a quarantined-but-not-yet-committed envelope can still be
    /// retried on redelivery.
    ///
    /// 🎯️ HIGH-2: `edits_for_ids` errors loudly instead of silently filtering — see its doc.
    /// Reads from `edits_by_id` (not `self.envelope.vcs.edits`): both already contain every
    /// `degraded_ids` entry at this point, but `edits_by_id` doesn't depend on the `vcs.edits`
    /// push above having already run, so this block's own correctness never depends on
    /// sequencing against that mutation.
    ///
    /// 🌀️ `mutation_ids_for_edit` is async (📡️replication); `flat_map`'s closure is sync
    /// (R10 shape 1), so it's hoisted into an explicit loop.
    ///
    /// 🎯️ H1: same reasoning as the quarantine conflict above — derive from the degraded
    /// edits' own HLCs, not the arrival-order-dependent `self.clock`.
    /// 🌀️ `HybridLogicalTimestamp::cmp_key` is async; `max_by_key` needs a sync key extractor
    /// (R10 shape 1), so the max is found by an explicit fold over resolved keys instead.
    ///
    /// `>=`: `Iterator::max_by_key` keeps the LAST equally-maximum element on a tie.
    async fn ingest_remote_merge(&mut self, envelope: semio_framework_os_spr::MutationEnvelope) -> Result<semio_framework_os_spr::MergeReport, VcsError> {
        self.ensure_durable_group_idle()?;
        let no_op_report = |policy: semio_framework_os_spr::MergePolicy, insertion_index: usize| semio_framework_os_spr::MergeReport { policy, accepted: true, insertion_index: insertion_index as u32, replayed: Vec::new(), worst: None, conflict: None };
        self.owners.displaced_retirements.reserve(2)?;
        let mut candidate_dag = (*self.log.dag).clone();
        let insertion = match candidate_dag.insert(envelope.clone()) {
            Ok(insertion) => insertion,
            Err(rejected) => {
                self.owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreConflictRetirement::causal_envelope(rejected.envelope)));
                self.owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreMutationDagRetirement::new(candidate_dag)));
                return Err(VcsError::Backbone(rejected.error.to_string()));
            }
        };
        if let semio_framework_os_spr::InsertResult::AlreadyApplied(rejected) = insertion {
            let equivalent = self.assert_equivalent_remote_mutation(&envelope);
            self.owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreConflictRetirement::causal_envelope(rejected)));
            self.owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreMutationDagRetirement::new(candidate_dag)));
            equivalent?;
            return Ok(no_op_report(self.log.merge_policy, self.log.applied_edit_ids.len()));
        }
        let mut batch: Vec<Edit<Mutation>> = Vec::new();
        let mut ready_transitions: Vec<semio_framework_os_spr::MutationEnvelope> = Vec::new();
        loop {
            let ready_envelope = match candidate_dag.take_next_applied() {
                semio_framework_os_spr::MutationDagAppliedStep::Envelope(envelope) => envelope,
                semio_framework_os_spr::MutationDagAppliedStep::SeededIdentity => continue,
                semio_framework_os_spr::MutationDagAppliedStep::Complete => break,
            };
            if semio_framework_os_spr::is_history_transition(&ready_envelope) {
                ready_transitions.push(ready_envelope);
                continue;
            }
            let admitted = match edit_from_operation_envelope::<Mutation>(&ready_envelope).await {
                Ok(mut edit) => {
                    edit.actor = Some(ready_envelope.actor.0.clone());
                    stamp_edit_semantics::<P, Mutation>(&mut edit, &self.log.envelope.schema);
                    match self.log.envelope.vcs.edits.find_near(|existing| existing.id == edit.id) {
                        Some(existing) => {
                            let verdict = self.assert_equivalent_remote_envelope(existing, &ready_envelope).map(|()| None);
                            retire_scratch_edits::<P, Mutation>([edit]);
                            verdict
                        }
                        None if batch.iter().any(|existing: &Edit<Mutation>| existing.id == edit.id) => {
                            let error = VcsError::ValidationFailed(format!("remote ingest repeats authoritative edit {}", edit.id));
                            retire_scratch_edits::<P, Mutation>([edit]);
                            Err(error)
                        }
                        None => Ok(Some(edit)),
                    }
                }
                Err(error) => Err(error),
            };
            let edit = match admitted {
                Ok(Some(edit)) => edit,
                Ok(None) => continue,
                Err(error) => {
                    retire_scratch_edits::<P, Mutation>(batch);
                    return self.refuse_with_candidate_dag(candidate_dag, error);
                }
            };
            self.log.clock.merge(&ready_envelope.timestamp);
            batch.push(edit);
        }
        if batch.is_empty() {
            if ready_transitions.is_empty() {
                self.replace_causal_dag_retained(candidate_dag)?;
                return Ok(no_op_report(self.log.merge_policy, self.log.applied_edit_ids.len()));
            }
            self.cursor.last_projection_cause = None;
            let replayed = match self.admit_remote_transitions(ready_transitions).await {
                Ok(replayed) => replayed,
                Err(error) => return self.refuse_with_candidate_dag(candidate_dag, error),
            };
            self.replace_causal_dag_retained(candidate_dag)?;
            self.replay.pending_report.edit_ids = Some(Vec::new());
            self.bump()?;
            self.cursor.last_projection_cause = Some(self.cursor.last_projection_cause.filter(|cause| *cause == ArtifactProjectionCause::Replay).unwrap_or(ArtifactProjectionCause::RemoteIngest));
            let worst = replayed.iter().flat_map(|entry| entry.messages.iter()).map(|message| message.level).max();
            return Ok(semio_framework_os_spr::MergeReport { policy: self.log.merge_policy, accepted: true, insertion_index: self.log.applied_edit_ids.len() as u32, replayed, worst, conflict: None });
        }
        let edit_hlc = |edit: &Edit<Mutation>| edit.mutation_meta.first().map_or_else(|| HybridLogicalTimestamp { actor: 0, physical_ms: 0, logical: 0 }, |meta| meta.timestamp);
        batch.sort_by_key(|edit| edit_hlc(edit).cmp_key());
        let batch_keys: Vec<(u64, u64, u64)> = batch.iter().map(|edit| edit_hlc(edit).cmp_key()).collect();
        let known_hlc = |edit_id: &str, edits: &ArtifactHistoryLedger<Edit<Mutation>>| edits.find_near(|edit| edit.id == *edit_id).map(edit_hlc);
        let min_batch_key = batch_keys[0];
        let mut lo = 0usize;
        let mut hi = self.log.applied_edit_ids.len();
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let candidate_key = known_hlc(&self.log.applied_edit_ids[mid], &self.log.envelope.vcs.edits).map(|hlc| hlc.cmp_key());
            let still_before = candidate_key.is_none_or(|key| key < min_batch_key);
            if still_before {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        let mut k = lo;
        for edit in &batch {
            for dependency in edit.mutation_meta.first().map(|meta| meta.dependencies.clone()).unwrap_or_default() {
                if let Some(position) = self.log.applied_edit_ids.iter().position(|id| *id == dependency.0) {
                    k = k.max(position + 1);
                }
            }
        }
        let mut order: Vec<String> = self.log.applied_edit_ids.to_vec();
        for (edit, &hlc_key) in batch.iter().zip(batch_keys.iter()) {
            let mut insert_at = order.len();
            for offset in k..order.len() {
                let existing_key = known_hlc(&order[offset], &self.log.envelope.vcs.edits).map(|existing| existing.cmp_key());
                if existing_key.is_some_and(|key| key > hlc_key) {
                    insert_at = offset;
                    break;
                }
            }
            order.insert(insert_at, edit.id.clone());
        }
        let base = if k == self.log.applied_edit_ids.len() {
            Arc::clone(&*self.live.current)
        } else {
            match Self::fold_history(&self.log.envelope, &order[..k], &self.live.supersessions).await {
                Ok(base) => Arc::new(base),
                Err(error) => {
                    retire_scratch_edits::<P, Mutation>(batch);
                    return self.refuse_with_candidate_dag(candidate_dag, error);
                }
            }
        };
        let mut edits_by_id: HashMap<String, Edit<Mutation>> = self.log.envelope.vcs.edits.iter().map(|edit| (edit.id.clone(), edit.clone())).collect();
        for edit in &batch {
            edits_by_id.insert(edit.id.clone(), edit.clone());
        }
        let replay = StoreReplay::<P, Mutation>::replay_suffix_partitioned(base, &order, k, &edits_by_id, &self.log.envelope.schema, &self.live.supersessions, self.log.merge_policy);
        let (state, committed_ids, quarantined_ids, rebased_inverse, replayed) = match replay {
            Ok(mut replay) => {
                let taken = (replay.take_state().expect("a finished merge replay holds its projection"), replay.take_committed(), replay.take_quarantined(), replay.take_rebased_undo_rows(), replay.take_replayed());
                let _ = self.retire_replay_result(replay);
                taken
            }
            Err(error) => {
                retire_scratch_edits::<P, Mutation>(batch);
                retire_scratch_edits::<P, Mutation>(edits_by_id.into_values());
                return self.refuse_with_candidate_dag(candidate_dag, error);
            }
        };
        let document_id = ArtifactId(self.log.envelope.id.clone());
        let schema = SchemaId(self.log.envelope.schema.clone());
        let mut degraded_ids: Vec<String> = Vec::new();
        for id in &committed_ids {
            let is_degraded = match replayed.iter().find(|entry| &entry.edit_id == id) {
                Some(entry) => semio_framework_os_spr::worst_level(&entry.messages).is_some_and(|level| level >= semio_framework_diagnostic::Severity::Warning),
                None => false,
            };
            if is_degraded {
                degraded_ids.push(id.clone());
            }
        }
        if let Err(error) = self.ensure_open_conflict_capacity(usize::from(!quarantined_ids.is_empty()) + usize::from(!degraded_ids.is_empty())) {
            retire_shared_projection::<P, Mutation>(state);
            retire_scratch_edits::<P, Mutation>(batch);
            retire_scratch_edits::<P, Mutation>(edits_by_id.into_values());
            for (_, inverse) in rebased_inverse {
                retire_scratch_operations::<P, Mutation>(inverse);
            }
            return self.refuse_with_candidate_dag(candidate_dag, error);
        }
        if quarantined_ids.is_empty() {
            self.replace_causal_dag_retained(candidate_dag)?;
        } else {
            self.owners.displaced_retirements.reserve(1)?;
            self.owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreMutationDagRetirement::new(candidate_dag)));
        }
        let mut quarantine_conflict_id = None;
        if !quarantined_ids.is_empty() {
            let quarantined_edits = Self::edits_for_ids(&quarantined_ids, &edits_by_id)?;
            let mut envelopes: Vec<semio_framework_os_spr::MutationEnvelope> = Vec::new();
            for edit in &quarantined_edits {
                envelopes.extend(semio_framework_os_spr::mutation_envelope_from_edit::<P, Mutation>(edit, &document_id, &schema).map_err(|error| VcsError::Serialize(error.to_string()))?);
            }
            let conflict_actors = canonical_conflict_actors(envelopes.iter().map(|envelope| envelope.actor.clone())).await;
            let mut quarantine_mutation_ids: Vec<MutationId> = Vec::new();
            for edit in &quarantined_edits {
                quarantine_mutation_ids.extend(semio_framework_os_spr::mutation_ids_for_edit(edit));
            }
            let quarantine_messages = conflict_messages_for_edits(&quarantined_edits, &replayed).await?;
            let kind = semio_framework_os_spr::ConflictKind::Quarantined { envelopes };
            let mut conflict_hlc = self.log.clock;
            let mut conflict_hlc_key: Option<(u64, u64, u64)> = None;
            for edit in &quarantined_edits {
                let hlc = edit_hlc(edit);
                let key = hlc.cmp_key();
                if conflict_hlc_key.is_none_or(|current| key >= current) {
                    conflict_hlc_key = Some(key);
                    conflict_hlc = hlc;
                }
            }
            let id = semio_framework_os_spr::ConflictId::new(&kind, &ArtifactId(self.log.envelope.id.clone()), &quarantine_mutation_ids, &conflict_hlc).await;
            self.log.envelope.conflicts.push(semio_framework_os_spr::Conflict { id: id.clone(), kind, status: semio_framework_os_spr::ConflictStatus::Open, messages: quarantine_messages, actors: conflict_actors, timestamp: conflict_hlc });
            self.prune_resolved_conflicts()?;
            for quarantined_id in &quarantined_ids {
                self.replace_edit_messages(quarantined_id, Vec::new())?;
            }
            retire_scratch_edits::<P, Mutation>(quarantined_edits);
            quarantine_conflict_id = Some(id);
        }
        for edit in &batch {
            if committed_ids.contains(&edit.id) && !self.log.envelope.vcs.edits.iter().any(|existing| existing.id == edit.id) {
                let reservation = self.reserve_edit_history_slot()?;
                self.insert_reserved_edit_history(reservation, edit.clone())?;
            }
        }
        for (edit_id, inverse) in rebased_inverse {
            match self.log.envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id) {
                Some(edit) => retire_scratch_operations::<P, Mutation>(std::mem::replace(&mut edit.inverse, inverse)),
                None => retire_scratch_operations::<P, Mutation>(inverse),
            }
        }
        self.replace_applied_edit_ids_retained(self.log.applied_edit_ids.iter().take(k).cloned().chain(committed_ids.iter().cloned()).collect())?;
        for edit_messages in &replayed {
            if committed_ids.contains(&edit_messages.edit_id) {
                self.replace_edit_messages(&edit_messages.edit_id, edit_messages.messages.clone())?;
            }
        }
        let positions: HashMap<&str, usize> = self.log.applied_edit_ids.iter().enumerate().map(|(index, edit_id)| (edit_id.as_str(), index)).collect();
        for edit in self.log.envelope.vcs.edits.iter_mut() {
            if let Some(index) = positions.get(edit.id.as_str()) {
                edit.sequence_number = *index as i32 + 1;
            }
        }
        drop(positions);
        self.log.edit_sequence = self.log.applied_edit_ids.len() as i32;
        self.replace_tail_undo_cache_retained(None)?;
        self.replace_current_retained(state)?;
        self.cursor.revision_dirty_from = Some(self.cursor.revision_dirty_from.map_or(k, |dirty| dirty.min(k)));
        let mut degraded_conflict_id = None;
        if !degraded_ids.is_empty() {
            let degraded_edits = Self::edits_for_ids(&degraded_ids, &edits_by_id)?;
            let degraded_messages = conflict_messages_for_edits(&degraded_edits, &replayed).await?;
            let degraded_actors = canonical_conflict_actors(degraded_edits.iter().filter_map(|edit| edit.actor.clone()).map(ActorId));
            let mut degraded_mutation_ids: Vec<MutationId> = Vec::new();
            for edit in &degraded_edits {
                degraded_mutation_ids.extend(semio_framework_os_spr::mutation_ids_for_edit(edit));
            }
            let kind = semio_framework_os_spr::ConflictKind::Degraded { edit_ids: degraded_ids.clone() };
            let mut conflict_hlc = self.log.clock;
            let mut conflict_hlc_key: Option<(u64, u64, u64)> = None;
            for edit in &degraded_edits {
                let hlc = edit_hlc(edit);
                let key = hlc.cmp_key();
                if conflict_hlc_key.is_none_or(|current| key >= current) {
                    conflict_hlc_key = Some(key);
                    conflict_hlc = hlc;
                }
            }
            let id = semio_framework_os_spr::ConflictId::new(&kind, &ArtifactId(self.log.envelope.id.clone()), &degraded_mutation_ids, &conflict_hlc).await;
            self.log.envelope.conflicts.push(semio_framework_os_spr::Conflict { id: id.clone(), kind, status: semio_framework_os_spr::ConflictStatus::Open, messages: degraded_messages, actors: degraded_actors.await, timestamp: conflict_hlc });
            self.prune_resolved_conflicts()?;
            retire_scratch_edits::<P, Mutation>(degraded_edits);
            degraded_conflict_id = Some(id);
        }
        self.cursor.last_projection_cause = None;
        let mut replayed = replayed;
        if !ready_transitions.is_empty() || !self.log.envelope.transitions.is_empty() {
            replayed.extend(self.admit_remote_transitions(ready_transitions).await?);
        }
        let worst = replayed.iter().flat_map(|edit_messages| edit_messages.messages.iter()).map(|message| message.level).max();
        self.replay.pending_report.edit_ids = Some(batch.iter().filter(|edit| committed_ids.contains(&edit.id)).map(|edit| edit.id.clone()).collect());
        self.bump()?;
        self.cursor.last_projection_cause = Some(self.cursor.last_projection_cause.filter(|cause| *cause == ArtifactProjectionCause::Replay).unwrap_or(ArtifactProjectionCause::RemoteIngest));
        let accepted = batch.iter().all(|edit| committed_ids.contains(&edit.id));
        retire_scratch_edits::<P, Mutation>(batch);
        retire_scratch_edits::<P, Mutation>(edits_by_id.into_values());
        Ok(semio_framework_os_spr::MergeReport { policy: self.log.merge_policy, accepted, insertion_index: k as u32, replayed, worst, conflict: quarantine_conflict_id.or(degraded_conflict_id) })
    }

    /// 📥️ Pumps every queued inbound message from the attached backbone into the timeline.
    pub async fn pump(&mut self) -> Result<bool, VcsError> {
        self.pump_with_reports().await.map(|(changed, _)| changed)
    }

    /// 🪆️ A member lane names a document this store does not own; only the composing app can
    /// resolve `(slot, child_id)` to a live member, so the message waits for its owner.
    ///
    /// A store never consumes acks (they flow store→actor); drain and ignore any that echo back.
    async fn pump_with_reports(&mut self) -> Result<(bool, Vec<semio_framework_os_spr::MergeReport>), VcsError> {
        let Some(mut backbone) = self.wire.backbone.take() else {
            return Ok((false, Vec::new()));
        };
        let received = backbone.receive().await;
        self.replace_backbone_retained(Some(backbone))?;
        let messages = received?;
        if messages.is_empty() {
            return Ok((false, Vec::new()));
        }
        let mut acked_op_ids: Vec<String> = Vec::new();
        let mut reports = Vec::new();
        for message in messages {
            match message {
                BackboneMessage::Genesis { pack } => self.verify_genesis(&pack)?,
                BackboneMessage::Mutations { envelopes } => {
                    let envelopes = semio_framework_os_spr::decode_envelopes(&envelopes).map_err(|error| VcsError::Deserialize(error.to_string()))?;
                    let op_ids: Vec<String> = envelopes.iter().map(|envelope| envelope.mutation_id.0.clone()).collect();
                    for envelope in envelopes {
                        reports.push(self.ingest_remote(envelope).await?);
                    }
                    acked_op_ids.extend(op_ids);
                }
                member @ BackboneMessage::Member { .. } => self.wire.member_inbox.push_back(member),
                BackboneMessage::Ack { .. } => {}
                BackboneMessage::Retract { mutation_ids } => {
                    self.retract_transitions(&mutation_ids).await?;
                }
            }
        }
        if !acked_op_ids.is_empty() {
            if let Some(mut backbone) = self.wire.backbone.take() {
                let result = backbone.send(BackboneMessage::Ack { op_ids: acked_op_ids }).await;
                self.replace_backbone_retained(Some(backbone))?;
                result?;
            }
        }
        Ok((true, reports))
    }
}

impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    /// 🪞️ Delegates to [`StoreWire::ensure_durable_group_idle`].
    #[inline] pub fn ensure_durable_group_idle(&self) -> Result<(), VcsError> { self.wire.ensure_durable_group_idle() }

    /// 🪞️ Delegates to [`StoreCursor::generation`].
    #[inline] pub fn generation(&self) -> u64 { self.cursor.generation(&self.wire) }

    /// 🪞️ Delegates to [`StoreWire::artifact_revision`].
    #[inline] pub fn artifact_revision(&self) -> ArtifactRevision { self.wire.artifact_revision(&self.log) }

    /// 🪞️ Delegates to [`StoreCursor::generation_now`].
    #[inline] pub fn generation_now(&self) -> u64 { self.cursor.generation_now(&self.wire) }

    /// 🪞️ Delegates to [`StoreLog::clock_now`].
    #[inline] pub fn clock_now(&self) -> HybridLogicalTimestamp { self.log.clock_now() }

    /// 🪞️ Delegates to [`StoreCursor::content_revision`].
    #[inline] pub fn content_revision(&self) -> [u8; 32] { self.cursor.content_revision(&self.wire) }

    /// 🪞️ Delegates to [`StoreCursor::content_revision_now`].
    #[inline] pub fn content_revision_now(&self) -> [u8; 32] { self.cursor.content_revision_now(&self.wire) }

    /// 🪞️ Delegates to [`StoreCursor::projection_stamp`].
    #[inline] pub fn projection_stamp(&self) -> ArtifactProjectionStamp { self.cursor.projection_stamp(&self.log, &self.wire) }

    /// 🪞️ Delegates to [`StoreLive::projection_event`].
    #[inline] pub fn projection_event<Previous, Policy>(&self, cause: ArtifactProjectionCause, previous: Option<Previous>, cache_mode: ArtifactProjectionCacheMode, policy: Policy) -> Result<ArtifactProjectionEvent<P, Previous, Policy>, VcsError> { self.live.projection_event::<Previous, Policy>(&self.log, &self.wire, &self.cursor, cause, previous, cache_mode, policy) }

    /// 🪞️ Delegates to [`StoreCursor::last_projection_invalidation`].
    #[inline] pub fn last_projection_invalidation(&self) -> Option<ArtifactProjectionInvalidation> { self.cursor.last_projection_invalidation(&self.log, &self.wire) }

    /// 🪞️ Delegates to [`StoreReplay::invalidate_after_replay`].
    #[inline] pub fn invalidate_after_replay(&mut self) -> Result<ArtifactProjectionInvalidation, VcsError> { self.replay.invalidate_after_replay(&mut self.log, &mut self.owners, &mut self.wire, &mut self.cursor, &mut self.live) }

    /// 🪞️ Delegates to [`StoreReplay::invalidate_after_policy_change`].
    #[inline] pub fn invalidate_after_policy_change(&mut self) -> Result<ArtifactProjectionInvalidation, VcsError> { self.replay.invalidate_after_policy_change(&mut self.log, &mut self.owners, &mut self.wire, &mut self.cursor, &mut self.live) }

    /// 🪞️ Delegates to [`StoreReplay::invalidate_after_external_resource_change`].
    #[inline] pub fn invalidate_after_external_resource_change(&mut self) -> Result<ArtifactProjectionInvalidation, VcsError> { self.replay.invalidate_after_external_resource_change(&mut self.log, &mut self.owners, &mut self.wire, &mut self.cursor, &mut self.live) }

    /// 🪞️ Delegates to [`StoreReplay::invalidate_projections`].
    #[inline] pub fn invalidate_projections(&mut self, cause: ArtifactProjectionCause) -> Result<ArtifactProjectionInvalidation, VcsError> { self.replay.invalidate_projections(&mut self.log, &mut self.owners, &mut self.wire, &mut self.cursor, &mut self.live, cause) }

    /// 🪞️ Delegates to [`StoreCursor::accept_projection_result`].
    #[inline] pub fn accept_projection_result<Output, Diff>(&self, result: ArtifactProjectionResult<Output, Diff>) -> Result<AcceptedArtifactProjection<Output, Diff>, StaleArtifactProjection> { self.cursor.accept_projection_result::<Output, Diff>(&self.log, &self.wire, result) }

    /// 🪞️ Delegates to [`StoreLog::envelope`].
    #[inline] pub fn envelope(&self) -> &ArtifactEnvelope<P, Mutation> { self.log.envelope() }

    /// 🪞️ Delegates to [`StoreLog::envelope_view`].
    #[inline] pub fn envelope_view(&self) -> ArtifactEnvelopeView<'_, P, Mutation> { self.log.envelope_view() }

    /// 🪞️ Delegates to [`StoreWire::applied_edit_ids`].
    #[inline] pub fn applied_edit_ids(&self) -> &semio_framework_os_vcs::HistoryPageStack<String> { self.wire.applied_edit_ids(&self.log) }

    /// 🪞️ Delegates to [`StoreWire::redo_edit_ids`].
    #[inline] pub fn redo_edit_ids(&self) -> &semio_framework_os_vcs::HistoryPageStack<String> { self.wire.redo_edit_ids(&self.log) }

    /// 🪞️ Delegates to [`StoreLog::current_checkpoint_id`].
    #[inline] pub fn current_checkpoint_id(&self) -> Option<&str> { self.log.current_checkpoint_id() }

    /// 🪞️ Delegates to [`StoreLog::local_actor_id`].
    #[inline] pub fn local_actor_id(&self) -> &ActorId { self.log.local_actor_id() }

    /// 🪞️ Delegates to [`StoreLog::authoring_verb`].
    #[inline] pub fn authoring_verb(&self) -> Option<&str> { self.log.authoring_verb() }

    /// 🪞️ Delegates to [`StoreLog::set_authoring_verb`].
    #[inline] pub fn set_authoring_verb(&mut self, verb: Option<String>) { self.log.set_authoring_verb(verb) }

    /// 🪞️ Delegates to [`StoreLog::open_transaction`].
    #[inline] pub fn open_transaction(&self) -> Option<&OpenToolTransaction> { self.log.open_transaction() }

    /// 🪞️ Delegates to [`StoreLog::edit_mutations`].
    #[inline] pub fn edit_mutations(&self) -> Option<(&[Mutation], &semio_framework_value::list::PagedList<Mutation, {usize::MAX}>, &[MutationMeta])> { self.log.edit_mutations() }

    /// 🪞️ Delegates to [`StoreLog::history_columns`].
    #[inline] pub async fn history_columns(&self) -> Vec<HistoryColumn> { self.log.history_columns().await }

    /// 🪞️ Delegates to [`StoreLive::snapshot`].
    #[inline] pub fn snapshot(&self) -> Result<P, VcsError> { self.live.snapshot(&self.wire) }

    /// 🪞️ Delegates to [`StoreLive::snapshot_ref`].
    #[inline] pub fn snapshot_ref(&self) -> &P { self.live.snapshot_ref(&self.wire) }

    /// 🪞️ Delegates to [`StoreLive::derived_snapshot_head`].
    #[inline] pub fn derived_snapshot_head(&self) -> ArtifactDerivedSnapshot<P> { self.live.derived_snapshot_head(&self.owners, &self.wire, &self.cursor) }

    /// 🪞️ Delegates to [`StoreCursor::snapshot_read_derived`].
    #[inline] pub fn snapshot_read_derived(&self, source: &ArtifactDerivedSnapshot<P>) -> Result<ErasedSnapshotRead, VcsError> where P: Sync, { self.cursor.snapshot_read_derived(&self.owners, &self.wire, source) }

    /// 🪞️ Delegates to [`StoreOwners::retained_derived_read`].
    #[inline] pub fn retained_derived_read(&self, source: &ArtifactDerivedSnapshot<P>) -> Result<ErasedSnapshotRead, ValueError> where P: Sync { self.owners.retained_derived_read(source) }

    /// 🪞️ Delegates to [`StoreCursor::derive_provisional_snapshot`].
    #[inline] pub fn derive_provisional_snapshot(&self, source: &ArtifactDerivedSnapshot<P>, op: &Mutation) -> Result<Option<ArtifactDerivedSnapshot<P>>, VcsError> { self.cursor.derive_provisional_snapshot(&self.owners, &self.wire, source, op) }

    /// 🪞️ Delegates to [`StoreLive::derived_snapshot_before`].
    #[inline] pub fn derived_snapshot_before(&mut self, target: &MutationId, drafts: &protocol::HistoryInputDrafts) -> Result<ArtifactDerivedSnapshot<P>, VcsError> { self.live.derived_snapshot_before(&mut self.log, &mut self.owners, &mut self.wire, &mut self.cursor, target, drafts) }

    /// 🪞️ Delegates to [`StoreCursor::derive_history_snapshot`].
    #[inline] pub fn derive_history_snapshot(&self, source: &ArtifactDerivedSnapshot<P>, op: &Mutation) -> Result<(Option<ArtifactDerivedSnapshot<P>>, Vec<protocol::MutationMessage>), VcsError> { self.cursor.derive_history_snapshot(&self.owners, &self.wire, source, op) }

    /// 🪞️ Delegates to [`StoreCursor::begin_derived_history_preview`].
    #[inline] pub fn begin_derived_history_preview(&self, target: MutationId, drafts: protocol::HistoryInputDrafts) -> Result<ArtifactDerivedHistoryPreview<P, Mutation>, VcsError> { self.cursor.begin_derived_history_preview(&self.log, &self.owners, &self.wire, target, drafts) }

    /// 🪞️ Delegates to [`StoreOwners::prepare_history_read_retirement`].
    #[inline] pub fn prepare_history_read_retirement(&self,grant:RetainedCloneGrant)->Result<Option<(PreparedHistoryReadRetirement<P,Mutation>,RetainedCloneProgress)>,ValueError> where P:Sync { self.owners.prepare_history_read_retirement(grant) }

    /// 🪞️ Delegates to [`StoreOwners::retire_derived_history_preview`].
    #[inline] pub fn retire_derived_history_preview(&self, owner: &mut Option<ArtifactDerivedHistoryPreview<P, Mutation>>,grant:RetainedCloneGrant) -> Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>, ValueError> where P: Sync { self.owners.retire_derived_history_preview(owner, grant) }

    /// 🪞️ Delegates to [`StoreOwners::retire_derived_report_replay`].
    #[inline] pub fn retire_derived_report_replay(&self, owner: &mut Option<ArtifactDerivedReplay<P, Mutation>>,grant:RetainedCloneGrant) -> Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>, ValueError> where P: Sync { self.owners.retire_derived_report_replay(owner, grant) }

    /// 🪞️ Delegates to [`StoreOwners::retire_finished_history_replay`].
    #[inline] pub fn retire_finished_history_replay(&self, owner: &mut Option<EditReplayResult<P, Mutation>>,grant:RetainedCloneGrant) -> Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>, ValueError> where P: Sync { self.owners.retire_finished_history_replay(owner, grant) }

    /// 🪞️ Delegates to [`StoreLive::step_derived_history_preview`].
    #[inline] pub fn step_derived_history_preview(&self, preview: &mut ArtifactDerivedHistoryPreview<P, Mutation>, retirement_grant:RetainedCloneGrant, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> { self.live.step_derived_history_preview(&self.log, &self.owners, &self.wire, &self.cursor, preview, retirement_grant, deadline) }

    /// 🪞️ Delegates to [`StoreCursor::finish_derived_history_preview`].
    #[inline] pub fn finish_derived_history_preview(&self, owner: &mut Option<ArtifactDerivedHistoryPreview<P, Mutation>>) -> Result<ArtifactDerivedSnapshot<P>, VcsError> { self.cursor.finish_derived_history_preview(&self.owners, &self.wire, owner) }

    /// 🪞️ Delegates to [`StoreCursor::begin_derived_report_replay`].
    #[inline] pub fn begin_derived_report_replay(&self, drafts: protocol::HistoryInputDrafts, from: Option<MutationId>) -> Result<ArtifactDerivedReplay<P, Mutation>, VcsError> { self.cursor.begin_derived_report_replay(&self.log, &self.owners, &self.wire, drafts, from) }

    /// 🪞️ Delegates to [`StoreLive::step_derived_report_replay`].
    #[inline] pub fn step_derived_report_replay(&self, owner: &mut ArtifactDerivedReplay<P, Mutation>, retirement_grant:RetainedCloneGrant, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> { self.live.step_derived_report_replay(&self.log, &self.owners, &self.wire, &self.cursor, owner, retirement_grant, deadline) }

    /// 🪞️ Delegates to [`StoreCursor::finish_derived_report_replay`].
    #[inline] pub fn finish_derived_report_replay(&self, owner: &mut Option<ArtifactDerivedReplay<P, Mutation>>) -> Result<(EditReplayResult<P, Mutation>, Option<ArtifactDerivedSnapshot<P>>), VcsError> { self.cursor.finish_derived_report_replay(&self.owners, &self.wire, owner) }

    /// 🪞️ Delegates to [`StoreLive::snapshot_read`].
    #[inline] pub fn snapshot_read(&self) -> Result<SnapshotRead<P>, VcsError> where P: Sync, { self.live.snapshot_read(&self.owners, &self.wire, &self.cursor) }

    /// 🪞️ Delegates to [`StoreLive::snapshot_owner`].
    #[inline] pub fn snapshot_owner(&self) -> Arc<P> { self.live.snapshot_owner(&self.wire) }

    /// 🪞️ Delegates to [`StoreLive::snapshot_root`].
    #[inline] pub fn snapshot_root(&self) -> Arc<P> { self.live.snapshot_root(&self.wire) }

    /// 🪞️ Delegates to [`StoreOwners::owned_snapshot_retirement_factory`].
    #[inline] pub fn owned_snapshot_retirement_factory(&self)->Result<&Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,ValueError> { self.owners.owned_snapshot_retirement_factory() }

    /// 🪞️ Delegates to [`StoreOwners::owned_mutation_retirement_factory`].
    #[inline] pub fn owned_mutation_retirement_factory(&self)->Result<&Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,ValueError> { self.owners.owned_mutation_retirement_factory() }

    /// 🪞️ Delegates to [`StoreOwners::owned_mutation_retirement_birth_bytes`].
    #[inline] pub fn owned_mutation_retirement_birth_bytes(&self)->Result<usize,ValueError> { self.owners.owned_mutation_retirement_birth_bytes() }

    /// 🪞️ Delegates to [`StoreOwners::retire_owned_mutation`].
    #[inline] pub fn retire_owned_mutation(&self,owner:&mut Option<Mutation>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError> { self.owners.retire_owned_mutation(owner, grant) }

    /// 🪞️ Delegates to [`StoreOwners::install_document_store_owners_exact`].
    #[inline] pub fn install_document_store_owners_exact(&mut self, mut owners: DocumentStoreOwners<P, Mutation>) -> Result<(), (ValueError, DocumentStoreOwners<P, Mutation>)> { self.owners.install_document_store_owners_exact(&mut self.log, owners) }

    /// 🪞️ Delegates to [`StoreLog::install_operation_wire`].
    #[inline] pub fn install_operation_wire(&mut self, wire: ArtifactOperationWire<Mutation>) { self.log.install_operation_wire(wire) }

    /// 🪞️ Delegates to [`StoreLog::operation_wire`].
    #[inline] pub fn operation_wire(&self) -> Option<ArtifactOperationWire<Mutation>> { self.log.operation_wire() }

    /// 🪞️ Delegates to [`StoreOwners::owned_disposer_installed`].
    #[inline] pub fn owned_disposer_installed(&self) -> bool { self.owners.owned_disposer_installed() }

    /// 🪞️ Delegates to [`StoreLive::replace_current_retained`].
    #[inline] pub fn replace_current_retained(&mut self, next: Arc<P>) -> Result<(), VcsError> { self.live.replace_current_retained(&mut self.log, &mut self.owners, next) }

    /// 🪞️ Delegates to [`StoreLive::prepare_document_root_commit`].
    #[inline] fn prepare_document_root_commit(&mut self, additional_displaced_owners: usize) -> Result<ArtifactStoreDocumentRootCommitAuthority<P, Mutation>, VcsError> { self.live.prepare_document_root_commit(&mut self.owners, additional_displaced_owners) }

    /// 🪞️ Delegates to [`StoreLive::commit_document_roots_retained`].
    #[inline] fn commit_document_roots_retained( &mut self, next_envelope: ArtifactEnvelope<P, Mutation>, next_current: Arc<P>, next_dag: semio_framework_os_spr::MutationDag, next_tail: Option<(String, Arc<P>)>, authority: ArtifactStoreDocumentRootCommitAuthority<P, Mutation>, ) { self.live.commit_document_roots_retained(&mut self.log, &mut self.owners, next_envelope, next_current, next_dag, next_tail, authority) }

    /// 🪞️ Delegates to [`StoreCursor::push_revision_replacement_reserved`].
    #[inline] pub fn push_revision_replacement_reserved(&mut self, next: CursorRevisionAccumulator) { self.cursor.push_revision_replacement_reserved(&mut self.owners, next) }

    /// 🪞️ Delegates to [`StoreLive::replace_applied_edit_ids_retained`].
    #[inline] pub fn replace_applied_edit_ids_retained(&mut self, next: Vec<String>) -> Result<(), VcsError> { self.live.replace_applied_edit_ids_retained(&mut self.log, &mut self.owners, &mut self.cursor, next) }

    /// 🪞️ Delegates to [`StoreLive::replace_redo_edit_ids_retained`].
    #[inline] pub fn replace_redo_edit_ids_retained(&mut self, next: Vec<String>) -> Result<(), VcsError> { self.live.replace_redo_edit_ids_retained(&mut self.log, &mut self.owners, &mut self.cursor, next) }

    /// 🪞️ Delegates to [`StoreReplay::replace_pending_report_retained`].
    #[inline] fn replace_pending_report_retained(&mut self, next: PendingCommandReport) -> Result<(), VcsError> { self.replay.replace_pending_report_retained(&mut self.owners, next) }

    /// 🪞️ Delegates to [`StoreWire::replace_backbone_retained`].
    #[inline] pub fn replace_backbone_retained(&mut self, next: Option<Backbones>) -> Result<(), VcsError> { self.wire.replace_backbone_retained(&mut self.owners, next) }

    /// 🪞️ Delegates to [`StoreOwners::replace_causal_dag_retained`].
    #[inline] fn replace_causal_dag_retained(&mut self, next: semio_framework_os_spr::MutationDag) -> Result<(), VcsError> { self.owners.replace_causal_dag_retained(&mut self.log, next) }

    /// 🪞️ Delegates to [`StoreOwners::reserve_edit_history_slot`].
    #[inline] pub fn reserve_edit_history_slot(&mut self) -> Result<ArtifactStoreHistoryCommitReservation, VcsError> { self.owners.reserve_edit_history_slot(&mut self.log) }

    /// 🪞️ Delegates to [`StoreLive::insert_reserved_edit_history`].
    #[inline] pub fn insert_reserved_edit_history(&mut self, mut reservation: ArtifactStoreHistoryCommitReservation, edit: Edit<Mutation>) -> Result<ArtifactHistoryKey, VcsError> { self.live.insert_reserved_edit_history(&mut self.log, &mut self.owners, reservation, edit) }

    /// 🪞️ Delegates to [`StoreLive::replace_tail_undo_cache_retained`].
    #[inline] pub fn replace_tail_undo_cache_retained(&mut self, next: Option<(String, Arc<P>)>) -> Result<(), VcsError> { self.live.replace_tail_undo_cache_retained(&mut self.log, &mut self.owners, next) }

    /// 🪞️ Delegates to [`StoreOwners::maintenance_retirements_step`].
    #[inline] pub fn maintenance_retirements_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.owners.maintenance_retirements_step(grant) }

    /// 🪞️ Delegates to [`StoreOwners::handoff_batch_publication_close`].
    #[inline] pub fn handoff_batch_publication_close(&mut self, publication: &mut Option<ArtifactStoreBatchPublication<P, Mutation>>) -> Result<bool, VcsError> where P: Sync, { self.owners.handoff_batch_publication_close(publication) }

    /// 🪞️ Delegates to [`StoreOwners::maintenance_retirements_terminal_is_empty`].
    #[inline] pub fn maintenance_retirements_terminal_is_empty(&self) -> bool { self.owners.maintenance_retirements_terminal_is_empty() }

    /// 🪞️ Delegates to [`StoreOwners::maintenance_retirements_demands`].
    #[inline] pub fn maintenance_retirements_demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.owners.maintenance_retirements_demands(copy) }

    /// 🪞️ Delegates to [`StoreOwners::maintenance_retirements_under_pressure`].
    #[inline] pub fn maintenance_retirements_under_pressure(&self) -> bool { self.owners.maintenance_retirements_under_pressure() }

    /// 🪞️ Delegates to [`StoreOwners::close_owned_phase_witness`].
    #[inline] pub fn close_owned_phase_witness(&self) -> String { self.owners.close_owned_phase_witness() }

    /// 🪞️ Delegates to [`StoreWire::close_displaced_witness`].
    #[inline] pub fn close_displaced_witness(&self) -> String { self.wire.close_displaced_witness(&self.owners) }

    /// 🪞️ Delegates to [`StoreOwners::close_one_factory_alias`].
    #[inline] fn close_one_factory_alias(&mut self) { self.owners.close_one_factory_alias() }

    /// 🪞️ Delegates to [`StoreOwners::retire_snapshot_alias`].
    #[inline] pub fn retire_snapshot_alias(&self,alias:&mut Option<Arc<P>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError> where P:Sync { self.owners.retire_snapshot_alias(alias, grant) }

    /// 🪞️ Delegates to [`StoreOwners::snapshot_read_retirement_demand`].
    #[inline] pub fn snapshot_read_retirement_demand(&self) -> semio_framework_value::RetirementDemand { self.owners.snapshot_read_retirement_demand() }

    /// 🪞️ Delegates to [`StoreOwners::returned_snapshot_read_retirement_demand`].
    #[inline] pub fn returned_snapshot_read_retirement_demand(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> where P: Sync, { self.owners.returned_snapshot_read_retirement_demand() }

    /// 🪞️ Delegates to [`StoreOwners::take_returned_snapshot_read_retirement`].
    #[inline] pub fn take_returned_snapshot_read_retirement(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> where P: Sync, { self.owners.take_returned_snapshot_read_retirement(grant) }

    /// 🪞️ Delegates to [`StoreLive::close_cursor_demands`].
    #[inline] pub fn close_cursor_demands(&self, phase: ArtifactStoreCursorDisposerPhase, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.live.close_cursor_demands(&self.log, &self.owners, &self.wire, phase, body) }

    /// 🪞️ Delegates to [`StoreWire::close_take_backbone_retirement`].
    #[inline] pub fn close_take_backbone_retirement(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> { self.wire.close_take_backbone_retirement(grant) }

    /// 🪞️ Delegates to [`StoreOwners::snapshot_read_leases_terminal_is_empty`].
    #[inline] pub fn snapshot_read_leases_terminal_is_empty(&self) -> bool { self.owners.snapshot_read_leases_terminal_is_empty() }

    /// 🪞️ Delegates to [`StoreOwners::returned_snapshot_read_count`].
    #[inline] pub fn returned_snapshot_read_count(&self) -> usize { self.owners.returned_snapshot_read_count() }

    /// 🪞️ Delegates to [`StoreOwners::outstanding_snapshot_read_count`].
    #[inline] pub fn outstanding_snapshot_read_count(&self) -> usize { self.owners.outstanding_snapshot_read_count() }

    /// 🪞️ Delegates to [`StoreLive::fold_current`].
    #[inline] pub async fn fold_current(&self) -> Result<P, VcsError> { self.live.fold_current(&self.log).await }

    /// 🪞️ Delegates to [`StoreReplay::replay_suffix_partitioned`].
    #[inline] pub fn replay_suffix_partitioned( base: Arc<P>, order: &[String], k: usize, edits: &impl ReplayEdits<Mutation>, schema: &str, supersessions: &EffectiveSupersessions, policy: semio_framework_os_spr::MergePolicy, ) -> Result<EditReplayResult<P, Mutation>, VcsError> { StoreReplay::<P, Mutation>::replay_suffix_partitioned(base, order, k, edits, schema, supersessions, policy) }

    /// 🪞️ Delegates to [`StoreLog::ensure_open_conflict_capacity`].
    #[inline] fn ensure_open_conflict_capacity(&self, additional_open: usize) -> Result<(), VcsError> { self.log.ensure_open_conflict_capacity(additional_open) }

    /// 🪞️ Delegates to [`StoreOwners::prune_resolved_conflicts`].
    #[inline] pub fn prune_resolved_conflicts(&mut self) -> Result<(), VcsError> { self.owners.prune_resolved_conflicts(&mut self.log) }

    /// 🪞️ Delegates to [`StoreLog::conflicts`].
    #[inline] pub fn conflicts(&self) -> &[semio_framework_os_spr::Conflict] { self.log.conflicts() }

    /// 🪞️ Delegates to [`StoreLog::open_conflicts`].
    #[inline] pub fn open_conflicts(&self) -> impl Iterator<Item = &semio_framework_os_spr::Conflict> { self.log.open_conflicts() }

    /// 🪞️ Delegates to [`StoreLog::merge_policy`].
    #[inline] pub fn merge_policy(&self) -> semio_framework_os_spr::MergePolicy { self.log.merge_policy() }

    /// 🪞️ Delegates to [`StoreLog::set_merge_policy`].
    #[inline] pub fn set_merge_policy(&mut self, policy: semio_framework_os_spr::MergePolicy) { self.log.set_merge_policy(policy) }

    /// 🪞️ Delegates to [`StoreLog::messages_for_edit`].
    #[inline] pub fn messages_for_edit(&self, edit_id: &str) -> &[semio_framework_os_spr::MutationMessage] { self.log.messages_for_edit(edit_id) }

    /// 🪞️ Delegates to [`StoreLog::trunk_alternative_id`].
    #[inline] pub fn trunk_alternative_id(&self) -> String { self.log.trunk_alternative_id() }

    /// 🪞️ Delegates to [`StoreLog::active_line_id`].
    #[inline] pub fn active_line_id(&self) -> String { self.log.active_line_id() }

    /// 🪞️ Delegates to [`StoreLive::supersessions`].
    #[inline] pub fn supersessions(&self) -> &EffectiveSupersessions { self.live.supersessions() }

    /// 🪞️ Delegates to [`StoreLog::replay_edits`].
    #[inline] pub fn replay_edits(&self) -> &ArtifactHistoryLedger<Edit<Mutation>> { self.log.replay_edits() }

    /// 🪞️ Delegates to [`StoreLive::mutation_ops`].
    #[inline] pub fn mutation_ops(&self) -> Result<Vec<AppliedMutation<'_, Mutation>>, VcsError> { self.live.mutation_ops(&self.log, &self.wire, &self.cursor) }

    /// 🪞️ Delegates to [`StoreCursor::mutation_position`].
    #[inline] pub fn mutation_position(&self, target: &MutationId) -> Result<(usize, usize), VcsError> { self.cursor.mutation_position(&self.log, &self.wire, target) }

    /// 🪞️ Delegates to [`StoreLive::applied_mutation`].
    #[inline] pub fn applied_mutation(&self, target: &MutationId) -> Result<AppliedMutation<'_, Mutation>, VcsError> { self.live.applied_mutation(&self.log, &self.wire, &self.cursor, target) }

    /// 🪞️ Delegates to [`StoreLive::applied_edit_mutations`].
    #[inline] pub fn applied_edit_mutations(&self, position: usize) -> Result<Vec<AppliedMutation<'_, Mutation>>, VcsError> { self.live.applied_edit_mutations(&self.log, &self.wire, &self.cursor, position) }

    /// 🪞️ Delegates to [`StoreLive::mutation_outcomes`].
    #[inline] pub fn mutation_outcomes(&self) -> Result<Vec<protocol::MutationReplayOutcome>, VcsError> { self.live.mutation_outcomes(&self.log, &self.wire, &self.cursor) }

    /// 🪞️ Delegates to [`StoreLive::applied_edit_outcomes`].
    #[inline] pub fn applied_edit_outcomes(&self, position: usize) -> Result<Vec<protocol::MutationReplayOutcome>, VcsError> { self.live.applied_edit_outcomes(&self.log, &self.wire, &self.cursor, position) }

    /// 🪞️ Delegates to [`StoreCursor::applied_edit_position`].
    #[inline] pub fn applied_edit_position(&self, edit_id: &str) -> Option<usize> { self.cursor.applied_edit_position(&self.log, &self.wire, edit_id) }

    /// 🪞️ Delegates to [`StoreLive::unit_id`].
    #[inline] pub fn unit_id(&self, mutation_id: &MutationId) -> Result<Option<String>, VcsError> { self.live.unit_id(&self.log, &self.wire, &self.cursor, mutation_id) }

    /// 🪞️ Delegates to [`StoreCursor::unit_operations`].
    #[inline] pub fn unit_operations(&self, mutation_id: &MutationId) -> Result<Vec<MutationId>, VcsError> { self.cursor.unit_operations(&self.log, &self.wire, mutation_id) }

    /// 🪞️ Delegates to [`StoreLive::state_before`].
    #[inline] pub fn state_before(&mut self, mutation_id: &MutationId, drafts: &protocol::HistoryInputDrafts) -> Result<Arc<P>, VcsError> { self.live.state_before(&mut self.log, &mut self.owners, &mut self.wire, &mut self.cursor, mutation_id, drafts) }

    /// 🪞️ Delegates to [`StoreLive::begin_report_replay`].
    #[inline] pub fn begin_report_replay(&self, drafts: &protocol::HistoryInputDrafts, from: Option<&MutationId>) -> Result<EditReplay<P, Mutation>, VcsError> { self.live.begin_report_replay(&self.log, &self.wire, &self.cursor, drafts, from) }

    /// 🪞️ Delegates to [`StoreLive::refuse_blocking_history`].
    #[inline] pub fn refuse_blocking_history(&self) -> Result<(), VcsError> { self.live.refuse_blocking_history(&self.log) }

    /// 🪞️ Delegates to [`StoreLive::replay_report`].
    #[inline] pub fn replay_report(&self, result: &EditReplayResult<P, Mutation>) -> Result<protocol::ReplayReport, VcsError> { self.live.replay_report(&self.log, &self.wire, &self.cursor, result) }

    /// 🪞️ Delegates to [`StoreCursor::admit_apply_batch`].
    #[inline] pub fn admit_apply_batch( &self, operation:semio_framework_job::OperationId, expected_generation:u64, expected_revision:[u8;32], actor:&mut Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_value::SharedUtf8>>, mutations:&mut Vec<Mutation>, lane:HistoryLane, factory:Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P,Mutation>>>, transaction:&mut Option<protocol::TransactionRef>, verb:Option<&str>, outbound:bool, grant:RetainedCloneGrant, )->Result<Option<(ArtifactStoreBatchPublication<P,Mutation>,RetainedCloneProgress)>,ValueError> where P:Sync,Mutation:Send { self.cursor.admit_apply_batch(&self.log, &self.owners, &self.wire, operation, expected_generation, expected_revision, actor, mutations, lane, factory, transaction, verb, outbound, grant) }

    /// 🪞️ Delegates to [`StoreCursor::begin_apply_batch`].
    #[inline] pub fn begin_apply_batch( &self, operation: semio_framework_job::OperationId, expected_generation: u64, expected_revision: [u8; 32], actor: semio_framework_value::SharedUtf8, mutations: Vec<Mutation>, lane: HistoryLane, factory: Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>>, transaction: Option<protocol::TransactionRef>, ) -> Result<ArtifactStoreBatchPublication<P, Mutation>, ArtifactStoreBatchAdmissionRejected<Mutation>> where P: Sync, Mutation: Send, { self.cursor.begin_apply_batch(&self.log, &self.owners, &self.wire, operation, expected_generation, expected_revision, actor, mutations, lane, factory, transaction) }

    /// 🪞️ Delegates to [`StoreCursor::begin_outbound_apply_batch`].
    #[allow(clippy::too_many_arguments)] #[inline] pub fn begin_outbound_apply_batch( &self, operation: semio_framework_job::OperationId, expected_generation: u64, expected_revision: [u8; 32], actor: semio_framework_value::SharedUtf8, mutations: Vec<Mutation>, factory: Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>>, transaction: Option<protocol::TransactionRef>, ) -> Result<ArtifactStoreBatchPublication<P, Mutation>, ArtifactStoreBatchAdmissionRejected<Mutation>> where P: Sync, Mutation: Send, { self.cursor.begin_outbound_apply_batch(&self.log, &self.owners, &self.wire, operation, expected_generation, expected_revision, actor, mutations, factory, transaction) }

    /// 🪞️ Delegates to [`StoreReplay::flush_published_apply_batch`].
    #[inline] pub async fn flush_published_apply_batch(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mutation>) -> Result<bool, VcsError> { self.replay.flush_published_apply_batch(&mut self.log, &mut self.owners, &mut self.wire, publication).await }

    /// 🪞️ Delegates to [`StoreCursor::admit_member_apply_batch`].
    #[inline] pub fn admit_member_apply_batch( &self, operation:semio_framework_job::OperationId, expected_generation:u64, expected_revision:[u8;32], actor:&mut Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_value::SharedUtf8>>, mutations:&mut Vec<Mutation>, transaction:&mut Option<protocol::TransactionRef>, grant:RetainedCloneGrant, )->Result<Option<(ArtifactStoreBatchPublication<P,Mutation>,RetainedCloneProgress)>,ValueError> where P:Sync,Mutation:Send { self.cursor.admit_member_apply_batch(&self.log, &self.owners, &self.wire, operation, expected_generation, expected_revision, actor, mutations, transaction, grant) }

    /// 🪞️ Delegates to [`StoreCursor::begin_member_apply_batch`].
    #[inline] pub fn begin_member_apply_batch( &self, operation: semio_framework_job::OperationId, expected_generation: u64, expected_revision: [u8; 32], actor: semio_framework_value::SharedUtf8, mutations: Vec<Mutation>, ) -> Result<ArtifactStoreBatchPublication<P, Mutation>, ArtifactStoreBatchAdmissionRejected<Mutation>> where P: Sync, Mutation: Send, { self.cursor.begin_member_apply_batch(&self.log, &self.owners, &self.wire, operation, expected_generation, expected_revision, actor, mutations) }

    /// 🪞️ Delegates to [`StoreCursor::begin_apply_batch_owned`].
    #[allow(clippy::too_many_arguments)] #[inline] pub fn begin_apply_batch_owned<A>( &self, operation: semio_framework_job::OperationId, expected_generation: u64, expected_revision: [u8; 32], actor: semio_framework_value::SharedUtf8, lane: HistoryLane, group_id: Option<String>, source: ArtifactStoreBatchSourceOf<P, Mutation, A>, outbound: bool, transaction: Option<protocol::TransactionRef>, ) -> Result<ArtifactStoreBatchPublication<P, Mutation>, ArtifactStoreBatchAdmissionRejected<A::Input>> where P: Sync + Send, A: ArtifactStoreBatchItemAuthority<P, Mutation> + 'static, Mutation: 'static, P: 'static, { self.cursor.begin_apply_batch_owned::<A>(&self.log, &self.wire, operation, expected_generation, expected_revision, actor, lane, group_id, source, outbound, transaction) }

    /// 🪞️ Delegates to [`StoreLive::batch_item_base`].
    #[inline] fn batch_item_base(&self, staged_root: Option<&Arc<P>>) -> Result<SnapshotRead<P>, VcsError> where P: Sync, { self.live.batch_item_base(&self.owners, &self.wire, &self.cursor, staged_root) }

    /// 🪞️ Delegates to [`StoreLog::batch_amend_target_ref`].
    #[inline] fn batch_amend_target_ref(&self, transaction: Option<&protocol::TransactionRef>) -> Option<&str> { self.log.batch_amend_target_ref(transaction) }

    /// 🪞️ Delegates to [`StoreLog::batch_amend_target`].
    #[inline] fn batch_amend_target(&self, transaction: Option<&protocol::TransactionRef>) -> Option<String> { self.log.batch_amend_target(transaction) }

    /// 🪞️ Delegates to [`StoreLive::advance_prepared_foreign_steps`].
    #[inline] pub fn advance_prepared_foreign_steps( &self, publication:&ArtifactStoreBatchPublication<P,Mutation>, output:&mut protocol::ForeignStepsPreparation, grant:RetainedCloneGrant, )->Result<RetainedCloneStep,ValueError> { self.live.advance_prepared_foreign_steps(&self.cursor, publication, output, grant) }

    /// 🪞️ Delegates to [`StoreLive::advance_prepared_operation_wire`].
    #[inline] pub fn advance_prepared_operation_wire( &self, publication:&ArtifactStoreBatchPublication<P,Mutation>, cursor:&mut ArtifactPreparedOperationCursor,output:&mut[u8],grant:RetainedCloneGrant, )->Result<ArtifactPreparedOperationProgress,ArtifactPreparedOperationError> { self.live.advance_prepared_operation_wire(&self.cursor, publication, cursor, output, grant) }

    /// 🪞️ Delegates to [`StoreLog::cancel_apply_batch`].
    #[inline] pub fn cancel_apply_batch(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mutation>) -> bool { self.log.cancel_apply_batch(publication) }

    /// 🪞️ Delegates to [`StoreOwners::fold_batch_item`].
    #[inline] fn fold_batch_item(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mutation>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemAdvance, VcsError> { self.owners.fold_batch_item(&mut self.log, publication, grant) }

    /// 🪞️ Delegates to [`StoreOwners::empty_batch_stage`].
    #[inline] pub fn empty_batch_stage(authority: &Arc<ArtifactStoreOneItemLiveAuthority>) -> ArtifactStoreBatchStage<P, Mutation> { StoreOwners::<P, Mutation>::empty_batch_stage(authority) }

    /// 🪞️ Delegates to [`StoreOwners::empty_batch_edit`].
    #[inline] pub fn empty_batch_edit(authority: &ArtifactStoreOneItemLiveAuthority) -> Edit<Mutation> { StoreOwners::<P, Mutation>::empty_batch_edit(authority) }

    /// 🪞️ Delegates to [`StoreLog::edit_mutation_ids`].
    #[inline] pub fn edit_mutation_ids(&self, edit_id: &str) -> Result<Vec<MutationId>, VcsError> { self.log.edit_mutation_ids(edit_id) }

    /// 🪞️ Delegates to [`StoreLog::checkpoint_origin`].
    #[inline] pub fn checkpoint_origin(&self, checkpoint_id: &str) -> Option<MutationId> { self.log.checkpoint_origin(checkpoint_id) }

    /// 🪞️ Delegates to [`StoreLog::seed_known_events`].
    #[inline] fn seed_known_events(&mut self) -> Result<(), VcsError> { self.log.seed_known_events() }

    /// 🪞️ Delegates to [`StoreReplay::admit_remote_transitions`].
    #[inline] async fn admit_remote_transitions(&mut self, transitions: Vec<semio_framework_os_spr::MutationEnvelope>) -> Result<Vec<semio_framework_os_spr::EditMessages>, VcsError> { self.replay.admit_remote_transitions(&mut self.log, &mut self.owners, &mut self.wire, &mut self.cursor, &mut self.live, transitions).await }

    /// 🪞️ Delegates to [`StoreLog::admit_history_shape`].
    #[inline] pub fn admit_history_shape(&self, envelopes: &[semio_framework_os_spr::MutationEnvelope]) -> Result<(), VcsError> { self.log.admit_history_shape(envelopes) }

    /// 🪞️ Delegates to [`StoreReplay::retract_transitions`].
    #[inline] async fn retract_transitions(&mut self, refused: &[String]) -> Result<Vec<String>, VcsError> { self.replay.retract_transitions(&mut self.log, &mut self.owners, &mut self.cursor, &mut self.live, refused).await }

    /// 🪞️ Delegates to [`StoreLive::insert_transition`].
    #[inline] pub fn insert_transition(&mut self, envelope: semio_framework_os_spr::MutationEnvelope) { self.live.insert_transition(&mut self.log, envelope) }

    /// 🪞️ Delegates to [`StoreLog::transition_envelope`].
    #[inline] pub fn transition_envelope(&self, transition: &semio_framework_os_spr::HistoryTransition, dependencies: Vec<MutationId>, target: Vec<String>, timestamp: HybridLogicalTimestamp) -> semio_framework_os_spr::MutationEnvelope { self.log.transition_envelope(transition, dependencies, target, timestamp) }

    /// 🪞️ Delegates to [`StoreLive::reproject`].
    #[inline] pub async fn reproject(&mut self) -> Result<Vec<semio_framework_os_spr::EditMessages>, VcsError> { self.live.reproject(&mut self.log, &mut self.owners, &mut self.cursor).await }

    /// 🪞️ Delegates to [`StoreReplay::tail_step`].
    #[inline] fn tail_step(&self, envelope: &semio_framework_os_spr::MutationEnvelope) -> Option<TailStep> { self.replay.tail_step(&self.log, &self.cursor, &self.live, envelope) }

    /// 🪞️ Delegates to [`StoreLive::live_prefix_state`].
    #[inline] pub fn live_prefix_state(&self, length: usize) -> Result<(Arc<P>, Vec<(usize, Option<[u8; 32]>, Arc<P>)>), VcsError> { self.live.live_prefix_state(&self.log, &self.cursor, length) }

    /// 🪞️ Delegates to [`StoreLive::reproject_replayed`].
    #[inline] pub fn reproject_replayed(&mut self, result: EditReplayResult<P, Mutation>) -> Result<Vec<semio_framework_os_spr::EditMessages>, VcsError> { self.live.reproject_replayed(&mut self.log, &mut self.owners, &mut self.cursor, result) }

    /// 🪞️ Delegates to [`StoreOwners::retire_replay_result`].
    #[inline] pub fn retire_replay_result(&mut self, mut result: EditReplayResult<P, Mutation>) -> Result<(), VcsError> { self.owners.retire_replay_result(result) }

    /// 🪞️ Delegates to [`StoreOwners::retire_pending_replay`].
    #[inline] pub fn retire_pending_replay(&mut self, replay: Option<([u8; 32], EditReplay<P, Mutation>)>) -> Result<(), VcsError> { self.owners.retire_pending_replay(replay) }

    /// 🪞️ Delegates to [`StoreLive::replace_supersessions_retained`].
    #[inline] pub fn replace_supersessions_retained(&mut self, next: EffectiveSupersessions) -> Result<(), VcsError> { self.live.replace_supersessions_retained(&mut self.owners, next) }

    /// 🪞️ Delegates to [`StoreLive::admit_prefix_snapshots`].
    #[inline] pub fn admit_prefix_snapshots(&mut self, snapshots: Vec<(usize, Option<[u8; 32]>, Arc<P>)>) -> Result<(), VcsError> { self.live.admit_prefix_snapshots(&mut self.log, &mut self.owners, snapshots) }

    /// 🪞️ Delegates to [`StoreLive::clear_prefix_ring`].
    #[inline] fn clear_prefix_ring(&mut self) -> Result<(), VcsError> { self.live.clear_prefix_ring(&mut self.owners) }

    /// 🪞️ Delegates to [`StoreReplay::append_transaction`].
    #[inline] async fn append_transaction(&mut self, mutations: Vec<Mutation>, transaction: protocol::TransactionRef, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> { self.replay.append_transaction(&mut self.log, &mut self.owners, &mut self.cursor, &mut self.live, mutations, transaction, identity).await }

    /// 🪞️ Delegates to [`StoreReplay::continue_pending_replay_across_append`].
    #[inline] pub fn continue_pending_replay_across_append(&mut self, before: [u8; 32], edit_id: &str, appended: usize) { self.replay.continue_pending_replay_across_append(&mut self.cursor, before, edit_id, appended) }

    /// 🪞️ Delegates to [`StoreReplay::commit_open_transaction`].
    #[inline] pub fn commit_open_transaction(&mut self, transaction_id: &str) -> Result<(), VcsError> { self.replay.commit_open_transaction(&mut self.log, &mut self.owners, &mut self.cursor, &mut self.live, transaction_id) }

    /// 🪞️ Delegates to [`StoreReplay::abort_open_transaction`].
    #[inline] pub fn abort_open_transaction(&mut self, transaction_id: &str) -> Result<(), VcsError> { self.replay.abort_open_transaction(&mut self.log, &mut self.owners, &mut self.cursor, &mut self.live, transaction_id) }

    /// 🪞️ Delegates to [`StoreReplay::flush_announcements`].
    #[inline] pub async fn flush_announcements(&mut self) -> Result<(), VcsError> { self.replay.flush_announcements(&mut self.owners, &mut self.wire).await }

    /// 🪞️ Delegates to [`StoreLive::lift_open_transaction`].
    #[inline] async fn lift_open_transaction(&mut self) -> Result<Option<(OpenToolTransaction, Edit<Mutation>)>, VcsError> { self.live.lift_open_transaction(&mut self.log, &mut self.owners, &mut self.cursor).await }

    /// 🪞️ Delegates to [`StoreReplay::restore_open_transaction`].
    #[inline] async fn restore_open_transaction(&mut self, open: OpenToolTransaction, mut edit: Edit<Mutation>) -> Result<(), VcsError> { self.replay.restore_open_transaction(&mut self.log, &mut self.owners, &mut self.cursor, &mut self.live, open, edit).await }

    /// 🪞️ Delegates to [`StoreReplay::defer_remote_replays`].
    #[inline] pub fn defer_remote_replays(&mut self, budget: Option<ReplayTurnBudget>) { self.replay.defer_remote_replays(budget) }

    /// 🪞️ Delegates to [`StoreReplay::defer_local_replays`].
    #[inline] pub fn defer_local_replays(&mut self, budget: Option<ReplayTurnBudget>) { self.replay.defer_local_replays(budget) }

    /// 🪞️ Delegates to [`StoreReplay::local_step_pending`].
    #[inline] pub fn local_step_pending(&self) -> bool { self.replay.local_step_pending() }

    /// 🪞️ Delegates to [`StoreReplay::discard_local_step`].
    #[inline] pub fn discard_local_step(&mut self) -> bool { self.replay.discard_local_step(&mut self.owners) }

    /// 🪞️ Delegates to [`StoreReplay::reprojection_progress`].
    #[inline] pub fn reprojection_progress(&self) -> Option<ReplayProgress> { self.replay.reprojection_progress(&self.cursor) }

    /// 🪞️ Delegates to [`StoreReplay::reprojection_operation_preparation_progress`].
    #[inline] pub fn reprojection_operation_preparation_progress(&self) -> Option<ArtifactReplayPreparationProgress> { self.replay.reprojection_operation_preparation_progress(&self.cursor) }

    /// 🪞️ Delegates to [`StoreReplay::cancel_reprojection`].
    #[inline] pub fn cancel_reprojection(&mut self) -> bool { self.replay.cancel_reprojection(&mut self.owners) }

    /// 🪞️ Delegates to [`StoreReplay::step_reprojection`].
    #[inline] pub async fn step_reprojection(&mut self, deadline_us: Option<u64>) -> Result<Option<ReplayProgress>, VcsError> { self.replay.step_reprojection(&mut self.log, &mut self.owners, &mut self.wire, &mut self.cursor, &mut self.live, deadline_us).await }

    /// 🪞️ Delegates to [`StoreReplay::defer_local_step`].
    #[inline] pub async fn defer_local_step(&mut self, step: PendingLocalStep, clock: HybridLogicalTimestamp) -> Result<Option<PendingLocalStep>, VcsError> { self.replay.defer_local_step(&mut self.log, &mut self.owners, &mut self.wire, &mut self.cursor, &mut self.live, step, clock).await }

    /// 🪞️ Delegates to [`StoreLog::author_head_alternative`].
    #[inline] async fn author_head_alternative(&self, name: String, clock: &mut HybridLogicalTimestamp, envelopes: &mut Vec<semio_framework_os_spr::MutationEnvelope>, identity: &mut EntityIdentityAuthority<'_>) -> Result<String, VcsError> { self.log.author_head_alternative(name, clock, envelopes, identity).await }

    /// 🪞️ Delegates to [`StoreLog::prospective_fold`].
    #[inline] fn prospective_fold(&self, candidates: &[semio_framework_os_spr::MutationEnvelope]) -> Result<semio_framework_os_spr::HistoryFold, VcsError> { self.log.prospective_fold(candidates) }

    /// 🪞️ Delegates to [`StoreLive::dry_run`].
    #[inline] fn dry_run(&self, fold: &semio_framework_os_spr::HistoryFold) -> Result<EditReplayResult<P, Mutation>, VcsError> { self.live.dry_run(&self.log, &self.cursor, fold) }

    /// 🪞️ Delegates to [`StoreLive::supersede_inputs`].
    #[inline] pub fn supersede_inputs(&self, inputs: Vec<SupersedeInput<Mutation>>) -> Result<(Vec<protocol::SupersededInput>, Vec<String>), VcsError> { self.live.supersede_inputs(&self.log, &self.wire, &self.cursor, inputs) }

    /// 🪞️ Delegates to [`StoreLive::draft_inputs`].
    #[inline] fn draft_inputs(&self, drafts: &protocol::HistoryInputDrafts) -> Result<(Vec<protocol::SupersededInput>, Vec<String>), VcsError> { self.live.draft_inputs(&self.log, &self.wire, &self.cursor, drafts) }

    /// 🪞️ Delegates to [`StoreReplay::record_replay_report`].
    #[inline] pub fn record_replay_report(&mut self, report: &protocol::ReplayReport) { self.replay.record_replay_report(report) }

    /// 🪞️ Delegates to [`StoreReplay::absorb_merge_report`].
    #[inline] fn absorb_merge_report(&mut self, report: &semio_framework_os_spr::MergeReport) { self.replay.absorb_merge_report(report) }

    /// 🪞️ Delegates to [`StoreLog::edit_lane`].
    #[inline] pub fn edit_lane(&self, edit_id: &str) -> HistoryLane { self.log.edit_lane(edit_id) }

    /// 🪞️ Delegates to [`StoreLog::redo_position`].
    #[inline] fn redo_position(&self, lane: impl Fn(HistoryLane) -> bool) -> Option<usize> { self.log.redo_position(lane) }

    /// 🪞️ Delegates to [`StoreReplay::switch_local_alternative`].
    #[inline] async fn switch_local_alternative(&mut self, alternative_id: String) -> Result<(), VcsError> { self.replay.switch_local_alternative(&mut self.log, &mut self.owners, &mut self.wire, &mut self.cursor, &mut self.live, alternative_id).await }

    /// 🪞️ Delegates to [`StoreReplay::checkout_local_checkpoint`].
    #[inline] async fn checkout_local_checkpoint(&mut self, checkpoint_id: String) -> Result<(), VcsError> { self.replay.checkout_local_checkpoint(&mut self.log, &mut self.owners, &mut self.wire, &mut self.cursor, &mut self.live, checkpoint_id).await }

    /// 🪞️ Delegates to [`StoreLog::viewer_head_now`].
    #[inline] pub fn viewer_head_now(&self) -> (Option<String>, Option<String>) { self.log.viewer_head_now() }

    /// 🪞️ Delegates to [`StoreLog::pending_checkpoint_transition`].
    #[inline] pub async fn pending_checkpoint_transition(&self, message: Option<String>, authors: Vec<Author>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(semio_framework_os_spr::HistoryTransition, Vec<MutationId>, String), VcsError> { self.log.pending_checkpoint_transition(message, authors, identity).await }

    /// 🪞️ Delegates to [`StoreReplay::apply_command`].
    #[inline] async fn apply_command(&mut self, mutations: Vec<Mutation>, lane: HistoryLane, transaction: Option<protocol::TransactionRef>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> { self.replay.apply_command(&mut self.log, &mut self.owners, &mut self.cursor, &mut self.live, mutations, lane, transaction, identity).await }

    /// 🪞️ Delegates to [`StoreOwners::replace_edit_messages`].
    #[inline] pub fn replace_edit_messages(&mut self, edit_id: &str, messages: Vec<semio_framework_os_spr::MutationMessage>) -> Result<(), VcsError> { self.owners.replace_edit_messages(&mut self.log, edit_id, messages) }

    /// 🪞️ Delegates to [`StoreOwners::authored_operation_bytes`].
    #[inline] pub fn authored_operation_bytes(&mut self,mutations:&[Mutation],identity:&mut EntityIdentityAuthority<'_>)->Result<Vec<Vec<u8>>,VcsError> { self.owners.authored_operation_bytes(&mut self.log, mutations, identity) }

    /// 🪞️ Delegates to [`StoreOwners::authored_edit_operation_bytes`].
    #[inline] pub fn authored_edit_operation_bytes(&mut self,identity:&mut EntityIdentityAuthority<'_>)->Result<Option<(Vec<Vec<u8>>,Vec<Vec<u8>>)>,VcsError> { self.owners.authored_edit_operation_bytes(&mut self.log, identity) }

    /// 🪞️ Delegates to [`StoreLog::snapshot_pack`].
    #[inline] pub async fn snapshot_pack(&self) -> Result<ArtifactPackFiles, VcsError> where P: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot, { self.log.snapshot_pack().await }

    /// 🪞️ Delegates to [`StoreLive::snapshot_json`].
    #[inline] pub fn snapshot_json(&self) -> Result<String, VcsError> { self.live.snapshot_json(&self.wire) }

    /// 🪞️ Delegates to [`StoreLog::envelope_json`].
    #[inline] pub fn envelope_json(&self) -> Result<String, VcsError> where P: ToValue, Mutation: ToValue, { self.log.envelope_json() }

    /// 🪞️ Delegates to [`StoreLog::event_log`].
    #[inline] pub fn event_log(&self) -> Result<Vec<semio_framework_os_spr::MutationEnvelope>, VcsError> { self.log.event_log() }

    /// 🪞️ Delegates to [`StoreLog::event_log_payload`].
    #[inline] pub fn event_log_payload(&self) -> Result<Vec<u8>, VcsError> { self.log.event_log_payload() }

    /// 🪞️ Delegates to [`StoreLog::announce_tail_edit_payload`].
    #[inline] pub fn announce_tail_edit_payload(&mut self) -> Result<Vec<u8>, VcsError> { self.log.announce_tail_edit_payload() }

    /// 🪞️ Delegates to [`StoreWire::send_member_mutations`].
    #[inline] pub async fn send_member_mutations(&mut self, owner: &str, slot: &str, child_id: &str, envelopes: Vec<u8>) -> Result<(), VcsError> { self.wire.send_member_mutations(&mut self.owners, owner, slot, child_id, envelopes).await }

    /// 🪞️ Delegates to [`StoreWire::take_member_inbound`].
    #[inline] pub fn take_member_inbound(&mut self) -> Vec<(String, String, String, Vec<u8>)> { self.wire.take_member_inbound() }

    /// 🪞️ Delegates to [`StoreWire::announce_history`].
    #[inline] async fn announce_history(&mut self) -> Result<(), VcsError> { self.wire.announce_history(&mut self.log, &mut self.owners).await }

    /// 🪞️ Delegates to [`StoreReplay::detach_backbone`].
    #[inline] pub fn detach_backbone(&mut self) -> Result<Option<Backbones>, VcsError> { self.replay.detach_backbone(&mut self.log, &mut self.owners, &mut self.wire, &mut self.cursor, &mut self.live) }

    /// 🪞️ Delegates to [`StoreLog::backbone_ref`].
    #[inline] pub fn backbone_ref(&self) -> Option<&ArtifactBackboneRef> { self.log.backbone_ref() }

    /// 🪞️ Delegates to [`StoreOwners::refuse_with_candidate_dag`].
    #[inline] fn refuse_with_candidate_dag<T>(&mut self, candidate_dag: semio_framework_os_spr::MutationDag, error: VcsError) -> Result<T, VcsError> { self.owners.refuse_with_candidate_dag::<T>(candidate_dag, error) }

    /// 🪞️ Delegates to [`StoreLog::assert_equivalent_remote_envelope`].
    #[inline] fn assert_equivalent_remote_envelope(&self, existing: &Edit<Mutation>, incoming: &semio_framework_os_spr::MutationEnvelope) -> Result<(), VcsError> { self.log.assert_equivalent_remote_envelope(existing, incoming) }

    /// 🪞️ Delegates to [`StoreLog::assert_equivalent_remote_mutation`].
    #[inline] fn assert_equivalent_remote_mutation(&self, incoming: &semio_framework_os_spr::MutationEnvelope) -> Result<(), VcsError> { self.log.assert_equivalent_remote_mutation(incoming) }

    /// 🪞️ Delegates to [`StoreReplay::resolve_conflict`].
    #[inline] pub async fn resolve_conflict(&mut self, conflict_id: &str, resolution: semio_framework_os_spr::ConflictResolution) -> Result<semio_framework_os_spr::MergeReport, VcsError> where P: Sync, { self.replay.resolve_conflict(&mut self.log, &mut self.owners, &mut self.wire, &mut self.cursor, &mut self.live, conflict_id, resolution).await }

    /// 🪞️ Delegates to [`StoreLog::verify_genesis`].
    #[inline] fn verify_genesis(&self, pack: &[u8]) -> Result<(), VcsError> { self.log.verify_genesis(pack) }

    /// 🪞️ Delegates to [`StoreReplay::flush_outbound`].
    #[inline] pub async fn flush_outbound(&mut self) -> Result<(), VcsError> { self.replay.flush_outbound(&mut self.owners, &mut self.wire).await }

    /// 🪞️ Delegates to [`StoreLog::edit_is_local`].
    #[inline] pub fn edit_is_local(&self, edit_id: &str) -> bool { self.log.edit_is_local(edit_id) }

    /// 🪞️ Delegates to [`StoreCursor::restamp_applied_tail`].
    #[inline] pub fn restamp_applied_tail(&mut self) { self.cursor.restamp_applied_tail(&mut self.log) }

    /// 🪞️ Delegates to [`StoreCursor::push_applied_edit_id`].
    #[inline] pub fn push_applied_edit_id(&mut self, edit_id: String) { self.cursor.push_applied_edit_id(&mut self.log, edit_id) }

    /// 🪞️ Delegates to [`StoreLive::pop_applied_edit_id`].
    #[inline] fn pop_applied_edit_id(&mut self) -> Option<String> { self.live.pop_applied_edit_id(&mut self.log, &mut self.cursor) }

    /// 🪞️ Delegates to [`StoreCursor::push_redo_edit_id`].
    #[inline] fn push_redo_edit_id(&mut self, edit_id: String) { self.cursor.push_redo_edit_id(&mut self.log, edit_id) }

    /// 🪞️ Delegates to [`StoreCursor::pop_redo_edit_id`].
    #[inline] fn pop_redo_edit_id(&mut self) -> Option<String> { self.cursor.pop_redo_edit_id(&mut self.log) }

    /// 🪞️ Delegates to [`StoreReplay::bump`].
    #[inline] pub fn bump(&mut self) -> Result<(), VcsError> { self.replay.bump(&mut self.log, &mut self.owners, &mut self.cursor, &mut self.live) }

    /// 🪞️ Delegates to [`StoreReplay::assert_mirrors_are_live`].
    #[cfg(test)] #[inline] pub fn assert_mirrors_are_live(&self) { self.replay.assert_mirrors_are_live(&self.log, &self.cursor, &self.live) }
}

impl<P, Mutation> ArtifactStoreCloseHost<P, Mutation> for ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    fn close_host_member_inbox_capacity(&self) -> usize { self.wire.member_inbox.capacity() }
    fn close_host_backbone_attached(&self) -> bool { self.wire.backbone.is_some() }
    fn close_host_prefix_ring_is_empty(&self) -> bool { self.live.prefix_ring.is_empty() }
    fn close_host_snapshot_read_leases(&self) -> &SnapshotReadLeaseRegistryOwner { &self.owners.snapshot_read_leases }
    fn close_host_initial_snapshot_retirement_factory(&self) -> Option<&Arc<dyn ArtifactOwnedValueRetirementFactory<P>>> { self.owners.initial_snapshot_retirement_factory.as_ref() }
    fn snapshot_read_leases_terminal_is_empty(&self) -> bool { ArtifactStore::snapshot_read_leases_terminal_is_empty(self) }
    fn maintenance_retirements_terminal_is_empty(&self) -> bool { ArtifactStore::maintenance_retirements_terminal_is_empty(self) }
    fn maintenance_retirements_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { ArtifactStore::maintenance_retirements_step(self, grant) }
    fn close_take_backbone_retirement(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> { ArtifactStore::close_take_backbone_retirement(self, grant) }
    fn close_cursor_admit(&mut self, phase: ArtifactStoreCursorDisposerPhase, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> { ArtifactStore::close_cursor_admit(self, phase, grant) }
    fn close_cursor_demands(&self, phase: ArtifactStoreCursorDisposerPhase, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { ArtifactStore::close_cursor_demands(self, phase, body) }
    fn owned_roots_terminal_is_empty(&self) -> bool { ArtifactStore::owned_roots_terminal_is_empty(self) }
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
impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: Clone + PartialEq + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    /// 🪞️ Delegates to [`StoreLive::enable_convergence_early_exit`].
    #[inline] pub fn enable_convergence_early_exit(&mut self) { self.live.enable_convergence_early_exit() }
}

/// 🏁️ How a finished replay's drafts are authored: overwrite them in every alternative and checkpoint containing the
/// operations, supersede them within the existing alternative `alternative_id` only (the line the replay was viewed on — the
/// undo or redo of a finalize), or keep the originals and branch a new alternative `name` at the head that supersedes them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HistoryFinalization {
    Overwrite,
    Scope { alternative_id: String },
    Alternative { name: String },
}

/// ✏️ Where an authored supersession applies: every alternative, one existing alternative, or a new alternative branched at
/// the head.
pub enum SupersedeScope {
    Document,
    Alternative(String),
    NewAlternative(String),
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
impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    fn resident_backings_terminal_is_empty(&self) -> bool {
        self.log.dag.backing_is_empty() && self.log.applied_edit_ids.capacity() == 0 && self.log.redo_edit_ids.capacity() == 0 && self.cursor.revision_accumulator.applied.capacity() == 0 && self.cursor.revision_accumulator.redo.capacity() == 0 && self.owners.displaced_retirements.owners.capacity() == 0 && self.live.prefix_ring.capacity() == 0 && self.wire.member_inbox.capacity() == 0 && self.replay.pending_report.messages.capacity() == 0 && self.replay.pending_report.outbound.capacity() == 0 && self.replay.pending_report.edit_ids.as_ref().is_none_or(|ids| ids.capacity() == 0) && self.owners.snapshot_read_leases.empty_backing_demands().is_ok_and(|demand| demand.release_bytes == 0)
    }
}

impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    /// 🪞️ Delegates to [`StoreOwners::factory_aliases_present`].
    #[inline] pub fn factory_aliases_present(&self) -> bool { self.owners.factory_aliases_present() }
}

impl<P, Mutation> Drop for ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    /// 🪦️ Exact terminal-empty shallow-shell witness, strict on the ordinary path — the store must
    /// have been retired down to an empty shell before it may be dropped.
    ///
    /// 🪲️ `|| std::thread::panicking()` is not a weakening of that witness: a `Drop` that panics
    /// while an earlier panic is already unwinding is a double panic, which Rust escalates to
    /// `panic in a destructor during cleanup` / `thread caused non-unwinding panic. aborting.` —
    /// killing the whole test process and erasing libtest's summary for every other test in the
    /// binary. Without this guard ANY failing assertion that still holds a live `ArtifactStore`
    /// reports as a `SIGABRT` with no counts instead of as one ordinary test failure. See
    /// `📓️fable-mcp-artifact-quick-recursion.md` §"the amplifier".
    fn drop(&mut self) {
        let terminal = self.log.envelope_detached
            && self.live.current_detached
            && self.wire.backbone.is_none()
            && self.log.dag.terminal_is_empty()
            && self.log.applied_edit_ids.is_empty()
            && self.log.redo_edit_ids.is_empty()
            && self.log.current_checkpoint_id.is_none()
            && self.log.local_actor_id.0.is_empty()
            && self.cursor.revision_accumulator.applied.is_empty()
            && self.cursor.revision_accumulator.redo.is_empty()
            && self.cursor.revision_accumulator.mutation_positions.is_empty()
            && self.cursor.revision_accumulator.indexed_edits.is_empty()
            && self.cursor.revision_accumulator.unit_flags.is_empty()
            && self.live.tail_undo_cache.is_none()
            && self.live.supersessions.is_empty()
            && self.live.prefix_ring.is_empty()
            && self.owners.snapshot_read_leases.terminal_is_empty()
            && self.owners.displaced_retirements.terminal_is_empty()
            && self.owners.owned_disposer.is_none()
            && self.owners.owned_disposer_terminal
            && self.resident_backings_terminal_is_empty()
            && self.owners.snapshot_read_leases.closed()
            && !self.factory_aliases_present()
            && self.owners.factory_retirement_tickets.terminal_is_empty()
            && self.replay.pending_report.edit_ids.is_none()
            && self.replay.pending_report.messages.is_empty()
            && self.replay.pending_report.outbound.is_empty()
            && self.replay.pending_report.worst.is_none()
            && self.wire.durable_group_root.is_none();
        assert!(terminal || std::thread::panicking(), "artifact store reached Drop without its exact terminal-empty shallow-shell witness");
        unsafe {
            drop(std::mem::ManuallyDrop::take(&mut self.wire.backbone));
            drop(std::mem::ManuallyDrop::take(&mut self.log.dag));
            drop(std::mem::ManuallyDrop::take(&mut self.log.applied_edit_ids));
            drop(std::mem::ManuallyDrop::take(&mut self.log.redo_edit_ids));
            drop(std::mem::ManuallyDrop::take(&mut self.log.current_checkpoint_id));
            drop(std::mem::ManuallyDrop::take(&mut self.log.local_actor_id));
            drop(std::mem::ManuallyDrop::take(&mut self.cursor.revision_accumulator));
            drop(std::mem::ManuallyDrop::take(&mut self.live.tail_undo_cache));
            drop(std::mem::ManuallyDrop::take(&mut self.live.supersessions));
            drop(std::mem::ManuallyDrop::take(&mut self.live.prefix_ring));
            drop(std::mem::ManuallyDrop::take(&mut self.owners.snapshot_retirement_factory));
            drop(std::mem::ManuallyDrop::take(&mut self.owners.initial_snapshot_retirement_factory));
            drop(std::mem::ManuallyDrop::take(&mut self.owners.mutation_retirement_factory));
            drop(std::mem::ManuallyDrop::take(&mut self.owners.snapshot_read_leases));
            drop(std::mem::ManuallyDrop::take(&mut self.owners.displaced_retirements));
            drop(std::mem::ManuallyDrop::take(&mut self.owners.owned_disposer));
            drop(std::mem::ManuallyDrop::take(&mut self.owners.one_item_preparation_factory));
            drop(std::mem::ManuallyDrop::take(&mut self.owners.replay_preparation_factory));
            drop(std::mem::ManuallyDrop::take(&mut self.owners.one_item_wire_preparation_factory));
            drop(std::mem::ManuallyDrop::take(&mut self.owners.factory_retirement_tickets));
            drop(std::mem::ManuallyDrop::take(&mut self.replay.pending_report));
            drop(std::mem::ManuallyDrop::take(&mut self.wire.durable_group_root));
        }
    }
}

pub async fn preflight_merge_by_id<T: PartialEq>(local: &[T], remote: &[T], id_of: impl Fn(&T) -> &String) -> Result<(), VcsError> {
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

pub async fn merge_by_id<T: Clone + PartialEq>(local: &mut Vec<T>, remote: Vec<T>, id_of: impl Fn(&T) -> &String) -> Result<(), VcsError> {
    preflight_merge_by_id(local, &remote, &id_of).await?;
    let mut existing: HashSet<String> = local.iter().map(|item| id_of(item).clone()).collect();
    for item in remote {
        if existing.insert(id_of(&item).clone()) {
            local.push(item);
        }
    }
    Ok(())
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
/// 🪒️ The `.spr` byte log without the transitions `refused` retracts ([`transition_retraction_closure`]) — what a sync
/// actor persists after the hub refused them — and the retracted ids.
pub async fn retract_history_transitions_from_spr(spr: &[u8], refused: &[String]) -> Result<(Vec<u8>, Vec<String>), VcsError> {
    let mut log = semio_framework_os_spr::decode_history(spr, &semio_framework_os_spr::DecodeOptions::default()).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let retracted = transition_retraction_closure(&log.transitions, |record| record.id.as_str(), |record| record.dependencies.iter().map(String::as_str).collect(), refused);
    log.transitions.retain(|record| !retracted.contains(&record.id));
    let options = semio_framework_os_spr::EncodeOptions { write_backwards_section: true, ..semio_framework_os_spr::EncodeOptions::default() };
    let bytes = semio_framework_os_spr::encode_history(&log, &options).await.map_err(|error| VcsError::Serialize(error.to_string()))?;
    Ok((bytes, retracted))
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
//#region 🔖️PairMerge
impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    /// 🫸️ [`Self::merge_persisted_history`] over the pair's undecoded `.spr` bytes.
    pub async fn merge_persisted_pair(&mut self, pack: &[u8], spr: &[u8]) -> Result<PairMerge, VcsError> {
        let history = semio_framework_os_spr::decode_history(spr, &semio_framework_os_spr::DecodeOptions::default()).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
        self.merge_persisted_history(pack, history).await
    }

    /// 🪭️ Merges the event log of a persisted pair — the genesis `pack` and its decoded `history`, another writer's copy of
    /// THIS document — into this live store. Every operation and history transition the store lacks is ingested exactly like
    /// a remote event ([`Self::ingest_remote`]): this replica keeps its head, its own edits and whatever replay is in flight,
    /// and the pair's viewer head is never read — only a cold load ([`parse_document_pack`]) stands where a pair says. A pair
    /// of another document (id, schema or genesis) is refused before anything is parsed: replacing a document is a load, not
    /// a merge. An ingest refusal stops the merge; what was taken before it stays, as with any remote batch (every ingest is
    /// atomic).
    pub async fn merge_persisted_history(&mut self, pack: &[u8], history: semio_framework_os_spr::HistoryLog) -> Result<PairMerge, VcsError> {
        self.ensure_durable_group_idle()?;
        if history.doc_id != self.log.envelope.id || history.schema != self.log.envelope.schema || artifact_initial_digest_of_pack(pack) != self.log.initial_digest {
            return Err(VcsError::ValidationFailed(format!("the persisted pair holds document {}, not {}: a merge takes this document's own log only", history.doc_id, self.log.envelope.id)));
        }
        let ParsedDocumentText { envelope: foreign, snapshot } = parse_decoded_document_spr::<P, Mutation>(pack, history, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
        retire_replayed_projection::<P, Mutation>(snapshot);
        let mut failure = None;
        let mut events = Vec::new();
        for edit in foreign.vcs.edits.iter() {
            match semio_framework_os_spr::mutation_envelopes_from_edit_since::<P, Mutation>(edit, 0, &ArtifactId(foreign.id.clone()), &SchemaId(foreign.schema.clone())) {
                Ok(envelopes) => events.extend(envelopes),
                Err(error) => {
                    failure = Some(VcsError::Serialize(error.to_string()));
                    break;
                }
            }
        }
        events.extend(foreign.transitions.iter().cloned());
        foreign.retire_unadopted();
        if let Some(error) = failure {
            return Err(error);
        }
        let known: HashSet<MutationId> = self.event_log()?.into_iter().map(|event| event.mutation_id).collect();
        let offered: HashSet<MutationId> = events.iter().map(|event| event.mutation_id.clone()).collect();
        let mut merge = PairMerge { merged: 0, ahead: known.iter().filter(|id| !offered.contains(*id)).count() };
        for event in events.into_iter().filter(|event| !known.contains(&event.mutation_id)) {
            if self.ingest_remote(event).await?.accepted {
                merge.merged += 1;
            }
        }
        Ok(merge)
    }
}
//#endregion 🔖️PairMerge
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
