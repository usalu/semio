//! 🧪️ Original reference inputs retain exact native values, authority and controlled inverse publication.
use super::*;
fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("🧪️tests/reference-reduction.json")).unwrap()
}
fn dsl(value: &serde_json::Value) -> DslValue {
    let parsed = semio_framework_pack_json::parse(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    semio_framework_pack_json::to_dsl_value(&parsed)
}
#[test]
fn original_reference_reduction_neutral_cases_publish_and_undo_exact_source() {
    let fixture = fixture();
    let snapshot = Arc::new(IfcSnapshot::from_value(dsl(&fixture["snapshot"])).unwrap());
    for row in fixture["cases"].as_array().unwrap() {
        let event = editing::SnapshotEditEvent::SetValue { path: row["path"].as_str().unwrap().into(), value: dsl(&row["value"]) };
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot, event, &mut caller, &mut work, |input, caller, work| {
            let mut result = None;
            for turn_index in 0..100_000 {
                match turn(caller, 1_000_000, false, |cx| work.step(input, cx)) {
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
            assert_eq!(result.is_ok(), row["admitted"].as_bool().unwrap(), "{}", row["name"]);
            if let Ok(emit) = result {
                assert!(work.reference.complete());
                let mut next = snapshot.as_ref().clone();
                let mut inverses = Vec::new();
                for mutation in emit.artifact_mutations {
                    inverses.splice(0..0, mutation.inverse(&next).unwrap());
                    next = protocol::apply_diff_cold(mutation.diff(&next).diff(), &next).unwrap();
                }
                for inverse in inverses {
                    next = protocol::apply_diff_cold(inverse.diff(&next).diff(), &next).unwrap();
                }
                assert_eq!(&next, snapshot.as_ref());
            }
            caller.cancel().cancel_now();
            close(caller, work);
        });
        caller.finish();
    }
}
#[test]
fn original_constituent_reference_work_progresses_with_single_turn_fuel_and_cancels_every_prefix() {
    let fixture = fixture();
    let snapshot = Arc::new(IfcSnapshot::from_value(dsl(&fixture["snapshot"])).unwrap());
    let original = snapshot.as_ref().clone();
    let mut turns = 0;
    for cancel_at in std::iter::once(None).chain((1..100_000).map(Some)) {
        if cancel_at.is_some_and(|turn| turn > turns) {
            break;
        }
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
        let event = editing::SnapshotEditEvent::SetValue { path: "/entities/8/complex/0/args/1/reference".into(), value: DslValue::String("16".into()) };
        inputs(&snapshot, event, &mut caller, &mut work, |input, caller, work| {
            for turn_index in 1..100_000 {
                if cancel_at == Some(turn_index) {
                    caller.cancel().cancel_now();
                }
                let result = caller.work_turn(RetainedCloneGrant { maximum_items: 1, ..grant() }, 1, |cx| work.step(input, cx));
                assert_eq!(snapshot.as_ref(), &original);
                if cancel_at == Some(turn_index) {
                    assert!(result.is_err());
                    break;
                }
                if let ArtifactCommandWorkStep::Complete(emit) = result.unwrap() {
                    assert_eq!(emit.artifact_mutations.len(), 1);
                    assert!(matches!(&emit.artifact_mutations[0],IfcMutation::SetEntityComplex(change)if change.id==13&&change.complex[0].args[1]==IfcValue::Reference(16)));
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
fn enumeration_source() -> Arc<IfcSnapshot> {
    let typed = |value: &str| IfcValue::TypedValue(IfcTypedValue { name: "IFCLABEL".into(), items: vec![IfcValue::String(value.into())] });
    Arc::new(IfcSnapshot {
        entities: vec![
            IfcEntity { id: 1, name: "IFCPROPERTYENUMERATEDVALUE".into(), args: vec![IfcValue::String("Colour".into()), IfcValue::Unset, IfcValue::Aggregate(vec![typed("Red")]), IfcValue::Reference(u64::MAX)], complex: vec![] },
            IfcEntity { id: u64::MAX, name: "IFCPROPERTYENUMERATION".into(), args: vec![IfcValue::String("Palette".into()), IfcValue::Aggregate(vec![typed("Red"), typed("Blue")]), IfcValue::Unset], complex: vec![] },
        ],
        ..Default::default()
    })
}
#[test]
fn original_contextual_enumeration_request_receives_actual_typed_payload_and_undoes() {
    let snapshot = enumeration_source();
    let typed = |value: &str| IfcValue::TypedValue(IfcTypedValue { name: "IFCLABEL".into(), items: vec![IfcValue::String(value.into())] });
    let contract: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/enumeration-request.json")).unwrap();
    for row in contract["cases"].as_array().unwrap() {
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
        let parsed = semio_framework_pack_json::parse(row["source"].as_str().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let insert = row["event"].as_str() == Some("insert");
        if insert {
            work = Work::new(editing::INSERT_SNAPSHOT_VALUE_ACTION_ID);
        }
        let path = row["path"].as_str().unwrap().into();
        let value = semio_framework_pack_json::to_dsl_value(&parsed);
        let event = if insert { editing::SnapshotEditEvent::InsertValue { path, value } } else { editing::SnapshotEditEvent::SetValue { path, value } };
        inputs(&snapshot, event, &mut caller, &mut work, |input, caller, work| {
            let mut result = None;
            for turn_index in 0..10000 {
                match turn(caller, 1_000_000, false, |cx| work.step(input, cx)) {
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
                assert!(turn_index < 9999);
            }
            let result = result.unwrap();
            assert_eq!(result.is_ok(), row["admitted"].as_bool().unwrap(), "{}", row["id"]);
            let Ok(emit) = result else {
                caller.cancel().cancel_now();
                close(caller, work);
                return;
            };
            assert_eq!(emit.artifact_mutations.len(), 1);
            let mutation = &emit.artifact_mutations[0];
            let next = protocol::apply_diff_cold(mutation.diff(&snapshot).diff(), &snapshot).unwrap();
            assert_eq!(next.entities[0].args[2], IfcValue::Aggregate(if insert { vec![typed("Red"), typed("Blue")] } else { vec![typed("Blue")] }));
            let mut restored = next.clone();
            for inverse in mutation.inverse(&snapshot).unwrap() {
                restored = protocol::apply_diff_cold(inverse.diff(&restored).diff(), &restored).unwrap();
            }
            assert_eq!(restored, *snapshot);
            caller.cancel().cancel_now();
            close(caller, work);
        });
        caller.finish();
    }
}
#[test]
fn original_contextual_enumeration_source_receiving_cancels_every_prefix_and_closes_before_source() {
    let snapshot = enumeration_source();
    let original = snapshot.as_ref().clone();
    let mut turns = 0;
    for cancel_at in std::iter::once(None).chain((1..100_000).map(Some)) {
        if cancel_at.is_some_and(|turn| turn > turns) {
            break;
        }
        let mut caller = OriginalWorkFixture::new();
        let mut work = Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
        let parsed = semio_framework_pack_json::parse("{\"$ifcEnumerationChoice\":{\"definition\":18446744073709551615,\"choice\":\"1\",\"list\":true}}", semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let event = editing::SnapshotEditEvent::SetValue { path: "/entities/0/args/2".into(), value: semio_framework_pack_json::to_dsl_value(&parsed) };
        inputs(&snapshot, event, &mut caller, &mut work, |input, caller, work| {
            for turn_index in 1..100_000 {
                if cancel_at == Some(turn_index) {
                    caller.cancel().cancel_now();
                }
                let result = caller.work_turn(RetainedCloneGrant { maximum_items: 1, ..grant() }, 1, |cx| work.step(input, cx));
                assert_eq!(snapshot.as_ref(), &original);
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
