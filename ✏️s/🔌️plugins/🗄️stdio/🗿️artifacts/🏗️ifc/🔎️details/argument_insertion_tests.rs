//! 🧪️ Missing root arguments retain only original new input before central insertion and undo.
use super::*;
fn contract() -> serde_json::Value {
    serde_json::from_str(include_str!("🧪️tests/argument-insertion.json")).unwrap()
}
fn native(value: &serde_json::Value) -> IfcValue {
    if value["kind"] == "reference" {
        return IfcValue::Reference(value["value"].as_str().unwrap().parse().unwrap());
    }
    let parsed = semio_framework_pack_json::parse(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    IfcValue::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).unwrap()
}
fn source(row: &serde_json::Value) -> Arc<IfcSnapshot> {
    let arguments = row["args"].as_array().unwrap().iter().map(native).collect();
    let (name, args, complex) =
        if let Some(root) = row["root"].as_str() { (root.into(), vec![], vec![IfcComplexType { name: row["name"].as_str().unwrap().into(), args: arguments }]) } else { (row["name"].as_str().unwrap().into(), arguments, vec![]) };
    let mut entities = vec![IfcEntity { id: row["entityId"].as_str().unwrap_or("1").parse().unwrap(), name, args, complex }];
    if let Some(targets) = row["targets"].as_array() {
        entities.extend(targets.iter().map(|target| IfcEntity { id: target["id"].as_str().unwrap().parse().unwrap(), name: target["name"].as_str().unwrap().into(), args: vec![], complex: vec![] }));
    }
    Arc::new(IfcSnapshot { entities, ..Default::default() })
}
fn event(row: &serde_json::Value) -> editing::SnapshotEditEvent {
    if row["event"] == "remove" {
        return editing::SnapshotEditEvent::RemoveValue { path: row["path"].as_str().unwrap().into() };
    }
    if row["value"]["kind"] == "reference" {
        return editing::SnapshotEditEvent::InsertValue { path: row["path"].as_str().unwrap().into(), value: native(&row["value"]).to_value() };
    }
    let parsed = semio_framework_pack_json::parse(&row["value"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    editing::SnapshotEditEvent::InsertValue { path: row["path"].as_str().unwrap().into(), value: semio_framework_pack_json::to_dsl_value(&parsed) }
}
#[test]
fn original_missing_root_argument_insertions_follow_neutral_domains_and_actual_central_inverse() {
    for row in contract()["cases"].as_array().unwrap() {
        let snapshot = source(row);
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::INSERT_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, event(row), &mut caller, &mut work, |input, caller, work| {
            let mut result = None;
            for turn in 0..100000 {
                if work.prepared && work.value.is_none() && !work.value_edit.started() && work.pending.is_none() && work.reference.complete() && work.enumeration.complete() {
                    let demand = work.work_demands(input, 4096).unwrap();
                    for axis in 0..6 {
                        let mut denied = grant();
                        let mut fuel = 1;
                        match axis {
                            0 => denied.maximum_items = 0,
                            1 if demand.copy_bytes > 0 => denied.maximum_copy_bytes = demand.copy_bytes - 1,
                            2 if demand.capacity_bytes > 0 => denied.maximum_capacity_bytes = demand.capacity_bytes - 1,
                            3 if demand.release_bytes > 0 => denied.maximum_release_bytes = demand.release_bytes - 1,
                            4 => denied.maximum_depth = 0,
                            5 => fuel = 0,
                            _ => continue,
                        }
                        caller
                            .work_turn(denied, fuel, |cx| {
                                let before = cx.retained_progress();
                                let step = work.step(input, cx)?;
                                assert!(!matches!(step, ArtifactCommandWorkStep::Complete(_)));
                                assert_eq!(before, cx.retained_progress());
                                Ok(())
                            })
                            .unwrap();
                    }
                }
                let step = caller.work_turn(RetainedCloneGrant { maximum_items: 1, ..grant() }, 1, |cx| work.step(input, cx));
                assert_eq!(work.reduction_ticks(), 0);
                assert!(work.entity.is_none() && work.semantic_emit.is_none() && work.cursor.terminal_is_empty());
                match step {
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
                assert!(turn < 99999);
            }
            let result = result.unwrap();
            assert_eq!(result.is_ok(), row["admitted"].as_bool().unwrap(), "{}", row["id"]);
            if let Ok(emit) = result {
                assert_eq!(emit.artifact_mutations.len(), 1);
                let mutation = &emit.artifact_mutations[0];
                let IfcMutation::InsertEntityArg(value) = mutation else { panic!("central argument insertion required") };
                assert_eq!(value.id, snapshot.entities[0].id);
                assert_eq!(value.index, snapshot.entities[0].args.len());
                assert_eq!(value.value, native(&row["value"]));
                let next = protocol::apply_diff_cold(mutation.diff(&snapshot).diff(), &snapshot).unwrap();
                let mut expected = snapshot.as_ref().clone();
                expected.entities[0].args.push(native(&row["value"]));
                assert_eq!(next, expected);
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
fn original_missing_argument_insertions_cancel_every_preparation_prefix_and_close() {
    let contract = contract();
    let constituents: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/constituent-insertion.json")).unwrap();
    let removals: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/constituent-removal.json")).unwrap();
    for row in
        contract["cases"].as_array().unwrap().iter().chain(constituents["cases"].as_array().unwrap()).chain(removals["cases"].as_array().unwrap()).filter(|row| {
            matches!(row["id"].as_str(), Some("dash-category" | "exact-id-unicode" | "missing-coordinate-list" | "dash-unicode" | "coordinate-list" | "middle" | "first-extra" | "middle-extra" | "last-extra" | "exact-id-unicode-removal"))
        })
    {
        let snapshot = source(row);
        let original = snapshot.as_ref().clone();
        let mut turns = 0;
        for cancel_at in std::iter::once(None).chain((1..100000).map(Some)) {
            if cancel_at.is_some_and(|turn| turn > turns) {
                break;
            }
            let mut caller = OriginalWorkFixture::new();
            let mut work = Work::new(if row["event"] == "remove" { editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID } else { editing::INSERT_SNAPSHOT_VALUE_ACTION_ID });
            inputs(&snapshot, event(row), &mut caller, &mut work, |input, caller, work| {
                for turn in 1..100000 {
                    if cancel_at == Some(turn) {
                        caller.cancel().cancel_now();
                    }
                    let step = caller.work_turn(RetainedCloneGrant { maximum_items: 1, ..grant() }, 1, |cx| work.step(input, cx));
                    assert_eq!(snapshot.as_ref(), &original);
                    assert_eq!(work.reduction_ticks(), 0);
                    assert!(work.entity.is_none() && work.semantic_emit.is_none());
                    if cancel_at == Some(turn) {
                        assert!(step.is_err());
                        break;
                    }
                    if let ArtifactCommandWorkStep::Complete(emit) = step.unwrap() {
                        assert_eq!(emit.artifact_mutations.len(), 1);
                        turns = turn;
                        break;
                    }
                    assert!(turn < 99999);
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

#[test]
fn original_missing_constituent_argument_insertions_follow_neutral_arity_and_central_inverse() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/constituent-insertion.json")).unwrap();
    check_constituent_arguments(&contract);
}
#[test]
fn original_constituent_argument_removals_follow_neutral_arity_and_central_inverse() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/constituent-removal.json")).unwrap();
    check_constituent_arguments(&contract);
}
fn check_constituent_arguments(contract: &serde_json::Value) {
    for row in contract["cases"].as_array().unwrap() {
        let snapshot = source(row);
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(if row["event"] == "remove" { editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID } else { editing::INSERT_SNAPSHOT_VALUE_ACTION_ID });
        inputs(&snapshot, event(row), &mut caller, &mut work, |input, caller, work| {
            let mut result = None;
            for turn in 0..100000 {
                if work.parts.is_some() && work.value.is_none() && !work.value_edit.started() && work.pending.is_none() {
                    let demand = work.work_demands(input, 4096).unwrap();
                    let length = work.parts.as_ref().unwrap()[0].args.len();
                    for axis in 0..4 {
                        let mut denied = grant();
                        let mut fuel = 1;
                        match axis {
                            0 => denied.maximum_items = 0,
                            1 => denied.maximum_copy_bytes = demand.copy_bytes - 1,
                            2 => denied.maximum_depth = 0,
                            _ => fuel = 0,
                        }
                        caller
                            .work_turn(denied, fuel, |cx| {
                                let before = cx.retained_progress();
                                let step = work.step(input, cx)?;
                                assert!(!matches!(step, ArtifactCommandWorkStep::Complete(_)));
                                assert_eq!(before, cx.retained_progress());
                                Ok(())
                            })
                            .unwrap();
                        assert!(work.value.is_none());
                        assert_eq!(length, work.parts.as_ref().unwrap()[0].args.len());
                    }
                }
                let step = caller.work_turn(RetainedCloneGrant { maximum_items: 1, ..grant() }, 1, |cx| work.step(input, cx));
                assert_eq!(work.reduction_ticks(), 0);
                assert!(work.entity.is_none() && work.semantic_emit.is_none() && work.cursor.terminal_is_empty());
                match step {
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
                assert!(turn < 99999);
            }
            let result = result.unwrap();
            assert_eq!(result.is_ok(), row["admitted"].as_bool().unwrap(), "{}", row["id"]);
            if let Ok(emit) = result {
                assert_eq!(emit.artifact_mutations.len(), 1);
                let mutation = &emit.artifact_mutations[0];
                assert!(matches!(mutation, IfcMutation::SetEntityComplex(_)));
                let next = protocol::apply_diff_cold(mutation.diff(&snapshot).diff(), &snapshot).unwrap();
                let mut expected = snapshot.as_ref().clone();
                let args = &mut expected.entities[0].complex[0].args;
                let index = row["path"].as_str().unwrap().rsplit('/').next().unwrap();
                let index = if index == "-" { args.len() } else { index.parse().unwrap() };
                if row["event"] == "remove" {
                    args.remove(index);
                } else {
                    args.insert(index, native(&row["value"]));
                }
                assert_eq!(next, expected);
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
