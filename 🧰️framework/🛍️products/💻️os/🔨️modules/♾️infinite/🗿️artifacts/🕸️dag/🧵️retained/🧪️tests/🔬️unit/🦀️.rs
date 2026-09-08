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
    let source = include_str!("../../../🌿️vcs/🧪️fixtures/🔣️mutations.json");
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
