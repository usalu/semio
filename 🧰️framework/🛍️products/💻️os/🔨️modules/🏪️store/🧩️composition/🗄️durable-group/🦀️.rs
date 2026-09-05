//! 🗺️ Closed codec and admission fence for one durable parent+drawing+value Store decision.

use super::CursorRevisionAccumulator;
use crate::os_dsl::{DslField, DslValue, FieldValue, FromValue as ValueFromValue, Shape, ToValue as ValueToValue};
use crate::os_spr::{Edit, HybridLogicalTimestamp, Mutation as StoreMutation};
use crate::os_store::{pack_rt, ArtifactDsl, ArtifactPack, ArtifactStore, ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPrepared, OwnerRef, PackDecodeOptions, PackEncodeOptions, PackError, PackVerificationLevel, TextError};
use semio_framework_value_derive::{FromValue, ToValue};
use std::sync::Arc;

pub const DURABLE_OWNED_GROUP_DECISION_SCHEMA_V1: &str = "semio.store.durable-owned-three-member-decision.v1";
pub const DURABLE_OWNED_GROUP_ANCHOR_SCHEMA_V1: &str = "semio.store.owned-three-member-anchor.v1";
pub const DURABLE_OWNED_GROUP_SHAPE_V1: &str = "parent-drawing-value";
pub const DURABLE_OWNED_GROUP_EVENT_MAX_BYTES: usize = 491_520;
pub const DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES: usize = 162_000;
pub const DURABLE_OWNED_GROUP_STRUCTURAL_MAX_BYTES: usize = 4_096;
pub const DURABLE_OWNED_GROUP_ID_MAX_BYTES: usize = 256;
const DURABLE_OWNED_GROUP_UNBOUND_OUTCOME_SCHEMA_V1: &str = "semio.store.unbound-one-item-outcome.v1";
const DURABLE_OWNED_GROUP_BOUND_OUTCOME_SCHEMA_V1: &str = "semio.store.bound-one-item-outcome.v1";
const DURABLE_OWNED_GROUP_JSON_MAX_DEPTH: usize = 32;
const DURABLE_OWNED_GROUP_JSON_MAX_ITEMS: usize = 65_536;

const PARENT_ROLE: &str = "parent";
const DRAWING_ROLE: &str = "drawing";
const VALUE_ROLE: &str = "value";
const PARENT_RECOVERY_SCHEMA: &str = "semio.store.one-item-outcome.gis-gismap-v1";
const DRAWING_RECOVERY_SCHEMA: &str = "semio.store.one-item-outcome.stdio-drawing-v1";
const VALUE_RECOVERY_SCHEMA: &str = "semio.store.one-item-outcome.stdio-value-v1";

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DurableOwnedGroupAnchorV1 {
    pub(crate) schema: String,
    pub(crate) parent: crate::os_io::ArtifactRef,
    pub(crate) shape: String,
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DurableOwnedGroupMemberV1 {
    pub(crate) role: String,
    pub(crate) reference: crate::os_io::ArtifactRef,
    pub(crate) owner: Option<OwnerRef>,
    pub(crate) expected_generation: u64,
    pub(crate) expected_revision: [u8; 32],
    pub(crate) recovery_schema: String,
    pub(crate) recovery_pack: Vec<u8>,
    pub(crate) recovery_pack_sha256: String,
    pub(crate) unbound_outcome_sha256: String,
    pub(crate) post_generation: u64,
    pub(crate) post_revision: [u8; 32],
}

pub(super) struct DurableStorePreparedOutcomesV1 {
    pub(super) parent: DurableStorePreparedOutcomeV1,
    pub(super) drawing: DurableStorePreparedOutcomeV1,
    pub(super) value: DurableStorePreparedOutcomeV1,
}

macro_rules! value_field {
    ($kind:ty) => {
        impl DslField for $kind {
            fn shape() -> Shape {
                Shape::Value
            }
            fn to_value(&self) -> FieldValue {
                FieldValue::Value(crate::os_dsl::to_dsl_value(self).expect("typed value conversion"))
            }
            fn from_value(value: &FieldValue) -> Result<Self, String> {
                match value {
                    FieldValue::Value(value) => crate::os_dsl::from_dsl_value(value.clone()),
                    other => Err(format!("expected Value, found {other:?}")),
                }
            }
        }
    };
}

value_field!(DurableOwnedGroupAnchorV1);
value_field!(DurableOwnedGroupMemberV1);

#[derive(Clone, Debug, PartialEq, Eq, ToValue)]
#[value(rename_all = "camelCase")]
struct DurableOwnedThreeMemberUnsignedMemberV1 {
    role: String,
    reference: crate::os_io::ArtifactRef,
    owner: Option<OwnerRef>,
    expected_generation: u64,
    expected_revision: [u8; 32],
    recovery_schema: String,
    unbound_outcome_sha256: String,
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue)]
#[value(rename_all = "camelCase")]
struct DurableOwnedThreeMemberUnsignedV1 {
    schema: String,
    anchor: DurableOwnedGroupAnchorV1,
    parent: DurableOwnedThreeMemberUnsignedMemberV1,
    drawing: DurableOwnedThreeMemberUnsignedMemberV1,
    value: DurableOwnedThreeMemberUnsignedMemberV1,
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, crate::os_dsl::DslArtifact)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(id = "store.durable-owned-three-member-decision")]
pub struct DurableOwnedThreeMemberDecisionV1 {
    pub(crate) schema: String,
    pub(crate) anchor: DurableOwnedGroupAnchorV1,
    pub(crate) anchor_sha256: String,
    pub(crate) decision_sha256: String,
    pub(crate) parent: DurableOwnedGroupMemberV1,
    pub(crate) drawing: DurableOwnedGroupMemberV1,
    pub(crate) value: DurableOwnedGroupMemberV1,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DurableOwnedGroupDecisionError {
    InvalidSchema,
    InvalidIdentity,
    InvalidOwner,
    InvalidFrontier,
    InvalidHash,
    InvalidOutcome,
    NonCanonical,
    RecoveryPackTooLarge,
    EventTooLarge,
    Codec(String),
}

impl std::fmt::Display for DurableOwnedGroupDecisionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSchema => formatter.write_str("durable owned group schema is invalid"),
            Self::InvalidIdentity => formatter.write_str("durable owned group identity is invalid"),
            Self::InvalidOwner => formatter.write_str("durable owned group owner is invalid"),
            Self::InvalidFrontier => formatter.write_str("durable owned group frontier is invalid"),
            Self::InvalidHash => formatter.write_str("durable owned group hash is invalid"),
            Self::InvalidOutcome => formatter.write_str("durable owned group prepared outcome is invalid"),
            Self::NonCanonical => formatter.write_str("durable owned group encoding is not canonical"),
            Self::RecoveryPackTooLarge => formatter.write_str("durable owned group recovery pack exceeds its member bound"),
            Self::EventTooLarge => formatter.write_str("durable owned group decision exceeds its event bound"),
            Self::Codec(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for DurableOwnedGroupDecisionError {}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, crate::os_dsl::DslArtifact)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(id = "store.unbound-one-item-outcome")]
struct DurableUnboundOneItemOutcomeV1 {
    schema: String,
    recovery_schema: String,
    operation: u64,
    base_generation: u64,
    base_revision: [u8; 32],
    base_applied_edit_count: u64,
    next_sequence_number: i32,
    next_clock_canonical_json: Vec<u8>,
    actor: String,
    edit_without_group_canonical_json: Vec<u8>,
    post_snapshot_pack: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, crate::os_dsl::DslArtifact)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(id = "store.bound-one-item-outcome")]
struct DurableBoundOneItemOutcomeV1 {
    schema: String,
    recovery_schema: String,
    operation: u64,
    base_generation: u64,
    base_revision: [u8; 32],
    base_applied_edit_count: u64,
    next_sequence_number: i32,
    next_clock_canonical_json: Vec<u8>,
    actor: String,
    group_id: String,
    edit_canonical_json: Vec<u8>,
    post_snapshot_pack: Vec<u8>,
    edit_digest: [u8; 32],
    post_generation: u64,
    post_revision: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct DurableStorePreparedOutcomeV1 {
    recovery_schema: String,
    pack: Vec<u8>,
    sha256: String,
}

pub(super) struct DurableStoreVerifiedOutcomeV1<P, Mutation> {
    pub(super) authority: Arc<ArtifactStoreOneItemLiveAuthority>,
    pub(super) edit: Box<Edit<Mutation>>,
    pub(super) post_snapshot: Arc<P>,
}

struct DurableStorePreparedOwnerV1<P, Mutation> {
    prepared: ArtifactStoreOneItemPrepared<P, Mutation>,
    unbound: DurableStorePreparedOutcomeV1,
}

pub(super) struct DurableStoreBoundOutcomeV1<P, Mutation> {
    prepared: ArtifactStoreOneItemPrepared<P, Mutation>,
    unbound_sha256: String,
    recovery_schema: String,
    recovery_pack: Vec<u8>,
    recovery_pack_sha256: String,
    expected_generation: u64,
    expected_revision: [u8; 32],
    post_generation: u64,
    post_revision: [u8; 32],
}

pub(super) struct DurableOwnedThreeStorePreparedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
    parent: DurableStorePreparedOwnerV1<ParentP, ParentMutation>,
    drawing: DurableStorePreparedOwnerV1<DrawingP, DrawingMutation>,
    value: DurableStorePreparedOwnerV1<ValueP, ValueMutation>,
}

pub(super) struct DurableOwnedThreeStoreBoundV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
    pub(super) decision: DurableOwnedThreeMemberDecisionV1,
    pub(super) parent: DurableStoreBoundOutcomeV1<ParentP, ParentMutation>,
    pub(super) drawing: DurableStoreBoundOutcomeV1<DrawingP, DrawingMutation>,
    pub(super) value: DurableStoreBoundOutcomeV1<ValueP, ValueMutation>,
}

pub(super) enum DurableOwnedThreeStoreRecoveryV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
    Apply(DurableOwnedThreeStoreBoundV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>),
    AlreadyApplied,
}

/// 🧾️ Trusted durable-boundary witness returned only after the decision transaction is synced.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DurableOwnedGroupJournalReceiptV1 {
    pub anchor_sha256: String,
    pub decision_sha256: String,
    pub transaction_id: u64,
    pub segment_index: u64,
}

/// 🧭️ One retained journal turn. `Absent` is the only result that restores abort authority after
/// a commit attempt; an error remains an uncertain external state and must be resolved by retry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DurableOwnedGroupJournalAdvanceV1 {
    Pending,
    Absent,
    Committed(DurableOwnedGroupJournalReceiptV1),
}

/// 🧯️ Retained, cancellable owner for one durable decision attempt and its recovery resolution.
pub trait DurableOwnedGroupJournalCommitV1: Send {
    fn advance(&mut self, grant: super::ArtifactStoreOneItemGrant) -> Result<DurableOwnedGroupJournalAdvanceV1, String>;
    fn cancel(&mut self);
    fn begin_close(&mut self);
    fn close_step(&mut self, grant: super::ArtifactStoreOneItemGrant) -> Result<super::SnapshotRetirementStep, String>;
    fn terminal_is_empty(&self) -> bool;
}

/// 🗄️ Kernel-owned journal port implemented by a storage owner with an exclusive writer permit.
/// Construction is an infallible, non-I/O owner transfer; every fallible or externally visible
/// journal action begins in the returned retained operation's first `advance` turn.
pub trait DurableOwnedGroupJournalSinkV1: Send {
    fn begin_commit(&mut self, decision_pack: Vec<u8>, decision_sha256: String) -> Box<dyn DurableOwnedGroupJournalCommitV1>;
}

pub(super) struct ArtifactStoreDurableGroupRootV1<P> {
    pub(super) visibility: Arc<crate::os_vcs::ArtifactGroupVisibility>,
    pub(super) current: Option<Arc<P>>,
    pub(super) generation: u64,
    pub(super) content_revision: [u8; 32],
    pub(super) applied_edit_ids: Vec<String>,
    pub(super) redo_edit_ids: Vec<String>,
    pub(super) last_projection_cause: Option<super::ArtifactProjectionCause>,
    edit_sequence: i32,
    clock: HybridLogicalTimestamp,
    pub(super) local_actor_id: Option<String>,
    revision_accumulator: Option<CursorRevisionAccumulator>,
    tail_undo_cache: Option<(String, Arc<P>)>,
    authority: Option<Arc<ArtifactStoreOneItemLiveAuthority>>,
    displaced_reservation: Option<super::ArtifactStoreDisplacedOwnerReservation>,
    pub(super) adopted: bool,
}

fn retained_string_stack(source: &[String], appended: Option<&str>) -> Result<Vec<String>, DurableOwnedGroupDecisionError> {
    let mut values = Vec::new();
    values.try_reserve_exact(crate::os_vcs::ARTIFACT_HISTORY_LEDGER_CAPACITY).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
    values.extend(source.iter().cloned());
    if let Some(value) = appended {
        values.push(value.into());
    }
    Ok(values)
}

fn retained_revision_stack(source: &[super::CursorRevisionRecord]) -> Result<Vec<super::CursorRevisionRecord>, DurableOwnedGroupDecisionError> {
    let mut values = Vec::new();
    values.try_reserve_exact(crate::os_vcs::ARTIFACT_HISTORY_LEDGER_CAPACITY).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
    values.extend(source.iter().cloned());
    Ok(values)
}

fn stage_store_member<P, Mutation>(
    store: &mut ArtifactStore<P, Mutation>,
    outcome: DurableStoreBoundOutcomeV1<P, Mutation>,
    visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>,
) -> Result<(), (DurableOwnedGroupDecisionError, DurableStoreBoundOutcomeV1<P, Mutation>)>
where
    P: ArtifactPack + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    Mutation: StoreMutation<P> + Clone + ValueToValue + ValueFromValue + Send + 'static,
{
    let reject = |reason, outcome| Err((reason, outcome));
    if store.durable_group_root.is_some()
        || !visibility.pending()
        || store.backbone.is_some()
        || store.snapshot_retirement_factory.is_none()
        || store.mutation_retirement_factory.is_none()
        || store.envelope.vcs.edits.group_visibility().is_some()
        || store.envelope.cursor.as_ref().is_none_or(|cursor| cursor.group_visibility().is_some())
    {
        return reject(DurableOwnedGroupDecisionError::InvalidFrontier, outcome);
    }
    if outcome.prepared.seal.authority.validate_prepared(&outcome.prepared).is_err()
        || store.generation != outcome.expected_generation
        || store.content_revision != outcome.expected_revision
        || outcome.expected_generation.checked_add(1) != Some(outcome.post_generation)
        || outcome.prepared.seal.authority.generation.0 != outcome.expected_generation
        || outcome.prepared.seal.authority.base_revision != outcome.expected_revision
        || outcome.prepared.seal.authority.base_applied_edit_count != store.applied_edit_ids.len()
        || outcome.prepared.edit.id != outcome.prepared.applied_edit_id
        || outcome.prepared.edit.id != outcome.prepared.tail_edit_id
        || outcome.prepared.edit.sequence_number != outcome.prepared.seal.authority.next_sequence_number
        || outcome.prepared.next_clock != outcome.prepared.seal.authority.next_clock
        || store.applied_edit_ids.len() >= crate::os_vcs::ARTIFACT_HISTORY_LEDGER_CAPACITY
    {
        return reject(DurableOwnedGroupDecisionError::InvalidOutcome, outcome);
    }
    let group_id = outcome.prepared.seal.authority.group_id.as_deref().unwrap_or_default();
    if !valid_hash(group_id) || outcome.prepared.edit.mutation_meta.first().and_then(|meta| meta.group_id.as_deref()) != Some(group_id) {
        return reject(DurableOwnedGroupDecisionError::InvalidHash, outcome);
    }
    let applied_edit_ids = match retained_string_stack(&store.applied_edit_ids, Some(&outcome.prepared.edit.id)) {
        Ok(values) => values,
        Err(error) => return reject(error, outcome),
    };
    let redo_edit_ids = match retained_string_stack(&[], None) {
        Ok(values) => values,
        Err(error) => return reject(error, outcome),
    };
    let cursor_owners = super::ArtifactCursorOwners {
        applied_edit_ids: match retained_string_stack(&store.applied_edit_ids, Some(&outcome.prepared.edit.id)) {
            Ok(values) => values,
            Err(error) => return reject(error, outcome),
        },
        redo_edit_ids: match retained_string_stack(&[], None) {
            Ok(values) => values,
            Err(error) => return reject(error, outcome),
        },
        checkpoint_id: (&*store.current_checkpoint_id).clone(),
    };
    let mut revision_accumulator = super::CursorRevisionAccumulator {
        identity_digest: store.revision_accumulator.identity_digest,
        applied: match retained_revision_stack(&store.revision_accumulator.applied) {
            Ok(values) => values,
            Err(error) => return reject(error, outcome),
        },
        redo: match retained_revision_stack(&[]) {
            Ok(values) => values,
            Err(error) => return reject(error, outcome),
        },
    };
    let previous = revision_accumulator.applied.last().map_or(revision_accumulator.identity_digest, |record| record.prefix_digest);
    revision_accumulator.applied.push(super::CursorRevisionRecord {
        id_digest: super::CursorRevisionAccumulator::hash_record(b"edit-id", &[outcome.prepared.edit.id.as_bytes()]),
        edit_digest: outcome.prepared.edit_digest,
        prefix_digest: super::CursorRevisionAccumulator::hash_record(b"applied", &[&previous, &outcome.prepared.edit_digest]),
    });
    if revision_accumulator.revision(store.current_checkpoint_id.as_deref()) != outcome.post_revision {
        return reject(DurableOwnedGroupDecisionError::InvalidFrontier, outcome);
    }
    let displaced_reservation = match store.displaced_retirements.reserve_owner_slots(12) {
        Ok(reservation) => reservation,
        Err(_) => return reject(DurableOwnedGroupDecisionError::InvalidFrontier, outcome),
    };
    let history_reservation = match store.envelope.vcs.edits.reserve_group_one(visibility) {
        Ok(reservation) => reservation,
        Err(()) => {
            store.displaced_retirements.release_owner_slots(displaced_reservation).expect("unconsumed durable group retirement reservation remains exact");
            return reject(DurableOwnedGroupDecisionError::InvalidFrontier, outcome);
        }
    };
    store.envelope.cursor.as_mut().expect("durable group stage preflight retained its cursor").stage_group_owned(cursor_owners, visibility).expect("durable group cursor stage remains exact after exclusive preflight");
    let post_revision = outcome.post_revision;
    let DurableStoreBoundOutcomeV1 { prepared, .. } = outcome;
    let super::ArtifactStoreOneItemPrepared { edit, post_snapshot, next_clock, edit_digest: _, local_actor, applied_edit_id: _, tail_edit_id, seal } = prepared;
    store.envelope.vcs.edits.stage_group_reserved(history_reservation, *edit, visibility).unwrap_or_else(|_| panic!("durable group history stage remains exact after its exclusive reservation"));
    *store.durable_group_root = Some(ArtifactStoreDurableGroupRootV1 {
        visibility: Arc::clone(visibility),
        current: Some(post_snapshot),
        generation: store.generation + 1,
        content_revision: post_revision,
        applied_edit_ids,
        redo_edit_ids,
        last_projection_cause: Some(super::ArtifactProjectionCause::Apply),
        edit_sequence: seal.authority.next_sequence_number,
        clock: next_clock,
        local_actor_id: local_actor,
        revision_accumulator: Some(revision_accumulator),
        tail_undo_cache: Some((tail_edit_id, Arc::clone(&store.current))),
        authority: Some(seal.authority),
        displaced_reservation: Some(displaced_reservation),
        adopted: false,
    });
    Ok(())
}

