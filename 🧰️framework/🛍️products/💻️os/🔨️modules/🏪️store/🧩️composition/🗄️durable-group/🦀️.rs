//! 🗺️ Closed codec and admission fence for one durable parent+drawing+value Store decision.
use semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPreparedParts;

use semio_framework_os_store_ledger::CursorRevisionAccumulator;
use pack::snapshot::{ArtifactDsl,ArtifactPack};
use semio_framework_dsl_record::DslField;
use semio_framework_dsl_record::{BorrowedDslRecord,native_encoding::FieldProjectionSource};
use semio_framework_pack_json::ArtifactCanonicalJsonTree;
use semio_framework_value::DslValue;
use semio_framework_dsl_record::FieldValue;
use semio_framework_value::FromValue as ValueFromValue;
use semio_framework_dsl_record::Shape;
use semio_framework_value::ToValue as ValueToValue;
use semio_framework_os_spr::{Edit, HybridLogicalTimestamp, Mutation as StoreMutation};
use semio_framework_os_store_engine::lifted_store::ArtifactStore;
use semio_framework_os_store_one_item_publication::{ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPrepared};
use semio_framework_os_store_envelope::owner::OwnerRef;
use semio_framework_os_pack::{PackDecodeOptions, PackEncodeOptions, PackError, PackVerificationLevel};
use semio_framework_diagnostic::TextError;
use semio_framework_os_pack::pack_rt;
use semio_framework_value_derive::{FromValue, ToValue};
use std::sync::Arc;
use std::mem::size_of;
use semio_framework_value::ErasedSnapshotRetirement as _;
use std::collections::BTreeMap;
pub use semio_framework_os_store_durable_group_wire::lifted_durable_group::{DurableOwnedThreeMemberDecisionV1, ArtifactStoreDurableGroupRootV1, DURABLE_OWNED_GROUP_ANCHOR_SCHEMA_V1, DURABLE_OWNED_GROUP_DECISION_SCHEMA_V1, DURABLE_OWNED_GROUP_EVENT_MAX_BYTES, DURABLE_OWNED_GROUP_ID_MAX_BYTES, DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES, DURABLE_OWNED_GROUP_SHAPE_V1, DURABLE_OWNED_GROUP_STRUCTURAL_MAX_BYTES, DurableOwnedGroupAnchorV1, DurableOwnedGroupDecisionError, DurableOwnedGroupJournalAdvanceV1, DurableOwnedGroupJournalCommitV1, DurableOwnedGroupJournalReceiptV1, DurableOwnedGroupJournalSinkV1, DurableOwnedGroupMapFrontiersV1, DurableOwnedGroupMapHandlesV1, DurableOwnedGroupMemberV1, DurableOwnedGroupVerifiedThreeEditsV1, DurableOwnedMapMemberAdmissionV1, DurableOwnedMapRecoveryAdvanceV1, DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedThreeStoreCommitPhaseV1, DurableOwnedThreeStoreMapAssemblyAdvanceV1, DurableOwnedThreeStoreMapAssemblyFailureV1, DurableOwnedThreeStoreMapAssemblyPhaseV1, DurableOwnedThreeStoreSnapshotV1, DurableStorePreparedOutcomeV1, DurableStorePreparedOutcomesV1, DurableStoreVerifiedOutcomeV1, StagedRootRetirement};
#[cfg(feature = "durable-group-testing")]
use semio_framework_os_store_durable_group_wire::lifted_durable_group::{durable_group_test_bind_member, durable_group_test_member, durable_group_test_outcome, DurableOwnedGroupTestMember};
use semio_framework_os_store_durable_group_wire::lifted_durable_group::{DurablePlanningOutputV1, DRAWING_RECOVERY_SCHEMA, DRAWING_ROLE, DURABLE_OWNED_GROUP_BOUND_OUTCOME_SCHEMA_V1, DurableBoundOneItemOutcomeV1, DurableMemberRecoveryFrontier, DurableOwnedThreeMemberUnsignedV1, DurableUnboundOneItemOutcomeV1, PARENT_RECOVERY_SCHEMA, PARENT_ROLE, VALUE_RECOVERY_SCHEMA, VALUE_ROLE, close_assembly_publication, close_journal_owner, member_identity_matches, member_matches, parse_canonical_json_value, take_assembly_prepared, valid_hash, valid_reference};
use semio_framework_os_store_engine_stage::lifted_durable_group::{DurableStoreBindingPlanV1, DurableStorePreparedOwnerV1, bound_member, prepare_staged_diagnostics, clear_adopted_store_member, prepared_base_matches_store, prepared_member, recovery_frontier, retire_binding_member, retire_unstaged_store_member, stage_store_member, store_member_identity, validate_adopted_store_member, verify_bound_member_edit};
pub use semio_framework_os_store_engine_stage::lifted_durable_group::{DurableStoreBoundOutcomeV1, abort_staged_store_member, adopt_staged_store_member, stage_prebuilt_batch_root};
use semio_framework_os_store_engine_stage::lifted_durable_group::planning;

