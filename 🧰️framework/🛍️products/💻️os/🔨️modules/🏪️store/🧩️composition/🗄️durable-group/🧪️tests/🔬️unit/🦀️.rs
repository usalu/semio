use semio_framework_os_store_engine_stage::lifted_durable_group::planning;
use semio_framework_dsl_record::DslField;
use semio_framework_os_store_testing_fixtures::demo::DemoMutation; use semio_framework_os_store_testing_fixtures::demo::SetN;
use semio_framework_os_store_testing_doubles::DemoOneItemPreparationFactory; use semio_framework_os_store_testing_fixtures::DemoSnapshot; use semio_framework_os_store_testing_doubles::demo_closable_store_owners;
use std::sync::Arc;
use pack::snapshot::ArtifactDsl;
use semio_framework_os_store_engine::lifted_store::ArtifactStore;
use semio_framework_os_store_one_item_publication::{ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPrepared};
use semio_framework_os_store_durable_group_wire::lifted_durable_group::{DRAWING_ROLE, DURABLE_OWNED_GROUP_BOUND_OUTCOME_SCHEMA_V1, DURABLE_OWNED_GROUP_EVENT_MAX_BYTES, DURABLE_OWNED_GROUP_JSON_MAX_DEPTH, DURABLE_OWNED_GROUP_JSON_MAX_ITEMS, DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES, DURABLE_OWNED_GROUP_STRUCTURAL_MAX_BYTES, DURABLE_OWNED_GROUP_UNBOUND_OUTCOME_SCHEMA_V1, DurableBoundOneItemOutcomeV1, DurableOwnedGroupDecisionError, DurableOwnedGroupJournalAdvanceV1, DurableOwnedGroupJournalCommitV1, DurableOwnedGroupJournalReceiptV1, DurableOwnedGroupJournalSinkV1, DurableOwnedGroupMapFrontiersV1, DurableOwnedGroupMapHandlesV1, DurableOwnedGroupMemberV1, DurableOwnedMapMemberAdmissionV1, DurableOwnedMapRecoveryAdvanceV1, DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedThreeStoreCommitPhaseV1, DurableOwnedThreeStoreMapAssemblyAdvanceV1, DurableOwnedThreeStoreMapAssemblyFailureV1, DurableOwnedThreeStoreMapAssemblyPhaseV1, DurableStorePreparedOutcomeV1, DurableStorePreparedOutcomesV1, DurableUnboundOneItemOutcomeV1, PARENT_RECOVERY_SCHEMA, PARENT_ROLE, VALUE_ROLE, close_journal_owner, parse_canonical_json_value, valid_identity, validate_json_budget};
use semio_framework_value::{DslValue, ToValue as ValueToValue};
use semio_framework_os_store_durable_group::{DurableOwnedGroupJournalRecordV1, DurableOwnedMapCommitOwnersV1, DurableOwnedMapRecoveryAdmissionV1, DurableOwnedMapRecoveryOwnersV1, DurableOwnedMapRecoveryRejectedV1, DurableOwnedMapRecoveryStartV1, DurableOwnedThreeMemberDecisionV1, DurableOwnedThreeStoreBoundV1, DurableOwnedThreeStoreMapAssemblyOwnersV1, DurableOwnedThreeStoreMapAssemblyV1, DurableOwnedThreeStorePreparedV1, DurableOwnedThreeStoreRecoveryV1, capture_store_owned_three_snapshot};
use semio_framework_os_spr::{Edit, HybridLogicalTimestamp};
use semio_framework_dsl_record::{FieldValue, Shape};
use semio_framework_os_store_envelope::owner::OwnerRef;
use semio_framework_os_pack::{PackDecodeOptions, PackEncodeOptions, PackVerificationLevel, pack_rt};

include!("🧰️group-outcome-and-store-fixtures/🦀️.rs");
include!("📔️fake-journal-and-map-assembly-fixtures/🦀️.rs");
include!("🗺️map-assembly-and-decision-laws/🦀️.rs");
include!("🧾️journal-commit-cancellation-and-stage-error-laws/🦀️.rs");
include!("🎰️map-host-slot-foreign-anchor-and-carrier-laws/🦀️.rs");