fn retain_displaced_owner(store: &mut super::ArtifactStoreDisplacedRetirements, reservation: &mut super::ArtifactStoreDisplacedOwnerReservation, owner: Box<dyn super::ErasedSnapshotRetirement>) {
    if let Err(owner) = store.push_owner_reserved(reservation, owner) {
        store.push_reserved(owner);
    }
}

fn abort_staged_store_member<P, Mutation>(store: &mut ArtifactStore<P, Mutation>, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>) -> Result<(), DurableOwnedGroupDecisionError>
where
    P: ArtifactPack + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    Mutation: StoreMutation<P> + Clone + ValueToValue + ValueFromValue + Send + 'static,
{
    let Some(root) = store.durable_group_root.as_ref() else { return Ok(()) };
    if !Arc::ptr_eq(&root.visibility, visibility) || root.adopted || visibility.pending() || visibility.committed() {
        return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
    }
    let mutation_factory = (&*store.mutation_retirement_factory).clone().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    let snapshot_factory = (&*store.snapshot_retirement_factory).clone().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    let edit = store.envelope.vcs.edits.abort_group_one(visibility).map_err(|()| DurableOwnedGroupDecisionError::InvalidFrontier)?.ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    if store.envelope.vcs.edits.abort_group_one(visibility).map_err(|()| DurableOwnedGroupDecisionError::InvalidFrontier)?.is_some() {
        return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
    }
    let cursor = store.envelope.cursor.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?.abort_group_owned(visibility).map_err(|()| DurableOwnedGroupDecisionError::InvalidFrontier)?;
    let mut root = store.durable_group_root.take().expect("validated aborted durable group root remains owned");
    let mut reservation = root.displaced_reservation.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, Box::new(super::ArtifactStoreDecodedEditRetirement::new(edit, mutation_factory)));
    retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, Box::new(super::ArtifactStoreCursorRetirement::new(super::ArtifactCursor::from_owners(cursor))));
    if let Some(current) = root.current.take() {
        retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, snapshot_factory.retire(current));
    }
    if !root.applied_edit_ids.is_empty() || root.applied_edit_ids.capacity() != 0 {
        retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, Box::new(super::ArtifactStoreStringVectorRetirement::new(std::mem::take(&mut root.applied_edit_ids))));
    }
    if !root.redo_edit_ids.is_empty() || root.redo_edit_ids.capacity() != 0 {
        retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, Box::new(super::ArtifactStoreStringVectorRetirement::new(std::mem::take(&mut root.redo_edit_ids))));
    }
    if let Some(revision) = root.revision_accumulator.take() {
        retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, Box::new(super::ArtifactStoreRevisionAccumulatorRetirement::new(revision)));
    }
    if let Some(actor) = root.local_actor_id.take() {
        retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, Box::new(super::ArtifactStoreStringRetirement::new(actor)));
    }
    if let Some((edit_id, snapshot)) = root.tail_undo_cache.take() {
        retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, Box::new(super::ArtifactStoreStringRetirement::new(edit_id)));
        if !Arc::ptr_eq(&snapshot, &store.current) {
            retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, snapshot_factory.retire(snapshot));
        }
    }
    if let Some(authority) = root.authority.take() {
        retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, authority.retire());
    }
    store.displaced_retirements.release_owner_slots(reservation).map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?;
    Ok(())
}

fn adopt_staged_store_member<P, Mutation>(store: &mut ArtifactStore<P, Mutation>, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>) -> Result<(), DurableOwnedGroupDecisionError>
where
    P: ArtifactPack + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    Mutation: StoreMutation<P> + Clone + ValueToValue + ValueFromValue + Send + 'static,
{
    let Some(root) = store.durable_group_root.as_ref() else { return Err(DurableOwnedGroupDecisionError::InvalidFrontier) };
    if !Arc::ptr_eq(&root.visibility, visibility) || root.adopted || !visibility.committed() {
        return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
    }
    let snapshot_factory = (&*store.snapshot_retirement_factory).clone().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    store.envelope.vcs.edits.adopt_group(visibility).map_err(|()| DurableOwnedGroupDecisionError::InvalidFrontier)?;
    let displaced_cursor = store.envelope.cursor.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?.adopt_group_owned(visibility).map_err(|()| DurableOwnedGroupDecisionError::InvalidFrontier)?;
    let mut root = store.durable_group_root.take().expect("validated committed durable group root remains owned");
    let mut reservation = root.displaced_reservation.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, Box::new(super::ArtifactStoreCursorRetirement::new(super::ArtifactCursor::from_owners(displaced_cursor))));
    let previous_applied = std::mem::replace(&mut *store.applied_edit_ids, std::mem::take(&mut root.applied_edit_ids));
    if !previous_applied.is_empty() || previous_applied.capacity() != 0 {
        retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, Box::new(super::ArtifactStoreStringVectorRetirement::new(previous_applied)));
    }
    let previous_redo = std::mem::replace(&mut *store.redo_edit_ids, std::mem::take(&mut root.redo_edit_ids));
    if !previous_redo.is_empty() || previous_redo.capacity() != 0 {
        retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, Box::new(super::ArtifactStoreStringVectorRetirement::new(previous_redo)));
    }
    let previous_revision = std::mem::replace(&mut *store.revision_accumulator, root.revision_accumulator.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?);
    if !previous_revision.applied.is_empty() || previous_revision.applied.capacity() != 0 || !previous_revision.redo.is_empty() || previous_revision.redo.capacity() != 0 {
        retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, Box::new(super::ArtifactStoreRevisionAccumulatorRetirement::new(previous_revision)));
    }
    let previous_current = std::mem::replace(&mut *store.current, root.current.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?);
    retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, snapshot_factory.retire(previous_current));
    if let Some(previous_actor) = std::mem::replace(&mut *store.local_actor_id, root.local_actor_id.take()) {
        retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, Box::new(super::ArtifactStoreStringRetirement::new(previous_actor)));
    }
    if let Some((edit_id, snapshot)) = store.tail_undo_cache.take() {
        retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, Box::new(super::ArtifactStoreStringRetirement::new(edit_id)));
        if !Arc::ptr_eq(&snapshot, &store.current) {
            retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, snapshot_factory.retire(snapshot));
        }
    }
    *store.tail_undo_cache = root.tail_undo_cache.take();
    let previous_report = std::mem::take(&mut *store.pending_report);
    if previous_report.edit_ids.as_ref().is_some_and(|ids| !ids.is_empty() || ids.capacity() != 0) || !previous_report.messages.is_empty() || previous_report.messages.capacity() != 0 || previous_report.worst.is_some() {
        retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, Box::new(super::ArtifactStorePendingReportRetirement::new(previous_report)));
    }
    if let Some(authority) = root.authority.take() {
        retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, authority.retire());
    }
    store.generation = root.generation;
    store.content_revision = root.content_revision;
    store.edit_sequence = root.edit_sequence;
    store.clock = root.clock;
    store.last_projection_cause = root.last_projection_cause;
    store.displaced_retirements.release_owner_slots(reservation).map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?;
    root.adopted = true;
    *store.durable_group_root = Some(root);
    Ok(())
}

fn validate_adopted_store_member<P, Mutation>(store: &ArtifactStore<P, Mutation>, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>) -> Result<(), DurableOwnedGroupDecisionError>
where
    P: Clone + ValueToValue + ValueFromValue,
    Mutation: StoreMutation<P> + Clone + ValueToValue + ValueFromValue,
{
    let root = store.durable_group_root.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?;
    if !root.adopted
        || !Arc::ptr_eq(&root.visibility, visibility)
        || root.current.is_some()
        || root.revision_accumulator.is_some()
        || root.tail_undo_cache.is_some()
        || root.authority.is_some()
        || root.displaced_reservation.is_some()
        || !root.applied_edit_ids.is_empty()
        || !root.redo_edit_ids.is_empty()
    {
        return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
    }
    Ok(())
}

fn clear_adopted_store_member<P, Mutation>(store: &mut ArtifactStore<P, Mutation>, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>) -> Result<(), DurableOwnedGroupDecisionError>
where
    P: Clone + ValueToValue + ValueFromValue,
    Mutation: StoreMutation<P> + Clone + ValueToValue + ValueFromValue,
{
    validate_adopted_store_member(store, visibility)?;
    drop(store.durable_group_root.take());
    Ok(())
}

fn retire_unstaged_store_member<P, Mutation>(store: &mut ArtifactStore<P, Mutation>, outcome: DurableStoreBoundOutcomeV1<P, Mutation>) -> Result<(), DurableOwnedGroupDecisionError>
where
    P: ArtifactPack + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    Mutation: StoreMutation<P> + Clone + ValueToValue + ValueFromValue + Send + 'static,
{
    let mutation_factory = (&*store.mutation_retirement_factory).clone().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    let snapshot_factory = (&*store.snapshot_retirement_factory).clone().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    let mut reservation = store.displaced_retirements.reserve_owner_slots(6).map_err(|_| DurableOwnedGroupDecisionError::InvalidFrontier)?;
    let DurableStoreBoundOutcomeV1 { prepared, .. } = outcome;
    let super::ArtifactStoreOneItemPrepared { edit, post_snapshot, next_clock: _, edit_digest: _, local_actor, applied_edit_id, tail_edit_id, seal } = prepared;
    retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, Box::new(super::ArtifactStoreDecodedEditRetirement::new(*edit, mutation_factory)));
    retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, snapshot_factory.retire(post_snapshot));
    if let Some(actor) = local_actor {
        retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, Box::new(super::ArtifactStoreStringRetirement::new(actor)));
    }
    retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, Box::new(super::ArtifactStoreStringRetirement::new(applied_edit_id)));
    retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, Box::new(super::ArtifactStoreStringRetirement::new(tail_edit_id)));
    retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, seal.authority.retire());
    store.displaced_retirements.release_owner_slots(reservation).map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)
}

/// 🚦️ Observable retained commit phase; no phase after `Journal` can regain abort authority.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DurableOwnedThreeStoreCommitPhaseV1 {
    StagingParent,
    StagingDrawing,
    StagingValue,
    StartingJournal,
    Journal,
    PublishingParentLease,
    PublishingDrawingLease,
    PublishingValueLease,
    AdoptingParent,
    AdoptingDrawing,
    AdoptingValue,
    ClearingParent,
    ClearingDrawing,
    ClearingValue,
    AwaitingAck,
    AbortingValue,
    AbortingDrawing,
    AbortingParent,
    ClosingJournal,
    Complete,
}

/// 📡️ One bounded coordinator result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DurableOwnedThreeStoreCommitAdvanceV1 {
    Progress(DurableOwnedThreeStoreCommitPhaseV1),
    Blocked,
    AwaitingAck(DurableOwnedGroupJournalReceiptV1),
    Complete,
}

/// 📸️ One cross-store snapshot projection selected by one captured shared decision.
pub struct DurableOwnedThreeStoreSnapshotV1<ParentP, DrawingP, ValueP> {
    pub parent: Arc<ParentP>,
    pub drawing: Arc<DrawingP>,
    pub value: Arc<ValueP>,
    pub generations: [u64; 3],
    pub revisions: [[u8; 32]; 3],
}

fn captured_store_snapshot<P, Mutation>(store: &ArtifactStore<P, Mutation>, visibility: Option<&Arc<crate::os_vcs::ArtifactGroupVisibility>>, committed: bool) -> Result<(Arc<P>, u64, [u8; 32]), DurableOwnedGroupDecisionError>
where
    P: Clone + ValueToValue + ValueFromValue,
    Mutation: StoreMutation<P> + Clone + ValueToValue + ValueFromValue,
{
    match store.durable_group_root.as_ref() {
        Some(root) => {
            let expected = visibility.ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?;
            if !Arc::ptr_eq(&root.visibility, expected) {
                return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
            }
            if committed {
                if root.adopted {
                    validate_adopted_store_member(store, expected)?;
                } else {
                    return Ok((Arc::clone(root.current.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?), root.generation, root.content_revision));
                }
            }
        }
        None if committed && visibility.is_some() => return Err(DurableOwnedGroupDecisionError::InvalidFrontier),
        None => {}
    }
    Ok((Arc::clone(&*store.current), store.generation, store.content_revision))
}

pub(super) fn capture_store_owned_three_snapshot<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>(
    parent_store: &ArtifactStore<ParentP, ParentMutation>,
    drawing_store: &ArtifactStore<DrawingP, DrawingMutation>,
    value_store: &ArtifactStore<ValueP, ValueMutation>,
) -> Result<DurableOwnedThreeStoreSnapshotV1<ParentP, DrawingP, ValueP>, DurableOwnedGroupDecisionError>
where
    ParentP: Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue,
    DrawingP: Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue,
    ValueP: Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue,
{
    let roots = [parent_store.durable_group_root.as_ref().map(|root| &root.visibility), drawing_store.durable_group_root.as_ref().map(|root| &root.visibility), value_store.durable_group_root.as_ref().map(|root| &root.visibility)];
    let visibility = roots.iter().flatten().next().copied();
    if visibility.is_some_and(|visibility| roots.iter().flatten().any(|candidate| !Arc::ptr_eq(visibility, candidate))) {
        return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
    }
    let committed = visibility.is_some_and(|visibility| visibility.capture().committed_for(visibility) == Ok(true));
    if committed && roots.iter().any(Option::is_none) {
        return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
    }
    let parent = captured_store_snapshot(parent_store, visibility, committed)?;
    let drawing = captured_store_snapshot(drawing_store, visibility, committed)?;
    let value = captured_store_snapshot(value_store, visibility, committed)?;
    Ok(DurableOwnedThreeStoreSnapshotV1 { parent: parent.0, drawing: drawing.0, value: value.0, generations: [parent.1, drawing.1, value.1], revisions: [parent.2, drawing.2, value.2] })
}

/// 🧷️ Store-owned fixed-three coordinator retaining all candidates through journal resolution.
pub(super) struct DurableOwnedThreeStoreCommitV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
    parent: Option<DurableStoreBoundOutcomeV1<ParentP, ParentMutation>>,
    drawing: Option<DurableStoreBoundOutcomeV1<DrawingP, DrawingMutation>>,
    value: Option<DurableStoreBoundOutcomeV1<ValueP, ValueMutation>>,
    decision: Option<DurableOwnedThreeMemberDecisionV1>,
    decision_pack: Option<Vec<u8>>,
    decision_sha256: String,
    visibility_owner: Option<crate::os_vcs::ArtifactGroupVisibilityOwner>,
    visibility: Option<Arc<crate::os_vcs::ArtifactGroupVisibility>>,
    journal: Option<Box<dyn DurableOwnedGroupJournalCommitV1>>,
    receipt: Option<DurableOwnedGroupJournalReceiptV1>,
    phase: DurableOwnedThreeStoreCommitPhaseV1,
    cancel_requested: bool,
    cancel_forwarded: bool,
    journal_close_started: bool,
}

/// 🗺️ Retained Map publication owner. The three Stores, coordinator, and exclusive journal sink
/// cross the request boundary together and can leave it only through one terminal handoff.
pub struct DurableOwnedMapCommitOperationV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue,
    DrawingP: Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue,
    ValueP: Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue,
{
    parent: std::mem::ManuallyDrop<Option<ArtifactStore<ParentP, ParentMutation>>>,
    drawing: std::mem::ManuallyDrop<Option<ArtifactStore<DrawingP, DrawingMutation>>>,
    value: std::mem::ManuallyDrop<Option<ArtifactStore<ValueP, ValueMutation>>>,
    coordinator: std::mem::ManuallyDrop<Option<DurableOwnedThreeStoreCommitV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>>>,
    sink: std::mem::ManuallyDrop<Option<Box<dyn DurableOwnedGroupJournalSinkV1>>>,
}

