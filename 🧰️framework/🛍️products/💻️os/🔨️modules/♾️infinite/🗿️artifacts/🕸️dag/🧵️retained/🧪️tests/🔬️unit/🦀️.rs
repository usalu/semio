use super::*;
use crate::os_dsl::FromValue as _;

fn close(retirement: &mut dyn ErasedSnapshotRetirement) {
    for _ in 0..100_000 {
        match retirement.close_step(1, 7).expect("bounded DAG retirement") {
            SnapshotRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty());
                return;
            }
            SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= 7);
            }
            SnapshotRetirementStep::Blocked => panic!("nonzero DAG retirement grant blocked"),
        }
    }
    panic!("DAG retirement did not reach terminal-empty");
}

#[test]
fn neutral_fixture_retires_exact_mutation_shared_snapshot_and_final_snapshot_owners() {
    let source = include_str!("../../../🌿️vcs/🧫️fixtures/🔣️mutations.json");
    let oracle: serde_json::Value = serde_json::from_str(source).expect("serde oracle");
    let fixture = crate::os_pack::json::parse(source).expect("first-party fixture parser");
    let first_party_oracle: serde_json::Value = serde_json::from_str(&crate::os_pack::json::to_json_string(&fixture)).expect("first-party fixture as serde oracle");
    assert_eq!(first_party_oracle, oracle);
    let row = &fixture["valid"].as_array().expect("valid cases")[0];
    let mut value = row["payload"].clone();
    value.as_object_mut().expect("mutation payload").insert("operation".to_string(), row["operation"].clone());
    let mutation = DagMutation::from_value(crate::os_pack::json::to_dsl_value(&value)).expect("fixture mutation");
    let mut mutation_retirement = DagMutationRetirementFactory.retire_owned(mutation);
    assert_eq!(mutation_retirement.close_step(0, 7).unwrap(), SnapshotRetirementStep::Blocked);
    assert_eq!(mutation_retirement.close_step(1, 0).unwrap(), SnapshotRetirementStep::Blocked);
    close(mutation_retirement.as_mut());

    let snapshot = crate::default_dag_document();
    assert!(!snapshot.nodes.is_empty());
    let shared = Arc::new(snapshot.clone());
    let observer = Arc::clone(&shared);
    assert_eq!(Arc::strong_count(&shared), 2);
    let mut shared_retirement = DagSnapshotRetirementFactory.retire(shared);
    assert_eq!(shared_retirement.close_step(0, 7).unwrap(), SnapshotRetirementStep::Blocked);
    assert!(matches!(shared_retirement.close_step(1, 0).unwrap(), SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }));
    assert_eq!(shared_retirement.close_step(1, 0).unwrap(), SnapshotRetirementStep::Complete);
    assert!(shared_retirement.terminal_is_empty());
    assert_eq!(Arc::strong_count(&observer), 1);
    assert_eq!(observer.nodes, snapshot.nodes);

    let final_owner = Arc::new(snapshot);
    assert_eq!(Arc::strong_count(&final_owner), 1);
    let mut final_retirement = DagSnapshotRetirementFactory.retire(final_owner);
    assert_eq!(final_retirement.close_step(0, 7).unwrap(), SnapshotRetirementStep::Blocked);
    assert!(matches!(final_retirement.close_step(1, 0).unwrap(), SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }));
    assert_eq!(final_retirement.close_step(1, 0).unwrap(), SnapshotRetirementStep::Blocked);
    close(final_retirement.as_mut());
}

/// 📦️ A DAG document opens as an owned member of a composed document through its OWN whole-pack
/// codec, and the owner the opener hands back retires to terminal-empty under a bounded grant.
/// Guards the 2026-09-21 fix: the declaration was `UnsupportedMemberSnapshotOpen`, whose `step` has
/// exactly one answer — `Rejected` at step 0 — so every composed replacement and every document
/// archive carrying a real DAG member was refused before it began.
#[test]
fn dag_opens_as_an_owned_member_through_its_own_pack_codec() {
    assert_eq!(
        std::any::type_name::<<DagSnapshot as MemberStoreOwner<DagMutation>>::SnapshotOpen>(),
        std::any::type_name::<crate::os_store::PackMemberSnapshotOpen<DagSnapshot>>(),
        "a DAG member must open through PackMemberSnapshotOpen"
    );
    let snapshot = crate::default_dag_document();
    assert!(!snapshot.nodes.is_empty(), "the round-tripped member must carry real content");
    let encoded = crate::os_store::ArtifactPack::encode_pack(&snapshot);
    let decoded = <DagSnapshot as crate::os_store::ArtifactPack>::decode_pack(&encoded).expect("the member opener's whole-pack decode");
    assert_eq!(decoded.nodes, snapshot.nodes, "the opener's decode round-trips the exact member snapshot");
    let mut cursor = crate::os_store::retirement::RetireOwned::retirement(decoded);
    for turn in 0..1_000_000 {
        match cursor.close_step(4_096) {
            crate::os_store::retirement::RetirementStep::Complete if cursor.terminal_is_empty() => break,
            crate::os_store::retirement::RetirementStep::BudgetExhausted => panic!("the member opener's owner cursor stalled on turn {turn}"),
            _ => {}
        }
        assert!(turn < 999_999, "the member opener's owner cursor never reached terminal-empty");
    }
    let mut zero = crate::os_store::retirement::RetireOwned::retirement(crate::default_dag_document());
    assert!(matches!(zero.close_step(0), crate::os_store::retirement::RetirementStep::BudgetExhausted), "a zero grant is exhaustion, never progress");
    for turn in 0..1_000_000 {
        match zero.close_step(4_096) {
            crate::os_store::retirement::RetirementStep::Complete if zero.terminal_is_empty() => break,
            crate::os_store::retirement::RetirementStep::BudgetExhausted => panic!("the refused turn left the cursor stuck on turn {turn}"),
            _ => {}
        }
    }
}
