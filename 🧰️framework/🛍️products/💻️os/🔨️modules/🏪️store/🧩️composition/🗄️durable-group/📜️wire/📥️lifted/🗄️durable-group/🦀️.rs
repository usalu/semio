//! 📥️ Items of `durable_group` that belong to `os-store-durable-group-wire` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🧩️composition/🗄️durable-group/🦀️.rs`.
use semio_framework_os_store_ledger::CursorRevisionAccumulator;
use pack::snapshot::{ArtifactDsl,ArtifactPack};
use semio_framework_value::FromValue as ValueFromValue;
use semio_framework_value::ToValue as ValueToValue;
use semio_framework_os_spr::{Edit, HybridLogicalTimestamp};
use semio_framework_os_store_one_item_publication::{ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPrepared};
use semio_framework_os_store_envelope::owner::OwnerRef;
use semio_framework_os_pack::{PackDecodeOptions, PackEncodeOptions, PackError, PackVerificationLevel};
use semio_framework_diagnostic::TextError;
use semio_framework_os_pack::pack_rt;
use semio_framework_value_derive::{FromValue, ToValue};
use std::sync::Arc;
use std::mem::size_of;
use semio_framework_value::ErasedSnapshotRetirement as _;
pub const DURABLE_OWNED_GROUP_DECISION_SCHEMA_V1: &str = "semio.store.durable-owned-three-member-decision.v1";

pub const DURABLE_OWNED_GROUP_ANCHOR_SCHEMA_V1: &str = "semio.store.owned-three-member-anchor.v1";

pub const DURABLE_OWNED_GROUP_SHAPE_V1: &str = "parent-drawing-value";

pub const DURABLE_OWNED_GROUP_EVENT_MAX_BYTES: usize = 491_520;

pub const DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES: usize = 162_000;

pub const DURABLE_OWNED_GROUP_STRUCTURAL_MAX_BYTES: usize = 4_096;

pub const DURABLE_OWNED_GROUP_ID_MAX_BYTES: usize = 256;

pub const DURABLE_OWNED_GROUP_UNBOUND_OUTCOME_SCHEMA_V1: &str = "semio.store.unbound-one-item-outcome.v1";

pub const DURABLE_OWNED_GROUP_BOUND_OUTCOME_SCHEMA_V1: &str = "semio.store.bound-one-item-outcome.v1";

pub const DURABLE_OWNED_GROUP_JSON_MAX_DEPTH: usize = 32;

pub const DURABLE_OWNED_GROUP_JSON_MAX_ITEMS: usize = 65_536;

pub const PARENT_ROLE: &str = "parent";

pub const DRAWING_ROLE: &str = "drawing";

pub const VALUE_ROLE: &str = "value";

pub const PARENT_RECOVERY_SCHEMA: &str = "semio.store.one-item-outcome.gis-gismap-v1";

pub const DRAWING_RECOVERY_SCHEMA: &str = "semio.store.one-item-outcome.stdio-drawing-v1";

pub const VALUE_RECOVERY_SCHEMA: &str = "semio.store.one-item-outcome.stdio-value-v1";

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DurableOwnedGroupAnchorV1 {
    pub(crate) schema: String,
    pub(crate) parent: semio_framework_artifact_reference::ArtifactRef,
    pub(crate) shape: String,
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DurableOwnedGroupMemberV1 {
    pub(crate) role: String,
    pub(crate) reference: semio_framework_artifact_reference::ArtifactRef,
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

pub struct DurableStorePreparedOutcomesV1 {
    pub parent: DurableStorePreparedOutcomeV1,
    pub drawing: DurableStorePreparedOutcomeV1,
    pub value: DurableStorePreparedOutcomeV1,
}

impl semio_framework_dsl_record::BorrowedDslField for DurableOwnedGroupAnchorV1{
 const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Value;
}

impl semio_framework_dsl_record::BorrowedDslField for DurableOwnedGroupMemberV1{
 const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Value;
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue)]
#[value(rename_all = "camelCase")]
struct DurableOwnedThreeMemberUnsignedMemberV1 {
    role: String,
    reference: semio_framework_artifact_reference::ArtifactRef,
    owner: Option<OwnerRef>,
    expected_generation: u64,
    expected_revision: [u8; 32],
    recovery_schema: String,
    unbound_outcome_sha256: String,
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue)]
#[value(rename_all = "camelCase")]
pub struct DurableOwnedThreeMemberUnsignedV1 {
    schema: String,
    anchor: DurableOwnedGroupAnchorV1,
    parent: DurableOwnedThreeMemberUnsignedMemberV1,
    drawing: DurableOwnedThreeMemberUnsignedMemberV1,
    value: DurableOwnedThreeMemberUnsignedMemberV1,
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
    Retirement(semio_framework_value::ValueError),
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
            Self::Retirement(error) => std::fmt::Display::fmt(error, formatter),
        }
    }
}

