
use super::*;
use crate::os_store::ErasedSnapshotRetirement;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};

#[test]
fn graph_parameter_intent_matches_strict_typed_schema() {
    let fixture = semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
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
fn graph_parameter_intent_retirement_preserves_exact_bytes_and_worker_transfer() {
    let fixture = semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let text = fixture.get("longWidgetId").unwrap();
    let law = fixture.get("retirement").unwrap();
    for pause in law.get("cancelAt").and_then(semio_framework_pack_json::Value::as_array).unwrap() {
        let payload = SetGraphParameter {
            widget_id: text.get("unit").and_then(semio_framework_pack_json::Value::as_str).unwrap().repeat(text.get("repetitions").and_then(semio_framework_pack_json::Value::as_u64).unwrap() as usize),
            value: 4.0,
            surface_id: Some(law.get("surfaceUnit").and_then(semio_framework_pack_json::Value::as_str).unwrap().repeat(law.get("surfaceRepetitions").and_then(semio_framework_pack_json::Value::as_u64).unwrap() as usize)),
        };
        let expected = payload.widget_id.capacity() + payload.surface_id.as_ref().unwrap().capacity();
        let pointers = [payload.widget_id.as_ptr(), payload.surface_id.as_ref().unwrap().as_ptr()];
        let mut owner = payload.into_retirement();
        let grant = RetainedCloneGrant { maximum_items: law.get("maximumItems").unwrap().as_u64().unwrap() as usize, maximum_copy_bytes: law.get("copyBytes").unwrap().as_u64().unwrap() as usize, maximum_capacity_bytes: law.get("capacityBytes").unwrap().as_u64().unwrap() as usize, maximum_release_bytes: law.get("releaseProbeBytes").unwrap().as_u64().unwrap() as usize, maximum_depth: law.get("maximumDepth").unwrap().as_u64().unwrap() as usize };
        for _ in 0..pause.as_u64().unwrap() + law.get("repeatedRefusals").unwrap().as_u64().unwrap() {
            assert_eq!(owner.close_step(grant).unwrap().progress(), RetainedCloneProgress::default());
            assert_eq!(owner.bytes[0].as_ref().unwrap().as_ptr(), pointers[0]);
            assert_eq!(owner.bytes[1].as_ref().unwrap().as_ptr(), pointers[1]);
        }
        assert_eq!(owner.close_step(RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap().progress(), RetainedCloneProgress::default());
        assert_eq!(owner.close_step(RetainedCloneGrant { maximum_release_bytes: 0, ..grant }).unwrap().progress(), RetainedCloneProgress::default());
        assert!(owner.close_step(RetainedCloneGrant { maximum_depth: 0, ..grant }).is_err());
        let released = std::thread::spawn(move || {
            let mut released = 0;
            for _ in 0..2 {
                let exact = RetainedCloneGrant { maximum_release_bytes: owner.next_release_byte_demand().unwrap(), ..grant };
                let step = owner.close_step(exact).unwrap();
                assert!(step.progress().fits(exact));
                released += step.progress().released_bytes;
            }
            assert!(owner.terminal_is_empty());
            released
        }).join().unwrap();
        assert_eq!(released, expected);
    }
    let owner = SetGraphParameter { widget_id: "guard".into(), value: 1.0, surface_id: None }.into_retirement();
    assert!(std::panic::catch_unwind(|| drop(owner)).is_err());
}
