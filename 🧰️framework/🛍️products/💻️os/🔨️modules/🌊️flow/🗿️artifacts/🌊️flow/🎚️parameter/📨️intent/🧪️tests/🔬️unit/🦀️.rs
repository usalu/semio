
use super::*;
use crate::os_store::ErasedSnapshotRetirement;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};

#[test]
fn graph_parameter_intent_matches_strict_typed_schema() {
    let fixture = semio_framework_pack_json::parse(include_str!("../🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for row in fixture.get("cases").and_then(semio_framework_pack_json::Value::as_array).unwrap() {
        let payload: SetGraphParameter = semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(row)).unwrap();
        payload.validate().unwrap();
        assert_eq!(semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&payload)), *row);
    }
    for row in fixture.get("rejected").and_then(semio_framework_pack_json::Value::as_array).unwrap() {
        assert!(<SetGraphParameter as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(row)).map_or(true, |value| value.validate().is_err()));
    }
    let row = fixture.get("longWidgetId").unwrap();
    let widget_id = row.get("unit").and_then(semio_framework_pack_json::Value::as_str).unwrap().repeat(row.get("repetitions").and_then(semio_framework_pack_json::Value::as_u64).unwrap() as usize);
    assert_eq!(widget_id.len(), row.get("expectedBytes").and_then(semio_framework_pack_json::Value::as_u64).unwrap() as usize);
    let payload = SetGraphParameter { widget_id, value: row.get("value").and_then(semio_framework_pack_json::Value::as_f64).unwrap(), surface_id: None };
    payload.validate().unwrap();
    let round_tripped: SetGraphParameter = semio_framework_pack_json::from_json_str(&semio_framework_pack_json::to_json_string(&payload), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(round_tripped, payload);
}

#[test]
fn graph_parameter_intent_rejects_non_finite_before_retained_admission() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(SetGraphParameter { widget_id: "slider".into(), value, surface_id: None }.validate().is_err());
    }
}

#[test]
fn graph_parameter_intent_retires_through_its_derived_owner() {
    use semio_framework_value::{retained_clone::RetainedCloneGrant, retirement::controlled::ControlledRetirement};
    let payload = SetGraphParameter { widget_id: "widget".repeat(64), value: 4.0, surface_id: Some("surface".repeat(64)) };
    let mut owner = ControlledRetirement::new(payload).map_err(|(error, _)| error).unwrap();
    for _ in 0..4096 {
        if owner.terminal_is_empty() { break; }
        let copy = 4096.max(owner.next_copy_byte_demand().unwrap());
        let demand = owner.next_demand(copy).unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        let mut turn = semio_framework_job::OriginalRetirementPump::new().turn(1 << 16, u64::MAX, || Some(0), grant).unwrap();
        turn.with_effect(|effect| owner.step(copy, effect)).unwrap().expect("graph parameter close quote admitted");
    }
    assert!(owner.terminal_is_empty());
}

#[test]
fn original_graph_intent_indexed_json_retains_exact_strings_and_system_receipts() {
    use crate::os_store::ArtifactCanonicalJsonCursor;
    use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let row = &fixture["canonicalPolicy"];
    let grant = RetainedCloneGrant { maximum_items: row["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: row["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: row["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: row["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: row["maximumDepth"].as_u64().unwrap() as usize };
    for row in fixture["cases"].as_array().unwrap() {
        let source: SetGraphParameter = serde_json::from_value(row.clone()).unwrap();
        let original = source.widget_id.as_ptr();
        let expected = serde_json::to_vec(&source).unwrap();
        let mut bytes = Vec::with_capacity(expected.len());
        let mut encoder = ArtifactCanonicalJsonCursor::default();
        for _ in 0..expected.len() * 4 + 64 {
            let mut output = [0xa5; 8];
            let (denied, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| encoder.encode_chunk_admitted(&source, &mut output, RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap());
            assert_eq!(denied.ownership.progress(), RetainedCloneProgress::default()); assert_eq!(denied.written_bytes, 0); assert_eq!(output, [0xa5; 8]); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| encoder.encode_chunk_admitted(&source, &mut output, grant).unwrap());
            let progress = step.ownership.progress();
            assert!(progress.copied_items <= grant.maximum_items && progress.copied_bytes <= grant.maximum_copy_bytes);
            assert_eq!((progress.retained_capacity_bytes, progress.released_bytes), (0, 0)); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert!(output[step.written_bytes..].iter().all(|byte| *byte == 0xa5)); bytes.extend_from_slice(&output[..step.written_bytes]);
            if encoder.is_complete() { break; }
        }
        assert!(encoder.is_complete()); assert_eq!(bytes, expected); assert_eq!(source.widget_id.as_ptr(), original);
    }
}