impl std::error::Error for DurableOwnedGroupDecisionError {}

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_os_dsl::DslArtifact)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact(id = "store.unbound-one-item-outcome")]
pub struct DurableUnboundOneItemOutcomeV1 {
    schema: String,
    recovery_schema: String,
    operation: u64,
    base_generation: u64,
    base_revision: [u8; 32],
    base_applied_edit_count: u64,
    next_sequence_number: i32,
    #[dsl(base64)]
    next_clock_canonical_json: Vec<u8>,
    actor: semio_framework_value::SharedUtf8,
    #[dsl(base64)]
    edit_without_group_canonical_json: Vec<u8>,
    #[dsl(base64)]
    post_snapshot_pack: Vec<u8>,
}

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_os_dsl::DslArtifact)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact(id = "store.bound-one-item-outcome")]
pub struct DurableBoundOneItemOutcomeV1 {
    schema: String,
    recovery_schema: String,
    operation: u64,
    base_generation: u64,
    base_revision: [u8; 32],
    base_applied_edit_count: u64,
    next_sequence_number: i32,
    #[dsl(base64)]
    next_clock_canonical_json: Vec<u8>,
    actor: semio_framework_value::SharedUtf8,
    group_id: String,
    #[dsl(base64)]
    edit_canonical_json: Vec<u8>,
    #[dsl(base64)]
    post_snapshot_pack: Vec<u8>,
    edit_digest: [u8; 32],
    post_generation: u64,
    post_revision: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DurableStorePreparedOutcomeV1 {
    recovery_schema: String,
    pack: Vec<u8>,
    sha256: String,
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

pub struct StagedRootRetirement<P, Mutation> {
    pub edit: Option<semio_framework_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirement<Mutation>>,
    pub external_strings: [Option<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>; 2],
    pub cursor: Option<semio_framework_os_store_read::ArtifactStoreCursorRetirement>,
    pub applied: Option<semio_framework_os_store_read::ArtifactStoreStringVectorRetirement>,
    pub redo: Option<semio_framework_os_store_read::ArtifactStoreStringVectorRetirement>,
    pub revision: Option<semio_framework_os_store_read::ArtifactStoreRevisionAccumulatorRetirement>,
    pub report: Option<semio_framework_os_store_ledger::ArtifactStorePendingReportRetirement>,
    pub tail_id: Option<semio_framework_os_store_read::ArtifactStoreStringRetirement>,
    pub authority: Option<semio_framework_os_store_one_item_publication::lifted_canonical_edit::ArtifactStoreOneItemAuthorityRetirement>,
    pub snapshots: [Option<Arc<P>>; 2],
    pub active: Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
    pub factory: Option<Arc<dyn semio_framework_value::SnapshotRetirementFactory<P>>>,
    pub factory_close: Option<semio_framework_value::FactoryAuthority>,
    pub phase: u8,
}

impl<P, Mutation> StagedRootRetirement<P, Mutation> {
    pub fn new(factory: Arc<dyn semio_framework_value::SnapshotRetirementFactory<P>>) -> Self {
        Self { edit: None, external_strings: Default::default(), cursor: None, applied: None, redo: None, revision: None, report: None, tail_id: None, authority: None, snapshots: [None, None], active: None, factory: Some(factory), factory_close: None, phase: 0 }
    }

    fn inline_owner(&self) -> Option<&dyn semio_framework_value::ErasedSnapshotRetirement> {
        match self.phase {
            0 => self.cursor.as_ref().map(|owner| owner as _),
            1 => self.applied.as_ref().map(|owner| owner as _),
            2 => self.redo.as_ref().map(|owner| owner as _),
            3 => self.revision.as_ref().map(|owner| owner as _),
            4 => self.report.as_ref().map(|owner| owner as _),
            5 => self.tail_id.as_ref().map(|owner| owner as _),
            6 => self.authority.as_ref().map(|owner| owner as _),
            _ => None,
        }
    }

    fn inline_owner_mut(&mut self) -> Option<&mut dyn semio_framework_value::ErasedSnapshotRetirement> {
        match self.phase {
            0 => self.cursor.as_mut().map(|owner| owner as _),
            1 => self.applied.as_mut().map(|owner| owner as _),
            2 => self.redo.as_mut().map(|owner| owner as _),
            3 => self.revision.as_mut().map(|owner| owner as _),
            4 => self.report.as_mut().map(|owner| owner as _),
            5 => self.tail_id.as_mut().map(|owner| owner as _),
            6 => self.authority.as_mut().map(|owner| owner as _),
            _ => None,
        }
    }

    fn clear_inline(&mut self) {
        match self.phase {
            0 => { self.cursor = None; }
            1 => { self.applied = None; }
            2 => { self.redo = None; }
            3 => { self.revision = None; }
            4 => { self.report = None; }
            5 => { self.tail_id = None; }
            6 => { self.authority = None; }
            _ => {}
        }
    }
}

impl<P: Send + Sync, Mutation: Send + 'static> StagedRootRetirement<P, Mutation> {
    pub fn demands(&self, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
        let nested = |mut demand: RetirementDemand| { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "staged root child depth overflow"))?; Ok(demand) };
        if let Some(active) = self.active.as_ref() { return nested(semio_framework_os_store_read::artifact_retirement_box_demands(active, maximum_body_bytes)?); }
        if self.edit.is_some() { return semio_framework_os_store_read::retirement_owner_demands(&self.edit, maximum_body_bytes); }
        if self.external_strings.iter().any(Option::is_some) { return Ok(RetirementDemand { capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>(), depth: 2, ..Default::default() }); }
        if let Some(owner) = self.inline_owner() { return nested(RetirementDemand { copy_bytes: owner.next_copy_byte_demand()?, capacity_bytes: owner.next_capacity_byte_demand(maximum_body_bytes)?, release_bytes: owner.next_release_byte_demand()?, depth: owner.next_depth_demand()? }); }
        if (7..9).contains(&self.phase) {
            if let Some(snapshot) = self.snapshots[usize::from(self.phase - 7)].as_ref() { return Ok(RetirementDemand { capacity_bytes: self.factory.as_ref().expect("staged root retains its snapshot factory").retirement_birth_bytes(snapshot), depth: 2, ..Default::default() }); }
        }
        if self.phase >= 9 && self.factory.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        if let Some(factory) = self.factory_close.as_ref() { return nested(factory.demands(maximum_body_bytes)?); }
        Ok(RetirementDemand { depth: usize::from(!self.terminal_is_empty()), ..Default::default() })
    }
}

