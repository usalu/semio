//! 🗺️ Closed codec and admission fence for one durable parent+drawing+value Store decision.
use semio_os_store_one_item_publication::ArtifactStoreOneItemPreparedParts;

use semio_os_store_ledger::CursorRevisionAccumulator;
use pack::snapshot::{ArtifactDsl,ArtifactPack};
use semio_framework_dsl_record::DslField;
use semio_framework_dsl_record::{BorrowedDslRecord,native_encoding::FieldProjectionSource};
use semio_framework_pack_json::ArtifactCanonicalJsonTree;
use semio_framework_value::DslValue;
use semio_framework_dsl_record::FieldValue;
use semio_framework_value::FromValue as ValueFromValue;
use semio_framework_dsl_record::Shape;
use semio_framework_value::ToValue as ValueToValue;
use semio_os_spr::{Edit, HybridLogicalTimestamp, Mutation as StoreMutation};
use semio_os_store_engine::lifted_store::ArtifactStore;
use semio_os_store_one_item_publication::{ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPrepared};
use semio_os_store_envelope::owner::OwnerRef;
use semio_os_pack::{PackDecodeOptions, PackEncodeOptions, PackError, PackVerificationLevel};
use semio_framework_diagnostic::TextError;
use semio_os_pack::pack_rt;
use semio_framework_value_derive::{FromValue, ToValue};
use std::sync::Arc;
use std::mem::size_of;
use semio_framework_value::ErasedSnapshotRetirement as _;
use std::collections::BTreeMap;
pub use semio_os_store_durable_group_wire::lifted_durable_group::{DurableOwnedThreeMemberDecisionV1, ArtifactStoreDurableGroupRootV1, DURABLE_OWNED_GROUP_ANCHOR_SCHEMA_V1, DURABLE_OWNED_GROUP_DECISION_SCHEMA_V1, DURABLE_OWNED_GROUP_EVENT_MAX_BYTES, DURABLE_OWNED_GROUP_ID_MAX_BYTES, DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES, DURABLE_OWNED_GROUP_SHAPE_V1, DURABLE_OWNED_GROUP_STRUCTURAL_MAX_BYTES, DurableOwnedGroupAnchorV1, DurableOwnedGroupDecisionError, DurableOwnedGroupJournalAdvanceV1, DurableOwnedGroupJournalCommitV1, DurableOwnedGroupJournalReceiptV1, DurableOwnedGroupJournalSinkV1, DurableOwnedGroupMapFrontiersV1, DurableOwnedGroupMapHandlesV1, DurableOwnedGroupMemberV1, DurableOwnedGroupVerifiedThreeEditsV1, DurableOwnedMapMemberAdmissionV1, DurableOwnedMapRecoveryAdvanceV1, DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedThreeStoreCommitPhaseV1, DurableOwnedThreeStoreMapAssemblyAdvanceV1, DurableOwnedThreeStoreMapAssemblyFailureV1, DurableOwnedThreeStoreMapAssemblyPhaseV1, DurableOwnedThreeStoreSnapshotV1, DurableStorePreparedOutcomeV1, DurableStorePreparedOutcomesV1, DurableStoreVerifiedOutcomeV1, StagedRootRetirement};
#[cfg(feature = "durable-group-testing")]
use semio_os_store_durable_group_wire::lifted_durable_group::{durable_group_test_bind_member, durable_group_test_member, durable_group_test_outcome, DurableOwnedGroupTestMember};
use semio_os_store_durable_group_wire::lifted_durable_group::{DurablePlanningOutputV1, DRAWING_RECOVERY_SCHEMA, DRAWING_ROLE, DURABLE_OWNED_GROUP_BOUND_OUTCOME_SCHEMA_V1, DurableBoundOneItemOutcomeV1, DurableMemberRecoveryFrontier, DurableOwnedThreeMemberUnsignedV1, DurableUnboundOneItemOutcomeV1, PARENT_RECOVERY_SCHEMA, PARENT_ROLE, VALUE_RECOVERY_SCHEMA, VALUE_ROLE, close_assembly_publication, close_journal_owner, member_identity_matches, member_matches, parse_canonical_json_value, take_assembly_prepared, valid_hash, valid_reference};
use semio_os_store_engine_stage::lifted_durable_group::{DurableStoreBindingPlanV1, DurableStorePreparedOwnerV1, bound_member, prepare_staged_diagnostics, clear_adopted_store_member, prepared_base_matches_store, prepared_member, recovery_frontier, retire_binding_member, retire_unstaged_store_member, stage_store_member, store_member_identity, validate_adopted_store_member, verify_bound_member_edit};
pub use semio_os_store_engine_stage::lifted_durable_group::{DurableStoreBoundOutcomeV1, abort_staged_store_member, adopt_staged_store_member, stage_prebuilt_batch_root};
use semio_os_store_engine_stage::lifted_durable_group::planning;

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


