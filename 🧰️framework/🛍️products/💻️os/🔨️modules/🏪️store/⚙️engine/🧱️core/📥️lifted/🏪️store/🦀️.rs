//! 📥️ Items of `store` that belong to `os-store-engine-core` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🦀️.rs`.
use semio_framework_os_store_text_io::ArtifactStoreSnapshot as _;
use semio_framework_os_vcs::io::binary::genesis::AdmittedArtifactGenesis;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_os_store_replay::replay_preparation;
use replay_preparation::ArtifactReplayPreparationFactory;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_os_spr::{ActorId, ArtifactId, HybridLogicalTimestamp, MutationId, SchemaId};
use semio_framework_os_spr::{Edit, Mutation, MutationDiff, MutationMeta, OpBinary, OpText};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::marker::PhantomData;
use std::sync::Arc;
use semio_framework_os_store_read::{ArtifactDerivedHistoryPreview, ArtifactDerivedReplay, ArtifactDerivedSnapshot, ErasedSnapshotRead};
use semio_framework_os_store_engine_owners::DocumentStoreOwners;
use semio_framework_os_store_read::{ArtifactHistoryReadRetirement, ArtifactStoreCursorRetirement, ArtifactStoreDisplacedRetirements, ArtifactStoreEnvelopeRetirement, ArtifactStoreHistoryMetadataRetirement, ArtifactStoreMutationDagRetirement, ArtifactStoreRevisionAccumulatorRetirement, ArtifactStoreStringRetirement, ArtifactStoreStringVectorRetirement, DISPLACED_RELIEF_STEPS, HistoryReadPlan, SnapshotReadLeaseRefusal, SnapshotReadLeaseRegistryOwner};
use semio_framework_os_store_read::history_read_admission;
use history_read_admission::PreparedHistoryReadRetirement;
use semio_framework_value_snapshot_read::SnapshotReadRegistryHandle;
use semio_framework_os_spr::command::MutationMessageLedgerRetirement as ArtifactStoreMessageLedgerRetirement;
use semio_framework_os_spr::conflict::ProtocolConflictRetirement as ArtifactStoreConflictRetirement;
use semio_framework_value::factory::FactorySharedRetirement as ReturnedSnapshotReadRetirement;
use semio_framework_os_store_text_io::operation_identity;
use semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
use semio_framework_os_vcs::io::binary::entity_identity::{mint_alternative_id, mint_change_id};
use semio_framework_os_vcs::io::binary::checkpoint_identity::content_addressed_checkpoint_id_with_pending_change;
use semio_framework_os_vcs::{Author, VcsError};
use semio_framework_os_vcs::{Alternative, ArtifactHistoryKey, ArtifactHistoryLedger, ArtifactVcs, Change, Checkpoint};
use semio_framework_os_store_space_schema::{AcceptedArtifactProjection, ArtifactBackboneRef, ArtifactCursor, ArtifactEnvelope, ArtifactProjectionCause, ArtifactProjectionInvalidation, ArtifactProjectionResult, ArtifactProjectionStamp, ArtifactRevision, HistoryLane, OpenToolTransaction, StaleArtifactProjection};
use semio_framework_os_store_replay::PendingReprojection;
use semio_framework_os_store_envelope::owner::OwnerRef;
use semio_framework_os_store_child::{owner_ref_from_record, owner_ref_spec_producer, owner_ref_to_record};
//#region 🔖️EphemeralLanes

use semio_framework_os_store_ephemeral::*;
use semio_framework_os_store_text_io::ArtifactPackFiles;
use semio_framework_os_store_owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
use pack::snapshot::ArtifactPack;
use semio_framework_os_store_text_io::ArtifactStoreSnapshot;
use crate::lifted_document_materialize::{EMPTY_CHECKPOINT_MESSAGE, now_iso};
use semio_framework_os_store_initialization_runtime::lifted_document_materialize::now_ms;
use crate::lifted_document_materialize::uncommitted_edit_ids;
use semio_framework_os_store_text_io::text_format::print_document_pack;
use semio_framework_os_store_text_io::text_format::mutation_ids;
use semio_framework_os_store_command::OpsSupersededInput;
use semio_framework_os_store_ledger::{ArtifactEditMessageLedger, fold_envelope_history, fold_event_log};
use semio_framework_os_store_ledger::{ARTIFACT_EDIT_MESSAGE_ID_BYTES, ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY, ArtifactStoreHistoryCommitReservation, CursorRevisionAccumulator, ReplayProjection, StackMirrorDirt, edit_timestamp, envelope_viewer_head, operation_semantic_kind, retire_replayed_projection, retire_scratch_operations};
#[cfg(test)]
use semio_framework_os_store_ledger::hot_path_census;
use semio_framework_os_store_one_item_batch::lifted_one_item_publication::{ArtifactStoreBatchAdmissionRejected, ArtifactStoreBatchPublication};
use semio_framework_os_store_one_item_publication::{ArtifactStoreOneItemAdvance, ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPreparationFactory, ArtifactStoreOneItemPrepared, ArtifactStoreOneItemPublicationPhase};
use semio_framework_os_store_one_item_publication::lifted_publication::MemberStoreOneItemWirePreparationFactory;
use semio_framework_os_store_one_item_publication::{ArtifactStoreBatchItemAuthority, ArtifactStoreBatchSourceOf, ArtifactTypedBatchItemAuthority};
use semio_framework_os_store_one_item_batch::lifted_one_item_publication::ArtifactStoreBatchStage;
use semio_framework_os_store_one_item_publication::prepared_retirement::ArtifactStorePreparedRetirement;
use semio_framework_os_store_operation_wire::{ArtifactOperationWire, ArtifactPreparedOperationSource};
use semio_framework_os_store_one_item::{ARTIFACT_STORE_ONE_ITEM_ID_BYTES, ArtifactStoreOneItemCheckpoint, ArtifactStoreOneItemFootprint, ArtifactStoreOneItemGrant};
use semio_framework_os_store_replay::{EditReplay, EditReplayResult, EffectiveSupersessions, ReplayProgress, apply_operation};
use semio_framework_os_store_replay::{REPROJECTION_OWNER_SLOTS, fold_effective_edit, mirror_id_suffix, order_ledger_as};
//#region 🔖️Backbone
//#region 🔖️Backbone
use semio_framework_os_store_backbone as backbone;
use backbone::{BackboneMessage, Backbones};
use semio_framework_os_store_backbone::backbone_retirement::ArtifactStoreBackboneRetirement;
use semio_framework_os_store_engine_owners::lifted_store::ArtifactStoreOwnedDisposer;
use semio_framework_os_store_initialization_runtime::lifted_store::bound_edit_messages;
//#region 🔖️Composition
//#region 🔖️CompositionDsl
impl semio_framework_dsl_record::DslField for OwnerRef {
    /// 🚫️async: E4 — see `DslField::shape`'s tag on the trait.
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Record(owner_ref_spec_producer())
    }
    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, ValueError> {
        control.checkpoint()?;
        Ok(semio_framework_dsl_record::Shape::Record(owner_ref_spec_producer()))
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Record(owner_ref_to_record(self))
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Record(record) => owner_ref_from_record(record),
            other => Err(format!("expected Record, found {other:?}")),
        }
    }
}


/// 🧾 Receipt from the sole store write gate (`dispatch` / `reset`). `messages`/`worst` carry
/// whatever `crate::os_spr::MutationMessage`s the command's own replay produced (empty/`None` for a
/// structural command with nothing to report) — `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-
/// FIRST-CLASS-CONFLICTS` §C6. `Eq` dropped (was `#[derive(.., PartialEq, Eq)]`): `EditMessages`
/// only derives `PartialEq`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CommandReceipt {
    pub edit_ids: Vec<String>,
    pub generation: u64,
    pub messages: Vec<semio_framework_os_spr::EditMessages>,
    pub worst: Option<semio_framework_diagnostic::Severity>,
}


/// 👁️ Read-only view over a document envelope — mutation is sealed through `dispatch`/`reset`.
#[derive(Clone, Copy, Debug)]
pub struct ArtifactEnvelopeView<'a, P, Mutation> {
    envelope: &'a ArtifactEnvelope<P, Mutation>,
}

impl<'a, P, Mutation> ArtifactEnvelopeView<'a, P, Mutation> {
    pub async fn schema(&self) -> &str {
        &self.envelope.schema
    }
    pub async fn id(&self) -> &str {
        &self.envelope.id
    }
    pub async fn vcs(&self) -> &ArtifactVcs<AdmittedArtifactGenesis<P>, Mutation> {
        &self.envelope.vcs
    }
    pub async fn backbone(&self) -> Option<&ArtifactBackboneRef> {
        self.envelope.backbone.as_ref()
    }
    pub async fn active_alternative_id(&self) -> Option<&str> {
        self.envelope.active_alternative_id.as_deref()
    }
    pub async fn cursor(&self) -> Option<&ArtifactCursor> {
        self.envelope.cursor.as_ref()
    }
    pub async fn edit_messages(&self) -> &ArtifactEditMessageLedger {
        &self.envelope.edit_messages
    }
    pub async fn conflicts(&self) -> &[semio_framework_os_spr::Conflict] {
        &self.envelope.conflicts
    }
    pub async fn inner(&self) -> &'a ArtifactEnvelope<P, Mutation> {
        self.envelope
    }
}
//#endregion 🔖️CompositionDsl
//#endregion 🔖️Composition

//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️TextFormat

impl TryFrom<OpsSupersededInput> for semio_framework_os_spr::SupersededInput {
    type Error = String;

    fn try_from(input: OpsSupersededInput) -> Result<Self, String> {
        let replacement = match (input.schema, input.payload) {
            (Some(schema), Some(payload)) => semio_framework_os_spr::InputReplacement::Input { schema, payload: semio_framework_io_base64::base64_standard_decode(&payload).map_err(|error| format!("supersede input {} payload: {error}", input.target))? },
            (None, None) => semio_framework_os_spr::InputReplacement::Withdrawn,
            _ => return Err(format!("supersede input {} needs both schema and payload, or neither", input.target)),
        };
        Ok(Self { target: MutationId(input.target), replacement })
    }
}
//#endregion 🔖️TextFormat
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
//#region 🔖️CommandFormat

/// 📜️ One row of a checkpoint history/ancestor graph.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct HistoryColumn {
    pub checkpoint_id: String,
    pub timestamp: String,
    pub labels: Vec<String>,
    pub authors: Vec<Author>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub parent_checkpoint_id: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub lane: usize,
    pub alternative_ids: Vec<String>,
}

/// 🌿️ The branched alternatives of `envelope`: every listed alternative but the trunk, whose checkpoints are the main line
/// the history view draws on lane 0.
pub fn branched_alternatives<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>) -> impl Iterator<Item = &Alternative> {
    let trunk = semio_framework_os_spr::trunk_alternative_id(&ArtifactId(envelope.id.clone()));
    envelope.vcs.alternatives.iter().filter(move |alternative| alternative.id != trunk)
}

pub async fn checkpoint_alternatives<'a, P, Mutation>(envelope: &'a ArtifactEnvelope<P, Mutation>, checkpoint_id: &str) -> Vec<&'a Alternative> {
    branched_alternatives(envelope).filter(|alternative| alternative.checkpoint_ids.iter().any(|id| id == checkpoint_id)).collect()
}

pub async fn is_checkpoint_main_only<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, checkpoint_id: &str) -> bool {
    checkpoint_alternatives(envelope, checkpoint_id).await.is_empty()
}

/// 🌀️ Self-recursive async fn — `Box::pin` at the recursive call site only (R10 shape 3):
/// the fn's own opaque future type would otherwise need to contain itself (E0733).
pub async fn has_main_only_descendant<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, children_of: &HashMap<String, Vec<String>>, checkpoint_id: &str, seen: &mut HashSet<String>) -> bool {
    if !seen.insert(checkpoint_id.to_string()) {
        return false;
    }
    for child_id in children_of.get(checkpoint_id).into_iter().flatten() {
        if is_checkpoint_main_only(envelope, child_id).await || Box::pin(has_main_only_descendant(envelope, children_of, child_id, seen)).await {
            return true;
        }
    }
    false
}

/// 🛤️ Assigns each checkpoint a swimlane: branched alternatives get lanes `1..n` in array order, lane
/// `0` is the main trunk. A checkpoint sits on lane 0 if it belongs to no alternative or has any
/// main-only descendant (cycle-guarded DFS); otherwise it takes its single alternative's lane, or
/// the minimum lane among several. Mirrors premigration `assignHistoryCheckpointLanes`.
pub async fn assign_history_checkpoint_lanes<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>) -> HashMap<String, usize> {
    let mut lane_by_alternative: HashMap<String, usize> = HashMap::new();
    for (index, alternative) in branched_alternatives(envelope).enumerate() {
        lane_by_alternative.insert(alternative.id.clone(), index + 1);
    }
    let mut children_of: HashMap<String, Vec<String>> = HashMap::new();
    for checkpoint in &envelope.vcs.checkpoints {
        if let Some(parent_id) = &checkpoint.parent_id {
            children_of.entry(parent_id.clone()).or_default().push(checkpoint.id.clone());
        }
    }
    let mut lane_by_checkpoint_id: HashMap<String, usize> = HashMap::new();
    for checkpoint in &envelope.vcs.checkpoints {
        if checkpoint.parent_id.is_none() {
            lane_by_checkpoint_id.insert(checkpoint.id.clone(), 0);
            continue;
        }
        let mut seen = HashSet::new();
        if is_checkpoint_main_only(envelope, &checkpoint.id).await || has_main_only_descendant(envelope, &children_of, &checkpoint.id, &mut seen).await {
            lane_by_checkpoint_id.insert(checkpoint.id.clone(), 0);
            continue;
        }
        let alternatives = checkpoint_alternatives(envelope, &checkpoint.id);
        let lanes: Vec<usize> = alternatives.await.iter().map(|alternative| *lane_by_alternative.get(&alternative.id).unwrap_or(&0)).collect();
        let lane = if lanes.len() == 1 { lanes[0] } else { lanes.into_iter().min().unwrap_or(0) };
        lane_by_checkpoint_id.insert(checkpoint.id.clone(), lane);
    }
    lane_by_checkpoint_id
}

/// 📜️ Builds the ancestor-graph rows for a checkpoint history view: newest checkpoint first,
/// each carrying its swimlane, labels (alternative names, `"main"` fallback on the newest unlabeled
/// row), and authors. Mirrors premigration `buildHistoryColumns`.
pub async fn build_history_columns<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>) -> Vec<HistoryColumn> {
    let lane_by_checkpoint_id = assign_history_checkpoint_lanes(envelope).await;
    let mut columns = Vec::with_capacity(envelope.vcs.checkpoints.len());
    for (index, checkpoint) in envelope.vcs.checkpoints.iter().rev().enumerate() {
        let alternatives = checkpoint_alternatives(envelope, &checkpoint.id).await;
        let alternative_ids: Vec<String> = alternatives.iter().map(|alternative| alternative.id.clone()).collect();
        let mut labels: Vec<String> = alternatives.iter().map(|alternative| alternative.name.clone()).collect();
        if labels.is_empty() && index == 0 {
            labels.push("main".into());
        }
        columns.push(HistoryColumn {
            checkpoint_id: checkpoint.id.clone(),
            timestamp: checkpoint.timestamp.clone(),
            labels,
            authors: checkpoint.authors.clone(),
            parent_checkpoint_id: checkpoint.parent_id.clone(),
            description: checkpoint.message.clone(),
            lane: *lane_by_checkpoint_id.get(&checkpoint.id).unwrap_or(&0),
            alternative_ids,
        });
    }
    columns
}
//#endregion 🔖️CommandFormat
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
/// 🔬️ Histories up to this many applied edits re-derive every incremental mirror from scratch after each test-build bump.
#[doc(hidden)]
pub const MIRROR_ORACLE_HISTORY: usize = 256;

/// 🔬️ The history length up to which the test oracle rebuilds every mirror on every bump; beyond it only every [`MIRROR_ORACLE_STRIDE`]th length is rebuilt, so a production-shaped history costs O(n²/stride) instead of O(n²).
#[doc(hidden)]
pub const MIRROR_ORACLE_EVERY_HISTORY: usize = 64;

/// 🔬️ The history-length stride of the sampled test oracle ([`MIRROR_ORACLE_EVERY_HISTORY`]).
#[doc(hidden)]
pub const MIRROR_ORACLE_STRIDE: usize = 32;
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
//#region 📬️OneItemPublication
//#region 🧩️MemberOneItemPublication

/// 📜️ The event-sourced history of one store: the envelope and its causal DAG, the applied and redo edit-id stacks, the checkpoint, actor, clock and merge policy.
pub struct StoreLog<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    pub envelope: std::mem::ManuallyDrop<ArtifactEnvelope<P, Mutation>>,
    pub envelope_detached: bool,
    pub dag: std::mem::ManuallyDrop<semio_framework_os_spr::MutationDag>,
    pub applied_edit_ids: std::mem::ManuallyDrop<semio_framework_os_vcs::HistoryPageStack<String>>,
    pub redo_edit_ids: std::mem::ManuallyDrop<semio_framework_os_vcs::HistoryPageStack<String>>,
    pub edit_sequence: i32,
    /// 🧭️ The checkpoint new commits parent onto; advances on commit/checkout/switch. Not
    /// part of the wire envelope — callers that reconstruct the store per call (e.g. a WASM plugin)
    /// must save/restore it themselves via {@link current_checkpoint_id}/{@link set_current_checkpoint_id}.
    pub current_checkpoint_id: std::mem::ManuallyDrop<Option<String>>,
    /// 🖋️ Identity of the local actor driving this store. Set from each local `Apply`/
    /// `Apply`'s operation author; compared against `Edit.actor` so undo never touches foreign
    /// edits. Not part of the wire envelope — callers that reconstruct the store per call must
    /// supply the opened actor at construction and preserve it across replay.
    pub local_actor_id: std::mem::ManuallyDrop<ActorId>,
    /// ⚖️ This store's local `crate::os_spr::MergePolicy` — authority-local state, defaults
    /// to `Normal`, never carried on the wire envelope (never part of an artifact's shared history).
    pub merge_policy: semio_framework_os_spr::MergePolicy,
    /// ⏰️ Monotone hybrid logical clock: `tick`s on every local apply, `merge`s in a remote
    /// tick on every ingest — replaces the old per-call `HybridLogicalTimestamp::new(0, now_ms())`
    /// construction in `replay_mutations`. Not part of the wire envelope.
    pub clock: HybridLogicalTimestamp,
    /// 🧬️ Stable identity of the immutable genesis content, computed once on load/reset rather
    /// than by a composing app's action dispatch.
    pub initial_digest: [u8; 32],
    /// 🪪️ The canonical operation wire of this store's mutation vocabulary ([`ArtifactOperationWire`]), installed once at construction.
    pub operation_wire: Option<ArtifactOperationWire<Mutation>>,
    /// 🏷️ The id of the action or command whose `Apply` this store is dispatching, stamped on the edit it mints (an
    /// amend keeps the verb of the edit it extends). Set by the dispatching runtime via
    /// {@link set_authoring_verb}; not part of the wire envelope.
    pub authoring_verb: Option<String>,
}

/// ♻️ The retirement discipline of one store: the installed retirement factories, the snapshot-read leases, the displaced-owner buffer and the owned disposer every value swap pays into.
pub struct StoreOwners<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    /// ♻️ Exact domain-owned bounded disposer for immutable snapshot roots transferred out
    /// of this store. Composition retirement fails closed until the owner installs it.
    pub snapshot_retirement_factory: std::mem::ManuallyDrop<Option<Arc<dyn SnapshotRetirementFactory<P>>>>,
    pub initial_snapshot_retirement_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>>,
    pub mutation_retirement_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>>>,
    pub snapshot_read_leases: std::mem::ManuallyDrop<SnapshotReadLeaseRegistryOwner>,
    pub displaced_retirements: std::mem::ManuallyDrop<ArtifactStoreDisplacedRetirements>,
    pub owned_disposer: std::mem::ManuallyDrop<Option<Box<dyn ArtifactStoreOwnedDisposer<P, Mutation>>>>,
    pub owned_disposer_terminal: bool,
    pub one_item_preparation_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>>>,
    pub replay_preparation_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactReplayPreparationFactory<P, Mutation>>>>,
    pub one_item_wire_preparation_factory: std::mem::ManuallyDrop<Option<Arc<dyn MemberStoreOneItemWirePreparationFactory<P, Mutation>>>>,
    pub factory_retirement_tickets: std::mem::ManuallyDrop<semio_framework_value::FactoryChildTickets<6>>,
}