#[path = "📒️group-decision-journal-record-and-snapshot-capture/🦀️.rs"]
mod group_decision_journal_record_and_snapshot_capture;
pub use group_decision_journal_record_and_snapshot_capture::{
    capture_store_owned_three_snapshot, DurableOwnedGroupJournalRecordV1,
    DurableOwnedThreeStoreBoundV1,
    DurableOwnedThreeStorePreparedV1, DurableOwnedThreeStoreRecoveryV1,
};

#[path = "🛣️map-commit-recovery-hosts-and-assembly-owners/🦀️.rs"]
mod map_commit_recovery_hosts_and_assembly_owners;
use map_commit_recovery_hosts_and_assembly_owners::{
    DurableOwnedMapCommitOperationV1, DurableOwnedMapRecoveryOperationV1,
    DurableOwnedThreeStoreCommitV1,
};
pub use map_commit_recovery_hosts_and_assembly_owners::{
    DurableOwnedMapCommitHostV1, DurableOwnedMapCommitOwnersV1, DurableOwnedMapRecoveryAdmissionV1,
    DurableOwnedMapRecoveryHostV1, DurableOwnedMapRecoveryOwnersV1,
    DurableOwnedMapRecoveryRejectedV1, DurableOwnedMapRecoveryStartV1,
    DurableOwnedThreeStoreMapAssemblyOwnersV1, DurableOwnedThreeStoreMapAssemblyV1,
};

#[path = "🧱️map-assembly-open-and-recovery-steps/🦀️.rs"]
mod map_assembly_open_and_recovery_steps;

#[path = "🚢️map-assembly-commit-and-publication-steps/🦀️.rs"]
mod map_assembly_commit_and_publication_steps;

#[path = "🛠️map-assembly-recovery-completion/🦀️.rs"]
mod map_assembly_recovery_completion;
pub use map_assembly_recovery_completion::recover_decision_store_owned;

#[path = "🪪️three-member-decision-operations-codecs-and-testing/🦀️.rs"]
mod three_member_decision_operations_codecs_and_testing;
#[cfg(feature="durable-group-testing")]
pub use three_member_decision_operations_codecs_and_testing::{
    durable_owned_group_journal_test_record, durable_owned_group_journal_test_record_from_edits,
};


impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> std::fmt::Debug for DurableOwnedThreeStorePreparedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.debug_struct("DurableOwnedThreeStorePreparedV1").field("binding_step", &self.binding_step).finish_non_exhaustive() }
}


impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> std::fmt::Debug for DurableOwnedThreeStoreBoundV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.debug_struct("DurableOwnedThreeStoreBoundV1").field("decision_sha256", &self.decision.decision_sha256).finish_non_exhaustive() }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedThreeStorePreparedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
    fn originals_empty(&self) -> bool { self.parent.originals_empty() && self.drawing.originals_empty() && self.value.originals_empty() }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedThreeStorePreparedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + ValueToValue + Send + Sync + 'static,
    ParentMutation: StoreMutation<ParentP> + ValueToValue + ValueFromValue + Send + 'static,
    DrawingP: ArtifactPack + ValueToValue + Send + Sync + 'static,
    DrawingMutation: StoreMutation<DrawingP> + ValueToValue + ValueFromValue + Send + 'static,
    ValueP: ArtifactPack + ValueToValue + Send + Sync + 'static,
    ValueMutation: StoreMutation<ValueP> + ValueToValue + ValueFromValue + Send + 'static,
{
    fn retire_originals_step(&mut self, parent: &mut ArtifactStore<ParentP, ParentMutation>, drawing: &mut ArtifactStore<DrawingP, DrawingMutation>, value: &mut ArtifactStore<ValueP, ValueMutation>, grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant) -> Result<bool, DurableOwnedGroupDecisionError> {
        self.retirement_progress=Default::default();
        if !self.decision_identity.ready(){self.retirement_progress=self.decision_identity.close(grant.retained_grant()).map_err(DurableOwnedGroupDecisionError::Retirement)?;return Ok(false);}
        #[cfg(feature = "deflate")]
        if !self.decision_encoding.terminal_is_empty() { self.retirement_progress=self.decision_encoding.close(grant.retained_grant()).map_err(DurableOwnedGroupDecisionError::Retirement)?;return Ok(false); }
        if !self.parent.originals_empty() { retire_binding_member(&mut self.parent, &mut parent.parts(), grant)?;self.retirement_progress=self.parent.retirement_progress; return Ok(false); }
        if !self.drawing.originals_empty() { retire_binding_member(&mut self.drawing, &mut drawing.parts(), grant)?;self.retirement_progress=self.drawing.retirement_progress; return Ok(false); }
        if !self.value.originals_empty() { retire_binding_member(&mut self.value, &mut value.parts(), grant)?;self.retirement_progress=self.value.retirement_progress; return Ok(false); }
        if let Some(owner)=self.decision_retirement.as_mut(){self.retirement_progress=owner.step(grant.retained_grant()).map_err(DurableOwnedGroupDecisionError::Retirement)?.progress();if owner.terminal_is_empty(){self.decision_retirement=None;}return Ok(false);}
        if let Some(owner)=self.decision_pack_retirement.as_mut(){self.retirement_progress=owner.step(grant.retained_grant()).map_err(DurableOwnedGroupDecisionError::Retirement)?.progress();if owner.terminal_is_empty(){self.decision_pack_retirement=None;}return Ok(false);}
        let copy=size_of::<Box<DurableOwnedThreeMemberDecisionV1>>()+size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<Box<DurableOwnedThreeMemberDecisionV1>>>()+size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<DurablePlanningOutputV1>>()+size_of::<DurablePlanningOutputV1>();
        if self.decision.is_some()||self.decision_pack.is_some(){if !grant.permits_one()||grant.maximum_copy_bytes<copy{return Ok(false);}if let Some(decision)=self.decision.take(){self.decision_retirement=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(decision).map_err(|(reason,_)|DurableOwnedGroupDecisionError::Retirement(reason))?);}else if let Some(pack)=self.decision_pack.take(){self.decision_pack_retirement=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(DurablePlanningOutputV1::Pack(pack)).map_err(|(reason,_)|DurableOwnedGroupDecisionError::Retirement(reason))?);}self.retirement_progress=semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()};return Ok(false);}
        Ok(true)
    }
}


impl planning::DurableDecisionSource for DurableOwnedThreeMemberDecisionV1 {
    fn fields(&self) -> semio_framework_os_store_durable_group_wire::lifted_durable_group::DurableBorrowedDecisionFields<'_> {
        semio_framework_os_store_durable_group_wire::lifted_durable_group::DurableBorrowedDecisionFields { schema: &self.schema, anchor: &self.anchor, anchor_sha256: &self.anchor_sha256, decision_sha256: &self.decision_sha256, parent: &self.parent, drawing: &self.drawing, value: &self.value }
    }
    fn set_anchor_hash(&mut self, hash: String) { self.anchor_sha256 = hash; }
    fn set_decision_hash(&mut self, hash: String) { self.decision_sha256 = hash; }
}
