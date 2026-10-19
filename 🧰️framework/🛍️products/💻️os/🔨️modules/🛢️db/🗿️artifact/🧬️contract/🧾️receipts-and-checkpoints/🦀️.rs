//! 🧾️ Receipts and checkpoints: the command receipt, the checkpoint publication snapshot, the durable group journal append, decision and recovered checkpoint and the committed recovery state, advance and terminals. See [`super`].

use super::*;

//#region 🔖️Ids
//#region 🔖️Receipt
/// 🧾️ What `ArtifactEngine::submit` returns: the committed batch's identity, the document's
/// new `Frontier`, the durability actually requested, any detected conflicts, and the post-commit
/// state's content hash. Mirrors the `db` facade's frozen `CommandReceipt` shape, except `frontier`
/// is `Frontier` (this crate's own internal currency) rather than the facade's
/// `protocol::ArtifactId`-keyed twin — the facade converts via `to_core_document_id`'s inverse at
/// its own boundary (see module doc's bridge note).
#[derive(Clone, Debug, PartialEq)]
pub struct CommandReceipt {
    pub command_id: protocol::MutationId,
    pub frontier: Frontier,
    pub durability: DurabilityClass,
    pub conflicts: Vec<ConflictRecord>,
    pub state_hash: Option<ContentHash>,
    /// 📨️ Every `protocol::MutationMessage` the outcome step graded this batch's
    /// `db_conflict::ConflictRecord`s into (contract §C9) — present even on an accepted-but-degraded
    /// commit (`options.policy` let a `Warning`-or-below worst level through), empty on a clean one.
    pub messages: Vec<protocol::MutationMessage>,
}

/// 📍️ One actor-serialized checkpoint-publication observation, including the exact
/// committed tip identity omitted from the generic database frontier.
#[derive(Clone, Debug, PartialEq)]
pub struct CheckpointPublicationSnapshot {
    pub frontier: Frontier,
    pub head_edit_id: Option<protocol::MutationId>,
}

/// 🧾️ Actor-owned outcome for one Store-admitted durable fixed-three decision event.
pub enum ArtifactDurableGroupJournalAppendV1 {
    Absent,
    Rejected(DbError),
    Committed(store::durable_group::DurableOwnedGroupJournalReceiptV1),
}

/// 🔏️ One fixed-three decision proven to occupy exactly one committed WAL transaction body.
pub struct ArtifactCommittedDurableGroupDecisionV1 {
    pub document: ArtifactId,
    pub transaction_id: u64,
    pub segment_index: u64,
    pub record: store::durable_group::DurableOwnedGroupJournalRecordV1,
}

/// 📍️ Read-only actor projection recovered from one sole committed fixed-three Event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactDurableGroupRecoveredCheckpointV1 {
    pub receipt: store::durable_group::DurableOwnedGroupJournalReceiptV1,
    pub head_edit_id: protocol::MutationId,
    pub chain_hash: [u8; 32],
    pub record: store::durable_group::DurableOwnedGroupJournalRecordV1,
    pub(super) already_applied: bool,
}

impl ArtifactDurableGroupRecoveredCheckpointV1 {
    pub fn receipt(&self) -> &store::durable_group::DurableOwnedGroupJournalReceiptV1 {
        &self.receipt
    }

    pub fn head_edit_id(&self) -> &protocol::MutationId {
        &self.head_edit_id
    }

    pub fn chain_hash(&self) -> [u8; 32] {
        self.chain_hash
    }

    pub fn already_applied(&self) -> bool {
        self.already_applied
    }

    /// 🔎️ Projects immutable typed edits only after revalidating the DB-consumed committed record.
    pub fn verify_fixed_three_edits<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>(
        &self,
    ) -> Result<store::durable_group::DurableOwnedGroupVerifiedThreeEditsV1<ParentMutation, DrawingMutation, ValueMutation>, store::durable_group::DurableOwnedGroupDecisionError>
    where
        ParentP: pack::snapshot::ArtifactPack,
        ParentMutation: semio_framework_value::ToValue + semio_framework_value::FromValue,
        DrawingP: pack::snapshot::ArtifactPack,
        DrawingMutation: semio_framework_value::ToValue + semio_framework_value::FromValue,
        ValueP: pack::snapshot::ArtifactPack,
        ValueMutation: semio_framework_value::ToValue + semio_framework_value::FromValue,
    {
        self.record.verify_fixed_three_edits::<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>()
    }
}

impl ArtifactCommittedDurableGroupDecisionV1 {
    pub fn document(&self) -> &ArtifactId {
        &self.document
    }

