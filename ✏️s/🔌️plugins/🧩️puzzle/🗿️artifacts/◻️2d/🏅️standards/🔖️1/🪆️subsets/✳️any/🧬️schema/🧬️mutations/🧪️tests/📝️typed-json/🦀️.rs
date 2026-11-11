//! 📝️ Public typed JSON mutation contracts match independent Serde admission and the central applier.
use semio_s_artifact_puzzle_2d_mutations::{Puzzle2dSnapshot,standards::v1::subsets::any::schema::mutations::{Puzzle2dMutation,create_node,delete_node,move_node}};
use semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::records::PUZZLE_2D_SCHEMA;
use semio_os_kernel as protocol;
use protocol::ColdMutation;
use semio_framework_test_fixtures::fixture_text;
use serde_json::{json,Value};


#[path = "🔬️oracle/🦀️.rs"]
mod json_oracle;

/// 🪢️ A sequence of concrete kinds — a delete, a move, a create — applies centrally to the document it states and its
/// inverse rows, replayed last-to-first, restore the canonical pre-edit document. Nothing here differences two boards.
#[test]
fn concrete_kinds_round_trip_through_the_central_applier() {
    let before = json!({ "schema": PUZZLE_2D_SCHEMA, "nodes": [{ "id": "n1", "anchor": "fixed", "x": 0.0, "y": 0.0, "handles": [] }, { "id": "n2", "anchor": "fixed", "x": 10.0, "y": 0.0, "handles": [] }], "edges": [] });
    let after = json!({ "schema": PUZZLE_2D_SCHEMA, "nodes": [{ "id": "n2", "anchor": "fixed", "x": 99.0, "y": 0.0, "handles": [] }, { "id": "n3", "anchor": "fixed", "x": 1.0, "y": 0.0, "handles": [] }], "edges": [] });
    let canonical = |value: &Value| json_oracle::snapshot(value).expect("independent typed puzzle2d snapshot");
    let before_snapshot: Puzzle2dSnapshot=semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(&before)).expect("base admits");
    let created: semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::records::Puzzle2dNode=semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(&json!({ "id": "n3", "anchor": "fixed", "x": 1.0, "y": 0.0, "handles": [] }))).expect("node admits");
    let operations=vec![delete_node("n1".into()), move_node("n2".into(), 99.0, 0.0), create_node(created, None)];
    let mut forward = before_snapshot.clone();
    let mut inverses = Vec::new();
    for operation in &operations {
        inverses.extend(<Puzzle2dMutation as ColdMutation<Puzzle2dSnapshot>>::inverse(operation, &forward).expect("valid retained mutation inverse snapshot"));
        let next = protocol::apply_diff_cold(<Puzzle2dMutation as ColdMutation<Puzzle2dSnapshot>>::diff(operation, &forward).diff(), &forward).expect("valid mutation diff");
        json_oracle::retire(std::mem::replace(&mut forward, next));
    }
    let expected_after = canonical(&after);
    assert_eq!(forward, expected_after);
    json_oracle::retire(expected_after);
    for inverse in inverses.iter().rev() {
        let next = protocol::apply_diff_cold(<Puzzle2dMutation as ColdMutation<Puzzle2dSnapshot>>::diff(inverse, &forward).diff(), &forward).expect("valid mutation diff");
        json_oracle::retire(std::mem::replace(&mut forward, next));
    }
    let expected_before = canonical(&before);
    assert_eq!(forward, expected_before, "backwards operations must restore the pre-edit document");
    json_oracle::retire(expected_before);
    json_oracle::retire(forward);
    json_oracle::retire(before_snapshot);
    json_oracle::retire(operations);
    json_oracle::retire(inverses);
}

/// 🧫️ Language-neutral roots retain typed mutation authority and match Serde's admitted values: every committed case states the
/// concrete kinds that take `before` to `after`, decoded by our codec and by the third-party oracle alike.
#[test]
fn puzzle2d_typed_mutation_fixture_and_refusal_law() {
    let fixture: serde_json::Value=serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations/📸️typed-mutation/🔣️.json")).expect("neutral fixture");
    assert_eq!(fixture["schema"], "semio.puzzle.2d.typed-mutation.v2");
    for case in fixture["cases"].as_array().expect("cases") {
        let before: Puzzle2dSnapshot=semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(&case["before"])).expect("base admits");
        let after: Puzzle2dSnapshot=semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(&case["after"])).expect("result admits");
        let oracle_before: Puzzle2dSnapshot=json_oracle::snapshot(&case["before"]).expect("third-party base");
        let oracle_after: Puzzle2dSnapshot=json_oracle::snapshot(&case["after"]).expect("third-party result");
        assert_eq!(before,oracle_before);
        assert_eq!(after,oracle_after);
        let operations: Vec<Puzzle2dMutation>=case["operations"].as_array().expect("operations").iter().map(|operation| {
            let ours: Puzzle2dMutation=semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(operation)).expect("operation admits");
            let oracle: Puzzle2dMutation=json_oracle::mutation(operation).expect("third-party operation");
            assert_eq!(ours,oracle);
            json_oracle::retire(oracle);
            ours
        }).collect();
        let mut current=before.clone();
        let mut inverse=Vec::new();
        for operation in &operations {
            inverse.extend(<Puzzle2dMutation as ColdMutation<Puzzle2dSnapshot>>::inverse(operation,&current).expect("inverse"));
            let next=protocol::apply_diff_cold(<Puzzle2dMutation as ColdMutation<Puzzle2dSnapshot>>::diff(operation,&current).diff(),&current).expect("typed apply");
            json_oracle::retire(std::mem::replace(&mut current,next));
        }
        assert_eq!(current,oracle_after);
        for operation in inverse.iter().rev() {
            let next=protocol::apply_diff_cold(<Puzzle2dMutation as ColdMutation<Puzzle2dSnapshot>>::diff(operation,&current).diff(),&current).expect("typed inverse apply");
            json_oracle::retire(std::mem::replace(&mut current,next));
        }
        assert_eq!(current,oracle_before);
        json_oracle::retire(current);
        json_oracle::retire(before);
        json_oracle::retire(after);
        json_oracle::retire(oracle_before);
        json_oracle::retire(oracle_after);
        json_oracle::retire(operations);
        json_oracle::retire(inverse);
    }
    for value in fixture["invalid"].as_array().expect("invalid") {
        let actual= <Puzzle2dSnapshot as semio_framework_value::FromValue>::from_value(semio_framework_value::DslValue::from(value));
        let oracle=json_oracle::snapshot(value);
        assert!(actual.is_err());
        assert!(oracle.is_err());
    }
}
