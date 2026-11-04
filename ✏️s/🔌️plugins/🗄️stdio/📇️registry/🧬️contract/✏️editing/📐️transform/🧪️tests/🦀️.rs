//! 🧪️ The transform gesture reproduces the three.js oracle recorded in the language-agnostic fixture and refuses malformed requests.

use super::*;
use semio_framework_value::DslValue;

const FIXTURE: &str = include_str!("../../🧪️tests/🧫️fixtures/📐️transform/🔣️.json");

fn json(source: &str) -> DslValue {
    semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(source, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap())
}

fn numbers(value: &DslValue) -> Vec<f64> {
    value.as_array().unwrap().iter().map(|item| item.as_f64().unwrap()).collect()
}

fn close(actual: &[f64], expected: &[f64], tolerance: f64) {
    assert_eq!(actual.len(), expected.len());
    actual.iter().zip(expected).for_each(|(left, right)| assert!((left - right).abs() <= tolerance, "{actual:?} != {expected:?}"));
}

#[test]
fn points_directions_and_quaternions_match_the_three_oracle() {
    let fixture = json(FIXTURE);
    let tolerance = fixture.get("tolerance").unwrap().as_f64().unwrap();
    let cases = fixture.get("cases").unwrap().as_array().unwrap();
    assert!(!cases.is_empty());
    cases.iter().for_each(|case| {
        let transform = Transform::from_value(case.get("transform").unwrap()).unwrap();
        case.get("points").unwrap().as_array().unwrap().iter().for_each(|row| {
            let input: [f64; 3] = numbers(row.get("input").unwrap()).try_into().unwrap();
            close(&transform.point(input), &numbers(row.get("expected").unwrap()), tolerance);
        });
        case.get("directions").unwrap().as_array().unwrap().iter().for_each(|row| {
            let input: [f64; 3] = numbers(row.get("input").unwrap()).try_into().unwrap();
            if input.iter().all(|value| *value == 0.0) { return; }
            close(&transform.direction(input), &numbers(row.get("expected").unwrap()), tolerance);
        });
        close(&transform.quaternion([0.0, 0.0, 0.0, 1.0]), &numbers(case.get("quaternion").unwrap()), tolerance);
    });
}

#[test]
fn the_identity_transform_changes_nothing() {
    let transform = Transform::from_value(&json("{}")).unwrap();
    assert!(transform.is_identity());
    assert_eq!(transform.point([1.5, -2.0, 3.0]), [1.5, -2.0, 3.0]);
}

#[test]
fn malformed_transforms_are_refused_with_a_stable_code() {
    ["[]", r#"{"translate":[1,2]}"#, r#"{"scale":[1,0,1]}"#, r#"{"rotate":"x"}"#, r#"{"translate":[1,2,"a"]}"#].iter().for_each(|source| {
        assert_eq!(Transform::from_value(&json(source)).unwrap_err().code.0, "snapshot-edit.transform-invalid", "{source}");
    });
}

#[test]
fn only_the_transform_path_is_the_gesture() {
    let other = SnapshotEditEvent::SetValue { path: "/vertices/0/x".into(), value: json("1") };
    assert!(Transform::from_event(&other).is_none());
    let event = SnapshotEditEvent::SetValue { path: TRANSFORM_PATH.into(), value: json(r#"{"translate":[1,0,0]}"#) };
    assert_eq!(Transform::from_event(&event).unwrap().unwrap().translate, [1.0, 0.0, 0.0]);
}