impl<P: Send + Sync, Mutation: Send + 'static> semio_framework_value::ErasedSnapshotRetirement for StagedRootRetirement<P, Mutation> {
    fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.demands(maximum_body_bytes)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.demands(0)?.depth) }
    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "staged root exceeds admitted depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if self.active.is_some() { return semio_framework_os_store_read::artifact_retirement_box_close_step(&mut self.active, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if self.edit.is_some() { return semio_framework_os_store_read::retirement_owner_close(&mut self.edit, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(slot)=self.external_strings.iter_mut().find(|slot|slot.is_some()){let original=slot.take().unwrap();return match semio_framework_value::retirement::admit_owned_retirement(original,child){Ok((owner,receipt))=>{self.active=Some(owner);Ok(RetainedCloneStep::Progress(receipt))},Err((error,original))=>{*slot=Some(original);Err(error)}};}
        if let Some(owner) = self.inline_owner_mut() {
            if !owner.terminal_is_empty() {
                let step = owner.close_step(child)?;
                return semio_framework_value::retained_clone::admit_retained_clone_close(child, step, owner.terminal_is_empty(), "staged inline owner").map(|step| RetainedCloneStep::Progress(step.progress()));
            }
            self.clear_inline();
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if (7..9).contains(&self.phase) {
            let index = usize::from(self.phase - 7);
            if let Some(snapshot) = self.snapshots[index].take() {
                return match self.factory.as_ref().expect("staged root retains its snapshot factory").retire(snapshot, child) {
                    Ok((owner, progress)) => { self.active = Some(owner); semio_framework_value::retained_clone::admit_retained_clone_progress(child, progress, "staged snapshot constructor")?; Ok(RetainedCloneStep::Progress(progress)) },
                    Err((error, snapshot)) => { self.snapshots[index] = Some(snapshot); Err(error) },
                };
            }
        }
        if self.phase < 9 { self.phase += 1; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty })); }
        if let Some(factory) = self.factory.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            self.factory_close = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), ..empty }));
        }
        if let Some(factory) = self.factory_close.as_mut() {
            let step = factory.step(child)?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "staged snapshot factory")?;
            if factory.terminal_is_empty() { self.factory_close = None; }
            return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) });
        }
        Ok(RetainedCloneStep::Complete(empty))
    }
    fn terminal_is_empty(&self) -> bool { self.edit.is_none() && self.external_strings.iter().all(Option::is_none) && self.cursor.is_none() && self.applied.is_none() && self.redo.is_none() && self.revision.is_none() && self.report.is_none() && self.tail_id.is_none() && self.authority.is_none() && self.snapshots.iter().all(Option::is_none) && self.active.is_none() && self.factory.is_none() && self.factory_close.is_none() }
}

