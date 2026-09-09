//! 🧪️ Direct mutation laws against language-neutral input and state vectors.

use crate::schema::diff::GltfDiff;
use crate::schema::mutations::GltfMutation;
use crate::GltfSnapshot;
use protocol::{FromValue, Mutation, MutationDiff, MutationKind, ToValue};
use serde_json::Value;

fn assert_projection(expected: &Value, actual: &Value) {
    match expected {
        Value::Object(fields) => {
            let object = actual.as_object().expect("encoded object");
            for (key, value) in fields {
                assert_projection(value, object.get(key).expect(key));
            }
        }
        Value::Array(items) => {
            let values = actual.as_array().expect("encoded array");
            assert_eq!(items.len(), values.len());
            for (expected, actual) in items.iter().zip(values) {
                assert_projection(expected, actual);
            }
        }
        _ => assert_eq!(expected, actual),
    }
}

pub(crate) fn decode<T: FromValue + ToValue>(value: &Value) -> T {
    let decoded = pack::from_json_str(&value.to_string()).expect("owned codec decodes fixture");
    let encoded: Value = serde_json::from_str(&pack::to_json_string(&decoded)).expect("independent JSON parser accepts owned encoding");
    assert_projection(value, &encoded);
    decoded
}

pub(crate) fn scene_snapshot(value: &Value) -> GltfSnapshot {
    let mut snapshot = GltfSnapshot::default();
    snapshot.document.scene = decode(&value["scene"]);
    snapshot.document.scenes = decode(&value["scenes"]);
    snapshot
}

pub(crate) fn assert_laws<K: MutationKind<GltfSnapshot, GltfMutation>>(mutation: &K, base: &GltfSnapshot, expected: &GltfSnapshot) {
    let outcome = mutation.diff(base);
    assert!(outcome.messages().is_empty(), "{:?}", outcome.messages());
    let wire: Value = serde_json::from_str(&pack::to_json_string(outcome.diff())).unwrap();
    let diff: GltfDiff = decode(&wire);
    assert_eq!(&diff, outcome.diff());
    let after = diff.apply(base).expect("typed diff applies");
    assert_eq!(&after, expected);
    assert_eq!(diff.apply(base).unwrap(), after);
    let inverse = mutation.inverse(base);
    assert_eq!(inverse.len(), 1);
    let wire: Value = serde_json::from_str(&pack::to_json_string(&inverse)).unwrap();
    let inverse: Vec<GltfMutation> = decode(&wire);
    let mut restored = after;
    for mutation in inverse {
        let outcome = mutation.diff(&restored);
        assert!(outcome.messages().is_empty(), "{:?}", outcome.messages());
        restored = outcome.diff().apply(&restored).expect("typed inverse applies");
    }
    assert_eq!(&restored, base);
}
