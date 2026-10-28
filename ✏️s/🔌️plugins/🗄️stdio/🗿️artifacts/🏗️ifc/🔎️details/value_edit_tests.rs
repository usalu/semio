//! 🧪️ Neutral edits exercise actual original authority, every frontier and retained close.
use super::*;

fn dsl(value: &serde_json::Value) -> DslValue {
    let parsed = semio_framework_pack_json::parse(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    semio_framework_pack_json::to_dsl_value(&parsed)
}
fn event(value: &serde_json::Value) -> editing::SnapshotEditEvent {
    let path = value["path"].as_str().unwrap().into();
    match value["kind"].as_str().unwrap() {
        "set" => editing::SnapshotEditEvent::SetValue { path, value: dsl(&value["value"]) },
        "insert" => editing::SnapshotEditEvent::InsertValue { path, value: dsl(&value["value"]) },
        "remove" => editing::SnapshotEditEvent::RemoveValue { path },
        "move" => editing::SnapshotEditEvent::MoveValue { path, from: value["from"].as_str().unwrap().into() },
        _ => panic!("unknown neutral edit"),
    }
}
fn grant(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth }
}
fn edit(value: IfcValue, mut event: editing::SnapshotEditEvent, cancel_at: Option<usize>) -> (Option<IfcValue>, usize) {
    let mut caller = semio_framework_plugin::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().unwrap();
    let mut decoder = semio_framework_job::OriginalJobDecodeOwner::empty();
    let mut cx = caller.context().with_original_decoder(&mut decoder);
    let policy = editing::fixture_mounted_owner_policy();
    let mut cursor = IfcValueEdit::default();
    let mut owner = Some(value);
    let mut stages = 0;
    let mut completed = false;
    for stage in 1..30_000 {
        stages = stage;
        if cancel_at == Some(stage) {
            cx.original_cancel_token().cancel_now();
        }
        let demand = cursor.demands().unwrap();
        assert!(
            demand.copy_bytes <= policy.preparation.maximum_copy_bytes
                && demand.capacity_bytes <= policy.preparation.maximum_capacity_bytes
                && demand.release_bytes <= policy.preparation.maximum_release_bytes
                && demand.depth <= policy.preparation.maximum_depth
        );
        match cx.with_retained_loan(grant(demand), |cx| cursor.advance(&mut owner, &mut event, 0, false, None, cx)) {
            Ok(true) => {
                completed = true;
                break;
            }
            Err(error) => {
                assert!(cancel_at.is_some() && cx.is_cancelled(), "unexpected original edit refusal: {error:?}");
                break;
            }
            _ => {}
        }
    }
    assert_eq!(completed, cancel_at.is_none());
    cursor.begin_close();
    for _ in 0..30_000 {
        if cursor.terminal_is_empty() {
            break;
        }
        let demand = cursor.close_demands().unwrap();
        cx.with_retained_loan(grant(demand), |cx| semio_framework_job::close_original_retained_step(cx, demand, |grant| cursor.close_step(grant))).unwrap();
    }
    assert!(cursor.terminal_is_empty());
    let fuel = cx.fuel_remaining();
    drop(cx);
    let fuel = caller.retire_decoder(&mut decoder, fuel).unwrap();
    caller.retire(fuel).unwrap();
    (completed.then_some(owner).flatten(), stages)
}
#[test]
fn neutral_native_edits_keep_original_custody_at_every_cancellation_prefix() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/value-edit.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let before = || IfcValue::from_value(dsl(&case["before"])).unwrap();
        let (result, stages) = edit(before(), event(&case["event"]), None);
        assert_eq!(result.unwrap(), IfcValue::from_value(dsl(&case["after"])).unwrap());
        for prefix in 1..=stages {
            assert!(edit(before(), event(&case["event"]), Some(prefix)).0.is_none());
        }
    }
}
#[test]
fn native_collection_moves_one_large_owned_slot_per_turn() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/value-edit.json")).unwrap();
    let case = &fixture["largeCollection"];
    let count = case["items"].as_u64().unwrap() as usize;
    let bytes = case["textBytes"].as_u64().unwrap() as usize;
    let values = || IfcValue::Aggregate((0..count).map(|index| IfcValue::String(format!("{index}:{}", "x".repeat(bytes)))).collect());
    let command = || editing::SnapshotEditEvent::MoveValue { from: format!("/value/{}", case["moveFrom"].as_u64().unwrap()), path: format!("/value/{}", case["moveTo"].as_u64().unwrap()) };
    let (result, stages) = edit(values(), command(), None);
    let IfcValue::Aggregate(items) = result.unwrap() else { panic!("collection result required") };
    assert!(items[count - 1].as_str().unwrap().starts_with("0:"));
    assert!(items[0].as_str().unwrap().starts_with("1:"));
    assert!(stages >= count);
    for prefix in [1, 4, count / 2, count, stages] {
        assert!(edit(values(), command(), Some(prefix)).0.is_none());
    }
}

#[test]
fn original_large_nested_text_keeps_its_allocation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/value-edit.json")).unwrap();
    let case = &fixture["largeInput"];
    let text = "x".repeat(case["textBytes"].as_u64().unwrap() as usize);
    let address = text.as_ptr();
    let event = editing::SnapshotEditEvent::SetValue { path: case["path"].as_str().unwrap().into(), value: DslValue::String(text) };
    let result = edit(IfcValue::Aggregate(vec![IfcValue::String("before".into())]), event, None).0.unwrap();
    let IfcValue::Aggregate(values) = result else { panic!("aggregate result required") };
    assert_eq!(values[0].as_str().unwrap().as_ptr(), address);
}