impl<P, Mutation> Drop for StagedRootRetirement<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.edit.is_none() && self.external_strings.iter().all(Option::is_none) && self.cursor.is_none() && self.applied.is_none() && self.redo.is_none() && self.revision.is_none() && self.report.is_none() && self.tail_id.is_none() && self.authority.is_none() && self.snapshots.iter().all(Option::is_none) && self.active.is_none() && self.factory.is_none() && self.factory_close.is_none()), "staged root retirement dropped before all exact outgoing owners were closed");
    }
}

pub fn retained_string_stack(source: impl IntoIterator<Item = String>, appended: Option<&str>) -> Result<semio_framework_os_vcs::HistoryPageStack<String>, DurableOwnedGroupDecisionError> {
    let mut values = semio_framework_os_vcs::HistoryPageStack::try_new().map_err(|error| DurableOwnedGroupDecisionError::Codec(error.into()))?;
    for value in source {
        values.try_push(value).map_err(|_| DurableOwnedGroupDecisionError::Codec("history catalog address space exhausted".into()))?;
    }
    if let Some(value) = appended {
        values.try_push(value.to_string()).map_err(|_| DurableOwnedGroupDecisionError::Codec("history catalog address space exhausted".into()))?;
    }
    Ok(values)
}

pub fn retained_revision_stack(source: impl IntoIterator<Item = semio_framework_os_store_ledger::CursorRevisionRecord>) -> Result<semio_framework_os_vcs::HistoryPageStack<semio_framework_os_store_ledger::CursorRevisionRecord>, DurableOwnedGroupDecisionError> {
    let mut values = semio_framework_os_vcs::HistoryPageStack::try_new().map_err(|error| DurableOwnedGroupDecisionError::Codec(error.into()))?;
    for value in source {
        values.try_push(value).map_err(|_| DurableOwnedGroupDecisionError::Codec("history catalog address space exhausted".into()))?;
    }
    Ok(values)
}

pub fn retain_displaced_owner(store: &mut semio_framework_os_store_read::ArtifactStoreDisplacedRetirements, reservation: &mut semio_framework_os_store_read::ArtifactStoreDisplacedOwnerReservation, owner: Box<dyn semio_framework_value::ErasedSnapshotRetirement>) {
    if let Err(owner) = store.push_owner_reserved(reservation, owner) {
        store.push_reserved(owner);
    }
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

/// ⏭️ One bounded lower restoration turn; it deliberately exposes no receipt or durable approval acknowledgement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DurableOwnedMapRecoveryAdvanceV1 {
    Progress(DurableOwnedThreeStoreCommitPhaseV1),
    Blocked,
    Fault(DurableOwnedGroupDecisionError),
    AwaitingAcknowledgement,
    Complete,
}

/// 🎫 One exact unbound Store admission. No group identity can enter before the private bind.
pub struct DurableOwnedMapMemberAdmissionV1<Mutation> {
    pub operation: semio_framework_job::OperationId,
    pub expected_generation: u64,
    pub expected_revision: [u8; 32],
    pub actor: semio_framework_value::SharedUtf8,
    pub mutation: Option<Mutation>,
}

impl<Mutation> DurableOwnedMapMemberAdmissionV1<Mutation> {
    pub fn new(operation: semio_framework_job::OperationId, expected_generation: u64, expected_revision: [u8; 32], actor: semio_framework_value::SharedUtf8, mutation: Mutation) -> Self {
        Self { operation, expected_generation, expected_revision, actor, mutation: Some(mutation) }
    }
}