/// 📡️ The transport of one store: the attached backbone, the member inbox and the durable-group root.
pub struct StoreWire<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    pub backbone: std::mem::ManuallyDrop<Option<Backbones>>,
    /// 📨️ Transient handoff from whichever `dispatch_inner` arm actually produced messages
    /// this call (`apply_command`/`ingest_remote`/`resolve_conflict`) to `dispatch`,
    /// which drains it into the returned `CommandReceipt`. Reset at the top of every `dispatch`
    /// call; every other arm leaves it at its `Default`. Not part of the wire envelope.
    /// 🪆️ Member-addressed backbone messages this store pumped off its own transport but does
    /// NOT own: a composed document's children are replicas of their own, and only the composing app
    /// knows which live member a `(slot, child_id)` lane names. Buffered here so a pump inside an
    /// ordinary `dispatch` can never silently drop a child's remote edit, drained by
    /// {@link take_member_inbound}, and retired with the transport when the store closes.
    pub member_inbox: VecDeque<BackboneMessage>,
    pub durable_group_root: std::mem::ManuallyDrop<Option<semio_framework_os_store_durable_group_wire::lifted_durable_group::ArtifactStoreDurableGroupRootV1<P, Mutation>>>,
}

/// 🧭️ The revision mirror of one store: the content revision, its accumulator and mirror dirt, the generation and the last projection cause.
pub struct StoreCursor<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    pub generation: u64,
    pub last_projection_cause: Option<ArtifactProjectionCause>,
    /// 🧬️ Fixed cursor digest refreshed at mutation boundaries and cloned by composition reads.
    pub content_revision: [u8; 32],
    /// 🧬️ Prefix-authenticated applied/redo cursor state. Tail mutations update one record;
    /// load/reset and the already-cold interior-history paths rebuild only their changed suffix.
    pub revision_accumulator: std::mem::ManuallyDrop<CursorRevisionAccumulator>,
    /// 📍️ The first applied position whose revision record a replay rewrote (effective input, inverse or messages);
    /// the next {@link bump} rebuilds the cursor revision from there.
    pub revision_dirty_from: Option<usize>,
    /// 🪞️ The first applied and redo positions whose ids changed since {@link bump} last mirrored them ([`StackMirrorDirt`]).
    pub mirror_dirt: StackMirrorDirt,
    pub marker: std::marker::PhantomData<fn() -> (P, Mutation)>,
}
//#endregion 🧩️MemberOneItemPublication
//#endregion 📬️OneItemPublication
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
/// 🪙️ The witness a live store log hands out for minting publication authorities; it has no public constructor.
pub struct StoreMintToken {
    _sealed: (),
}

impl one_item_publication::StoreMintWitness for StoreMintToken {}

impl<P, Mutation> StoreLog<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    /// 🪙️ The mint witness of this live log: what the engine presents to [`ArtifactStoreOneItemLiveAuthority::mint_for_store`].
    #[inline]
    pub fn mint_witness(&self) -> &'static StoreMintToken {
        &StoreMintToken { _sealed: () }
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
impl<P, Mutation> StoreLog<P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    /// ⏰️ Reads this Store's hybrid logical clock without suspension, so a server that mints a
    /// stamped publication clock can make it strictly after the document's own last edit instead of
    /// guessing from its wall clock.
    pub fn clock_now(&self) -> HybridLogicalTimestamp {
        self.clock
    }

    pub fn envelope(&self) -> &ArtifactEnvelope<P, Mutation> {
        &self.envelope
    }

    /// 👁️ Read-only envelope view — prefer this over mutating through public fields.
    pub fn envelope_view(&self) -> ArtifactEnvelopeView<'_, P, Mutation> {
        ArtifactEnvelopeView { envelope: &self.envelope }
    }

    /// 🧭️ The checkpoint new commits currently parent onto (defaults to the latest checkpoint
    /// on construction/`set_state`; advances on commit/checkout/switch).
    pub fn current_checkpoint_id(&self) -> Option<&str> {
        self.current_checkpoint_id.as_deref()
    }

    /// 🖋️ The local actor id used to distinguish this store's own edits from ingested ones.
    /// The constructor supplies this authority; replay and history authors preserve it.
    pub fn local_actor_id(&self) -> &ActorId {
        &self.local_actor_id
    }

    /// 🖋️ Binds the opened runtime instance actor explicitly; mutation authors remain durable edit metadata.

    /// 🏷️ The verb id the next locally minted edit is stamped with (see {@link set_authoring_verb}).
    pub fn authoring_verb(&self) -> Option<&str> {
        self.authoring_verb.as_deref()
    }

    /// 🏷️ Names the action or command whose `Apply` the caller dispatches next, so the edit it mints carries a
    /// locale-neutral label source every reload and peer resolves the same way; the caller clears it after dispatch.
    pub fn set_authoring_verb(&mut self, verb: Option<String>) {
        self.authoring_verb = verb.filter(|value| !value.is_empty());
    }

    /// 🎞️ The tool transaction open on this store, if any ([`ArtifactCommand::AppendTransaction`]).
    pub fn open_transaction(&self) -> Option<&OpenToolTransaction> {
        self.envelope.open_transaction.as_ref()
    }

    /// 🔧️ The most recently created/amended edit's `(forwards, inverse, per-operation meta)`.
    /// Used right after `dispatch(Apply{..})`/`AppendTransaction` to build a `KernelMutation`/`InvocationResult`
    /// with a true inverse from the just-recorded `Edit.inverse`.
    pub fn edit_mutations(&self) -> Option<(&[Mutation], &semio_framework_value::list::PagedList<Mutation, {usize::MAX}>, &[MutationMeta])> {
        self.envelope.vcs.edits.last().map(|edit| (edit.forwards.as_slice(), &edit.inverse, edit.mutation_meta.as_slice()))
    }

    /// 📜️ Ancestor-graph rows for this store's checkpoint history. See {@link build_history_columns}.
    pub async fn history_columns(&self) -> Vec<HistoryColumn> {
        build_history_columns(&self.envelope).await
    }

    /// 🎟️ Borrows the exact physical history cursor frame allocation before any owner transfers.
    pub fn history_read_retirement_birth_bytes()->usize {history_read_admission::history_read_retirement_birth_bytes::<P,Mutation>()}

    /// 🪪️ Installs a richer-than-catalog operation wire (the SDK's document stores carry the `(entity, kind)` schema pair), once, before the catalog.
    pub fn install_operation_wire(&mut self, wire: ArtifactOperationWire<Mutation>) {
        assert!(self.operation_wire.is_none(), "a store installs its canonical operation wire exactly once");
        self.operation_wire = Some(wire);
    }

    /// 🪪️ The canonical operation wire this store authors with, when one is installed.
    pub fn operation_wire(&self) -> Option<ArtifactOperationWire<Mutation>> {
        self.operation_wire
    }

    fn take_string_at_retained(target: &mut std::mem::ManuallyDrop<semio_framework_os_vcs::HistoryPageStack<String>>, position: usize) -> String {
        target.remove(position)
    }

    /// ♻️ Retires one snapshot alias a client held beside the store (a tool run's base or overlay, a folded intermediate):
    /// the last alias hands its snapshot to the exact owned-value retirement the store's own returned reads use, any other
    /// alias only drops its count. A client never drops an alias plainly — a snapshot whose roots must be retired (an
    /// `OrderedMap`) panics on a plain drop of its last owner.
    /// 📏️ Borrows the exact snapshot alias retirement frame before original ownership transfers.
    pub fn snapshot_alias_retirement_birth_bytes()->usize {ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes()}

    /// 🎯️ MEDIUM-3 (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` J1
    /// robustness pass): `envelope.conflicts` used to be appended on every rejection/degradation and
    /// truncated only incidentally (`resolve_conflict`'s `Quarantined`+`Accept` arm discarding the
    /// re-ingest's own transient conflicts). A peer that keeps sending a batch this replica keeps
    /// quarantining — `ingest_remote`'s dag never advances on quarantine, so the SAME envelope is
    /// eligible for redelivery forever — grows this list, and the persisted `.spr`, without bound.
    /// Two named caps, two different failure modes because `Open`/resolved conflicts carry different
    /// weight: a resolved (`Accepted`/`Discarded`) conflict is a closed historical fact a UI no
    /// longer needs enumerated, so it is PRUNABLE — `prune_resolved_conflicts` evicts the oldest
    /// (lowest push-index, i.e. first non-`Open` entry found scanning from the front) resolved
    /// conflicts once the total exceeds [`Self::RESOLVED_CONFLICT_CAP`]. An `Open` conflict is a
    /// pending decision a human/authority has not yet acted on — silently dropping one would erase
    /// unresolved history, so it is never evicted; instead [`Self::ensure_open_conflict_capacity`]
    /// is checked BEFORE any state mutation in every call site that might mint one, and refuses with
    /// a loud, typed `VcsError` (atomic — nothing applied) once open conflicts would exceed
    /// [`Self::OPEN_CONFLICT_CAP`], forcing existing ones to be resolved before backlog can grow
    /// further. `edit_messages` (the durable per-edit message ledger) was reviewed for the same
    /// pattern and deliberately gets NO cap: unlike `conflicts`, it only grows once per edit that
    /// ever reaches `committed_ids` (`record_edit_messages`/`replace_edit_messages` are never called
    /// for a quarantined id — its entry is explicitly cleared instead), so its size tracks genuine
    /// applied-history growth, not redelivery-of-rejected-content amplification; capping it would
    /// mean silently losing a still-applied edit's own diagnostic record, which is strictly worse.
    pub const OPEN_CONFLICT_CAP: usize = 256;

    /// 🎯️ MEDIUM-3: refuses to mint `additional_open` more `Open` conflicts once doing so would push
    /// this envelope past [`Self::OPEN_CONFLICT_CAP`]. Callers check this BEFORE mutating any store
    /// field for the call, so a refusal is atomic — see the const's own doc for why open conflicts
    /// are capped-but-never-silently-dropped rather than pruned like resolved ones.
    pub fn ensure_open_conflict_capacity(&self, additional_open: usize) -> Result<(), VcsError> {
        if additional_open == 0 {
            return Ok(());
        }
        let open_now = self.envelope.conflicts.iter().filter(|conflict| conflict.status == semio_framework_os_spr::ConflictStatus::Open).count();
        if open_now + additional_open > Self::OPEN_CONFLICT_CAP {
            return Err(VcsError::ValidationFailed(format!(
                "open conflict backlog is at capacity ({open_now} of {} open, {additional_open} more pending) — resolve existing open conflicts before this artifact can record another",
                Self::OPEN_CONFLICT_CAP
            )));
        }
        Ok(())
    }

    /// ⚔️ Every conflict this store has ever raised (`Open`, `Accepted`, and `Discarded`
    /// alike) — see {@link conflicts} field doc.
    pub fn conflicts(&self) -> &[semio_framework_os_spr::Conflict] {
        &self.envelope.conflicts
    }

    /// ⚔️ The subset of {@link conflicts} still `Open` — what a UI's Conflicts panel lists.
    pub fn open_conflicts(&self) -> impl Iterator<Item = &semio_framework_os_spr::Conflict> {
        self.envelope.conflicts.iter().filter(|conflict| conflict.status == semio_framework_os_spr::ConflictStatus::Open)
    }

    /// ⚖️ This store's local `crate::os_spr::MergePolicy` (see the field doc).
    pub fn merge_policy(&self) -> semio_framework_os_spr::MergePolicy {
        self.merge_policy
    }

    /// ⚖️ Sets this store's local `crate::os_spr::MergePolicy`. Local/authority state only —
    /// never wire-carried, never part of shared history; takes effect starting with the NEXT
    /// `dispatch`/`ingest_remote` call.
    pub fn set_merge_policy(&mut self, policy: semio_framework_os_spr::MergePolicy) {
        self.merge_policy = policy;
    }

    /// 📒️ Every `crate::os_spr::MutationMessage` `edit_id`'s own replay raised — empty for an
    /// edit that raised none (including an unknown `edit_id`), never an error (see the field doc).
    pub fn messages_for_edit(&self, edit_id: &str) -> &[semio_framework_os_spr::MutationMessage] {
        self.envelope.edit_messages.get_by_id(edit_id).map_or(&[], |entry| entry.messages.as_slice())
    }

    //#region 🔖️TimeTravelReads
    /// 🌳️ The id of this document's trunk — the implicit root line, a first-class alternative listed first in
    /// `envelope().vcs.alternatives` (with an empty name the UI localizes) once a commit made on it gives it a chain.
    pub fn trunk_alternative_id(&self) -> String {
        semio_framework_os_spr::trunk_alternative_id(&ArtifactId(self.envelope.id.clone()))
    }

    /// 🚉️ The line the document is on: the active branched alternative, or the trunk while none is.
    pub fn active_line_id(&self) -> String {
        self.envelope.active_alternative_id.clone().unwrap_or_else(|| self.trunk_alternative_id())
    }

    /// 🔖️ `alternative_id` as a `Checkout` names it: `None` for the trunk (the log never names it), the id otherwise.
    pub fn branched_alternative(&self, alternative_id: String) -> Option<String> {
        (alternative_id != self.trunk_alternative_id()).then_some(alternative_id)
    }

    /// 📖️ The edits a replay from {@link begin_report_replay} steps against.
    pub fn replay_edits(&self) -> &ArtifactHistoryLedger<Edit<Mutation>> {
        &self.envelope.vcs.edits
    }

    //#endregion 🔖️TimeTravelReads
    //#region 📬️OneItemPublication
    /// 🎥️ The edit a batch appends to instead of minting a ledger slot: the open edit of the tool transaction it carries
    /// (design §15). Outside a transaction every batch is its own edit.
    pub fn batch_amend_target_ref(&self, transaction: Option<&protocol::TransactionRef>) -> Option<&str> {
        self.envelope.open_transaction.as_ref().filter(|open| transaction.is_some_and(|transaction| open.transaction.id == transaction.id) && self.applied_edit_ids.last() == Some(&open.edit_id)).map(|open| open.edit_id.as_str())
    }

    pub fn batch_amend_target(&self, transaction: Option<&protocol::TransactionRef>) -> Option<String> {
        self.envelope.open_transaction.as_ref().filter(|open| transaction.is_some_and(|transaction| open.transaction.id == transaction.id) && self.applied_edit_ids.last() == Some(&open.edit_id)).map(|open| open.edit_id.clone())
    }

    pub fn cancel_apply_batch(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mutation>) -> bool {
        if matches!(publication.phase, ArtifactStoreOneItemPublicationPhase::Complete | ArtifactStoreOneItemPublicationPhase::Closing) {
            return false;
        }
        publication.cancel_requested = true;
        publication.begin_close();
        true
    }

    //#endregion 📬️OneItemPublication
    //#region 🔖️EventSourcedHistory
    /// 🪪️ The wire operations `edit_id` owns — the replica-neutral identity transitions name.
    pub fn edit_mutation_ids(&self, edit_id: &str) -> Result<Vec<MutationId>, VcsError> {
        let edit = self.envelope.vcs.edits.find_near(|edit| edit.id == edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.to_string()))?;
        Ok(semio_framework_os_spr::mutation_ids_for_edit::<P, Mutation>(edit))
    }

    /// 🌱️ The transition that brought `checkpoint_id` into being — a `Commit`, or the
    /// `Repin` that re-identified it — so a transition naming the checkpoint depends on it causally.
    pub fn checkpoint_origin(&self, checkpoint_id: &str) -> Option<MutationId> {
        self.envelope
            .transitions
            .iter()
            .rev()
            .find(|envelope| match semio_framework_os_spr::history_transition_from_envelope(envelope) {
                Ok(Some(semio_framework_os_spr::HistoryTransition::Commit(checkpoint))) => checkpoint.checkpoint_id == checkpoint_id,
                Ok(Some(semio_framework_os_spr::HistoryTransition::Repin { pinned_checkpoint_id, .. })) => pinned_checkpoint_id == checkpoint_id,
                _ => false,
            })
            .map(|envelope| envelope.mutation_id.clone())
    }

    /// 🫚️ The `Branch` transition that registered `alternative_id` — so a commit on that line depends on it causally and no
    /// replica folds the commit before it lists the line. The trunk has no registration.
    fn alternative_origin(&self, alternative_id: &str) -> Option<MutationId> {
        self.envelope
            .transitions
            .iter()
            .rev()
            .find(|envelope| matches!(semio_framework_os_spr::history_transition_from_envelope(envelope), Ok(Some(semio_framework_os_spr::HistoryTransition::Branch { alternative_id: registered, .. })) if registered == alternative_id))
            .map(|envelope| envelope.mutation_id.clone())
    }

    /// ✉️ `edit`'s operations as causal wire envelopes — one per forward operation.
    pub fn operation_envelopes(&self, edit: &Edit<Mutation>) -> Result<Vec<semio_framework_os_spr::MutationEnvelope>, VcsError> {
        self.operation_envelopes_since(edit, 0)
    }

    /// ✂️ The envelopes of `edit`'s operations from position `from` on: the range an amend appended.
    pub fn operation_envelopes_since(&self, edit: &Edit<Mutation>, from: usize) -> Result<Vec<semio_framework_os_spr::MutationEnvelope>, VcsError> {
        semio_framework_os_spr::mutation_envelopes_from_edit_since::<P, Mutation>(edit, from, &ArtifactId(self.envelope.id.clone()), &SchemaId(self.envelope.schema.clone())).map_err(|error| VcsError::Serialize(error.to_string()))
    }

    /// 👁️ The newest operation of an author other than `actor` this replica has applied — how far
    /// an operation `actor` authors now has seen the other authors' work. Remote edits join the ledger in
    /// arrival order, which is the hub's commit order; an undone edit was seen all the same.
    pub fn observed_foreign_operation(&self, actor: &str) -> Option<MutationId> {
        let edit = self.envelope.vcs.edits.iter().rev().find(|edit| edit.actor.as_deref().is_some_and(|author| author != actor))?;
        semio_framework_os_spr::mutation_ids_for_edit::<P, Mutation>(edit).pop()
    }

    /// 🌾️ Marks every recorded event applied in the causal graph, so a peer's later events that
    /// depend on them are never held back — whatever path (load, batched publication) recorded them.
    pub fn seed_known_events(&mut self) -> Result<(), VcsError> {
        let mut identities: Vec<MutationId> = self.envelope.transitions.iter().map(|transition| transition.mutation_id.clone()).collect();
        for edit in &self.envelope.vcs.edits {
            identities.extend(semio_framework_os_spr::mutation_ids_for_edit::<P, Mutation>(edit));
        }
        for identity in identities {
            match self.dag.seed_applied(identity) {
                Ok(()) => {}
                Err(rejected) if rejected.error == semio_framework_os_spr::MutationDagError::Duplicate => {}
                Err(rejected) => return Err(VcsError::ValidationFailed(rejected.error.to_string())),
            }
        }
        Ok(())
    }

    /// 🗂️ Refuses the first of `envelopes` whose transition this store's [`crate::os_spr::HistoryShape`] does not hold —
    /// with `HistoryShape` — or that does not decode, before any of them is recorded.
    pub fn admit_history_shape(&self, envelopes: &[semio_framework_os_spr::MutationEnvelope]) -> Result<(), VcsError> {
        let shape = self.envelope.history_shape;
        for envelope in envelopes {
            if let Some(kind) = semio_framework_os_spr::history_transition_from_envelope(envelope).map_err(|error| VcsError::Deserialize(error.to_string()))?.map(|transition| transition.kind()).filter(|kind| !shape.admits(*kind)) {
                return Err(VcsError::HistoryShape { shape, kind });
            }
        }
        Ok(())
    }

    /// 🖊️ The actor this replica authors its history transitions as.
    pub fn transition_actor(&self) -> ActorId {
        (*self.local_actor_id).clone()
    }

    /// 📧️ The envelope of one locally authored transition at `timestamp`, declaring `target` as the address it writes.
    pub fn transition_envelope(&self, transition: &semio_framework_os_spr::HistoryTransition, dependencies: Vec<MutationId>, target: Vec<String>, timestamp: HybridLogicalTimestamp) -> semio_framework_os_spr::MutationEnvelope {
        let actor = self.transition_actor();
        let mut envelope = semio_framework_os_spr::history_transition_envelope(transition, &ArtifactId(self.envelope.id.clone()), &actor, dependencies, timestamp);
        if matches!(transition, semio_framework_os_spr::HistoryTransition::Supersede(_)) {
            envelope.observed = self.observed_foreign_operation(&actor.0);
        }
        envelope.target = target;
        envelope
    }

    /// 🧮️ The full fold of the event log at this replica's head ([`fold_envelope_history`]) — O(history), so never on the
    /// path of a local edit that joins the fold as its tail ([`Self::folded_tail`]).
    pub fn fold_live_log(&self) -> Result<semio_framework_os_spr::HistoryFold, VcsError> {
        #[cfg(test)]
        hot_path_census::count(|census| census.folds += 1);
        fold_envelope_history(&self.envelope)
    }

    /// 🧭️ The greatest `(hlc)` key of every event in the log: the transitions are kept in key order, the edits are not.
    pub fn event_frontier(&self) -> (u64, u64, u64) {
        let transitions = self.envelope.transitions.last().map(|transition| transition.timestamp.cmp_key());
        self.envelope.vcs.edits.iter().map(|edit| edit_timestamp(edit).cmp_key()).chain(transitions).max().unwrap_or_default()
    }

    //#endregion 🔖️EventSourcedHistory
    //#region 🔖️ToolTransactions
    /// 🧿️ The open tool transaction when `transaction_id` names it, else `UnknownTransaction`.
    pub fn open_transaction_named(&self, transaction_id: &str) -> Result<OpenToolTransaction, VcsError> {
        self.envelope.open_transaction.clone().filter(|open| open.transaction.id == transaction_id).ok_or_else(|| VcsError::UnknownTransaction(transaction_id.to_string()))
    }

    //#endregion 🔖️ToolTransactions
    //#region 🔖️DeferredReprojection
    /// 🪐️ The history fold of the event log with a waiting change joined — its remote transitions, and its local step's
    /// transitions at the head that step moves to — touching nothing.
    pub fn pending_fold(&self, pending: &PendingReprojection<P, Mutation>) -> Result<semio_framework_os_spr::HistoryFold, VcsError> {
        let local = pending.local.as_ref();
        let candidates: Vec<semio_framework_os_spr::MutationEnvelope> = pending.transitions.iter().chain(local.map_or(&[][..], |local| local.envelopes.as_slice())).cloned().collect();
        let head = local.map_or_else(|| envelope_viewer_head(&self.envelope), |local| self.viewer_head(&local.head));
        self.prospective_fold_at(&candidates, &head)
    }

    //#endregion 🔖️DeferredReprojection
    //#region 🔖️Supersede
    /// 🪴️ Authors into `envelopes` the commit of every pending edit (when one is pending or no checkpoint exists) and a
    /// `Branch` of `name` at the resulting head, answering the new alternative's id.
    pub async fn author_head_alternative(&self, name: String, clock: &mut HybridLogicalTimestamp, envelopes: &mut Vec<semio_framework_os_spr::MutationEnvelope>, identity: &mut EntityIdentityAuthority<'_>) -> Result<String, VcsError> {
        let pending = uncommitted_edit_ids(&self.envelope, &*self.applied_edit_ids).await;
        let checkpoint_id = if !pending.is_empty() || self.envelope.vcs.checkpoints.is_empty() {
            if self.applied_edit_ids.is_empty() {
                return Err(VcsError::NoCheckpoint);
            }
            let (transition, dependencies, checkpoint_id) = self.pending_checkpoint_transition(None, Vec::new(), identity).await?;
            clock.tick(now_ms());
            envelopes.push(self.transition_envelope(&transition, dependencies, Vec::new(), *clock));
            checkpoint_id
        } else {
            (*self.current_checkpoint_id).clone().or_else(|| self.envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone())).ok_or(VcsError::NoCheckpoint)?
        };
        let alternative_id = identity.encode(|control|mint_alternative_id(&name, std::slice::from_ref(&checkpoint_id), control))?;
        if self.envelope.vcs.alternatives.iter().any(|alternative| alternative.id == alternative_id) {
            return Err(VcsError::ValidationFailed(format!("alternative {alternative_id} already exists")));
        }
        let dependencies = match envelopes.last() {
            Some(commit) => vec![commit.mutation_id.clone()],
            None => self.checkpoint_origin(&checkpoint_id).into_iter().collect(),
        };
        clock.tick(now_ms());
        envelopes.push(self.transition_envelope(&semio_framework_os_spr::HistoryTransition::Branch { alternative_id: alternative_id.clone(), name, checkpoint_id }, dependencies, Vec::new(), *clock));
        Ok(alternative_id)
    }

    /// 🔮️ The history fold of the event log with `candidates` joined, touching nothing.
    pub fn prospective_fold(&self, candidates: &[semio_framework_os_spr::MutationEnvelope]) -> Result<semio_framework_os_spr::HistoryFold, VcsError> {
        self.prospective_fold_at(candidates, &envelope_viewer_head(&self.envelope))
    }

    /// 🧐️ [`Self::prospective_fold`] for `head` instead of this replica's head.
    pub fn prospective_fold_at(&self, candidates: &[semio_framework_os_spr::MutationEnvelope], head: &semio_framework_os_spr::ViewerHead) -> Result<semio_framework_os_spr::HistoryFold, VcsError> {
        let mut transitions = self.envelope.transitions.clone();
        transitions.extend(candidates.iter().cloned());
        fold_event_log::<P, Mutation>(&self.envelope.id, &self.envelope.vcs.edits.iter().collect::<Vec<_>>(), &transitions, &self.envelope.conflicts, head)
    }

    //#endregion 🔖️Supersede
    //#region 🔖️HistoryLane
    /// 🛤️ The lane `edit_id` was recorded under. Absence from `envelope.lanes` means
    /// `Document` by construction — `apply_command` below only ever inserts a map
    /// entry for a NON-`Document` lane, so an ordinary document edit (and every edit that predates
    /// this field) never gets one. `HistoryLane` is `Copy`, so this returns by value like
    /// `edit_is_local` returns `bool`.
    pub fn edit_lane(&self, edit_id: &str) -> HistoryLane {
        self.envelope.lanes.get(edit_id).copied().unwrap_or_default()
    }

    /// 🛤️ Nearest-to-top redo entry this actor authored whose lane satisfies `lane`. Another
    /// actor's reverted edit sits on the shared redo stack too, but only its author may reinstate it.
    pub fn redo_position(&self, lane: impl Fn(HistoryLane) -> bool) -> Option<usize> {
        self.redo_edit_ids.iter().rposition(|id| self.edit_is_local(id) && lane(self.edit_lane(id)))
    }

    /// 👀️ This replica's head `(line, checkpoint)` as the envelope holds it.
    pub fn viewer_head_now(&self) -> (Option<String>, Option<String>) {
        (self.envelope.active_alternative_id.clone(), self.envelope.viewer_checkpoint_id.clone())
    }

    /// 🧿️ The fold head of `(line, checkpoint)`: an absent line (or the trunk's id) is the trunk.
    pub fn viewer_head(&self, (line, checkpoint): &(Option<String>, Option<String>)) -> semio_framework_os_spr::ViewerHead {
        let trunk = self.trunk_alternative_id();
        semio_framework_os_spr::ViewerHead { line_id: line.clone().filter(|line| *line != trunk).unwrap_or(trunk), checkpoint_id: checkpoint.clone() }
    }

    /// 🚩️ The `Commit` transition (with its dependencies and new checkpoint id) grouping every applied-but-uncommitted edit.
    pub async fn pending_checkpoint_transition(&self, message: Option<String>, authors: Vec<Author>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(semio_framework_os_spr::HistoryTransition, Vec<MutationId>, String), VcsError> {
        let pending = uncommitted_edit_ids(&self.envelope, &*self.applied_edit_ids).await;
        if pending.is_empty() {
            return Err(VcsError::ValidationFailed(EMPTY_CHECKPOINT_MESSAGE.to_string()));
        }
        let change_id = identity.encode(|control|mint_change_id(&pending, message.as_deref(), control))?;
        let saved_at = now_iso();
        let parent = self.current_checkpoint_id.as_ref().and_then(|id| self.envelope.vcs.checkpoints.iter().find(|checkpoint| checkpoint.id == *id));
        let mut change_ids = parent.map(|checkpoint| checkpoint.change_ids.clone()).unwrap_or_default();
        let parent_id = parent.map(|checkpoint| checkpoint.id.clone());
        change_ids.push(change_id.clone());
        let timestamp = now_iso();
        let checkpoint_id = content_addressed_checkpoint_id_with_pending_change(parent_id.as_deref(), &change_ids, &self.envelope.vcs.changes, &change_id, &pending, message.as_deref(), &saved_at, message.as_deref(), &authors, &timestamp, &[]);
        let mut mutation_ids = Vec::new();
        for edit_id in &pending {
            mutation_ids.extend(self.edit_mutation_ids(edit_id)?);
        }
        let mut dependencies = mutation_ids.clone();
        dependencies.extend(parent_id.as_deref().and_then(|parent_id| self.checkpoint_origin(parent_id)));
        dependencies.extend(self.envelope.active_alternative_id.as_deref().and_then(|line_id| self.alternative_origin(line_id)));
        let authors = authors.into_iter().map(|author| semio_framework_os_spr::TransitionAuthor { id: author.id, name: author.name, avatar: author.avatar }).collect();
        let checkpoint =
            semio_framework_os_spr::TransitionCheckpoint { checkpoint_id: checkpoint_id.clone(), parent_id, change_id, mutation_ids, description: message.clone(), saved_at, authors, message, timestamp, line_id: self.envelope.active_alternative_id.clone() };
        Ok((semio_framework_os_spr::HistoryTransition::Commit(checkpoint), dependencies, checkpoint_id))
    }

    //#endregion 🔖️HistoryLane
    /// 📸️ The whole document as its persisted `pack` (genesis) + `spr` (event log) bytes —
    /// for export and archive persistence; replicas never exchange it (they exchange events).
    pub async fn snapshot_pack(&self) -> Result<ArtifactPackFiles, VcsError>
    where P: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot,
    {
        print_document_pack(&self.envelope).await
    }

    /// 📦️ Serializes the full document envelope (snapshot + VCS history) as JSON over the
    /// first-party `ToValue` bridge. `capture_read()` stays fallible (a group-visibility
    /// consistency check); the resulting `ArtifactEnvelopeRead` is then infallibly convertible —
    /// see its own `impl ToValue` docstring.
    pub fn envelope_json(&self) -> Result<String, VcsError>
    where
        P: ToValue,
        Mutation: ToValue,
    {
        let read = self.envelope.capture_read().map_err(|e| VcsError::Serialize(e.to_string()))?;
        Ok(semio_framework_pack_json::to_json_string(&read))
    }

    /// 📜️ Every event of this replica's log as causal envelopes: each edit's operations in
    /// ledger order, then every history transition in HLC order. An open tool transaction's edit is not in the shared log
    /// before its commit.
    pub fn event_log(&self) -> Result<Vec<semio_framework_os_spr::MutationEnvelope>, VcsError> {
        let mut events = Vec::new();
        for edit in self.envelope.vcs.edits.iter().filter(|edit| self.envelope.holds_committed_edit(&edit.id)) {
            events.extend(self.operation_envelopes(edit)?);
        }
        events.extend(self.envelope.transitions.iter().cloned());
        Ok(events)
    }

    /// 📦️ This replica's whole event log as ONE encoded envelope payload. A composed member has
    /// no transport of its own, so its composing parent announces this on the member's behalf under the
    /// member's `(slot, child_id)` lane the moment the parent attaches a backbone.
    pub fn event_log_payload(&self) -> Result<Vec<u8>, VcsError> {
        Ok(semio_framework_os_spr::encode_envelopes(&self.event_log()?))
    }

    /// 📤️ Seeds this replica's TAIL edit into its own causal dag and hands back exactly the
    /// envelopes {@link flush_apply_outbound} would have sent, for a caller that owns the transport —
    /// the composed-member half of outbound announcement. Empty when nothing has ever been applied.
    pub fn announce_tail_edit_payload(&mut self) -> Result<Vec<u8>, VcsError> {
        let Some(edit) = self.envelope.vcs.edits.last().filter(|edit| self.envelope.holds_committed_edit(&edit.id)) else { return Ok(Vec::new()) };
        let document_id = ArtifactId(self.envelope.id.clone());
        let schema = SchemaId(self.envelope.schema.clone());
        let op_envelopes = semio_framework_os_spr::mutation_envelope_from_edit::<P, Mutation>(edit, &document_id, &schema).map_err(|error| VcsError::Serialize(error.to_string()))?;
        for op_envelope in &op_envelopes {
            match self.dag.seed_applied(op_envelope.mutation_id.clone()) {
                Ok(()) => {}
                Err(rejected) if rejected.error == semio_framework_os_spr::MutationDagError::Duplicate => {}
                Err(rejected) => return Err(VcsError::ValidationFailed(rejected.error.to_string())),
            }
        }
        Ok(semio_framework_os_spr::encode_envelopes(&op_envelopes))
    }

    pub fn backbone_ref(&self) -> Option<&ArtifactBackboneRef> {
        self.envelope.backbone.as_ref()
    }

    pub fn assert_equivalent_remote_envelope(&self, existing: &Edit<Mutation>, incoming: &semio_framework_os_spr::MutationEnvelope) -> Result<(), VcsError> {
        let document_id = ArtifactId(self.envelope.id.clone());
        let schema = SchemaId(self.envelope.schema.clone());
        let established = semio_framework_os_spr::mutation_envelope_from_edit::<P, Mutation>(existing, &document_id, &schema).map_err(|error| VcsError::Serialize(error.to_string()))?;
        if established.len() == 1 && established.first().is_some_and(|candidate| Self::same_operation_identity_and_payload(candidate, incoming)) {
            return Ok(());
        }
        Err(VcsError::ValidationFailed(format!("remote mutation id {} conflicts with its established payload", incoming.mutation_id.0)))
    }

    pub fn assert_equivalent_remote_mutation(&self, incoming: &semio_framework_os_spr::MutationEnvelope) -> Result<(), VcsError> {
        if semio_framework_os_spr::is_history_transition(incoming) {
            return match self.envelope.transitions.iter().find(|known| known.mutation_id == incoming.mutation_id) {
                Some(known) if Self::same_operation_identity_and_payload(known, incoming) => Ok(()),
                Some(_) => Err(VcsError::ValidationFailed(format!("remote transition {} conflicts with its established payload", incoming.mutation_id.0))),
                None => Err(VcsError::ValidationFailed(format!("remote transition {} was marked applied without an established payload", incoming.mutation_id.0))),
            };
        }
        let document_id = ArtifactId(self.envelope.id.clone());
        let schema = SchemaId(self.envelope.schema.clone());
        for edit in &self.envelope.vcs.edits {
            for established in semio_framework_os_spr::mutation_envelope_from_edit::<P, Mutation>(edit, &document_id, &schema).map_err(|error| VcsError::Serialize(error.to_string()))? {
                if established.mutation_id == incoming.mutation_id {
                    return if Self::same_operation_identity_and_payload(&established, incoming) { Ok(()) } else { Err(VcsError::ValidationFailed(format!("remote mutation id {} conflicts with its established payload", incoming.mutation_id.0))) };
                }
            }
        }
        Err(VcsError::ValidationFailed(format!("remote mutation id {} was marked applied without an established payload", incoming.mutation_id.0)))
    }

    /// 🚫️async: E1 pure field comparison, consumed only through `is_some_and`/`Iterator::all`
    /// sync closures — see R9.
    pub fn same_operation_identity_and_payload(left: &semio_framework_os_spr::MutationEnvelope, right: &semio_framework_os_spr::MutationEnvelope) -> bool {
        left.mutation_id == right.mutation_id && left.document_id == right.document_id && left.diff.schema == right.diff.schema && left.diff.payload == right.diff.payload
    }

    fn take_candidate_envelope(&mut self) -> ArtifactEnvelope<P, Mutation> {
        assert!(!self.envelope_detached, "resolution candidate envelope authority was already detached");
        self.envelope_detached = true;
        unsafe { std::mem::ManuallyDrop::take(&mut self.envelope) }
    }

    /// 🧬️ A peer's `Genesis` must name this document's own initial snapshot: replicas of one
    /// document share their genesis, so a mismatch is a different document, never something to merge.
    pub fn verify_genesis(&self, pack: &[u8]) -> Result<(), VcsError> {
        if *semio_framework_hash::hash(pack).as_bytes() != self.initial_digest {
            return Err(VcsError::ValidationFailed("backbone genesis names a different initial snapshot; replicas of one document share their genesis".into()));
        }
        Ok(())
    }

    /// 🖋️ Whether `edit_id` was authored by this opened actor; every other actor is foreign.
    pub fn edit_is_local(&self, edit_id: &str) -> bool {
        self.envelope.vcs.edits.find_near(|edit| edit.id == edit_id).is_some_and(|edit| edit.actor.as_deref() == Some(self.local_actor_id().0.as_str()))
    }
}