/// 🧳 Exact live owners returned after one retained Map publication reaches terminal emptiness.
pub struct DurableOwnedMapCommitOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue,
    DrawingP: Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue,
    ValueP: Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue,
{
    pub parent: ArtifactStore<ParentP, ParentMutation>,
    pub drawing: ArtifactStore<DrawingP, DrawingMutation>,
    pub value: ArtifactStore<ValueP, ValueMutation>,
    pub sink: Box<dyn DurableOwnedGroupJournalSinkV1>,
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedThreeStoreBoundV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
    pub(super) fn begin_retained_commit(self) -> Result<DurableOwnedThreeStoreCommitV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>, DurableOwnedGroupDecisionError> {
        self.decision.validate()?;
        let decision_pack = self.decision.encode_pack_with(&PackEncodeOptions::default()).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        if decision_pack.len() > DURABLE_OWNED_GROUP_EVENT_MAX_BYTES {
            return Err(DurableOwnedGroupDecisionError::EventTooLarge);
        }
        let decision_sha256 = self.decision.decision_sha256.clone();
        let visibility_owner = crate::os_vcs::ArtifactGroupVisibilityOwner::new();
        let visibility = visibility_owner.view();
        Ok(DurableOwnedThreeStoreCommitV1 {
            parent: Some(self.parent),
            drawing: Some(self.drawing),
            value: Some(self.value),
            decision: Some(self.decision),
            decision_pack: Some(decision_pack),
            decision_sha256,
            visibility_owner: Some(visibility_owner),
            visibility: Some(visibility),
            journal: None,
            receipt: None,
            phase: DurableOwnedThreeStoreCommitPhaseV1::StagingParent,
            cancel_requested: false,
            cancel_forwarded: false,
            journal_close_started: false,
        })
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedThreeStoreCommitV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
    DrawingP: ArtifactPack + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
    ValueP: ArtifactPack + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
{
    pub(super) fn phase(&self) -> DurableOwnedThreeStoreCommitPhaseV1 {
        self.phase
    }

    pub(super) fn cancel(&mut self) -> bool {
        if matches!(
            self.phase,
            DurableOwnedThreeStoreCommitPhaseV1::PublishingParentLease
                | DurableOwnedThreeStoreCommitPhaseV1::PublishingDrawingLease
                | DurableOwnedThreeStoreCommitPhaseV1::PublishingValueLease
                | DurableOwnedThreeStoreCommitPhaseV1::AdoptingParent
                | DurableOwnedThreeStoreCommitPhaseV1::AdoptingDrawing
                | DurableOwnedThreeStoreCommitPhaseV1::AdoptingValue
                | DurableOwnedThreeStoreCommitPhaseV1::ClearingParent
                | DurableOwnedThreeStoreCommitPhaseV1::ClearingDrawing
                | DurableOwnedThreeStoreCommitPhaseV1::ClearingValue
                | DurableOwnedThreeStoreCommitPhaseV1::AwaitingAck
                | DurableOwnedThreeStoreCommitPhaseV1::ClosingJournal
                | DurableOwnedThreeStoreCommitPhaseV1::Complete
        ) {
            return false;
        }
        self.cancel_requested = true;
        true
    }

    fn begin_abort(&mut self) -> Result<(), DurableOwnedGroupDecisionError> {
        if !self.visibility_owner.as_mut().is_some_and(crate::os_vcs::ArtifactGroupVisibilityOwner::abort) {
            return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
        }
        self.phase = DurableOwnedThreeStoreCommitPhaseV1::AbortingValue;
        Ok(())
    }

    fn visibility(&self) -> Result<Arc<crate::os_vcs::ArtifactGroupVisibility>, DurableOwnedGroupDecisionError> {
        self.visibility.as_ref().map(Arc::clone).ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)
    }

    fn start_journal_close(&mut self) {
        if self.journal_close_started {
            return;
        }
        if let Some(journal) = self.journal.as_mut() {
            journal.begin_close();
            self.journal_close_started = true;
        }
    }

    pub(super) fn acknowledge(&mut self, receipt: &DurableOwnedGroupJournalReceiptV1) -> bool {
        if self.phase != DurableOwnedThreeStoreCommitPhaseV1::AwaitingAck || self.receipt.as_ref() != Some(receipt) {
            return false;
        }
        self.start_journal_close();
        self.phase = DurableOwnedThreeStoreCommitPhaseV1::ClosingJournal;
        true
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.phase == DurableOwnedThreeStoreCommitPhaseV1::Complete
            && self.parent.is_none()
            && self.drawing.is_none()
            && self.value.is_none()
            && self.decision.is_none()
            && self.decision_pack.is_none()
            && self.decision_sha256.is_empty()
            && self.visibility_owner.is_none()
            && self.visibility.is_none()
            && self.journal.is_none()
            && self.receipt.is_none()
    }

    pub(super) fn mount_map(
        self,
        parent: ArtifactStore<ParentP, ParentMutation>,
        drawing: ArtifactStore<DrawingP, DrawingMutation>,
        value: ArtifactStore<ValueP, ValueMutation>,
        sink: Box<dyn DurableOwnedGroupJournalSinkV1>,
    ) -> DurableOwnedMapCommitOperationV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
        DurableOwnedMapCommitOperationV1 {
            parent: std::mem::ManuallyDrop::new(Some(parent)),
            drawing: std::mem::ManuallyDrop::new(Some(drawing)),
            value: std::mem::ManuallyDrop::new(Some(value)),
            coordinator: std::mem::ManuallyDrop::new(Some(self)),
            sink: std::mem::ManuallyDrop::new(Some(sink)),
        }
    }

    pub(super) fn advance(
        &mut self,
        parent_store: &mut ArtifactStore<ParentP, ParentMutation>,
        drawing_store: &mut ArtifactStore<DrawingP, DrawingMutation>,
        value_store: &mut ArtifactStore<ValueP, ValueMutation>,
        sink: &mut dyn DurableOwnedGroupJournalSinkV1,
        grant: super::ArtifactStoreOneItemGrant,
    ) -> Result<DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedGroupDecisionError> {
        if self.phase == DurableOwnedThreeStoreCommitPhaseV1::Complete {
            return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Complete);
        }
        if !grant.permits_one() {
            return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Blocked);
        }
        if self.cancel_requested
            && matches!(self.phase, DurableOwnedThreeStoreCommitPhaseV1::StagingParent | DurableOwnedThreeStoreCommitPhaseV1::StagingDrawing | DurableOwnedThreeStoreCommitPhaseV1::StagingValue | DurableOwnedThreeStoreCommitPhaseV1::StartingJournal)
        {
            self.begin_abort()?;
            return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Progress(self.phase));
        }
        match self.phase {
            DurableOwnedThreeStoreCommitPhaseV1::StagingParent => {
                let outcome = self.parent.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                match stage_store_member(parent_store, outcome, &self.visibility()?) {
                    Ok(()) => self.phase = DurableOwnedThreeStoreCommitPhaseV1::StagingDrawing,
                    Err((error, outcome)) => {
                        self.parent = Some(outcome);
                        self.begin_abort()?;
                        return Err(error);
                    }
                }
            }
            DurableOwnedThreeStoreCommitPhaseV1::StagingDrawing => {
                let outcome = self.drawing.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                match stage_store_member(drawing_store, outcome, &self.visibility()?) {
                    Ok(()) => self.phase = DurableOwnedThreeStoreCommitPhaseV1::StagingValue,
                    Err((error, outcome)) => {
                        self.drawing = Some(outcome);
                        self.begin_abort()?;
                        return Err(error);
                    }
                }
            }
            DurableOwnedThreeStoreCommitPhaseV1::StagingValue => {
                let outcome = self.value.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                match stage_store_member(value_store, outcome, &self.visibility()?) {
                    Ok(()) => self.phase = DurableOwnedThreeStoreCommitPhaseV1::StartingJournal,
                    Err((error, outcome)) => {
                        self.value = Some(outcome);
                        self.begin_abort()?;
                        return Err(error);
                    }
                }
            }
            DurableOwnedThreeStoreCommitPhaseV1::StartingJournal => {
                let decision_bytes = self.decision_pack.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?.len();
                if grant.maximum_bytes < decision_bytes {
                    return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Blocked);
                }
                let decision_pack = self.decision_pack.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                self.journal = Some(sink.begin_commit(decision_pack, self.decision_sha256.clone()));
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::Journal;
            }
            DurableOwnedThreeStoreCommitPhaseV1::Journal => {
                let journal = self.journal.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                if self.cancel_requested && !self.cancel_forwarded {
                    journal.cancel();
                    self.cancel_forwarded = true;
                }
                match journal.advance(super::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: grant.maximum_bytes }).map_err(DurableOwnedGroupDecisionError::Codec)? {
                    DurableOwnedGroupJournalAdvanceV1::Pending => return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Progress(self.phase)),
                    DurableOwnedGroupJournalAdvanceV1::Absent => {
                        self.begin_abort()?;
                    }
                    DurableOwnedGroupJournalAdvanceV1::Committed(receipt) => {
                        if receipt.decision_sha256 != self.decision_sha256 || self.decision.as_ref().is_none_or(|decision| receipt.anchor_sha256 != decision.anchor_sha256) {
                            return Err(DurableOwnedGroupDecisionError::InvalidHash);
                        }
                        if !self.visibility_owner.as_mut().is_some_and(crate::os_vcs::ArtifactGroupVisibilityOwner::commit) {
                            return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
                        }
                        self.receipt = Some(receipt);
                        self.phase = DurableOwnedThreeStoreCommitPhaseV1::PublishingParentLease;
                    }
                }
            }
            DurableOwnedThreeStoreCommitPhaseV1::PublishingParentLease => {
                let root = parent_store.durable_group_root.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?;
                if !parent_store.snapshot_read_leases.publish_authority(root.generation, root.content_revision) {
                    return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Blocked);
                }
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::PublishingDrawingLease;
            }
            DurableOwnedThreeStoreCommitPhaseV1::PublishingDrawingLease => {
                let root = drawing_store.durable_group_root.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?;
                if !drawing_store.snapshot_read_leases.publish_authority(root.generation, root.content_revision) {
                    return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Blocked);
                }
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::PublishingValueLease;
            }
            DurableOwnedThreeStoreCommitPhaseV1::PublishingValueLease => {
                let root = value_store.durable_group_root.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?;
                if !value_store.snapshot_read_leases.publish_authority(root.generation, root.content_revision) {
                    return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Blocked);
                }
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::AdoptingParent;
            }
            DurableOwnedThreeStoreCommitPhaseV1::AdoptingParent => {
                adopt_staged_store_member(parent_store, &self.visibility()?)?;
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::AdoptingDrawing;
            }
            DurableOwnedThreeStoreCommitPhaseV1::AdoptingDrawing => {
                adopt_staged_store_member(drawing_store, &self.visibility()?)?;
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::AdoptingValue;
            }
            DurableOwnedThreeStoreCommitPhaseV1::AdoptingValue => {
                adopt_staged_store_member(value_store, &self.visibility()?)?;
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::ClearingParent;
            }
            DurableOwnedThreeStoreCommitPhaseV1::ClearingParent => {
                validate_adopted_store_member(parent_store, &self.visibility()?)?;
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::ClearingDrawing;
            }
            DurableOwnedThreeStoreCommitPhaseV1::ClearingDrawing => {
                validate_adopted_store_member(drawing_store, &self.visibility()?)?;
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::ClearingValue;
            }
            DurableOwnedThreeStoreCommitPhaseV1::ClearingValue => {
                let visibility = self.visibility()?;
                validate_adopted_store_member(value_store, &visibility)?;
                clear_adopted_store_member(parent_store, &visibility)?;
                clear_adopted_store_member(drawing_store, &visibility)?;
                clear_adopted_store_member(value_store, &visibility)?;
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::AwaitingAck;
            }
            DurableOwnedThreeStoreCommitPhaseV1::AwaitingAck => {
                return Ok(DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(self.receipt.clone().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?));
            }
            DurableOwnedThreeStoreCommitPhaseV1::AbortingValue => {
                if let Some(outcome) = self.value.take() {
                    retire_unstaged_store_member(value_store, outcome)?;
                } else {
                    abort_staged_store_member(value_store, &self.visibility()?)?;
                }
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::AbortingDrawing;
            }
            DurableOwnedThreeStoreCommitPhaseV1::AbortingDrawing => {
                if let Some(outcome) = self.drawing.take() {
                    retire_unstaged_store_member(drawing_store, outcome)?;
                } else {
                    abort_staged_store_member(drawing_store, &self.visibility()?)?;
                }
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::AbortingParent;
            }
            DurableOwnedThreeStoreCommitPhaseV1::AbortingParent => {
                if let Some(outcome) = self.parent.take() {
                    retire_unstaged_store_member(parent_store, outcome)?;
                } else {
                    abort_staged_store_member(parent_store, &self.visibility()?)?;
                }
                self.start_journal_close();
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::ClosingJournal;
            }
            DurableOwnedThreeStoreCommitPhaseV1::ClosingJournal => {
                if let Some(journal) = self.journal.as_mut() {
                    match journal.close_step(super::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: grant.maximum_bytes }).map_err(DurableOwnedGroupDecisionError::Codec)? {
                        super::SnapshotRetirementStep::Complete => {
                            if !journal.terminal_is_empty() {
                                return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
                            }
                            drop(self.journal.take());
                        }
                        super::SnapshotRetirementStep::Blocked => return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Blocked),
                        super::SnapshotRetirementStep::Pending { .. } => return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Progress(self.phase)),
                    }
                }
                self.decision = None;
                self.decision_pack = None;
                self.decision_sha256.clear();
                self.visibility_owner = None;
                self.visibility = None;
                self.receipt = None;
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::Complete;
            }
            DurableOwnedThreeStoreCommitPhaseV1::Complete => return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Complete),
        }
        Ok(DurableOwnedThreeStoreCommitAdvanceV1::Progress(self.phase))
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> Drop for DurableOwnedThreeStoreCommitV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
    fn drop(&mut self) {
        assert!(
            self.phase == DurableOwnedThreeStoreCommitPhaseV1::Complete
                && self.parent.is_none()
                && self.drawing.is_none()
                && self.value.is_none()
                && self.decision.is_none()
                && self.decision_pack.is_none()
                && self.decision_sha256.is_empty()
                && self.visibility_owner.is_none()
                && self.visibility.is_none()
                && self.journal.is_none()
                && self.receipt.is_none(),
            "durable owned group coordinator reached Drop before exact terminal emptiness"
        );
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedMapCommitOperationV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
    DrawingP: ArtifactPack + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
    ValueP: ArtifactPack + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
{
    pub fn phase(&self) -> Option<DurableOwnedThreeStoreCommitPhaseV1> {
        self.coordinator.as_ref().map(DurableOwnedThreeStoreCommitV1::phase)
    }

    pub fn capture_snapshot(&self) -> Result<DurableOwnedThreeStoreSnapshotV1<ParentP, DrawingP, ValueP>, DurableOwnedGroupDecisionError> {
        capture_store_owned_three_snapshot(
            self.parent.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
            self.drawing.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
            self.value.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
        )
    }

    pub fn cancel(&mut self) -> bool {
        self.coordinator.as_mut().is_some_and(DurableOwnedThreeStoreCommitV1::cancel)
    }

    pub fn acknowledge(&mut self, receipt: &DurableOwnedGroupJournalReceiptV1) -> bool {
        self.coordinator.as_mut().is_some_and(|coordinator| coordinator.acknowledge(receipt))
    }

    pub fn advance(&mut self, grant: super::ArtifactStoreOneItemGrant) -> Result<DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedGroupDecisionError> {
        let coordinator = self.coordinator.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        let parent = self.parent.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        let drawing = self.drawing.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        let value = self.value.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        let sink = self.sink.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        coordinator.advance(parent, drawing, value, sink.as_mut(), grant)
    }

    pub fn take_terminal_owners(&mut self) -> Option<DurableOwnedMapCommitOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>> {
        if !self.coordinator.as_ref().is_some_and(DurableOwnedThreeStoreCommitV1::terminal_is_empty) || self.parent.is_none() || self.drawing.is_none() || self.value.is_none() || self.sink.is_none() {
            return None;
        }
        drop(self.coordinator.take().expect("validated terminal Map coordinator remains owned"));
        Some(DurableOwnedMapCommitOwnersV1 {
            parent: self.parent.take().expect("validated terminal parent Store remains owned"),
            drawing: self.drawing.take().expect("validated terminal drawing Store remains owned"),
            value: self.value.take().expect("validated terminal value Store remains owned"),
            sink: self.sink.take().expect("validated terminal journal sink remains owned"),
        })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.parent.is_none() && self.drawing.is_none() && self.value.is_none() && self.coordinator.is_none() && self.sink.is_none()
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> Drop for DurableOwnedMapCommitOperationV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue,
    DrawingP: Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue,
    ValueP: Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue,
{
    fn drop(&mut self) {
        assert!(self.parent.is_none() && self.drawing.is_none() && self.value.is_none() && self.coordinator.is_none() && self.sink.is_none(), "retained Map commit operation reached Drop before exact terminal owner handoff");
    }
}

impl DurableUnboundOneItemOutcomeV1 {
    fn decode_canonical_pack(bytes: &[u8]) -> Result<Self, DurableOwnedGroupDecisionError> {
        if bytes.len() > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES {
            return Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge);
        }
        let options = PackDecodeOptions {
            verification: PackVerificationLevel::Full,
            preserve_unknown: true,
            limits: crate::os_pack::PackLimits {
                max_file_len: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                max_segment_len: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                max_symbols: 128,
                max_depth: 32,
                max_items: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                max_total_alloc: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
            },
        };
        let (envelope, inner) = crate::os_store::semio_format::unwrap_binary(bytes).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        if envelope.envelope_id() != <Self as ArtifactDsl>::envelope_id() {
            return Err(DurableOwnedGroupDecisionError::InvalidSchema);
        }
        let file = crate::os_io::resolve_ready(crate::os_pack::format::PackFile::open_manifest(inner.as_slice(), &options.limits, options.verification)).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        let manifest = file.manifest().ok_or_else(|| DurableOwnedGroupDecisionError::Codec("unbound outcome manifest is absent".into()))?;
        if manifest.uncompressed_body_len > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64 {
            return Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge);
        }
        if manifest.doc_frame_count != 1 || manifest.chunk_count != 0 || file.chunk_count() != 0 || manifest.field_count != 11 || manifest.schema_hash != crate::os_pack::schema_hash(&Self::__dsl_spec()) {
            return Err(DurableOwnedGroupDecisionError::NonCanonical);
        }
        let outcome = <Self as ArtifactPack>::decode_pack_with(bytes, &options).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        if outcome.schema != DURABLE_OWNED_GROUP_UNBOUND_OUTCOME_SCHEMA_V1
            || !valid_identity(&outcome.recovery_schema)
            || !valid_identity(&outcome.actor)
            || outcome.post_snapshot_pack.is_empty()
            || outcome.post_snapshot_pack.len() > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES
            || outcome.encode_pack() != bytes
        {
            return Err(DurableOwnedGroupDecisionError::NonCanonical);
        }
        Ok(outcome)
    }
}

impl DurableStorePreparedOutcomeV1 {
    fn from_prepared<P, Mutation>(recovery_schema: &str, prepared: &ArtifactStoreOneItemPrepared<P, Mutation>) -> Result<Self, DurableOwnedGroupDecisionError>
    where
        P: ArtifactPack,
        Mutation: ValueToValue + ValueFromValue,
    {
        let authority = &prepared.seal.authority;
        if !valid_identity(recovery_schema) || authority.group_id.is_some() || !valid_edit_identities(prepared.edit.as_ref()) {
            return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
        }
        authority.validate_prepared(prepared).map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?;
        let post_snapshot_pack = prepared.post_snapshot.encode_pack();
        if post_snapshot_pack.len() > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES {
            return Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge);
        }
        let outcome = DurableUnboundOneItemOutcomeV1 {
            schema: DURABLE_OWNED_GROUP_UNBOUND_OUTCOME_SCHEMA_V1.into(),
            recovery_schema: recovery_schema.into(),
            operation: authority.operation.0,
            base_generation: authority.generation.0,
            base_revision: authority.base_revision,
            base_applied_edit_count: authority.base_applied_edit_count.try_into().map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?,
            next_sequence_number: authority.next_sequence_number,
            next_clock_canonical_json: crate::os_pack::json::to_json_string(&authority.next_clock).into_bytes(),
            actor: authority.actor.clone(),
            edit_without_group_canonical_json: crate::os_pack::json::to_json_string(prepared.edit.as_ref()).into_bytes(),
            post_snapshot_pack,
        };
        let pack = outcome.encode_pack_with(&PackEncodeOptions::default()).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        if pack.len() > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES {
            return Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge);
        }
        let owned = Self { recovery_schema: recovery_schema.into(), sha256: semio_framework_hash::sha256_hex(&pack), pack };
        let verified = owned.verify_inverse::<P, Mutation>()?;
        if verified.authority.operation != authority.operation
            || verified.authority.generation != authority.generation
            || verified.authority.base_revision != authority.base_revision
            || verified.authority.base_applied_edit_count != authority.base_applied_edit_count
            || verified.authority.next_sequence_number != authority.next_sequence_number
            || verified.authority.next_clock != authority.next_clock
            || verified.authority.actor != authority.actor
            || ValueToValue::to_value(verified.edit.as_ref()) != ValueToValue::to_value(prepared.edit.as_ref())
            || verified.post_snapshot.encode_pack() != prepared.post_snapshot.encode_pack()
        {
            return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
        }
        Ok(owned)
    }

    pub(super) fn verify_inverse<P, Mutation>(&self) -> Result<DurableStoreVerifiedOutcomeV1<P, Mutation>, DurableOwnedGroupDecisionError>
    where
        P: ArtifactPack,
        Mutation: ValueToValue + ValueFromValue,
    {
        if self.sha256 != semio_framework_hash::sha256_hex(&self.pack) {
            return Err(DurableOwnedGroupDecisionError::InvalidHash);
        }
        let outcome = DurableUnboundOneItemOutcomeV1::decode_canonical_pack(&self.pack)?;
        if outcome.recovery_schema != self.recovery_schema {
            return Err(DurableOwnedGroupDecisionError::InvalidSchema);
        }
        let next_clock: HybridLogicalTimestamp = parse_canonical_json_value(&outcome.next_clock_canonical_json)?;
        let edit: Edit<Mutation> = parse_canonical_json_value(&outcome.edit_without_group_canonical_json)?;
        if !valid_edit_identities(&edit) {
            return Err(DurableOwnedGroupDecisionError::InvalidIdentity);
        }
        let post_options = PackDecodeOptions {
            verification: PackVerificationLevel::Full,
            preserve_unknown: true,
            limits: crate::os_pack::PackLimits {
                max_file_len: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                max_segment_len: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                max_symbols: 128,
                max_depth: 32,
                max_items: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                max_total_alloc: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
            },
        };
        let post_snapshot = P::decode_pack_with(&outcome.post_snapshot_pack, &post_options).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        if post_snapshot.encode_pack() != outcome.post_snapshot_pack {
            return Err(DurableOwnedGroupDecisionError::NonCanonical);
        }
        let base_applied_edit_count = outcome.base_applied_edit_count.try_into().map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?;
        let authority = Arc::new(ArtifactStoreOneItemLiveAuthority {
            operation: semio_framework_job::OperationId(outcome.operation),
            generation: semio_framework_job::Generation(outcome.base_generation),
            base_revision: outcome.base_revision,
            base_applied_edit_count,
            next_sequence_number: outcome.next_sequence_number,
            next_clock,
            actor: outcome.actor,
            group_id: None,
        });
        authority.validate_semantic_edit(&edit).map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?;
        Ok(DurableStoreVerifiedOutcomeV1 { authority, edit: Box::new(edit), post_snapshot: Arc::new(post_snapshot) })
    }
}