/// 🚫 Terminal pre-journal assembly outcome. Every variant still returns all three Stores and the sink.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DurableOwnedThreeStoreMapAssemblyFailureV1 {
    Cancelled,
    Admission { role: &'static str, reason: String },
    Preparation { role: &'static str, reason: String },
    Binding(DurableOwnedGroupDecisionError),
}

/// 🧭 Observable retained assembly phase. `Mounted` contains the existing fixed host and no loose Store.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DurableOwnedThreeStoreMapAssemblyPhaseV1 {
    AdmittingParent,
    AdmittingDrawing,
    AdmittingValue,
    PreparingParent,
    PreparingDrawing,
    PreparingValue,
    ClosingValue,
    ClosingDrawing,
    ClosingParent,
    Binding,
    Mounted,
    Terminal,
    Empty,
}

/// ⏭️ One bounded assembly turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DurableOwnedThreeStoreMapAssemblyAdvanceV1 {
    Progress(DurableOwnedThreeStoreMapAssemblyPhaseV1),
    Blocked,
    Mounted,
    Terminal,
}

pub fn close_assembly_publication<P: Send + Sync + 'static, Mutation: Send + 'static>(publication: &mut Option<semio_framework_os_store_one_item_batch::lifted_one_item_publication::ArtifactStoreBatchPublication<P, Mutation>>, grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant) -> Result<bool, DurableOwnedGroupDecisionError> {
    use semio_framework_value::retained_clone::{RetainedCloneStep, admit_retained_clone_close};
    let Some(owner) = publication.as_mut() else { return Ok(true) };
    let grant = grant.retained_grant();
    let codec = DurableOwnedGroupDecisionError::Retirement;
    let step = owner.close_step(grant).map_err(codec)?;
    match admit_retained_clone_close(grant, step, owner.terminal_is_empty(), "durable group publication").map_err(codec)? {
        RetainedCloneStep::Complete(_) => {
            drop(publication.take());
            Ok(true)
        }
        RetainedCloneStep::Progress(_) => Ok(false),
    }
}

pub fn take_assembly_prepared<P, Mutation>(publication: &mut Option<semio_framework_os_store_one_item_batch::lifted_one_item_publication::ArtifactStoreBatchPublication<P, Mutation>>) -> Result<ArtifactStoreOneItemPrepared<P, Mutation>, DurableOwnedGroupDecisionError> {
    let publication = publication.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    if publication.phase() != semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPublicationPhase::Publishing {
        return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
    }
    publication.take_staged_prepared().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)
}

