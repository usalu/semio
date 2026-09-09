use super::*;
use serde_json::{json, Value};

fn decode(value: &Value) -> MapDelta<DslValue> {
    MapDelta::from_value(serde_json::from_value(value.clone()).unwrap()).unwrap()
}

fn base(value: &Value) -> BTreeMap<String, DslValue> {
    BTreeMap::from_value(serde_json::from_value(value.clone()).unwrap()).unwrap()
}

/// 🧪️ Shared neutral vectors check codec, strict application, compact composition and atomic failure.
#[test]
fn shared_map_delta_neutral_contract() {
    let fixture: Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let expected = decode(&row["compact"]);
        let mut composed = MapDelta::default();
        for entry in row["source"].as_array().unwrap() { composed.absorb(decode(&json!({ "entries": [entry] }))); }
        assert_eq!(composed, expected, "{}", row["name"]);
        let encoded = serde_json::to_value(expected.to_value()).unwrap();
        assert_eq!(encoded, row["compact"], "{}", row["name"]);
        assert_eq!(decode(&encoded), expected);
        let original = base(&row["before"]);
        let mut actual = original.clone();
        let outcome = expected.apply_to(&mut actual);
        if let Some(code) = row["error"].as_str() {
            assert_eq!(outcome.unwrap_err().code, code, "{}", row["name"]);
            assert_eq!(actual, original, "failed map application must be atomic");
        } else {
            outcome.unwrap();
            assert_eq!(serde_json::to_value(actual.to_value()).unwrap(), row["after"], "{}", row["name"]);
        }
    }
    for row in fixture["invalid"].as_array().unwrap() {
        assert!(MapDelta::<DslValue>::from_value(serde_json::from_value(row.clone()).unwrap()).is_err());
    }
    let duplicate = json!({ "entries": [fixture["cases"][0]["source"][0], fixture["cases"][0]["source"][0]] });
    assert!(MapDelta::<DslValue>::from_value(serde_json::from_value(duplicate).unwrap()).is_err());
    println!("[DEBUG] shared map delta native codec and apply matched 12 neutral vectors, preserving null, strict removal and atomic rejection");
}

/// 🪢️ Every compact single-key triple has associative output and identical successful bases.
#[test]
fn shared_map_delta_exhaustive_composition() {
    let mut changes = Vec::new();
    for precondition in ["any", "present", "absent"] {
        for operation in [json!({"kind":"set","value":null}), json!({"kind":"remove"})] {
            changes.push(decode(&json!({"entries":[{"key":"key","precondition":precondition,"operation":operation}]})));
        }
    }
    for a in &changes { for b in &changes { for c in &changes {
        let mut left = a.clone(); left.absorb(b.clone()); left.absorb(c.clone());
        let mut right_tail = b.clone(); right_tail.absorb(c.clone());
        let mut right = a.clone(); right.absorb(right_tail);
        assert_eq!(left, right);
        for value in [json!({}), json!({"key":"old"}), json!({"key":null})] {
            let mut sequential = base(&value);
            let sequence = a.apply_to(&mut sequential).and_then(|_| b.apply_to(&mut sequential)).and_then(|_| c.apply_to(&mut sequential));
            let mut compact = base(&value);
            let combined = left.apply_to(&mut compact);
            assert_eq!(combined.is_ok(), sequence.is_ok());
            if combined.is_ok() { assert_eq!(compact, sequential); }
        }
    } } }
    println!("[DEBUG] shared map delta native exhaustive composition matched 648 bases and associative compact output");
}