    pub fn transaction_id(&self) -> u64 {
        self.transaction_id
    }

    pub fn segment_index(&self) -> u64 {
        self.segment_index
    }

    pub fn receipt(&self) -> store::durable_group::DurableOwnedGroupJournalReceiptV1 {
        store::durable_group::DurableOwnedGroupJournalReceiptV1 {
            anchor_sha256: self.record.anchor_sha256().to_string(),
            decision_sha256: self.record.decision_sha256().to_string(),
            transaction_id: self.transaction_id,
            segment_index: self.segment_index,
        }
    }

    pub(super) fn recovered_checkpoint(&self) -> ArtifactDurableGroupRecoveredCheckpointV1 {
        ArtifactDurableGroupRecoveredCheckpointV1 {
            receipt: self.receipt(),
            head_edit_id: protocol::MutationId(self.record.parent_edit_id().to_string()),
            chain_hash: self.record.parent_post_revision(),
            record: self.record.clone(),
            already_applied: false,
        }
    }

    pub fn into_store_owned_recovery<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>(
        self,
        admission: store::durable_group::DurableOwnedMapRecoveryAdmissionV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>,
    ) -> ArtifactCommittedDurableGroupRecoveryV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
    where
        ParentP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
        DrawingP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
        ValueP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
    {
        ArtifactCommittedDurableGroupRecoveryV1 { state: std::mem::ManuallyDrop::new(Some(ArtifactCommittedDurableGroupRecoveryStateV1::Admitting { witness: self, admission, last_error: None })) }
    }
}

pub(super) enum ArtifactCommittedDurableGroupRecoveryStateV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ParentMutation: store::Mutation<ParentP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingMutation: store::Mutation<DrawingP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueMutation: store::Mutation<ValueP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
{
    Admitting {
        witness: ArtifactCommittedDurableGroupDecisionV1,
        admission: store::durable_group::DurableOwnedMapRecoveryAdmissionV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>,
        last_error: Option<store::durable_group::DurableOwnedGroupDecisionError>,
    },
    Recovering {
        document: ArtifactId,
        receipt: store::durable_group::DurableOwnedGroupJournalReceiptV1,
        host: store::durable_group::DurableOwnedMapRecoveryHostV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>,
    },
    Complete {
        document: ArtifactId,
        receipt: store::durable_group::DurableOwnedGroupJournalReceiptV1,
        already_applied: bool,
        owners: store::durable_group::DurableOwnedMapRecoveryOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>,
    },
}

/// 🔁️ One non-cancellable exact-three-Store restoration driven only by a consumed committed WAL witness.
#[must_use = "committed recovery must reach one terminal three-Store owner handoff"]
pub(crate) struct ArtifactCommittedDurableGroupRecoveryV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ParentMutation: store::Mutation<ParentP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingMutation: store::Mutation<DrawingP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueMutation: store::Mutation<ValueP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
{
    pub state: std::mem::ManuallyDrop<Option<ArtifactCommittedDurableGroupRecoveryStateV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArtifactCommittedDurableGroupRecoveryAdvanceV1 {
    Progress(store::durable_group::DurableOwnedThreeStoreCommitPhaseV1),
        DiagnosticProgress(store::durable_group::DurableOwnedThreeStoreCommitPhaseV1, semio_framework_value::retained_clone::RetainedCloneProgress),
Blocked,
    Fault(store::durable_group::DurableOwnedGroupDecisionError),
    Complete,
}

pub(crate) struct ArtifactCommittedDurableGroupRecoveryTerminalV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ParentMutation: store::Mutation<ParentP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingMutation: store::Mutation<DrawingP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueMutation: store::Mutation<ValueP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
{
    pub document: ArtifactId,
    pub receipt: store::durable_group::DurableOwnedGroupJournalReceiptV1,
    pub(crate) already_applied: bool,
    pub owners: store::durable_group::DurableOwnedMapRecoveryOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>,
}

pub(crate) struct ArtifactCommittedDurableGroupRecoveryRejectedTerminalV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ParentMutation: store::Mutation<ParentP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingMutation: store::Mutation<DrawingP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueMutation: store::Mutation<ValueP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
{
    pub document: ArtifactId,
    pub receipt: store::durable_group::DurableOwnedGroupJournalReceiptV1,
    pub error: store::durable_group::DurableOwnedGroupDecisionError,
    pub owners: store::durable_group::DurableOwnedMapRecoveryOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>,
}
//#endregion 🔖️Receipt
//#endregion 🔖️Ids