/// 🎟️ Runs one retirement step as one original effect of the caller's receiving turn; `None` means the turn did not admit the quoted work and nothing ran.
pub(crate) fn effect_turn<R>(
    receiving: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,
    grant: semio_os_store_one_item::ArtifactStoreOneItemGrant,
    run: impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>) -> Result<R, DurableOwnedGroupDecisionError>,
) -> Result<Option<R>, DurableOwnedGroupDecisionError> {
    receiving.with_effect(grant.retained_grant(), u64::MAX, |_, effect| run(effect)).map_err(DurableOwnedGroupDecisionError::Retirement)?.transpose()
}

/// 🫴️ Runs one resident-frame original encoding turn on the receiving control and settles its pair before the caller sees any output.
pub(crate) fn encoding_turn<R>(
    receiving: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,
    operation: impl FnOnce(usize, &mut dyn FnMut(semio_framework_value::native_encoding::NativeEncodeProgress) -> bool, &mut dyn semio_framework_value_native_owned::native_decoding::NativeSendAllocationLoan) -> Result<R, DurableOwnedGroupDecisionError>,
) -> Result<R, DurableOwnedGroupDecisionError> {
    use DurableOwnedGroupDecisionError::Retirement;
    let invalid = || Retirement(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "native encoding turn left inconsistent original custody"));
    let mut frame = None;
    let mut call = semio_framework_value_native_owned::native_decoding::NativeCallFailure::<R, DurableOwnedGroupDecisionError> { output: None, operation: None, context: None, receipt_refusal: None };
    let run = receiving.with_encoding_receiver(&mut frame, &mut call, |room, observer, allocation, _, _, output, failure| match operation(room, observer, allocation) { Ok(value) => *output = Some(value), Err(error) => *failure = Some(error) });
    let mut refusal = run.err();
    if let Some(frame) = frame.as_mut() {
        if let Err(error) = frame.settle_pair() { refusal.get_or_insert(error); } else {
            match frame.take_allocation_failure() { Ok(failure) if failure.is_empty() => {}, Ok(_) => { refusal.get_or_insert_with(|| match invalid() { Retirement(error) => error, _ => unreachable!() }); }, Err(error) => { refusal.get_or_insert(error); } }
            if let Err(error) = receiving.release_encoding_receiver(frame) { refusal.get_or_insert(error); }
        }
    }
    let semio_framework_value_native_owned::native_decoding::NativeCallFailure { output, operation, context, receipt_refusal } = call;
    if let Some(error) = operation { return Err(error); }
    if let Some(error) = context.or(receipt_refusal).or(refusal) { return Err(Retirement(error)); }
    output.ok_or_else(invalid)
}

/// 🎟️ Runs one close step of a value-level retirement owner inside [`effect_turn`] and answers its progress.
pub(crate) fn close_turn(
    receiving: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,
    grant: semio_os_store_one_item::ArtifactStoreOneItemGrant,
    run: impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>) -> Result<semio_framework_value::RetainedCloneProgress, semio_framework_value::ValueError>,
) -> Result<semio_framework_value::RetainedCloneProgress, DurableOwnedGroupDecisionError> {
    Ok(effect_turn(receiving, grant, |effect| run(effect).map_err(DurableOwnedGroupDecisionError::Retirement))?.unwrap_or_default())
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> std::fmt::Debug for DurableOwnedThreeStorePreparedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.debug_struct("DurableOwnedThreeStorePreparedV1").field("binding_step", &self.binding_step).finish_non_exhaustive() }
}


impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> std::fmt::Debug for DurableOwnedThreeStoreBoundV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.debug_struct("DurableOwnedThreeStoreBoundV1").field("decision_sha256", &self.decision.decision_sha256).finish_non_exhaustive() }
}