impl<P, Mutation> StoreOwners<P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    /// 🧳️ Retains an original projection from this exact issuer, including an older captured frontier, for readonly custody.
    pub fn retained_derived_read(&self, source: &ArtifactDerivedSnapshot<P>) -> Result<ErasedSnapshotRead, ValueError>
    where P: Sync {
        if !source.registry.ptr_eq(&self.snapshot_read_leases) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "projection read requires its original Store issuer")); }
        let owner = Arc::clone(&source.owner);
        let lease = match source.registry.try_issue(Arc::clone(&owner)) { Ok(lease) => lease, Err(alias) => { drop(alias); return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "original projection read issuer is unavailable")); } };
        Ok(ErasedSnapshotRead::new(owner, lease))
    }

    /// 🧰️ Prepares the exact history retirement frame before the caller commits its original Session.
    pub fn prepare_history_read_retirement(&self,grant:RetainedCloneGrant)->Result<Option<(PreparedHistoryReadRetirement<P,Mutation>,RetainedCloneProgress)>,ValueError>
    where P:Sync {
        let snapshots=self.initial_snapshot_retirement_factory.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"history preparation requires its exact snapshot issuer"))?;
        let mutations=self.mutation_retirement_factory.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"history preparation requires its exact mutation issuer"))?;
        let Some((mut prepared,receipt))=history_read_admission::prepare_history_read_retirement(snapshots,mutations,grant)?else{return Ok(None)};
        prepared.bind_registry(self.snapshot_read_leases.identity());Ok(Some((prepared,receipt)))
    }

    fn admit_history_owner_retirement<T>(&self,owner:&mut Option<T>,grant:RetainedCloneGrant,install:fn(&mut ArtifactHistoryReadRetirement<P,Mutation>,T))->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>
    where P:Sync {
        if owner.is_none(){return Ok(None);}
        let snapshots=self.initial_snapshot_retirement_factory.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"history cancellation requires its exact snapshot issuer"))?;
        let mutations=self.mutation_retirement_factory.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"history cancellation requires its exact mutation issuer"))?;
        history_read_admission::admit_history_read_retirement(owner,snapshots,mutations,grant,install)
    }

    /// 🛑️ Transfers a cancelled preview's exact owners to bounded cold retirement, including stale frontiers.
    pub fn retire_derived_history_preview(&self, owner: &mut Option<ArtifactDerivedHistoryPreview<P, Mutation>>,grant:RetainedCloneGrant) -> Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>, ValueError>
    where P: Sync {
        let Some(preview) = owner.as_ref() else { return Ok(None) };
        if !preview.registry.ptr_eq(&self.snapshot_read_leases) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"foreign history preview cannot retire through this Store")); }
        self.admit_history_owner_retirement(owner,grant,|retirement,preview|*retirement.preview=Some(preview))
    }

    /// 🛑️ Transfers a cancelled replay and its planner to bounded cold retirement without publishing any fold.
    pub fn retire_derived_report_replay(&self, owner: &mut Option<ArtifactDerivedReplay<P, Mutation>>,grant:RetainedCloneGrant) -> Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>, ValueError>
    where P: Sync {
        let Some(replay) = owner.as_ref() else { return Ok(None) };
        if !replay.registry.ptr_eq(&self.snapshot_read_leases) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"foreign history replay cannot retire through this Store")); }
        self.admit_history_owner_retirement(owner,grant,|retirement,replay|*retirement.replay=Some(replay))
    }

    /// 🧹️ Transfers a discarded completed history review to the same bounded registered owner retirement.
    pub fn retire_finished_history_replay(&self, owner: &mut Option<EditReplayResult<P, Mutation>>,grant:RetainedCloneGrant) -> Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>, ValueError>
    where P: Sync {
        self.admit_history_owner_retirement(owner,grant,|retirement,finished|*retirement.finished=Some(finished))
    }

    /// 🪪️ Borrows the original installed snapshot issuer without creating a lease or frame.
    pub fn owned_snapshot_retirement_factory(&self)->Result<&Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,ValueError>{
        self.initial_snapshot_retirement_factory.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"derived snapshot retirement requires its original installed issuer"))
    }

    /// 🪪️ Borrows the installed issuer without cloning or transferring original ownership.
    pub fn owned_mutation_retirement_factory(&self)->Result<&Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,ValueError>{
        self.mutation_retirement_factory.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"owned mutation retirement requires its installed exact issuer"))
    }

    /// 📏️ Quotes the original-value frame while requiring its installed mutation issuer.
    pub fn owned_mutation_retirement_birth_bytes(&self)->Result<usize,ValueError>{
        self.owned_mutation_retirement_factory()?;
        Ok(semio_framework_value::FactoryOwnedRetirement::<Mutation>::constructor_capacity_bytes())
    }

    /// ♻️ Preserves the exact caller mutation until its installed issuer admits every frame currency.
    pub fn retire_owned_mutation(&self,owner:&mut Option<Mutation>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>{
        if owner.is_none(){return Ok(None);}
        let factory=self.owned_mutation_retirement_factory()?;
        semio_framework_value::FactoryOwnedRetirement::<Mutation>::admit_original(owner,factory,grant).map(|admitted|admitted.map(|(frame,progress)|(frame as Box<dyn ErasedSnapshotRetirement>,progress)))
    }

    /// 🔐️ Installs the one domain-supplied owner catalog before a store can enter
    /// retained replacement or close. There is no default catalog and a second installation faults.
    /// The catalog also carries the canonical operation wire of its mutation vocabulary (a catalog cannot be built
    /// for a mutation without a canonical tree) and hands it to the store unless the store already holds a richer one,
    /// so a store that can record history (which needs a catalog) can always author.
    pub fn install_document_store_owners_exact(&mut self, store_log: &mut StoreLog<P, Mutation>, mut owners: DocumentStoreOwners<P, Mutation>) -> Result<(), (ValueError, DocumentStoreOwners<P, Mutation>)> {
        if !owners.constructor_is_complete() { return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"installed original catalog requires every admitted factory ticket before transfer"),owners)); }
        if self.snapshot_retirement_factory.is_some() || self.initial_snapshot_retirement_factory.is_some() || self.mutation_retirement_factory.is_some() || self.owned_disposer.is_some() || self.owned_disposer_terminal { return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"store already retains its original owner catalog"),owners)); }
        *self.factory_retirement_tickets = owners.take_factory_retirement_tickets();
        *self.snapshot_retirement_factory = owners.snapshot_retirement.take();
        *self.initial_snapshot_retirement_factory = owners.initial_snapshot_retirement.take();
        *self.mutation_retirement_factory = owners.mutation_retirement.take();
        *self.owned_disposer = owners.store_disposer.take();
        *self.one_item_preparation_factory = owners.one_item_preparation.take();
        *self.replay_preparation_factory = owners.replay_preparation.take();
        *self.one_item_wire_preparation_factory = owners.one_item_wire_preparation.take();
        store_log.operation_wire.get_or_insert(owners.operation_wire);
        Ok(())
    }

    pub fn owned_disposer_installed(&self) -> bool {
        self.owned_disposer.is_some() || self.owned_disposer_terminal
    }

    fn displaced_envelope_retirement(&self, envelope: ArtifactEnvelope<P, Mutation>) -> Result<Box<dyn ErasedSnapshotRetirement>, VcsError> {
        let initial_snapshot_factory = (*self.initial_snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store envelope replacement requires its exact initial-snapshot retirement factory".into()))?;
        let mutation_factory = (*self.mutation_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store envelope replacement requires its exact mutation retirement factory".into()))?;
        Ok(Box::new(ArtifactStoreEnvelopeRetirement::new(envelope, initial_snapshot_factory, mutation_factory)))
    }

    fn replace_envelope_retained(&mut self, store_log: &mut StoreLog<P, Mutation>, next: ArtifactEnvelope<P, Mutation>) -> Result<(), VcsError> {
        self.displaced_retirements.reserve(1)?;
        let initial_snapshot_factory = (*self.initial_snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store envelope replacement requires its exact initial-snapshot retirement factory".into()))?;
        let mutation_factory = (*self.mutation_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store envelope replacement requires its exact mutation retirement factory".into()))?;
        let previous = std::mem::replace(&mut *store_log.envelope, next);
        self.displaced_retirements.push_reserved(Box::new(ArtifactStoreEnvelopeRetirement::new(previous, initial_snapshot_factory, mutation_factory)));
        Ok(())
    }

    pub fn replace_causal_dag_retained(&mut self, store_log: &mut StoreLog<P, Mutation>, next: semio_framework_os_spr::MutationDag) -> Result<(), VcsError> {
        self.displaced_retirements.reserve(1)?;
        let previous = std::mem::replace(&mut *store_log.dag, next);
        self.displaced_retirements.push_reserved(Box::new(ArtifactStoreMutationDagRetirement::new(previous)));
        Ok(())
    }

    pub fn reserve_edit_history_slot(&mut self, store_log: &mut StoreLog<P, Mutation>) -> Result<ArtifactStoreHistoryCommitReservation, VcsError> {
        if self.mutation_retirement_factory.is_none() {
            return Err(VcsError::ValidationFailed("edit history insertion requires its exact mutation retirement factory".into()));
        }
        let rejected_owner = self.displaced_retirements.reserve_owner_slots(1)?;
        match store_log.envelope.vcs.edits.reserve_one() {
            Ok(history) => Ok(ArtifactStoreHistoryCommitReservation { history, rejected_owner }),
            Err(_) => {
                self.displaced_retirements.release_owner_slots(rejected_owner)?;
                Err(VcsError::HistoryFull { capacity: store_log.envelope.vcs.edits.len() })
            }
        }
    }

    fn reserve_change_history_slot(&mut self, store_log: &mut StoreLog<P, Mutation>) -> Result<ArtifactStoreHistoryCommitReservation, VcsError> {
        let rejected_owner = self.displaced_retirements.reserve_owner_slots(1)?;
        match store_log.envelope.vcs.changes.reserve_one() {
            Ok(history) => Ok(ArtifactStoreHistoryCommitReservation { history, rejected_owner }),
            Err(_) => {
                self.displaced_retirements.release_owner_slots(rejected_owner)?;
                Err(VcsError::ValidationFailed("change history ledger is saturated".into()))
            }
        }
    }

    fn insert_reserved_change_history(&mut self, store_log: &mut StoreLog<P, Mutation>, mut reservation: ArtifactStoreHistoryCommitReservation, change: Change) -> Result<ArtifactHistoryKey, VcsError> {
        match store_log.envelope.vcs.changes.insert_reserved(reservation.history, change) {
            Ok(key) => {
                self.displaced_retirements.release_owner_slots(reservation.rejected_owner)?;
                Ok(key)
            }
            Err((_, change)) => {
                let owner = Box::new(ArtifactStoreHistoryMetadataRetirement::change(change));
                if let Err(owner) = self.displaced_retirements.push_owner_reserved(&mut reservation.rejected_owner, owner) {
                    self.displaced_retirements.push_reserved(owner);
                }
                Err(VcsError::ValidationFailed("change history reservation is stale".into()))
            }
        }
    }

    fn reserve_checkpoint_history_slot(&mut self, store_log: &mut StoreLog<P, Mutation>) -> Result<ArtifactStoreHistoryCommitReservation, VcsError> {
        let rejected_owner = self.displaced_retirements.reserve_owner_slots(1)?;
        match store_log.envelope.vcs.checkpoints.reserve_one() {
            Ok(history) => Ok(ArtifactStoreHistoryCommitReservation { history, rejected_owner }),
            Err(_) => {
                self.displaced_retirements.release_owner_slots(rejected_owner)?;
                Err(VcsError::ValidationFailed("checkpoint history ledger is saturated".into()))
            }
        }
    }

    fn insert_reserved_checkpoint_history(&mut self, store_log: &mut StoreLog<P, Mutation>, mut reservation: ArtifactStoreHistoryCommitReservation, checkpoint: Checkpoint) -> Result<ArtifactHistoryKey, VcsError> {
        match store_log.envelope.vcs.checkpoints.insert_reserved(reservation.history, checkpoint) {
            Ok(key) => {
                self.displaced_retirements.release_owner_slots(reservation.rejected_owner)?;
                Ok(key)
            }
            Err((_, checkpoint)) => {
                let owner = Box::new(ArtifactStoreHistoryMetadataRetirement::checkpoint(checkpoint));
                if let Err(owner) = self.displaced_retirements.push_owner_reserved(&mut reservation.rejected_owner, owner) {
                    self.displaced_retirements.push_reserved(owner);
                }
                Err(VcsError::ValidationFailed("checkpoint history reservation is stale".into()))
            }
        }
    }

    fn reserve_alternative_history_slot(&mut self, store_log: &mut StoreLog<P, Mutation>) -> Result<ArtifactStoreHistoryCommitReservation, VcsError> {
        let rejected_owner = self.displaced_retirements.reserve_owner_slots(1)?;
        match store_log.envelope.vcs.alternatives.reserve_one() {
            Ok(history) => Ok(ArtifactStoreHistoryCommitReservation { history, rejected_owner }),
            Err(_) => {
                self.displaced_retirements.release_owner_slots(rejected_owner)?;
                Err(VcsError::ValidationFailed("alternative history ledger is saturated".into()))
            }
        }
    }

    fn insert_reserved_alternative_history(&mut self, store_log: &mut StoreLog<P, Mutation>, mut reservation: ArtifactStoreHistoryCommitReservation, alternative: Alternative) -> Result<ArtifactHistoryKey, VcsError> {
        match store_log.envelope.vcs.alternatives.insert_reserved(reservation.history, alternative) {
            Ok(key) => {
                self.displaced_retirements.release_owner_slots(reservation.rejected_owner)?;
                Ok(key)
            }
            Err((_, alternative)) => {
                let owner = Box::new(ArtifactStoreHistoryMetadataRetirement::alternative(alternative));
                if let Err(owner) = self.displaced_retirements.push_owner_reserved(&mut reservation.rejected_owner, owner) {
                    self.displaced_retirements.push_reserved(owner);
                }
                Err(VcsError::ValidationFailed("alternative history reservation is stale".into()))
            }
        }
    }

    pub fn maintenance_retirements_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.displaced_retirements.close_step(grant)
    }

    /// 🚚️ Transfers an interrupted publication to this Store's bounded maintenance queue before
    /// its driver leaves scope. The queue retains the exact preparation and clone cursor owners;
    /// subsequent maintenance turns cancel unpublished work and retire every owner under grants.
    pub fn handoff_batch_publication_close(&mut self, publication: &mut Option<ArtifactStoreBatchPublication<P, Mutation>>) -> Result<bool, VcsError>
    where
        P: Sync,
    {
        if publication.is_none() {
            return Ok(false);
        }
        self.displaced_retirements.reserve(1)?;
        let mut owner = publication.take().expect("validated publication close handoff remains present");
        owner.begin_close();
        self.displaced_retirements.push_reserved(Box::new(owner));
        Ok(true)
    }

    pub fn maintenance_retirements_terminal_is_empty(&self) -> bool {
        self.displaced_retirements.terminal_is_empty()
    }

    /// 📏️ Borrows the queued original owner's next payload or terminal frame release.
    pub fn maintenance_retirements_demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.displaced_retirements.demands(copy) }

    /// 🌡️ True while the displaced-owner queue holds at least [`ARTIFACT_STORE_DISPLACED_PRESSURE_OCCUPANCY`] owners.
    pub fn maintenance_retirements_under_pressure(&self) -> bool {
        self.displaced_retirements.under_pressure()
    }

    /// 🔭️ Which cursor phase the installed store disposer is draining right now.
    pub fn close_owned_phase_witness(&self) -> String {
        self.owned_disposer.as_ref().map_or_else(|| "uninstalled".to_string(), |disposer| disposer.close_phase_witness())
    }

    pub fn close_one_factory_alias(&mut self) {
        if self.snapshot_retirement_factory.is_some() { drop(self.snapshot_retirement_factory.take()); }
        else if self.initial_snapshot_retirement_factory.is_some() { drop(self.initial_snapshot_retirement_factory.take()); }
        else if self.mutation_retirement_factory.is_some() { drop(self.mutation_retirement_factory.take()); }
        else if self.one_item_preparation_factory.is_some() { drop(self.one_item_preparation_factory.take()); }
        else if self.replay_preparation_factory.is_some() { drop(self.replay_preparation_factory.take()); }
        else if self.one_item_wire_preparation_factory.is_some() { drop(self.one_item_wire_preparation_factory.take()); }
    }

    pub fn retire_snapshot_alias(&self,alias:&mut Option<Arc<P>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>
    where P:Sync {
        if alias.is_none(){return Ok(None);}
        let factory=self.initial_snapshot_retirement_factory.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"snapshot alias retirement requires its installed exact issuer"))?;
        ReturnedSnapshotReadRetirement::admit_original(alias,factory,grant)
    }

    /// 📏️ Quotes the frame birth of retiring one captured snapshot alias.
    pub fn snapshot_read_retirement_demand(&self) -> semio_framework_value::RetirementDemand {
        semio_framework_value::RetirementDemand { capacity_bytes: ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(), depth: 2, ..Default::default() }
    }

    /// 📏️ Quotes the frame birth of retiring the next returned original root, nothing when none returned.
    pub fn returned_snapshot_read_retirement_demand(&self) -> Result<semio_framework_value::RetirementDemand, ValueError>
    where
        P: Sync,
    {
        if self.snapshot_read_leases.closed() || !self.snapshot_read_leases.has_returned() { return Ok(Default::default()); }
        self.snapshot_read_leases.returned_admission_demands::<P>(|_| self.snapshot_read_retirement_demand()).map_err(SnapshotReadLeaseRefusal::into_value_error)
    }

    /// 🎟️ Admits one returned original root under the caller's complete physical grant.
    pub fn take_returned_snapshot_read_retirement(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError>
    where
        P: Sync,
    {
        if !self.snapshot_read_leases.has_returned() { return Ok((None, Default::default())); }
        let factory = self.initial_snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "returned snapshot read requires its exact installed issuer"))?;
        match self.snapshot_read_leases.try_admit_one_returned::<P, _>(grant, |original, child| {
            match ReturnedSnapshotReadRetirement::admit(original, Arc::clone(factory), child) {
                Ok(admitted) => Ok(admitted),
                Err((error, original, capability)) => { drop(capability); Err((error, original)) }
            }
        }) {
            Ok(admitted) => Ok(admitted),
            Err(SnapshotReadLeaseRefusal::Busy) => Ok((None, Default::default())),
            Err(refusal) => Err(refusal.into_value_error()),
        }
    }

    pub fn snapshot_read_leases_terminal_is_empty(&self) -> bool {
        self.snapshot_read_leases.terminal_is_empty()
    }

    /// 🧾️ Leases returned to the registry whose roots it still holds until a returned-read pump reclaims them —
    /// every one of them keeps a whole document root alive.
    pub fn returned_snapshot_read_count(&self) -> usize {
        self.snapshot_read_leases.returned_count()
    }

    /// 🔬️ Leases still OUT — issued and not yet returned. A close cursor in its `ReturnedReads`
    /// phase answers `Blocked` on exactly this: it has nothing returned to retire and the registry
    /// is not terminal, so it waits for a holder that only the holder can release. `Blocked` is
    /// therefore permanent for the closer, never transient, and a caller that retries it spins for
    /// ever. This names the count so a refusal can say whether a live reader or an unreclaimed
    /// returned lease is the owner.
    pub fn outstanding_snapshot_read_count(&self) -> usize {
        self.snapshot_read_leases.occupied_count().saturating_sub(self.returned_snapshot_read_count())
    }

    pub const RESOLVED_CONFLICT_CAP: usize = 512;

    /// 🎯️ MEDIUM-3: oldest-first eviction of RESOLVED (`Accepted`/`Discarded`) conflicts once the
    /// total exceeds [`Self::RESOLVED_CONFLICT_CAP`] — `Open` conflicts are never touched. See the
    /// const's own doc for the reasoning.
    pub fn prune_resolved_conflicts(&mut self, store_log: &mut StoreLog<P, Mutation>) -> Result<(), VcsError> {
        while store_log.envelope.conflicts.len() > Self::RESOLVED_CONFLICT_CAP {
            let Some(index) = store_log.envelope.conflicts.iter().position(|conflict| conflict.status != semio_framework_os_spr::ConflictStatus::Open) else { break };
            self.displaced_retirements.reserve(1)?;
            let conflict = store_log.envelope.conflicts.remove(index);
            self.displaced_retirements.push_reserved(Box::new(ArtifactStoreConflictRetirement::new(conflict)));
        }
        Ok(())
    }

    //#region 📬️OneItemPublication
    /// 🔏 The gesture's declared row budget, not the budget minus its forwards. An item's
    /// inverse row count is the VARIABLE part of a declaration, and reserving only the
    /// leftover made the staging buffer — not the gesture-wide `PreflightingCommit` gate —
    /// the first refuser of an under-declared multi-item gesture, with the wrong message and
    /// before the fold had folded anything the gate could weigh. The gate still refuses the
    /// overrun; this buffer only has to be able to hold the rows the per-item fold admits.
    ///
    /// 🔏 A stamped publication's first folded item keeps the identity its durable decision was
    /// hashed over. The `#position` form is the store's way of naming the several mutations one
    /// local gesture folds into one edit; rewriting the stamped item to it produced a committed
    /// decision whose `mutation_id` no longer matched the approval target the WAL verifies.
    pub fn fold_batch_item(&mut self, store_log: &mut StoreLog<P, Mutation>, publication: &mut ArtifactStoreBatchPublication<P, Mutation>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemAdvance, VcsError> {
        let authority=publication.authority.as_ref().ok_or_else(||VcsError::ValidationFailed("batched fold lost its original authority".into()))?;
        let control=size_of::<usize>();
        if grant.maximum_depth==0||grant.maximum_copy_bytes<control{return Ok(ArtifactStoreOneItemAdvance::Blocked);}
        if publication.stage.is_none(){
            let capacity=size_of::<ArtifactStoreBatchStage<P,Mutation>>();
            if grant.maximum_capacity_bytes<capacity{return Ok(ArtifactStoreOneItemAdvance::Blocked);}
            let mut stage=Self::empty_batch_stage(authority);stage.mutation_retirement=(*self.mutation_retirement_factory).clone();stage.snapshot_retirement=(*self.snapshot_retirement_factory).clone();
            if stage.mutation_retirement.is_none()||stage.snapshot_retirement.is_none(){return Err(VcsError::ValidationFailed("batched fold lacks its exact original retirement issuers".into()));}
            publication.stage=Some(Box::new(stage));return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:control,retained_capacity_bytes:capacity,..Default::default()}));
        }
        let stage=publication.stage.as_mut().unwrap();
        if stage.edit.is_none(){let capacity=size_of::<Edit<Mutation>>();if grant.maximum_capacity_bytes<capacity{return Ok(ArtifactStoreOneItemAdvance::Blocked);}stage.edit=Some(Box::new(Self::empty_batch_edit(authority)));return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:control,retained_capacity_bytes:capacity,..Default::default()}));}
        macro_rules! reserve_original {($owner:expr,$count:expr,$ty:ty)=>{{let owner=$owner;if owner.capacity()<$count{let capacity=$count.checked_mul(std::mem::size_of::<$ty>()).ok_or_else(||VcsError::ValidationFailed("batch original capacity overflow".into()))?;if grant.maximum_capacity_bytes<capacity||grant.maximum_release_bytes<owner.capacity()*std::mem::size_of::<$ty>(){return Ok(ArtifactStoreOneItemAdvance::Blocked);}let released=owner.capacity()*std::mem::size_of::<$ty>();owner.try_reserve_exact($count-owner.len()).map_err(|_|VcsError::ValidationFailed("batch original reservation failed".into()))?;let actual=owner.capacity()*std::mem::size_of::<$ty>();return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:control,retained_capacity_bytes:actual,released_bytes:released,..Default::default()}));}}};}
        reserve_original!(&mut stage.unit_flags,publication.admitted_items,(usize,bool));
        reserve_original!(&mut stage.edit.as_mut().unwrap().forwards,publication.admitted_items,Mutation);
        reserve_original!(&mut stage.edit.as_mut().unwrap().mutation_meta,publication.admitted_items,MutationMeta);
        let inverse=&mut stage.edit.as_mut().unwrap().inverse;
        let backing=if publication.transaction_open{inverse.next_capacity_allocation_bytes(publication.footprint.work_items)}else{inverse.next_exact_capacity_allocation_bytes(publication.footprint.work_items)}.map_err(|error|VcsError::ValidationFailed(error.reason.into()))?;
        if let Some(capacity)=backing{let depth=inverse.next_reserve_depth_demand().map_err(|error|VcsError::ValidationFailed(error.reason.into()))?;if grant.maximum_capacity_bytes<capacity||grant.maximum_depth<depth{return Ok(ArtifactStoreOneItemAdvance::Blocked);}let step=if publication.transaction_open{inverse.reserve_capacity_one(publication.footprint.work_items,grant.maximum_capacity_bytes)}else{inverse.reserve_exact_capacity_one(publication.footprint.work_items,grant.maximum_capacity_bytes)}.map_err(|error|VcsError::ValidationFailed(error.reason.into()))?;return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:usize::from(step.progressed),retained_capacity_bytes:step.allocated_bytes,..Default::default()}));}
        if stage.folding.is_none(){
            let candidate=publication.preparation.as_ref().and_then(|owner|owner.prepared()).ok_or_else(||VcsError::ValidationFailed("batched fold lost its original prepared candidate".into()))?;authority.validate_prepared(candidate).map_err(VcsError::from)?;
            if candidate.edit().forwards.len()!=1||candidate.edit().mutation_meta.len()!=1||candidate.edit().id.is_empty()||candidate.edit().id.len()>ARTIFACT_STORE_ONE_ITEM_ID_BYTES||!candidate.applied_edit_id.eq_str(&candidate.edit().id)||!candidate.tail_edit_id.eq_str(&candidate.edit().id)||candidate.next_clock()!=authority.next_clock()||!candidate.local_actor().as_ref().is_some_and(|actor|actor.eq_str(&authority.actor())){return Err(VcsError::ValidationFailed("batched fold candidate disagrees with its original fixed authority".into()));}
            if candidate.edit().inverse.len().saturating_add(1)>publication.footprint.work_items{return Err(VcsError::TooLarge{rows:candidate.edit().inverse.len()+1,capacity:publication.footprint.work_items});}
            let demand=ArtifactStoreOneItemPrepared::<P,Mutation>::retirement_birth_demand();if demand.admit(grant.retained_grant()).is_err(){return Ok(ArtifactStoreOneItemAdvance::Blocked);}
            let candidate=publication.preparation.as_mut().unwrap().take_prepared().unwrap();
            let(cursor,ownership)=candidate.admit_fold_retirement(Arc::clone(stage.mutation_retirement.as_ref().unwrap()),Arc::clone(stage.snapshot_retirement.as_ref().unwrap()),grant.retained_grant()).map_err(|(error,_,_,_)|VcsError::ValidationFailed(error.into_message()))?;stage.folding=Some(cursor);stage.fold_phase=0;return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),ownership));
        }
        let cursor=stage.folding.as_mut().unwrap();let original=cursor.edit.as_mut().unwrap();let target=stage.edit.as_mut().unwrap();
        macro_rules! copy_string {($target:expr,$source:expr)=>{{let source=$source;let copy=source.len();if grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}$target=source.to_owned();stage.fold_phase+=1;return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:copy,..Default::default()}));}};}
        let mut ownership=RetainedCloneProgress{copied_items:1,copied_bytes:control,..Default::default()};
        match stage.fold_phase{
            0=>{if stage.folded==0{copy_string!(target.id,original.id.as_str());}stage.fold_phase=1;},
            1=>{if stage.folded==0{copy_string!(stage.applied_edit_id,target.id.as_str());}stage.fold_phase=2;},
            2=>{if stage.folded==0{copy_string!(stage.tail_edit_id,target.id.as_str());}stage.fold_phase=3;},
            3=>{if stage.folded==0{let copy=size_of::<Edit<Mutation>>();if grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}target.actor=original.actor.take();target.line=original.line.take();target.started_at=std::mem::take(&mut original.started_at);stage.local_actor=cursor.strings[0].take();stage.next_clock=cursor.fold_clock;ownership.copied_bytes=copy;}stage.fold_phase=4;},
            4=>{if stage.folded==0{if let Some(verb)=publication.verb.as_deref(){let copy=verb.len();if grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}target.verb=Some(verb.to_owned());ownership.copied_bytes=copy;ownership.retained_capacity_bytes=copy;}}stage.fold_phase=5;},
            5=>{if !original.inverse.is_empty(){let depth=original.inverse.next_pop_depth_demand().map_err(|error|VcsError::ValidationFailed(error.reason.into()))?.max(target.inverse.next_push_depth_demand().map_err(|error|VcsError::ValidationFailed(error.reason.into()))?);let copy=size_of::<Mutation>();if grant.maximum_depth<depth||grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}let item=original.inverse.pop().unwrap();target.inverse.push_reserved(item).unwrap_or_else(|_|panic!("original inverse admission lost its reserved slot"));ownership.copied_bytes=copy;}else{stage.fold_phase=6;}},
            6=>{let copy=size_of::<Mutation>();if grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}target.forwards.push(original.forwards.pop().expect("original one-item forward"));ownership.copied_bytes=copy;stage.fold_phase=7;},
            7=>{let copy=size_of::<MutationMeta>();if grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}let meta=&mut original.mutation_meta[0];target.mutation_meta.push(MutationMeta{mutation_id:None,dependencies:std::mem::take(&mut meta.dependencies),base_version:meta.base_version,author_id:meta.author_id.take(),timestamp:meta.timestamp,undo_policy:meta.undo_policy,payload_hash:meta.payload_hash.take(),semantic_kind:None,label:None,group_id:meta.group_id.take(),origin:std::mem::take(&mut meta.origin),transaction:None});ownership.copied_bytes=copy;stage.fold_phase=8;},
            8=>{let position=target.forwards.len()-1;let primary=publication.admitted_items==1&&!publication.transaction_open;let copy=target.id.len()+if primary{0}else{1+position.checked_ilog10().unwrap_or(0)as usize+1};if grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}let mut id=String::with_capacity(copy);id.push_str(&target.id);if !primary{use std::fmt::Write;write!(id,"#{position}").unwrap();}target.mutation_meta.last_mut().unwrap().mutation_id=Some(MutationId(id));ownership.copied_bytes=copy;ownership.retained_capacity_bytes=copy;stage.fold_phase=9;},
            9=>{let kind=target.forwards.last().unwrap().descriptor().semantic_kind;let copy=store_log.envelope.schema.len()+1+kind.len();if grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}let mut value=String::with_capacity(copy);value.push_str(&store_log.envelope.schema);value.push('#');value.push_str(kind);target.mutation_meta.last_mut().unwrap().semantic_kind=Some(SchemaId(value));ownership.copied_bytes=copy;ownership.retained_capacity_bytes=copy;stage.fold_phase=10;},
            10=>{let source=target.forwards.last().unwrap().descriptor().display_name;let copy=source.len();if grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}target.mutation_meta.last_mut().unwrap().label=Some(source.to_owned());ownership.copied_bytes=copy;ownership.retained_capacity_bytes=copy;stage.fold_phase=11;},
            11=>{if let Some(transaction)=publication.transaction.as_ref(){let copy=transaction.id.len()+transaction.tool.len();if grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}target.mutation_meta.last_mut().unwrap().transaction=Some(transaction.clone());ownership.copied_bytes=copy;ownership.retained_capacity_bytes=copy;}stage.fold_phase=12;},
            12=>{if target.forwards.last().unwrap().may_emit_foreign_steps(){let Some(presence)=cursor.foreign_step_presence else{return Ok(ArtifactStoreOneItemAdvance::Blocked);};let copy=size_of::<(usize,bool)>();if grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}stage.unit_flags.push((target.forwards.len()-1,presence));ownership.copied_bytes=copy;}stage.fold_phase=13;},
            13=>{let copy=size_of::<Arc<P>>();if grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}let incoming=cursor.post.take().expect("original prepared post root");*cursor.post=stage.post.replace(incoming);ownership.copied_bytes=copy;stage.fold_phase=14;},
            _=>{let copy=size_of::<ArtifactStoreOneItemCheckpoint>()+size_of::<Box<ArtifactStorePreparedRetirement<P,Mutation>>>();if grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}let item=publication.preparation.as_ref().unwrap().checkpoint();stage.folded+=1;stage.completed_items=stage.completed_items.saturating_add(item.completed_items);stage.completed_bytes=stage.completed_bytes.saturating_add(item.completed_bytes);stage.digest=cursor.fold_digest;stage.retiring=stage.folding.take().map(|owner|owner as Box<dyn ErasedSnapshotRetirement>);publication.item_closing=true;publication.preparation.as_mut().unwrap().begin_close();ownership.copied_bytes=copy;},
        }
        Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),ownership))
    }

    /// 🧾️ The empty staged shell every fold accumulates into — no allocation happens here; the
    /// first fold reserves the gesture's exact admitted capacity.
    pub fn empty_batch_stage(authority: &Arc<ArtifactStoreOneItemLiveAuthority>) -> ArtifactStoreBatchStage<P, Mutation> {
        ArtifactStoreBatchStage {
            unit_flags: Vec::new(),
            folding: None,
            fold_phase: 0,
            digest_owner: None,
            digest_complete: false,
            edit: None,
            pending_edit: None,
            post: None,
            next_clock: authority.next_clock(),
            local_actor: None,
            applied_edit_id: String::new(),
            tail_edit_id: String::new(),
            digest: [0; 32],
            folded: 0,
            completed_items: 0,
            completed_bytes: 0,
            mutation_retirement: None,
            snapshot_retirement: None,
            retiring: None,
            factory_close: Default::default(),
        }
    }

    pub fn empty_batch_edit(authority: &ArtifactStoreOneItemLiveAuthority) -> Edit<Mutation> {
        Edit {
                line: None,
                id: String::new(),
                actor: None,
                forwards: Vec::new(),
                inverse: Vec::new().into(),
                mutation_meta: Vec::new(),
                verb: None,
                sequence_number: authority.next_sequence_number(),
                started_at: String::new(),
                finished_at: None,
            }
    }

    //#endregion 📬️OneItemPublication
    //#region 🔖️EventSourcedHistory
    /// 🛫️ Refuses a reprojection before it adopts anything: `fold`'s new history facts fit their ledgers and their pins
    /// parse, `replay`'s new message entries fit the message ledger, and the displaced-owner queue holds every owner the
    /// adoption (with `recorded` prefix projections) and the revision bump that follows it displace.
    pub fn preflight_reprojection(&self, store_log: &StoreLog<P, Mutation>, fold: &semio_framework_os_spr::HistoryFold, replay: Option<&EditReplayResult<P, Mutation>>, recorded: usize) -> Result<(), VcsError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

        let vcs = &store_log.envelope.vcs;
        let changes = fold.changes.iter().filter(|change| !vcs.changes.iter().any(|known| known.id == change.id)).count();
        let checkpoints = fold
            .checkpoints
            .iter()
            .filter(|checkpoint| !vcs.checkpoints.iter().any(|known| known.id == checkpoint.id || (known.parent_id == checkpoint.parent_id && known.change_ids == checkpoint.change_ids && !fold.checkpoints.iter().any(|folded| folded.id == known.id))))
            .count();
        let alternatives = fold.alternatives.iter().filter(|alternative| !vcs.alternatives.iter().any(|known| known.id == alternative.id)).count();
        if changes > vcs.changes.vacancies() || checkpoints > vcs.checkpoints.vacancies() || alternatives > vcs.alternatives.vacancies() {
            return Err(VcsError::ValidationFailed("history ledgers cannot hold the folded facts".into()));
        }
        for pin in fold.checkpoints.iter().flat_map(|checkpoint| checkpoint.pins.iter()) {
            semio_framework_artifact_reference::ArtifactRef::parse_uri(&pin.child_uri).map_err(VcsError::Deserialize)?;
        }
        let entries = replay.map_or(&[][..], |result| result.replayed.as_slice());
        let admitted = entries.iter().filter(|entry| !entry.messages.is_empty() && store_log.envelope.edit_messages.get_by_id(&entry.edit_id).is_none()).count();
        if admitted > ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY.saturating_sub(store_log.envelope.edit_messages.len()) {
            return Err(VcsError::ValidationFailed("artifact edit-message ledger cannot hold the replayed messages".into()));
        }
        self.displaced_retirements.reserve(usize::from(changes + checkpoints + alternatives > 0) + entries.len() + 2 * recorded + REPROJECTION_OWNER_SLOTS)
    }

    /// 🧹️ Hands a finished replay's untaken outputs and original residual to the displaced queue; a replay without installed
    /// issuers (its aliases would wait behind the queue's own shared-root retirements) or a saturated queue retires them
    /// unscheduled from their own quoted demands instead.
    pub fn retire_replay_result(&mut self, mut result: EditReplayResult<P, Mutation>) -> Result<(), VcsError> {
        result.begin_retirement();
        if result.terminal_is_empty() {
            return Ok(());
        }
        match self.displaced_retirements.reserve(1) {
            Ok(()) if !result.cold_authority() => {
                self.displaced_retirements.push_reserved(Box::new(result));
                Ok(())
            }
            _ => result.retire_unscheduled().map_err(|error| VcsError::ValidationFailed(error.into_message())),
        }
    }

    /// 🧹️ Hands a raw replay's original owners to the displaced queue; a saturated queue retires them unscheduled instead.
    pub fn retire_raw_replay(&mut self, mut replay: EditReplay<P, Mutation>) -> Result<(), VcsError> {
        replay.begin_retirement();
        if replay.terminal_is_empty() {
            return Ok(());
        }
        match self.displaced_retirements.reserve(1) {
            Ok(()) if !replay.cold_authority() => {
                self.displaced_retirements.push_reserved(Box::new(replay));
                Ok(())
            }
            _ => replay.retire_unscheduled().map_err(|error| VcsError::ValidationFailed(error.into_message())),
        }
    }

    /// 🧹️ Hands the running replay of a deferred reprojection, if any, to [`Self::retire_raw_replay`].
    pub fn retire_pending_replay(&mut self, replay: Option<([u8; 32], EditReplay<P, Mutation>)>) -> Result<(), VcsError> {
        replay.map_or(Ok(()), |(_, replay)| self.retire_raw_replay(replay))
    }

    /// 🏛️ Materializes the fold's change/checkpoint/alternative facts into the history ledgers: a re-identified
    /// (repinned) checkpoint is renamed in place, every alternative's chain follows the fold, facts the fold no longer
    /// holds (a retracted transition's) leave, and new facts are inserted.
    pub fn adopt_history_facts(&mut self, store_log: &mut StoreLog<P, Mutation>, fold: &semio_framework_os_spr::HistoryFold) -> Result<(), VcsError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

        let shrank = |fault: semio_framework_os_vcs::ArtifactHistoryReservationFault| VcsError::ValidationFailed(format!("a history ledger cannot follow its fold: {fault:?}"));
        let stale_change = |known: &Change| !fold.changes.iter().any(|change| change.id == known.id);
        if store_log.envelope.vcs.changes.iter().any(stale_change) {
            store_log.envelope.vcs.changes.extract_if(stale_change).map_err(shrank)?;
        }
        for change in &fold.changes {
            if store_log.envelope.vcs.changes.iter().any(|known| known.id == change.id) {
                continue;
            }
            let reservation = self.reserve_change_history_slot(store_log)?;
            self.insert_reserved_change_history(store_log, reservation, Change { id: change.id.clone(), edit_ids: change.edit_ids.clone(), description: change.description.clone(), saved_at: change.saved_at.clone() })?;
        }
        let mut fresh = Vec::new();
        for checkpoint in &fold.checkpoints {
            let mut pins = Vec::with_capacity(checkpoint.pins.len());
            for pin in &checkpoint.pins {
                pins.push(semio_framework_os_vcs::CompositionPin { child_ref: semio_framework_artifact_reference::ArtifactRef::parse_uri(&pin.child_uri).map_err(VcsError::Deserialize)?, checkpoint_id: pin.checkpoint_id.clone() });
            }
            let known_position = store_log
                .envelope
                .vcs
                .checkpoints
                .iter()
                .position(|known| known.id == checkpoint.id)
                .or_else(|| store_log.envelope.vcs.checkpoints.iter().position(|known| known.parent_id == checkpoint.parent_id && known.change_ids == checkpoint.change_ids && !fold.checkpoints.iter().any(|folded| folded.id == known.id)));
            let Some(position) = known_position else {
                fresh.push((checkpoint, pins));
                continue;
            };
            let known = &mut store_log.envelope.vcs.checkpoints[position];
            if known.id != checkpoint.id {
                known.id = checkpoint.id.clone();
            }
            if known.composition_pins != pins {
                known.composition_pins = pins;
            }
        }
        let stale_checkpoint = |known: &Checkpoint| !fold.checkpoints.iter().any(|checkpoint| checkpoint.id == known.id);
        if store_log.envelope.vcs.checkpoints.iter().any(stale_checkpoint) {
            store_log.envelope.vcs.checkpoints.extract_if(stale_checkpoint).map_err(shrank)?;
        }
        for (checkpoint, pins) in fresh {
            let authors = checkpoint.authors.iter().map(|author| Author { id: author.id.clone(), name: author.name.clone(), avatar: author.avatar.clone() }).collect();
            let reservation = self.reserve_checkpoint_history_slot(store_log)?;
            self.insert_reserved_checkpoint_history(store_log, 
                reservation,
                Checkpoint {
                    id: checkpoint.id.clone(),
                    change_ids: checkpoint.change_ids.clone(),
                    parent_id: checkpoint.parent_id.clone(),
                    authors,
                    message: checkpoint.message.clone(),
                    timestamp: checkpoint.timestamp.clone(),
                    composition_pins: pins,
                },
            )?;
        }
        let stale_alternative = |known: &Alternative| !fold.alternatives.iter().any(|alternative| alternative.id == known.id);
        if store_log.envelope.vcs.alternatives.iter().any(stale_alternative) {
            store_log.envelope.vcs.alternatives.extract_if(stale_alternative).map_err(shrank)?;
        }
        for alternative in &fold.alternatives {
            if let Some(known) = store_log.envelope.vcs.alternatives.iter_mut().find(|known| known.id == alternative.id) {
                if known.checkpoint_ids != alternative.checkpoint_ids {
                    known.checkpoint_ids = alternative.checkpoint_ids.clone();
                }
                continue;
            }
            let reservation = self.reserve_alternative_history_slot(store_log)?;
            self.insert_reserved_alternative_history(store_log, reservation, Alternative { id: alternative.id.clone(), name: alternative.name.clone(), checkpoint_ids: alternative.checkpoint_ids.clone() })?;
        }
        order_ledger_as(&mut store_log.envelope.vcs.changes, |change: &Change| change.id.as_str(), fold.changes.iter().map(|change| change.id.as_str()));
        order_ledger_as(&mut store_log.envelope.vcs.checkpoints, |checkpoint: &Checkpoint| checkpoint.id.as_str(), fold.checkpoints.iter().map(|checkpoint| checkpoint.id.as_str()));
        order_ledger_as(&mut store_log.envelope.vcs.alternatives, |alternative: &Alternative| alternative.id.as_str(), fold.alternatives.iter().map(|alternative| alternative.id.as_str()));
        Ok(())
    }

    //#endregion 🔖️EventSourcedHistory
    pub fn replace_edit_messages(&mut self, store_log: &mut StoreLog<P, Mutation>, edit_id: &str, messages: Vec<semio_framework_os_spr::MutationMessage>) -> Result<(), VcsError> {
        if edit_id.len() > ARTIFACT_EDIT_MESSAGE_ID_BYTES {
            return Err(VcsError::ValidationFailed("artifact edit-message id exceeds its fixed byte authority".into()));
        }
        let established = store_log.envelope.edit_messages.get_by_id(edit_id).is_some();
        self.displaced_retirements.reserve(1)?;
        if messages.is_empty() {
            if let Some(entry) = store_log.envelope.edit_messages.remove_id(edit_id) {
                self.displaced_retirements.push_reserved(Box::new(ArtifactStoreMessageLedgerRetirement::new(entry.edit_id, entry.messages)));
            }
            return Ok(());
        }
        let mut candidate = semio_framework_os_spr::EditMessages { edit_id: edit_id.to_string(), messages };
        bound_edit_messages(edit_id, &mut candidate.messages);
        if established {
            let previous = std::mem::replace(store_log.envelope.edit_messages.get_mut_by_id(edit_id).expect("established edit-message identity remains live"), candidate);
            self.displaced_retirements.push_reserved(Box::new(ArtifactStoreMessageLedgerRetirement::new(previous.edit_id, previous.messages)));
        } else if let Err(rejected) = store_log.envelope.edit_messages.admit(candidate) {
            self.displaced_retirements.push_reserved(Box::new(ArtifactStoreMessageLedgerRetirement::new(rejected.edit_id, rejected.messages)));
            return Err(VcsError::ValidationFailed("artifact edit-message fixed ledger rejected its exact replacement owner".into()));
        }
        Ok(())
    }

    /// 🔂️ Replays `operations` over `pre_snapshot`, returning forwards, reversed-inverse,
    /// per-operation metadata, the resulting snapshot, and every `crate::os_spr::MutationMessage`
    /// the replay raised. Shared by `Apply` and `AppendTransaction`. This IS the artifact engine —
    /// `crate::os_engine::ArtifactEngine` never existed as a live trait (see
    /// `.claude/plans/the-mutations-are-extremely-compiled-pumpkin.md`), so `Mutation::diff`/
    /// `inverse` are called directly here on purpose, not as a placeholder for a future indirection.
    /// ATOMIC: if `self.merge_policy.rejects(worst)`, returns `Err` before this method's caller has
    /// touched any store field — nothing about the attempted batch is applied.
    ///
    /// 🌀️ `.await` resolves to an owned value — `.reverse()` on the unawaited future's
    /// result was mutating a throwaway temporary, never `back` itself.
    ///
    /// 🌀️ `mint_mutation_id` is async; `Option::unwrap_or_else`'s closure is sync
    /// (R10 shape 1), so it's written as an explicit match instead.
    ///
    /// 🎯️ An authored timestamp is durable as authored; the local clock observes it
    /// so its next generated timestamp remains causally later.
    ///
    /// 🎞️ CW3: direct blake3 (same primitive `crate::os_pack::ContentHash` uses) replaces the
    /// old `framework_hash::hash_bytes` String hash — `crate::os_spr::PayloadHash` is
    /// now `[u8; 32]`, not a hex string. NOT `crate::os_pack::stored_content_hash`, which reads a pack
    /// FILE's footer rather than hashing arbitrary bytes. 🎯️ B2: hashes the real
    /// `OpBinary` encoding, not a JSON serialization — two ops that encode identically
    /// via `encode_op()` but differ in JSON shape (or vice versa) must hash identically.
    ///
    /// 🩹️ Everything the replay builds — its working projection, the command's operations, their inverses — is a
    /// [`ScratchOwner`]: a refusal on ANY exit retires all of it cold, and the caller adopts each part where the store takes it.
    /// 📦️ Materializes the installed original operation source under the caller's retained admission.
    pub fn authored_operation_bytes(&mut self, store_log: &mut StoreLog<P, Mutation>,mutations:&[Mutation],identity:&mut EntityIdentityAuthority<'_>)->Result<Vec<Vec<u8>>,VcsError>{Self::authored_operation_rows(mutations.len(),mutations.iter(),self.one_item_preparation_factory.as_ref(),store_log.operation_wire,&mut self.displaced_retirements,identity)}

    /// ↩️ Streams the original recorded forward and inverse rows without cloning the durable edit.
    pub fn authored_edit_operation_bytes(&mut self, store_log: &mut StoreLog<P, Mutation>,identity:&mut EntityIdentityAuthority<'_>)->Result<Option<(Vec<Vec<u8>>,Vec<Vec<u8>>)>,VcsError>{
        let Some(edit)=store_log.envelope.vcs.edits.last()else{return Ok(None)};
        let forwards=Self::authored_operation_rows(edit.forwards.len(),edit.forwards.iter(),self.one_item_preparation_factory.as_ref(),store_log.operation_wire,&mut self.displaced_retirements,identity)?;
        let inverse=Self::authored_operation_rows(edit.inverse.len(),edit.inverse.iter(),self.one_item_preparation_factory.as_ref(),store_log.operation_wire,&mut self.displaced_retirements,identity)?;
        Ok(Some((forwards,inverse)))
    }

    /// 🪪️ The one resolution of an authored operation's wire: the installed factory's domain codec when it speaks one, else the store's canonical wire.
    fn resolve_operation_wire<'a>(factory:Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P,Mutation>>>,wire:Option<ArtifactOperationWire<Mutation>>,mutation:&'a Mutation)->Option<ArtifactPreparedOperationSource<'a>>{
        factory.and_then(|factory|factory.operation_wire_source(mutation)).or_else(||wire.and_then(|wire|wire.source(mutation)))
    }

    fn authored_operation_rows<'a>(count:usize,mutations:impl Iterator<Item=&'a Mutation>,factory:Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P,Mutation>>>,wire:Option<ArtifactOperationWire<Mutation>>,retirements:&mut ArtifactStoreDisplacedRetirements,identity:&mut EntityIdentityAuthority<'_>)->Result<Vec<Vec<u8>>,VcsError> where Mutation:'a{
        let slots=count.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"operation wire retirement slots overflow"))?;
        retirements.reserve(slots)?;
        let mut rows=identity.encode(|control|{control.charge(size_of::<operation_identity::OperationWireRows>())?;Ok::<_,ValueError>(Box::new(operation_identity::OperationWireRows::new()))})?;
        let result=(||{
            identity.encode(|control|rows.initialize(count,control))?;
            for mutation in mutations{
                let mut cursor=identity.encode(|control|{control.charge(size_of::<operation_identity::OperationIdentityCursor>())?;Ok::<_,ValueError>(Box::new(operation_identity::OperationIdentityCursor::new()))})?;
                let encoded=match Self::resolve_operation_wire(factory,wire,mutation){
                    Some(source)=>identity.encode(|control|cursor.encode_into(source,rows.active(),control)),
                    None=>Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"authoring requires an installed canonical operation wire or an original domain codec")),
                };
                retirements.push_reserved(cursor);
                encoded?;rows.append();
            }
            Ok::<_,ValueError>(rows.take())
        })();
        retirements.push_reserved(rows);
        result.map_err(VcsError::from)
    }

    fn authored_operation_identity(&mut self, store_log: &mut StoreLog<P, Mutation>,mutation:&Mutation,stamp:(u64,u64,u64),identity:&mut EntityIdentityAuthority<'_>)->Result<(String,[u8;32]),VcsError>{
        self.displaced_retirements.reserve(1)?;
        let mut cursor=identity.encode(|control|{control.charge(size_of::<operation_identity::OperationIdentityCursor>())?;Ok::<_,ValueError>(Box::new(operation_identity::OperationIdentityCursor::new()))})?;
        let committed=match Self::resolve_operation_wire(self.one_item_preparation_factory.as_ref(),store_log.operation_wire,mutation){
            Some(source)=>identity.encode(|control|cursor.commit(source,stamp,control)),
            None=>Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"authoring requires an installed canonical operation wire or an original domain codec")),
        };
        self.displaced_retirements.push_reserved(cursor);
        committed.map_err(VcsError::from)
    }

    pub async fn replay_mutations(&mut self, store_log: &mut StoreLog<P, Mutation>, pre_snapshot: &P, mutations: Vec<Mutation>, transaction: Option<&protocol::TransactionRef>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(ScratchOwner<Vec<Mutation>>, ScratchOwner<Vec<Mutation>>, Vec<MutationMeta>, ScratchOwner<P>, Vec<semio_framework_os_spr::MutationMessage>, Vec<(usize, bool)>), VcsError> {
        let mut snapshot = ScratchOwner::new(pre_snapshot.clone(), retire_replayed_projection::<P, Mutation>);
        let forwards = ScratchOwner::new(mutations, retire_scratch_operations::<P, Mutation>);
        let mut inverse = ScratchOwner::new(Vec::new(), retire_scratch_operations::<P, Mutation>);
        let mut candidate_clock = store_log.clock;
        let mut mutation_meta = Vec::with_capacity(forwards.len());
        let mut messages = Vec::new();
        let mut unit_flags = Vec::new();
        for op_index in 0..forwards.len() {
            if forwards[op_index].may_emit_foreign_steps() { unit_flags.push((op_index, forwards[op_index].foreign_step_source(&snapshot, 0).map_err(VcsError::from)?.is_some())); }
            let mut back = forwards[op_index].inverse(&*snapshot).map_err(VcsError::InverseRefused)?;
            back.reverse();
            inverse.extend(back);
            let authored_timestamp = forwards[op_index].timestamp();
            let timestamp = match authored_timestamp {
                Some(timestamp) => {
                    candidate_clock.merge(&timestamp);
                    timestamp
                }
                None => {
                    candidate_clock.tick(now_ms());
                    candidate_clock
                }
            };
            let (encoded_identity,payload_hash)=self.authored_operation_identity(store_log, &forwards[op_index],(candidate_clock.actor,candidate_clock.physical_ms,candidate_clock.logical),identity)?;
            let authored_id = forwards[op_index].mutation_id();
            let mutation_id = match authored_id {
                Some(id) => id,
                None => MutationId(encoded_identity),
            };
            let mutation = &forwards[op_index];
            mutation_meta.push(MutationMeta {
                mutation_id: Some(mutation_id),
                dependencies: mutation.dependencies(),
                base_version: mutation.base_version().map_or(0, |version| version.0),
                author_id: Some(mutation.author_id().unwrap_or_else(|| store_log.local_actor_id().clone())),
                timestamp,
                undo_policy: mutation.undo_policy(),
                payload_hash: Some(semio_framework_os_spr::PayloadHash(payload_hash)),
                semantic_kind: Some(operation_semantic_kind::<P, Mutation>(mutation, &store_log.envelope.schema)),
                label: Some(mutation.descriptor().display_name.to_string()),
                group_id: None,
                origin: Default::default(),
                transaction: transaction.cloned(),
            });
            let (applied, op_messages) = apply_operation::<P, Mutation>(&*snapshot, mutation, op_index as u32);
            messages.extend(op_messages);
            let next = applied?;
            retire_replayed_projection::<P, Mutation>(std::mem::replace(&mut *snapshot, next));
        }
        if let Some(level) = semio_framework_os_spr::worst_level(&messages) {
            if store_log.merge_policy.rejects(level) {
                return Err(VcsError::Rejected { policy: store_log.merge_policy, messages });
            }
        }
        store_log.clock = candidate_clock;
        Ok((forwards, inverse, mutation_meta, snapshot, messages, unit_flags))
    }

    /// 🧹️ Refuses an ingest after its candidate causal graph was built: the candidate is
    /// retired through the bounded close protocol, never dropped, and the store stays unchanged.
    pub fn refuse_with_candidate_dag<T>(&mut self, candidate_dag: semio_framework_os_spr::MutationDag, error: VcsError) -> Result<T, VcsError> {
        self.displaced_retirements.reserve(1)?;
        self.displaced_retirements.push_reserved(Box::new(ArtifactStoreMutationDagRetirement::new(candidate_dag)));
        Err(error)
    }

    /// 🎯️ Mirrors `applied_edit_ids`/`redo_edit_ids`/`current_checkpoint_id` into
    /// `envelope.cursor` — the single choke point that keeps the persisted cursor in sync with
    /// live undo/redo state. Called from every `bump()`, so every mutating command re-syncs it.
    /// Only the suffixes `dirt` names are rewritten ([`StackMirrorDirt`]); a missing cursor, or one whose group root is
    /// staged, is replaced whole.
    pub fn sync_cursor(&mut self, store_log: &mut StoreLog<P, Mutation>, dirt: StackMirrorDirt) {
        if store_log.envelope.cursor.as_ref().is_none_or(|cursor| cursor.group_visibility().is_some()) {
            let next = Some(ArtifactCursor::new((*store_log.applied_edit_ids).clone(), (*store_log.redo_edit_ids).clone(), (*store_log.current_checkpoint_id).clone()));
            if let Some(previous) = std::mem::replace(&mut store_log.envelope.cursor, next) {
                self.displaced_retirements.push_reserved(Box::new(ArtifactStoreCursorRetirement::new(previous)));
            }
            return;
        }
        let cursor = store_log.envelope.cursor.as_mut().expect("a mirrored cursor remains present");
        let applied = mirror_id_suffix(&mut cursor.applied_edit_ids, &store_log.applied_edit_ids, dirt.applied);
        let redo = mirror_id_suffix(&mut cursor.redo_edit_ids, &store_log.redo_edit_ids, dirt.redo);
        let checkpoint = if cursor.checkpoint_id != *store_log.current_checkpoint_id { std::mem::replace(&mut cursor.checkpoint_id, (*store_log.current_checkpoint_id).clone()) } else { None };
        for displaced in [applied, redo].into_iter().flatten() {
            self.displaced_retirements.push_reserved(Box::new(ArtifactStoreStringVectorRetirement::new(displaced)));
        }
        if let Some(previous) = checkpoint {
            self.displaced_retirements.push_reserved(Box::new(ArtifactStoreStringRetirement::new(previous)));
        }
    }

    /// 🌡️ Retires displaced owners within this change's own bounded grant while the queue is under pressure
    /// ([`ARTIFACT_STORE_DISPLACED_PRESSURE_OCCUPANCY`]): every bump retires at most [`DISPLACED_RELIEF_STEPS`] one-item steps
    /// of [`ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES`] — more than any single change displaces — so a burst of changes without
    /// maintenance turns (a long ingest, a test or tool driving the store directly) keeps the queue below its fixed authority,
    /// at O(change) cost, while the runtime's maintenance rotation still drains the rest. A blocked owner (an outstanding
    /// read) stops the relief.
    pub fn relieve_displaced_pressure(&mut self) -> Result<(), VcsError> {
        for _ in 0..DISPLACED_RELIEF_STEPS {
            if !self.displaced_retirements.under_pressure() {
                break;
            }
            let refused = |error: ValueError| VcsError::ValidationFailed(error.into_message());
            let demand = self.displaced_retirements.demands(ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(refused)?;
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
            match self.displaced_retirements.close_step(grant).map_err(refused)? {
                RetainedCloneStep::Complete(_) => break,
                RetainedCloneStep::Progress(progress) if progress == RetainedCloneProgress::default() => break,
                RetainedCloneStep::Progress(_) => {}
            }
        }
        Ok(())
    }
}

impl<P, Mutation> StoreWire<P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    pub fn durable_group_read_root(&self) -> Option<&semio_framework_os_store_durable_group_wire::lifted_durable_group::ArtifactStoreDurableGroupRootV1<P, Mutation>> {
        self.durable_group_root.as_ref().filter(|root| root.visibility.committed() && !root.adopted)
    }

    pub fn ensure_durable_group_idle(&self) -> Result<(), VcsError> {
        if self.durable_group_root.is_some() {
            return Err(VcsError::ValidationFailed("artifact store belongs to an unresolved durable group decision".into()));
        }
        Ok(())
    }

    /// 🧬️ Returns the current history identity used to invalidate derived work.
    pub fn artifact_revision(&self, store_log: &StoreLog<P, Mutation>) -> ArtifactRevision {
        let group = self.durable_group_read_root();
        ArtifactRevision {
            artifact_id: store_log.envelope.id.clone(),
            schema: store_log.envelope.schema.clone(),
            applied_edit_ids: group.map_or_else(|| (*store_log.applied_edit_ids).clone(), |root| root.applied_edit_ids.clone()),
            redo_edit_ids: group.map_or_else(|| (*store_log.redo_edit_ids).clone(), |root| root.redo_edit_ids.clone()),
            checkpoint_id: (*store_log.current_checkpoint_id).clone(),
        }
    }

    pub fn applied_edit_ids(&self, store_log: &StoreLog<P, Mutation>) -> &semio_framework_os_vcs::HistoryPageStack<String> {
        self.durable_group_read_root().map_or(&store_log.applied_edit_ids, |root| &root.applied_edit_ids)
    }

    /// ↪️ Pending redo stack (edit ids undone since the last fresh `Apply`).
    pub fn redo_edit_ids(&self, store_log: &StoreLog<P, Mutation>) -> &semio_framework_os_vcs::HistoryPageStack<String> {
        self.durable_group_read_root().map_or(&store_log.redo_edit_ids, |root| &root.redo_edit_ids)
    }

    pub fn replace_backbone_retained(&mut self, store_owners: &mut StoreOwners<P, Mutation>, next: Option<Backbones>) -> Result<(), VcsError> {
        store_owners.displaced_retirements.reserve(usize::from(self.backbone.is_some()))?;
        if let Some(previous) = std::mem::replace(&mut *self.backbone, next) {
            store_owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreBackboneRetirement::new(previous)));
        }
        Ok(())
    }

    /// 🔭️ Why a close that refuses in its displaced-owner phase cannot finish: the queue census
    /// plus the staged-group root that holds its reservation open.
    pub fn close_displaced_witness(&self, store_owners: &StoreOwners<P, Mutation>) -> String {
        let group = self.durable_group_root.as_ref().map_or("none", |root| if root.adopted { "adopted" } else { "staged" });
        format!("{}/group={group}", store_owners.displaced_retirements.witness())
    }

    pub fn close_take_backbone_retirement(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> {
        ArtifactStoreBackboneRetirement::admit_attached(&mut self.member_inbox, &mut self.backbone, grant)
    }

    /// 📮️ Announces one composed member's events on THIS store's backbone under that member's
    /// exact lane identity. A composed document owns one replica endpoint — the parent's — so without
    /// this a child lane's edits never leave the process and two replicas diverge on every child edit.
    pub async fn send_member_mutations(&mut self, store_owners: &mut StoreOwners<P, Mutation>, owner: &str, slot: &str, child_id: &str, envelopes: Vec<u8>) -> Result<(), VcsError> {
        if envelopes.is_empty() {
            return Ok(());
        }
        let Some(mut backbone) = self.backbone.take() else { return Ok(()) };
        let result = backbone.send(BackboneMessage::Member { owner: owner.to_string(), slot: slot.to_string(), child_id: child_id.to_string(), envelopes }).await;
        self.replace_backbone_retained(store_owners, Some(backbone))?;
        result
    }

    /// 📬️ Drains every member-addressed message this store pumped off its transport, as
    /// `(owner, slot, child_id, envelopes)` — the composing app resolves the lane and folds it.
    pub fn take_member_inbound(&mut self) -> Vec<(String, String, String, Vec<u8>)> {
        self.member_inbox
            .drain(..)
            .filter_map(|message| match message {
                BackboneMessage::Member { owner, slot, child_id, envelopes } => Some((owner, slot, child_id, envelopes)),
                _ => None,
            })
            .collect()
    }

    /// 📣️ Sends `Genesis` (the initial snapshot pack, identifying the document) followed by the
    /// whole event log in `Mutations` batches no larger than one document-backbone batch.
    pub async fn announce_history(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>) -> Result<(), VcsError> {
        if self.backbone.is_none() {
            return Ok(());
        }
        let genesis = BackboneMessage::Genesis { pack: store_log.envelope.vcs.genesis.pack().to_vec() };
        let mut messages = vec![genesis];
        let mut batch: Vec<semio_framework_os_spr::MutationEnvelope> = Vec::new();
        let mut batch_bytes = 0usize;
        for event in store_log.event_log()? {
            let mut encoded = Vec::new();
            semio_framework_os_spr::encode_envelope(&event, &mut encoded);
            if !batch.is_empty() && batch_bytes + encoded.len() > semio_framework_os_spr::DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES / 2 {
                messages.push(BackboneMessage::Mutations { envelopes: semio_framework_os_spr::encode_envelopes(&std::mem::take(&mut batch)) });
                batch_bytes = 0;
            }
            batch_bytes += encoded.len();
            batch.push(event);
        }
        if !batch.is_empty() {
            messages.push(BackboneMessage::Mutations { envelopes: semio_framework_os_spr::encode_envelopes(&batch) });
        }
        let Some(mut backbone) = self.backbone.take() else { return Ok(()) };
        let mut result = Ok(());
        for message in messages {
            result = backbone.send(message).await;
            if result.is_err() {
                break;
            }
        }
        self.replace_backbone_retained(store_owners, Some(backbone))?;
        result
    }
}