impl<P, Mutation> ArtifactStoreOneItemPrepared<P, Mutation>
where
    P: ArtifactPack,
    Mutation: ValueToValue + ValueFromValue,
{
    pub(super) fn durable_unbound_outcome(&self, recovery_schema: &str) -> Result<DurableStorePreparedOutcomeV1, DurableOwnedGroupDecisionError> {
        DurableStorePreparedOutcomeV1::from_prepared(recovery_schema, self)
    }

    fn into_durable_unbound(self, recovery_schema: &str) -> Result<DurableStorePreparedOwnerV1<P, Mutation>, DurableOwnedGroupDecisionError> {
        let unbound = self.durable_unbound_outcome(recovery_schema)?;
        Ok(DurableStorePreparedOwnerV1 { prepared: self, unbound })
    }
}

fn parse_canonical_json_value<T>(bytes: &[u8]) -> Result<T, DurableOwnedGroupDecisionError>
where
    T: ValueToValue + ValueFromValue,
{
    validate_json_budget(bytes)?;
    let json = std::str::from_utf8(bytes).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
    let value = crate::os_pack::json::from_json_str(json).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
    if crate::os_pack::json::to_json_string(&value).as_bytes() != bytes {
        return Err(DurableOwnedGroupDecisionError::NonCanonical);
    }
    Ok(value)
}

fn validate_json_budget(bytes: &[u8]) -> Result<(), DurableOwnedGroupDecisionError> {
    if bytes.len() > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES {
        return Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge);
    }
    let mut depth = 0usize;
    let mut items = usize::from(!bytes.is_empty());
    let mut in_string = false;
    let mut escaped = false;
    for byte in bytes.iter().copied() {
        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            } else if byte < 0x20 {
                return Err(DurableOwnedGroupDecisionError::NonCanonical);
            }
            continue;
        }
        match byte {
            b'"' => in_string = true,
            b'{' | b'[' => {
                depth = depth.checked_add(1).ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                if depth > DURABLE_OWNED_GROUP_JSON_MAX_DEPTH {
                    return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
                }
            }
            b'}' | b']' => {
                depth = depth.checked_sub(1).ok_or(DurableOwnedGroupDecisionError::NonCanonical)?;
            }
            b',' => {
                items = items.checked_add(1).ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                if items > DURABLE_OWNED_GROUP_JSON_MAX_ITEMS {
                    return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
                }
            }
            _ => {}
        }
    }
    if depth != 0 || in_string || escaped {
        return Err(DurableOwnedGroupDecisionError::NonCanonical);
    }
    Ok(())
}

fn valid_edit_identities<Mutation>(edit: &Edit<Mutation>) -> bool {
    valid_identity(&edit.id)
        && edit.actor.as_deref().is_none_or(valid_identity)
        && edit.mutation_meta.iter().all(|meta| {
            meta.mutation_id.as_ref().is_none_or(|id| valid_identity(&id.0))
                && meta.dependencies.iter().all(|id| valid_identity(&id.0))
                && meta.author_id.as_ref().is_none_or(|id| valid_identity(&id.0))
                && meta.semantic_kind.as_ref().is_none_or(|id| valid_identity(&id.0))
                && meta.group_id.as_deref().is_none_or(valid_identity)
        })
}

fn store_post_revision<P, Mutation>(store: &ArtifactStore<P, Mutation>, authority: &ArtifactStoreOneItemLiveAuthority, edit_digest: [u8; 32]) -> Result<[u8; 32], DurableOwnedGroupDecisionError>
where
    P: Clone + ValueToValue + ValueFromValue,
    Mutation: Clone + ValueToValue + ValueFromValue + StoreMutation<P>,
{
    if store.generation != authority.generation.0
        || store.content_revision != authority.base_revision
        || store.applied_edit_ids.len() != authority.base_applied_edit_count
        || store.revision_accumulator.applied.len() != authority.base_applied_edit_count
    {
        return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
    }
    let previous = store.revision_accumulator.applied.last().map_or(store.revision_accumulator.identity_digest, |record| record.prefix_digest);
    let applied = CursorRevisionAccumulator::hash_record(b"applied", &[&previous, &edit_digest]);
    let applied_len = authority.base_applied_edit_count.checked_add(1).ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)? as u64;
    let empty_len = 0_u64;
    Ok(CursorRevisionAccumulator::hash_record(b"cursor", &[&applied, &applied_len.to_be_bytes(), &store.revision_accumulator.identity_digest, &empty_len.to_be_bytes(), store.current_checkpoint_id.as_deref().unwrap_or_default().as_bytes()]))
}

impl<P, Mutation> DurableStorePreparedOwnerV1<P, Mutation>
where
    P: ArtifactPack + Clone + ValueToValue + ValueFromValue,
    Mutation: StoreMutation<P> + ValueToValue + ValueFromValue,
{
    fn bind(self, store: &ArtifactStore<P, Mutation>, group_id: &str) -> Result<DurableStoreBoundOutcomeV1<P, Mutation>, DurableOwnedGroupDecisionError> {
        if !valid_hash(group_id) {
            return Err(DurableOwnedGroupDecisionError::InvalidHash);
        }
        let base_authority = Arc::clone(&self.prepared.seal.authority);
        base_authority.validate_prepared(&self.prepared).map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?;
        if base_authority.group_id.is_some() {
            return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
        }
        let expected_generation = base_authority.generation.0;
        let expected_revision = base_authority.base_revision;
        let post_generation = expected_generation.checked_add(1).ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?;
        let ArtifactStoreOneItemPrepared { mut edit, post_snapshot, local_actor, applied_edit_id, tail_edit_id, .. } = self.prepared;
        if edit.mutation_meta.len() != 1 {
            return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
        }
        let meta = edit.mutation_meta.first_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        if meta.group_id.is_some() {
            return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
        }
        meta.group_id = Some(group_id.into());
        if !valid_edit_identities(edit.as_ref()) {
            return Err(DurableOwnedGroupDecisionError::InvalidIdentity);
        }
        let authority = Arc::new(ArtifactStoreOneItemLiveAuthority {
            operation: base_authority.operation,
            generation: base_authority.generation,
            base_revision: base_authority.base_revision,
            base_applied_edit_count: base_authority.base_applied_edit_count,
            next_sequence_number: base_authority.next_sequence_number,
            next_clock: base_authority.next_clock,
            actor: base_authority.actor.clone(),
            group_id: Some(group_id.into()),
        });
        let edit_digest = authority.prepared_edit_digest(edit.as_ref()).map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?;
        let post_revision = store_post_revision(store, &authority, edit_digest)?;
        let outcome = DurableBoundOneItemOutcomeV1 {
            schema: DURABLE_OWNED_GROUP_BOUND_OUTCOME_SCHEMA_V1.into(),
            recovery_schema: self.unbound.recovery_schema.clone(),
            operation: authority.operation.0,
            base_generation: expected_generation,
            base_revision: expected_revision,
            base_applied_edit_count: authority.base_applied_edit_count.try_into().map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?,
            next_sequence_number: authority.next_sequence_number,
            next_clock_canonical_json: crate::os_pack::json::to_json_string(&authority.next_clock).into_bytes(),
            actor: authority.actor.clone(),
            group_id: group_id.into(),
            edit_canonical_json: crate::os_pack::json::to_json_string(edit.as_ref()).into_bytes(),
            post_snapshot_pack: post_snapshot.encode_pack(),
            edit_digest,
            post_generation,
            post_revision,
        };
        let recovery_pack = outcome.encode_pack_with(&PackEncodeOptions::default()).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        if recovery_pack.len() > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES {
            return Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge);
        }
        let actor = local_actor.ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        if actor != authority.actor || applied_edit_id != edit.id || tail_edit_id != edit.id {
            return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
        }
        let prepared = authority.seal_prepared_owned(edit, post_snapshot, edit_digest, [actor, applied_edit_id, tail_edit_id]);
        let recovery_pack_sha256 = semio_framework_hash::sha256_hex(&recovery_pack);
        Ok(DurableStoreBoundOutcomeV1 { prepared, unbound_sha256: self.unbound.sha256, recovery_schema: self.unbound.recovery_schema, recovery_pack, recovery_pack_sha256, expected_generation, expected_revision, post_generation, post_revision })
    }
}

impl DurableBoundOneItemOutcomeV1 {
    fn decode_canonical_pack(bytes: &[u8]) -> Result<Self, DurableOwnedGroupDecisionError> {
        if bytes.len() > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES {
            return Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge);
        }
        let options = PackDecodeOptions {
            verification: PackVerificationLevel::Full,
            preserve_unknown: true,
            limits: crate::os_pack::PackLimits {
                max_file_len: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                max_segment_len: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                max_symbols: 128,
                max_depth: 32,
                max_items: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                max_total_alloc: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
            },
        };
        let (envelope, inner) = crate::os_store::semio_format::unwrap_binary(bytes).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        if envelope.envelope_id() != <Self as ArtifactDsl>::envelope_id() {
            return Err(DurableOwnedGroupDecisionError::InvalidSchema);
        }
        let file = crate::os_io::resolve_ready(crate::os_pack::format::PackFile::open_manifest(inner.as_slice(), &options.limits, options.verification)).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        let manifest = file.manifest().ok_or_else(|| DurableOwnedGroupDecisionError::Codec("bound outcome manifest is absent".into()))?;
        if manifest.uncompressed_body_len > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64 {
            return Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge);
        }
        if manifest.doc_frame_count != 1 || manifest.chunk_count != 0 || file.chunk_count() != 0 || manifest.field_count != 15 || manifest.schema_hash != crate::os_pack::schema_hash(&Self::__dsl_spec()) {
            return Err(DurableOwnedGroupDecisionError::NonCanonical);
        }
        let outcome = <Self as ArtifactPack>::decode_pack_with(bytes, &options).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        if outcome.schema != DURABLE_OWNED_GROUP_BOUND_OUTCOME_SCHEMA_V1
            || !valid_identity(&outcome.recovery_schema)
            || !valid_identity(&outcome.actor)
            || !valid_hash(&outcome.group_id)
            || outcome.post_snapshot_pack.is_empty()
            || outcome.post_snapshot_pack.len() > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES
            || outcome.encode_pack() != bytes
        {
            return Err(DurableOwnedGroupDecisionError::NonCanonical);
        }
        Ok(outcome)
    }

    fn unbound_pack<Mutation>(&self) -> Result<Vec<u8>, DurableOwnedGroupDecisionError>
    where
        Mutation: ValueToValue + ValueFromValue,
    {
        let mut edit: Edit<Mutation> = parse_canonical_json_value(&self.edit_canonical_json)?;
        if edit.mutation_meta.len() != 1 {
            return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
        }
        let meta = edit.mutation_meta.first_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        if meta.group_id.as_deref() != Some(self.group_id.as_str()) {
            return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
        }
        meta.group_id = None;
        if !valid_edit_identities(&edit) {
            return Err(DurableOwnedGroupDecisionError::InvalidIdentity);
        }
        let outcome = DurableUnboundOneItemOutcomeV1 {
            schema: DURABLE_OWNED_GROUP_UNBOUND_OUTCOME_SCHEMA_V1.into(),
            recovery_schema: self.recovery_schema.clone(),
            operation: self.operation,
            base_generation: self.base_generation,
            base_revision: self.base_revision,
            base_applied_edit_count: self.base_applied_edit_count,
            next_sequence_number: self.next_sequence_number,
            next_clock_canonical_json: self.next_clock_canonical_json.clone(),
            actor: self.actor.clone(),
            edit_without_group_canonical_json: crate::os_pack::json::to_json_string(&edit).into_bytes(),
            post_snapshot_pack: self.post_snapshot_pack.clone(),
        };
        outcome.encode_pack_with(&PackEncodeOptions::default()).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))
    }
}