impl DurableUnboundOneItemOutcomeV1 {
    pub fn decode_canonical_pack(bytes: &[u8]) -> Result<Self, DurableOwnedGroupDecisionError> {
        if bytes.len() > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES {
            return Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge);
        }
        let options = PackDecodeOptions {
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
        };
        let (envelope, inner) = semio_framework_os_semio::unwrap_binary(bytes).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        if !envelope.matches_identity(<Self as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Pack, 1) {
            return Err(DurableOwnedGroupDecisionError::InvalidSchema);
        }
        let file = ::semio_framework_async::poll::resolve_ready(pack::format::PackFile::open_manifest(inner.as_slice(), &options.limits, options.verification)).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        let manifest = file.manifest().ok_or_else(|| DurableOwnedGroupDecisionError::Codec("unbound outcome manifest is absent".into()))?;
        if manifest.uncompressed_body_len > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64 {
            return Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge);
        }
        if manifest.doc_frame_count != 1 || manifest.chunk_count != 0 || file.chunk_count() != 0 || manifest.field_count != 11 || manifest.schema_hash != semio_framework_os_pack::schema_hash(&Self::__dsl_spec()) {
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
    pub fn from_prepared<P, Mutation>(recovery_schema: &str, prepared: &ArtifactStoreOneItemPrepared<P, Mutation>) -> Result<Self, DurableOwnedGroupDecisionError>
    where
        P: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot,
        Mutation: ValueToValue + ValueFromValue,
    {
        let authority = &prepared.authority();
        if !valid_identity(recovery_schema) || authority.group_id().is_some() || !valid_edit_identities(prepared.edit()) {
            return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
        }
        authority.validate_prepared(prepared).map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?;
        let post_snapshot_pack = prepared.post_snapshot().encode_pack();
        if post_snapshot_pack.len() > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES {
            return Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge);
        }
        let outcome = DurableUnboundOneItemOutcomeV1 {
            schema: DURABLE_OWNED_GROUP_UNBOUND_OUTCOME_SCHEMA_V1.into(),
            recovery_schema: recovery_schema.into(),
            operation: authority.operation().0,
            base_generation: authority.generation().0,
            base_revision: authority.base_revision(),
            base_applied_edit_count: authority.base_applied_edit_count().try_into().map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?,
            next_sequence_number: authority.next_sequence_number(),
            next_clock_canonical_json: semio_framework_pack_json::to_json_string(&authority.next_clock()).into_bytes(),
            actor: authority.actor_utf8().clone(),
            edit_without_group_canonical_json: semio_framework_pack_json::to_json_string(prepared.edit()).into_bytes(),
            post_snapshot_pack,
        };
        let pack = outcome.encode_pack_with(&PackEncodeOptions::default()).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        if pack.len() > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES {
            return Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge);
        }
        let owned = Self { recovery_schema: recovery_schema.into(), sha256: semio_framework_hash::sha256_hex(&pack), pack };
        let verified = owned.verify_inverse::<P, Mutation>()?;
        if verified.authority.operation() != authority.operation()
            || verified.authority.generation() != authority.generation()
            || verified.authority.base_revision() != authority.base_revision()
            || verified.authority.base_applied_edit_count() != authority.base_applied_edit_count()
            || verified.authority.next_sequence_number() != authority.next_sequence_number()
            || verified.authority.next_clock() != authority.next_clock()
            || verified.authority.actor() != authority.actor()
            || semio_framework_value::ToValue::to_value(verified.edit.as_ref()) != semio_framework_value::ToValue::to_value(prepared.edit())
            || verified.post_snapshot.encode_pack() != prepared.post_snapshot().encode_pack()
        {
            return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
        }
        Ok(owned)
    }

    pub fn verify_inverse<P, Mutation>(&self) -> Result<DurableStoreVerifiedOutcomeV1<P, Mutation>, DurableOwnedGroupDecisionError>
    where
        P: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot,
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
            limits: semio_framework_os_pack::PackLimits {
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
        let authority = Arc::new(ArtifactStoreOneItemLiveAuthority::mint(
            semio_framework_job::OperationId(outcome.operation),
            semio_framework_job::Generation(outcome.base_generation),
            outcome.base_revision,
            base_applied_edit_count,
            outcome.next_sequence_number,
            next_clock,
            outcome.actor,
            edit.line.clone(),
            None,
            None,
        ));
        authority.validate_semantic_edit(&edit).map_err(|_| DurableOwnedGroupDecisionError::InvalidOutcome)?;
        Ok(DurableStoreVerifiedOutcomeV1 { authority, edit: Box::new(edit), post_snapshot: Arc::new(post_snapshot) })
    }
}

pub fn parse_canonical_json_value<T>(bytes: &[u8]) -> Result<T, DurableOwnedGroupDecisionError>
where
    T: ValueToValue + ValueFromValue,
{
    validate_json_budget(bytes)?;
    let json = std::str::from_utf8(bytes).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
    let value = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
    if semio_framework_pack_json::to_json_string(&value).as_bytes() != bytes {
        return Err(DurableOwnedGroupDecisionError::NonCanonical);
    }
    Ok(value)
}

pub fn validate_json_budget(bytes: &[u8]) -> Result<(), DurableOwnedGroupDecisionError> {
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

pub fn valid_edit_identities<Mutation>(edit: &Edit<Mutation>) -> bool {
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

impl DurableBoundOneItemOutcomeV1 {
    pub fn decode_canonical_pack(bytes: &[u8]) -> Result<Self, DurableOwnedGroupDecisionError> {
        if bytes.len() > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES {
            return Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge);
        }
        let options = PackDecodeOptions {
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
        };
        let (envelope, inner) = semio_framework_os_semio::unwrap_binary(bytes).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        if !envelope.matches_identity(<Self as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Pack, 1) {
            return Err(DurableOwnedGroupDecisionError::InvalidSchema);
        }
        let file = ::semio_framework_async::poll::resolve_ready(pack::format::PackFile::open_manifest(inner.as_slice(), &options.limits, options.verification)).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))?;
        let manifest = file.manifest().ok_or_else(|| DurableOwnedGroupDecisionError::Codec("bound outcome manifest is absent".into()))?;
        if manifest.uncompressed_body_len > DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES as u64 {
            return Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge);
        }
        if manifest.doc_frame_count != 1 || manifest.chunk_count != 0 || file.chunk_count() != 0 || manifest.field_count != 15 || manifest.schema_hash != semio_framework_os_pack::schema_hash(&Self::__dsl_spec()) {
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

    pub fn unbound_pack<Mutation>(&self) -> Result<Vec<u8>, DurableOwnedGroupDecisionError>
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
            edit_without_group_canonical_json: semio_framework_pack_json::to_json_string(&edit).into_bytes(),
            post_snapshot_pack: self.post_snapshot_pack.clone(),
        };
        outcome.encode_pack_with(&PackEncodeOptions::default()).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.to_string()))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DurableMemberRecoveryFrontier {
    Base,
    Post,
}