#[cfg(feature = "durable-group-testing")]
fn fixture_test_member(case: &serde_json::Value, ordinal: u64) -> (DurableOwnedGroupTestMember<DemoSnapshot, String>, std::sync::Weak<DemoSnapshot>) {
    let actor = semio_framework_value::SharedUtf8::admit(fixture()["openedActor"].as_str().unwrap().to_string(), crate::os_store::component::tests::physical_test_close_grant()).unwrap_or_else(|(error, _)| panic!("original fixture actor: {error}" )).0;
    let edit = Edit { id: format!("durable-fixture-edit-{ordinal}"), actor: Some(actor.clone()), line: None, forwards: serde_json::from_value(case["forwards"].clone()).unwrap(), inverse: serde_json::from_value::<Vec<String>>(case["inverse"].clone()).unwrap().into(), mutation_meta: vec![crate::os_spr::MutationMeta { mutation_id: Some(crate::os_spr::MutationId(format!("durable-fixture-edit-{ordinal}#0"))), dependencies: Vec::new(), base_version: ordinal, author_id: Some(crate::os_spr::ActorId(actor)), timestamp: HybridLogicalTimestamp { actor: ordinal, physical_ms: 1_000 + ordinal, logical: 1 }, undo_policy: crate::os_spr::UndoPolicy::ExactBaseOnly, payload_hash: None, semantic_kind: None, label: None, group_id: None, origin: Default::default(), transaction: None }], verb: None, sequence_number: ordinal as i32, started_at: String::new(), finished_at: None };
    let post_snapshot = Arc::new(serde_json::from_value::<DemoSnapshot>(case["postSnapshot"].clone()).unwrap());
    let weak = Arc::downgrade(&post_snapshot);
    (DurableOwnedGroupTestMember { edit, post_snapshot, mutation_retirement: Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<String>::default()), snapshot_retirement: Arc::new(semio_framework_value::retirement::SharedValueRetirementFactory::<DemoSnapshot>::default()) }, weak)
}

#[cfg(feature = "durable-group-testing")]
#[test]
fn durable_test_fixture_closes_every_original_on_success_and_typed_refusal() {
    let laws = fixture();
    for case in laws["helperCases"].as_array().unwrap() {
        let (parent, parent_weak) = fixture_test_member(case, 1);
        let (drawing, drawing_weak) = fixture_test_member(case, 2);
        let (value, value_weak) = fixture_test_member(case, 3);
        let result = durable_owned_group_journal_test_record_from_edits(reference(&laws["anchor"]["parent"]), parent, drawing, value);
        assert_eq!([parent_weak.strong_count(), drawing_weak.strong_count(), value_weak.strong_count()], [0; 3], "{} closes all original snapshot strong owners", case["name"]);
        if let Some(expected) = case["expectedRefusal"].as_str() {
            let Err(DurableOwnedGroupDecisionError::Preparation(error)) = result else { panic!("fixture retains its typed preparation refusal"); };
            assert_eq!(error.kind.as_str(), expected);
        } else {
            let record = result.unwrap();
            let decision = DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(record.canonical_pack()).unwrap();
            for member in [&decision.parent, &decision.drawing, &decision.value] {
                let bound = DurableBoundOneItemOutcomeV1::decode_canonical_pack(&member.recovery_pack).unwrap();
                let independent: serde_json::Value = serde_json::from_slice(&bound.edit_canonical_json).unwrap();
                assert_eq!(independent["forwards"], case["forwards"]);
                assert_eq!(independent["inverse"], case["inverse"]);
                assert_eq!(independent["mutationMeta"][0]["group_id"], decision.decision_sha256);
            }
        }
        eprintln!("[DEBUG] durable fixture {} retires all three original roots", case["name"]);
    }
}

#[cfg(feature = "durable-group-testing")]
#[test]
fn durable_test_fixture_denied_frame_restores_exact_candidates_and_issuers() {
    let laws = fixture();
    let denied: semio_framework_value::retained_clone::RetainedCloneGrant = serde_json::from_value(laws["helperRetirementAdmission"].clone()).unwrap();
    let (member, weak) = fixture_test_member(&laws["helperCases"][0], 1);
    let edit_pointer = member.edit.forwards.as_ptr();
    let post_pointer = Arc::as_ptr(&member.post_snapshot);
    let mutation_pointer = Arc::as_ptr(&member.mutation_retirement);
    let snapshot_pointer = Arc::as_ptr(&member.snapshot_retirement);
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| semio_framework_os_store_one_item_publication::prepared_retirement::ArtifactStorePreparedRetirement::admit_returned(member.edit, member.post_snapshot, member.mutation_retirement, member.snapshot_retirement, denied));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    let Err((error, edit, post_snapshot, mutation_retirement, snapshot_retirement)) = result else { panic!("unfunded returned fixture frame admitted"); };
    assert_eq!(error.kind, semio_framework_value::ValueRefusalKind::OwnershipLimit);
    assert_eq!(edit.forwards.as_ptr(), edit_pointer);
    assert_eq!(Arc::as_ptr(&post_snapshot), post_pointer);
    assert!(std::ptr::addr_eq(Arc::as_ptr(&mutation_retirement), mutation_pointer));
    assert!(std::ptr::addr_eq(Arc::as_ptr(&snapshot_retirement), snapshot_pointer));
    assert_eq!(weak.strong_count(), 1);
    let member = DurableOwnedGroupTestMember { edit, post_snapshot, mutation_retirement, snapshot_retirement };
    assert!(durable_group_test_outcome(PARENT_RECOVERY_SCHEMA, 1, member).is_ok());
    assert_eq!(weak.strong_count(), 0);
    eprintln!("[DEBUG] durable fixture unfunded frame returns all exact candidates/issuers and then closes funded");
}
