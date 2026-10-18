//! 📋️ Outcome and journal receipt types: the unbound and bound one item outcomes, the prepared and verified outcomes, the journal receipt, the verified three edits, the journal advance, commit and sink and the durable group root. See [`super`].

use super::*;

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_os_dsl::DslArtifact)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact(id = "store.unbound-one-item-outcome")]
pub struct DurableUnboundOneItemOutcomeV1 {
    pub schema: String,
    pub recovery_schema: String,
    pub operation: u64,
    pub base_generation: u64,
    pub base_revision: [u8; 32],
    pub base_applied_edit_count: u64,
    pub next_sequence_number: i32,
    #[dsl(base64)]
    pub next_clock_canonical_json: Vec<u8>,
    pub actor: semio_framework_value::SharedUtf8,
    #[dsl(base64)]
    pub edit_without_group_canonical_json: Vec<u8>,
    #[dsl(base64)]
    pub post_snapshot_pack: Vec<u8>,
}

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_os_dsl::DslArtifact)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact(id = "store.bound-one-item-outcome")]
pub struct DurableBoundOneItemOutcomeV1 {
    pub schema: String,
    pub recovery_schema: String,
    pub operation: u64,
    pub base_generation: u64,
    pub base_revision: [u8; 32],
    pub base_applied_edit_count: u64,
    pub next_sequence_number: i32,
    #[dsl(base64)]
    pub next_clock_canonical_json: Vec<u8>,
    pub actor: semio_framework_value::SharedUtf8,
    pub group_id: String,
    #[dsl(base64)]
    pub edit_canonical_json: Vec<u8>,
    #[dsl(base64)]
    pub post_snapshot_pack: Vec<u8>,
    pub edit_digest: [u8; 32],
    pub post_generation: u64,
    pub post_revision: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DurableStorePreparedOutcomeV1 {
    pub recovery_schema: String,
    pub pack: Vec<u8>,
    pub sha256: String,
}

pub struct DurableStoreVerifiedOutcomeV1<P, Mutation> {
    pub authority: Arc<ArtifactStoreOneItemLiveAuthority>,
    pub edit: Box<Edit<Mutation>>,
    pub post_snapshot: Arc<P>,
}

/// 🧾️ Trusted durable-boundary witness returned only after the decision transaction is synced.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DurableOwnedGroupJournalReceiptV1 {
    pub anchor_sha256: String,
    pub decision_sha256: String,
    pub transaction_id: u64,
    pub segment_index: u64,
}

/// 🔎️ Immutable typed edits projected only after Store verifies every bound fixed-three outcome.
pub struct DurableOwnedGroupVerifiedThreeEditsV1<ParentMutation, DrawingMutation, ValueMutation> {
    pub decision_sha256: String,
    pub anchor_sha256: String,
    pub document: semio_framework_artifact_reference::ArtifactRef,
    pub parent: Edit<ParentMutation>,
    pub drawing: Edit<DrawingMutation>,
    pub value: Edit<ValueMutation>,
}

impl<ParentMutation, DrawingMutation, ValueMutation> DurableOwnedGroupVerifiedThreeEditsV1<ParentMutation, DrawingMutation, ValueMutation> {
    pub fn decision_sha256(&self) -> &str {
        &self.decision_sha256
    }

    pub fn anchor_sha256(&self) -> &str {
        &self.anchor_sha256
    }

    pub fn document(&self) -> &semio_framework_artifact_reference::ArtifactRef {
        &self.document
    }

    pub fn parent(&self) -> &Edit<ParentMutation> {
        &self.parent
    }

    pub fn drawing(&self) -> &Edit<DrawingMutation> {
        &self.drawing
    }

    pub fn value(&self) -> &Edit<ValueMutation> {
        &self.value
    }
}

/// 🧭️ One retained journal turn. `Rejected` proves that no mailbox/WAL transfer began,
/// while `Absent` proves an accepted cancellation left no durable record. An error remains an
/// uncertain external state and must be resolved by the same retained owner.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DurableOwnedGroupJournalAdvanceV1 {
    Pending,
    Absent,
    Rejected(String),
    Committed(DurableOwnedGroupJournalReceiptV1),
}

/// 🧯️ Retained, cancellable owner for one durable decision attempt and its recovery resolution.
pub trait DurableOwnedGroupJournalCommitV1: semio_framework_value::ErasedSnapshotRetirement {
    fn advance(&mut self, grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant) -> Result<DurableOwnedGroupJournalAdvanceV1, String>;
    fn cancel(&mut self);
    fn begin_close(&mut self);
    fn retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError>;
}

/// 🎟️ Closes the retained journal body and its original Box allocation in separate admitted turns.
pub fn close_journal_owner(owner: &mut Option<Box<dyn DurableOwnedGroupJournalCommitV1>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
    use semio_framework_value::retained_clone::RetainedCloneStep;
    if owner.is_none() { return Ok(RetainedCloneStep::Complete(Default::default())); }
    let demand = semio_framework_value::factory_ticket_demands(owner.as_ref().unwrap(), grant.maximum_copy_bytes)?;
    if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
    semio_framework_value::close_factory_ticket(owner, grant)
}

/// 🗄️ Kernel-owned journal port implemented by a storage owner with an exclusive writer permit.
/// Construction is an infallible, non-I/O owner transfer; every fallible or externally visible
/// journal action begins in the returned retained operation's first `advance` turn.
pub trait DurableOwnedGroupJournalSinkV1: Send {
    fn begin_commit(&mut self, decision_pack: Vec<u8>, decision_sha256: String) -> Box<dyn DurableOwnedGroupJournalCommitV1>;
}

pub struct ArtifactStoreDurableGroupRootV1<P, Mutation> {
    pub visibility: Arc<semio_framework_os_vcs::ArtifactGroupVisibility>,
    pub current: Option<Arc<P>>,
    pub generation: u64,
    pub content_revision: [u8; 32],
    pub applied_edit_ids: semio_framework_os_vcs::HistoryPageStack<String>,
    pub redo_edit_ids: semio_framework_os_vcs::HistoryPageStack<String>,
    pub last_projection_cause: Option<semio_framework_os_store_space_schema::ArtifactProjectionCause>,
    pub edit_sequence: i32,
    pub clock: HybridLogicalTimestamp,
    pub revision_accumulator: Option<CursorRevisionAccumulator>,
    pub tail_undo_cache: Option<(String, Arc<P>)>,
    pub authority: Option<Arc<ArtifactStoreOneItemLiveAuthority>>,
    pub displaced_reservation: Option<semio_framework_os_store_read::ArtifactStoreDisplacedOwnerReservation>,
    pub retirement: Option<Box<StagedRootRetirement<P, Mutation>>>,
    pub adopted: bool,
}