impl ArtifactDsl for DurableUnboundOneItemOutcomeV1 {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;

    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }

    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = semio_framework_os_semio::split_text_preamble(text).map(|(_, body)| body).unwrap_or(text);
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }

    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = semio_framework_os_semio::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Dsl, 1).expect("valid unbound outcome envelope");
        semio_framework_os_semio::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for DurableUnboundOneItemOutcomeV1 {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let inner = pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = semio_framework_os_semio::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Pack, 1).map_err(|error| PackError::from(error.into_value_error()))?;
        Ok(semio_framework_os_semio::wrap_binary(&envelope, &inner))
    }

    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = semio_framework_os_semio::unwrap_binary(bytes).map_err(|error| PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Pack, 1) {
            return Err(PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "unbound outcome pack envelope mismatch")));
        }
        let (record, _) = pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(semio_framework_os_store_text_io::text_error_to_pack_error)
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl semio_framework_os_store_text_io::ArtifactStoreSnapshot for DurableUnboundOneItemOutcomeV1 {

}

impl ArtifactDsl for DurableBoundOneItemOutcomeV1 {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;

    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }

    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = semio_framework_os_semio::split_text_preamble(text).map(|(_, body)| body).unwrap_or(text);
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }

    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = semio_framework_os_semio::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Dsl, 1).expect("valid bound outcome envelope");
        semio_framework_os_semio::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for DurableBoundOneItemOutcomeV1 {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let inner = pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = semio_framework_os_semio::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Pack, 1).map_err(|error| PackError::from(error.into_value_error()))?;
        Ok(semio_framework_os_semio::wrap_binary(&envelope, &inner))
    }

    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = semio_framework_os_semio::unwrap_binary(bytes).map_err(|error| PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Pack, 1) {
            return Err(PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "bound outcome pack envelope mismatch")));
        }
        let (record, _) = pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(semio_framework_os_store_text_io::text_error_to_pack_error)
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl semio_framework_os_store_text_io::ArtifactStoreSnapshot for DurableBoundOneItemOutcomeV1 {

}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DurableOwnedGroupMapHandlesV1 {
    pub(crate) drawing: semio_framework_artifact_reference::ArtifactRef,
    pub(crate) value: semio_framework_artifact_reference::ArtifactRef,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DurableOwnedGroupMapFrontiersV1 {
    pub(crate) parent_generation: u64,
    pub(crate) parent_revision: [u8; 32],
    pub(crate) drawing_generation: u64,
    pub(crate) drawing_revision: [u8; 32],
    pub(crate) value_generation: u64,
    pub(crate) value_revision: [u8; 32],
}

pub fn valid_hash(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

pub fn valid_identity(value: &str) -> bool {
    !value.is_empty() && value.len() <= DURABLE_OWNED_GROUP_ID_MAX_BYTES && !value.chars().any(char::is_control)
}

pub fn valid_reference(reference: &semio_framework_artifact_reference::ArtifactRef) -> bool {
    valid_identity(&reference.artifact_id) && valid_identity(&reference.dialect.artifact_kind) && valid_identity(&reference.dialect.standard) && valid_identity(&reference.dialect.subset)
}

pub fn unsigned_member(member: &DurableOwnedGroupMemberV1) -> DurableOwnedThreeMemberUnsignedMemberV1 {
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

pub fn member_identity_matches(member: &DurableOwnedGroupMemberV1, role: &str, recovery_schema: &str, reference_uri: &str, anchor: &DurableOwnedGroupAnchorV1) -> bool {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

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

pub fn member_matches(member: &DurableOwnedGroupMemberV1, role: &str, recovery_schema: &str, reference_uri: &str, anchor: &DurableOwnedGroupAnchorV1) -> bool {
    member_identity_matches(member, role, recovery_schema, reference_uri, anchor) && member.expected_generation.checked_add(1) == Some(member.post_generation) && member.post_revision != member.expected_revision
}

#[cfg(feature = "durable-group-testing")]
fn durable_group_test_revision(bytes: &[u8]) -> [u8; 32] {
    let digest = semio_framework_hash::sha256_hex(bytes);
    std::array::from_fn(|index| u8::from_str_radix(&digest[index * 2..index * 2 + 2], 16).expect("SHA-256 hexadecimal byte"))
}

#[cfg(feature = "durable-group-testing")]
pub fn durable_group_test_outcome<P, Mutation>(recovery_schema: &str, ordinal: u64, edit: Edit<Mutation>, post_snapshot: P) -> Result<DurableStorePreparedOutcomeV1, DurableOwnedGroupDecisionError>
where
    P: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot,
    Mutation: ValueToValue + ValueFromValue,
{
    let actor = edit.actor.clone().ok_or(DurableOwnedGroupDecisionError::InvalidIdentity)?;
    let meta = edit.mutation_meta.first().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    let authority = Arc::new(ArtifactStoreOneItemLiveAuthority::mint(
        semio_framework_job::OperationId(10_000 + ordinal),
        semio_framework_job::Generation(ordinal),
        durable_group_test_revision(format!("durable-group-test-base-{ordinal}").as_bytes()),
        0,
        edit.sequence_number,
        meta.timestamp,
        actor,
        edit.line.clone(),
        meta.group_id.clone(),
        None,
    ));
    let prepared = authority.prepare_one_item(edit, Arc::new(post_snapshot)).map_err(DurableOwnedGroupDecisionError::Codec)?;
    DurableStorePreparedOutcomeV1::from_prepared(recovery_schema, &prepared)
}

#[cfg(feature = "durable-group-testing")]
pub fn durable_group_test_member(role: &str, reference: semio_framework_artifact_reference::ArtifactRef, owner: Option<OwnerRef>, outcome: &DurableStorePreparedOutcomeV1) -> Result<DurableOwnedGroupMemberV1, DurableOwnedGroupDecisionError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    let unbound = DurableUnboundOneItemOutcomeV1::decode_canonical_pack(&outcome.pack)?;
    let post_revision = durable_group_test_revision(format!("{}:{}:{}", role, reference.to_uri(), outcome.sha256).as_bytes());
    Ok(DurableOwnedGroupMemberV1 {
        role: role.into(),
        reference,
        owner,
        expected_generation: unbound.base_generation,
        expected_revision: unbound.base_revision,
        recovery_schema: outcome.recovery_schema.clone(),
        recovery_pack: Vec::new(),
        recovery_pack_sha256: String::new(),
        unbound_outcome_sha256: outcome.sha256.clone(),
        post_generation: unbound.base_generation.checked_add(1).ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?,
        post_revision,
    })
}

#[cfg(feature = "durable-group-testing")]
pub fn durable_group_test_bind_member<Mutation>(member: &mut DurableOwnedGroupMemberV1, outcome: &DurableStorePreparedOutcomeV1, decision_sha256: &str) -> Result<(), DurableOwnedGroupDecisionError>
where
    Mutation: ValueToValue + ValueFromValue,
{
    let unbound = DurableUnboundOneItemOutcomeV1::decode_canonical_pack(&outcome.pack)?;
    let mut edit: Edit<Mutation> = parse_canonical_json_value(&unbound.edit_without_group_canonical_json)?;
    if edit.mutation_meta.len() != 1 {
        return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
    }
    let meta = edit.mutation_meta.first_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    if meta.group_id.replace(decision_sha256.into()).is_some() {
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
        group_id: decision_sha256.into(),
        edit_canonical_json: semio_framework_pack_json::to_json_string(&edit).into_bytes(),
        post_snapshot_pack: unbound.post_snapshot_pack,
        edit_digest: CursorRevisionAccumulator::edit_digest(&edit),
        post_generation: member.post_generation,
        post_revision: member.post_revision,
    };
    member.recovery_pack = bound.encode_pack();
    member.recovery_pack_sha256 = semio_framework_hash::sha256_hex(&member.recovery_pack);
    Ok(())
}
