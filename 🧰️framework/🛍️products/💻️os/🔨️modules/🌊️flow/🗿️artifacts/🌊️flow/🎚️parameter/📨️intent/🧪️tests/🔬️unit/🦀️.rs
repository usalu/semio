
use super::*;
use crate::os_store::{ErasedSnapshotRetirement, SnapshotRetirementStep};

#[test]
fn graph_parameter_intent_matches_strict_typed_schema() {
    let fixture = crate::os_pack::json::parse(include_str!("../../🧪️fixture/🔣️.json")).unwrap();
    for row in fixture.get("cases").and_then(crate::os_pack::json::Value::as_array).unwrap() {
        let payload: SetGraphParameter = crate::os_dsl::FromValue::from_value(crate::os_pack::json::to_dsl_value(row)).unwrap();
        payload.validate().unwrap();
        assert_eq!(crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(&payload)), *row);
    }
    for row in fixture.get("rejected").and_then(crate::os_pack::json::Value::as_array).unwrap() {
        assert!(<SetGraphParameter as crate::os_dsl::FromValue>::from_value(crate::os_pack::json::to_dsl_value(row)).map_or(true, |value| value.validate().is_err()));
    }
    let row = fixture.get("longWidgetId").unwrap();
    let widget_id = row.get("unit").and_then(crate::os_pack::json::Value::as_str).unwrap().repeat(row.get("repetitions").and_then(crate::os_pack::json::Value::as_u64).unwrap() as usize);
    assert_eq!(widget_id.len(), row.get("expectedBytes").and_then(crate::os_pack::json::Value::as_u64).unwrap() as usize);
    let payload = SetGraphParameter { widget_id, value: row.get("value").and_then(crate::os_pack::json::Value::as_f64).unwrap(), surface_id: None };
    payload.validate().unwrap();
    let round_tripped: SetGraphParameter = crate::os_pack::json::from_json_str(&crate::os_pack::json::to_json_string(&payload)).unwrap();
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
    let fixture = crate::os_pack::json::parse(include_str!("../../🧪️fixture/🔣️.json")).unwrap();
    let text = fixture.get("longWidgetId").unwrap();
    let law = fixture.get("retirement").unwrap();
    for maximum in [1, 4096] {
        for pause in law.get("cancelAt").and_then(crate::os_pack::json::Value::as_array).unwrap() {
            let payload = SetGraphParameter {
                widget_id: text.get("unit").and_then(crate::os_pack::json::Value::as_str).unwrap().repeat(text.get("repetitions").and_then(crate::os_pack::json::Value::as_u64).unwrap() as usize),
                value: 4.0,
                surface_id: Some(law.get("surfaceUnit").and_then(crate::os_pack::json::Value::as_str).unwrap().repeat(law.get("surfaceRepetitions").and_then(crate::os_pack::json::Value::as_u64).unwrap() as usize)),
            };
            let expected = payload.widget_id.len() + payload.surface_id.as_ref().unwrap().len();
            let mut owner = payload.into_retirement();
            let mut released = 0;
            for _ in 0..pause.as_u64().unwrap() {
                if let SnapshotRetirementStep::Pending { released_bytes, .. } = owner.close_step(1, maximum).unwrap() {
                    released += released_bytes;
                }
            }
            assert_eq!(owner.close_step(0, maximum).unwrap(), SnapshotRetirementStep::Blocked);
            assert_eq!(owner.close_step(1, 0).unwrap(), SnapshotRetirementStep::Blocked);
            let (owner, released) = std::thread::spawn(move || {
                for _ in 0..100_000 {
                    match owner.close_step(1, maximum).unwrap() {
                        SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                            assert!(released_items <= 1 && released_bytes <= maximum);
                            released += released_bytes;
                        }
                        SnapshotRetirementStep::Complete => break,
                        SnapshotRetirementStep::Blocked => panic!("positive intent retirement grant blocked"),
                    }
                }
                (owner, released)
            })
            .join()
            .unwrap();
            assert!(owner.terminal_is_empty());
            assert_eq!(released, expected);
        }
    }
    let owner = SetGraphParameter { widget_id: "guard".into(), value: 1.0, surface_id: None }.into_retirement();
    assert!(std::panic::catch_unwind(|| drop(owner)).is_err());
}