impl<P, Mutation> StoreCursor<P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    pub fn generation(&self, store_wire: &StoreWire<P, Mutation>) -> u64 {
        store_wire.durable_group_read_root().map_or(self.generation, |root| root.generation)
    }

    /// 🪪️ Reads the event-maintained generation at an already-owned operation boundary.
    pub fn generation_now(&self, store_wire: &StoreWire<P, Mutation>) -> u64 {
        self.generation(store_wire)
    }

    pub fn content_revision(&self, store_wire: &StoreWire<P, Mutation>) -> [u8; 32] {
        store_wire.durable_group_read_root().map_or(self.content_revision, |root| root.content_revision)
    }

    /// 🧬️ Reads the fixed-width event-maintained content revision without suspension.
    pub fn content_revision_now(&self, store_wire: &StoreWire<P, Mutation>) -> [u8; 32] {
        self.content_revision(store_wire)
    }

    /// 🎯️ Returns the exact revision plus local generation a projection result must match.
    pub fn projection_stamp(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>) -> ArtifactProjectionStamp {
        ArtifactProjectionStamp { revision: store_wire.artifact_revision(store_log), generation: self.generation(store_wire) }
    }

    /// 📣️ Returns the last successful transition through the shared projection invalidation seam.
    ///
    /// 🌀️ `projection_stamp` is async; `Option::map`'s closure is sync (R10 shape 1).
    pub fn last_projection_invalidation(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>) -> Option<ArtifactProjectionInvalidation> {
        store_wire.durable_group_read_root().map_or(self.last_projection_cause, |root| root.last_projection_cause).map(|cause| ArtifactProjectionInvalidation { cause, stamp: self.projection_stamp(store_log, store_wire) })
    }

    /// 🛂️ Accepts only a result computed for the exact current revision and generation. Accepted
    /// semantic diffs are returned to their owning strict-apply boundary, never applied here.
    ///
    /// 🌀️ A future is consumed by one `.await` (R10 shape 2) — resolved once, reused below.
    pub fn accept_projection_result<Output, Diff>(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, result: ArtifactProjectionResult<Output, Diff>) -> Result<AcceptedArtifactProjection<Output, Diff>, StaleArtifactProjection> {
        let current = self.projection_stamp(store_log, store_wire);
        if result.stamp != current {
            return Err(StaleArtifactProjection { computed_for: result.stamp, current });
        }
        Ok(AcceptedArtifactProjection { output: result.output, proposed_diff: result.proposed_diff })
    }

    pub fn seal_derived_snapshot(&self, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, owner: Arc<P>) -> ArtifactDerivedSnapshot<P> {
        ArtifactDerivedSnapshot { owner, registry: store_owners.snapshot_read_leases.clone(), generation: self.generation(store_wire), revision: self.content_revision(store_wire) }
    }

    pub fn admit_derived_source(&self, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, registry: &SnapshotReadRegistryHandle, generation: u64, revision: [u8; 32]) -> Result<(), VcsError> {
        if !registry.ptr_eq(&store_owners.snapshot_read_leases) || generation != self.generation(store_wire) || revision != self.content_revision(store_wire) {
            return Err(VcsError::ValidationFailed("derived snapshot belongs to another Store or event frontier".into()));
        }
        Ok(())
    }

    /// 🪞️ Issues a member read only from this Store's genuine derived projection capability.
    pub fn snapshot_read_derived(&self, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, source: &ArtifactDerivedSnapshot<P>) -> Result<ErasedSnapshotRead, VcsError>
    where
        P: Sync,
    {
        self.admit_derived_source(store_owners, store_wire, &source.registry, source.generation, source.revision)?;
        if !store_owners.snapshot_read_leases.publish_authority(source.generation, source.revision) {
            return Err(VcsError::ValidationFailed("snapshot read commit authority is busy or exhausted".into()));
        }
        let alias = Arc::clone(&source.owner);
        let lease = store_owners.snapshot_read_leases.try_issue(Arc::clone(&alias)).map_err(|_| VcsError::ValidationFailed("snapshot read lease registry is busy, saturated, or exhausted".into()))?;
        Ok(ErasedSnapshotRead::new(alias, lease))
    }

    /// 🧩️ Derives a provisional member projection through this Store's own operation vocabulary and merge policy.
    pub fn derive_provisional_snapshot(&self, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, source: &ArtifactDerivedSnapshot<P>, op: &Mutation) -> Result<Option<ArtifactDerivedSnapshot<P>>, VcsError> {
        self.admit_derived_source(store_owners, store_wire, &source.registry, source.generation, source.revision)?;
        let outcome = <Mutation as self::Mutation<P>>::diff(op, source.snapshot());
        let applicable = outcome.is_applicable(protocol::MergePolicy::default());
        let (diff, _) = outcome.into_parts();
        let next = applicable.then(|| semio_framework_os_spr::apply_diff(&diff, source.snapshot()).ok()).flatten();
        <<Mutation as self::Mutation<P>>::Diff as MutationDiff<P>>::retire_cold(diff);
        Ok(next.map(|owner| self.seal_derived_snapshot(store_owners, store_wire, Arc::new(owner))))
    }

    /// ✏️ Derives one history preview input with the replay's keep-and-record application rule.
    pub fn derive_history_snapshot(&self, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, source: &ArtifactDerivedSnapshot<P>, op: &Mutation) -> Result<(Option<ArtifactDerivedSnapshot<P>>, Vec<protocol::MutationMessage>), VcsError> {
        self.admit_derived_source(store_owners, store_wire, &source.registry, source.generation, source.revision)?;
        let (diff, mut messages) = <Mutation as self::Mutation<P>>::diff(op, source.snapshot()).into_parts();
        let applied = semio_framework_os_spr::apply_diff(&diff, source.snapshot());
        <<Mutation as self::Mutation<P>>::Diff as MutationDiff<P>>::retire_cold(diff);
        let next = match applied {
            Ok(owner) => Some(self.seal_derived_snapshot(store_owners, store_wire, Arc::new(owner))),
            Err(error) => {
                messages.push(protocol::MutationMessage::fatal(error.code, error.message).at(error.target));
                None
            }
        };
        Ok((next, messages))
    }

    /// 🕰️ Starts a preview cursor without scanning, copying or hashing the live history.
    pub fn begin_derived_history_preview(&self, store_log: &StoreLog<P, Mutation>, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, target: MutationId, drafts: protocol::HistoryInputDrafts) -> Result<ArtifactDerivedHistoryPreview<P, Mutation>, VcsError> {
        store_wire.ensure_durable_group_idle()?;
        Ok(ArtifactDerivedHistoryPreview {
            plan: Some(HistoryReadPlan::new(Some(target), drafts, store_log.applied_edit_ids.len(), false)), plan_retirement:None, drafts: protocol::HistoryInputDrafts::new(), target: None, cursor: 0, operation: 0, state: None,
            progress: ReplayProgress::default(), registry: store_owners.snapshot_read_leases.clone(), generation: self.generation(store_wire), revision: self.content_revision(store_wire), marker: PhantomData, finished: false,
        })
    }

    /// 🏁️ Seals a completed prefix against the same store frontier that admitted its cursor.
    pub fn finish_derived_history_preview(&self, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, owner: &mut Option<ArtifactDerivedHistoryPreview<P, Mutation>>) -> Result<ArtifactDerivedSnapshot<P>, VcsError> {
        let preview = owner.as_mut().ok_or(VcsError::HistoryReplaying)?;
        self.admit_derived_source(store_owners, store_wire, &preview.registry, preview.generation, preview.revision)?;
        Ok(self.seal_derived_snapshot(store_owners, store_wire, preview.take_finished_snapshot_owner()?))
    }

    /// ⏪️ Starts sealed replay planning; every history identity and effective-input copy happens in a later slice.
    pub fn begin_derived_report_replay(&self, store_log: &StoreLog<P, Mutation>, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, drafts: protocol::HistoryInputDrafts, from: Option<MutationId>) -> Result<ArtifactDerivedReplay<P, Mutation>, VcsError> {
        store_wire.ensure_durable_group_idle()?;
        Ok(ArtifactDerivedReplay { replay: None, plan: Some(HistoryReadPlan::new(from, drafts, store_log.applied_edit_ids.len(), true)), plan_retirement:None, registry: store_owners.snapshot_read_leases.clone(), generation: self.generation(store_wire), revision: self.content_revision(store_wire) })
    }

    /// 🏁️ Returns a finished replay and its genuine complete projection for preview and subsequent publication.
    pub fn finish_derived_report_replay(&self, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, owner: &mut Option<ArtifactDerivedReplay<P, Mutation>>) -> Result<(EditReplayResult<P, Mutation>, Option<ArtifactDerivedSnapshot<P>>), VcsError> {
        let replay = owner.as_ref().ok_or_else(|| VcsError::ValidationFailed("derived replay owner is absent".into()))?;
        self.admit_derived_source(store_owners, store_wire, &replay.registry, replay.generation, replay.revision)?;
        if !replay.replay.as_ref().is_some_and(|replay|replay.is_finished()) { return Err(VcsError::ValidationFailed("derived replay has not finished".into())); }
        let replay = owner.take().expect("admitted finished derived replay owner is present");
        let result = replay.replay.expect("finished replay cursor").finish().expect("admitted finished replay completes without another fallible operation");
        let head = result.state().map(|owner| self.seal_derived_snapshot(store_owners, store_wire, Arc::clone(owner)));
        Ok((result, head))
    }

    pub fn revision_owner_slots(value: &CursorRevisionAccumulator) -> usize {
        usize::from(!value.applied.is_empty() || value.applied.capacity() != 0 || !value.redo.is_empty() || value.redo.capacity() != 0 || !value.mutation_positions.is_empty() || !value.indexed_edits.is_empty() || !value.unit_flags.is_empty())
    }

    pub fn push_revision_replacement_reserved(&mut self, store_owners: &mut StoreOwners<P, Mutation>, next: CursorRevisionAccumulator) {
        let previous = std::mem::replace(&mut *self.revision_accumulator, next);
        if !previous.applied.is_empty() || previous.applied.capacity() != 0 || !previous.redo.is_empty() || previous.redo.capacity() != 0 || !previous.mutation_positions.is_empty() || !previous.indexed_edits.is_empty() || !previous.unit_flags.is_empty() {
            store_owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreRevisionAccumulatorRetirement::new(previous)));
        }
    }

    fn replace_revision_accumulator_retained(&mut self, store_owners: &mut StoreOwners<P, Mutation>, next: CursorRevisionAccumulator) -> Result<(), VcsError> {
        store_owners.displaced_retirements.reserve(Self::revision_owner_slots(&self.revision_accumulator))?;
        self.push_revision_replacement_reserved(store_owners, next);
        Ok(())
    }

    //#region 🔖️TimeTravelReads
    /// 📍️ One exact identity's current applied position, validated against the maintained revision authority.
    pub fn mutation_position(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, target: &MutationId) -> Result<(usize, usize), VcsError> { self.locate_mutation(store_log, store_wire, target) }

    /// 🔎️ The maintained applied position of `edit_id`, fenced by the exact current cursor identity.
    pub fn applied_edit_position(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, edit_id: &str) -> Option<usize> {
        let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit_id.as_bytes()]);
        let root = store_wire.durable_group_read_root().and_then(|root| root.revision_accumulator.as_ref());
        let position = root.and_then(|root| root.indexed_edits.get(&digest)).or_else(|| self.revision_accumulator.indexed_edits.get(&digest))?.0;
        store_wire.applied_edit_ids(store_log).get(position).filter(|id| id.as_str() == edit_id).map(|_| position)
    }

    pub fn applied_revision(&self, store_wire: &StoreWire<P, Mutation>) -> &CursorRevisionAccumulator {
        store_wire.durable_group_read_root().map_or(&self.revision_accumulator, |root| root.revision_accumulator.as_ref().expect("committed group retains its exact revision"))
    }

    pub fn applied_unit_flag(&self, store_wire: &StoreWire<P, Mutation>, edit: [u8; 32], operation: usize) -> bool {
        store_wire.durable_group_read_root().and_then(|root| root.revision_accumulator.as_ref()).and_then(|root| root.unit_flags.get(&(edit, operation))).or_else(|| self.revision_accumulator.unit_flags.get(&(edit, operation))).copied().unwrap_or(false)
    }

    /// 📖️ The applied edit at `position`.
    pub fn applied_edit(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, position: usize) -> Result<&Edit<Mutation>, VcsError> {
        let edit_id = store_wire.applied_edit_ids(store_log).get(position).ok_or_else(|| VcsError::ValidationFailed(format!("no applied edit at position {position}")))?;
        self.applied_revision(store_wire).applied.get(position).and_then(|record| record.ledger_key).and_then(|key| store_log.envelope.vcs.edits.get_key(key)).filter(|edit| edit.id == *edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))
    }

    /// 📍️ The applied position of the edit owning `mutation_id` and the operation's index within it.
    pub fn locate_mutation(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, mutation_id: &MutationId) -> Result<(usize, usize), VcsError> {
        let root = store_wire.durable_group_read_root().and_then(|root| root.revision_accumulator.as_ref());
        if let Some((position, index, digest)) = root.and_then(|root| root.mutation_positions.get(mutation_id)).or_else(|| self.revision_accumulator.mutation_positions.get(mutation_id)) {
            if store_wire.applied_edit_ids(store_log).get(*position).filter(|id| CursorRevisionAccumulator::hash_record(b"edit-id", &[id.as_bytes()]) == *digest).is_some() {
                let edit = self.applied_edit(store_log, store_wire, *position)?;
                if semio_framework_os_spr::mutation_id_for_edit_operation::<P, Mutation>(edit, *index).as_ref() == Some(mutation_id) { return Ok((*position, *index)); }
            }
        }
        Err(VcsError::ValidationFailed(format!("{} is not an applied operation", mutation_id.0)))
    }

    /// 🪬️ This store's part of the unit of `mutation_id`, in operation order: every operation its edit recorded under the
    /// unit's group, or the operation alone when it is its own unit ([`Self::unit_id`]).
    pub fn unit_operations(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, mutation_id: &MutationId) -> Result<Vec<MutationId>, VcsError> {
        let (position, index) = self.locate_mutation(store_log, store_wire, mutation_id)?;
        let edit = self.applied_edit(store_log, store_wire, position)?;
        let group = edit.mutation_meta.get(index).and_then(|meta| meta.group_id.as_ref());
        if group.is_none() || self.applied_edit_units(store_log, store_wire, position)?[index].is_none() {
            return Ok(vec![mutation_id.clone()]);
        }
        Ok(semio_framework_os_spr::mutation_ids_for_edit::<P, Mutation>(edit).into_iter().enumerate().filter(|(member, _)| edit.mutation_meta.get(*member).and_then(|meta| meta.group_id.as_ref()) == group).map(|(_, member)| member).collect())
    }

    /// 🫴️ Recorded unit facts captured against each original operation's actual historical prestate by publication,
    /// initialization and replay; metadata supplies the transaction origin and the gesture's current group identity.
    pub fn applied_edit_units(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, position: usize) -> Result<Vec<Option<String>>, VcsError> {
        let edit = self.applied_edit(store_log, store_wire, position)?;
        let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit.id.as_bytes()]);
        Ok((0..edit.forwards.len()).map(|index| {
            let meta = edit.mutation_meta.get(index);
            let spans = self.applied_unit_flag(store_wire, digest, index) || matches!(meta.map(|meta| &meta.origin), Some(semio_framework_os_spr::MutationOrigin::Transaction { .. }));
            spans.then(|| meta.and_then(|meta| meta.group_id.clone()).unwrap_or_else(|| semio_framework_os_spr::mutation_id_for_edit_operation::<P, Mutation>(edit, index).expect("recorded unit identity").0))
        }).collect())
    }

    //#endregion 🔖️TimeTravelReads
    //#region 📬️OneItemPublication
    /// 🎯️ Publishes exactly one durable lane item after validating the live generation.
    /// The returned lane witness is independent of the edit receipt so a retained publisher can
    /// validate, ACK, retry, and retire one semantic unit without entering a batch API.
    /// 🎟️ Admits ONE gesture's exact app-owned retained mutation preparations. No generic mutation
    /// method is called here or by `advance_apply_batch`; absence of a domain factory is explicit.
    /// A single-mutation publication is simply the `mutations.len() == 1` case of this one machine.

    /// 🎟️ Admits the original captured actor, input backing and transaction before any constructor work.
    pub fn admit_apply_batch(
        &self, store_log: &StoreLog<P, Mutation>, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, operation:semio_framework_job::OperationId, expected_generation:u64, expected_revision:[u8;32],
        actor:&mut Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_value::SharedUtf8>>,
        mutations:&mut Vec<Mutation>, lane:HistoryLane,
        factory:Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P,Mutation>>>,
        transaction:&mut Option<protocol::TransactionRef>, verb:Option<&str>, outbound:bool, grant:RetainedCloneGrant,
    )->Result<Option<(ArtifactStoreBatchPublication<P,Mutation>,RetainedCloneProgress)>,ValueError>
    where P:Sync,Mutation:Send {
        use semio_framework_value::{ValueRefusalKind,retirement::controlled::ControlledRetirement};
        let factory=factory.ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"batched publication requires its original typed preparation issuer"))?;
        let retirement=store_owners.mutation_retirement_factory.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"batched publication requires its original mutation retirement issuer"))?;
        let original_actor=actor.as_ref().filter(|owner|owner.original_is_untouched()).and_then(ControlledRetirement::original).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication requires its original paid actor capture"))?;
        if mutations.is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication requires original mutation inputs"));}
        if store_wire.durable_group_root.is_some()||store_log.envelope.open_transaction.as_ref().is_some_and(|open|transaction.as_ref().is_none_or(|transaction|transaction.id!=open.transaction.id)){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication has an unresolved original transaction or group decision"));}
        if self.generation!=expected_generation||self.content_revision!=expected_revision{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication original generation or revision is stale"));}
        if lane!=HistoryLane::Document||store_wire.backbone.is_some()&&!outbound{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"batched publication requires its original document/outbound lane"));}
        if original_actor.is_empty()||original_actor.len()>ARTIFACT_STORE_ONE_ITEM_ID_BYTES{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication actor exceeds its original identity bound"));}
        let cursor=store_log.envelope.cursor.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication requires its original live cursor"))?;
        if cursor.applied_edit_ids.len()!=store_log.applied_edit_ids.len()||cursor.redo_edit_ids.len()!=store_log.redo_edit_ids.len()||cursor.checkpoint_id.as_deref()!=store_log.current_checkpoint_id.as_deref(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication original cursor is stale"));}
        let next_sequence_number=store_log.edit_sequence.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication original sequence is exhausted"))?;
        let mut next_clock=store_log.clock;next_clock.tick(now_ms());
        match factory.stamped_clock(){Some(stamped)if store_log.edit_sequence==0||(stamped.physical_ms,stamped.logical)>(store_log.clock.physical_ms,store_log.clock.logical)=>next_clock=stamped,Some(_)=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication original stamp does not follow the Store clock")),None=>{}}
        let stamped=factory.stamped_mutation_id().map(|identity|identity.0.as_str());let line=store_log.envelope.active_alternative_id.as_deref();
        if stamped.is_some_and(|text|text.is_empty()||text.len()>ARTIFACT_STORE_ONE_ITEM_ID_BYTES)||line.is_some_and(|text|text.len()>ARTIFACT_STORE_ONE_ITEM_ID_BYTES)||verb.is_some_and(|text|text.len()>ARTIFACT_STORE_ONE_ITEM_ID_BYTES){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication original metadata exceeds its admitted identity bound"));}
        let text_bytes=stamped.map_or(0,str::len)+line.map_or(0,str::len)+verb.map_or(0,str::len);
        let copied_bytes=size_of::<ArtifactTypedBatchItemAuthority<P,Mutation>>()+size_of::<ArtifactStoreBatchSourceOf<P,Mutation,ArtifactTypedBatchItemAuthority<P,Mutation>>>()+size_of::<ArtifactStoreOneItemLiveAuthority>()+size_of::<ArtifactStoreBatchPublication<P,Mutation>>()+size_of::<Vec<Mutation>>()+size_of::<Option<protocol::TransactionRef>>()+size_of_val(actor)+text_bytes;
        let retained_capacity_bytes=size_of::<ArtifactStoreBatchSourceOf<P,Mutation,ArtifactTypedBatchItemAuthority<P,Mutation>>>()+semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<ArtifactStoreOneItemLiveAuthority>()+text_bytes;
        if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_capacity_bytes<retained_capacity_bytes||grant.maximum_depth==0{return Ok(None);}
        let text=|original:&str|{let mut value=String::with_capacity(original.len());value.push_str(original);value};
        let stamped_edit_id=stamped.map(text);let line=line.map(text);let verb=verb.map(text);
        let admitted_items=mutations.len();
        let original_actor=actor.as_mut().unwrap().take_original().expect("observed untouched original captured actor");drop(actor.take());
        let source=ArtifactStoreBatchSourceOf::new(ArtifactTypedBatchItemAuthority{preparation:Arc::clone(factory),retirement:Arc::clone(retirement)},std::mem::take(mutations),None,store_log.operation_wire);
        let authority=Arc::new(ArtifactStoreOneItemLiveAuthority::mint_for_store(store_log.mint_witness(), operation, semio_framework_job::Generation(expected_generation), expected_revision, store_log.applied_edit_ids.len(), next_sequence_number, next_clock, original_actor, line, None, stamped_edit_id));
        let publication=ArtifactStoreBatchPublication{operation,expected_generation,expected_revision,lane,footprint:ArtifactStoreOneItemFootprint{work_items:0,retained_bytes:0},source_census_completed:0,source_census_finished:false,admitted_items,authority:Some(authority),authority_retirement:None,source:Some(Box::new(source)),preparation:None,item_closing:false,stage:None,group_preparation:None,retained_checkpoint:Default::default(),receipt:None,attempts:0,published:false,cancel_requested:false,close_started:false,fault:None,refused_base:None,refused_authority:None,refused_issuers:[None,None],phase:ArtifactStoreOneItemPublicationPhase::Preparing,verb,transaction:transaction.take(),outbound,announce_from:0,announce_items:0,transaction_open:false};
        Ok(Some((publication,RetainedCloneProgress{copied_items:1,copied_bytes,retained_capacity_bytes,released_bytes:0})))
    }

    pub fn begin_apply_batch(
        &self, store_log: &StoreLog<P, Mutation>, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>,
        operation: semio_framework_job::OperationId,
        expected_generation: u64,
        expected_revision: [u8; 32],
        actor: semio_framework_value::SharedUtf8,
        mutations: Vec<Mutation>,
        lane: HistoryLane,
        factory: Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>>,
        transaction: Option<protocol::TransactionRef>,
    ) -> Result<ArtifactStoreBatchPublication<P, Mutation>, ArtifactStoreBatchAdmissionRejected<Mutation>>
    where
        P: Sync,
        Mutation: Send,
    {
        self.begin_typed_apply_batch(store_log, store_owners, store_wire, operation, expected_generation, expected_revision, actor, mutations, lane, factory, false, transaction)
    }

    /// 📤️ Admits ONE gesture exactly like [`Self::begin_apply_batch`], but on a store with an attached
    /// backbone: the one edit it publishes stays unannounced until [`Self::flush_published_apply_batch`]
    /// sends it as ONE `BackboneMessage::Mutations` batch, and the publication refuses its ACK before.
    /// See `📋️tool-run-contract.md` §2.7.4 (ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS).
    #[allow(clippy::too_many_arguments)]
    pub fn begin_outbound_apply_batch(
        &self, store_log: &StoreLog<P, Mutation>, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>,
        operation: semio_framework_job::OperationId,
        expected_generation: u64,
        expected_revision: [u8; 32],
        actor: semio_framework_value::SharedUtf8,
        mutations: Vec<Mutation>,
        factory: Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>>,
        transaction: Option<protocol::TransactionRef>,
    ) -> Result<ArtifactStoreBatchPublication<P, Mutation>, ArtifactStoreBatchAdmissionRejected<Mutation>>
    where
        P: Sync,
        Mutation: Send,
    {
        self.begin_typed_apply_batch(store_log, store_owners, store_wire, operation, expected_generation, expected_revision, actor, mutations, HistoryLane::Document, factory, true, transaction)
    }

    #[allow(clippy::too_many_arguments)]
    fn begin_typed_apply_batch(
        &self, store_log: &StoreLog<P, Mutation>, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>,
        operation: semio_framework_job::OperationId,
        expected_generation: u64,
        expected_revision: [u8; 32],
        actor: semio_framework_value::SharedUtf8,
        mutations: Vec<Mutation>,
        lane: HistoryLane,
        factory: Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>>,
        outbound: bool,
        transaction: Option<protocol::TransactionRef>,
    ) -> Result<ArtifactStoreBatchPublication<P, Mutation>, ArtifactStoreBatchAdmissionRejected<Mutation>>
    where
        P: Sync,
        Mutation: Send,
    {
        let Some(factory) = factory else {
            return Err(ArtifactStoreBatchAdmissionRejected { reason: "batched publication requires an explicit app-owned ArtifactStoreOneItemPreparationFactory".into(), mutations });
        };
        let Some(retirement) = store_owners.mutation_retirement_factory.as_ref() else { return Err(ArtifactStoreBatchAdmissionRejected { reason: "batched inputs require their original mutation retirement authority".into(), mutations }); };
        let source = ArtifactStoreBatchSourceOf::new(ArtifactTypedBatchItemAuthority { preparation: Arc::clone(factory), retirement: Arc::clone(retirement) }, mutations, None, store_log.operation_wire);
        self.begin_apply_batch_owned(store_log, store_wire, operation, expected_generation, expected_revision, actor, lane, None, source, outbound, transaction).map_err(|rejected| {
            let (reason, mutations) = rejected.into_owners();
            ArtifactStoreBatchAdmissionRejected { reason, mutations }
        })
    }

    /// 🎟️ Borrows the original installed catalog and publication inputs under the caller's full turn.
    pub fn admit_member_apply_batch(
        &self, store_log: &StoreLog<P, Mutation>, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, operation:semio_framework_job::OperationId, expected_generation:u64, expected_revision:[u8;32],
        actor:&mut Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_value::SharedUtf8>>,
        mutations:&mut Vec<Mutation>, transaction:&mut Option<protocol::TransactionRef>, grant:RetainedCloneGrant,
    )->Result<Option<(ArtifactStoreBatchPublication<P,Mutation>,RetainedCloneProgress)>,ValueError>
    where P:Sync,Mutation:Send {
        self.admit_apply_batch(store_log, store_owners, store_wire, operation,expected_generation,expected_revision,actor,mutations,HistoryLane::Document,store_owners.one_item_preparation_factory.as_ref(),transaction,None,false,grant)
    }

    /// 🧩 Uses the exact typed factory installed by the member's owner catalog.
    pub fn begin_member_apply_batch(
        &self, store_log: &StoreLog<P, Mutation>, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>,
        operation: semio_framework_job::OperationId,
        expected_generation: u64,
        expected_revision: [u8; 32],
        actor: semio_framework_value::SharedUtf8,
        mutations: Vec<Mutation>,
    ) -> Result<ArtifactStoreBatchPublication<P, Mutation>, ArtifactStoreBatchAdmissionRejected<Mutation>>
    where
        P: Sync,
        Mutation: Send,
    {
        self.begin_apply_batch(store_log, store_owners, store_wire, operation, expected_generation, expected_revision, actor, mutations, HistoryLane::Document, store_owners.one_item_preparation_factory.as_ref(), None)
    }

    /// ⏰️ A stamped clock is compared against the document's OWN history, not against the
    /// wall clock this Store happened to be constructed at. `clock` is seeded
    /// `HybridLogicalTimestamp::new(0, now_ms())` at construction, so a Store that has published
    /// no edit of its own carries a machine fact, not a document fact, and rejecting a stamp
    /// against it refused every first publication whose stamp was minted anywhere but this
    /// process's `now_ms()`. Once this Store has published an edit, `clock` IS the last edit's
    /// stamp (the commit path assigns `self.clock = next_clock`), and strictly-after is then
    /// exactly the hybrid-logical invariant the ordering depends on.
    ///
    /// 🔏 The stamped identity travels with the stamped clock: a durable committer hashes its
    /// canonical command over BOTH before the Store is reached, so an authority that minted its
    /// own content-addressed `edit_id()` published an edit that no verifier could bind back to
    /// the approval it executed. It is admitted under the same fixed identity capacity every
    /// other Store identity answers to.
    #[allow(clippy::too_many_arguments)]
    pub fn begin_apply_batch_owned<A>(
        &self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>,
        operation: semio_framework_job::OperationId,
        expected_generation: u64,
        expected_revision: [u8; 32],
        actor: semio_framework_value::SharedUtf8,
        lane: HistoryLane,
        group_id: Option<String>,
        source: ArtifactStoreBatchSourceOf<P, Mutation, A>,
        outbound: bool,
        transaction: Option<protocol::TransactionRef>,
    ) -> Result<ArtifactStoreBatchPublication<P, Mutation>, ArtifactStoreBatchAdmissionRejected<A::Input>>
    where
        P: Sync + Send,
        A: ArtifactStoreBatchItemAuthority<P, Mutation> + 'static,
        Mutation: 'static,
        P: 'static,
    {
        let reject = |reason: String, source: ArtifactStoreBatchSourceOf<P, Mutation, A>| ArtifactStoreBatchAdmissionRejected { reason, mutations: source.inputs.into() };
        if source.inputs.is_empty() {
            return Err(reject("batched publication requires at least one admitted mutation owner".into(), source));
        }
        if store_wire.durable_group_root.is_some() {
            return Err(reject("batched publication cannot interleave with an unresolved durable group decision".into(), source));
        }
        if let Some(open) = store_log.envelope.open_transaction.as_ref().filter(|open| transaction.as_ref().is_none_or(|transaction| transaction.id != open.transaction.id)) {
            return Err(reject(format!("batched publication cannot interleave with the open tool transaction {}", open.transaction.id), source));
        }
        if self.generation != expected_generation || self.content_revision != expected_revision {
            return Err(reject("batched publication base generation or revision is stale".into(), source));
        }
        if lane != HistoryLane::Document {
            return Err(reject("batched publication has no retained side-lane map preparation authority".into(), source));
        }
        if store_wire.backbone.is_some() && !outbound {
            return Err(reject("batched publication has no retained outbound backbone encoder/sender authority".into(), source));
        }
        if actor.is_empty() || actor.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err(reject("batched publication actor exceeds its fixed identity capacity".into(), source));
        }
        if group_id.as_ref().is_some_and(|id| id.is_empty() || id.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err(reject("batched publication group exceeds its fixed identity capacity".into(), source));
        }
        let Some(cursor) = store_log.envelope.cursor.as_ref() else {
            return Err(reject("batched publication requires an initialized live cursor authority".into(), source));
        };
        if cursor.applied_edit_ids.len() != store_log.applied_edit_ids.len() || cursor.redo_edit_ids.len() != store_log.redo_edit_ids.len() || cursor.checkpoint_id.as_deref() != store_log.current_checkpoint_id.as_deref() {
            return Err(reject("batched publication live cursor is stale or lacks preinstalled fixed capacity".into(), source));
        }
        let next_sequence_number = match store_log.edit_sequence.checked_add(1) {
            Some(value) => value,
            None => return Err(reject("batched edit sequence exhausted".into(), source)),
        };
        let mut next_clock = store_log.clock;
        next_clock.tick(now_ms());
        match source.authority.as_ref().and_then(ArtifactStoreBatchItemAuthority::stamped_clock) {
            Some(stamped) if store_log.edit_sequence == 0 || (stamped.physical_ms, stamped.logical) > (store_log.clock.physical_ms, store_log.clock.logical) => next_clock = stamped,
            Some(_) => return Err(reject("stamped publication clock is not strictly after this Store's own last published edit".into(), source)),
            None => {}
        }
        let stamped_edit_id = match source.authority.as_ref().and_then(ArtifactStoreBatchItemAuthority::stamped_mutation_id) {
            Some(identity) if identity.0.is_empty() || identity.0.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES => {
                return Err(reject("stamped publication identity exceeds its fixed identity capacity".into(), source));
            }
            Some(identity) => Some(identity.0.clone()),
            None => None,
        };
        let admitted_items = source.inputs.len();
        let authority = Arc::new(ArtifactStoreOneItemLiveAuthority::mint_for_store(
            store_log.mint_witness(),
            operation,
            semio_framework_job::Generation(expected_generation),
            expected_revision,
            store_log.applied_edit_ids.len(),
            next_sequence_number,
            next_clock,
            actor,
            store_log.envelope.active_alternative_id.clone(),
            group_id,
            stamped_edit_id,
        ));
        Ok(ArtifactStoreBatchPublication {
            operation,
            expected_generation,
            expected_revision,
            lane,
            footprint: ArtifactStoreOneItemFootprint { work_items: 0, retained_bytes: 0 },
            source_census_completed: 0,
            source_census_finished: false,
            admitted_items,
            authority: Some(authority),
            authority_retirement: None,
            source: Some(Box::new(source)),
            preparation: None,
            item_closing: false,
            stage: None,
            group_preparation: None,
            retained_checkpoint: ArtifactStoreOneItemCheckpoint::default(),
            receipt: None,
            attempts: 0,
            published: false,
            cancel_requested: false,
            close_started: false,
            fault: None,
            refused_base: None,
            refused_authority: None,
            refused_issuers: [None, None],
            phase: ArtifactStoreOneItemPublicationPhase::Preparing,
            verb: None,
            transaction,
            outbound,
            announce_from: 0,
            announce_items: 0,
            transaction_open: false,
        })
    }

    //#endregion 📬️OneItemPublication
    //#region 🔖️EventSourcedHistory
    /// 🎴️ Takes what a finished Report replay publishes — its messages and reached projection — and installs its unit flags
    /// and rebased inverses; every untaken owner stays attached for [`Self::retire_replay_result`].
    pub fn take_report_replay(&mut self, store_log: &mut StoreLog<P, Mutation>, result: &mut EditReplayResult<P, Mutation>) -> Result<(Vec<semio_framework_os_spr::EditMessages>, Arc<P>), VcsError> {
        self.revision_accumulator.unit_flags.extend(std::mem::take(&mut result.unit_flags));
        let replayed = result.take_replayed();
        if let Some(entry) = replayed.iter().find(|entry| entry.edit_id.len() > ARTIFACT_EDIT_MESSAGE_ID_BYTES) {
            return Err(VcsError::ValidationFailed(format!("replayed edit id {} exceeds its fixed byte authority", entry.edit_id)));
        }
        let state = result.take_state().ok_or_else(|| VcsError::ValidationFailed("report replay lost its reached projection".into()))?;
        for (edit_id, inverse) in result.take_rebased_undo_rows() {
            match store_log.envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id) {
                Some(edit) => retire_scratch_operations::<P, Mutation>(std::mem::replace(&mut edit.inverse, inverse)),
                None => retire_scratch_operations::<P, Mutation>(inverse),
            }
        }
        Ok((replayed, state))
    }

    //#endregion 🔖️EventSourcedHistory
    /// 🖍️ The applied tail edit's recorded content changed outside a mutation (a group or origin stamp): the next
    /// {@link bump} rebuilds its revision record.
    pub fn restamp_applied_tail(&mut self, store_log: &mut StoreLog<P, Mutation>) {
        self.mirror_dirt.applied = self.mirror_dirt.applied.min(store_log.applied_edit_ids.len().saturating_sub(1));
    }

    /// ➕️ Pushes `edit_id` onto the applied stack; its mirrors are stale from the new tail on.
    pub fn push_applied_edit_id(&mut self, store_log: &mut StoreLog<P, Mutation>, edit_id: String) {
        self.mirror_dirt.applied = self.mirror_dirt.applied.min(store_log.applied_edit_ids.len());
        store_log.applied_edit_ids.push(edit_id);
    }

    /// ➕️ Pushes `edit_id` onto the redo stack; its mirrors are stale from the new top on.
    pub fn push_redo_edit_id(&mut self, store_log: &mut StoreLog<P, Mutation>, edit_id: String) {
        self.mirror_dirt.redo = self.mirror_dirt.redo.min(store_log.redo_edit_ids.len());
        store_log.redo_edit_ids.push(edit_id);
    }

    /// ➖️ Pops the redo top; its mirrors are stale past the new length.
    pub fn pop_redo_edit_id(&mut self, store_log: &mut StoreLog<P, Mutation>) -> Option<String> {
        let edit_id = store_log.redo_edit_ids.pop()?;
        self.mirror_dirt.redo = self.mirror_dirt.redo.min(store_log.redo_edit_ids.len());
        Some(edit_id)
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
impl<P, Mutation> StoreOwners<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    pub fn factory_aliases_present(&self) -> bool { self.snapshot_retirement_factory.is_some() || self.initial_snapshot_retirement_factory.is_some() || self.mutation_retirement_factory.is_some() || self.one_item_preparation_factory.is_some() || self.replay_preparation_factory.is_some() || self.one_item_wire_preparation_factory.is_some() }
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
/// 🩹️ A scratch owner — a working projection, the operations of a command, an edit not yet in the ledger — that leaves
/// through its technology's cold retirement on EVERY exit path until [`Self::adopt`] hands it to the store. An artifact whose
/// projection or operation owns a fail-closed root aborts the guest on a bare drop, so a command the store refuses (a rejected
/// apply, a refused inverse, an encode failure, a failing diff, a saturated ledger) must never let what it built reach `Drop`.
/// Holding it here makes that structural: a new early return cannot miss it.
pub struct ScratchOwner<T> {
    pub owner: Option<T>,
    pub retire: fn(T),
}

impl<T> ScratchOwner<T> {
    pub fn new(owner: T, retire: fn(T)) -> Self {
        Self { owner: Some(owner), retire }
    }

    /// 🤝️ Hands the owner to the store, which owns its retirement from here on.
    pub fn adopt(mut self) -> T {
        self.owner.take().expect("a scratch owner is adopted exactly once")
    }
}

impl<T> std::ops::Deref for ScratchOwner<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.owner.as_ref().expect("a scratch owner is present until it is adopted")
    }
}