impl<P, Mutation> DurableStoreBoundOutcomeV1<P, Mutation>
where
    P: ArtifactPack + Clone + ValueToValue + ValueFromValue,
    Mutation: StoreMutation<P> + ValueToValue + ValueFromValue,
{
    fn recover(store: &ArtifactStore<P, Mutation>, member: &DurableOwnedGroupMemberV1, group_id: &str) -> Result<Self, DurableOwnedGroupDecisionError> {
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
                    limits: crate::os_pack::PackLimits {
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
        let authority = Arc::new(ArtifactStoreOneItemLiveAuthority {
            operation: semio_framework_job::OperationId(outcome.operation),
            generation: semio_framework_job::Generation(outcome.base_generation),
            base_revision: outcome.base_revision,
            base_applied_edit_count: outcome.base_applied_edit_count.try_into().map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?,
            next_sequence_number: outcome.next_sequence_number,
            next_clock,
            actor: outcome.actor,
            group_id: Some(group_id.into()),
        });
        authority.validate_semantic_edit(&edit).map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?;
        if store_post_revision(store, &authority, outcome.edit_digest)? != outcome.post_revision || outcome.post_generation != outcome.base_generation.checked_add(1).ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)? {
            return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
        }
        let edit_id = edit.id.clone();
        let actor = authority.actor.clone();
        let prepared = authority.seal_prepared_owned(Box::new(edit), post_snapshot, outcome.edit_digest, [actor, edit_id.clone(), edit_id]);
        Ok(Self {
            prepared,
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

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedThreeStorePreparedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack,
    ParentMutation: ValueToValue + ValueFromValue,
    DrawingP: ArtifactPack,
    DrawingMutation: ValueToValue + ValueFromValue,
    ValueP: ArtifactPack,
    ValueMutation: ValueToValue + ValueFromValue,
{
    pub(super) fn from_store_prepared(
        parent: ArtifactStoreOneItemPrepared<ParentP, ParentMutation>,
        drawing: ArtifactStoreOneItemPrepared<DrawingP, DrawingMutation>,
        value: ArtifactStoreOneItemPrepared<ValueP, ValueMutation>,
    ) -> Result<Self, DurableOwnedGroupDecisionError> {
        Ok(Self { parent: parent.into_durable_unbound(PARENT_RECOVERY_SCHEMA)?, drawing: drawing.into_durable_unbound(DRAWING_RECOVERY_SCHEMA)?, value: value.into_durable_unbound(VALUE_RECOVERY_SCHEMA)? })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum DurableMemberRecoveryFrontier {
    Base,
    Post,
}

fn recovery_frontier<P, Mutation>(store: &ArtifactStore<P, Mutation>, member: &DurableOwnedGroupMemberV1, group_id: &str) -> Result<DurableMemberRecoveryFrontier, DurableOwnedGroupDecisionError>
where
    P: ArtifactPack + Clone + ValueToValue + ValueFromValue,
    Mutation: StoreMutation<P> + ValueToValue + ValueFromValue,
{
    if (store.generation, store.content_revision) == (member.expected_generation, member.expected_revision) {
        return Ok(DurableMemberRecoveryFrontier::Base);
    }
    if (store.generation, store.content_revision) != (member.post_generation, member.post_revision) {
        return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
    }
    let outcome = DurableBoundOneItemOutcomeV1::decode_canonical_pack(&member.recovery_pack)?;
    let edit: Edit<Mutation> = parse_canonical_json_value(&outcome.edit_canonical_json)?;
    let tail = store.applied_edit_ids.last().and_then(|id| store.envelope.vcs.edits.iter().find(|candidate| candidate.id == *id)).ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?;
    let accumulator = store.revision_accumulator.applied.last().ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?;
    if outcome.group_id != group_id
        || !valid_edit_identities(&edit)
        || outcome.unbound_pack::<Mutation>().map(|pack| semio_framework_hash::sha256_hex(&pack))? != member.unbound_outcome_sha256
        || CursorRevisionAccumulator::edit_digest(&edit) != outcome.edit_digest
        || accumulator.edit_digest != outcome.edit_digest
        || crate::os_pack::json::to_json_string(tail).as_bytes() != outcome.edit_canonical_json
        || store.current.encode_pack() != outcome.post_snapshot_pack
    {
        return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
    }
    Ok(DurableMemberRecoveryFrontier::Post)
}

impl DurableOwnedThreeMemberDecisionV1 {
    pub(super) fn recover_store_owned<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>(
        &self,
        parent_store: &ArtifactStore<ParentP, ParentMutation>,
        drawing_store: &ArtifactStore<DrawingP, DrawingMutation>,
        value_store: &ArtifactStore<ValueP, ValueMutation>,
    ) -> Result<DurableOwnedThreeStoreRecoveryV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>, DurableOwnedGroupDecisionError>
    where
        ParentP: ArtifactPack + Clone + ValueToValue + ValueFromValue,
        ParentMutation: StoreMutation<ParentP> + ValueToValue + ValueFromValue,
        DrawingP: ArtifactPack + Clone + ValueToValue + ValueFromValue,
        DrawingMutation: StoreMutation<DrawingP> + ValueToValue + ValueFromValue,
        ValueP: ArtifactPack + Clone + ValueToValue + ValueFromValue,
        ValueMutation: StoreMutation<ValueP> + ValueToValue + ValueFromValue,
    {
        self.validate()?;
        let identities = [store_member_identity(parent_store)?, store_member_identity(drawing_store)?, store_member_identity(value_store)?];
        for ((reference, owner), member) in identities.iter().zip([&self.parent, &self.drawing, &self.value]) {
            if reference != &member.reference || owner != &member.owner {
                return Err(DurableOwnedGroupDecisionError::InvalidOwner);
            }
        }
        let frontiers = [recovery_frontier(parent_store, &self.parent, &self.decision_sha256)?, recovery_frontier(drawing_store, &self.drawing, &self.decision_sha256)?, recovery_frontier(value_store, &self.value, &self.decision_sha256)?];
        if frontiers.iter().all(|frontier| *frontier == DurableMemberRecoveryFrontier::Post) {
            return Ok(DurableOwnedThreeStoreRecoveryV1::AlreadyApplied);
        }
        if !frontiers.iter().all(|frontier| *frontier == DurableMemberRecoveryFrontier::Base) {
            return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
        }
        Ok(DurableOwnedThreeStoreRecoveryV1::Apply(DurableOwnedThreeStoreBoundV1 {
            parent: DurableStoreBoundOutcomeV1::recover(parent_store, &self.parent, &self.decision_sha256)?,
            drawing: DurableStoreBoundOutcomeV1::recover(drawing_store, &self.drawing, &self.decision_sha256)?,
            value: DurableStoreBoundOutcomeV1::recover(value_store, &self.value, &self.decision_sha256)?,
            decision: self.clone(),
        }))
    }
}

fn store_member_identity<P, Mutation>(store: &ArtifactStore<P, Mutation>) -> Result<(crate::os_io::ArtifactRef, Option<OwnerRef>), DurableOwnedGroupDecisionError>
where
    P: Clone + ValueToValue + ValueFromValue,
    Mutation: Clone + ValueToValue + ValueFromValue + StoreMutation<P>,
{
    let envelope = &store.envelope;
    let dialect = envelope.dialect.clone().ok_or(DurableOwnedGroupDecisionError::InvalidIdentity)?;
    Ok((crate::os_io::ArtifactRef { artifact_id: envelope.id.clone(), dialect }, envelope.owner.clone()))
}

fn prepared_member<P, Mutation>(role: &str, reference: crate::os_io::ArtifactRef, owner: Option<OwnerRef>, prepared: &DurableStorePreparedOwnerV1<P, Mutation>) -> Result<DurableOwnedGroupMemberV1, DurableOwnedGroupDecisionError> {
    let authority = &prepared.prepared.seal.authority;
    authority.validate_prepared(&prepared.prepared).map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?;
    if authority.group_id.is_some() {
        return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
    }
    Ok(DurableOwnedGroupMemberV1 {
        role: role.into(),
        reference,
        owner,
        expected_generation: authority.generation.0,
        expected_revision: authority.base_revision,
        recovery_schema: prepared.unbound.recovery_schema.clone(),
        recovery_pack: Vec::new(),
        recovery_pack_sha256: String::new(),
        unbound_outcome_sha256: prepared.unbound.sha256.clone(),
        post_generation: authority.generation.0.checked_add(1).ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?,
        post_revision: [0; 32],
    })
}

fn bound_member<P, Mutation>(role: &str, reference: crate::os_io::ArtifactRef, owner: Option<OwnerRef>, bound: &DurableStoreBoundOutcomeV1<P, Mutation>) -> DurableOwnedGroupMemberV1 {
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

fn prepared_base_matches_store<P, Mutation>(store: &ArtifactStore<P, Mutation>, member: &DurableOwnedGroupMemberV1, prepared: &ArtifactStoreOneItemPrepared<P, Mutation>) -> bool
where
    P: Clone + ValueToValue + ValueFromValue,
    Mutation: Clone + ValueToValue + ValueFromValue + StoreMutation<P>,
{
    store.generation == member.expected_generation && store.content_revision == member.expected_revision && store.applied_edit_ids.len() == prepared.seal.authority.base_applied_edit_count
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedThreeStorePreparedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + ValueToValue + ValueFromValue,
    DrawingP: ArtifactPack + Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + ValueToValue + ValueFromValue,
    ValueP: ArtifactPack + Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + ValueToValue + ValueFromValue,
{
    pub(super) fn bind_store_owned(
        self,
        parent_store: &ArtifactStore<ParentP, ParentMutation>,
        drawing_store: &ArtifactStore<DrawingP, DrawingMutation>,
        value_store: &ArtifactStore<ValueP, ValueMutation>,
    ) -> Result<DurableOwnedThreeStoreBoundV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>, DurableOwnedGroupDecisionError> {
        let (parent_reference, parent_owner) = store_member_identity(parent_store)?;
        let (drawing_reference, drawing_owner) = store_member_identity(drawing_store)?;
        let (value_reference, value_owner) = store_member_identity(value_store)?;
        if parent_reference == drawing_reference || parent_reference == value_reference || drawing_reference == value_reference {
            return Err(DurableOwnedGroupDecisionError::InvalidIdentity);
        }
        let anchor = DurableOwnedGroupAnchorV1 { schema: DURABLE_OWNED_GROUP_ANCHOR_SCHEMA_V1.into(), parent: parent_reference.clone(), shape: DURABLE_OWNED_GROUP_SHAPE_V1.into() };
        let parent = prepared_member(PARENT_ROLE, parent_reference.clone(), parent_owner.clone(), &self.parent)?;
        let drawing = prepared_member(DRAWING_ROLE, drawing_reference.clone(), drawing_owner.clone(), &self.drawing)?;
        let value = prepared_member(VALUE_ROLE, value_reference.clone(), value_owner.clone(), &self.value)?;
        if !member_identity_matches(&parent, PARENT_ROLE, PARENT_RECOVERY_SCHEMA, &parent_reference.to_uri(), &anchor)
            || !member_identity_matches(&drawing, DRAWING_ROLE, DRAWING_RECOVERY_SCHEMA, "gismap-drawing!s.stdio.semio@v1/drawing", &anchor)
            || !member_identity_matches(&value, VALUE_ROLE, VALUE_RECOVERY_SCHEMA, "gismap-value!s.stdio.semio@v1/value", &anchor)
            || !prepared_base_matches_store(parent_store, &parent, &self.parent.prepared)
            || !prepared_base_matches_store(drawing_store, &drawing, &self.drawing.prepared)
            || !prepared_base_matches_store(value_store, &value, &self.value.prepared)
        {
            return Err(DurableOwnedGroupDecisionError::InvalidOwner);
        }
        let anchor_sha256 = semio_framework_hash::sha256_hex(crate::os_pack::json::to_json_string(&anchor).as_bytes());
        let mut decision = DurableOwnedThreeMemberDecisionV1 { schema: DURABLE_OWNED_GROUP_DECISION_SCHEMA_V1.into(), anchor, anchor_sha256, decision_sha256: String::new(), parent, drawing, value };
        decision.decision_sha256 = semio_framework_hash::sha256_hex(decision.canonical_unsigned_json().as_bytes());
        let parent = self.parent.bind(parent_store, &decision.decision_sha256)?;
        let drawing = self.drawing.bind(drawing_store, &decision.decision_sha256)?;
        let value = self.value.bind(value_store, &decision.decision_sha256)?;
        decision.parent = bound_member(PARENT_ROLE, parent_reference, parent_owner, &parent);
        decision.drawing = bound_member(DRAWING_ROLE, drawing_reference, drawing_owner, &drawing);
        decision.value = bound_member(VALUE_ROLE, value_reference, value_owner, &value);
        decision.validate()?;
        Ok(DurableOwnedThreeStoreBoundV1 { decision, parent, drawing, value })
    }
}

impl ArtifactDsl for DurableUnboundOneItemOutcomeV1 {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;

    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }

    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = crate::os_store::semio_format::split_text_preamble(text).map(|(_, body)| body).unwrap_or(text);
        let record = crate::os_dsl::parse(body, &Self::__dsl_spec(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }

    fn print_dsl(&self) -> String {
        let body = crate::os_dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), crate::os_dsl::JoinMode::Document);
        let envelope = crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Dsl, 1).expect("valid unbound outcome envelope");
        crate::os_store::semio_format::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for DurableUnboundOneItemOutcomeV1 {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let inner = pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Pack, 1).map_err(|error| PackError::Schema(error.to_string()))?;
        Ok(crate::os_store::semio_format::wrap_binary(&envelope, &inner))
    }

    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = crate::os_store::semio_format::unwrap_binary(bytes).map_err(|error| PackError::Schema(error.to_string()))?;
        if envelope.envelope_id() != <Self as ArtifactDsl>::envelope_id() {
            return Err(PackError::Schema("unbound outcome pack envelope mismatch".into()));
        }
        let (record, _) = pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(crate::os_store::text_error_to_pack_error)
    }

    fn record_spec() -> Option<crate::os_dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl ArtifactDsl for DurableBoundOneItemOutcomeV1 {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;

    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }

    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = crate::os_store::semio_format::split_text_preamble(text).map(|(_, body)| body).unwrap_or(text);
        let record = crate::os_dsl::parse(body, &Self::__dsl_spec(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }

    fn print_dsl(&self) -> String {
        let body = crate::os_dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), crate::os_dsl::JoinMode::Document);
        let envelope = crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Dsl, 1).expect("valid bound outcome envelope");
        crate::os_store::semio_format::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for DurableBoundOneItemOutcomeV1 {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let inner = pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Pack, 1).map_err(|error| PackError::Schema(error.to_string()))?;
        Ok(crate::os_store::semio_format::wrap_binary(&envelope, &inner))
    }

    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = crate::os_store::semio_format::unwrap_binary(bytes).map_err(|error| PackError::Schema(error.to_string()))?;
        if envelope.envelope_id() != <Self as ArtifactDsl>::envelope_id() {
            return Err(PackError::Schema("bound outcome pack envelope mismatch".into()));
        }
        let (record, _) = pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(crate::os_store::text_error_to_pack_error)
    }

    fn record_spec() -> Option<crate::os_dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DurableOwnedGroupMapHandlesV1 {
    pub(crate) drawing: crate::os_io::ArtifactRef,
    pub(crate) value: crate::os_io::ArtifactRef,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DurableOwnedGroupMapFrontiersV1 {
    pub(crate) parent_generation: u64,
    pub(crate) parent_revision: [u8; 32],
    pub(crate) drawing_generation: u64,
    pub(crate) drawing_revision: [u8; 32],
    pub(crate) value_generation: u64,
    pub(crate) value_revision: [u8; 32],
}

fn valid_hash(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

fn valid_identity(value: &str) -> bool {
    !value.is_empty() && value.len() <= DURABLE_OWNED_GROUP_ID_MAX_BYTES && !value.chars().any(char::is_control)
}

fn valid_reference(reference: &crate::os_io::ArtifactRef) -> bool {
    valid_identity(&reference.artifact_id) && valid_identity(&reference.dialect.artifact_kind) && valid_identity(&reference.dialect.standard) && valid_identity(&reference.dialect.subset)
}

fn unsigned_member(member: &DurableOwnedGroupMemberV1) -> DurableOwnedThreeMemberUnsignedMemberV1 {
    DurableOwnedThreeMemberUnsignedMemberV1 {
        role: member.role.clone(),
        reference: member.reference.clone(),
        owner: member.owner.clone(),
        expected_generation: member.expected_generation,
        expected_revision: member.expected_revision,
        recovery_schema: member.recovery_schema.clone(),
        unbound_outcome_sha256: member.unbound_outcome_sha256.clone(),
    }
}

fn member_identity_matches(member: &DurableOwnedGroupMemberV1, role: &str, recovery_schema: &str, reference_uri: &str, anchor: &DurableOwnedGroupAnchorV1) -> bool {
    if member.role != role || member.recovery_schema != recovery_schema || member.reference.to_uri() != reference_uri || !valid_reference(&member.reference) || !valid_identity(&member.recovery_schema) {
        return false;
    }
    match (role, &member.owner) {
        (PARENT_ROLE, None) => member.reference == anchor.parent,
        (DRAWING_ROLE, Some(owner)) => owner.parent == anchor.parent && owner.slot == DRAWING_ROLE && owner.child_id == "gismap-drawing",
        (VALUE_ROLE, Some(owner)) => owner.parent == anchor.parent && owner.slot == VALUE_ROLE && owner.child_id == "gismap-value",
        _ => false,
    }
}

fn member_matches(member: &DurableOwnedGroupMemberV1, role: &str, recovery_schema: &str, reference_uri: &str, anchor: &DurableOwnedGroupAnchorV1) -> bool {
    member_identity_matches(member, role, recovery_schema, reference_uri, anchor) && member.expected_generation.checked_add(1) == Some(member.post_generation) && member.post_revision != member.expected_revision
}

impl DurableOwnedThreeMemberDecisionV1 {
    #[cfg(test)]
    fn seal_fixture(
        anchor: DurableOwnedGroupAnchorV1,
        mut parent: DurableOwnedGroupMemberV1,
        mut drawing: DurableOwnedGroupMemberV1,
        mut value: DurableOwnedGroupMemberV1,
        outcomes: &DurableStorePreparedOutcomesV1,
    ) -> Result<Self, DurableOwnedGroupDecisionError> {
        if [&parent, &drawing, &value].iter().any(|member| member.recovery_pack.len() > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES) {
            return Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge);
        }
        for (member, outcome) in [(&mut parent, &outcomes.parent), (&mut drawing, &outcomes.drawing), (&mut value, &outcomes.value)] {
            if member.recovery_schema != outcome.recovery_schema || outcome.sha256 != semio_framework_hash::sha256_hex(&outcome.pack) || DurableUnboundOneItemOutcomeV1::decode_canonical_pack(&outcome.pack)?.recovery_schema != member.recovery_schema {
                return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
            }
            member.unbound_outcome_sha256.clone_from(&outcome.sha256);
        }
        let anchor_sha256 = semio_framework_hash::sha256_hex(crate::os_pack::json::to_json_string(&anchor).as_bytes());
        let mut decision = Self { schema: DURABLE_OWNED_GROUP_DECISION_SCHEMA_V1.into(), anchor, anchor_sha256, decision_sha256: String::new(), parent, drawing, value };
        decision.decision_sha256 = semio_framework_hash::sha256_hex(decision.canonical_unsigned_json().as_bytes());
        for (member, prepared) in [(&mut decision.parent, &outcomes.parent), (&mut decision.drawing, &outcomes.drawing), (&mut decision.value, &outcomes.value)] {
            let unbound = DurableUnboundOneItemOutcomeV1::decode_canonical_pack(&prepared.pack)?;
            let mut edit: Edit<String> = parse_canonical_json_value(&unbound.edit_without_group_canonical_json)?;
            if edit.mutation_meta.len() != 1 {
                return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
            }
            let meta = edit.mutation_meta.first_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
            if meta.group_id.replace(decision.decision_sha256.clone()).is_some() {
                return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
            }
            let bound = DurableBoundOneItemOutcomeV1 {
                schema: DURABLE_OWNED_GROUP_BOUND_OUTCOME_SCHEMA_V1.into(),
                recovery_schema: unbound.recovery_schema,
                operation: unbound.operation,
                base_generation: unbound.base_generation,
                base_revision: unbound.base_revision,
                base_applied_edit_count: unbound.base_applied_edit_count,
                next_sequence_number: unbound.next_sequence_number,
                next_clock_canonical_json: unbound.next_clock_canonical_json,
                actor: unbound.actor,
                group_id: decision.decision_sha256.clone(),
                edit_canonical_json: crate::os_pack::json::to_json_string(&edit).into_bytes(),
                post_snapshot_pack: unbound.post_snapshot_pack,
                edit_digest: CursorRevisionAccumulator::edit_digest(&edit),
                post_generation: member.post_generation,
                post_revision: member.post_revision,
            };
            member.recovery_pack = bound.encode_pack();
            member.recovery_pack_sha256 = semio_framework_hash::sha256_hex(&member.recovery_pack);
        }
        decision.validate()?;
        Ok(decision)
    }

    pub(crate) fn canonical_json(&self) -> String {
        crate::os_pack::json::to_json_string(self)
    }

    pub(crate) fn canonical_unsigned_json(&self) -> String {
        crate::os_pack::json::to_json_string(&DurableOwnedThreeMemberUnsignedV1 {
            schema: self.schema.clone(),
            anchor: self.anchor.clone(),
            parent: unsigned_member(&self.parent),
            drawing: unsigned_member(&self.drawing),
            value: unsigned_member(&self.value),
        })
    }

    pub(crate) fn parse_canonical_json(json: &str) -> Result<Self, DurableOwnedGroupDecisionError> {
        if json.len() > DURABLE_OWNED_GROUP_EVENT_MAX_BYTES {
            return Err(DurableOwnedGroupDecisionError::EventTooLarge);
        }
        let decision: Self = crate::os_pack::json::from_json_str(json).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        if decision.canonical_json() != json {
            return Err(DurableOwnedGroupDecisionError::NonCanonical);
        }
        decision.validate()?;
        Ok(decision)
    }

    pub(crate) fn decode_canonical_pack(bytes: &[u8]) -> Result<Self, DurableOwnedGroupDecisionError> {
        if bytes.len() > DURABLE_OWNED_GROUP_EVENT_MAX_BYTES {
            return Err(DurableOwnedGroupDecisionError::EventTooLarge);
        }
        let (envelope, inner) = crate::os_store::semio_format::unwrap_binary(bytes).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        if envelope.envelope_id() != <Self as ArtifactDsl>::envelope_id() {
            return Err(DurableOwnedGroupDecisionError::InvalidSchema);
        }
        let options = PackDecodeOptions {
            verification: PackVerificationLevel::Full,
            preserve_unknown: true,
            limits: crate::os_pack::PackLimits {
                max_file_len: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES as u64,
                max_segment_len: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES as u64,
                max_symbols: 128,
                max_depth: 16,
                max_items: DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64,
                max_total_alloc: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES as u64,
            },
        };
        let file = crate::os_io::resolve_ready(crate::os_pack::format::PackFile::open_manifest(inner.as_slice(), &options.limits, options.verification)).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        let manifest = file.manifest().ok_or_else(|| DurableOwnedGroupDecisionError::Codec("durable group manifest is absent".into()))?;
        if manifest.uncompressed_body_len > DURABLE_OWNED_GROUP_EVENT_MAX_BYTES as u64 {
            return Err(DurableOwnedGroupDecisionError::EventTooLarge);
        }
        if manifest.doc_frame_count != 1 || manifest.chunk_count != 0 || file.chunk_count() != 0 || manifest.field_count != 7 {
            return Err(DurableOwnedGroupDecisionError::NonCanonical);
        }
        if manifest.schema_hash != crate::os_pack::schema_hash(&Self::__dsl_spec()) {
            return Err(DurableOwnedGroupDecisionError::InvalidSchema);
        }
        let decision = <Self as ArtifactPack>::decode_pack_with(bytes, &options).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        decision.validate()?;
        if decision.encode_pack() != bytes {
            return Err(DurableOwnedGroupDecisionError::NonCanonical);
        }
        Ok(decision)
    }

    pub(crate) fn validate(&self) -> Result<(), DurableOwnedGroupDecisionError> {
        if self.schema != DURABLE_OWNED_GROUP_DECISION_SCHEMA_V1 || self.anchor.schema != DURABLE_OWNED_GROUP_ANCHOR_SCHEMA_V1 || self.anchor.shape != DURABLE_OWNED_GROUP_SHAPE_V1 {
            return Err(DurableOwnedGroupDecisionError::InvalidSchema);
        }
        if !valid_reference(&self.anchor.parent) || self.anchor.parent.dialect.to_coordinate() != "s.gis.gismap@1/*" {
            return Err(DurableOwnedGroupDecisionError::InvalidIdentity);
        }
        let parent_uri = self.anchor.parent.to_uri();
        if !member_matches(&self.parent, PARENT_ROLE, PARENT_RECOVERY_SCHEMA, &parent_uri, &self.anchor)
            || !member_matches(&self.drawing, DRAWING_ROLE, DRAWING_RECOVERY_SCHEMA, "gismap-drawing!s.stdio.semio@v1/drawing", &self.anchor)
            || !member_matches(&self.value, VALUE_ROLE, VALUE_RECOVERY_SCHEMA, "gismap-value!s.stdio.semio@v1/value", &self.anchor)
        {
            return Err(DurableOwnedGroupDecisionError::InvalidOwner);
        }
        for member in [&self.parent, &self.drawing, &self.value] {
            if member.recovery_pack.len() > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES {
                return Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge);
            }
            if !valid_hash(&member.recovery_pack_sha256) || !valid_hash(&member.unbound_outcome_sha256) || member.recovery_pack_sha256 != semio_framework_hash::sha256_hex(&member.recovery_pack) {
                return Err(DurableOwnedGroupDecisionError::InvalidHash);
            }
            let outcome = DurableBoundOneItemOutcomeV1::decode_canonical_pack(&member.recovery_pack)?;
            if outcome.recovery_schema != member.recovery_schema
                || outcome.group_id != self.decision_sha256
                || outcome.base_generation != member.expected_generation
                || outcome.base_revision != member.expected_revision
                || outcome.post_generation != member.post_generation
                || outcome.post_revision != member.post_revision
            {
                return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
            }
        }
        let unsigned = self.canonical_unsigned_json();
        if unsigned.len() > DURABLE_OWNED_GROUP_STRUCTURAL_MAX_BYTES {
            return Err(DurableOwnedGroupDecisionError::InvalidIdentity);
        }
        if self.anchor_sha256 != semio_framework_hash::sha256_hex(crate::os_pack::json::to_json_string(&self.anchor).as_bytes()) || self.decision_sha256 != semio_framework_hash::sha256_hex(unsigned.as_bytes()) {
            return Err(DurableOwnedGroupDecisionError::InvalidHash);
        }
        if self.encode_pack().len() > DURABLE_OWNED_GROUP_EVENT_MAX_BYTES {
            return Err(DurableOwnedGroupDecisionError::EventTooLarge);
        }
        Ok(())
    }

    pub(crate) fn admit_map(&self, handles: &DurableOwnedGroupMapHandlesV1, frontiers: &DurableOwnedGroupMapFrontiersV1) -> Result<(), DurableOwnedGroupDecisionError> {
        self.validate()?;
        if handles.drawing != self.drawing.reference || handles.value != self.value.reference {
            return Err(DurableOwnedGroupDecisionError::InvalidIdentity);
        }
        if (frontiers.parent_generation, frontiers.parent_revision) != (self.parent.expected_generation, self.parent.expected_revision)
            || (frontiers.drawing_generation, frontiers.drawing_revision) != (self.drawing.expected_generation, self.drawing.expected_revision)
            || (frontiers.value_generation, frontiers.value_revision) != (self.value.expected_generation, self.value.expected_revision)
        {
            return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
        }
        Ok(())
    }
}

impl ArtifactDsl for DurableOwnedThreeMemberDecisionV1 {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = crate::os_store::semio_format::split_text_preamble(text).map(|(_, body)| body).unwrap_or(text);
        let record = crate::os_dsl::parse(body, &Self::__dsl_spec(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = crate::os_dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), crate::os_dsl::JoinMode::Document);
        let envelope = crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Dsl, 1).expect("valid durable group envelope");
        crate::os_store::semio_format::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for DurableOwnedThreeMemberDecisionV1 {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let inner = pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Pack, 1).map_err(|error| PackError::Schema(error.to_string()))?;
        Ok(crate::os_store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = crate::os_store::semio_format::unwrap_binary(bytes).map_err(|error| PackError::Schema(error.to_string()))?;
        if envelope.envelope_id() != <Self as ArtifactDsl>::envelope_id() {
            return Err(PackError::Schema("durable group pack envelope mismatch".into()));
        }
        let (record, _) = pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(crate::os_store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<crate::os_dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

#[cfg(test)]
mod tests {
    use super::super::fixture_mutations::demo::{DemoMutation, SetN};
    use super::super::tests::{demo_closable_store_owners, DemoSnapshot};
    use super::*;

    fn fixture() -> serde_json::Value {
        serde_json::from_str(include_str!("🧪️fixtures/🔣️.json")).expect("durable group fixture")
    }

    fn hex(value: &str) -> Vec<u8> {
        value.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect()
    }

    fn hex_string(bytes: &[u8]) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut encoded = String::with_capacity(bytes.len() * 2);
        for byte in bytes {
            encoded.push(HEX[usize::from(byte >> 4)] as char);
            encoded.push(HEX[usize::from(byte & 0x0f)] as char);
        }
        encoded
    }

    fn revision(value: &str) -> [u8; 32] {
        hex(value).try_into().expect("fixed revision")
    }

    fn reference(value: &serde_json::Value) -> crate::os_io::ArtifactRef {
        crate::os_pack::json::from_json_str(&serde_json::to_string(value).unwrap()).expect("fixture reference")
    }

    fn member(value: &serde_json::Value) -> DurableOwnedGroupMemberV1 {
        DurableOwnedGroupMemberV1 {
            role: value["role"].as_str().unwrap().into(),
            reference: reference(&value["reference"]),
            owner: if value["owner"].is_null() { None } else { Some(crate::os_pack::json::from_json_str(&serde_json::to_string(&value["owner"]).unwrap()).expect("fixture owner")) },
            expected_generation: value["expectedGeneration"].as_u64().unwrap(),
            expected_revision: revision(value["expectedRevisionHex"].as_str().unwrap()),
            recovery_schema: value["recoverySchema"].as_str().unwrap().into(),
            recovery_pack: hex(value["callerRecoveryPackHex"].as_str().unwrap()),
            recovery_pack_sha256: value["callerRecoveryPackSha256"].as_str().unwrap().into(),
            unbound_outcome_sha256: value["unboundOutcomeSha256"].as_str().unwrap().into(),
            post_generation: value["postGeneration"].as_u64().unwrap(),
            post_revision: revision(value["postRevisionHex"].as_str().unwrap()),
        }
    }

    fn prepared_outcome(role: &str, recovery_schema: &str, generation: u64, base_revision: [u8; 32], ordinal: u64) -> (ArtifactStoreOneItemPrepared<DslValue, String>, DurableStorePreparedOutcomeV1) {
        let actor = format!("map-owner-{role}");
        let next_clock = HybridLogicalTimestamp { actor: ordinal, physical_ms: 1_000 + ordinal, logical: ordinal + 1 };
        let authority = Arc::new(ArtifactStoreOneItemLiveAuthority {
            operation: semio_framework_job::OperationId(100 + ordinal),
            generation: semio_framework_job::Generation(generation),
            base_revision,
            base_applied_edit_count: ordinal as usize,
            next_sequence_number: ordinal as i32 + 10,
            next_clock,
            actor: actor.clone(),
            group_id: None,
        });
        let edit_id = format!("map-{role}-edit-{ordinal}");
        let edit = Edit {
            id: edit_id.clone(),
            actor: Some(actor.clone()),
            forwards: vec![format!("{role}:forward")],
            inverse: vec![format!("{role}:inverse")],
            mutation_meta: vec![crate::os_spr::MutationMeta {
                mutation_id: Some(crate::os_spr::MutationId(format!("{edit_id}#0"))),
                dependencies: Vec::new(),
                base_version: generation,
                author_id: Some(crate::os_spr::ActorId(actor)),
                timestamp: next_clock,
                undo_policy: crate::os_spr::UndoPolicy::ExactBaseOnly,
                payload_hash: None,
                semantic_kind: Some(crate::os_spr::SchemaId(recovery_schema.into())),
                label: Some(format!("prepare {role}")),
                group_id: None,
                origin: Default::default(),
            }],
            description: Some(format!("prepared {role} outcome")),
            coalesce_key: None,
            sequence_number: ordinal as i32 + 10,
            started_at: format!("2026-09-05T00:00:0{ordinal}Z"),
            finished_at: Some(format!("2026-09-05T00:00:1{ordinal}Z")),
        };
        let post_snapshot = Arc::new(DslValue::Object(vec![("role".into(), DslValue::String(role.into())), ("ordinal".into(), DslValue::uint(ordinal))]));
        let prepared = authority.prepare_one_item(edit, post_snapshot).expect("Store seals one exact unbound prepared owner");
        let outcome = prepared.durable_unbound_outcome(recovery_schema).expect("Store derives and verifies the unbound outcome pack");
        (prepared, outcome)
    }

    fn outcomes() -> DurableStorePreparedOutcomesV1 {
        let fixture = fixture();
        let parent = &fixture["members"]["parent"];
        let drawing = &fixture["members"]["drawing"];
        let value = &fixture["members"]["value"];
        DurableStorePreparedOutcomesV1 {
            parent: prepared_outcome(PARENT_ROLE, parent["recoverySchema"].as_str().unwrap(), parent["expectedGeneration"].as_u64().unwrap(), revision(parent["expectedRevisionHex"].as_str().unwrap()), 1).1,
            drawing: prepared_outcome(DRAWING_ROLE, drawing["recoverySchema"].as_str().unwrap(), drawing["expectedGeneration"].as_u64().unwrap(), revision(drawing["expectedRevisionHex"].as_str().unwrap()), 2).1,
            value: prepared_outcome(VALUE_ROLE, value["recoverySchema"].as_str().unwrap(), value["expectedGeneration"].as_u64().unwrap(), revision(value["expectedRevisionHex"].as_str().unwrap()), 3).1,
        }
    }

    fn decision() -> DurableOwnedThreeMemberDecisionV1 {
        let fixture = fixture();
        let outcomes = outcomes();
        DurableOwnedThreeMemberDecisionV1::seal_fixture(
            crate::os_pack::json::from_json_str(&serde_json::to_string(&fixture["anchor"]).unwrap()).expect("fixture anchor"),
            member(&fixture["members"]["parent"]),
            member(&fixture["members"]["drawing"]),
            member(&fixture["members"]["value"]),
            &outcomes,
        )
        .expect("fixture decision")
    }

    async fn owned_store(id: &str, dialect: crate::os_io::ArtifactDialect, owner: Option<OwnerRef>) -> ArtifactStore<DemoSnapshot, DemoMutation> {
        let mut envelope = crate::os_store::create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", id, DemoSnapshot { n: Some(0) }, None);
        envelope.dialect = Some(dialect);
        envelope.owner = owner;
        let mut store = ArtifactStore::new(envelope).await.expect("owned group fixture Store");
        store.install_member_store_owners_exact(demo_closable_store_owners());
        store
    }

    fn store_prepared(store: &ArtifactStore<DemoSnapshot, DemoMutation>, ordinal: u64, next: i32) -> ArtifactStoreOneItemPrepared<DemoSnapshot, DemoMutation> {
        let actor = format!("map-owner-{ordinal}");
        let next_clock = HybridLogicalTimestamp { actor: ordinal, physical_ms: 2_000 + ordinal, logical: ordinal + 1 };
        let authority = Arc::new(ArtifactStoreOneItemLiveAuthority {
            operation: semio_framework_job::OperationId(200 + ordinal),
            generation: semio_framework_job::Generation(store.generation),
            base_revision: store.content_revision,
            base_applied_edit_count: store.applied_edit_ids.len(),
            next_sequence_number: store.edit_sequence + 1,
            next_clock,
            actor: actor.clone(),
            group_id: None,
        });
        let edit_id = format!("map-store-edit-{ordinal}");
        let edit = Edit {
            id: edit_id.clone(),
            actor: Some(actor.clone()),
            forwards: vec![DemoMutation::SetN(SetN { n: next })],
            inverse: vec![DemoMutation::SetN(SetN { n: store.snapshot_ref().n.unwrap_or_default() })],
            mutation_meta: vec![crate::os_spr::MutationMeta {
                mutation_id: Some(crate::os_spr::MutationId(format!("{edit_id}#0"))),
                dependencies: Vec::new(),
                base_version: store.generation,
                author_id: Some(crate::os_spr::ActorId(actor)),
                timestamp: next_clock,
                undo_policy: crate::os_spr::UndoPolicy::ExactBaseOnly,
                payload_hash: None,
                semantic_kind: Some(crate::os_spr::SchemaId(format!("map-role-{ordinal}"))),
                label: Some(format!("durable map member {ordinal}")),
                group_id: None,
                origin: Default::default(),
            }],
            description: Some(format!("durable map member {ordinal}")),
            coalesce_key: None,
            sequence_number: store.edit_sequence + 1,
            started_at: format!("2026-09-05T00:01:0{ordinal}Z"),
            finished_at: Some(format!("2026-09-05T00:01:1{ordinal}Z")),
        };
        authority.prepare_one_item(edit, Arc::new(DemoSnapshot { n: Some(next) })).expect("Store seals one exact group candidate")
    }

    async fn owned_three_stores() -> (ArtifactStore<DemoSnapshot, DemoMutation>, ArtifactStore<DemoSnapshot, DemoMutation>, ArtifactStore<DemoSnapshot, DemoMutation>) {
        let parent_dialect = crate::os_io::ArtifactDialect { artifact_kind: "s.gis.gismap".into(), standard: "1".into(), subset: "*".into() };
        let child_dialect = |subset: &str| crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() };
        let parent_reference = crate::os_io::ArtifactRef { artifact_id: "map-a".into(), dialect: parent_dialect.clone() };
        let parent = owned_store("map-a", parent_dialect, None).await;
        let drawing = owned_store("gismap-drawing", child_dialect(DRAWING_ROLE), Some(OwnerRef { parent: parent_reference.clone(), slot: DRAWING_ROLE.into(), child_id: "gismap-drawing".into() })).await;
        let value = owned_store("gismap-value", child_dialect(VALUE_ROLE), Some(OwnerRef { parent: parent_reference, slot: VALUE_ROLE.into(), child_id: "gismap-value".into() })).await;
        (parent, drawing, value)
    }

    fn close_demo_artifact_store(store: &mut ArtifactStore<DemoSnapshot, DemoMutation>) {
        for _ in 0..4_096 {
            let step = crate::os_store::SpaceMember::close_owned_step(store, 1, 512).expect("durable group fixture Store closes under its bounded owner grant");
            if step == crate::os_store::SnapshotRetirementStep::Complete {
                assert!(crate::os_store::SpaceMember::close_owned_terminal_is_empty(store));
                return;
            }
        }
        panic!("durable group fixture Store did not reach its exact terminal-empty witness");
    }

    async fn assert_erased_snapshot_authority(store: &mut ArtifactStore<DemoSnapshot, DemoMutation>, expected: i32) {
        let generation = store.generation();
        let revision = store.content_revision();
        let read = crate::os_store::SpaceMember::snapshot_read_erased(store).await.expect("erased group read publishes its selected authority");
        assert_eq!(read.typed::<DemoSnapshot>().expect("erased group read retains the exact snapshot type").n, Some(expected));
        assert!(store.snapshot_read_leases.authority_matches(generation, revision));
        let mut retirement = crate::os_store::SpaceMember::retire_snapshot_read_erased(store, read).unwrap_or_else(|_| panic!("erased group read returns to its exact Store"));
        for _ in 0..64 {
            match retirement.close_step(1, 4096).expect("erased group read retirement remains infallible") {
                crate::os_store::SnapshotRetirementStep::Complete => {
                    assert!(retirement.terminal_is_empty());
                    return;
                }
                crate::os_store::SnapshotRetirementStep::Pending { .. } => {}
                crate::os_store::SnapshotRetirementStep::Blocked => panic!("erased group read retirement has no external wait"),
            }
        }
        panic!("erased group read retirement must terminate within its bounded fixture");
    }

    fn bound_three(
        parent: &ArtifactStore<DemoSnapshot, DemoMutation>,
        drawing: &ArtifactStore<DemoSnapshot, DemoMutation>,
        value: &ArtifactStore<DemoSnapshot, DemoMutation>,
    ) -> DurableOwnedThreeStoreBoundV1<DemoSnapshot, DemoMutation, DemoSnapshot, DemoMutation, DemoSnapshot, DemoMutation> {
        DurableOwnedThreeStorePreparedV1::from_store_prepared(store_prepared(parent, 1, 7), store_prepared(drawing, 2, 11), store_prepared(value, 3, 13))
            .expect("Store owns exactly three unbound candidates")
            .bind_store_owned(parent, drawing, value)
            .expect("Store binds exactly three candidate owners")
    }

    #[derive(Clone, Copy)]
    enum FakeJournalResolution {
        Commit,
        AbsentAfterCancel,
        ErrorThenCommit,
        WrongAnchorThenAbsentAfterCancel,
    }

    #[derive(Default)]
    struct FakeJournalState {
        begins: usize,
        advances: usize,
        cancelled: bool,
        close_count: usize,
        decision_pack: Vec<u8>,
    }

    struct FakeJournalSink {
        resolution: FakeJournalResolution,
        state: Arc<std::sync::Mutex<FakeJournalState>>,
    }

    struct FakeJournalCommit {
        resolution: FakeJournalResolution,
        state: Arc<std::sync::Mutex<FakeJournalState>>,
        decision_sha256: String,
        anchor_sha256: String,
        close_started: bool,
    }

    impl DurableOwnedGroupJournalSinkV1 for FakeJournalSink {
        fn begin_commit(&mut self, decision_pack: Vec<u8>, decision_sha256: String) -> Box<dyn DurableOwnedGroupJournalCommitV1> {
            let decision = DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&decision_pack).expect("fake journal independently admits the canonical decision");
            assert_eq!(decision.decision_sha256, decision_sha256);
            let mut state = self.state.lock().unwrap();
            state.begins += 1;
            state.decision_pack = decision_pack;
            drop(state);
            Box::new(FakeJournalCommit { resolution: self.resolution, state: Arc::clone(&self.state), decision_sha256, anchor_sha256: decision.anchor_sha256, close_started: false })
        }
    }

    impl DurableOwnedGroupJournalCommitV1 for FakeJournalCommit {
        fn advance(&mut self, grant: crate::os_store::ArtifactStoreOneItemGrant) -> Result<DurableOwnedGroupJournalAdvanceV1, String> {
            if !grant.permits_one() {
                return Ok(DurableOwnedGroupJournalAdvanceV1::Pending);
            }
            let mut state = self.state.lock().unwrap();
            state.advances += 1;
            if state.advances == 1 {
                return Ok(DurableOwnedGroupJournalAdvanceV1::Pending);
            }
            match self.resolution {
                FakeJournalResolution::Commit => {
                    Ok(DurableOwnedGroupJournalAdvanceV1::Committed(DurableOwnedGroupJournalReceiptV1 { anchor_sha256: self.anchor_sha256.clone(), decision_sha256: self.decision_sha256.clone(), transaction_id: 41, segment_index: 7 }))
                }
                FakeJournalResolution::AbsentAfterCancel if state.cancelled => Ok(DurableOwnedGroupJournalAdvanceV1::Absent),
                FakeJournalResolution::AbsentAfterCancel => Ok(DurableOwnedGroupJournalAdvanceV1::Pending),
                FakeJournalResolution::ErrorThenCommit if state.advances == 2 => Err("uncertain sync result".into()),
                FakeJournalResolution::ErrorThenCommit => {
                    Ok(DurableOwnedGroupJournalAdvanceV1::Committed(DurableOwnedGroupJournalReceiptV1 { anchor_sha256: self.anchor_sha256.clone(), decision_sha256: self.decision_sha256.clone(), transaction_id: 43, segment_index: 9 }))
                }
                FakeJournalResolution::WrongAnchorThenAbsentAfterCancel if state.cancelled => Ok(DurableOwnedGroupJournalAdvanceV1::Absent),
                FakeJournalResolution::WrongAnchorThenAbsentAfterCancel => {
                    Ok(DurableOwnedGroupJournalAdvanceV1::Committed(DurableOwnedGroupJournalReceiptV1 { anchor_sha256: "0".repeat(64), decision_sha256: self.decision_sha256.clone(), transaction_id: 47, segment_index: 11 }))
                }
            }
        }

        fn cancel(&mut self) {
            self.state.lock().unwrap().cancelled = true;
        }

        fn begin_close(&mut self) {
            self.state.lock().unwrap().close_count += 1;
            self.close_started = true;
        }

        fn close_step(&mut self, grant: crate::os_store::ArtifactStoreOneItemGrant) -> Result<crate::os_store::SnapshotRetirementStep, String> {
            if !self.close_started || !grant.permits_one() {
                return Ok(crate::os_store::SnapshotRetirementStep::Blocked);
            }
            Ok(crate::os_store::SnapshotRetirementStep::Complete)
        }

        fn terminal_is_empty(&self) -> bool {
            self.close_started
        }
    }

    #[test]
    fn durable_owned_group_decision_matches_neutral_canonical_hash_and_bounds() {
        let fixture = fixture();
        let decision = decision();
        assert_eq!(decision.anchor_sha256, fixture["expected"]["anchorSha256"]);
        assert_eq!(decision.decision_sha256, fixture["expected"]["decisionSha256"]);
        assert_eq!(decision.canonical_unsigned_json(), fixture["expected"]["unsignedJson"]);
        assert_eq!(fixture["cases"].as_array().unwrap().len(), 8);
        let json = decision.canonical_json();
        assert_eq!(DurableOwnedThreeMemberDecisionV1::parse_canonical_json(&json).unwrap(), decision);
        let pack = decision.encode_pack();
        assert!(pack.len() <= DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);
        assert_eq!(DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&pack).unwrap(), decision);
        let handles = DurableOwnedGroupMapHandlesV1 { drawing: decision.drawing.reference.clone(), value: decision.value.reference.clone() };
        let frontiers = DurableOwnedGroupMapFrontiersV1 {
            parent_generation: decision.parent.expected_generation,
            parent_revision: decision.parent.expected_revision,
            drawing_generation: decision.drawing.expected_generation,
            drawing_revision: decision.drawing.expected_revision,
            value_generation: decision.value.expected_generation,
            value_revision: decision.value.expected_revision,
        };
        decision.admit_map(&handles, &frontiers).unwrap();
        let mut bound_derivations = decision.clone();
        bound_derivations.parent.recovery_pack.push(0);
        bound_derivations.parent.recovery_pack_sha256 = semio_framework_hash::sha256_hex(&bound_derivations.parent.recovery_pack);
        bound_derivations.parent.post_generation += 1;
        bound_derivations.parent.post_revision[0] ^= 1;
        assert_eq!(bound_derivations.canonical_unsigned_json(), decision.canonical_unsigned_json());
        assert_eq!(semio_framework_hash::sha256_hex(bound_derivations.canonical_unsigned_json().as_bytes()), decision.decision_sha256);
        assert_eq!(DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES * 3 + DURABLE_OWNED_GROUP_STRUCTURAL_MAX_BYTES, 490_096);
        assert!(490_096 <= DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);
        let varint = |value: usize| {
            if value < 1 << 7 {
                1
            } else if value < 1 << 14 {
                2
            } else if value < 1 << 21 {
                3
            } else if value < 1 << 28 {
                4
            } else {
                5
            }
        };
        let frame = |payload: usize| varint(payload + 2) + payload + 10;
        assert_eq!(129 + frame(8) + frame(DURABLE_OWNED_GROUP_EVENT_MAX_BYTES) + frame(12) + 75, 491_779);
        assert!(491_779 <= 507_904);
    }

    #[test]
    fn durable_store_prepared_outcome_derives_and_verifies_exact_unbound_bytes() {
        let fixture = fixture();
        let parent = &fixture["members"]["parent"];
        let (prepared, outcome) = prepared_outcome(PARENT_ROLE, parent["recoverySchema"].as_str().unwrap(), parent["expectedGeneration"].as_u64().unwrap(), revision(parent["expectedRevisionHex"].as_str().unwrap()), 1);
        let verified = outcome.verify_inverse::<DslValue, String>().expect("Store-owned bytes invert to the same typed owners");
        assert_eq!(ValueToValue::to_value(verified.edit.as_ref()), ValueToValue::to_value(prepared.edit.as_ref()),);
        assert_eq!(verified.post_snapshot.encode_pack(), prepared.post_snapshot.encode_pack(),);
        assert_eq!(outcome.sha256, semio_framework_hash::sha256_hex(&outcome.pack));
        assert_eq!(hex_string(&outcome.pack), parent["unboundOutcomePackHex"]);
        assert_eq!(outcome.sha256, parent["unboundOutcomeSha256"]);

        let mut modified_bytes = outcome.clone();
        *modified_bytes.pack.last_mut().expect("nonempty outcome pack") ^= 1;
        assert!(matches!(modified_bytes.verify_inverse::<DslValue, String>(), Err(DurableOwnedGroupDecisionError::InvalidHash)));

        let mut reordered = DurableUnboundOneItemOutcomeV1::decode_canonical_pack(&outcome.pack).expect("canonical outcome");
        let mut reordered_edit: DslValue = crate::os_pack::json::from_json_str(std::str::from_utf8(&reordered.edit_without_group_canonical_json).unwrap()).expect("edit value");
        let DslValue::Object(fields) = &mut reordered_edit else { panic!("edit projection is an object") };
        fields.rotate_left(1);
        reordered.edit_without_group_canonical_json = crate::os_pack::json::to_json_string(&reordered_edit).into_bytes();
        let reordered_pack = reordered.encode_pack();
        let reordered = DurableStorePreparedOutcomeV1 { recovery_schema: outcome.recovery_schema.clone(), sha256: semio_framework_hash::sha256_hex(&reordered_pack), pack: reordered_pack };
        assert!(matches!(reordered.verify_inverse::<DslValue, String>(), Err(DurableOwnedGroupDecisionError::NonCanonical)));

        let mut retagged = DurableUnboundOneItemOutcomeV1::decode_canonical_pack(&outcome.pack).expect("canonical outcome");
        let mut retagged_edit: DslValue = crate::os_pack::json::from_json_str(std::str::from_utf8(&retagged.edit_without_group_canonical_json).unwrap()).expect("edit value");
        let DslValue::Object(fields) = &mut retagged_edit else { panic!("edit projection is an object") };
        let sequence = fields.iter_mut().find(|(name, _)| name == "sequenceNumber").expect("sequence field");
        sequence.1 = DslValue::float(sequence.1.as_i64().expect("signed sequence") as f64);
        retagged.edit_without_group_canonical_json = crate::os_pack::json::to_json_string(&retagged_edit).into_bytes();
        let retagged_pack = retagged.encode_pack();
        let retagged = DurableStorePreparedOutcomeV1 { recovery_schema: outcome.recovery_schema.clone(), sha256: semio_framework_hash::sha256_hex(&retagged_pack), pack: retagged_pack };
        assert!(retagged.verify_inverse::<DslValue, String>().is_err());

        let all = outcomes();
        for (role, outcome) in [(PARENT_ROLE, &all.parent), (DRAWING_ROLE, &all.drawing), (VALUE_ROLE, &all.value)] {
            assert_eq!(hex_string(&outcome.pack), fixture["members"][role]["unboundOutcomePackHex"]);
            assert_eq!(outcome.sha256, fixture["members"][role]["unboundOutcomeSha256"]);
        }
    }

    #[test]
    fn durable_owned_group_decision_rejects_forged_identity_commitment_and_capacity() {
        let decision = decision();
        let mut forged = decision.clone();
        forged.drawing.owner.as_mut().unwrap().slot = "image".into();
        assert_eq!(forged.validate(), Err(DurableOwnedGroupDecisionError::InvalidOwner));
        let mut forged_child = decision.clone();
        forged_child.drawing.reference.artifact_id = "image".into();
        assert_eq!(forged_child.validate(), Err(DurableOwnedGroupDecisionError::InvalidOwner));
        let mut forged_parent = decision.clone();
        forged_parent.value.owner.as_mut().unwrap().parent.artifact_id = "map-b".into();
        assert_eq!(forged_parent.validate(), Err(DurableOwnedGroupDecisionError::InvalidOwner));
        let mut tampered = decision.clone();
        tampered.value.recovery_pack[0] ^= 1;
        assert_eq!(tampered.validate(), Err(DurableOwnedGroupDecisionError::InvalidHash));
        let mut oversized = decision.clone();
        oversized.parent.recovery_pack.resize(DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES + 1, 0);
        oversized.parent.recovery_pack_sha256 = semio_framework_hash::sha256_hex(&oversized.parent.recovery_pack);
        assert_eq!(oversized.validate(), Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge));
        assert_eq!(DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&vec![0; DURABLE_OWNED_GROUP_EVENT_MAX_BYTES + 1]), Err(DurableOwnedGroupDecisionError::EventTooLarge));
        let canonical = decision.canonical_json();
        let unknown = format!("{},\"image\":{{}}}}", canonical.strip_suffix('}').unwrap());
        assert!(DurableOwnedThreeMemberDecisionV1::parse_canonical_json(&unknown).is_err());
        let duplicate = canonical.replacen("{\"schema\":", "{\"schema\":\"semio.store.durable-owned-three-member-decision.v1\",\"schema\":", 1);
        assert_eq!(DurableOwnedThreeMemberDecisionV1::parse_canonical_json(&duplicate), Err(DurableOwnedGroupDecisionError::NonCanonical));
        let mut stale = DurableOwnedGroupMapFrontiersV1 {
            parent_generation: decision.parent.expected_generation,
            parent_revision: decision.parent.expected_revision,
            drawing_generation: decision.drawing.expected_generation,
            drawing_revision: decision.drawing.expected_revision,
            value_generation: decision.value.expected_generation,
            value_revision: decision.value.expected_revision,
        };
        stale.value_generation += 1;
        let handles = DurableOwnedGroupMapHandlesV1 { drawing: decision.drawing.reference.clone(), value: decision.value.reference.clone() };
        assert_eq!(decision.admit_map(&handles, &stale), Err(DurableOwnedGroupDecisionError::InvalidFrontier));
        let mut noncanonical = decision.encode_pack();
        noncanonical.push(0);
        assert!(DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&noncanonical).is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn durable_store_owned_three_member_bind_and_base_recovery_retain_exact_private_owners() {
        let (mut parent_store, mut drawing_store, mut value_store) = owned_three_stores().await;
        let bound = bound_three(&parent_store, &drawing_store, &value_store);
        let decision = bound.decision.clone();
        assert!(decision.validate().is_ok());
        assert_eq!(decision.parent.expected_revision, parent_store.content_revision_now());
        assert_eq!(decision.drawing.expected_revision, drawing_store.content_revision_now());
        assert_eq!(decision.value.expected_revision, value_store.content_revision_now());
        assert_eq!(bound.parent.prepared.edit.mutation_meta[0].group_id.as_deref(), Some(decision.decision_sha256.as_str()));
        assert_eq!(bound.drawing.prepared.edit.mutation_meta[0].group_id.as_deref(), Some(decision.decision_sha256.as_str()));
        assert_eq!(bound.value.prepared.edit.mutation_meta[0].group_id.as_deref(), Some(decision.decision_sha256.as_str()));
        for member in [&decision.parent, &decision.drawing, &decision.value] {
            assert!(member.recovery_pack.starts_with(b"\x89SEM\r\n\x1a\n"));
            assert_eq!(member.recovery_pack_sha256, semio_framework_hash::sha256_hex(&member.recovery_pack));
        }
        let recovered = match decision.recover_store_owned(&parent_store, &drawing_store, &value_store).expect("all-base recovery verifies every bound outcome") {
            DurableOwnedThreeStoreRecoveryV1::Apply(recovered) => recovered,
            DurableOwnedThreeStoreRecoveryV1::AlreadyApplied => panic!("base stores cannot report already applied"),
        };
        for prepared in [&recovered.parent.prepared, &recovered.drawing.prepared, &recovered.value.prepared] {
            prepared.seal.authority.validate_prepared(prepared).expect("recovery reconstructs a valid private Store seal");
            assert_eq!(prepared.edit.mutation_meta[0].group_id.as_deref(), Some(decision.decision_sha256.as_str()));
        }
        let mut tampered = decision.clone();
        tampered.value.recovery_pack[0] ^= 1;
        assert!(tampered.recover_store_owned(&parent_store, &drawing_store, &value_store).is_err());
        drop(recovered);
        drop(bound);
        close_demo_artifact_store(&mut value_store);
        close_demo_artifact_store(&mut drawing_store);
        close_demo_artifact_store(&mut parent_store);
    }

    #[semio_framework_async_macros::async_test]
    async fn durable_store_group_journal_commit_flips_one_shared_root_then_adopts_exactly_once() {
        let (mut parent, mut drawing, mut value) = owned_three_stores().await;
        let mut coordinator = bound_three(&parent, &drawing, &value).begin_retained_commit().expect("Store creates one retained group coordinator");
        let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
        let mut sink = FakeJournalSink { resolution: FakeJournalResolution::Commit, state: Arc::clone(&state) };
        let grant = crate::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES };
        while coordinator.phase() != DurableOwnedThreeStoreCommitPhaseV1::StartingJournal {
            coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("fixed-three staging turn");
            let read = capture_store_owned_three_snapshot(&parent, &drawing, &value).expect("pending partial staging reads all-old");
            assert_eq!([read.parent.n, read.drawing.n, read.value.n], [Some(0), Some(0), Some(0)]);
        }
        let decision_bytes = coordinator.decision_pack.as_ref().expect("starting journal retains decision bytes").len();
        let one_byte = crate::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1 };
        assert_eq!(coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, one_byte).unwrap(), DurableOwnedThreeStoreCommitAdvanceV1::Blocked);
        assert_eq!(state.lock().unwrap().begins, 0);
        let insufficient = crate::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: decision_bytes - 1 };
        assert_eq!(coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, insufficient).unwrap(), DurableOwnedThreeStoreCommitAdvanceV1::Blocked);
        assert_eq!(state.lock().unwrap().begins, 0);
        let exact = crate::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: decision_bytes };
        assert_eq!(coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, exact).unwrap(), DurableOwnedThreeStoreCommitAdvanceV1::Progress(DurableOwnedThreeStoreCommitPhaseV1::Journal));
        assert_eq!(state.lock().unwrap().begins, 1);
        let mut observed_pending = false;
        let mut observed_committed = false;
        let mut rejected_partial_committed_root = false;
        let mut rejected_foreign_visibility = false;
        let mut rejected_structurally_false_adoption = false;
        for _ in 0..64 {
            let step = coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("retained group commit turn");
            let read = capture_store_owned_three_snapshot(&parent, &drawing, &value).expect("one captured group read");
            if coordinator.phase() == DurableOwnedThreeStoreCommitPhaseV1::Journal {
                assert_eq!([read.parent.n, read.drawing.n, read.value.n], [Some(0), Some(0), Some(0)]);
                assert!(parent.set_local_actor_id(None).is_err());
                assert!(drawing.invalidate_after_replay().is_err());
                observed_pending = true;
            }
            if matches!(
                coordinator.phase(),
                DurableOwnedThreeStoreCommitPhaseV1::PublishingParentLease
                    | DurableOwnedThreeStoreCommitPhaseV1::PublishingDrawingLease
                    | DurableOwnedThreeStoreCommitPhaseV1::PublishingValueLease
                    | DurableOwnedThreeStoreCommitPhaseV1::AdoptingParent
                    | DurableOwnedThreeStoreCommitPhaseV1::AdoptingDrawing
                    | DurableOwnedThreeStoreCommitPhaseV1::AdoptingValue
                    | DurableOwnedThreeStoreCommitPhaseV1::ClearingParent
                    | DurableOwnedThreeStoreCommitPhaseV1::ClearingDrawing
                    | DurableOwnedThreeStoreCommitPhaseV1::ClearingValue
                    | DurableOwnedThreeStoreCommitPhaseV1::AwaitingAck
            ) {
                assert_eq!([read.parent.n, read.drawing.n, read.value.n], [Some(7), Some(11), Some(13)]);
                if !rejected_partial_committed_root {
                    let retained_value_root = value.durable_group_root.take().expect("committed value root");
                    assert!(capture_store_owned_three_snapshot(&parent, &drawing, &value).is_err());
                    *value.durable_group_root = Some(retained_value_root);
                    rejected_partial_committed_root = true;
                }
                if !rejected_foreign_visibility {
                    let foreign_owner = crate::os_vcs::ArtifactGroupVisibilityOwner::new();
                    let original_visibility = std::mem::replace(&mut value.durable_group_root.as_mut().expect("committed value root").visibility, foreign_owner.view());
                    assert!(capture_store_owned_three_snapshot(&parent, &drawing, &value).is_err());
                    value.durable_group_root.as_mut().expect("committed value root").visibility = original_visibility;
                    drop(foreign_owner);
                    rejected_foreign_visibility = true;
                }
                if !rejected_structurally_false_adoption && coordinator.phase() == DurableOwnedThreeStoreCommitPhaseV1::PublishingParentLease {
                    parent.durable_group_root.as_mut().expect("committed parent root").adopted = true;
                    assert!(capture_store_owned_three_snapshot(&parent, &drawing, &value).is_err());
                    parent.durable_group_root.as_mut().expect("committed parent root").adopted = false;
                    rejected_structurally_false_adoption = true;
                }
                assert_erased_snapshot_authority(&mut parent, 7).await;
                assert_erased_snapshot_authority(&mut drawing, 11).await;
                assert_erased_snapshot_authority(&mut value, 13).await;
                observed_committed = true;
            }
            if let DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(receipt) = step {
                assert!(coordinator.acknowledge(&receipt));
            }
            if coordinator.terminal_is_empty() {
                break;
            }
        }
        assert!(observed_pending && observed_committed && rejected_partial_committed_root && rejected_foreign_visibility && rejected_structurally_false_adoption && coordinator.terminal_is_empty());
        assert_eq!([parent.generation(), drawing.generation(), value.generation()], [1, 1, 1]);
        assert_eq!([parent.applied_edit_ids().len(), drawing.applied_edit_ids().len(), value.applied_edit_ids().len()], [1, 1, 1]);
        let journal = state.lock().unwrap();
        assert_eq!((journal.begins, journal.advances), (1, 2));
        assert_eq!(journal.close_count, 1);
        assert!(!journal.cancelled && !journal.decision_pack.is_empty());
        drop(journal);
        close_demo_artifact_store(&mut value);
        close_demo_artifact_store(&mut drawing);
        close_demo_artifact_store(&mut parent);
    }

    #[semio_framework_async_macros::async_test]
    async fn durable_store_group_cancellation_waits_for_trusted_absence_then_restores_all_old_roots() {
        let (mut parent, mut drawing, mut value) = owned_three_stores().await;
        let mut coordinator = bound_three(&parent, &drawing, &value).begin_retained_commit().expect("Store creates one retained group coordinator");
        let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
        let mut sink = FakeJournalSink { resolution: FakeJournalResolution::AbsentAfterCancel, state: Arc::clone(&state) };
        let grant = crate::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES };
        while coordinator.phase() != DurableOwnedThreeStoreCommitPhaseV1::Journal {
            coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("pre-journal stage");
        }
        assert!(coordinator.cancel());
        for _ in 0..64 {
            coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("cancel resolution turn");
            let read = capture_store_owned_three_snapshot(&parent, &drawing, &value).expect("cancelled captured group read");
            assert_eq!([read.parent.n, read.drawing.n, read.value.n], [Some(0), Some(0), Some(0)]);
            if coordinator.terminal_is_empty() {
                break;
            }
        }
        assert!(coordinator.terminal_is_empty());
        assert_eq!([parent.generation(), drawing.generation(), value.generation()], [0, 0, 0]);
        assert_eq!([parent.applied_edit_ids().len(), drawing.applied_edit_ids().len(), value.applied_edit_ids().len()], [0, 0, 0]);
        let journal = state.lock().unwrap();
        assert!(journal.cancelled);
        assert_eq!(journal.close_count, 1);
        assert_eq!(journal.advances, 2);
        drop(journal);
        close_demo_artifact_store(&mut value);
        close_demo_artifact_store(&mut drawing);
        close_demo_artifact_store(&mut parent);
    }

    #[semio_framework_async_macros::async_test]
    async fn durable_store_group_stage_error_retains_abort_owner_until_every_root_is_empty() {
        let (mut parent, mut drawing, mut value) = owned_three_stores().await;
        let mut coordinator = bound_three(&parent, &drawing, &value).begin_retained_commit().expect("Store creates one retained group coordinator");
        let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
        let mut sink = FakeJournalSink { resolution: FakeJournalResolution::Commit, state: Arc::clone(&state) };
        let grant = crate::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES };
        coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("parent stages before injected drawing failure");
        assert_eq!(coordinator.phase(), DurableOwnedThreeStoreCommitPhaseV1::StagingDrawing);
        let drawing_retirement_factory = drawing.snapshot_retirement_factory.take();
        assert_eq!(coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant), Err(DurableOwnedGroupDecisionError::InvalidFrontier));
        assert_eq!(coordinator.phase(), DurableOwnedThreeStoreCommitPhaseV1::AbortingValue);
        *drawing.snapshot_retirement_factory = drawing_retirement_factory;
        for _ in 0..64 {
            coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("retained stage-error owner continues abort cleanup");
            if coordinator.terminal_is_empty() {
                break;
            }
        }
        assert!(coordinator.terminal_is_empty());
        assert!(parent.durable_group_root.is_none() && drawing.durable_group_root.is_none() && value.durable_group_root.is_none());
        assert_eq!([parent.generation(), drawing.generation(), value.generation()], [0, 0, 0]);
        let journal = state.lock().unwrap();
        assert_eq!((journal.begins, journal.close_count), (0, 0));
        drop(journal);
        close_demo_artifact_store(&mut value);
        close_demo_artifact_store(&mut drawing);
        close_demo_artifact_store(&mut parent);
    }

    #[semio_framework_async_macros::async_test]
    async fn durable_store_group_uncertain_journal_error_retries_same_owner_without_rebegin_or_visibility_change() {
        let (mut parent, mut drawing, mut value) = owned_three_stores().await;
        let mut coordinator = bound_three(&parent, &drawing, &value).begin_retained_commit().expect("Store creates one retained group coordinator");
        let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
        let mut sink = FakeJournalSink { resolution: FakeJournalResolution::ErrorThenCommit, state: Arc::clone(&state) };
        let grant = crate::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES };
        while coordinator.phase() != DurableOwnedThreeStoreCommitPhaseV1::Journal {
            coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("pre-journal stage");
        }
        coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("journal first pending turn");
        assert!(coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).is_err());
        assert_eq!(coordinator.phase(), DurableOwnedThreeStoreCommitPhaseV1::Journal);
        let pending = capture_store_owned_three_snapshot(&parent, &drawing, &value).expect("uncertain journal read remains coherent");
        assert_eq!([pending.parent.n, pending.drawing.n, pending.value.n], [Some(0), Some(0), Some(0)]);
        {
            let journal = state.lock().unwrap();
            assert_eq!((journal.begins, journal.advances), (1, 2));
        }
        for _ in 0..64 {
            let step = coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("same journal owner retries");
            if let DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(receipt) = step {
                assert!(coordinator.acknowledge(&receipt));
            }
            if coordinator.terminal_is_empty() {
                break;
            }
        }
        assert!(coordinator.terminal_is_empty());
        {
            let journal = state.lock().unwrap();
            assert_eq!((journal.begins, journal.advances), (1, 3));
        }
        let committed = capture_store_owned_three_snapshot(&parent, &drawing, &value).expect("retried journal commit read");
        assert_eq!([committed.parent.n, committed.drawing.n, committed.value.n], [Some(7), Some(11), Some(13)]);
        close_demo_artifact_store(&mut value);
        close_demo_artifact_store(&mut drawing);
        close_demo_artifact_store(&mut parent);
    }

    #[semio_framework_async_macros::async_test]
    async fn durable_map_mounted_operation_retains_every_live_owner_across_request_error_until_terminal_handoff() {
        let (parent, drawing, value) = owned_three_stores().await;
        let coordinator = bound_three(&parent, &drawing, &value).begin_retained_commit().expect("Store creates one retained Map coordinator before mounting any request-facing turn");
        let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
        let sink = Box::new(FakeJournalSink { resolution: FakeJournalResolution::ErrorThenCommit, state: Arc::clone(&state) });
        let mut operation = coordinator.mount_map(parent, drawing, value, sink);
        let grant = crate::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES };
        let mut observed_uncertain_error = false;
        let mut observed_committed = false;
        for _ in 0..128 {
            match operation.advance(grant) {
                Err(DurableOwnedGroupDecisionError::Codec(error)) if error == "uncertain sync result" => {
                    observed_uncertain_error = true;
                    assert_eq!(operation.phase(), Some(DurableOwnedThreeStoreCommitPhaseV1::Journal));
                    let snapshot = operation.capture_snapshot().expect("mounted error observation retains one coherent Map read");
                    assert_eq!([snapshot.parent.n, snapshot.drawing.n, snapshot.value.n], [Some(0), Some(0), Some(0)]);
                }
                Ok(DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(receipt)) => {
                    observed_committed = true;
                    let snapshot = operation.capture_snapshot().expect("mounted committed observation retains one coherent Map read");
                    assert_eq!([snapshot.parent.n, snapshot.drawing.n, snapshot.value.n], [Some(7), Some(11), Some(13)]);
                    assert!(operation.acknowledge(&receipt));
                }
                Ok(DurableOwnedThreeStoreCommitAdvanceV1::Complete) => break,
                Ok(DurableOwnedThreeStoreCommitAdvanceV1::Progress(_) | DurableOwnedThreeStoreCommitAdvanceV1::Blocked) => {}
                Err(error) => panic!("mounted Map operation returned an unexpected terminal error: {error}"),
            }
        }
        assert!(observed_uncertain_error && observed_committed);
        let owners = operation.take_terminal_owners().expect("terminal Map operation returns every live Store and sink owner exactly once");
        assert!(operation.terminal_is_empty());
        assert!(operation.take_terminal_owners().is_none());
        let DurableOwnedMapCommitOwnersV1 { mut parent, mut drawing, mut value, sink } = owners;
        drop(sink);
        {
            let journal = state.lock().unwrap();
            assert_eq!((journal.begins, journal.advances, journal.close_count), (1, 3, 1));
        }
        close_demo_artifact_store(&mut value);
        close_demo_artifact_store(&mut drawing);
        close_demo_artifact_store(&mut parent);
    }

    #[semio_framework_async_macros::async_test]
    async fn durable_store_group_rejects_foreign_anchor_receipt_before_visibility_and_aborts_only_after_absence() {
        let (mut parent, mut drawing, mut value) = owned_three_stores().await;
        let mut coordinator = bound_three(&parent, &drawing, &value).begin_retained_commit().expect("Store creates one retained group coordinator");
        let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
        let mut sink = FakeJournalSink { resolution: FakeJournalResolution::WrongAnchorThenAbsentAfterCancel, state: Arc::clone(&state) };
        let grant = crate::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES };
        while coordinator.phase() != DurableOwnedThreeStoreCommitPhaseV1::Journal {
            coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("pre-journal stage");
        }
        coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("journal first pending turn");
        assert_eq!(coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant), Err(DurableOwnedGroupDecisionError::InvalidHash));
        assert_eq!(coordinator.phase(), DurableOwnedThreeStoreCommitPhaseV1::Journal);
        let pending = capture_store_owned_three_snapshot(&parent, &drawing, &value).expect("foreign receipt cannot flip visibility");
        assert_eq!([pending.parent.n, pending.drawing.n, pending.value.n], [Some(0), Some(0), Some(0)]);
        assert!(coordinator.cancel());
        for _ in 0..64 {
            coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("trusted absence cleanup turn");
            if coordinator.terminal_is_empty() {
                break;
            }
        }
        assert!(coordinator.terminal_is_empty());
        let journal = state.lock().unwrap();
        assert_eq!((journal.begins, journal.advances, journal.close_count), (1, 3, 1));
        drop(journal);
        close_demo_artifact_store(&mut value);
        close_demo_artifact_store(&mut drawing);
        close_demo_artifact_store(&mut parent);
    }

    #[test]
    fn durable_json_carriers_preserve_numeric_kinds_and_reject_control_and_resource_excess() {
        let fixture = fixture();
        let numeric = DslValue::Object(vec![("u64".into(), DslValue::uint(u64::MAX)), ("i64".into(), DslValue::int(i64::MIN)), ("float".into(), DslValue::float(1.5))]);
        let encoded = crate::os_pack::json::to_json_string(&numeric);
        for row in fixture["carrier"]["numericCases"].as_array().unwrap() {
            assert!(encoded.contains(row["canonical"].as_str().unwrap()));
        }
        let decoded: DslValue = parse_canonical_json_value(encoded.as_bytes()).expect("typed canonical JSON preserves numeric tags and extrema");
        assert_eq!(decoded, numeric);
        let clock = HybridLogicalTimestamp { actor: u64::MAX, physical_ms: u64::MAX, logical: u64::MAX };
        let clock_json = crate::os_pack::json::to_json_string(&clock);
        assert!(clock_json.matches("18446744073709551615").count() == 3);
        assert_eq!(parse_canonical_json_value::<HybridLogicalTimestamp>(clock_json.as_bytes()).unwrap(), clock);

        let control = fixture["carrier"]["escapedControlIdentity"].as_str().unwrap();
        assert!(!valid_identity(control));
        let raw_control = b"\"map\0x\"";
        assert_eq!(validate_json_budget(raw_control), Err(DurableOwnedGroupDecisionError::NonCanonical));
        let over_depth = format!("{}0{}", "[".repeat(DURABLE_OWNED_GROUP_JSON_MAX_DEPTH + 1), "]".repeat(DURABLE_OWNED_GROUP_JSON_MAX_DEPTH + 1));
        assert_eq!(validate_json_budget(over_depth.as_bytes()), Err(DurableOwnedGroupDecisionError::InvalidOutcome));
        let over_items = format!("[{}]", std::iter::repeat_n("0", DURABLE_OWNED_GROUP_JSON_MAX_ITEMS + 1).collect::<Vec<_>>().join(","));
        assert!(over_items.len() <= DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES);
        assert_eq!(validate_json_budget(over_items.as_bytes()), Err(DurableOwnedGroupDecisionError::InvalidOutcome));
    }

    #[test]
    fn durable_decision_rejects_deflate_expansion_before_document_body_allocation() {
        let decision = decision();
        let envelope = crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<DurableOwnedThreeMemberDecisionV1 as ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Pack, 1).unwrap();
        let mut expanded_record = decision.__dsl_to_record();
        expanded_record.fields.insert(u16::MAX, FieldValue::Bytes64(vec![0; DURABLE_OWNED_GROUP_EVENT_MAX_BYTES + 1]));
        let mut expanded_options = PackEncodeOptions::default();
        expanded_options.chunk_threshold = u64::MAX;
        expanded_options.frame_size = u64::MAX;
        let expanded_inner = pack_rt::encode_document(&DurableOwnedThreeMemberDecisionV1::__dsl_spec(), &expanded_record, &expanded_options).unwrap();
        let expanded = crate::os_store::semio_format::wrap_binary(&envelope, &expanded_inner);
        assert!(expanded.len() <= DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);
        assert_eq!(DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&expanded), Err(DurableOwnedGroupDecisionError::EventTooLarge));

        let mut framed_options = PackEncodeOptions::default();
        framed_options.frame_size = 64;
        let framed_inner = pack_rt::encode_document(&DurableOwnedThreeMemberDecisionV1::__dsl_spec(), &decision.__dsl_to_record(), &framed_options).unwrap();
        let framed = crate::os_store::semio_format::wrap_binary(&envelope, &framed_inner);
        assert!(framed.len() <= DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);
        assert_eq!(DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&framed), Err(DurableOwnedGroupDecisionError::NonCanonical));
    }
}
