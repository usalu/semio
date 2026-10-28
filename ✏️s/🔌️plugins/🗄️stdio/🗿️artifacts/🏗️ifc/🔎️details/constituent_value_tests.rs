//! 🧪️ Nonreference constituent SetValue retains original value work and central reversible publication.
use super::*;
fn contract() -> serde_json::Value {
    serde_json::from_str(include_str!("🧪️tests/constituent-value.json")).unwrap()
}
fn source(contract: &serde_json::Value) -> Arc<IfcSnapshot> {
    let parsed = semio_framework_pack_json::parse(&contract["snapshot"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    Arc::new(IfcSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).unwrap())
}
fn event(row: &serde_json::Value) -> editing::SnapshotEditEvent {
    let path = row["path"].as_str().unwrap().into();
    if row["event"] == "remove" {
        return editing::SnapshotEditEvent::RemoveValue { path };
    }
    if row["event"] == "move" {
        return editing::SnapshotEditEvent::MoveValue { path, from: row["from"].as_str().unwrap().into() };
    }
    let parsed = semio_framework_pack_json::parse(&row["value"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let value = semio_framework_pack_json::to_dsl_value(&parsed);
    if row["event"] == "insert" { editing::SnapshotEditEvent::InsertValue { path, value } } else { editing::SnapshotEditEvent::SetValue { path, value } }
}
fn work(row: &serde_json::Value) -> Work {
    Work::new(match row["event"].as_str() {
        Some("insert") => editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
        Some("remove") => editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
        Some("move") => editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
        _ => editing::SET_SNAPSHOT_VALUE_ACTION_ID,
    })
}
#[test]
fn constituent_nonreference_values_follow_neutral_declarations_and_actual_inverse() {
    let contract = contract();
    let snapshot = source(&contract);
    for row in contract["cases"].as_array().unwrap() {
        let mut caller = OriginalWorkFixture::new();
        let mut work = work(row);
        inputs(&snapshot, event(row), &mut caller, &mut work, |input, caller, work| {
            let mut result = None;
            for turn_index in 0..100_000 {
                match caller.work_turn(RetainedCloneGrant { maximum_items: 1, ..grant() }, 1, |cx| work.step(input, cx)) {
                    Ok(ArtifactCommandWorkStep::Complete(emit)) => {
                        result = Some(Ok(emit));
                        break;
                    }
                    Err(error) => {
                        result = Some(Err(error));
                        break;
                    }
                    _ => {}
                }
                assert!(turn_index < 99_999);
            }
            let result = result.unwrap();
            assert_eq!(result.is_ok(), row["admitted"].as_bool().unwrap(), "{}", row["id"]);
            if let Ok(emit) = result {
                assert_eq!(work.reduction_ticks(), 0);
                assert_eq!(emit.artifact_mutations.len(), 1);
                assert!(matches!(&emit.artifact_mutations[0], IfcMutation::SetEntityComplex(_)));
                let mutation = &emit.artifact_mutations[0];
                let next = protocol::apply_diff_cold(mutation.diff(&snapshot).diff(), &snapshot).unwrap();
                let position = if row["path"].as_str().unwrap().starts_with("/entities/0/") { 0 } else { 1 };
                assert_eq!(next.entities[position].args, snapshot.entities[position].args);
                assert_eq!(next.entities[1 - position], snapshot.entities[1 - position]);
                let expected = semio_framework_pack_json::parse(&row["expected"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
                let argument = if row["id"] == "optional-category" { 2 } else { 0 };
                assert_eq!(next.entities[position].complex[0].args[argument], IfcValue::from_value(semio_framework_pack_json::to_dsl_value(&expected)).unwrap());
                let mut restored = next;
                for inverse in mutation.inverse(&snapshot).unwrap() {
                    restored = protocol::apply_diff_cold(inverse.diff(&restored).diff(), &restored).unwrap();
                }
                assert_eq!(restored, *snapshot);
            }
            caller.cancel().cancel_now();
            close(caller, work);
            assert!(work.terminal_is_empty());
        });
        caller.finish();
        assert_eq!(Arc::strong_count(&snapshot), 1);
    }
}
#[test]
fn constituent_nonreference_value_cancels_every_prefix_under_original_close() {
    let contract = contract();
    let snapshot = source(&contract);
    let original = snapshot.as_ref().clone();
    for row in contract["cases"].as_array().unwrap().iter().filter(|row| matches!(row["id"].as_str(), Some("coordinate-payload" | "coordinate-insert" | "coordinate-remove" | "coordinate-move"))) {
        let mut turns = 0;
        for cancel_at in std::iter::once(None).chain((1..100_000).map(Some)) {
            if cancel_at.is_some_and(|turn| turn > turns) {
                break;
            }
            let mut caller = OriginalWorkFixture::new();
            let mut work = work(row);
            inputs(&snapshot, event(row), &mut caller, &mut work, |input, caller, work| {
                for turn_index in 1..100_000 {
                    if cancel_at == Some(turn_index) {
                        caller.cancel().cancel_now();
                    }
                    let result = caller.work_turn(RetainedCloneGrant { maximum_items: 1, ..grant() }, 1, |cx| work.step(input, cx));
                    assert_eq!(snapshot.as_ref(), &original);
                    assert_eq!(work.reduction_ticks(), 0);
                    if cancel_at == Some(turn_index) {
                        assert!(result.is_err());
                        break;
                    }
                    if let ArtifactCommandWorkStep::Complete(emit) = result.unwrap() {
                        assert_eq!(emit.artifact_mutations.len(), 1);
                        turns = turn_index;
                        break;
                    }
                    assert!(turn_index < 99_999);
                }
                caller.cancel().cancel_now();
                close(caller, work);
                assert!(work.terminal_is_empty());
            });
            caller.finish();
            assert_eq!(Arc::strong_count(&snapshot), 1);
        }
        assert!(turns > 20);
    }
}