impl<T> std::ops::DerefMut for ScratchOwner<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.owner.as_mut().expect("a scratch owner is present until it is adopted")
    }
}

impl<T> Drop for ScratchOwner<T> {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            (self.retire)(owner);
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
/// 🧮️ Folds the genesis over the effective forwards of `applied_edit_ids` ([`fold_effective_edit`]).
pub async fn fold_history<P, Mutation, I, S>(envelope: &ArtifactEnvelope<P, Mutation>, applied_edit_ids: I, supersessions: &EffectiveSupersessions) -> Result<P, VcsError>
where
    P: Clone,
    Mutation: self::Mutation<P> + OpBinary,
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut snapshot = ReplayProjection::<P, Mutation>::new(envelope.vcs.genesis.facts().snapshot().clone());
    let mut seen = HashSet::new();
    for edit_id in applied_edit_ids {
        let edit_id = edit_id.as_ref();
        if !seen.insert(edit_id.to_string()) {
            return Err(VcsError::ValidationFailed(format!("history repeats applied edit {edit_id}")));
        }
        let edit = envelope.vcs.edits.find_near(|entry| entry.id == edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.to_string()))?;
        fold_effective_edit(&mut snapshot, edit, &envelope.schema, supersessions, usize::MAX);
    }
    Ok(snapshot.into_inner())
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
